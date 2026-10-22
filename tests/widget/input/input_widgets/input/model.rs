//! Input against a text model with selection and undo (#1162)
//!
//! Plain, Shift and Ctrl keys go to the input and to a model of the text,
//! the cursor, the selection anchor and the undo history (as the texts
//! before and after each edit). After every key the text, the cursor and
//! the selection must equal the model's. Copy, cut and paste are left out:
//! with the `clipboard` feature they use the system clipboard.

use proptest::prelude::*;
use revue::event::{Key, KeyEvent};
use revue::widget::Input;

fn event() -> impl Strategy<Value = KeyEvent> {
    let key = prop_oneof![
        4 => prop::sample::select(vec!['a', 'b', ' ', '한', '😀']).prop_map(Key::Char),
        2 => Just(Key::Backspace),
        1 => Just(Key::Delete),
        2 => Just(Key::Left),
        2 => Just(Key::Right),
        1 => Just(Key::Home),
        1 => Just(Key::End),
        // Ctrl+A, Ctrl+Z, Ctrl+Y with ctrl below
        1 => Just(Key::Char('z')),
        1 => Just(Key::Char('y')),
        1 => Just(Key::Char('a')),
    ];
    (key, any::<bool>(), prop::bool::weighted(0.3)).prop_map(|(key, shift, ctrl)| {
        let mut e = KeyEvent::new(key);
        e.shift = shift;
        e.ctrl = ctrl;
        e
    })
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
    fn has_selection(&self) -> bool {
        self.anchor.is_some_and(|a| a != self.cursor)
    }

    fn selection(&self) -> Option<(usize, usize)> {
        self.anchor
            .map(|a| (a.min(self.cursor), a.max(self.cursor)))
    }

    fn word_left(&self) -> usize {
        let mut i = self.cursor;
        while i > 0 && self.text[i - 1].is_whitespace() {
            i -= 1;
        }
        while i > 0 && !self.text[i - 1].is_whitespace() {
            i -= 1;
        }
        i
    }

    fn word_right(&self) -> usize {
        let (mut i, len) = (self.cursor, self.text.len());
        while i < len && !self.text[i].is_whitespace() {
            i += 1;
        }
        while i < len && self.text[i].is_whitespace() {
            i += 1;
        }
        i
    }

    /// Replace `start..end` with `with` as one undo step
    fn edit(&mut self, start: usize, end: usize, with: &[char]) {
        self.undo.push(self.text.clone());
        self.redo.clear();
        self.text.splice(start..end, with.iter().copied());
        self.cursor = start + with.len();
        self.anchor = None;
    }

    fn delete_selection(&mut self) {
        let (start, end) = self.selection().unwrap();
        self.edit(start, end, &[]);
    }

    /// Move the cursor; Shift extends a selection from where it was
    fn move_to(&mut self, to: usize, shift: bool) {
        if shift {
            self.anchor.get_or_insert(self.cursor);
        } else {
            self.anchor = None;
        }
        self.cursor = to;
    }

    /// Apply `e`; returns true when it was undo or redo, after which the
    /// cursor is the input's to keep (the model knows only the text)
    fn apply(&mut self, e: &KeyEvent) -> bool {
        let len = self.text.len();
        match (e.ctrl, &e.key) {
            (true, Key::Char('a')) => {
                self.anchor = Some(0);
                self.cursor = len;
            }
            (true, Key::Char('z')) => {
                if let Some(before) = self.undo.pop() {
                    self.redo.push(std::mem::replace(&mut self.text, before));
                    self.anchor = None;
                }
                return true;
            }
            (true, Key::Char('y')) => {
                if let Some(after) = self.redo.pop() {
                    self.undo.push(std::mem::replace(&mut self.text, after));
                    self.anchor = None;
                }
                return true;
            }
            (true, Key::Left) => self.move_to(self.word_left(), e.shift),
            (true, Key::Right) => self.move_to(self.word_right(), e.shift),
            (true, Key::Backspace) if self.has_selection() => self.delete_selection(),
            (true, Key::Backspace) => {
                self.anchor = None;
                if self.cursor > 0 {
                    self.edit(self.word_left(), self.cursor, &[]);
                }
            }
            // Other Ctrl letters are commands left to the app
            (true, Key::Char(_)) => {}
            (_, Key::Char(c)) => {
                let (start, end) = self.selection().unwrap_or((self.cursor, self.cursor));
                self.edit(start, end, &[*c]);
            }
            (_, Key::Backspace | Key::Delete) if self.has_selection() => self.delete_selection(),
            (_, Key::Backspace) if self.cursor > 0 => self.edit(self.cursor - 1, self.cursor, &[]),
            (_, Key::Delete) if self.cursor < len => self.edit(self.cursor, self.cursor + 1, &[]),
            (_, Key::Backspace | Key::Delete) => {}
            (_, Key::Left) => self.move_to(self.cursor.saturating_sub(1), e.shift),
            (_, Key::Right) => self.move_to((self.cursor + 1).min(len), e.shift),
            (_, Key::Home) => self.move_to(0, e.shift),
            (_, Key::End) => self.move_to(len, e.shift),
            _ => {}
        }
        false
    }
}

/// Undoing a typed-over selection put the swapped edit on the redo stack,
/// so redo put the old text back instead of the typed one
#[test]
fn redo_after_typing_over_a_selection_types_it_again() {
    let mut input = Input::new().value("ab");
    input.handle_key_event(&KeyEvent::ctrl(Key::Char('a')));
    input.handle_key(&Key::Char('x'));
    assert!(input.undo());
    assert_eq!(input.text(), "ab");
    assert!(input.redo());
    assert_eq!(input.text(), "x");
    assert!(input.undo());
    assert_eq!(input.text(), "ab");
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    /// Any keys: the text, the cursor and the selection equal the model's;
    /// undo gives the text before the last edit and redo the text after it,
    /// with no selection and the cursor in the text.
    #[test]
    fn input_follows_the_text_model(events in prop::collection::vec(event(), 0..80)) {
        let mut input = Input::new();
        let mut m = Model::default();
        for e in &events {
            input.handle_key_event(e);
            let history = m.apply(e);
            if history {
                prop_assert!(input.cursor() <= m.text.len(), "cursor past the text after {:?}", e);
                m.cursor = input.cursor();
            }
            prop_assert_eq!(input.text(), m.text.iter().collect::<String>(), "after {:?}", e);
            prop_assert_eq!(input.cursor(), m.cursor, "after {:?}", e);
            prop_assert_eq!(input.selection(), m.selection(), "after {:?}", e);
            prop_assert_eq!(input.has_selection(), m.has_selection());
            prop_assert_eq!((input.can_undo(), input.can_redo()), (!m.undo.is_empty(), !m.redo.is_empty()));
            if let Some((start, end)) = m.selection() {
                let selected: String = m.text[start..end].iter().collect();
                prop_assert_eq!(input.selected_text(), Some(selected.as_str()));
            }
        }
    }
}
