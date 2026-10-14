//! Key handling for the multi-select widget: Enter and Escape in both
//! states, the dropdown keys while it is open and the tag keys while it is
//! closed

use crate::event::Key;
use crate::widget::traits::{EventResult, Interactive};

use super::MultiSelect;

impl Interactive for MultiSelect {
    fn handle_key(&mut self, event: &crate::event::KeyEvent) -> EventResult {
        // When closed and Tab pressed, let the focus manager handle navigation
        if event.key == Key::Tab && !self.open {
            return EventResult::Ignored;
        }

        let changed = self.handle_key(&event.key);
        if changed {
            EventResult::ConsumedAndRender
        } else {
            EventResult::Ignored
        }
    }

    crate::impl_focus_handlers!(state, no_blur);

    fn on_blur(&mut self) {
        self.state.focused = false;
        if self.open {
            self.close();
        }
    }
}

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

            _ if self.open => self.handle_dropdown_key(key),
            _ => self.handle_tag_key(key),
        }
    }

    /// A key other than Enter and Escape while the dropdown is open
    fn handle_dropdown_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char(' ') if !self.searchable => {
                if let Some(idx) = self.current_option() {
                    self.toggle_option(idx);
                }
                true
            }

            // Dropdown navigation
            Key::Down | Key::Char('j') => {
                self.cursor_down();
                true
            }

            Key::Up | Key::Char('k') => {
                self.cursor_up();
                true
            }

            Key::Backspace if self.searchable => {
                self.query.pop();
                self.update_filter();
                true
            }

            // Search typing
            Key::Char(c) if self.searchable => {
                self.query.push(*c);
                self.update_filter();
                true
            }

            // Close dropdown on Tab when open
            Key::Tab => {
                self.close();
                true
            }

            _ => false,
        }
    }

    /// A key other than Enter and Escape while the dropdown is closed
    fn handle_tag_key(&mut self, key: &Key) -> bool {
        match key {
            // Tag navigation
            Key::Left => {
                self.tag_cursor_left();
                true
            }

            Key::Right => {
                self.tag_cursor_right();
                true
            }

            // Delete tag
            Key::Backspace => {
                if self.tag_cursor.is_some() {
                    self.remove_tag_at_cursor();
                } else if !self.selected.is_empty() {
                    self.remove_last_tag();
                }
                true
            }

            Key::Delete if self.tag_cursor.is_some() => {
                self.remove_tag_at_cursor();
                true
            }

            // Select all
            Key::Char('a') => {
                self.select_all();
                true
            }

            // Clear selection
            Key::Char('c') => {
                self.clear_selection();
                true
            }

            _ => false,
        }
    }
}
