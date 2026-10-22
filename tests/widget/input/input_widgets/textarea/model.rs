//! TextArea against a text model with selection and undo (#1162)
//!
//! Keys and the selecting moves go to the editor and to a model that keeps
//! the text as one char list (lines joined by `'\n'`), the cursor and the
//! selection anchor as offsets in it, and the undo history as the texts
//! before and after each edit. After every step the text, the cursor and the
//! selected text must equal the model's. Copy, cut and paste are left out:
//! with the `clipboard` feature they use the system clipboard.

use proptest::prelude::*;
use revue::event::{Key, KeyEvent};
use revue::widget::TextArea;

/// Screen columns of a char, as the editor counts them
fn width(c: char) -> usize {
    if c == '한' {
        2
    } else {
        1
    }
}

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

#[derive(Clone, Debug)]
enum Op {
    /// A key, with Ctrl and Shift
    Key(Key, bool, bool),
    /// A move that extends the selection from where it was
    Select(Move),
    /// A word move that drops the selection
    Word(bool),
}

fn op() -> impl Strategy<Value = Op> {
    let plain = prop::sample::select(vec![
        Key::Char('a'),
        Key::Char(' '),
        Key::Char('한'),
        Key::Enter,
        Key::Tab,
        Key::Backspace,
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
        (Key::Char('a'), false),
        (Key::Char('z'), false),
        (Key::Char('z'), false),
        (Key::Char('y'), false),
        (Key::Char('z'), true),
        (Key::Char('k'), true),
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
        8 => plain.prop_map(|k| Op::Key(k, false, false)),
        3 => ctrl.prop_map(|(k, shift)| Op::Key(k, true, shift)),
        4 => mv.prop_map(Op::Select),
        1 => any::<bool>().prop_map(Op::Word),
    ]
}

#[derive(Debug, Default)]
struct Model {
    text: Vec<char>,
    cursor: usize,
    anchor: Option<usize>,
    undo: Vec<Vec<char>>,
    redo: Vec<Vec<char>>,
}

impl Model {
    fn line_start(&self, at: usize) -> usize {
        self.text[..at]
            .iter()
            .rposition(|&c| c == '\n')
            .map_or(0, |i| i + 1)
    }

    fn line_end(&self, at: usize) -> usize {
        self.text[at..]
            .iter()
            .position(|&c| c == '\n')
            .map_or(self.text.len(), |i| at + i)
    }

    /// (line, col) of an offset
    fn position(&self, at: usize) -> (usize, usize) {
        let line = self.text[..at].iter().filter(|&&c| c == '\n').count();
        (line, at - self.line_start(at))
    }

    /// The offset on the line above or below under the cursor's screen
    /// column
    fn vertical(&self, down: bool) -> usize {
        let start = self.line_start(self.cursor);
        let target = if down {
            let end = self.line_end(self.cursor);
            if end == self.text.len() {
                return self.cursor;
            }
            end + 1
        } else {
            if start == 0 {
                return self.cursor;
            }
            self.line_start(start - 1)
        };
        let col: usize = self.text[start..self.cursor]
            .iter()
            .map(|&c| width(c))
            .sum();
        let mut x = 0;
        for i in target..self.line_end(target) {
            x += width(self.text[i]);
            if col < x {
                return i;
            }
        }
        self.line_end(target)
    }

    fn target(&self, mv: Move) -> usize {
        let (cur, len) = (self.cursor, self.text.len());
        let (start, end) = (self.line_start(cur), self.line_end(cur));
        let ws = |i: usize| self.text[i].is_whitespace();
        match mv {
            Move::Left => cur.saturating_sub(1),
            Move::Right => (cur + 1).min(len),
            Move::Up => self.vertical(false),
            Move::Down => self.vertical(true),
            Move::Home => start,
            Move::End => end,
            // At the start of a line, to the end of the line above
            Move::WordLeft if cur == start => cur.saturating_sub(1),
            Move::WordLeft => {
                let mut i = cur;
                while i > start && ws(i - 1) {
                    i -= 1;
                }
                while i > start && !ws(i - 1) {
                    i -= 1;
                }
                i
            }
            Move::WordRight if cur == end => (cur + 1).min(len),
            Move::WordRight => {
                let mut i = cur;
                while i < end && !ws(i) {
                    i += 1;
                }
                while i < end && ws(i) {
                    i += 1;
                }
                i
            }
        }
    }

    fn selection(&self) -> Option<(usize, usize)> {
        self.anchor
            .map(|a| (a.min(self.cursor), a.max(self.cursor)))
    }

    /// The selection, when it holds any text
    fn selected(&self) -> Option<(usize, usize)> {
        self.selection().filter(|(start, end)| start < end)
    }

    /// Replace `start..end` with `with` as one undo step
    fn edit(&mut self, start: usize, end: usize, with: &str) {
        self.undo.push(self.text.clone());
        self.redo.clear();
        self.text.splice(start..end, with.chars());
        self.cursor = start + with.chars().count();
        self.anchor = None;
    }

    fn type_text(&mut self, s: &str) {
        let (start, end) = self.selected().unwrap_or((self.cursor, self.cursor));
        self.edit(start, end, s);
    }

    /// Applies `op`; true when it was undo or redo, after which the cursor
    /// is the editor's to keep (the model knows only the text)
    fn apply(&mut self, op: &Op) -> bool {
        let len = self.text.len();
        match *op {
            Op::Select(mv) => {
                self.anchor.get_or_insert(self.cursor);
                self.cursor = self.target(mv);
            }
            Op::Word(right) => {
                self.anchor = None;
                self.cursor = self.target(if right {
                    Move::WordRight
                } else {
                    Move::WordLeft
                });
            }
            Op::Key(Key::Char('a'), true, _) => {
                self.anchor = Some(0);
                self.cursor = len;
            }
            Op::Key(Key::Char('z'), true, false) => {
                if let Some(before) = self.undo.pop() {
                    self.redo.push(std::mem::replace(&mut self.text, before));
                }
                return true;
            }
            Op::Key(Key::Char('y' | 'z'), true, _) => {
                if let Some(after) = self.redo.pop() {
                    self.undo.push(std::mem::replace(&mut self.text, after));
                }
                return true;
            }
            // Ctrl+Shift+K: delete the cursor's line, with its line break
            Op::Key(Key::Char('k'), true, true) => {
                let (start, end) = (self.line_start(self.cursor), self.line_end(self.cursor));
                if self.text.contains(&'\n') {
                    let (from, to) = if end < len {
                        (start, end + 1)
                    } else {
                        (start - 1, end)
                    };
                    self.edit(from, to, "");
                    self.cursor = if end < len {
                        start
                    } else {
                        self.line_start(from)
                    };
                }
            }
            Op::Key(Key::Char(c), false, _) => self.type_text(&c.to_string()),
            Op::Key(Key::Enter, ..) => self.type_text("\n"),
            Op::Key(Key::Tab, ..) => self.type_text("    "),
            Op::Key(Key::Backspace | Key::Delete, ..) if self.selected().is_some() => {
                let (start, end) = self.selected().unwrap();
                self.edit(start, end, "");
            }
            Op::Key(Key::Backspace, ..) if self.cursor > 0 => {
                self.edit(self.cursor - 1, self.cursor, "")
            }
            Op::Key(Key::Delete, ..) if self.cursor < len => {
                self.edit(self.cursor, self.cursor + 1, "")
            }
            Op::Key(Key::Backspace | Key::Delete, ..) => self.anchor = None,
            Op::Key(key, ..) => {
                let mv = match key {
                    Key::Left => Move::Left,
                    Key::Right => Move::Right,
                    Key::Up => Move::Up,
                    Key::Down => Move::Down,
                    Key::Home => Move::Home,
                    Key::End => Move::End,
                    _ => unreachable!("{key:?}"),
                };
                self.anchor = None;
                self.cursor = self.target(mv);
            }
        }
        false
    }
}

fn run(ta: &mut TextArea, op: &Op) {
    match *op {
        Op::Key(key, ctrl, shift) => {
            let mut e = KeyEvent::new(key);
            e.ctrl = ctrl;
            e.shift = shift;
            ta.handle_key_event(&e);
        }
        Op::Select(mv) => {
            if !ta.has_selection() {
                ta.start_selection();
            }
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
        Op::Word(right) => {
            ta.clear_selection();
            if right {
                ta.move_word_right();
            } else {
                ta.move_word_left();
            }
        }
    }
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
/// delete that empty selection and leave the char before the cursor
#[test]
fn backspace_with_an_empty_selection_deletes_a_char() {
    let mut ta = TextArea::new().focused(true).content("ab");
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

/// Ctrl+Shift+K kept the anchor, which then selected other text
#[test]
fn deleting_a_line_drops_the_selection() {
    let mut ta = TextArea::new().focused(true).content("a\nb");
    ta.handle_key_event(&ctrl(Key::Char('a'), false));
    ta.handle_key_event(&ctrl(Key::Char('k'), true));
    assert_eq!(ta.get_content(), "a");
    assert_eq!(ta.get_selection(), None);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    /// Any keys and selecting moves: the text, the cursor and the selected
    /// text equal the model's; typing over a selection, deleting it and
    /// each other edit are one undo step; undo gives the text before the
    /// last edit and redo the text after it, with the cursor in the text.
    #[test]
    fn textarea_follows_the_text_model(ops in prop::collection::vec(op(), 0..80)) {
        let mut ta = TextArea::new().focused(true);
        let mut m = Model::default();
        for op in &ops {
            run(&mut ta, op);
            let history = m.apply(op);
            let content: String = m.text.iter().collect();
            prop_assert_eq!(ta.get_content(), content.clone(), "after {:?}", op);
            if history {
                let (line, col) = ta.cursor_position();
                let lines: Vec<&str> = content.split('\n').collect();
                prop_assert!(line < lines.len() && col <= lines[line].chars().count(),
                    "cursor {:?} past the text after {:?}", (line, col), op);
                m.cursor = lines[..line].iter().map(|l| l.chars().count() + 1).sum::<usize>() + col;
                // Where the selection goes after undo is the editor's too
                m.anchor = None;
                ta.clear_selection();
            }
            prop_assert_eq!(ta.cursor_position(), m.position(m.cursor), "after {:?}", op);
            let selected = m.selection().map(|(s, e)| m.text[s..e].iter().collect::<String>());
            prop_assert_eq!(ta.get_selection(), selected, "after {:?}", op);
        }
    }
}
