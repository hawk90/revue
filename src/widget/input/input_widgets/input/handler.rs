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
            Key::Backspace => Some(self.delete_word_left()),
            _ => None,
        }
    }

    /// Handle key event with modifiers, returns true if needs redraw.
    /// Returns false immediately if the widget is not focused.
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        if !self.focused {
            return false;
        }
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

        // Ctrl or Alt with a letter is a command, and this one is not
        // bound: leave it to the app rather than type the letter. Ctrl+Alt
        // with a letter is text, as Windows reports AltGr.
        if matches!(event.key, Key::Char(_)) && event.ctrl != event.alt {
            return false;
        }

        // Regular key handling (clears selection on most actions)
        self.handle_key(&event.key)
    }

    /// Handle key input (without modifiers), returns true if value changed
    pub fn handle_key(&mut self, key: &Key) -> bool {
        let char_len = self.char_count();

        match key {
            Key::Char(c) if !c.is_control() => {
                // Typing over a selection replaces it as one undo step,
                // the same as pasting over it.
                self.paste_text(&c.to_string());
                true
            }
            Key::Backspace => {
                if self.has_selection() {
                    self.delete_selection_with_undo()
                } else if self.cursor > 0 {
                    // An empty selection (anchor at the cursor) must not
                    // outlive the edit and turn into a stale range
                    self.clear_selection();
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
                    self.clear_selection();
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
