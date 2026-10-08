//! DataGrid column freeze functionality

use super::core::DataGrid;
use super::types::GridColumn;

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
            .saturating_sub(self.effective_frozen_left() + self.frozen_right);
        if self.scroll_col < scrollable.saturating_sub(1) {
            self.scroll_col += 1;
        }
    }

    /// Left-frozen column count as laid out: `frozen_left`, widened so that
    /// every visible column marked `GridColumn::frozen` (and every column
    /// before it in display order) is pinned too.
    pub(super) fn effective_frozen_left(&self) -> usize {
        self.frozen_left_for(&self.visible_columns_in_order())
    }

    /// `effective_frozen_left` for an already computed display order.
    pub(super) fn frozen_left_for(&self, visible_cols: &[(usize, &GridColumn)]) -> usize {
        let by_column = visible_cols
            .iter()
            .rposition(|(_, c)| c.frozen)
            .map_or(0, |i| i + 1);
        self.frozen_left.max(by_column)
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
