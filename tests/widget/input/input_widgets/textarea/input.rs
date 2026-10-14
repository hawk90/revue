//! TextArea keys: a model of the plain editing keys, and the Interactive
//! entry point

use proptest::prelude::*;
use revue::event::{Key, KeyEvent};
use revue::widget::traits::{EventResult, Interactive};
use revue::widget::TextArea;

/// Screen columns of a char, as the editor counts them.
fn width(c: char) -> usize {
    if c == '한' {
        2
    } else {
        1
    }
}

/// A plain model of the text and the cursor (line, char index).
struct Model {
    lines: Vec<Vec<char>>,
    line: usize,
    col: usize,
}

impl Model {
    /// The char index on `to` under the screen column of the cursor.
    fn same_column(&self, to: usize) -> usize {
        let col: usize = self.lines[self.line][..self.col]
            .iter()
            .map(|&c| width(c))
            .sum();
        let mut start = 0;
        for (i, &c) in self.lines[to].iter().enumerate() {
            if col < start + width(c) {
                return i;
            }
            start += width(c);
        }
        self.lines[to].len()
    }

    fn key(&mut self, key: &Key) {
        let len = self.lines[self.line].len();
        match key {
            Key::Char(c) => {
                self.lines[self.line].insert(self.col, *c);
                self.col += 1;
            }
            Key::Tab => {
                for _ in 0..4 {
                    self.key(&Key::Char(' '));
                }
            }
            Key::Enter => {
                let rest = self.lines[self.line].split_off(self.col);
                self.lines.insert(self.line + 1, rest);
                self.line += 1;
                self.col = 0;
            }
            Key::Backspace if self.col > 0 => {
                self.lines[self.line].remove(self.col - 1);
                self.col -= 1;
            }
            Key::Backspace if self.line > 0 => {
                let cur = self.lines.remove(self.line);
                self.line -= 1;
                self.col = self.lines[self.line].len();
                self.lines[self.line].extend(cur);
            }
            Key::Delete if self.col < len => {
                self.lines[self.line].remove(self.col);
            }
            Key::Delete if self.line + 1 < self.lines.len() => {
                let next = self.lines.remove(self.line + 1);
                self.lines[self.line].extend(next);
            }
            Key::Left if self.col > 0 => self.col -= 1,
            Key::Left if self.line > 0 => {
                self.line -= 1;
                self.col = self.lines[self.line].len();
            }
            Key::Right if self.col < len => self.col += 1,
            Key::Right if self.line + 1 < self.lines.len() => {
                self.line += 1;
                self.col = 0;
            }
            Key::Up if self.line > 0 => {
                self.col = self.same_column(self.line - 1);
                self.line -= 1;
            }
            Key::Down if self.line + 1 < self.lines.len() => {
                self.col = self.same_column(self.line + 1);
                self.line += 1;
            }
            Key::Home => self.col = 0,
            Key::End => self.col = len,
            _ => {}
        }
    }

    fn content(&self) -> String {
        self.lines
            .iter()
            .map(|l| l.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        Just(Key::Char('a')),
        Just(Key::Char('b')),
        Just(Key::Char(' ')),
        Just(Key::Char('한')),
        Just(Key::Tab),
        Just(Key::Enter),
        Just(Key::Backspace),
        Just(Key::Delete),
        Just(Key::Left),
        Just(Key::Right),
        Just(Key::Up),
        Just(Key::Down),
        Just(Key::Home),
        Just(Key::End),
    ]
}

proptest! {
    /// Any sequence of the plain editing keys leaves the text and the cursor
    /// a char-by-char model gives, through both entry points.
    #[test]
    fn prop_textarea_keys_follow_the_model(keys in prop::collection::vec(key(), 0..80)) {
        let mut plain = TextArea::new().focused(true);
        let mut event = TextArea::new().focused(true);
        let mut model = Model { lines: vec![Vec::new()], line: 0, col: 0 };
        for k in &keys {
            prop_assert!(plain.handle_key(k));
            prop_assert!(event.handle_key_event(&KeyEvent::new(*k)));
            model.key(k);
            prop_assert_eq!(plain.get_content(), model.content(), "after {:?}", k);
            prop_assert_eq!(plain.cursor_position(), (model.line, model.col), "after {:?}", k);
        }
        prop_assert_eq!(event.get_content(), plain.get_content());
        prop_assert_eq!(event.cursor_position(), plain.cursor_position());
    }

    /// An unfocused TextArea takes no key.
    #[test]
    fn prop_textarea_unfocused_ignores_keys(keys in prop::collection::vec(key(), 0..20)) {
        let mut ta = TextArea::new().content("ab\ncd");
        for k in &keys {
            prop_assert!(!ta.handle_key(k));
            prop_assert!(!ta.handle_key_event(&KeyEvent::new(*k)));
        }
        prop_assert_eq!(ta.get_content(), "ab\ncd");
        prop_assert_eq!(ta.cursor_position(), (0, 0));
    }
}

#[test]
fn test_textarea_interactive_handle_key_reports_use() {
    let mut ta = TextArea::new().focused(true);
    let consumed = Interactive::handle_key(&mut ta, &KeyEvent::new(Key::Char('x')));
    assert_eq!(consumed, EventResult::from(true));
    assert_eq!(ta.get_content(), "x");
    let ignored = Interactive::handle_key(&mut ta, &KeyEvent::new(Key::Escape));
    assert_eq!(ignored, EventResult::from(false));
}
