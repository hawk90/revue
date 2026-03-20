//! State management for Select

use super::Select;

impl Select {
    /// Check if dropdown is open
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Toggle dropdown open/close
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    /// Open the dropdown
    pub fn open(&mut self) {
        self.open = true;
    }

    /// Close the dropdown
    pub fn close(&mut self) {
        self.open = false;
    }

    /// Select next option
    pub fn select_next(&mut self) {
        self.selection.next();
    }

    /// Select previous option
    pub fn select_prev(&mut self) {
        self.selection.prev();
    }

    /// Select first option
    pub fn select_first(&mut self) {
        self.selection.first();
    }

    /// Select last option
    pub fn select_last(&mut self) {
        self.selection.last();
    }

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.selection.index
    }

    /// Get selected value
    pub fn value(&self) -> Option<&str> {
        self.options.get(self.selection.index).map(|s| s.as_str())
    }

    /// Get selected value (alias for [`value`](Self::value))
    pub fn get_value(&self) -> Option<&str> {
        self.value()
    }

    /// Get current search query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Set search query and update filter
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.update_filter();
    }

    /// Clear search query
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.reset_filter();
    }

    /// Check if searchable mode is enabled
    pub fn is_searchable(&self) -> bool {
        self.searchable
    }

    /// Get filtered options (indices into original options)
    pub fn filtered_options(&self) -> &[usize] {
        &self.filtered
    }

    /// Get number of visible (filtered) options
    pub fn visible_count(&self) -> usize {
        if self.query.is_empty() {
            self.options.len()
        } else {
            self.filtered.len()
        }
    }

    /// Get number of options
    pub fn len(&self) -> usize {
        self.options.len()
    }

    /// Check if select has no options
    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }

    /// Handle key input, returns true if selection changed
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        if !self.focused {
            return false;
        }
        use crate::event::Key;

        match key {
            Key::Enter => {
                if self.open {
                    self.close();
                    self.clear_query();
                } else {
                    self.open();
                }
                false
            }
            Key::Char(' ') if !self.searchable => {
                self.toggle();
                false
            }
            Key::Up | Key::Char('k') if self.open && !self.searchable => {
                let old = self.selection.index;
                self.select_prev();
                old != self.selection.index
            }
            Key::Down | Key::Char('j') if self.open && !self.searchable => {
                let old = self.selection.index;
                self.select_next();
                old != self.selection.index
            }
            Key::Up if self.open && self.searchable => {
                let old = self.selection.index;
                if self.query.is_empty() {
                    self.select_prev();
                } else {
                    self.select_prev_filtered();
                }
                old != self.selection.index
            }
            Key::Down if self.open && self.searchable => {
                let old = self.selection.index;
                if self.query.is_empty() {
                    self.select_next();
                } else {
                    self.select_next_filtered();
                }
                old != self.selection.index
            }
            Key::Escape if self.open => {
                self.close();
                self.clear_query();
                false
            }
            Key::Backspace if self.open && self.searchable => {
                self.query.pop();
                self.update_filter();
                true
            }
            Key::Char(c) if self.open && self.searchable => {
                self.query.push(*c);
                self.update_filter();
                true
            }
            Key::Home if self.open => {
                let old = self.selection.index;
                self.select_first();
                old != self.selection.index
            }
            Key::End if self.open => {
                let old = self.selection.index;
                self.select_last();
                old != self.selection.index
            }
            _ => false,
        }
    }
}
