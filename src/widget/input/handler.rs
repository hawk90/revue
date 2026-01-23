//! Key event handlers for the Input widget

use super::types::{EditOperation, Input};
use crate::event::{Key, KeyEvent};

impl Input {
    /// Handle Ctrl+key combinations
    ///
    /// Returns `Some(true)` if handled and needs redraw,
    /// `Some(false)` if handled but no redraw needed,
    /// `None` if not handled.
    pub(super) fn handle_ctrl_key(&mut self, event: &KeyEvent) -> Option<bool> {
        match event.key {
            Key::Char('a') => {
                self.select_all();
                Some(true)
            }
            Key::Char('c') => {
                self.copy();
                Some(true)
            }
            Key::Char('x') => Some(self.cut()),
            Key::Char('v') => Some(self.paste()),
            Key::Char('z') => Some(self.undo()),
            Key::Char('y') => Some(self.redo()),
            Key::Left => {
                // Move to previous word (with optional selection)
                if event.shift {
                    self.start_selection();
                } else {
                    self.clear_selection();
                }
                self.move_word_left();
                Some(true)
            }
            Key::Right => {
                // Move to next word (with optional selection)
                if event.shift {
                    self.start_selection();
                } else {
                    self.clear_selection();
                }
                self.move_word_right();
                Some(true)
            }
            Key::Backspace => {
                self.delete_word_left();
                Some(true)
            }
            _ => None,
        }
    }

    /// Handle key event with modifiers, returns true if needs redraw
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        // Try Ctrl combinations first
        if event.ctrl {
            if let Some(handled) = self.handle_ctrl_key(event) {
                return handled;
            }
        }

        // Try Shift combinations for selection
        if event.shift {
            if let Some(handled) = self.handle_shift_key(&event.key) {
                return handled;
            }
        }

        // Regular key handling (clears selection on most actions)
        self.handle_key(&event.key)
    }

    /// Handle key input (without modifiers), returns true if value changed
    pub fn handle_key(&mut self, key: &Key) -> bool {
        let char_len = self.char_count();

        match key {
            Key::Char(c) => {
                self.delete_selection_with_undo();
                // Create a string from the character and insert at cursor
                let s = c.to_string();
                let pos = self.cursor;
                self.push_undo(EditOperation::Insert {
                    pos,
                    text: s.clone(),
                });
                self.cursor = self.insert_at_char(self.cursor, &s);
                true
            }
            Key::Backspace => {
                if self.has_selection() {
                    self.delete_selection_with_undo()
                } else if self.cursor > 0 {
                    self.cursor -= 1;
                    // Get the character to be deleted for undo
                    let deleted = self.substring(self.cursor, self.cursor + 1).to_string();
                    self.push_undo(EditOperation::Delete {
                        pos: self.cursor,
                        text: deleted,
                    });
                    self.remove_char_at(self.cursor);
                    true
                } else {
                    false
                }
            }
            Key::Delete => {
                if self.has_selection() {
                    self.delete_selection_with_undo()
                } else if self.cursor < char_len {
                    // Get the character to be deleted for undo
                    let deleted = self.substring(self.cursor, self.cursor + 1).to_string();
                    self.push_undo(EditOperation::Delete {
                        pos: self.cursor,
                        text: deleted,
                    });
                    self.remove_char_at(self.cursor);
                    true
                } else {
                    false
                }
            }
            Key::Left => {
                self.clear_selection();
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
                true
            }
            Key::Right => {
                self.clear_selection();
                if self.cursor < char_len {
                    self.cursor += 1;
                }
                true
            }
            Key::Home => {
                self.clear_selection();
                self.cursor = 0;
                true
            }
            Key::End => {
                self.clear_selection();
                self.cursor = char_len;
                true
            }
            _ => false,
        }
    }
}
