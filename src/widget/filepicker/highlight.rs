//! The highlighted entry and keeping it in view

use super::types::PickerEntry;
use super::FilePicker;

impl FilePicker {
    /// Move highlight up
    pub fn highlight_previous(&mut self) {
        if self.highlighted > 0 {
            self.highlighted -= 1;
        }
    }

    /// Move highlight down
    pub fn highlight_next(&mut self) {
        if self.highlighted < self.entries.len().saturating_sub(1) {
            self.highlighted += 1;
        }
    }

    /// Get highlighted entry
    pub fn highlighted_entry(&self) -> Option<&PickerEntry> {
        self.entries.get(self.highlighted)
    }
}
