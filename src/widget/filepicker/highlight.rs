//! The highlighted entry and keeping it in view

use super::types::PickerEntry;
use super::FilePicker;

impl FilePicker {
    /// Move highlight up
    pub fn highlight_previous(&mut self) {
        if self.highlighted > 0 {
            self.highlighted -= 1;
            self.ensure_visible();
        }
    }

    /// Move highlight down
    pub fn highlight_next(&mut self) {
        if self.highlighted < self.entries.len().saturating_sub(1) {
            self.highlighted += 1;
            self.ensure_visible();
        }
    }

    /// Ensure highlighted item is visible
    fn ensure_visible(&mut self) {
        if self.highlighted < self.scroll_offset {
            self.scroll_offset = self.highlighted;
        } else if self.highlighted >= self.scroll_offset + self.max_visible {
            self.scroll_offset = self.highlighted - self.max_visible + 1;
        }
    }

    /// Get highlighted entry
    pub fn highlighted_entry(&self) -> Option<&PickerEntry> {
        self.entries.get(self.highlighted)
    }
}
