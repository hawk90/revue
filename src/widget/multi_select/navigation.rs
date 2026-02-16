//! Navigation for the multi-select widget

use super::types::MultiSelect;

impl MultiSelect {
    /// Move dropdown cursor down
    pub fn cursor_down(&mut self) {
        if self.filtered.is_empty() {
            return;
        }
        self.dropdown_cursor = (self.dropdown_cursor + 1) % self.filtered.len();
    }

    /// Move dropdown cursor up
    pub fn cursor_up(&mut self) {
        if self.filtered.is_empty() {
            return;
        }
        self.dropdown_cursor = self
            .dropdown_cursor
            .checked_sub(1)
            .unwrap_or(self.filtered.len() - 1);
    }

    /// Move tag cursor left
    pub fn tag_cursor_left(&mut self) {
        if self.selected.is_empty() {
            return;
        }
        match self.tag_cursor {
            None => self.tag_cursor = Some(self.selected.len() - 1),
            Some(0) => {} // Already at start
            Some(pos) => self.tag_cursor = Some(pos - 1),
        }
    }

    /// Move tag cursor right
    pub fn tag_cursor_right(&mut self) {
        match self.tag_cursor {
            None => {}
            Some(pos) if pos >= self.selected.len() - 1 => self.tag_cursor = None,
            Some(pos) => self.tag_cursor = Some(pos + 1),
        }
    }

    /// Get current dropdown option index
    pub fn current_option(&self) -> Option<usize> {
        self.filtered.get(self.dropdown_cursor).copied()
    }
}
