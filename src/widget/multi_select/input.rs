//! Key handling for the multi-select widget: Enter and Escape in both
//! states, the dropdown keys while it is open and the tag keys while it is
//! closed

use crate::event::{Key, KeyEvent};
use crate::widget::traits::{EventResult, Interactive};

use super::MultiSelect;

impl Interactive for MultiSelect {
    fn handle_key(&mut self, event: &crate::event::KeyEvent) -> EventResult {
        // When closed and Tab pressed, let the focus manager handle navigation
        if event.key == Key::Tab && !self.open {
            return EventResult::Ignored;
        }

        let changed = self.handle_key_event(event);
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
    /// Handle a key without modifiers, as
    /// [`handle_key_event`](Self::handle_key_event) does
    pub fn handle_key(&mut self, key: &Key) -> bool {
        self.handle_key_event(&KeyEvent::new(*key))
    }

    /// Handle a key with its modifiers; returns true if it needs a redraw.
    /// While the dropdown is open and searchable, the keys other than
    /// Enter, Escape, Up, Down and Tab edit the query as every single-line
    /// field does. Otherwise a letter with Ctrl or Alt is the app's
    /// shortcut, not a letter or a list key (with both it is AltGr: a
    /// typed character).
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        if self.state.disabled {
            return false;
        }
        let key = &event.key;
        let editing = self.open && self.searchable;
        if !editing && matches!(key, Key::Char(_)) && event.ctrl != event.alt {
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

            _ if self.open => self.handle_dropdown_key(event),
            _ => self.handle_tag_key(key),
        }
    }

    /// A key other than Enter and Escape while the dropdown is open
    fn handle_dropdown_key(&mut self, event: &KeyEvent) -> bool {
        match &event.key {
            Key::Char(' ') if !self.searchable => {
                if let Some(idx) = self.current_option() {
                    self.toggle_option(idx);
                }
                true
            }

            // Dropdown navigation; j and k only when they are not search text
            Key::Down => {
                self.cursor_down();
                true
            }
            Key::Char('j') if !self.searchable => {
                self.cursor_down();
                true
            }

            Key::Up => {
                self.cursor_up();
                true
            }
            Key::Char('k') if !self.searchable => {
                self.cursor_up();
                true
            }

            // Close dropdown on Tab when open
            Key::Tab => {
                self.close();
                true
            }

            // The other keys edit the query
            _ if self.searchable => self.edit_query(event),

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
