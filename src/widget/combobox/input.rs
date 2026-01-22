//! Input handling methods for Combobox

use super::super::Combobox;

impl Combobox {
    // ─────────────────────────────────────────────────────────────────────────
    // Input handling
    // ─────────────────────────────────────────────────────────────────────────

    /// Insert character at cursor
    pub fn insert_char(&mut self, c: char) {
        let byte_idx = self.char_to_byte_index(self.cursor);
        self.input.insert(byte_idx, c);
        self.cursor += 1;
        self.update_filter();
        self.open = true;
    }

    /// Delete character before cursor (backspace)
    pub fn delete_backward(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            let byte_idx = self.char_to_byte_index(self.cursor);
            if let Some((_, ch)) = self.input.char_indices().nth(self.cursor) {
                self.input.drain(byte_idx..byte_idx + ch.len_utf8());
            }
            self.update_filter();
        }
    }

    /// Delete character at cursor (delete)
    pub fn delete_forward(&mut self) {
        let char_count = self.input.chars().count();
        if self.cursor < char_count {
            let byte_idx = self.char_to_byte_index(self.cursor);
            if let Some((_, ch)) = self.input.char_indices().nth(self.cursor) {
                self.input.drain(byte_idx..byte_idx + ch.len_utf8());
            }
            self.update_filter();
        }
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        let char_count = self.input.chars().count();
        if self.cursor < char_count {
            self.cursor += 1;
        }
    }

    /// Move cursor to start
    pub fn move_to_start(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to end
    pub fn move_to_end(&mut self) {
        self.cursor = self.input.chars().count();
    }

    /// Handle key event
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        match key {
            Key::Char(c) => {
                self.insert_char(*c);
                true
            }
            Key::Backspace => {
                self.delete_backward();
                true
            }
            Key::Delete => {
                self.delete_forward();
                true
            }
            Key::Left => {
                self.move_left();
                false
            }
            Key::Right => {
                self.move_right();
                false
            }
            Key::Home => {
                self.move_to_start();
                false
            }
            Key::End => {
                self.move_to_end();
                false
            }
            Key::Up if self.open => {
                self.select_prev();
                true
            }
            Key::Down if self.open => {
                self.select_next();
                true
            }
            Key::Down if !self.open => {
                self.open_dropdown();
                true
            }
            Key::Enter if self.open => {
                self.select_current();
                true
            }
            Key::Enter if !self.open && self.allow_custom => {
                // Accept custom value
                true
            }
            Key::Escape if self.open => {
                self.close_dropdown();
                true
            }
            Key::Tab if self.open && !self.filtered.is_empty() => {
                // Tab completion: fill with highlighted option
                if let Some(&option_idx) = self.filtered.get(self.selected_idx) {
                    self.input = self.options[option_idx].label.clone();
                    self.cursor = self.input.chars().count();
                    self.update_filter();
                }
                true
            }
            _ => false,
        }
    }
}

impl Combobox {
    // ─────────────────────────────────────────────────────────────────────────
    // Helpers
    // ─────────────────────────────────────────────────────────────────────────

    /// Convert character index to byte index
    pub(super) fn char_to_byte_index(&self, char_idx: usize) -> usize {
        self.input
            .char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(self.input.len())
    }

    /// Calculate display width
    pub(super) fn display_width(&self, max_width: u16) -> u16 {
        if let Some(w) = self.width {
            return w.min(max_width);
        }

        let max_option_len = self
            .options
            .iter()
            .map(|o| o.label.len())
            .max()
            .unwrap_or(self.placeholder.len());

        // +4 for padding and borders
        ((max_option_len.max(20) + 4) as u16).min(max_width)
    }
}
