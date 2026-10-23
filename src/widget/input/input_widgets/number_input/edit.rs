//! Direct entry: the edit buffer and its cursor, and the keys

use super::NumberInput;
use crate::event::{Key, KeyEvent};
use crate::utils::EditOutcome;

impl NumberInput {
    /// Enter direct editing mode
    pub fn start_editing(&mut self) {
        if !self.editing {
            self.editing = true;
            self.edit.set_text(self.format_value());
            let end = self.edit.end();
            self.edit.set_cursor(end);
        }
    }

    /// Commit the edited value and exit editing mode
    pub fn commit_edit(&mut self) {
        if self.editing {
            if let Ok(value) = self.buffer().parse::<f64>() {
                self.value = self.clamp_value(value);
            }
            self.editing = false;
            self.edit.set_text("");
        }
    }

    /// Cancel editing and restore original value
    pub fn cancel_edit(&mut self) {
        self.editing = false;
        self.edit.set_text("");
    }

    /// Check if in editing mode
    pub fn is_editing(&self) -> bool {
        self.editing
    }

    /// The text typed in direct editing mode
    pub(super) fn buffer(&self) -> &str {
        self.edit.line(0).unwrap_or("")
    }

    /// Handle a key without modifiers, as
    /// [`handle_key_event`](Self::handle_key_event) does
    pub fn handle_key(&mut self, key: &Key) -> bool {
        self.handle_key_event(&KeyEvent::new(*key))
    }

    /// Handle a key: the step keys (Up/Down and `k`/`j`, PageUp/PageDown,
    /// Home/End for the minimum and maximum) commit an edit and step;
    /// Enter starts or commits an edit and Escape cancels it; a digit, `.`
    /// or `-` starts an edit or is typed. While editing, the other standard
    /// editing keys every single-line field shares work too (#985):
    /// Backspace, Delete, Left, Right, word moves and deletes, Shift
    /// selection, Ctrl+A, undo and redo. Returns true if the value changed
    /// or a redraw is needed.
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        if self.state.disabled {
            return false;
        }
        let plain = !event.ctrl && !event.alt;

        // The step keys commit an edit first
        let step: Option<fn(&mut Self)> = match event.key {
            _ if !plain => None,
            Key::Up | Key::Char('k') => Some(Self::increment),
            Key::Down | Key::Char('j') => Some(Self::decrement),
            Key::PageUp => Some(Self::increment_large),
            Key::PageDown => Some(Self::decrement_large),
            Key::Home => Some(Self::set_to_min),
            Key::End => Some(Self::set_to_max),
            _ => None,
        };
        if let Some(step) = step {
            if self.editing {
                self.commit_edit();
            }
            step(self);
            return true;
        }

        match event.key {
            // Enter editing mode or commit
            Key::Enter => {
                if self.editing {
                    self.commit_edit();
                } else {
                    self.start_editing();
                }
                true
            }

            // Cancel editing
            Key::Escape if self.editing => {
                self.cancel_edit();
                true
            }

            // Direct numeric input
            Key::Char(c) if plain && (c.is_ascii_digit() || c == '.' || c == '-') => {
                if !self.editing {
                    self.start_editing();
                    self.edit.set_text("");
                }
                if self.accepts(c) {
                    self.edit.insert(c.encode_utf8(&mut [0; 4]));
                }
                true
            }

            // Other characters are not typed
            Key::Char(_) if plain || (event.ctrl && event.alt) => false,

            _ if self.editing => self.edit.edit_key(event, 1) != EditOutcome::Unhandled,
            _ => false,
        }
    }

    /// Whether `c` may be typed at the cursor, replacing any selection: one
    /// `-`, at the start, and one `.`
    fn accepts(&self, c: char) -> bool {
        let (from, to) = self.edit.primary().range();
        let (from, to) = (self.edit.char_index(from), self.edit.char_index(to));
        let rest = self
            .buffer()
            .chars()
            .enumerate()
            .filter(|&(i, _)| i < from || i >= to)
            .map(|(_, ch)| ch);
        match c {
            '-' => from == 0 && !rest.clone().any(|ch| ch == '-'),
            '.' => !rest.clone().any(|ch| ch == '.'),
            _ => true,
        }
    }
}
