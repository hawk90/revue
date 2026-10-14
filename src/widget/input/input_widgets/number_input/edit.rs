//! Direct entry: the edit buffer and its cursor, and the keys

use super::NumberInput;
use crate::event::Key;

impl NumberInput {
    /// Enter direct editing mode
    pub fn start_editing(&mut self) {
        if !self.editing {
            self.editing = true;
            self.input_buffer = self.format_value();
            self.cursor = self.input_buffer.chars().count();
        }
    }

    /// Commit the edited value and exit editing mode
    pub fn commit_edit(&mut self) {
        if self.editing {
            if let Ok(value) = self.input_buffer.parse::<f64>() {
                self.value = self.clamp_value(value);
            }
            self.editing = false;
            self.input_buffer.clear();
        }
    }

    /// Cancel editing and restore original value
    pub fn cancel_edit(&mut self) {
        self.editing = false;
        self.input_buffer.clear();
    }

    /// Check if in editing mode
    pub fn is_editing(&self) -> bool {
        self.editing
    }

    fn char_to_byte_index(s: &str, char_idx: usize) -> usize {
        s.char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(s.len())
    }

    fn buffer_char_count(&self) -> usize {
        self.input_buffer.chars().count()
    }

    fn insert_char_at_cursor(&mut self, ch: char) {
        let byte_idx = Self::char_to_byte_index(&self.input_buffer, self.cursor);
        self.input_buffer.insert(byte_idx, ch);
        self.cursor += 1;
    }

    fn delete_char_before_cursor(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            let byte_idx = Self::char_to_byte_index(&self.input_buffer, self.cursor);
            // Get char directly from byte index for O(1) instead of O(n²) with .chars().nth()
            if let Some(ch) = self.input_buffer[byte_idx..].chars().next() {
                self.input_buffer.drain(byte_idx..byte_idx + ch.len_utf8());
            }
        }
    }

    fn delete_char_at_cursor(&mut self) {
        let char_count = self.buffer_char_count();
        if self.cursor < char_count {
            let byte_idx = Self::char_to_byte_index(&self.input_buffer, self.cursor);
            // Get char directly from byte index for O(1) instead of O(n²) with .chars().nth()
            if let Some(ch) = self.input_buffer[byte_idx..].chars().next() {
                self.input_buffer.drain(byte_idx..byte_idx + ch.len_utf8());
            }
        }
    }

    /// Handle key input, returns true if value changed or needs redraw
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.state.disabled {
            return false;
        }

        // The step keys commit an edit first
        let step: Option<fn(&mut Self)> = match key {
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

        match key {
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
            Key::Char(c) if c.is_ascii_digit() || *c == '.' || *c == '-' => {
                if !self.editing {
                    self.start_editing();
                    self.input_buffer.clear();
                    self.cursor = 0;
                }

                // Validate input
                let valid = match c {
                    '-' => self.cursor == 0 && !self.input_buffer.contains('-'),
                    '.' => !self.input_buffer.contains('.'),
                    _ => true,
                };

                if valid {
                    self.insert_char_at_cursor(*c);
                }
                true
            }

            // Backspace in editing mode
            Key::Backspace if self.editing => {
                self.delete_char_before_cursor();
                true
            }

            // Delete in editing mode
            Key::Delete if self.editing => {
                self.delete_char_at_cursor();
                true
            }

            // Cursor movement in editing mode
            Key::Left if self.editing && self.cursor > 0 => {
                self.cursor -= 1;
                true
            }
            Key::Right if self.editing && self.cursor < self.buffer_char_count() => {
                self.cursor += 1;
                true
            }

            _ => false,
        }
    }
}
