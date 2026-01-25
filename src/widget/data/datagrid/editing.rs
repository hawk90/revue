//! DataGrid cell editing

use super::core::DataGrid;
use crate::event::Key;

/// Cell editing state
#[derive(Clone, Debug, Default)]
pub struct EditState {
    /// Currently editing
    pub active: bool,
    /// Row being edited (actual index, not filtered)
    pub row: usize,
    /// Column being edited
    pub col: usize,
    /// Edit buffer
    pub buffer: String,
    /// Cursor position in buffer
    pub cursor: usize,
}

impl DataGrid {
    /// Check if currently editing a cell
    pub fn is_editing(&self) -> bool {
        self.edit_state.active
    }

    /// Start editing the selected cell
    pub fn start_edit(&mut self) -> bool {
        // Early bounds check - no clone needed, just copy indices
        let selected_row = self.selected_row;
        let selected_col = self.selected_col;

        if selected_col >= self.columns.len() {
            return false;
        }

        if !self.columns[selected_col].editable {
            return false;
        }

        // Get actual row index from filtered cache (zero-copy access)
        let row_idx = match self.filtered_cache.get(selected_row) {
            Some(&idx) => idx,
            None => return false,
        };

        if row_idx >= self.rows.len() {
            return false;
        }

        // Get current cell value
        let col_key = &self.columns[selected_col].key;
        let value = self.rows[row_idx].get(col_key).unwrap_or("").to_string();

        self.edit_state = EditState {
            active: true,
            row: row_idx,
            col: selected_col,
            cursor: value.chars().count(),
            buffer: value,
        };
        true
    }

    /// Commit the current edit
    pub fn commit_edit(&mut self) -> bool {
        if !self.edit_state.active {
            return false;
        }

        // Validate bounds before accessing
        if self.edit_state.col >= self.columns.len() {
            self.edit_state.active = false;
            return false;
        }
        if self.edit_state.row >= self.rows.len() {
            self.edit_state.active = false;
            return false;
        }

        let col_key = self.columns[self.edit_state.col].key.clone();
        let row = &mut self.rows[self.edit_state.row];

        // Update the cell value
        if let Some(cell) = row.data.iter_mut().find(|(k, _)| k == &col_key) {
            cell.1 = self.edit_state.buffer.clone();
        } else {
            row.data.push((col_key, self.edit_state.buffer.clone()));
        }

        self.edit_state.active = false;
        self.recompute_cache();
        true
    }

    /// Cancel the current edit
    pub fn cancel_edit(&mut self) {
        self.edit_state.active = false;
    }

    /// Get the current edit buffer
    pub fn edit_buffer(&self) -> Option<&str> {
        if self.edit_state.active {
            Some(&self.edit_state.buffer)
        } else {
            None
        }
    }

    /// Handle key input in edit mode
    pub(super) fn handle_edit_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Escape => {
                self.cancel_edit();
                true
            }
            Key::Enter => {
                self.commit_edit();
                true
            }
            Key::Char(c) => {
                let pos = self.edit_state.cursor;
                self.edit_state.buffer.insert(
                    self.edit_state
                        .buffer
                        .char_indices()
                        .nth(pos)
                        .map(|(i, _)| i)
                        .unwrap_or(self.edit_state.buffer.len()),
                    *c,
                );
                self.edit_state.cursor += 1;
                true
            }
            Key::Backspace => {
                if self.edit_state.cursor > 0 {
                    self.edit_state.cursor -= 1;
                    let byte_pos = self
                        .edit_state
                        .buffer
                        .char_indices()
                        .nth(self.edit_state.cursor)
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    if let Some((_, ch)) = self
                        .edit_state
                        .buffer
                        .char_indices()
                        .nth(self.edit_state.cursor)
                    {
                        let end = byte_pos + ch.len_utf8();
                        self.edit_state.buffer.drain(byte_pos..end);
                    }
                }
                true
            }
            Key::Delete => {
                let char_count = self.edit_state.buffer.chars().count();
                if self.edit_state.cursor < char_count {
                    let byte_pos = self
                        .edit_state
                        .buffer
                        .char_indices()
                        .nth(self.edit_state.cursor)
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    if let Some((_, ch)) = self
                        .edit_state
                        .buffer
                        .char_indices()
                        .nth(self.edit_state.cursor)
                    {
                        let end = byte_pos + ch.len_utf8();
                        self.edit_state.buffer.drain(byte_pos..end);
                    }
                }
                true
            }
            Key::Left => {
                if self.edit_state.cursor > 0 {
                    self.edit_state.cursor -= 1;
                }
                true
            }
            Key::Right => {
                let char_count = self.edit_state.buffer.chars().count();
                if self.edit_state.cursor < char_count {
                    self.edit_state.cursor += 1;
                }
                true
            }
            Key::Home => {
                self.edit_state.cursor = 0;
                true
            }
            Key::End => {
                self.edit_state.cursor = self.edit_state.buffer.chars().count();
                true
            }
            _ => false,
        }
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
                self.page_down(10);
                true
            }
            Key::PageUp => {
                self.page_up(10);
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
