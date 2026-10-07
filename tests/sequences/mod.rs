//! Shared machinery for the event-sequence layers: the steps a sequence is
//! made of, running one sequence against a widget with the invariants
//! checked after every step, the proptest driver, and the known-failures
//! ratchet.
//!
//! The invariants, after every step:
//!
//! 1. the step does not panic;
//! 2. rendering at the current size does not panic and writes nothing
//!    outside the area (the widget matrix's sentinel check);
//! 3. the widget's own cheap invariant holds (its catalog entry's `check`:
//!    selection inside the items, cursor inside the text, value inside its
//!    range, ...).
//!
//! The ratchet: a layer fails if a widget fails with a sequence that is not
//! listed in [`known_failures::KNOWN`], **or** if a listed widget/sequence no
//! longer fails. The list can only shrink.

pub mod catalog;
pub mod keys;
pub mod known_failures;
pub mod mouse;

#[path = "../matrix/sentinel.rs"]
pub mod sentinel;

use std::collections::BTreeSet;
use std::fmt;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use proptest::prelude::*;
use proptest::test_runner::{Config, RngAlgorithm, TestCaseError, TestRng, TestRunner};

use revue::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use revue::layout::Rect;

use catalog::Entry;
use sentinel::{catch, render_checked};

// ─── Steps ──────────────────────────────────────────────────────────────────

/// The key alphabet every widget is driven with: navigation, editing,
/// confirmation, a few printable characters (ASCII, digits, 한글, emoji, vim
/// commands) and a few modified keys.
pub const KEYS: &[KeyEvent] = &[
    k(Key::Down),
    k(Key::Up),
    k(Key::Left),
    k(Key::Right),
    k(Key::Home),
    k(Key::End),
    k(Key::PageUp),
    k(Key::PageDown),
    k(Key::Tab),
    k(Key::BackTab),
    k(Key::Enter),
    k(Key::Escape),
    k(Key::Char(' ')),
    k(Key::Backspace),
    k(Key::Delete),
    k(Key::Char('a')),
    k(Key::Char('x')),
    k(Key::Char('1')),
    k(Key::Char('0')),
    k(Key::Char('-')),
    k(Key::Char('+')),
    k(Key::Char('/')),
    k(Key::Char(':')),
    k(Key::Char('한')),
    k(Key::Char('😀')),
    k(Key::Char('j')),
    k(Key::Char('k')),
    k(Key::Char('h')),
    k(Key::Char('l')),
    k(Key::Char('g')),
    k(Key::Char('G')),
    k(Key::Char('i')),
    k(Key::Char('v')),
    k(Key::Char('d')),
    k(Key::Char('n')),
    k(Key::Char('y')),
    k(Key::Char('p')),
    k(Key::Char('u')),
    k(Key::Char('w')),
    k(Key::Char('q')),
    shift(Key::Left),
    shift(Key::Right),
    shift(Key::Up),
    shift(Key::Down),
    ctrl(Key::Char('a')),
    ctrl(Key::Char('z')),
    ctrl(Key::Char('y')),
    ctrl(Key::Char('d')),
    ctrl(Key::Char('f')),
    ctrl(Key::Left),
    ctrl(Key::Right),
    ctrl(Key::Home),
    ctrl(Key::End),
    alt(Key::Down),
    alt(Key::Up),
];

const fn k(key: Key) -> KeyEvent {
    KeyEvent {
        key,
        ctrl: false,
        alt: false,
        shift: false,
    }
}

const fn shift(key: Key) -> KeyEvent {
    KeyEvent {
        key,
        ctrl: false,
        alt: false,
        shift: true,
    }
}

const fn ctrl(key: Key) -> KeyEvent {
    KeyEvent {
        key,
        ctrl: true,
        alt: false,
        shift: false,
    }
}

const fn alt(key: Key) -> KeyEvent {
    KeyEvent {
        key,
        ctrl: false,
        alt: true,
        shift: false,
    }
}

fn key_name(ev: &KeyEvent) -> String {
    let mut s = String::new();
    if ev.ctrl {
        s.push_str("Ctrl+");
    }
    if ev.alt {
        s.push_str("Alt+");
    }
    if ev.shift {
        s.push_str("Shift+");
    }
    match ev.key {
        Key::Char(c) => s.push_str(&format!("'{c}'")),
        other => s.push_str(&format!("{other:?}")),
    }
    s
}

/// Sizes a `Render` step switches to (the widget is rendered after every
/// step at the current size; the first is `40x10`).
pub const SIZES: &[(u16, u16)] = &[
    (40, 10),
    (0, 0),
    (1, 1),
    (2, 1),
    (5, 2),
    (12, 3),
    (20, 6),
    (80, 24),
    (7, 40),
];

/// Mouse event kinds a `Mouse` step sends.
pub const MOUSE_KINDS: &[MouseEventKind] = &[
    MouseEventKind::Down(MouseButton::Left),
    MouseEventKind::Up(MouseButton::Left),
    MouseEventKind::Drag(MouseButton::Left),
    MouseEventKind::ScrollDown,
    MouseEventKind::ScrollUp,
    MouseEventKind::Move,
    MouseEventKind::Down(MouseButton::Right),
    MouseEventKind::ScrollLeft,
    MouseEventKind::ScrollRight,
];

/// A mouse coordinate on one axis, relative to the current area.
#[derive(Clone, Copy, Debug)]
pub enum Coord {
    /// The first cell of the area.
    Start,
    /// The middle of the area.
    Mid,
    /// The last cell of the area.
    Last,
    /// One past the last cell.
    End,
    /// One before the first cell.
    Before,
    /// Absolute 0.
    Zero,
    /// `u16::MAX`.
    Max,
    /// The area start plus an offset (may lie past the end).
    Plus(u16),
}

impl Coord {
    fn resolve(self, start: u16, len: u16) -> u16 {
        match self {
            Coord::Start => start,
            Coord::Mid => start.saturating_add(len / 2),
            Coord::Last => start.saturating_add(len).saturating_sub(1),
            Coord::End => start.saturating_add(len),
            Coord::Before => start.saturating_sub(1),
            Coord::Zero => 0,
            Coord::Max => u16::MAX,
            Coord::Plus(n) => start.saturating_add(n),
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Coord::Plus(n) => write!(f, "+{n}"),
            other => write!(f, "{}", format!("{other:?}").to_lowercase()),
        }
    }
}

/// One step of a sequence.
#[derive(Clone, Debug)]
pub enum Op {
    /// Send `KEYS[i]`.
    Key(usize),
    /// Apply the widget's `i`-th data mutation.
    Mutate(usize),
    /// Switch the render size to `SIZES[i]`.
    Render(usize),
    /// Send `MOUSE_KINDS[kind]` at `(x, y)`.
    Mouse { kind: usize, x: Coord, y: Coord },
}

/// Area of a `w`×`h` render: a 2-cell margin left/right and 1 row
/// above/below, so a write past any edge lands in the sentinel margin.
fn area_for((w, h): (u16, u16)) -> Rect {
    Rect::new(2, 1, w, h)
}

fn describe(op: &Op, mutations: &[&'static str]) -> String {
    match op {
        Op::Key(i) => key_name(&KEYS[*i]),
        Op::Mutate(i) => format!("<{}>", mutations[*i]),
        Op::Render(i) => format!("[{}x{}]", SIZES[*i].0, SIZES[*i].1),
        Op::Mouse { kind, x, y } => format!("{:?}@({x},{y})", MOUSE_KINDS[*kind]),
    }
}

/// A sequence as a stable one-line string: steps joined with a space.
pub fn describe_all(ops: &[Op], mutations: &[&'static str]) -> String {
    ops.iter()
        .map(|op| describe(op, mutations))
        .collect::<Vec<_>>()
        .join(" ")
}

// ─── Running a sequence ─────────────────────────────────────────────────────

/// Render at the current size and check the widget's own invariant.
fn check_step(subject: &dyn catalog::Subject, size: (u16, u16)) -> Result<(), String> {
    let area = area_for(size);
    if let Some(view) = subject.view() {
        match render_checked(view, (size.0 + 4, size.1 + 2), area, true) {
            Ok(None) => {}
            Ok(Some(detail)) => return Err(format!("out-of-area: {detail}")),
            Err(msg) => return Err(format!("render panicked: {msg}")),
        }
    }
    match catch(|| subject.check(area)) {
        Ok(Ok(())) => Ok(()),
        Ok(Err(msg)) => Err(format!("invariant: {msg}")),
        Err(msg) => Err(format!("check panicked: {msg}")),
    }
}

/// Run one sequence on a freshly built widget; `Err` names the failing step.
pub fn run_sequence(entry: &Entry, ops: &[Op]) -> Result<(), String> {
    let mut subject = catch(entry.build).map_err(|m| format!("build panicked: {m}"))?;
    let mutations = subject.mutations();
    let mut size = SIZES[0];
    check_step(&*subject, size).map_err(|m| format!("before any step: {m}"))?;
    for (i, op) in ops.iter().enumerate() {
        let area = area_for(size);
        let step = || format!("step {} ({})", i + 1, describe(op, &mutations));
        catch(|| match *op {
            Op::Key(k) => subject.key(&KEYS[k], area),
            Op::Mutate(m) => subject.mutate(m),
            Op::Render(_) => {}
            Op::Mouse { kind, x, y } => {
                let ev = MouseEvent::new(
                    x.resolve(area.x, area.width),
                    y.resolve(area.y, area.height),
                    MOUSE_KINDS[kind],
                );
                subject.mouse(&ev, area);
            }
        })
        .map_err(|m| format!("{} panicked: {m}", step()))?;
        if let Op::Render(s) = op {
            size = SIZES[*s];
        }
        check_step(&*subject, size).map_err(|m| format!("after {}: {m}", step()))?;
    }
    Ok(())
}

// ─── The proptest driver ────────────────────────────────────────────────────

/// Relative weights of the step kinds in a layer.
pub struct Weights {
    pub key: u32,
    pub mutate: u32,
    pub render: u32,
    pub mouse: u32,
}

/// Longest generated sequence.
const MAX_LEN: usize = 64;

fn coord() -> impl Strategy<Value = Coord> {
    prop_oneof![
        Just(Coord::Start),
        Just(Coord::Mid),
        Just(Coord::Last),
        Just(Coord::End),
        Just(Coord::Before),
        Just(Coord::Zero),
        Just(Coord::Max),
        (0u16..90).prop_map(Coord::Plus),
    ]
}

fn op_strategy(w: &Weights, mutations: usize) -> BoxedStrategy<Op> {
    let mut choices: Vec<(u32, BoxedStrategy<Op>)> = Vec::new();
    if w.key > 0 {
        choices.push((w.key, (0..KEYS.len()).prop_map(Op::Key).boxed()));
    }
    if w.mutate > 0 && mutations > 0 {
        choices.push((w.mutate, (0..mutations).prop_map(Op::Mutate).boxed()));
    }
    if w.render > 0 {
        choices.push((w.render, (0..SIZES.len()).prop_map(Op::Render).boxed()));
    }
    if w.mouse > 0 {
        choices.push((
            w.mouse,
            (0..MOUSE_KINDS.len(), coord(), coord())
                .prop_map(|(kind, x, y)| Op::Mouse { kind, x, y })
                .boxed(),
        ));
    }
    proptest::strategy::Union::new_weighted(choices).boxed()
}

fn runner(cases: u32) -> TestRunner {
    let config = Config {
        cases,
        failure_persistence: None,
        max_shrink_iters: 2048,
        rng_algorithm: RngAlgorithm::ChaCha,
        ..Config::default()
    };
    TestRunner::new_with_rng(config, TestRng::deterministic_rng(RngAlgorithm::ChaCha))
}

/// One observed failure: the widget, the shrunk sequence, what went wrong.
pub struct Failure {
    pub widget: &'static str,
    pub sequence: String,
    pub detail: String,
}

/// Drive one widget with `cases` random sequences; the shrunk failure, if any.
fn drive(entry: &Entry, weights: &Weights, cases: u32) -> Option<Failure> {
    let mutations = match catch(entry.build) {
        Ok(s) => s.mutations(),
        Err(m) => {
            return Some(Failure {
                widget: entry.name,
                sequence: String::new(),
                detail: format!("build panicked: {m}"),
            })
        }
    };
    let strategy = proptest::collection::vec(op_strategy(weights, mutations.len()), 1..=MAX_LEN);
    let mut runner = runner(cases);
    match runner.run(&strategy, |ops| {
        run_sequence(entry, &ops).map_err(TestCaseError::fail)
    }) {
        Ok(()) => None,
        Err(proptest::test_runner::TestError::Fail(why, ops)) => Some(Failure {
            widget: entry.name,
            sequence: describe_all(&ops, &mutations),
            detail: why.message().to_string(),
        }),
        Err(e) => Some(Failure {
            widget: entry.name,
            sequence: String::new(),
            detail: e.to_string(),
        }),
    }
}

/// A widget that has not finished after this is reported as hung.
const HANG: Duration = Duration::from_secs(120);

/// Drive every entry on its own thread (so one that hangs is reported and
/// abandoned instead of stalling the layer) and apply the ratchet.
pub fn run_layer(layer: &'static str, entries: Vec<Entry>, weights: Weights, cases: u32) {
    let weights = std::sync::Arc::new(weights);
    let names: Vec<&'static str> = entries.iter().map(|e| e.name).collect();
    let (tx, rx) = mpsc::channel();
    for (i, entry) in entries.into_iter().enumerate() {
        let tx = tx.clone();
        let weights = weights.clone();
        std::thread::Builder::new()
            .name(format!("seq-{}", entry.name))
            .stack_size(8 << 20)
            .spawn(move || {
                let _ = tx.send((i, drive(&entry, &weights, cases)));
            })
            .expect("spawn sequence thread");
    }
    drop(tx);

    let deadline = Instant::now() + HANG;
    let mut done = vec![false; names.len()];
    let mut failures = Vec::new();
    while done.iter().any(|d| !d) {
        match rx.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
            Ok((i, f)) => {
                done[i] = true;
                failures.extend(f);
            }
            Err(_) => break,
        }
    }
    for (i, name) in names.iter().enumerate() {
        if !done[i] {
            failures.push(Failure {
                widget: name,
                sequence: "hang".to_string(),
                detail: format!("did not finish within {HANG:?}"),
            });
        }
    }
    failures.sort_by(|a, b| a.widget.cmp(b.widget));
    ratchet(layer, &names, cases, failures);
}

// ─── The ratchet ────────────────────────────────────────────────────────────

/// Compare what a layer observed with the known-failures list for that layer
/// and panic with a readable report on any difference. A listed widget that
/// did not run (behind a feature that is off) is neither new nor fixed.
fn ratchet(layer: &str, ran: &[&'static str], cases: u32, failures: Vec<Failure>) {
    let known: BTreeSet<(&str, &str)> = known_failures::KNOWN
        .iter()
        .filter(|k| k.layer == layer && ran.contains(&k.widget))
        .map(|k| (k.widget, k.sequence))
        .collect();
    let seen: BTreeSet<(&str, &str)> = failures
        .iter()
        .map(|f| (f.widget, f.sequence.as_str()))
        .collect();
    let new: Vec<&Failure> = failures
        .iter()
        .filter(|f| !known.contains(&(f.widget, f.sequence.as_str())))
        .collect();
    let fixed: Vec<&(&str, &str)> = known.difference(&seen).collect();

    eprintln!(
        "event sequences [{layer}]: {} widgets x {cases} sequences, {} failing ({} known)",
        ran.len(),
        failures.len(),
        failures.len() - new.len()
    );
    if new.is_empty() && fixed.is_empty() {
        return;
    }

    let mut report = String::new();
    if !new.is_empty() {
        report.push_str(&format!(
            "\n{} NEW failure(s) in layer `{layer}` (fix the widget, or list them in tests/sequences/known_failures.rs):\n\n",
            new.len()
        ));
        for f in &new {
            report.push_str(&format!(
                "  {:<18} {}\n  {:<18}   -> {}\n",
                f.widget, f.sequence, "", f.detail
            ));
        }
        report.push_str("\nSuggested entries:\n");
        for f in &new {
            report.push_str(&format!(
                "    Known {{ layer: {layer:?}, widget: {:?}, sequence: {:?}, reason: \"TODO\" }},\n",
                f.widget, f.sequence
            ));
        }
    }
    if !fixed.is_empty() {
        report.push_str(&format!(
            "\n{} known failure(s) in layer `{layer}` no longer happen - remove them from tests/sequences/known_failures.rs:\n\n",
            fixed.len()
        ));
        for (widget, sequence) in fixed {
            let reason = known_failures::KNOWN
                .iter()
                .find(|k| k.layer == layer && k.widget == *widget && k.sequence == *sequence)
                .map_or("", |k| k.reason);
            report.push_str(&format!(
                "  {widget:<18} {sequence}\n  {:<18}   ({reason})\n",
                ""
            ));
        }
    }
    panic!("{report}");
}
