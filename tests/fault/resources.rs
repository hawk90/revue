//! External resource faults: files, clipboard tools, a file watcher and an
//! HTTP backend that misbehave.
//!
//! Injected:
//!
//! - **files**, into every loader that takes a path - `Image::from_file`,
//!   `AppBuilder::style` (stylesheet files), `AppConfig::load_from` (config
//!   files): missing, permission denied (`chmod 000`, unix, not as root), a
//!   directory, a sparse file past the loader's size limit, truncated and
//!   corrupt content, non-UTF-8 bytes, a symlink loop (unix), and a FIFO that
//!   streams more than the size limit (unix: its metadata says 0 bytes);
//! - **directories**, into `FilePicker`: missing, removed under it,
//!   unreadable, a symlink loop entry, a non-UTF-8 name (where the file
//!   system allows one), 5 000 entries; and a log file - huge, non-UTF-8 -
//!   read the way an app reads one into `LogViewer::load`;
//! - **file watching** (`hot-reload`): watching a missing path, the watched
//!   file deleted, replaced by a rename, its directory removed, two saves
//!   inside one debounce window, and `AppBuilder::hot_reload` on a stylesheet
//!   whose directory does not exist;
//! - **clipboard**: a `ClipboardBackend` that fails, the in-memory backend at
//!   its size limit, and (unix) the system backend pointed - through `PATH`,
//!   in a child process - at fake `pbcopy`/`xclip`/... tools that fail, hang,
//!   print non-UTF-8 or 20 MB, exit without reading, or are missing;
//! - **HTTP** (`HttpBackend` → `HttpClient`): backend errors, odd status codes
//!   (0, 1xx, 204 with a body, 6xx, 65535), lossy-decoded binary bodies, an
//!   8 MB body, a 100 000-deep JSON body, 10 000 headers.
//!
//! Invariants: nothing panics; a fault is reported - `Err`, an error state,
//! or nothing loaded - rather than loaded as garbage; and nothing hangs, each
//! case has a time limit.
//!
//! Case keys: `"<target> <fault>"`, e.g. `"image denied"`, `"css stream"`,
//! `"clipboard get hangs"`, `"watch rapid saves"`.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::{RenderContext, View};

use super::{eventually, ratchet, run_bounded, Case};

const LAYER: &str = "resources";
const LIMIT: Duration = Duration::from_secs(10);

/// Environment variable naming the clipboard case a child process runs.
#[cfg(unix)]
const CLIPBOARD_CHILD: &str = "REVUE_FAULT_CLIPBOARD_CASE";

// ─── Helpers ────────────────────────────────────────────────────────────────

/// A scratch directory under the target directory: inside the working
/// directory, so the hot reload watcher's path check accepts it.
fn scratch() -> tempfile::TempDir {
    tempfile::Builder::new()
        .prefix("fault-res-")
        .tempdir_in(env!("CARGO_TARGET_TMPDIR"))
        .expect("scratch dir")
}

fn paint(view: &dyn View, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    view.render(&mut ctx);
    buffer
}

fn text_of(buffer: &Buffer) -> String {
    let mut out = String::new();
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            if let Some(cell) = buffer.get(x, y) {
                out.push(cell.symbol);
            }
        }
        out.push('\n');
    }
    out
}

/// Fail if `f` took longer than `max`.
fn timed<R>(what: &str, max: Duration, f: impl FnOnce() -> R) -> Result<R, String> {
    let start = Instant::now();
    let r = f();
    let took = start.elapsed();
    if took > max {
        return Err(format!("{what} took {took:?} (max {max:?})"));
    }
    Ok(r)
}

/// Are we root? Then `chmod 000` does not deny anything.
#[cfg(unix)]
fn denial_works(path: &Path) -> bool {
    fs::read(path).is_err() && fs::read_dir(path).is_err()
}

#[cfg(unix)]
fn chmod(path: &Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt;
    let _ = fs::set_permissions(path, fs::Permissions::from_mode(mode));
}

// ─── File faults ────────────────────────────────────────────────────────────

/// A broken file a loader is pointed at.
#[derive(Clone, Copy, Debug)]
enum FileFault {
    Missing,
    Denied,
    Directory,
    /// A sparse file one byte past the loader's limit.
    Huge,
    Truncated,
    Corrupt,
    NonUtf8,
    SymlinkLoop,
    /// A FIFO streaming more than the limit; its metadata says 0 bytes.
    Stream,
}

const FILE_FAULTS: [FileFault; 9] = [
    FileFault::Missing,
    FileFault::Denied,
    FileFault::Directory,
    FileFault::Huge,
    FileFault::Truncated,
    FileFault::Corrupt,
    FileFault::NonUtf8,
    FileFault::SymlinkLoop,
    FileFault::Stream,
];

impl FileFault {
    fn key(self) -> &'static str {
        match self {
            FileFault::Missing => "missing",
            FileFault::Denied => "denied",
            FileFault::Directory => "directory",
            FileFault::Huge => "huge",
            FileFault::Truncated => "truncated",
            FileFault::Corrupt => "corrupt",
            FileFault::NonUtf8 => "non-utf8",
            FileFault::SymlinkLoop => "symlink-loop",
            FileFault::Stream => "stream",
        }
    }

    /// Can this fault be made on this platform?
    fn possible(self) -> bool {
        match self {
            FileFault::Denied | FileFault::SymlinkLoop | FileFault::Stream => cfg!(unix),
            _ => true,
        }
    }
}

/// What a loader is fed: its size limit, and good content to break.
struct Loader {
    name: &'static str,
    limit: u64,
    /// Valid content; truncated to half for `Truncated`.
    valid: Vec<u8>,
    /// Content of the right kind that does not parse.
    corrupt: Vec<u8>,
    /// Written once at the start of the stream, before the units.
    head: Vec<u8>,
    /// One unit of content, repeated to stream past the limit.
    unit: Vec<u8>,
    /// Written once at the end, so the whole stream is valid content.
    tail: Vec<u8>,
    /// `None` if the loader refused the file, `Some(what it loaded)` if not.
    load: fn(&Path) -> Option<String>,
}

/// Make the faulty file in `dir`; `None` if it cannot be made here (e.g.
/// `chmod 000` as root). The returned guard keeps a FIFO writer alive.
fn make_fault(dir: &Path, loader: &Loader, fault: FileFault) -> Option<PathBuf> {
    let path = dir.join(format!("{}.input", loader.name));
    match fault {
        FileFault::Missing => {}
        FileFault::Directory => fs::create_dir(&path).ok()?,
        FileFault::Huge => {
            let f = fs::File::create(&path).ok()?;
            f.set_len(loader.limit + 1).ok()?;
        }
        FileFault::Truncated => {
            fs::write(&path, &loader.valid[..loader.valid.len() / 2]).ok()?;
        }
        FileFault::Corrupt => fs::write(&path, &loader.corrupt).ok()?,
        FileFault::NonUtf8 => {
            let mut bytes = loader.valid.clone();
            bytes.splice(0..0, [0xff, 0xfe, 0xc3, 0x28, 0x80]);
            fs::write(&path, bytes).ok()?;
        }
        #[cfg(unix)]
        FileFault::Denied => {
            fs::write(&path, &loader.valid).ok()?;
            chmod(&path, 0o000);
            if !denial_works(&path) {
                return None;
            }
        }
        #[cfg(unix)]
        FileFault::SymlinkLoop => {
            let other = dir.join(format!("{}.loop", loader.name));
            std::os::unix::fs::symlink(&other, &path).ok()?;
            std::os::unix::fs::symlink(&path, &other).ok()?;
        }
        #[cfg(unix)]
        FileFault::Stream => {
            let status = std::process::Command::new("mkfifo")
                .arg(&path)
                .status()
                .ok()?;
            if !status.success() {
                return None;
            }
            let fifo = path.clone();
            let head = loader.head.clone();
            let unit = loader.unit.clone();
            let tail = loader.tail.clone();
            let total = loader.limit + (1 << 20);
            // The writer blocks until the loader opens the FIFO, and stops at
            // the first failed write - the loader closing its end.
            std::thread::spawn(move || {
                use std::io::Write;
                let Ok(mut f) = fs::OpenOptions::new().write(true).open(&fifo) else {
                    return;
                };
                if f.write_all(&head).is_err() {
                    return;
                }
                let mut written = head.len() as u64;
                while written < total {
                    if f.write_all(&unit).is_err() {
                        return;
                    }
                    written += unit.len() as u64;
                }
                let _ = f.write_all(&tail);
            });
        }
        #[cfg(not(unix))]
        _ => return None,
    }
    Some(path)
}

/// Every fault through one loader: each must be refused.
fn file_cases(loader: fn() -> Loader, cases: &mut Vec<Case>) {
    for fault in FILE_FAULTS {
        if !fault.possible() {
            continue;
        }
        let name = format!("{} {}", loader().name, fault.key());
        cases.push(Case::new(name, LIMIT, move || {
            let loader = loader();
            let dir = scratch();
            let path = make_fault(dir.path(), &loader, fault)?;
            let loaded = (loader.load)(&path);
            #[cfg(unix)]
            chmod(&path, 0o644);
            match (fault, loaded) {
                (_, None) => None,
                // Half a file may still be a valid one.
                (FileFault::Truncated, Some(_)) => None,
                (_, Some(what)) => Some(format!("loaded {what} from a {:?} file", fault)),
            }
        }));
    }
}

#[cfg(feature = "image")]
fn png() -> Vec<u8> {
    let img = image::RgbImage::from_fn(16, 16, |x, y| image::Rgb([x as u8 * 16, y as u8 * 16, 7]));
    let mut out = std::io::Cursor::new(Vec::new());
    img.write_to(&mut out, image::ImageFormat::Png)
        .expect("encode png");
    out.into_inner()
}

#[cfg(feature = "image")]
fn image_loader() -> Loader {
    let valid = png();
    // A PNG signature and an IHDR claiming 100 000 × 100 000, then garbage.
    let mut corrupt = valid[..8].to_vec();
    let mut ihdr = b"IHDR".to_vec();
    ihdr.extend_from_slice(&100_000u32.to_be_bytes());
    ihdr.extend_from_slice(&100_000u32.to_be_bytes());
    ihdr.extend_from_slice(&[8, 2, 0, 0, 0]);
    corrupt.extend_from_slice(&13u32.to_be_bytes());
    corrupt.extend_from_slice(&ihdr);
    corrupt.extend_from_slice(&crc32(&ihdr).to_be_bytes());
    corrupt.extend_from_slice(&[0x5a; 64]);
    Loader {
        name: "image",
        limit: 10 * 1024 * 1024,
        head: Vec::new(),
        unit: valid.clone(),
        tail: Vec::new(),
        valid,
        corrupt,
        load: |p| {
            revue::widget::Image::from_file(p)
                .ok()
                .map(|_| "an image".to_string())
        },
    }
}

#[cfg(feature = "image")]
fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &b in bytes {
        crc ^= u32::from(b);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

fn css_loader() -> Loader {
    Loader {
        name: "css",
        limit: revue::constants::MAX_CSS_FILE_SIZE,
        valid: b".a { color: red; }\n.b { color: blue; }\n".to_vec(),
        corrupt: b"\x00\x01{{{ }}} ;;; @@@ \x7f".to_vec(),
        head: Vec::new(),
        unit: b".a { color: red; }\n".to_vec(),
        tail: Vec::new(),
        load: |p| {
            let mut app = revue::core::app::App::builder()
                .size(20, 5)
                .style(p)
                .build();
            let rules = app.dom_renderer().stylesheet_mut().rules.len();
            (rules > 0).then(|| format!("{rules} CSS rules"))
        },
    }
}

#[cfg(feature = "config")]
fn config_loader() -> Loader {
    #[derive(serde::Deserialize)]
    struct Cfg {
        #[allow(dead_code)]
        name: String,
    }
    impl revue::patterns::AppConfig for Cfg {
        fn config_dir() -> &'static str {
            "revue-fault-injection"
        }
    }
    Loader {
        name: "config",
        limit: revue::constants::MAX_CONFIG_FILE_SIZE,
        valid: b"name = \"fault injection\"\n".to_vec(),
        corrupt: b"name = [[[ \"unterminated\n= = =\n".to_vec(),
        // One long string value: valid TOML, if read to the end.
        head: b"name = \"".to_vec(),
        unit: b"a".repeat(64 * 1024),
        tail: b"\"\n".to_vec(),
        load: |p| {
            use revue::patterns::AppConfig;
            Cfg::load_from(p).ok().map(|_| "a config".to_string())
        },
    }
}

// ─── Directories ────────────────────────────────────────────────────────────

/// Names the picker lists, read off the highlighted entry as it walks down.
fn picker_names(picker: &mut revue::widget::FilePicker) -> Vec<String> {
    let mut names = Vec::new();
    for _ in 0..10_000 {
        match picker.highlighted_entry() {
            Some(e) if names.last() != Some(&e.name) => names.push(e.name.clone()),
            _ => break,
        }
        picker.highlight_next();
    }
    names
}

fn picker_cases(cases: &mut Vec<Case>) {
    use revue::widget::FilePicker;

    cases.push(Case::new("picker missing", LIMIT, || {
        let dir = scratch();
        let missing = dir.path().join("nope");
        let mut picker = FilePicker::new().try_set_start_dir(&missing).ok()?;
        picker.refresh();
        let _ = paint(&picker, 60, 20);
        let _ = picker.enter();
        picker.go_up();
        None
    }));

    cases.push(Case::new("picker removed", LIMIT, || {
        let dir = scratch();
        let sub = dir.path().join("sub");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("a.txt"), "a").unwrap();
        let mut picker = FilePicker::new().start_dir(&sub);
        fs::remove_dir_all(&sub).unwrap();
        picker.refresh();
        let _ = paint(&picker, 60, 20);
        let _ = picker.enter();
        if picker.highlighted_entry().is_some() {
            return Some("a removed directory still lists entries".into());
        }
        None
    }));

    #[cfg(unix)]
    cases.push(Case::new("picker denied", LIMIT, || {
        let dir = scratch();
        let locked = dir.path().join("locked");
        fs::create_dir(&locked).unwrap();
        fs::write(locked.join("secret.txt"), "s").unwrap();
        chmod(&locked, 0o000);
        if !denial_works(&locked) {
            chmod(&locked, 0o755);
            return None;
        }
        let mut picker = FilePicker::new().start_dir(dir.path());
        let result = picker.navigate_to(&locked);
        let shown = picker.current_dir().to_path_buf();
        let _ = paint(&picker, 60, 20);
        chmod(&locked, 0o755);
        match result {
            Err(_) => None,
            Ok(()) if shown.ends_with("locked") => {
                Some("navigate_to an unreadable directory returned Ok and shows it as empty".into())
            }
            Ok(()) => None,
        }
    }));

    #[cfg(unix)]
    cases.push(Case::new("picker symlink-loop", LIMIT, || {
        let dir = scratch();
        std::os::unix::fs::symlink(dir.path().join("b"), dir.path().join("a")).unwrap();
        std::os::unix::fs::symlink(dir.path().join("a"), dir.path().join("b")).unwrap();
        fs::write(dir.path().join("real.txt"), "r").unwrap();
        let mut picker = FilePicker::new().start_dir(dir.path());
        let names = picker_names(&mut picker);
        let _ = paint(&picker, 60, 20);
        if !names.iter().any(|n| n == "real.txt") {
            return Some(format!("a symlink loop hid the real file: {names:?}"));
        }
        None
    }));

    cases.push(Case::new("picker non-utf8", LIMIT, || {
        let dir = scratch();
        #[cfg(unix)]
        let made = {
            use std::os::unix::ffi::OsStrExt;
            let name = std::ffi::OsStr::from_bytes(b"bad-\xff\xfe-name.txt");
            fs::write(dir.path().join(name), "x").is_ok()
        };
        #[cfg(not(unix))]
        let made = false;
        if !made {
            // The file system insists on valid names (APFS, NTFS).
            return None;
        }
        let mut picker = FilePicker::new().start_dir(dir.path());
        let names = picker_names(&mut picker);
        let _ = paint(&picker, 60, 20);
        if !names.iter().any(|n| n.starts_with("bad-")) {
            return Some(format!(
                "a file whose name is not UTF-8 is not listed: {names:?}"
            ));
        }
        None
    }));

    cases.push(Case::new("picker 5000-entries", LIMIT, || {
        let dir = scratch();
        for i in 0..5000 {
            fs::write(dir.path().join(format!("f{i:05}.txt")), "").unwrap();
        }
        let picker = timed("listing 5000 entries", Duration::from_secs(3), || {
            FilePicker::new().start_dir(dir.path())
        });
        match picker {
            Ok(picker) => {
                let _ = paint(&picker, 60, 20);
                None
            }
            Err(e) => Some(e),
        }
    }));

    cases.push(Case::new("logviewer huge-file", LIMIT, || {
        let dir = scratch();
        let path = dir.path().join("app.log");
        let mut bytes = Vec::with_capacity(20 << 20);
        let mut i = 0u64;
        while bytes.len() < 20 << 20 {
            bytes.extend_from_slice(
                format!("2024-01-01T00:00:{:02} INFO line {i} ", i % 60).as_bytes(),
            );
            bytes.extend_from_slice(b"\xff\xfe\n");
            bytes.extend_from_slice(b"\xc3\x28 broken utf8 \x00 nul\n");
            i += 1;
        }
        fs::write(&path, &bytes).unwrap();
        let text = String::from_utf8_lossy(&fs::read(&path).unwrap()).into_owned();
        let result = timed("loading a 20 MB log", Duration::from_secs(8), || {
            let mut viewer = revue::widget::LogViewer::new();
            viewer.load(&text);
            let _ = paint(&viewer, 80, 24);
            viewer
        });
        result.err()
    }));
}

// ─── File watching ──────────────────────────────────────────────────────────

#[cfg(feature = "hot-reload")]
fn watch_cases(cases: &mut Vec<Case>) {
    use revue::core::app::{HotReload, HotReloadConfig, HotReloadEvent};

    fn watcher(dir: &Path, debounce: Duration) -> HotReload {
        let mut hr = HotReload::with_config(HotReloadConfig {
            debounce,
            recursive: false,
        })
        .expect("watcher");
        hr.watch(dir).expect("watch dir");
        // Let the backend settle before the first change.
        std::thread::sleep(Duration::from_millis(200));
        while hr.poll().is_some() {}
        hr
    }

    /// Poll until an event naming `file` arrives, for at most `limit`.
    fn event_for(hr: &mut HotReload, file: &Path, limit: Duration) -> Option<HotReloadEvent> {
        let name = file.file_name().unwrap().to_owned();
        let mut found = None;
        eventually(limit, || {
            while let Some(e) = hr.poll() {
                let p = match &e {
                    HotReloadEvent::StylesheetChanged(p)
                    | HotReloadEvent::FileCreated(p)
                    | HotReloadEvent::FileDeleted(p) => p.clone(),
                    HotReloadEvent::Error(_) => continue,
                };
                if p.file_name() == Some(&name) {
                    found = Some(e);
                    return true;
                }
            }
            false
        });
        found
    }

    cases.push(Case::new("watch missing", LIMIT, || {
        let dir = scratch();
        let mut hr = HotReload::new().expect("watcher");
        match hr.watch(dir.path().join("missing.css")) {
            Err(_) => None,
            Ok(()) => Some("watching a missing path returned Ok".into()),
        }
    }));

    cases.push(Case::new("watch deleted", LIMIT, || {
        let dir = scratch();
        let file = dir.path().join("a.css");
        fs::write(&file, ".a{}").unwrap();
        let mut hr = watcher(dir.path(), Duration::from_millis(10));
        fs::remove_file(&file).unwrap();
        let _ = event_for(&mut hr, &file, Duration::from_secs(3));
        // The watcher keeps working: the file coming back is seen.
        fs::write(&file, ".b{}").unwrap();
        if event_for(&mut hr, &file, Duration::from_secs(3)).is_none() {
            return Some("after a delete, re-creating the file raised no event".into());
        }
        None
    }));

    cases.push(Case::new("watch renamed-over", LIMIT, || {
        let dir = scratch();
        let file = dir.path().join("a.css");
        fs::write(&file, ".a{}").unwrap();
        let mut hr = watcher(dir.path(), Duration::from_millis(10));
        let tmp = dir.path().join("a.css.tmp");
        fs::write(&tmp, ".b{}").unwrap();
        fs::rename(&tmp, &file).unwrap();
        if event_for(&mut hr, &file, Duration::from_secs(3)).is_none() {
            return Some("a file replaced by a rename raised no event for it".into());
        }
        None
    }));

    cases.push(Case::new("watch dir-removed", LIMIT, || {
        let dir = scratch();
        let sub = dir.path().join("styles");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("a.css"), ".a{}").unwrap();
        let mut hr = watcher(&sub, Duration::from_millis(10));
        fs::remove_dir_all(&sub).unwrap();
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(500) {
            let _ = hr.poll();
            let _ = hr.css_changed();
            std::thread::sleep(Duration::from_millis(10));
        }
        let _ = hr.wait_timeout(Duration::from_millis(50));
        let _ = hr.unwatch(&sub);
        None
    }));

    cases.push(Case::new("watch rapid-saves", LIMIT, || {
        // Two saves inside one debounce window: the second must still be
        // reported once the window has passed, or a reload reads the first
        // save and the last one is never applied.
        let dir = scratch();
        let file = dir.path().join("a.css");
        fs::write(&file, ".a{}").unwrap();
        let mut hr = watcher(dir.path(), Duration::from_millis(1500));
        fs::write(&file, ".a{color:red}").unwrap();
        event_for(&mut hr, &file, Duration::from_secs(3))?;
        // Let the backend deliver every event of the first save, then save
        // again inside the window.
        std::thread::sleep(Duration::from_millis(300));
        while hr.poll().is_some() {}
        fs::write(&file, ".a{color:blue}").unwrap();
        if event_for(&mut hr, &file, Duration::from_secs(5)).is_none() {
            return Some("a save inside the debounce window was dropped, never reported".into());
        }
        None
    }));

    cases.push(Case::new("watch app-missing-dir", LIMIT, || {
        let dir = scratch();
        let mut app = revue::core::app::App::builder()
            .size(20, 5)
            .style(dir.path().join("no/such/dir/a.css"))
            .css(".x { color: red; }")
            .hot_reload(true)
            .build();
        let rules = app.dom_renderer().stylesheet_mut().rules.len();
        (rules != 1).then(|| format!("expected the 1 inline rule, have {rules}"))
    }));
}

// ─── Clipboard ──────────────────────────────────────────────────────────────

fn clipboard_cases(cases: &mut Vec<Case>) {
    use revue::utils::clipboard::{
        Clipboard, ClipboardBackend, ClipboardError, ClipboardResult, MemoryClipboard,
    };

    struct Failing;
    impl ClipboardBackend for Failing {
        fn set(&self, _: &str) -> ClipboardResult<()> {
            Err(ClipboardError::CommandFailed("injected".into()))
        }
        fn get(&self) -> ClipboardResult<String> {
            Err(ClipboardError::Io(std::io::Error::other("injected")))
        }
        fn has_text(&self) -> ClipboardResult<bool> {
            Err(ClipboardError::NoClipboardTool)
        }
        fn clear(&self) -> ClipboardResult<()> {
            Err(ClipboardError::InvalidUtf8)
        }
    }

    cases.push(Case::new("clipboard failing-backend", LIMIT, || {
        let c = Clipboard::with_backend(Failing);
        let all_err =
            c.set("x").is_err() && c.get().is_err() && c.has_text().is_err() && c.clear().is_err();
        (!all_err).then(|| "a failing backend's error did not reach the caller".into())
    }));

    cases.push(Case::new("clipboard memory-limit", LIMIT, || {
        let max = revue::constants::MAX_CLIPBOARD_SIZE;
        let c = Clipboard::with_backend(MemoryClipboard::new());
        c.set("keep").ok()?;
        if c.set(&"x".repeat(max + 1)).is_ok() {
            return Some("content past MAX_CLIPBOARD_SIZE was accepted".into());
        }
        if c.get().ok().as_deref() != Some("keep") {
            return Some("a refused set changed the content".into());
        }
        // Escapes and control characters never reach the clipboard.
        c.set("a\x1b[31mb\x07c\u{0}").ok()?;
        let got = c.get().ok()?;
        (got != "abc").then(|| format!("control characters stored: {got:?}"))
    }));

    cases.push(Case::new("clipboard memory-threads", LIMIT, || {
        let c = MemoryClipboard::new();
        let threads: Vec<_> = (0..8)
            .map(|t| {
                let c = c.clone();
                std::thread::spawn(move || {
                    for i in 0..500 {
                        let _ = c.set(&format!("{t}-{i}"));
                        let _ = c.get();
                    }
                })
            })
            .collect();
        for t in threads {
            if t.join().is_err() {
                return Some("a thread panicked".into());
            }
        }
        None
    }));

    #[cfg(unix)]
    for &(name, _, _) in SYSTEM_CLIPBOARD {
        let key = format!("clipboard {name}");
        let child_case = name.to_string();
        cases.push(Case::new(key, Duration::from_secs(40), move || {
            let dir = scratch();
            let bin = dir.path().join("bin");
            fs::create_dir(&bin).unwrap();
            let (_, script, _) = SYSTEM_CLIPBOARD
                .iter()
                .find(|(n, _, _)| *n == child_case)
                .unwrap();
            if let Some(script) = script {
                use std::os::unix::fs::PermissionsExt;
                let body = format!("#!/bin/sh\n[ \"$1\" = \"--version\" ] && exit 0\n{script}\n");
                for tool in ["pbcopy", "pbpaste", "xclip", "xsel", "wl-copy", "wl-paste"] {
                    let p = bin.join(tool);
                    fs::write(&p, &body).unwrap();
                    fs::set_permissions(&p, fs::Permissions::from_mode(0o755)).unwrap();
                }
            }
            super::run_in_child(
                "fault::resources::resource_faults",
                &[
                    (CLIPBOARD_CHILD, std::ffi::OsStr::new(&child_case)),
                    ("PATH", bin.as_os_str()),
                ],
                Duration::from_secs(35),
            )
        }));
    }
}

/// System clipboard cases: name, the fake tool's script (`None`: no tool at
/// all), and the check the child runs with the fake tools first in `PATH`.
#[cfg(unix)]
type ClipboardCheck = fn() -> Option<String>;

#[cfg(unix)]
const SYSTEM_CLIPBOARD: &[(&str, Option<&str>, ClipboardCheck)] = &[
    ("get fails", Some("exit 3"), || {
        expect_err("paste", revue::utils::clipboard::paste())
    }),
    ("set fails", Some("/bin/cat >/dev/null; exit 3"), || {
        expect_err("copy", revue::utils::clipboard::copy("x"))
    }),
    ("get hangs", Some("exec /bin/sleep 60"), || {
        expect_err("paste", revue::utils::clipboard::paste())
    }),
    ("set hangs", Some("exec /bin/sleep 60"), || {
        expect_err("copy", revue::utils::clipboard::copy("x"))
    }),
    ("set unread", Some("exit 0"), || {
        expect_err("copy", revue::utils::clipboard::copy(&"y".repeat(1 << 20)))
    }),
    (
        "get non-utf8",
        Some("printf '\\377\\376abc'"),
        || match revue::utils::clipboard::paste() {
            Err(revue::utils::clipboard::ClipboardError::InvalidUtf8) => None,
            other => Some(format!("expected InvalidUtf8, got {other:?}")),
        },
    ),
    (
        "get huge",
        Some("exec /usr/bin/head -c 20000000 /dev/zero"),
        || expect_err("paste", revue::utils::clipboard::paste()),
    ),
    ("tool missing", None, || {
        expect_err("copy", revue::utils::clipboard::copy("x"))
            .or_else(|| expect_err("paste", revue::utils::clipboard::paste()))
    }),
];

#[cfg(unix)]
fn expect_err<T: std::fmt::Debug>(
    what: &str,
    r: revue::utils::clipboard::ClipboardResult<T>,
) -> Option<String> {
    match r {
        Err(_) => None,
        Ok(v) => Some(format!("{what} returned Ok({v:?})")),
    }
}

// ─── HTTP ───────────────────────────────────────────────────────────────────

fn http_cases(cases: &mut Vec<Case>) {
    use revue::widget::{
        HttpBackend, HttpClient, HttpRequest, HttpResponse, RequestState, ResponseView,
    };

    /// A backend that answers with a fixed result.
    struct Fixed(Result<HttpResponse, String>);
    impl HttpBackend for Fixed {
        fn send(&self, _: &HttpRequest) -> Result<HttpResponse, String> {
            self.0.clone()
        }
    }

    fn response(status: u16, content_type: &str, body: String) -> HttpResponse {
        HttpResponse {
            status,
            status_text: "Whatever".into(),
            headers: [("Content-Type".to_string(), content_type.to_string())]
                .into_iter()
                .collect(),
            size: body.len(),
            body,
            time: Duration::from_millis(5),
        }
    }

    /// Send through `backend` as an app does, then draw every view, scrolled
    /// to the top and far past the end.
    fn exercise(backend: &dyn HttpBackend) -> Result<HttpClient, String> {
        let mut client = HttpClient::new().url("https://example.test/x");
        match backend.send(client.request()) {
            Ok(r) => client.set_response(r),
            Err(e) => client.set_error(e),
        }
        timed("drawing the response", Duration::from_secs(3), || {
            for view in [ResponseView::Body, ResponseView::Headers, ResponseView::Raw] {
                client.set_view(view);
                for _ in 0..3 {
                    let _ = paint(&client, 100, 30);
                }
                client.scroll_down(usize::MAX);
                let _ = paint(&client, 100, 30);
                client.scroll_up(usize::MAX);
                client.toggle_headers();
                let _ = paint(&client, 100, 30);
            }
        })?;
        Ok(client)
    }

    cases.push(Case::new("http error", LIMIT, || {
        let client = match exercise(&Fixed(Err("connection reset by peer".into()))) {
            Ok(c) => c,
            Err(e) => return Some(e),
        };
        if client.state() != RequestState::Error || client.error().is_none() {
            return Some("a backend error left no error state".into());
        }
        let screen = text_of(&paint(&client, 100, 30));
        (!screen.contains("connection reset")).then(|| "the error is not on screen".into())
    }));

    for status in [0u16, 100, 204, 302, 399, 600, 999, u16::MAX] {
        cases.push(Case::new(
            format!("http status-{status}"),
            LIMIT,
            move || {
                let client = match exercise(&Fixed(Ok(response(
                    status,
                    "application/json",
                    "{\"a\": 1}".into(),
                )))) {
                    Ok(c) => c,
                    Err(e) => return Some(e),
                };
                let success = (200..300).contains(&status);
                let state = client.state();
                if success != (state == RequestState::Success) {
                    return Some(format!("status {status} left state {state:?}"));
                }
                None
            },
        ));
    }

    cases.push(Case::new("http binary-body", LIMIT, || {
        let bytes: Vec<u8> = (0..4096u32)
            .map(|i| (i.wrapping_mul(2_654_435_761) >> 24) as u8)
            .collect();
        let body = String::from_utf8_lossy(&bytes).into_owned();
        exercise(&Fixed(Ok(response(200, "application/octet-stream", body)))).err()
    }));

    cases.push(Case::new("http huge-body", LIMIT, || {
        let body = "0123456789abcdef ".repeat(8 << 20 >> 4);
        exercise(&Fixed(Ok(response(200, "text/plain", body)))).err()
    }));

    cases.push(Case::new("http deep-json", LIMIT, || {
        let body = format!("{}{}", "[".repeat(100_000), "]".repeat(100_000));
        exercise(&Fixed(Ok(response(200, "application/json", body)))).err()
    }));

    cases.push(Case::new("http many-headers", LIMIT, || {
        let mut r = response(200, "text/plain", "ok".into());
        for i in 0..10_000 {
            r.headers.insert(format!("X-H-{i}"), "v".repeat(i % 300));
        }
        r.headers.insert("X-Huge".into(), "h".repeat(1 << 20));
        exercise(&Fixed(Ok(r))).err()
    }));
}

// ─── The layer ──────────────────────────────────────────────────────────────

fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    #[cfg(feature = "image")]
    file_cases(image_loader, &mut cases);
    file_cases(css_loader, &mut cases);
    #[cfg(feature = "config")]
    file_cases(config_loader, &mut cases);
    picker_cases(&mut cases);
    #[cfg(feature = "hot-reload")]
    watch_cases(&mut cases);
    clipboard_cases(&mut cases);
    http_cases(&mut cases);
    cases
}

#[test]
fn resource_faults() {
    // The child side of a system clipboard case: run that one check with the
    // fake tools first in PATH and report through the exit status.
    #[cfg(unix)]
    if let Ok(name) = std::env::var(CLIPBOARD_CHILD) {
        let (_, _, check) = SYSTEM_CLIPBOARD
            .iter()
            .find(|(n, _, _)| *n == name)
            .expect("known clipboard case");
        if let Some(failure) = check() {
            panic!("CHILD: {failure}");
        }
        return;
    }

    let (ran, failures) = run_bounded(cases());
    ratchet(LAYER, &ran, failures);
}
