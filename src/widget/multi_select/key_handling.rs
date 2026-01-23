//! Key handling for the multi-select widget

use crate::event::Key;

use super::types::MultiSelect;

impl MultiSelect {
    /// Handle key input, returns true if needs redraw
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.state.disabled {
            return false;
        }

        match key {
            // Open/close/select
            Key::Enter => {
                if self.open {
                    if let Some(idx) = self.current_option() {
                        self.toggle_option(idx);
                    }
                } else {
                    self.open();
                }
                true
            }

            Key::Escape => {
                if self.open {
                    self.close();
                    true
                } else if self.tag_cursor.is_some() {
                    self.tag_cursor = None;
                    true
                } else {
                    false
                }
            }

            Key::Char(' ') if self.open && !self.searchable => {
                if let Some(idx) = self.current_option() {
                    self.toggle_option(idx);
                }
                true
            }

            // Dropdown navigation
            Key::Down | Key::Char('j') if self.open => {
                self.cursor_down();
                true
            }

            Key::Up | Key::Char('k') if self.open => {
                self.cursor_up();
                true
            }

            // Tag navigation
            Key::Left if !self.open => {
                self.tag_cursor_left();
                true
            }

            Key::Right if !self.open => {
                self.tag_cursor_right();
                true
            }

            // Delete tag
            Key::Backspace if !self.open => {
                if self.tag_cursor.is_some() {
                    self.remove_tag_at_cursor();
                } else if !self.selected.is_empty() {
                    self.remove_last_tag();
                }
                true
            }

            Key::Backspace if self.open && self.searchable => {
                self.query.pop();
                self.update_filter();
                true
            }

            Key::Delete if !self.open && self.tag_cursor.is_some() => {
                self.remove_tag_at_cursor();
                true
            }

            // Search typing
            Key::Char(c) if self.open && self.searchable => {
                self.query.push(*c);
                self.update_filter();
                true
            }

            // Select all
            Key::Char('a') if !self.open => {
                self.select_all();
                true
            }

            // Clear selection
            Key::Char('c') if !self.open => {
                self.clear_selection();
                true
            }

            _ => false,
        }
    }
}
