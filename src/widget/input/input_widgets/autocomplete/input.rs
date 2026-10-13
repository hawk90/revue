//! Keys: editing the value, moving through and accepting suggestions

use crate::event::{Key, KeyEvent};

use super::Autocomplete;

impl Autocomplete {
    /// Handle key event
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.key {
            Key::Char(c) => {
                let byte_idx = crate::utils::text::char_to_byte_index(&self.value, self.cursor);
                self.value.insert(byte_idx, c);
                self.cursor += 1;
                self.update_filter();
                true
            }
            Key::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    let byte_idx = crate::utils::text::char_to_byte_index(&self.value, self.cursor);
                    self.value.remove(byte_idx);
                    self.update_filter();
                }
                true
            }
            Key::Delete => {
                if self.cursor < self.value.chars().count() {
                    let byte_idx = crate::utils::text::char_to_byte_index(&self.value, self.cursor);
                    self.value.remove(byte_idx);
                    self.update_filter();
                }
                true
            }
            Key::Left => {
                self.cursor = self.cursor.saturating_sub(1);
                true
            }
            Key::Right => {
                self.cursor = (self.cursor + 1).min(self.value.chars().count());
                true
            }
            Key::Home => {
                self.cursor = 0;
                true
            }
            Key::End => {
                self.cursor = self.value.chars().count();
                true
            }
            Key::Up if self.dropdown_visible => {
                self.selection.up();
                true
            }
            Key::Down if self.dropdown_visible => {
                self.selection.down();
                true
            }
            Key::Enter | Key::Tab if self.dropdown_visible => {
                self.accept_selection();
                true
            }
            Key::Escape if self.dropdown_visible => {
                self.dropdown_visible = false;
                true
            }
            _ => false,
        }
    }
}
