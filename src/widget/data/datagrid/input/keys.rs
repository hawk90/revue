//! DataGrid key handling

use crate::event::{Key, KeyEvent};
use crate::widget::data::datagrid::DataGrid;

impl DataGrid {
    /// Handle a key with its modifiers: while a cell is edited, the
    /// standard editing keys every single-line field shares (#985), word
    /// moves and deletes, Shift selection, Ctrl+A, undo and redo included;
    /// otherwise as [`handle_key`](Self::handle_key)
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        if self.edit_state.active {
            return self.handle_edit_key_event(event);
        }
        self.handle_key(&event.key)
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        // If editing, delegate to edit handler
        if self.edit_state.active {
            return self.handle_edit_key(key);
        }

        match key {
            Key::Up | Key::Char('k') => {
                self.select_prev();
                true
            }
            Key::Down | Key::Char('j') => {
                self.select_next();
                true
            }
            Key::Left | Key::Char('h') => {
                self.select_prev_col();
                true
            }
            Key::Right | Key::Char('l') => {
                self.select_next_col();
                true
            }
            Key::PageDown => {
                let page = self.last_viewport_height.get().max(1);
                self.page_down(page);
                true
            }
            Key::PageUp => {
                let page = self.last_viewport_height.get().max(1);
                self.page_up(page);
                true
            }
            Key::Home | Key::Char('g') => {
                self.select_first();
                true
            }
            Key::End | Key::Char('G') => {
                self.select_last();
                true
            }
            Key::Enter => {
                // Try to start editing, fall back to sort
                if !self.start_edit() {
                    self.sort(self.selected_col);
                }
                true
            }
            Key::Char(' ') if self.options.multi_select => {
                self.toggle_selection();
                true
            }
            _ => false,
        }
    }
}
