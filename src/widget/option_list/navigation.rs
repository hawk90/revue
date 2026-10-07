//! Querying, selecting and moving the highlight between options

use super::{OptionEntry, OptionItem, OptionList};

impl OptionList {
    /// Get option count (excluding separators and groups)
    pub fn option_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| matches!(e, OptionEntry::Option(_)))
            .count()
    }

    /// Get highlighted option
    pub fn get_highlighted(&self) -> Option<&OptionItem> {
        let mut option_idx = 0;
        for entry in &self.entries {
            if let OptionEntry::Option(item) = entry {
                if option_idx == self.highlighted {
                    return Some(item);
                }
                option_idx += 1;
            }
        }
        None
    }

    /// Get selected option
    pub fn get_selected(&self) -> Option<&OptionItem> {
        let selected = self.selected?;
        let mut option_idx = 0;
        for entry in &self.entries {
            if let OptionEntry::Option(item) = entry {
                if option_idx == selected {
                    return Some(item);
                }
                option_idx += 1;
            }
        }
        None
    }

    /// Get selected value
    pub fn get_selected_value(&self) -> Option<&str> {
        self.get_selected()
            .and_then(|item| item.value.as_deref().or(Some(&item.text)))
    }

    /// Select highlighted option
    pub fn select_highlighted(&mut self) -> bool {
        if let Some(item) = self.get_highlighted() {
            if !item.disabled {
                self.selected = Some(self.highlighted);
                return true;
            }
        }
        false
    }

    /// Select by index
    pub fn select(&mut self, index: usize) {
        let mut option_idx = 0;
        for entry in &self.entries {
            if let OptionEntry::Option(item) = entry {
                if option_idx == index && !item.disabled {
                    self.selected = Some(index);
                    self.highlighted = index;
                    return;
                }
                option_idx += 1;
            }
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected = None;
    }

    /// Move highlight to previous option
    pub fn highlight_previous(&mut self) {
        if self.highlighted > 0 {
            self.highlighted -= 1;

            // Skip disabled items
            while self.highlighted > 0 {
                if let Some(item) = self.get_highlighted() {
                    if !item.disabled {
                        break;
                    }
                }
                self.highlighted -= 1;
            }

            self.ensure_visible();
        }
    }

    /// Move highlight to next option
    pub fn highlight_next(&mut self) {
        let max = self.option_count().saturating_sub(1);
        if self.highlighted < max {
            self.highlighted += 1;

            // Skip disabled items
            while self.highlighted < max {
                if let Some(item) = self.get_highlighted() {
                    if !item.disabled {
                        break;
                    }
                }
                self.highlighted += 1;
            }

            self.ensure_visible();
        }
    }

    /// Move to first option
    pub fn highlight_first(&mut self) {
        self.highlighted = 0;

        // Skip disabled items
        while self.highlighted < self.option_count().saturating_sub(1) {
            if let Some(item) = self.get_highlighted() {
                if !item.disabled {
                    break;
                }
            }
            self.highlighted += 1;
        }

        self.scroll_offset = 0;
    }

    /// Move to last option
    pub fn highlight_last(&mut self) {
        self.highlighted = self.option_count().saturating_sub(1);

        // Skip disabled items
        while self.highlighted > 0 {
            if let Some(item) = self.get_highlighted() {
                if !item.disabled {
                    break;
                }
            }
            self.highlighted -= 1;
        }

        self.ensure_visible();
    }

    /// Ensure highlighted item is visible
    fn ensure_visible(&mut self) {
        if self.highlighted < self.scroll_offset {
            self.scroll_offset = self.highlighted;
        } else if self.highlighted >= self.scroll_offset + self.max_visible {
            self.scroll_offset = self.highlighted - self.max_visible + 1;
        }
    }
}
