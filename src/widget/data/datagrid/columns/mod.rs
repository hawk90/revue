//! DataGrid column model: which columns are shown, and in what order

mod freeze;
mod reorder;
mod resize;
pub(crate) mod slots;
mod width;

use crate::widget::data::datagrid::types::GridColumn;
use crate::widget::data::datagrid::DataGrid;

impl DataGrid {
    /// Visible columns in display order (respecting `column_order`), each
    /// paired with its index into `self.columns`.
    pub(crate) fn visible_columns_in_order(&self) -> Vec<(usize, &GridColumn)> {
        if self.column_order.is_empty() {
            self.columns
                .iter()
                .enumerate()
                .filter(|(_, c)| c.visible)
                .collect()
        } else {
            self.column_order
                .iter()
                .filter_map(|&orig_idx| {
                    self.columns
                        .get(orig_idx)
                        .filter(|c| c.visible)
                        .map(|c| (orig_idx, c))
                })
                .collect()
        }
    }

    /// Every column index, hidden ones included, in display order:
    /// `column_order` (out-of-range and repeated entries dropped), then the
    /// columns it does not list, in definition order.
    pub(crate) fn full_column_order(&self) -> Vec<usize> {
        let n = self.columns.len();
        let mut seen = vec![false; n];
        let mut order = Vec::with_capacity(n);
        for i in self.column_order.iter().copied().chain(0..n) {
            if i < n && !seen[i] {
                seen[i] = true;
                order.push(i);
            }
        }
        order
    }

    /// Display position (among visible columns) of column `col`
    fn display_position(&self, col: usize) -> Option<usize> {
        self.visible_columns_in_order()
            .iter()
            .position(|&(orig, _)| orig == col)
    }
}
