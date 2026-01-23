//! Selection manipulation for the multi-select widget

use super::types::MultiSelect;

impl MultiSelect {
    /// Check if dropdown is open
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Get selected indices
    pub fn get_selected_indices(&self) -> &[usize] {
        &self.selected
    }

    /// Get selected values
    pub fn get_selected_values(&self) -> Vec<&str> {
        self.selected
            .iter()
            .filter_map(|&i| self.options.get(i).map(|o| o.value.as_str()))
            .collect()
    }

    /// Get selected labels
    pub fn get_selected_labels(&self) -> Vec<&str> {
        self.selected
            .iter()
            .filter_map(|&i| self.options.get(i).map(|o| o.label.as_str()))
            .collect()
    }

    /// Get number of selected items
    pub fn selection_count(&self) -> usize {
        self.selected.len()
    }

    /// Check if an option is selected
    pub fn is_selected(&self, index: usize) -> bool {
        self.selected.contains(&index)
    }

    /// Check if can select more items
    pub fn can_select_more(&self) -> bool {
        match self.max_selections {
            Some(max) => self.selected.len() < max,
            None => true,
        }
    }

    /// Get number of options
    pub fn len(&self) -> usize {
        self.options.len()
    }

    /// Check if there are no options
    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }

    /// Open the dropdown
    pub fn open(&mut self) {
        self.open = true;
        self.tag_cursor = None;
        self.reset_filter();
    }

    /// Close the dropdown
    pub fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.reset_filter();
    }

    /// Toggle dropdown
    pub fn toggle(&mut self) {
        if self.open {
            self.close();
        } else {
            self.open();
        }
    }

    /// Select an option by index
    pub fn select_option(&mut self, index: usize) {
        if index >= self.options.len() {
            return;
        }
        if self.options[index].disabled {
            return;
        }
        if !self.selected.contains(&index) && self.can_select_more() {
            self.selected.push(index);
        }
    }

    /// Deselect an option by index
    pub fn deselect_option(&mut self, index: usize) {
        self.selected.retain(|&i| i != index);
    }

    /// Toggle selection of an option
    pub fn toggle_option(&mut self, index: usize) {
        if self.is_selected(index) {
            self.deselect_option(index);
        } else {
            self.select_option(index);
        }
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) {
        self.selected.clear();
    }

    /// Select all options
    pub fn select_all(&mut self) {
        self.selected = (0..self.options.len())
            .filter(|&i| !self.options[i].disabled)
            .collect();
        if let Some(max) = self.max_selections {
            self.selected.truncate(max);
        }
    }

    /// Remove the last selected tag
    pub fn remove_last_tag(&mut self) {
        self.selected.pop();
    }

    /// Remove tag at cursor position
    pub fn remove_tag_at_cursor(&mut self) {
        if let Some(cursor) = self.tag_cursor {
            if cursor < self.selected.len() {
                self.selected.remove(cursor);
                // Adjust cursor
                if self.selected.is_empty() {
                    self.tag_cursor = None;
                } else if cursor >= self.selected.len() {
                    self.tag_cursor = Some(self.selected.len() - 1);
                }
            }
        }
    }
}
