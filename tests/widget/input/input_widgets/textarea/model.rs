//! TextArea is a multi-line `EditState` (#1261): any keys, selecting moves
//! and added cursors give the text, cursors, selection and undo state an
//! `EditState::multi_line()` gets from the same steps, with columns turned
//! into char indices. TextArea's own keys map to the model's edits: Tab
//! types four spaces and Ctrl+Shift+K deletes the lines. The edit rules
//! themselves are checked in `tests/utils/edit`. Paging is the view's and is
//! left out, as are copy, cut and paste.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use proptest::prelude::*;
use revue::event::{Key, KeyEvent};
use revue::utils::{EditCursor, EditMotion, EditState};
use revue::widget::TextArea;

#[derive(Clone, Copy, Debug)]
enum Move {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    WordLeft,
    WordRight,
}

impl Move {
    fn motion(self) -> EditMotion {
        match self {
            Move::Left => EditMotion::Left,
            Move::Right => EditMotion::Right,
            Move::Up => EditMotion::Up,
            Move::Down => EditMotion::Down,
            Move::Home => EditMotion::LineStart,
            Move::End => EditMotion::LineEnd,
            Move::WordLeft => EditMotion::WordLeft,
            Move::WordRight => EditMotion::WordRight,
        }
    }
}

#[derive(Clone, Debug)]
enum Op {
    /// A key, with Ctrl and Shift
    Key(Key, bool, bool),
    /// `start_selection`, then a move method: it extends the selection
    Select(Move),
    /// A cursor added at a line and char column
    AddCursor(usize, usize),
}

fn op() -> impl Strategy<Value = Op> {
    let plain = prop::sample::select(vec![
        Key::Char('a'),
        Key::Char(' '),
        Key::Char('.'),
        Key::Char('한'),
        Key::Char('\u{301}'),
        Key::Enter,
        Key::Tab,
        Key::Backspace,
        Key::Delete,
        Key::Left,
        Key::Right,
        Key::Up,
        Key::Down,
        Key::Home,
        Key::End,
    ]);
    let ctrl = prop::sample::select(vec![
        Key::Char('a'),
        Key::Char('z'),
        Key::Char('y'),
        Key::Char('k'),
        Key::Backspace,
        Key::Delete,
        Key::Left,
        Key::Right,
        Key::Home,
        Key::End,
    ]);
    let mv = prop::sample::select(vec![
        Move::Left,
        Move::Right,
        Move::Up,
        Move::Down,
        Move::Home,
        Move::End,
        Move::WordLeft,
        Move::WordRight,
    ]);
    prop_oneof![
        8 => (plain, prop::bool::weighted(0.3)).prop_map(|(k, shift)| Op::Key(k, false, shift)),
        3 => (ctrl, any::<bool>()).prop_map(|(k, shift)| Op::Key(k, true, shift)),
        2 => mv.prop_map(Op::Select),
        1 => (0..4usize, 0..6usize).prop_map(|(l, c)| Op::AddCursor(l, c)),
    ]
}

/// Run `op` on the model as TextArea should
fn model_step(state: &mut EditState, op: &Op) {
    match *op {
        Op::Key(Key::Tab, false, _) => {
            state.insert("    ");
        }
        Op::Key(Key::Char('k' | 'K'), true, true) => {
            state.delete_lines();
        }
        Op::Key(key, ctrl, shift) => {
            let mut e = KeyEvent::new(key);
            e.ctrl = ctrl;
            e.shift = shift;
            state.edit_key(&e, 10);
        }
        Op::Select(mv) => {
            state.move_cursors(mv.motion(), true);
        }
        Op::AddCursor(line, col) => {
            if state.cursors().len() < 100 {
                state.add_cursor(EditCursor::at(state.pos_at_char(line, col)));
            }
        }
    }
}

fn area_step(ta: &mut TextArea, op: &Op) {
    match *op {
        Op::Key(key, ctrl, shift) => {
            let mut e = KeyEvent::new(key);
            e.ctrl = ctrl;
            e.shift = shift;
            ta.handle_key_event(&e);
        }
        Op::Select(mv) => {
            ta.start_selection();
            match mv {
                Move::Left => ta.move_left(),
                Move::Right => ta.move_right(),
                Move::Up => ta.move_up(),
                Move::Down => ta.move_down(),
                Move::Home => ta.move_home(),
                Move::End => ta.move_end(),
                Move::WordLeft => ta.move_word_left(),
                Move::WordRight => ta.move_word_right(),
            }
        }
        Op::AddCursor(line, col) => ta.add_cursor_at(line, col),
    }
}

/// A clock that moves on a second at each reading, so every edit is an undo
/// step of its own
fn stepping_clock() -> impl Fn() -> Instant + Send + Sync + 'static {
    let start = Instant::now();
    let ticks = Arc::new(AtomicU64::new(0));
    move || start + Duration::from_secs(ticks.fetch_add(1, Ordering::Relaxed))
}

fn shift(key: Key) -> KeyEvent {
    let mut e = KeyEvent::new(key);
    e.shift = true;
    e
}

fn ctrl(key: Key, shift: bool) -> KeyEvent {
    let mut e = KeyEvent::ctrl(key);
    e.shift = shift;
    e
}

/// Typing over a selection deleted it and inserted the char as two undo
/// steps, so one undo left the selected text gone
#[test]
fn typing_over_a_selection_is_one_undo_step() {
    let mut ta = TextArea::new().focused(true).content("ab");
    ta.handle_key_event(&ctrl(Key::Char('a'), false));
    ta.handle_key(&Key::Char('x'));
    assert_eq!(ta.get_content(), "x");
    ta.undo();
    assert_eq!(ta.get_content(), "ab");
    ta.redo();
    assert_eq!(ta.get_content(), "x");
}

/// A selection started without moving holds no text; Backspace used to
/// delete that empty selection and leave the char before the cursor, and
/// typing over it added an undo step
#[test]
fn backspace_with_an_empty_selection_deletes_a_char() {
    let mut ta = TextArea::new()
        .focused(true)
        .clock(stepping_clock())
        .content("ab");
    ta.set_cursor(0, 2);
    ta.start_selection();
    ta.handle_key(&Key::Backspace);
    assert_eq!(ta.get_content(), "a");
    ta.start_selection();
    ta.handle_key(&Key::Char('c'));
    ta.undo();
    assert_eq!(ta.get_content(), "a");
    ta.undo();
    assert_eq!(
        ta.get_content(),
        "ab",
        "an empty selection added an undo step"
    );
}

/// Ctrl+Shift+K took only the cursor's line and kept the anchor, which then
/// selected other text; it takes every line the selection touches
#[test]
fn deleting_lines_takes_every_selected_line_and_drops_the_selection() {
    let mut ta = TextArea::new().focused(true).content("a\nb\nc");
    ta.set_cursor(0, 0);
    ta.handle_key_event(&shift(Key::Down));
    ta.handle_key_event(&shift(Key::Right));
    ta.handle_key_event(&ctrl(Key::Char('k'), true));
    assert_eq!(ta.get_content(), "c");
    assert_eq!(ta.get_selection(), None);
}

/// Line commands took only the primary cursor's line (#1253)
#[test]
fn deleting_lines_takes_every_cursors_line() {
    let mut ta = TextArea::new().focused(true).content("a\nb\nc\nd");
    ta.set_cursor(0, 0);
    ta.add_cursor_at(2, 0);
    ta.handle_key_event(&ctrl(Key::Char('k'), true));
    assert_eq!(ta.get_content(), "b\nd");
    assert_eq!(ta.cursor_positions(), [(0, 0), (1, 0)]);
}

/// Edits applied at the primary cursor only (#1006)
#[test]
fn typing_goes_in_at_every_cursor() {
    let mut ta = TextArea::new().focused(true).content("ab\ncd");
    ta.set_cursor(0, 1);
    ta.add_cursor_at(1, 1);
    ta.handle_key(&Key::Char('x'));
    assert_eq!(ta.get_content(), "axb\ncxd");
    ta.handle_key(&Key::Backspace);
    assert_eq!(ta.get_content(), "ab\ncd");
}

/// Cursors added with Ctrl+Alt+Down added below the primary cursor, so a
/// second press landed on the same line (#1006)
#[test]
fn adding_cursors_below_goes_a_line_further_each_time() {
    let mut ta = TextArea::new().focused(true).content("aa\nbb\ncc");
    ta.set_cursor(0, 1);
    ta.add_cursor_below();
    ta.add_cursor_below();
    assert_eq!(ta.cursor_positions(), [(0, 1), (1, 1), (2, 1)]);
    ta.add_cursor_above();
    assert_eq!(ta.cursor_count(), 3, "no line above the topmost");
    ta.handle_key(&Key::Char('#'));
    assert_eq!(ta.get_content(), "a#a\nb#b\nc#c");
    // Moves go at every cursor too
    ta.handle_key(&Key::End);
    assert_eq!(ta.cursor_positions(), [(0, 3), (1, 3), (2, 3)]);
}

/// Ctrl+D added the next occurrence without selecting the word under the
/// cursor, and typing replaced neither (#1006)
#[test]
fn ctrl_d_selects_the_word_then_adds_its_next_occurrence() {
    let mut ta = TextArea::new().focused(true).content("foo bar foo");
    ta.handle_key_event(&ctrl(Key::Char('d'), false));
    assert_eq!(ta.get_selection().as_deref(), Some("foo"));
    ta.handle_key_event(&ctrl(Key::Char('d'), false));
    assert_eq!(ta.cursor_count(), 2);
    ta.handle_key(&Key::Char('X'));
    assert_eq!(ta.get_content(), "X bar X");
    assert_eq!(ta.cursor_positions(), [(0, 1), (0, 7)]);
}

/// Undo put the cursor where the operation said, not where it was (#1251)
#[test]
fn undo_puts_back_the_cursors_and_selection() {
    let mut ta = TextArea::new().focused(true).content("one two");
    ta.set_cursor(0, 4);
    ta.handle_key_event(&shift(Key::End));
    ta.handle_key(&Key::Delete);
    assert_eq!(ta.get_content(), "one ");
    ta.undo();
    assert_eq!(ta.get_content(), "one two");
    assert_eq!(ta.get_selection().as_deref(), Some("two"));
    assert_eq!(ta.cursor_position(), (0, 7));
}

/// Shift with a move did not select (#1252)
#[test]
fn shift_with_a_move_selects() {
    let mut ta = TextArea::new().focused(true).content("ab\ncd");
    ta.handle_key_event(&shift(Key::Down));
    ta.handle_key_event(&shift(Key::Right));
    assert_eq!(ta.get_selection().as_deref(), Some("ab\nc"));
    ta.handle_key(&Key::Left);
    assert_eq!(ta.get_selection(), None);
    assert_eq!(ta.cursor_position(), (0, 0));
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    /// Any keys, selecting moves and added cursors: the TextArea's text,
    /// cursors, selection and undo state equal those of an EditState given
    /// the same steps
    #[test]
    fn textarea_follows_its_edit_state(ops in prop::collection::vec(op(), 0..80)) {
        // One frozen clock for both, so their undo steps group alike
        let now = Instant::now();
        let mut ta = TextArea::new().focused(true).clock(move || now);
        let mut state = EditState::multi_line().clock(move || now);
        for op in &ops {
            area_step(&mut ta, op);
            model_step(&mut state, op);
            prop_assert_eq!(ta.get_content(), state.text(), "after {:?}", op);
            let cursors: Vec<(usize, usize)> = state
                .cursors()
                .iter()
                .map(|c| (c.head().line, state.char_index(c.head())))
                .collect();
            prop_assert_eq!(ta.cursor_positions(), cursors, "after {:?}", op);
            let primary = state.cursor();
            prop_assert_eq!(ta.cursor_position(), (primary.line, state.char_index(primary)));
            let selected = state.primary().has_selection().then(|| {
                let (from, to) = state.primary().range();
                state.text_in(from, to)
            });
            prop_assert_eq!(ta.get_selection(), selected, "after {:?}", op);
            prop_assert_eq!((ta.can_undo(), ta.can_redo()), (state.can_undo(), state.can_redo()));
        }
    }
}

/// A terminal may report Ctrl+Shift+K as a capital without the Shift flag
#[test]
fn ctrl_capital_k_deletes_the_line() {
    let mut ta = TextArea::new().focused(true).content("a\nb");
    assert!(ta.handle_key_event(&KeyEvent::ctrl(Key::Char('K'))));
    assert_eq!(ta.get_content(), "b");
    // Without Shift, Ctrl+K is left to the app
    assert!(!ta.handle_key_event(&KeyEvent::ctrl(Key::Char('k'))));
}
