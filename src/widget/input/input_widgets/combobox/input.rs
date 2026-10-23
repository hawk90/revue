//! Input handling methods for Combobox

use super::super::dropdown::MIN_DROPDOWN_WIDTH;
use super::super::Combobox;

use crate::event::{Key, KeyEvent};
use crate::utils::{EditMotion, EditOutcome};

impl Combobox {
    // ─────────────────────────────────────────────────────────────────────────
    // Input handling
    // ─────────────────────────────────────────────────────────────────────────

    /// Insert character at cursor
    pub fn insert_char(&mut self, c: char) {
        if self.state.insert(c.encode_utf8(&mut [0; 4])) {
            self.edited();
        }
    }

    /// Delete character before cursor (backspace)
    pub fn delete_backward(&mut self) {
        if self.state.backspace() {
            self.update_filter();
        }
    }

    /// Delete character at cursor (delete)
    pub fn delete_forward(&mut self) {
        if self.state.delete() {
            self.update_filter();
        }
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        self.state.move_cursors(EditMotion::Left, false);
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        self.state.move_cursors(EditMotion::Right, false);
    }

    /// Move cursor to start
    pub fn move_to_start(&mut self) {
        self.state.move_cursors(EditMotion::LineStart, false);
    }

    /// Move cursor to end
    pub fn move_to_end(&mut self) {
        self.state.move_cursors(EditMotion::LineEnd, false);
    }

    /// Handle a key without modifiers, as
    /// [`handle_key_event`](Self::handle_key_event) does
    pub fn handle_key(&mut self, key: &Key) -> bool {
        self.handle_key_event(&KeyEvent::new(*key))
    }

    /// Handle a key: Up, Down, Enter, Escape and Tab work the dropdown (see
    /// the table on [`Combobox`]); the other keys are the standard editing
    /// keys every single-line field shares (#985), and an edit opens the
    /// dropdown. Returns whether the key was the combobox's, a cursor move
    /// included.
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        // The dropdown's keys whatever the modifiers, as before
        if self.dropdown_key(event.key) {
            return true;
        }
        match self.state.edit_key(event, 1) {
            EditOutcome::Edited => {
                self.edited();
                true
            }
            EditOutcome::Moved => true,
            EditOutcome::Unhandled => false,
        }
    }

    /// The dropdown's keys; false for any other key
    fn dropdown_key(&mut self, key: Key) -> bool {
        match key {
            Key::Up if self.open => self.select_prev(),
            Key::Down if self.open => self.select_next(),
            Key::Down => self.open_dropdown(),
            Key::Enter if self.open => {
                self.select_current();
            }
            // Accept custom value
            Key::Enter if self.allow_custom => {}
            Key::Escape if self.open => self.close_dropdown(),
            Key::Tab if self.open && !self.filtered.is_empty() => {
                // Tab completion: fill with highlighted option and close dropdown
                if let Some(&option_idx) = self.filtered.get(self.selected_idx) {
                    self.replace_input(self.options[option_idx].label.clone());
                    self.update_filter();
                }
                self.close_dropdown();
            }
            _ => return false,
        }
        true
    }

    /// After the text changed: filter again and show the matches
    fn edited(&mut self) {
        self.update_filter();
        self.open = true;
    }
}

impl Combobox {
    // ─────────────────────────────────────────────────────────────────────────
    // Helpers
    // ─────────────────────────────────────────────────────────────────────────

    /// Replace the text, with the cursor at its end
    pub(super) fn replace_input(&mut self, text: String) {
        self.state.set_text(text);
        let end = self.state.end();
        self.state.set_cursor(end);
    }

    /// The cursor as a char index into the text
    #[cfg(test)]
    pub(super) fn cursor_char(&self) -> usize {
        self.state.char_index(self.state.cursor())
    }

    /// Calculate display width
    pub(super) fn display_width(&self, max_width: u16) -> u16 {
        if let Some(w) = self.width {
            // The header needs room for its arrow; a smaller configured
            // width would leave none.
            return w.max(MIN_DROPDOWN_WIDTH).min(max_width);
        }

        let max_option_len = self
            .options
            .iter()
            .map(|o| crate::utils::display_width(&o.label))
            .max()
            .unwrap_or(crate::utils::display_width(&self.placeholder));

        // +4 for padding and borders
        ((max_option_len.max(20) + 4) as u16).min(max_width)
    }
}
