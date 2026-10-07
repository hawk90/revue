//! Changing the selection and moving the highlight

use super::SelectionList;

impl SelectionList {
    /// Toggle selection of an item
    pub fn toggle(&mut self, index: usize) {
        if index >= self.items.len() {
            return;
        }

        if self.items[index].disabled {
            return;
        }

        if self.is_selected(index) {
            // Deselect if above minimum
            if self.selected.len() > self.min_selections {
                self.selected.retain(|&i| i != index);
            }
        } else {
            // Select if below maximum
            if self.max_selections == 0 || self.selected.len() < self.max_selections {
                self.selected.push(index);
                self.selected.sort();
            }
        }
    }

    /// Toggle highlighted item
    pub fn toggle_highlighted(&mut self) {
        self.toggle(self.highlighted);
    }

    /// Select item
    pub fn select(&mut self, index: usize) {
        if index >= self.items.len() || self.items[index].disabled {
            return;
        }

        if !self.is_selected(index)
            && (self.max_selections == 0 || self.selected.len() < self.max_selections)
        {
            self.selected.push(index);
            self.selected.sort();
        }
    }

    /// Deselect item
    pub fn deselect(&mut self, index: usize) {
        if self.selected.len() > self.min_selections {
            self.selected.retain(|&i| i != index);
        }
    }

    /// Select all
    pub fn select_all(&mut self) {
        self.selected = (0..self.items.len())
            .filter(|&i| !self.items[i].disabled)
            .collect();

        if self.max_selections > 0 {
            self.selected.truncate(self.max_selections);
        }
    }

    /// Deselect all
    pub fn deselect_all(&mut self) {
        if self.min_selections == 0 {
            self.selected.clear();
        } else {
            self.selected.truncate(self.min_selections);
        }
    }

    /// Move highlight up
    pub fn highlight_previous(&mut self) {
        if self.highlighted > 0 {
            self.highlighted -= 1;
            self.ensure_visible();
        }
    }

    /// Move highlight down
    pub fn highlight_next(&mut self) {
        if self.highlighted < self.items.len().saturating_sub(1) {
            self.highlighted += 1;
            self.ensure_visible();
        }
    }

    /// Move highlight to start
    pub fn highlight_first(&mut self) {
        self.highlighted = 0;
        self.scroll_offset = 0;
    }

    /// Move highlight to end
    pub fn highlight_last(&mut self) {
        self.highlighted = self.items.len().saturating_sub(1);
        self.ensure_visible();
    }

    /// Ensure highlighted item is visible
    fn ensure_visible(&mut self) {
        let max_visible = if self.max_visible > 0 {
            self.max_visible
        } else {
            self.items.len()
        };

        if self.highlighted < self.scroll_offset {
            self.scroll_offset = self.highlighted;
        } else if self.highlighted >= self.scroll_offset + max_visible {
            self.scroll_offset = self.highlighted - max_visible + 1;
        }
    }
}
