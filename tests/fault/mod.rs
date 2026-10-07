//! Shared machinery for the fault-injection layers: catching panics quietly,
//! running a case in a child process when it may take the process down, and
//! the known-failures ratchet.
//!
//! The ratchet: a layer fails if it sees a failure that is not listed in
//! [`known_failures::KNOWN`], **or** if a listed failure no longer happens.
//! The list can only shrink.

pub mod concurrency;
pub mod events;
pub mod known_failures;
pub mod output;
pub mod parsers;
pub mod resources;

use std::cell::{Cell as StdCell, RefCell};
use std::collections::BTreeSet;
use std::panic::{self, AssertUnwindSafe};
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex, Once};
use std::time::{Duration, Instant};

/// One observed failure: a stable case key and what went wrong.
pub struct Failure {
    pub case: String,
    pub detail: String,
}

impl Failure {
    pub fn new(case: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            case: case.into(),
            detail: detail.into(),
        }
    }
}

thread_local! {
    static QUIET: StdCell<bool> = const { StdCell::new(false) };
    static LOCATION: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Install (once) a panic hook that stays silent while a case runs on this
/// thread and defers to the previous hook otherwise.
fn install_quiet_hook() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let previous = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            if QUIET.with(StdCell::get) {
                let at = info
                    .location()
                    .map(|l| format!("{}:{}", l.file(), l.line()))
                    .unwrap_or_default();
                LOCATION.with(|l| *l.borrow_mut() = at);
            } else {
                previous(info);
            }
        }));
    });
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    let msg = if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "<non-string panic payload>".to_string()
    };
    msg.lines().next().unwrap_or("").chars().take(160).collect()
}

/// Run `f` with panics caught and the hook silenced; `Err` is
/// `"<message> @ <file:line>"`.
pub fn catch<R>(f: impl FnOnce() -> R) -> Result<R, String> {
    install_quiet_hook();
    let was = QUIET.with(|q| q.replace(true));
    let result = panic::catch_unwind(AssertUnwindSafe(f));
    QUIET.with(|q| q.set(was));
    result.map_err(|p| {
        let at = LOCATION.with(|l| std::mem::take(&mut *l.borrow_mut()));
        format!("panic: {} @ {at}", panic_message(&*p))
    })
}

/// Shorten a string for a failure report.
pub fn preview(s: &str) -> String {
    let mut out: String = s.chars().take(60).collect();
    if s.chars().count() > 60 {
        out.push_str(&format!("... ({} chars)", s.chars().count()));
    }
    format!("{out:?}")
}

/// Compare what a layer observed with the known-failures list for that layer
/// and panic with a readable report on any difference.
///
/// `ran` is every case key the layer ran. A listed case that did not run -
/// its target is behind a feature that is off - is neither new nor fixed.
pub fn ratchet(layer: &str, ran: &BTreeSet<String>, failures: Vec<Failure>) {
    let cases_run = ran.len();
    let known: BTreeSet<&str> = known_failures::KNOWN
        .iter()
        .filter(|k| k.layer == layer)
        .flat_map(|k| k.cases.iter().copied())
        .filter(|case| ran.contains(*case))
        .collect();
    let seen: BTreeSet<&str> = failures.iter().map(|f| f.case.as_str()).collect();

    let new: Vec<&Failure> = failures
        .iter()
        .filter(|f| !known.contains(f.case.as_str()))
        .collect();
    let fixed: Vec<&&str> = known.difference(&seen).collect();

    eprintln!(
        "fault injection [{layer}]: {cases_run} cases, {} failing ({} known)",
        failures.len(),
        failures.len() - new.len()
    );
    if new.is_empty() && fixed.is_empty() {
        return;
    }

    let mut report = String::new();
    if !new.is_empty() {
        report.push_str(&format!(
            "\n{} NEW failure(s) in layer `{layer}` (fix the library, or list them in tests/fault/known_failures.rs):\n\n",
            new.len()
        ));
        for f in &new {
            report.push_str(&format!("  {:<44} {}\n", f.case, f.detail));
        }
        let cases: Vec<&str> = new.iter().map(|f| f.case.as_str()).collect();
        report.push_str(&format!(
            "\nSuggested entry:\n    Known {{ layer: {layer:?}, reason: \"TODO\", cases: &{cases:?} }},\n"
        ));
    }
    if !fixed.is_empty() {
        report.push_str(&format!(
            "\n{} known failure(s) in layer `{layer}` no longer happen - remove them from tests/fault/known_failures.rs:\n\n",
            fixed.len()
        ));
        for case in fixed {
            let reason = known_failures::KNOWN
                .iter()
                .find(|k| k.layer == layer && k.cases.contains(case))
                .map_or("", |k| k.reason);
            report.push_str(&format!("  {case:<44} ({reason})\n"));
        }
    }
    panic!("{report}");
}

// ─── Bounded cases ──────────────────────────────────────────────────────────
//
// The resource and concurrency layers share this runner: every case runs on
// its own thread (so thread-local reactive state starts clean and a case that
// deadlocks is reported and abandoned instead of stalling the layer), all
// cases run at once, and each has its own time limit.

/// A check: `None` if every invariant held, `Some(detail)` otherwise.
pub type Check = Box<dyn FnOnce() -> Option<String> + Send>;

/// One case of a bounded layer.
pub struct Case {
    pub name: String,
    /// A case still running after this is reported as hung.
    pub limit: Duration,
    pub check: Check,
}

impl Case {
    pub fn new(
        name: impl Into<String>,
        limit: Duration,
        check: impl FnOnce() -> Option<String> + Send + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            limit,
            check: Box::new(check),
        }
    }
}

/// Run every case concurrently, each on its own thread with panics caught,
/// and return the set of case keys that ran with the failures.
pub fn run_bounded(cases: Vec<Case>) -> (BTreeSet<String>, Vec<Failure>) {
    let start = Instant::now();
    let (tx, rx) = mpsc::channel();
    let mut pending: Vec<(String, Duration)> = Vec::new();
    for (i, case) in cases.into_iter().enumerate() {
        let tx = tx.clone();
        pending.push((case.name.clone(), case.limit));
        let check = case.check;
        std::thread::Builder::new()
            .name(format!("fault-{}", case.name))
            .stack_size(8 << 20)
            .spawn(move || {
                let started = Instant::now();
                let detail = catch(check).unwrap_or_else(Some);
                let _ = tx.send((i, detail, started.elapsed()));
            })
            .expect("spawn case thread");
    }
    drop(tx);

    let ran: BTreeSet<String> = pending.iter().map(|(n, _)| n.clone()).collect();
    let mut failures = Vec::new();
    let mut done = vec![false; pending.len()];
    let mut took: Vec<(Duration, usize)> = Vec::new();
    loop {
        let open: Vec<usize> = (0..pending.len()).filter(|&i| !done[i]).collect();
        if open.is_empty() {
            break;
        }
        let deadline = open.iter().map(|&i| pending[i].1).max().unwrap_or_default();
        let left = deadline.saturating_sub(start.elapsed());
        if left.is_zero() {
            break;
        }
        match rx.recv_timeout(left) {
            Ok((i, detail, elapsed)) => {
                done[i] = true;
                took.push((elapsed, i));
                if let Some(detail) = detail {
                    failures.push(Failure::new(pending[i].0.clone(), detail));
                }
            }
            Err(_) => break,
        }
    }
    for (i, (name, limit)) in pending.iter().enumerate() {
        if !done[i] {
            failures.push(Failure::new(
                name.clone(),
                format!("hang: did not finish within {limit:?}"),
            ));
        }
    }
    took.sort_by(|a, b| b.cmp(a));
    let slowest: Vec<String> = took
        .iter()
        .take(4)
        .map(|(d, i)| format!("{} {:.1}s", pending[*i].0, d.as_secs_f32()))
        .collect();
    eprintln!("  slowest: {}", slowest.join(", "));
    failures.sort_by(|a, b| a.case.cmp(&b.case));
    (ran, failures)
}

/// Wait until `cond` holds, polling, for at most `limit`; whether it held.
pub fn eventually(limit: Duration, mut cond: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + limit;
    loop {
        if cond() {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

/// A gate a background task blocks on: a task that "never finishes" within
/// the test, but does let go eventually so no thread outlives the run by long.
#[derive(Clone, Default)]
pub struct Gate(Arc<(Mutex<bool>, Condvar)>);

impl Gate {
    /// How long a closed gate holds a waiter at most.
    const CAP: Duration = Duration::from_secs(30);

    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&self) {
        let (lock, cvar) = &*self.0;
        *lock.lock().unwrap_or_else(|e| e.into_inner()) = true;
        cvar.notify_all();
    }

    pub fn wait(&self) {
        let (lock, cvar) = &*self.0;
        let guard = lock.lock().unwrap_or_else(|e| e.into_inner());
        let _ = cvar.wait_timeout_while(guard, Self::CAP, |open| !*open);
    }
}

/// Run one case of `test` (a `--exact` test path) in a child process of this
/// binary with `env` set, for a case that may take the process down (a stack
/// overflow aborts) or must run with a changed environment. `None` if the
/// child exited successfully within `limit`.
pub fn run_in_child(
    test: &str,
    env: &[(&str, &std::ffi::OsStr)],
    limit: Duration,
) -> Option<String> {
    use std::process::{Command, Stdio};

    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args([test, "--exact", "--test-threads=1", "--nocapture"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());
    for (key, value) in env {
        command.env(key, value);
    }
    let mut child = command.spawn().expect("spawn child");
    let deadline = Instant::now() + limit;
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            if status.success() {
                return None;
            }
            let stderr = child
                .wait_with_output()
                .map(|o| String::from_utf8_lossy(&o.stderr).into_owned())
                .unwrap_or_default();
            let lines: Vec<&str> = stderr.lines().collect();
            let why = ["CHILD:", "overflow", "panicked"]
                .iter()
                .find_map(|key| lines.iter().find(|l| l.contains(key)))
                .copied()
                .unwrap_or("")
                .chars()
                .take(200)
                .collect::<String>();
            return Some(format!("child failed ({status}): {why}"));
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Some(format!("hang: child still running after {limit:?}"));
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}
