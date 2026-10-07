//! `revue dev`: build the app, run it, and rebuild and restart it when its
//! source changes.
//!
//! The app owns the terminal while it runs, so `revue dev` stays silent until
//! the app stops. On a change it stops the app, puts the terminal back the
//! way it found it, rebuilds, prints any build errors, and starts the app
//! again. Stylesheets are not a reason to restart when the app's revue can
//! reload them itself: the app is built with the `revue/hot-reload` feature
//! and run with `REVUE_HOT_RELOAD=1`, which makes `AppBuilder::style` files
//! reload in place.

use colored::Colorize;
use notify::{EventKind, RecursiveMode, Watcher};
use std::io::{BufRead, BufReader, IsTerminal, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::time::{Duration, Instant};

use crate::commands::Result;

/// How long the tree must be quiet after a change before acting on it, so
/// that one save (or a `git checkout`) restarts the app once.
pub const QUIET_PERIOD: Duration = Duration::from_millis(250);

/// Act on a burst that never goes quiet after this long anyway.
pub const MAX_DELAY: Duration = Duration::from_secs(2);

/// How long a stopped app gets to exit before it is killed.
const STOP_GRACE: Duration = Duration::from_secs(2);

/// The environment variable revue's `AppBuilder` checks to turn on
/// stylesheet hot reload (`hot-reload` feature).
pub const HOT_RELOAD_ENV: &str = "REVUE_HOT_RELOAD";

/// Paths watched in every project, relative to the project root. Missing ones
/// are skipped.
pub const DEFAULT_WATCH: &[&str] = &["src", "styles", "Cargo.toml", "build.rs"];

/// Options for `revue dev`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DevOptions {
    /// Extra files or directories to watch, on top of [`DEFAULT_WATCH`].
    pub watch: Vec<PathBuf>,
    /// Which binary to run when the package has several.
    pub bin: Option<String>,
}

/// What a changed path means for the running app.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
    /// Source, manifest or data: rebuild and restart.
    Rebuild,
    /// A stylesheet: the app reloads it itself, if it can.
    Css,
    /// Build output, VCS data, editor scratch files: nothing to do.
    Ignore,
}

/// Classify a changed path, given relative to the project root.
pub fn classify(path: &Path) -> ChangeKind {
    let names: Vec<_> = path
        .components()
        .filter_map(|c| match c {
            Component::Normal(name) => Some(name.to_string_lossy()),
            _ => None,
        })
        .collect();
    if names.iter().any(|n| n == "target" || n.starts_with('.')) {
        return ChangeKind::Ignore;
    }
    let Some(file) = names.last() else {
        return ChangeKind::Ignore;
    };
    // Cargo rewrites Cargo.lock during a build: reacting to it would loop.
    if is_scratch_file(file) || file == "Cargo.lock" {
        return ChangeKind::Ignore;
    }
    let is_css = path
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("css"));
    // Anything under styles/ is a stylesheet or an editor's temp file for one.
    let in_styles = names.len() > 1 && names[0] == "styles";
    if is_css || in_styles {
        ChangeKind::Css
    } else {
        ChangeKind::Rebuild
    }
}

/// Editor backup, swap and probe files.
fn is_scratch_file(name: &str) -> bool {
    name.ends_with('~')
        || name == "4913" // vim's write-permission probe
        || [".swp", ".swo", ".swx", ".tmp"]
            .iter()
            .any(|ext| name.ends_with(ext))
}

/// How the app picks up stylesheet changes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CssMode {
    /// The app reloads its stylesheets itself; a CSS change needs nothing.
    HotReload {
        /// The dependency key of revue, for `--features <key>/hot-reload`.
        dep: String,
    },
    /// The app cannot reload them; a CSS change restarts it. The string says
    /// why, for the banner.
    Restart(String),
}

impl CssMode {
    /// Decide from the project's `Cargo.toml` and whether the revue it
    /// resolves to honors [`HOT_RELOAD_ENV`] (`None`: could not tell; see
    /// [`honors_hot_reload_env`]).
    pub fn detect(manifest: &str, honors_env: Option<bool>) -> CssMode {
        let Some(dep) = revue_dependency(manifest) else {
            return CssMode::Restart("the package does not depend on revue directly".to_string());
        };
        match honors_env {
            Some(true) => CssMode::HotReload { dep },
            Some(false) => CssMode::Restart(format!(
                "this revue predates {HOT_RELOAD_ENV}; a newer revue reloads CSS in place"
            )),
            None => CssMode::Restart("could not locate the revue source".to_string()),
        }
    }

    /// Whether a change of this kind needs a rebuild and restart.
    pub fn restarts_on(&self, kind: ChangeKind) -> bool {
        match kind {
            ChangeKind::Rebuild => true,
            ChangeKind::Css => matches!(self, CssMode::Restart(_)),
            ChangeKind::Ignore => false,
        }
    }
}

/// The key under `[dependencies]` that names revue (`revue`, or a renamed
/// dependency with `package = "revue"`).
pub fn revue_dependency(manifest: &str) -> Option<String> {
    let manifest: toml::Table = manifest.parse().ok()?;
    let deps = manifest.get("dependencies")?.as_table()?;
    deps.iter().find_map(|(key, spec)| {
        let package = spec.get("package").and_then(|p| p.as_str());
        match package {
            Some("revue") => Some(key.clone()),
            None if key == "revue" => Some(key.clone()),
            _ => None,
        }
    })
}

/// The directory of the revue package a project resolves to, from the
/// output of `cargo metadata --format-version 1`.
pub fn revue_package_dir(metadata: &str) -> Option<PathBuf> {
    let metadata: serde_json::Value = serde_json::from_str(metadata).ok()?;
    metadata["packages"].as_array()?.iter().find_map(|pkg| {
        if pkg["name"] != "revue" {
            return None;
        }
        Path::new(pkg["manifest_path"].as_str()?)
            .parent()
            .map(Path::to_path_buf)
    })
}

/// Does the revue source in `package_dir` read [`HOT_RELOAD_ENV`]?
///
/// Looking at the source rather than the version number works the same for
/// crates.io releases, git dependencies and local checkouts.
pub fn honors_hot_reload_env(package_dir: &Path) -> bool {
    walkdir::WalkDir::new(package_dir.join("src"))
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "rs"))
        .any(|entry| {
            std::fs::read_to_string(entry.path())
                .is_ok_and(|text| text.contains(&format!("\"{HOT_RELOAD_ENV}\"")))
        })
}

/// Arguments for `cargo build`.
pub fn cargo_build_args(css: &CssMode, bin: Option<&str>) -> Vec<String> {
    let mut args = vec![
        "build".to_string(),
        "--message-format=json-render-diagnostics".to_string(),
    ];
    if let CssMode::HotReload { dep } = css {
        args.push("--features".to_string());
        args.push(format!("{dep}/hot-reload"));
    }
    if let Some(bin) = bin {
        args.push("--bin".to_string());
        args.push(bin.to_string());
    }
    args
}

/// A binary cargo reported building.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltBinary {
    pub name: String,
    pub path: PathBuf,
}

/// Pick the binaries out of cargo's JSON messages.
pub fn built_binaries(messages: &str) -> Vec<BuiltBinary> {
    messages
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|msg| msg["reason"] == "compiler-artifact")
        .filter(|msg| {
            msg["target"]["kind"]
                .as_array()
                .is_some_and(|kinds| kinds.iter().any(|k| k == "bin"))
        })
        .filter_map(|msg| {
            Some(BuiltBinary {
                name: msg["target"]["name"].as_str()?.to_string(),
                path: PathBuf::from(msg["executable"].as_str()?),
            })
        })
        .collect()
}

/// Choose the binary to run: the one asked for, the only one, or the
/// package's `default-run`.
pub fn choose_binary(
    binaries: &[BuiltBinary],
    wanted: Option<&str>,
    default_run: Option<&str>,
) -> std::result::Result<PathBuf, String> {
    if let Some(name) = wanted.or(default_run) {
        return binaries
            .iter()
            .find(|b| b.name == name)
            .map(|b| b.path.clone())
            .ok_or_else(|| format!("cargo built no binary named '{name}'"));
    }
    match binaries {
        [] => Err("the package has no binary to run".to_string()),
        [only] => Ok(only.path.clone()),
        several => {
            let names: Vec<&str> = several.iter().map(|b| b.name.as_str()).collect();
            Err(format!(
                "the package has several binaries ({}); pick one with --bin",
                names.join(", ")
            ))
        }
    }
}

/// `package.default-run` from the manifest.
fn default_run(manifest: &str) -> Option<String> {
    let manifest: toml::Table = manifest.parse().ok()?;
    Some(
        manifest
            .get("package")?
            .get("default-run")?
            .as_str()?
            .to_string(),
    )
}

/// Collects a burst of changes and says when it has settled.
#[derive(Debug, Clone)]
pub struct Debouncer {
    quiet: Duration,
    max_delay: Duration,
    first: Option<Instant>,
    last: Option<Instant>,
    trigger: Option<PathBuf>,
}

impl Debouncer {
    pub fn new(quiet: Duration, max_delay: Duration) -> Self {
        Self {
            quiet,
            max_delay,
            first: None,
            last: None,
            trigger: None,
        }
    }

    /// Record a change that needs a restart.
    pub fn push(&mut self, path: PathBuf, now: Instant) {
        self.first.get_or_insert(now);
        self.last = Some(now);
        self.trigger.get_or_insert(path);
    }

    /// Is a restart pending?
    pub fn is_pending(&self) -> bool {
        self.first.is_some()
    }

    /// Has the pending burst settled (quiet for a while, or waited too long)?
    pub fn is_due(&self, now: Instant) -> bool {
        match (self.first, self.last) {
            (Some(first), Some(last)) => {
                now.duration_since(last) >= self.quiet
                    || now.duration_since(first) >= self.max_delay
            }
            _ => false,
        }
    }

    /// How long to wait before [`is_due`](Self::is_due) can turn true.
    pub fn time_left(&self, now: Instant) -> Option<Duration> {
        let (first, last) = (self.first?, self.last?);
        let quiet_at = last + self.quiet;
        let cap_at = first + self.max_delay;
        Some(quiet_at.min(cap_at).saturating_duration_since(now))
    }

    /// Clear the burst, returning the first path that changed in it.
    pub fn take(&mut self) -> Option<PathBuf> {
        self.first = None;
        self.last = None;
        self.trigger.take()
    }
}

/// Run `revue dev` in the current directory.
pub fn run(options: &DevOptions) -> Result<()> {
    let manifest_path = Path::new("Cargo.toml");
    if !manifest_path.exists() {
        return Err("Not in a Cargo project. Run 'revue new <name>' first.".into());
    }
    let root = std::env::current_dir()?.canonicalize()?;
    let terminal = TerminalGuard::capture();

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(move |result| {
        let _ = tx.send(result);
    })?;
    let mut watched = Vec::new();
    for path in DEFAULT_WATCH {
        let path = Path::new(path);
        if path.exists() {
            watch(&mut watcher, path)?;
            watched.push(path.to_path_buf());
        }
    }
    for path in &options.watch {
        if !path.exists() {
            return Err(format!("--watch path '{}' does not exist", path.display()).into());
        }
        watch(&mut watcher, path)?;
        watched.push(path.clone());
    }

    print_banner(&watched);

    let mut debouncer = Debouncer::new(QUIET_PERIOD, MAX_DELAY);
    let mut previous_css: Option<CssMode> = None;
    loop {
        // Re-read every time: the manifest (and so the revue version) may
        // have changed since the last build.
        let manifest = std::fs::read_to_string(manifest_path)?;
        let honors_env = cargo_metadata_revue_dir().map(|dir| honors_hot_reload_env(&dir));
        let css = CssMode::detect(&manifest, honors_env);
        if previous_css.as_ref() != Some(&css) {
            print_css_mode(&css);
        }
        previous_css = Some(css.clone());

        let mut app = match build(&css, options.bin.as_deref()) {
            Ok(exe) => Some(spawn(&exe, &css)?),
            Err(e) => {
                eprintln!("{} {}", "Build failed:".red().bold(), e);
                println!("{}", "Waiting for changes... (Ctrl+C to stop)".dimmed());
                None
            }
        };

        // Wait for a change that needs a restart, or for the app to exit.
        loop {
            if let Some(child) = app.as_mut() {
                if let Some(status) = child.try_wait()? {
                    // No exit code: a signal ended it before it could clean up.
                    terminal.restore(status.code().is_none());
                    app = None;
                    if status.success() {
                        println!("{}", "App exited. Stopping revue dev.".dimmed());
                        return Ok(());
                    }
                    eprintln!("{} {}", "App exited:".red().bold(), describe_exit(status));
                    println!("{}", "Waiting for changes... (Ctrl+C to stop)".dimmed());
                }
            }

            let now = Instant::now();
            if debouncer.is_due(now) {
                break;
            }
            let timeout = debouncer
                .time_left(now)
                .unwrap_or(Duration::from_millis(100))
                .min(Duration::from_millis(100));
            match rx.recv_timeout(timeout) {
                Ok(Ok(event)) => {
                    if matches!(event.kind, EventKind::Access(_)) {
                        continue;
                    }
                    for path in event.paths {
                        let relative = path.strip_prefix(&root).unwrap_or(&path);
                        if css.restarts_on(classify(relative)) {
                            debouncer.push(relative.to_path_buf(), Instant::now());
                        }
                    }
                }
                Ok(Err(e)) => {
                    if app.is_none() {
                        eprintln!("{} {}", "Watch error:".yellow(), e);
                    }
                }
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => {
                    return Err("file watcher stopped".into());
                }
            }
        }

        if let Some(child) = app.take() {
            stop(child);
            terminal.restore(true);
        }
        let trigger = debouncer.take();
        // Events from the save that triggered this restart are spent.
        while rx.try_recv().is_ok() {}
        if let Some(path) = trigger {
            println!();
            println!(
                "{} {} - rebuilding",
                "Changed:".cyan().bold(),
                path.display()
            );
        }
    }
}

fn watch(watcher: &mut notify::RecommendedWatcher, path: &Path) -> Result<()> {
    watcher
        .watch(path, RecursiveMode::Recursive)
        .map_err(|e| format!("cannot watch '{}': {}", path.display(), e).into())
}

/// Where the revue the project resolves to lives, from `cargo metadata`
/// (which also writes `Cargo.lock` if it is missing or stale, as a build
/// would).
fn cargo_metadata_revue_dir() -> Option<PathBuf> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1"])
        .stderr(Stdio::inherit())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    revue_package_dir(&String::from_utf8_lossy(&output.stdout))
}

fn print_banner(watched: &[PathBuf]) {
    println!("{}", "revue dev".cyan().bold());
    let names: Vec<String> = watched.iter().map(|p| p.display().to_string()).collect();
    println!("  {} {}", "Watching:".green(), names.join(", "));
    println!("  {} rebuild and restart the app", "On change:".green());
    println!("  {} quit the app; Ctrl+C while waiting", "Stop:".green());
}

fn print_css_mode(css: &CssMode) {
    match css {
        CssMode::HotReload { .. } => println!(
            "  {} reloaded in place, no restart ({HOT_RELOAD_ENV}=1)",
            "CSS:".green()
        ),
        CssMode::Restart(why) => {
            println!("  {} restarts the app ({why})", "CSS:".yellow())
        }
    }
    println!();
}

/// Build the project, letting cargo print progress and errors, and return
/// the binary to run.
fn build(css: &CssMode, bin: Option<&str>) -> std::result::Result<PathBuf, String> {
    let mut child = Command::new("cargo")
        .args(cargo_build_args(css, bin))
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("cannot run cargo: {e}"))?;
    let mut messages = String::new();
    if let Some(stdout) = child.stdout.take() {
        for line in BufReader::new(stdout).lines().map_while(|l| l.ok()) {
            messages.push_str(&line);
            messages.push('\n');
        }
    }
    let status = child.wait().map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("fix the errors above and save".to_string());
    }
    let manifest = std::fs::read_to_string("Cargo.toml").unwrap_or_default();
    choose_binary(
        &built_binaries(&messages),
        bin,
        default_run(&manifest).as_deref(),
    )
}

fn spawn(exe: &Path, css: &CssMode) -> Result<Child> {
    let mut command = Command::new(exe);
    if matches!(css, CssMode::HotReload { .. }) {
        command.env(HOT_RELOAD_ENV, "1");
    }
    command
        .spawn()
        .map_err(|e| format!("cannot start '{}': {}", exe.display(), e).into())
}

/// Stop the app: ask it to terminate, then kill it if it does not.
fn stop(mut child: Child) {
    #[cfg(unix)]
    {
        if let Ok(pid) = libc::pid_t::try_from(child.id()) {
            // SAFETY: plain kill(2) on a child we spawned and have not reaped.
            unsafe {
                libc::kill(pid, libc::SIGTERM);
            }
        }
        let deadline = Instant::now() + STOP_GRACE;
        while Instant::now() < deadline {
            if let Ok(Some(_)) = child.try_wait() {
                return;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }
    let _ = child.kill();
    let _ = child.wait();
}

fn describe_exit(status: ExitStatus) -> String {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(signal) = status.signal() {
            return format!("killed by signal {signal}");
        }
    }
    match status.code() {
        Some(code) => format!("exit code {code}"),
        None => "unknown status".to_string(),
    }
}

/// Puts the terminal back after the app stops.
///
/// A revue app restores the terminal when it quits or panics, but not when it
/// is killed, which is how `revue dev` stops it for a restart. So after a
/// kill `revue dev` undoes what the app may have turned on: it leaves the
/// alternate screen, shows the cursor and turns mouse reporting off. After
/// an app exits by itself that is skipped - leaving the alternate screen
/// again would move the cursor back over the app's panic message - and only
/// the line discipline is reset (on Unix, to the `stty -g` settings seen at
/// startup), which is a no-op when the app already restored it.
struct TerminalGuard {
    tty: bool,
    #[cfg(unix)]
    saved_stty: Option<String>,
}

/// What `revue::render::restore_terminal` sends, as raw ANSI: mouse capture,
/// bracketed paste and focus reporting off, colors reset, cursor shown,
/// alternate screen left.
pub const RESTORE_SEQUENCE: &str = concat!(
    "\x1b[?1006l\x1b[?1015l\x1b[?1003l\x1b[?1002l\x1b[?1000l",
    "\x1b[?2004l",
    "\x1b[?1004l",
    "\x1b[0m",
    "\x1b[?25h",
    "\x1b[?1049l",
);

impl TerminalGuard {
    fn capture() -> Self {
        let tty = std::io::stdout().is_terminal();
        Self {
            tty,
            #[cfg(unix)]
            saved_stty: if tty && std::io::stdin().is_terminal() {
                Command::new("stty")
                    .arg("-g")
                    .stdin(Stdio::inherit())
                    .output()
                    .ok()
                    .filter(|out| out.status.success())
                    .and_then(|out| String::from_utf8(out.stdout).ok())
                    .map(|s| s.trim().to_string())
            } else {
                None
            },
        }
    }

    /// Restore after the app stopped; `killed` when it did not get to clean
    /// up after itself.
    fn restore(&self, killed: bool) {
        if !self.tty {
            return;
        }
        if killed {
            let mut out = std::io::stdout();
            let _ = out.write_all(RESTORE_SEQUENCE.as_bytes());
            let _ = out.flush();
        }
        #[cfg(unix)]
        if let Some(saved) = &self.saved_stty {
            let _ = Command::new("stty")
                .arg(saved)
                .stdin(Stdio::inherit())
                .status();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_run_is_read_from_the_package() {
        let manifest = "[package]\nname = \"a\"\ndefault-run = \"main\"\n";
        assert_eq!(default_run(manifest).as_deref(), Some("main"));
        assert_eq!(default_run("[package]\nname = \"a\"\n"), None);
    }
}
