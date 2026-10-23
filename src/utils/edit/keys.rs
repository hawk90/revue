//! The standard editing keys

use super::motion::EditMotion;
use super::EditState;
use crate::event::{Key, KeyEvent};

/// What a key did to an [`EditState`]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditOutcome {
    /// The text changed
    Edited,
    /// The key was an editing key and the text did not change: a move, a
    /// selection, or an edit with nothing to do (Backspace at the start)
    Moved,
    /// Not an editing key here: the widget or the app may use it (Tab,
    /// Escape, Enter and Up/Down in a single-line field, Ctrl+letter)
    Unhandled,
}

impl EditState {
    /// Apply a standard editing key. `page_rows` is how many lines PageUp
    /// and PageDown move (the rows the field shows).
    ///
    /// | Key | Does |
    /// |---|---|
    /// | a character (no Ctrl or Alt, or both: AltGr) | type it |
    /// | Enter | a new line (multi-line) |
    /// | Backspace, Delete | delete; with Ctrl, a word |
    /// | ←, → | move; with Ctrl, a word; with Shift, select |
    /// | ↑, ↓, PageUp, PageDown | move a line or a page (multi-line); with Shift, select |
    /// | Home, End | line start or end; with Ctrl, the text's; with Shift, select |
    /// | Ctrl+A | select all |
    /// | Ctrl+Z | undo |
    /// | Ctrl+Y, Ctrl+Shift+Z | redo |
    pub fn edit_key(&mut self, key: &KeyEvent, page_rows: usize) -> EditOutcome {
        let multi = !self.single_line;
        let word = key.ctrl && !key.alt;
        let edited = |changed: bool| {
            if changed {
                EditOutcome::Edited
            } else {
                EditOutcome::Moved
            }
        };
        let motion = match key.key {
            Key::Char(c) if key.ctrl == key.alt && !c.is_control() => {
                return edited(self.insert(c.encode_utf8(&mut [0; 4])));
            }
            Key::Char(c) if word => {
                return match c.to_ascii_lowercase() {
                    'a' => {
                        self.select_all();
                        EditOutcome::Moved
                    }
                    'z' if key.shift || c == 'Z' => edited(self.redo()),
                    'z' => edited(self.undo()),
                    'y' => edited(self.redo()),
                    _ => EditOutcome::Unhandled,
                };
            }
            Key::Enter if multi && !key.ctrl && !key.alt => {
                return edited(self.insert("\n"));
            }
            Key::Backspace if word => return edited(self.delete_word_backward()),
            Key::Backspace => return edited(self.backspace()),
            Key::Delete if word => return edited(self.delete_word_forward()),
            Key::Delete => return edited(self.delete()),
            Key::Left if word => EditMotion::WordLeft,
            Key::Left => EditMotion::Left,
            Key::Right if word => EditMotion::WordRight,
            Key::Right => EditMotion::Right,
            Key::Up if multi => EditMotion::Up,
            Key::Down if multi => EditMotion::Down,
            Key::PageUp if multi => EditMotion::PageUp(page_rows),
            Key::PageDown if multi => EditMotion::PageDown(page_rows),
            Key::Home if key.ctrl => EditMotion::Start,
            Key::Home => EditMotion::LineStart,
            Key::End if key.ctrl => EditMotion::End,
            Key::End => EditMotion::LineEnd,
            _ => return EditOutcome::Unhandled,
        };
        self.move_cursors(motion, key.shift);
        EditOutcome::Moved
    }
}
