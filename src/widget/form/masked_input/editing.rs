//! Editing the value: inserting, deleting, moving the cursor and the peek countdown

use super::{MaskStyle, MaskedInput};

impl MaskedInput {
    /// Insert character at cursor
    pub fn insert_char(&mut self, c: char) {
        if self.disabled {
            return;
        }

        // Check max length
        if self.max_length > 0 && self.char_count() >= self.max_length {
            return;
        }

        self.value.insert(self.byte_at(self.cursor), c);
        self.cursor += 1;

        // Start peek countdown
        if matches!(self.mask_style, MaskStyle::Peek) {
            self.peek_countdown = self.peek_timeout;
        }
    }

    /// Delete character before cursor
    pub fn delete_backward(&mut self) {
        if self.disabled || self.cursor == 0 {
            return;
        }

        self.cursor -= 1;
        self.value.remove(self.byte_at(self.cursor));
    }

    /// Delete character at cursor
    pub fn delete_forward(&mut self) {
        if self.disabled || self.cursor >= self.char_count() {
            return;
        }

        self.value.remove(self.byte_at(self.cursor));
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        if self.cursor < self.char_count() {
            self.cursor += 1;
        }
    }

    /// Move cursor to start
    pub fn move_start(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to end
    pub fn move_end(&mut self) {
        self.cursor = self.char_count();
    }

    /// Update (call each frame for peek mode)
    pub fn update(&mut self) {
        if self.peek_countdown > 0 {
            self.peek_countdown -= 1;
        }
    }
}
