//! DataGrid column freeze functionality

use super::core::DataGrid;

impl DataGrid {
    /// Scroll columns left
    pub fn scroll_col_left(&mut self) {
        if self.scroll_col > 0 {
            self.scroll_col -= 1;
        }
    }

    /// Scroll columns right
    pub fn scroll_col_right(&mut self) {
        let scrollable = self
            .columns
            .len()
            .saturating_sub(self.frozen_left + self.frozen_right);
        if self.scroll_col < scrollable.saturating_sub(1) {
            self.scroll_col += 1;
        }
    }

    /// Get frozen left column count
    pub fn frozen_left(&self) -> usize {
        self.frozen_left
    }

    /// Get frozen right column count
    pub fn frozen_right(&self) -> usize {
        self.frozen_right
    }
}
