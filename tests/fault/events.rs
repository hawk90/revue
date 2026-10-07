//! Event anomalies: degenerate and hostile input pushed through the real `App`
//! event path (`PipelineHarness::send`) and, as a user handler would, into a
//! bench of interactive widgets - input, textarea, list, table, tabs, select
//! and a scroll view.
//!
//! Injected:
//!
//! - resizes to 0×0, 1×1, w×0, 0×h, 16384 on one side, past the buffer
//!   limits (16384 per side, 10 M cells) up to `u16::MAX`, and shrink-then-grow;
//! - mouse events of every kind at the screen's corners, just outside it, far
//!   outside it and at `u16::MAX`;
//! - a 1 MB paste, and a paste of control characters;
//! - a burst of 10 000 key events, with odd keys (F0/F24/F255, `Null`,
//!   `Unknown`, modifiers-only, NUL/ESC/ZWJ/combining characters);
//! - focus lost/gained and resizes interleaved with a burst.
//!
//! Invariants:
//!
//! 1. nothing panics, and nothing takes the process down (the cases that ask
//!    for more than the buffer limits run in a child process with a timeout);
//! 2. the app keeps rendering: after the case, a resize back to 40×12 and a
//!    draw put the bench's title on screen;
//! 3. widget invariants: every selection index is in range, the input cursor
//!    is within its text, the textarea cursor is within its lines, the scroll
//!    offset is within the content.
//!
//! Case keys: `"<group> <detail>"`, e.g. `"resize 0x0"`, `"mouse scroll-down
//! at 65535,65535"`, `"burst 10000 keys"`.

use std::time::{Duration, Instant};

use revue::event::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use revue::testing::PipelineHarness;
use revue::widget::{
    vstack, Column, Input, List, RenderContext, ScrollView, Select, Table, Tabs, Text, TextArea,
    View,
};

use super::{catch, ratchet, Failure};

const LAYER: &str = "events";
const TITLE: &str = "fault-bench";
const W: u16 = 40;
const H: u16 = 12;

/// The bench: one of each interactive widget, rendered top to bottom.
#[derive(Clone)]
struct Bench {
    tabs: Tabs,
    input: Input,
    select: Select,
    list: List<String>,
    table: Table,
    textarea: TextArea,
    scroll: ScrollView,
    /// Rows the app currently has, as a handler would track from `Resize`.
    rows: u16,
}

impl Bench {
    fn new() -> Self {
        let items: Vec<String> = (0..20).map(|i| format!("item {i} 한글")).collect();
        Self {
            tabs: Tabs::new().tabs(vec!["One", "Two", "Three"]),
            input: Input::new().value("hello").placeholder("type"),
            select: Select::new().options(items.clone()),
            list: List::new(items.clone()).selected(1),
            table: Table::new(vec![Column::new("Name"), Column::new("Value")])
                .rows(items.iter().map(|s| vec![s.clone(), "v".into()]).collect())
                .selected(1),
            textarea: TextArea::new()
                .content("line one\nline two 漢字\n")
                .line_numbers(true),
            scroll: ScrollView::new().content_height(50).scroll_offset(1),
            rows: H,
        }
    }

    /// Route an event the way an application's handler would.
    fn handle(&mut self, event: &Event) {
        match event {
            Event::Key(key) => {
                self.tabs.handle_key(&key.key);
                self.input.handle_key_event(key);
                self.select.handle_key(&key.key);
                self.list.handle_key(&key.key);
                match key.key {
                    Key::Up => self.table.select_prev(),
                    Key::Down => self.table.select_next(),
                    Key::Home => self.table.select_first(),
                    Key::End => self.table.select_last(),
                    Key::PageDown => self.table.page_down(self.rows as usize),
                    Key::PageUp => self.table.page_up(self.rows as usize),
                    _ => {}
                }
                self.textarea.handle_key_event(key);
                self.scroll.handle_key(&key.key, self.rows);
            }
            Event::Mouse(mouse) => {
                use revue::widget::traits::Interactive;
                let area = revue::layout::Rect::new(0, 0, W, self.rows);
                self.textarea.handle_mouse(mouse, area);
                self.select.handle_mouse(mouse, area);
                self.scroll.handle_mouse(mouse, self.rows);
                if mouse.is_left_click() {
                    // A click on a table row selects that row.
                    self.table.jump_to(mouse.y as usize);
                }
            }
            Event::Paste(text) => {
                let joined = format!("{}{}", self.input.text(), text);
                self.input.set_value(joined);
                self.textarea.insert_str(text);
            }
            Event::Resize(_, h) => self.rows = *h,
            _ => {}
        }
    }

    /// Widget invariants that are cheap to check; `None` if all hold.
    fn violation(&self) -> Option<String> {
        let in_range = |name: &str, idx: usize, len: usize| {
            (len > 0 && idx >= len).then(|| format!("{name} selection {idx} out of 0..{len}"))
        };
        if let Some(v) = in_range("tabs", self.tabs.selected_index(), self.tabs.len())
            .or_else(|| in_range("list", self.list.selected_index(), self.list.len()))
            .or_else(|| in_range("table", self.table.selected_index(), self.table.row_count()))
            .or_else(|| in_range("select", self.select.selected_index(), self.select.len()))
        {
            return Some(v);
        }
        let chars = self.input.text().chars().count();
        if self.input.cursor() > chars {
            return Some(format!(
                "input cursor {} past {chars} chars",
                self.input.cursor()
            ));
        }
        let (line, col) = self.textarea.cursor_position();
        let lines = self.textarea.line_count();
        if line >= lines.max(1) {
            return Some(format!("textarea cursor line {line} past {lines} lines"));
        }
        let content = self.textarea.get_content();
        let len = content
            .split('\n')
            .nth(line)
            .map_or(0, |l| l.chars().count());
        if col > len {
            return Some(format!(
                "textarea cursor col {col} past line {line} ({len} chars)"
            ));
        }
        if self.scroll.offset() > 50 {
            return Some(format!(
                "scroll offset {} past content 50",
                self.scroll.offset()
            ));
        }
        None
    }
}

impl View for Bench {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .child(Text::new(TITLE))
            .child(self.tabs.clone())
            .child(self.input.clone())
            .child(self.select.clone())
            .child(self.list.clone())
            .child(self.table.clone())
            .child(self.textarea.clone())
            .child(self.scroll.clone())
            .render(ctx);
    }
}

/// A harness and bench, driven together.
struct Rig {
    harness: PipelineHarness,
    bench: Bench,
}

impl Rig {
    fn new() -> Self {
        let mut harness = PipelineHarness::new(W, H).tab_navigation(true);
        let bench = Bench::new();
        harness.draw(&bench);
        Self { harness, bench }
    }

    fn send(&mut self, event: Event) {
        self.bench.handle(&event);
        self.harness.send(event, &mut self.bench);
    }

    fn draw(&mut self) {
        self.harness.draw(&self.bench);
    }

    /// Back to 40×12, draw, and check the bench is on screen and sound.
    fn verdict(mut self) -> Option<String> {
        self.send(Event::Resize(W, H));
        self.draw();
        if !self.harness.contains(TITLE) {
            return Some(format!(
                "stopped rendering: {TITLE:?} not on screen after resizing back"
            ));
        }
        self.bench.violation()
    }
}

fn key(k: Key) -> Event {
    Event::Key(KeyEvent::new(k))
}

fn key_mod(k: Key, ctrl: bool, alt: bool, shift: bool) -> Event {
    Event::Key(KeyEvent {
        key: k,
        ctrl,
        alt,
        shift,
    })
}

/// A small deterministic generator (xorshift), so bursts are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// Odd keys: what a terminal may report that a widget rarely sees.
fn odd_keys() -> Vec<Event> {
    let mut keys = vec![
        key(Key::F(0)),
        key(Key::F(24)),
        key(Key::F(255)),
        key(Key::Null),
        key(Key::Unknown),
        key_mod(Key::Unknown, true, false, false),
        key_mod(Key::Unknown, true, true, true),
        key_mod(Key::Null, false, true, false),
        key_mod(Key::BackTab, false, false, true),
        key_mod(Key::Tab, false, false, true),
        key_mod(Key::Insert, true, true, false),
    ];
    for ch in [
        '\0', '\u{1b}', '\u{7f}', '\u{200d}', '\u{301}', '\u{feff}', '\u{ffff}', '\r',
    ] {
        keys.push(key(Key::Char(ch)));
        keys.push(key_mod(Key::Char(ch), true, true, false));
    }
    keys
}

/// One random key of a burst: mostly editing and navigation, sometimes odd.
fn burst_key(rng: &mut Rng, odd: &[Event]) -> Event {
    const NAV: &[Key] = &[
        Key::Up,
        Key::Down,
        Key::Left,
        Key::Right,
        Key::Home,
        Key::End,
        Key::PageUp,
        Key::PageDown,
        Key::Enter,
        Key::Backspace,
        Key::Delete,
        Key::Tab,
        Key::BackTab,
        Key::Escape,
    ];
    match rng.below(10) {
        0..=4 => {
            let chars = ['a', 'Z', ' ', '한', '字', 'é', '\n', '😀'];
            key(Key::Char(chars[rng.below(chars.len() as u64) as usize]))
        }
        5..=7 => key(NAV[rng.below(NAV.len() as u64) as usize]),
        8 => {
            let ch = ['a', 'z', 'x', 'v', 'u', 'y', 'k', 'w'][rng.below(8) as usize];
            key_mod(Key::Char(ch), rng.below(2) == 0, rng.below(2) == 0, false)
        }
        _ => odd[rng.below(odd.len() as u64) as usize].clone(),
    }
}

/// What a case does to a rig.
type Scenario = Box<dyn Fn(&mut Rig)>;

/// Resize, type, and - unless the size is past what a test can paint in
/// time - draw at that size. Either way the verdict draws again at 40×12.
fn resize_case(w: u16, h: u16, draw: bool) -> Scenario {
    Box::new(move |rig| {
        rig.send(Event::Resize(w, h));
        if draw {
            rig.draw();
        }
        for k in [Key::Down, Key::Char('x'), Key::PageDown, Key::End] {
            rig.send(key(k));
        }
        if draw {
            rig.draw();
        }
    })
}

fn burst(
    n: usize,
    seed: u64,
    every: impl Fn(usize, &mut Rng) -> Option<Event> + 'static,
) -> Scenario {
    Box::new(move |rig| {
        let odd = odd_keys();
        let mut rng = Rng(seed);
        for i in 0..n {
            if let Some(extra) = every(i, &mut rng) {
                let is_resize = matches!(extra, Event::Resize(..));
                rig.send(extra);
                if is_resize {
                    rig.draw();
                }
            }
            rig.send(burst_key(&mut rng, &odd));
            if i % 100 == 99 {
                rig.draw();
            }
        }
        rig.draw();
    })
}

/// Cases that may ask for more memory than a test can give run in a child.
const CHILD_ENV: &str = "REVUE_FAULT_EVENT_CASE";
const CHILD_TIMEOUT: Duration = Duration::from_secs(20);

fn cases() -> Vec<(String, bool, Scenario)> {
    let mut cases: Vec<(String, bool, Scenario)> = Vec::new();

    // Resizes. `true` marks the cases past the buffer limits: they run in a
    // child process and are not painted at that size (10 M cells is too much
    // for a debug-build test).
    for (w, h, isolated) in [
        (0, 0, false),
        (1, 1, false),
        (W, 0, false),
        (0, H, false),
        (1, 0, false),
        (16384, 1, false),
        (1, 16384, false),
        (16385, 1, true),
        (16384, 16384, true),
        (u16::MAX, 1, true),
        (u16::MAX, u16::MAX, true),
    ] {
        cases.push((
            format!("resize {w}x{h}"),
            isolated,
            resize_case(w, h, !isolated),
        ));
    }
    cases.push((
        "resize shrink-then-grow".into(),
        false,
        Box::new(|rig| {
            for (w, h) in [(1, 1), (0, 0), (2, 1), (200, 60), (3, 40), (80, 2), (0, 5)] {
                rig.send(Event::Resize(w, h));
                rig.draw();
                rig.send(key(Key::Down));
                rig.send(key(Key::Char('q')));
                rig.draw();
            }
        }),
    ));

    // Mouse, everywhere.
    let kinds = [
        ("down", MouseEventKind::Down(MouseButton::Left)),
        ("right", MouseEventKind::Down(MouseButton::Right)),
        ("up", MouseEventKind::Up(MouseButton::Left)),
        ("drag", MouseEventKind::Drag(MouseButton::Left)),
        ("move", MouseEventKind::Move),
        ("scroll-down", MouseEventKind::ScrollDown),
        ("scroll-up", MouseEventKind::ScrollUp),
        ("scroll-left", MouseEventKind::ScrollLeft),
    ];
    let points = [
        (0, 0),
        (W - 1, H - 1),
        (W, H),
        (W, 0),
        (0, H),
        (1000, 5),
        (5, 1000),
        (u16::MAX, 0),
        (0, u16::MAX),
        (u16::MAX, u16::MAX),
    ];
    for (name, kind) in kinds {
        for (x, y) in points {
            cases.push((
                format!("mouse {name} at {x},{y}"),
                false,
                Box::new(move |rig| {
                    for _ in 0..3 {
                        rig.send(Event::Mouse(MouseEvent::new(x, y, kind)));
                    }
                    rig.draw();
                    rig.send(key(Key::Char('m')));
                    rig.draw();
                }),
            ));
        }
    }

    // Pastes.
    cases.push((
        "paste 1mb".into(),
        false,
        Box::new(|rig| {
            let line = "paste 한글 漢字 é 😀 \t tab and some ascii to fill the line\n";
            let text = line.repeat((1 << 20) / line.len() + 1);
            rig.send(Event::Paste(text));
            rig.draw();
            for k in [Key::End, Key::Home, Key::PageDown, Key::Backspace, Key::Up] {
                rig.send(key(k));
            }
            rig.draw();
        }),
    ));
    cases.push((
        "paste control".into(),
        false,
        Box::new(|rig| {
            let text: String = (0u8..0x20)
                .map(char::from)
                .chain(
                    "\u{1b}[31mred\u{1b}[0m\r\n\r\u{7f}\u{85}\u{2028}\u{202e}rtl\u{202c}".chars(),
                )
                .collect();
            for _ in 0..3 {
                rig.send(Event::Paste(text.clone()));
                rig.draw();
            }
            rig.send(Event::Paste(String::new()));
            rig.draw();
        }),
    ));

    // Bursts.
    cases.push((
        "burst 10000 keys".into(),
        false,
        burst(10_000, 0x9e37_79b9, |_, _| None),
    ));
    cases.push((
        "burst odd keys".into(),
        false,
        Box::new(|rig| {
            for _ in 0..20 {
                for k in odd_keys() {
                    rig.send(k);
                }
                rig.draw();
            }
        }),
    ));
    cases.push((
        "burst with focus changes".into(),
        false,
        burst(2_000, 0x2545_f491, |i, _| match i % 7 {
            0 => Some(Event::FocusLost),
            3 => Some(Event::FocusGained),
            5 => Some(Event::Tick),
            _ => None,
        }),
    ));
    cases.push((
        "burst with resizes".into(),
        false,
        burst(2_000, 0x1234_5678, |i, rng| {
            (i % 50 == 0).then(|| Event::Resize(rng.below(120) as u16, rng.below(50) as u16))
        }),
    ));
    cases
}

/// Run one scenario on a fresh rig; `None` if every invariant holds.
fn run_case(scenario: &Scenario) -> Option<String> {
    match catch(|| {
        let mut rig = Rig::new();
        scenario(&mut rig);
        rig.verdict()
    }) {
        Ok(verdict) => verdict,
        Err(panic) => Some(panic),
    }
}

/// Run one case in a child process: a case that asks for more memory than
/// exists aborts rather than panics, and must not take the layer down.
fn run_isolated(case: &str) -> Option<String> {
    use std::process::{Command, Stdio};

    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([
            "fault::events::event_anomalies",
            "--exact",
            "--test-threads=1",
        ])
        .env(CHILD_ENV, case)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn child");
    let deadline = Instant::now() + CHILD_TIMEOUT;
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            if status.success() {
                return None;
            }
            let stderr = child
                .wait_with_output()
                .map(|o| String::from_utf8_lossy(&o.stderr).into_owned())
                .unwrap_or_default();
            let why = stderr
                .lines()
                .find(|l| l.contains("panicked") || l.contains("memory") || l.contains("alloc"))
                .unwrap_or("")
                .to_string();
            return Some(format!("child failed ({status}): {why}"));
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Some(format!("child still running after {CHILD_TIMEOUT:?}"));
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[test]
fn event_anomalies() {
    let cases = cases();

    // The child side: run the one named case and report through the exit
    // status.
    if let Ok(name) = std::env::var(CHILD_ENV) {
        let (_, _, scenario) = cases
            .iter()
            .find(|(n, _, _)| *n == name)
            .expect("known case");
        if let Some(failure) = run_case(scenario) {
            panic!("{failure}");
        }
        return;
    }

    let mut failures = Vec::new();
    for (name, isolated, scenario) in &cases {
        let failure = if *isolated {
            run_isolated(name)
        } else {
            run_case(scenario)
        };
        if let Some(detail) = failure {
            failures.push(Failure::new(name.clone(), detail));
        }
    }
    let ran = cases.iter().map(|(name, _, _)| name.clone()).collect();
    ratchet(LAYER, &ran, failures);
}
