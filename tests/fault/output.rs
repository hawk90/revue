//! Output faults: `Terminal` and `CrosstermBackend` driven through
//! init -> several draws -> restore over writers that misbehave.
//!
//! Injected faults, each against a clean run's write and flush counts:
//!
//! - `fail-write#N` - the Nth `write` call errors (every N up to the clean
//!   run's count), every other call succeeds;
//! - `fail-flush#N` - the Nth `flush` errors;
//! - `interrupted#N` - the Nth `write` returns `Interrupted` once;
//! - `short` - every `write` accepts at most one byte;
//! - `zero` - every `write` returns `Ok(0)`.
//!
//! Each runs in two modes: `bail` stops at the first error and drops the
//! terminal (what `?` in an app does), `continue` keeps calling and then
//! restores explicitly.
//!
//! Invariants:
//!
//! 1. nothing panics;
//! 2. an injected error is returned by the call it happened in - not
//!    swallowed (`fail-*`, `zero`);
//! 3. retryable faults are invisible: every call succeeds and the bytes equal
//!    the clean run's (`interrupted`, `short`);
//! 4. no runaway loop: a scenario stays under a write budget;
//! 5. with a real TTY (unix, via a PTY): after the scenario raw mode is off,
//!    every terminal mode the output switched on (alternate screen, mouse,
//!    bracketed paste, focus events, hidden cursor) was switched off again
//!    afterwards, and the alternate screen was left at most once.
//!
//! Raw mode needs a TTY, so on unix the layer re-runs this test binary as a
//! child with a pseudo-terminal on stdin (crossterm puts stdin in raw mode
//! when it is a TTY). Elsewhere it runs in-process without `init`, and the
//! restore checks are skipped - restore is a no-op outside raw mode.
//!
//! Case keys: `"<terminal|backend> <fault> <bail|continue>"`.

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::io::{self, Write};
use std::rc::Rc;

use revue::layout::Rect;
use revue::render::{Backend, Buffer, Cell, CrosstermBackend, Modifier, Terminal};
use revue::style::Color;

use super::{catch, ratchet, Failure};

const LAYER: &str = "output";

/// Above this many `write` calls a scenario is a runaway loop.
const WRITE_BUDGET: usize = 1_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fault {
    None,
    FailWrite(usize),
    FailFlush(usize),
    Interrupted(usize),
    Short,
    Zero,
}

impl Fault {
    fn label(self) -> String {
        match self {
            Fault::None => "clean".into(),
            Fault::FailWrite(n) => format!("fail-write#{n}"),
            Fault::FailFlush(n) => format!("fail-flush#{n}"),
            Fault::Interrupted(n) => format!("interrupted#{n}"),
            Fault::Short => "short".into(),
            Fault::Zero => "zero".into(),
        }
    }

    /// The fault must surface as an error from the call it happened in.
    fn must_surface(self) -> bool {
        matches!(
            self,
            Fault::FailWrite(_) | Fault::FailFlush(_) | Fault::Zero
        )
    }

    /// The fault must be invisible: same calls succeed, same bytes.
    fn must_be_transparent(self) -> bool {
        matches!(self, Fault::None | Fault::Interrupted(_) | Fault::Short)
    }
}

/// What the writer saw, shared so it outlives the terminal (whose `Drop`
/// still writes).
#[derive(Default)]
struct Log {
    /// Bytes accepted - what reached the terminal.
    bytes: Vec<u8>,
    /// Bytes accepted plus bytes offered and refused, in order - what the
    /// library *tried* to send.
    attempted: Vec<u8>,
    writes: usize,
    flushes: usize,
    /// The step currently running.
    step: &'static str,
    /// Steps during which a fault was injected.
    fired_in: BTreeSet<&'static str>,
    runaway: bool,
}

struct FaultWriter {
    log: Rc<RefCell<Log>>,
    fault: Fault,
}

impl Write for FaultWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut log = self.log.borrow_mut();
        log.writes += 1;
        let n = log.writes;
        if n > WRITE_BUDGET {
            log.runaway = true;
            return Err(io::Error::other("write budget exhausted"));
        }
        let step = log.step;
        let accept = match self.fault {
            Fault::FailWrite(k) if k == n => {
                log.fired_in.insert(step);
                log.attempted.extend_from_slice(buf);
                return Err(io::Error::other("injected write failure"));
            }
            Fault::Interrupted(k) if k == n => {
                return Err(io::Error::from(io::ErrorKind::Interrupted));
            }
            Fault::Zero if !buf.is_empty() => {
                log.fired_in.insert(step);
                log.attempted.extend_from_slice(buf);
                return Ok(0);
            }
            Fault::Short => buf.len().min(1),
            _ => buf.len(),
        };
        log.bytes.extend_from_slice(&buf[..accept]);
        log.attempted.extend_from_slice(&buf[..accept]);
        Ok(accept)
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut log = self.log.borrow_mut();
        log.flushes += 1;
        if let Fault::FailFlush(k) = self.fault {
            if k == log.flushes {
                let step = log.step;
                log.fired_in.insert(step);
                return Err(io::Error::other("injected flush failure"));
            }
        }
        Ok(())
    }
}

/// Frame `i`: changing text, wide characters, colors, modifiers, a link.
fn frame(i: usize) -> Buffer {
    let (w, h) = (16, 4);
    let mut buf = Buffer::new(w, h);
    let hue = (i * 60) as u8;
    buf.put_str_styled(
        0,
        0,
        &format!("frame {i}"),
        Some(Color::rgb(hue, 200, 255 - hue)),
        Some(Color::rgb(10, 20, hue)),
    );
    buf.put_str(i as u16 % 3, 1, "한글漢字ｗ");
    if i.is_multiple_of(2) {
        buf.set(0, 2, Cell::new('B').bold().fg(Color::RED));
        buf.set(1, 2, Cell::new('i').italic().underline());
        buf.set(2, 2, Cell::new('r').reverse().dim());
    }
    buf.put_hyperlink(4, 2, "link", "https://example.com", Some(Color::CYAN), None);
    buf.put_str(0, 3, &"=".repeat(i % 16));
    buf
}

/// One scenario run: per-step results and what the writer saw.
struct Run {
    results: Vec<(&'static str, Result<(), String>)>,
    log: Log,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Bail,
    Continue,
}

impl Mode {
    fn label(self) -> &'static str {
        match self {
            Mode::Bail => "bail",
            Mode::Continue => "continue",
        }
    }
}

type Step<T> = (&'static str, fn(&mut T) -> Result<(), String>);

fn err<E: std::fmt::Display>(r: Result<(), E>) -> Result<(), String> {
    r.map_err(|e| e.to_string())
}

/// Drive `target` through `steps`, then restore (explicitly in `continue`
/// mode) and drop it.
fn drive<T>(
    log: &Rc<RefCell<Log>>,
    mut target: T,
    steps: &[Step<T>],
    restore: fn(&mut T) -> Result<(), String>,
    mode: Mode,
) -> Vec<(&'static str, Result<(), String>)> {
    let mut results = Vec::new();
    for (name, step) in steps {
        log.borrow_mut().step = name;
        let r = step(&mut target);
        let failed = r.is_err();
        results.push((*name, r));
        if failed && mode == Mode::Bail {
            break;
        }
    }
    if mode == Mode::Continue {
        log.borrow_mut().step = "restore";
        results.push(("restore", restore(&mut target)));
    }
    log.borrow_mut().step = "drop";
    drop(target);
    results
}

fn terminal_steps(tty: bool) -> Vec<Step<Terminal<FaultWriter>>> {
    let mut steps: Vec<Step<Terminal<FaultWriter>>> = Vec::new();
    if tty {
        steps.push(("init", |t| err(t.init_with_mouse(true))));
    }
    let rest: &[Step<Terminal<FaultWriter>>] = &[
        ("render#0", |t| err(t.render(&frame(0)))),
        ("render#1", |t| err(t.render(&frame(1)))),
        ("set_cursor", |t| err(t.set_cursor(3, 2))),
        ("show_cursor", |t| err(t.show_cursor())),
        ("hide_cursor", |t| err(t.hide_cursor())),
        ("force_redraw#2", |t| err(t.force_redraw(&frame(2)))),
        ("render_dirty#3", |t| {
            err(t.render_dirty(&frame(3), &[Rect::new(0, 0, 16, 2)]))
        }),
        ("clear", |t| err(t.clear())),
        ("render#4", |t| err(t.render(&frame(4)))),
    ];
    steps.extend_from_slice(rest);
    steps
}

fn backend_steps(tty: bool) -> Vec<Step<CrosstermBackend<FaultWriter>>> {
    let mut steps: Vec<Step<CrosstermBackend<FaultWriter>>> = Vec::new();
    if tty {
        steps.push(("init", |b| err(b.init_with_mouse(true))));
    }
    let rest: &[Step<CrosstermBackend<FaultWriter>>] = &[
        ("set_cursor", |b| err(b.set_cursor(1, 1))),
        ("set_fg", |b| err(b.set_fg(Color::rgb(1, 2, 3)))),
        ("set_bg", |b| err(b.set_bg(Color::BLUE))),
        ("set_modifier", |b| {
            err(b.set_modifier(Modifier::BOLD | Modifier::ITALIC | Modifier::UNDERLINE))
        }),
        (
            "write_all",
            |b| err(b.write_all("héllo 한글 ｗ".as_bytes())),
        ),
        ("reset_style", |b| err(b.reset_style())),
        ("flush", |b| err(Write::flush(b))),
        ("clear", |b| err(b.clear())),
        ("hide_cursor", |b| err(b.hide_cursor())),
        ("show_cursor", |b| err(b.show_cursor())),
        ("disable_mouse", |b| err(b.disable_mouse())),
        ("enable_mouse", |b| err(b.enable_mouse())),
        ("reset_fg", |b| err(b.reset_fg())),
        ("flush#2", |b| err(Write::flush(b))),
    ];
    steps.extend_from_slice(rest);
    steps
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Target {
    Terminal,
    Backend,
}

impl Target {
    fn label(self) -> &'static str {
        match self {
            Target::Terminal => "terminal",
            Target::Backend => "backend",
        }
    }
}

fn run(target: Target, fault: Fault, mode: Mode, tty: bool) -> Result<Run, String> {
    let log = Rc::new(RefCell::new(Log::default()));
    let writer = FaultWriter {
        log: Rc::clone(&log),
        fault,
    };
    let results = catch(|| match target {
        Target::Terminal => drive(
            &log,
            Terminal::with_size(writer, 16, 4),
            &terminal_steps(tty),
            |t| err(t.restore()),
            mode,
        ),
        Target::Backend => drive(
            &log,
            CrosstermBackend::new(writer),
            &backend_steps(tty),
            |b| err(b.restore()),
            mode,
        ),
    });
    let log = std::mem::take(&mut *log.borrow_mut());
    // A panic may have left raw mode on; turn it off so the next case starts
    // clean (the panic itself is already the failure).
    if tty && results.is_err() {
        let _ = crossterm::terminal::disable_raw_mode();
    }
    results.map(|results| Run { results, log })
}

/// Terminal modes the output can switch on, and what switches each off.
const MODES: &[(&str, &str, &str)] = &[
    ("alternate screen", "\x1b[?1049h", "\x1b[?1049l"),
    ("mouse capture", "\x1b[?1000h", "\x1b[?1000l"),
    ("SGR mouse", "\x1b[?1006h", "\x1b[?1006l"),
    ("bracketed paste", "\x1b[?2004h", "\x1b[?2004l"),
    ("focus events", "\x1b[?1004h", "\x1b[?1004l"),
    ("hidden cursor", "\x1b[?25l", "\x1b[?25h"),
];

fn rfind(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).rposition(|w| w == needle)
}

fn count(hay: &[u8], needle: &[u8]) -> usize {
    hay.windows(needle.len()).filter(|w| *w == needle).count()
}

/// Check one run against the invariants; `None` if they all hold.
fn check(run: &Run, fault: Fault, clean: &Run, tty: bool) -> Option<String> {
    let log = &run.log;
    if log.runaway {
        return Some(format!(
            "runaway: more than {WRITE_BUDGET} writes (in step {})",
            log.step
        ));
    }
    if fault.must_surface() {
        for (step, result) in &run.results {
            if log.fired_in.contains(step) && result.is_ok() {
                return Some(format!(
                    "`{step}` returned Ok after an injected {}",
                    fault.label()
                ));
            }
        }
    }
    if fault.must_be_transparent() {
        if let Some((step, Err(e))) = run.results.iter().find(|(_, r)| r.is_err()) {
            return Some(format!("`{step}` failed: {e}"));
        }
        if log.bytes != clean.log.bytes {
            return Some(format!(
                "output differs from the clean run ({} vs {} bytes)",
                log.bytes.len(),
                clean.log.bytes.len()
            ));
        }
    }
    if tty {
        if crossterm::terminal::is_raw_mode_enabled().unwrap_or(false) {
            let _ = crossterm::terminal::disable_raw_mode();
            return Some("raw mode still on after restore".into());
        }
        // `bytes` is what the terminal saw; `attempted` also holds what the
        // library tried to send - a restore that was refused still counts as
        // attempted.
        for (name, on, off) in MODES {
            if !log.bytes.windows(on.len()).any(|w| w == on.as_bytes()) {
                continue;
            }
            let last_on = rfind(&log.attempted, on.as_bytes()).unwrap_or(0);
            if !log.attempted[last_on..]
                .windows(off.len())
                .any(|w| w == off.as_bytes())
            {
                return Some(format!("{name} switched on and never switched off"));
            }
        }
        let leaves = count(&log.attempted, b"\x1b[?1049l");
        if leaves > 1 {
            return Some(format!("left the alternate screen {leaves} times"));
        }
    }
    None
}

/// Every case of the layer: the clean runs, then every fault against them.
fn run_layer(tty: bool) -> (BTreeSet<String>, Vec<Failure>) {
    let mut cases = BTreeSet::new();
    let mut failures = Vec::new();
    for target in [Target::Terminal, Target::Backend] {
        for mode in [Mode::Bail, Mode::Continue] {
            let key =
                |fault: Fault| format!("{} {} {}", target.label(), fault.label(), mode.label());
            cases.insert(key(Fault::None));
            let clean = match run(target, Fault::None, mode, tty) {
                Ok(clean) => clean,
                Err(panic) => {
                    failures.push(Failure::new(key(Fault::None), panic));
                    continue;
                }
            };
            if let Some(detail) = check(&clean, Fault::None, &clean, tty) {
                failures.push(Failure::new(key(Fault::None), detail));
                continue;
            }
            let (writes, flushes) = (clean.log.writes, clean.log.flushes);
            assert!(
                writes > 0 && flushes > 0,
                "the clean run must write and flush"
            );

            let faults = (1..=writes)
                .flat_map(|n| [Fault::FailWrite(n), Fault::Interrupted(n)])
                .chain((1..=flushes).map(Fault::FailFlush))
                .chain([Fault::Short, Fault::Zero]);
            for fault in faults {
                cases.insert(key(fault));
                let detail = match run(target, fault, mode, tty) {
                    Ok(run) => check(&run, fault, &clean, tty),
                    Err(panic) => Some(panic),
                };
                if let Some(detail) = detail {
                    failures.push(Failure::new(key(fault), detail));
                }
            }
        }
    }
    (cases, failures)
}

#[cfg(unix)]
mod pty {
    //! Running the layer in a child whose stdin is a pseudo-terminal.

    use std::collections::BTreeSet;
    use std::fs::{File, OpenOptions};
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    use super::super::Failure;

    /// Set in the child: where to write the results.
    pub const CHILD_ENV: &str = "REVUE_FAULT_OUTPUT_CHILD";

    /// Open a PTY pair: the master end and the slave end.
    fn open_pty() -> (File, File) {
        // SAFETY: plain libc calls on descriptors this function owns; `ptsname`
        // returns a NUL-terminated string copied out immediately.
        let (master, slave_path) = unsafe {
            let master = libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY);
            assert!(master >= 0, "posix_openpt failed");
            assert_eq!(libc::grantpt(master), 0, "grantpt failed");
            assert_eq!(libc::unlockpt(master), 0, "unlockpt failed");
            let name = libc::ptsname(master);
            assert!(!name.is_null(), "ptsname failed");
            let slave = std::ffi::CStr::from_ptr(name).to_str().unwrap().to_owned();
            (File::from_raw_fd(master), PathBuf::from(slave))
        };
        let slave = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&slave_path)
            .expect("open PTY slave");
        let size = libc::winsize {
            ws_row: 24,
            ws_col: 80,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        // SAFETY: TIOCSWINSZ reads a `winsize` from a valid pointer.
        unsafe { libc::ioctl(slave.as_raw_fd(), libc::TIOCSWINSZ, &size) };
        (master, slave)
    }

    /// The child side: run the layer and write the results to `path`.
    pub fn child(path: &Path) {
        let (cases, failures) = super::run_layer(true);
        let mut out = String::new();
        for case in cases {
            out.push_str(&format!("case\t{case}\n"));
        }
        for f in failures {
            out.push_str(&format!(
                "fail\t{}\t{}\n",
                f.case,
                f.detail.replace('\n', " ")
            ));
        }
        std::fs::write(path, out).expect("write child results");
    }

    /// The parent side: run the child on a PTY and read its results back.
    pub fn parent(test_name: &str) -> (BTreeSet<String>, Vec<Failure>) {
        let (master, slave) = open_pty();
        let results = std::env::temp_dir().join(format!(
            "revue-fault-output-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_file(&results);

        let mut child = Command::new(std::env::current_exe().unwrap())
            .args([test_name, "--exact", "--nocapture", "--test-threads=1"])
            .env(CHILD_ENV, &results)
            .stdin(Stdio::from(slave))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn child");

        let deadline = Instant::now() + Duration::from_secs(120);
        loop {
            if child.try_wait().unwrap().is_some() {
                break;
            }
            if Instant::now() >= deadline {
                let _ = child.kill();
                panic!("output layer child did not finish within 120 s");
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let output = child.wait_with_output().unwrap();
        drop(master);

        let Ok(text) = std::fs::read_to_string(&results) else {
            panic!(
                "output layer child produced no results ({:?}):\n{}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            );
        };
        let _ = std::fs::remove_file(&results);
        let cases = text
            .lines()
            .filter_map(|l| l.strip_prefix("case\t"))
            .map(str::to_owned)
            .collect();
        let failures = text
            .lines()
            .filter_map(|l| l.strip_prefix("fail\t")?.split_once('\t'))
            .map(|(case, detail)| Failure::new(case, detail))
            .collect();
        (cases, failures)
    }
}

#[test]
fn output_faults() {
    #[cfg(unix)]
    {
        if let Some(path) = std::env::var_os(pty::CHILD_ENV) {
            pty::child(std::path::Path::new(&path));
            return;
        }
        let (cases, failures) = pty::parent("fault::output::output_faults");
        ratchet(LAYER, &cases, failures);
    }
    #[cfg(not(unix))]
    {
        let (cases, failures) = run_layer(false);
        ratchet(LAYER, &cases, failures);
    }
}

/// Without a TTY the same scenarios still run - only the restore checks need
/// raw mode. This keeps the draw paths covered where no PTY exists, and on
/// unix it is the in-process twin of the PTY run.
#[test]
fn output_faults_without_a_tty() {
    let (cases, failures) = run_layer(false);
    ratchet("output-no-tty", &cases, failures);
}
