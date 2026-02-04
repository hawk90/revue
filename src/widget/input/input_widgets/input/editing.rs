//! Text editing operations for the Input widget

use super::types::{EditOperation, Input};

impl Input {
    // ─────────────────────────────────────────────────────────────────────────
    // Word navigation and editing
    // ─────────────────────────────────────────────────────────────────────────

    /// Move cursor to the left by one word (zero allocation)
    pub(super) fn move_word_left(&mut self) {
        if self.cursor == 0 {
            return;
        }

        // Get substring before cursor and iterate in reverse (no allocation)
        let byte_pos = self.char_to_byte_index(self.cursor);
        let before_cursor = &self.value[..byte_pos];

        let mut new_pos = self.cursor;

        // Skip whitespace going backwards
        for ch in before_cursor.chars().rev() {
            if ch.is_whitespace() {
                new_pos -= 1;
            } else {
                break;
            }
        }

        // Skip word characters going backwards
        let byte_pos = self.char_to_byte_index(new_pos);
        let before_new_pos = &self.value[..byte_pos];
        for ch in before_new_pos.chars().rev() {
            if !ch.is_whitespace() {
                new_pos -= 1;
            } else {
                break;
            }
        }

        self.cursor = new_pos;
    }

    /// Move cursor to the right by one word (zero allocation)
    pub(super) fn move_word_right(&mut self) {
        let char_len = self.char_count();
        if self.cursor >= char_len {
            return;
        }

        // Get substring after cursor (no allocation)
        let byte_pos = self.char_to_byte_index(self.cursor);
        let after_cursor = &self.value[byte_pos..];

        let mut advance = 0;

        // Skip current word characters
        for ch in after_cursor.chars() {
            if !ch.is_whitespace() {
                advance += 1;
            } else {
                break;
            }
        }

        // Get remaining substring and skip whitespace
        let new_byte_pos = self.char_to_byte_index(self.cursor + advance);
        let remaining = &self.value[new_byte_pos..];
        for ch in remaining.chars() {
            if ch.is_whitespace() {
                advance += 1;
            } else {
                break;
            }
        }

        self.cursor = (self.cursor + advance).min(char_len);
    }

    /// Delete word to the left with undo support
    pub(super) fn delete_word_left(&mut self) {
        if self.cursor == 0 {
            return;
        }

        let end = self.cursor;
        self.move_word_left();
        let start = self.cursor;

        // Get deleted text for undo
        let deleted = self.substring(start, end).to_string();
        self.push_undo(EditOperation::Delete {
            pos: start,
            text: deleted,
        });

        // Delete characters between new cursor position and old cursor position
        self.remove_char_range(start, end);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Value management
    // ─────────────────────────────────────────────────────────────────────────

    /// Clear the input (also clears undo history)
    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor = 0;
        self.clear_selection();
        self.clear_history();
    }

    /// Set value programmatically (also clears undo history)
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
        self.cursor = self.char_count();
        self.clear_selection();
        self.clear_history();
    }
}
