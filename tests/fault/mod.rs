//! Shared machinery for the fault-injection layers: catching panics quietly,
//! running a case in a child process when it may take the process down, and
//! the known-failures ratchet.
//!
//! The ratchet: a layer fails if it sees a failure that is not listed in
//! [`known_failures::KNOWN`], **or** if a listed failure no longer happens.
//! The list can only shrink.

pub mod events;
pub mod known_failures;
pub mod output;
pub mod parsers;

use std::cell::{Cell as StdCell, RefCell};
use std::collections::BTreeSet;
use std::panic::{self, AssertUnwindSafe};
use std::sync::Once;

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
