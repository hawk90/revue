//! DataGrid column freeze and horizontal scroll

use crate::widget::data::datagrid::types::GridColumn;
use crate::widget::data::datagrid::DataGrid;

impl DataGrid {
    /// Freeze N columns on the left (they stay visible during horizontal scroll)
    pub fn freeze_columns_left(mut self, count: usize) -> Self {
        self.frozen_left = count;
        self
    }

    /// Freeze N columns on the right (they stay visible during horizontal scroll)
    pub fn freeze_columns_right(mut self, count: usize) -> Self {
        self.frozen_right = count;
        self
    }

    /// Scroll columns left
    pub fn scroll_col_left(&mut self) {
        if self.scroll_col > 0 {
            self.scroll_col -= 1;
        }
    }

    /// Scroll columns right
    pub fn scroll_col_right(&mut self) {
        // Count only what scrolls, as `compute_column_slots` lays it out:
        // visible columns that are not frozen on either side.
        let visible_cols = self.visible_columns_in_order();
        let n = visible_cols.len();
        let frozen_left = self.frozen_left_for(&visible_cols).min(n);
        let frozen_right = self.frozen_right.min(n - frozen_left);
        let scrollable = n - frozen_left - frozen_right;
        if self.scroll_col < scrollable.saturating_sub(1) {
            self.scroll_col += 1;
        }
    }

    /// Left-frozen column count as laid out for the display order
    /// `visible_cols`: `frozen_left`, widened so that every visible column
    /// marked `GridColumn::frozen` (and every column before it in display
    /// order) is pinned too.
    pub(crate) fn frozen_left_for(&self, visible_cols: &[(usize, &GridColumn)]) -> usize {
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
