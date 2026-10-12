//! Sorting: the order rows are shown in

use super::core::CsvViewer;
use super::types::SortOrder;
use crate::utils::natural_cmp;

impl CsvViewer {
    /// Sort by column
    ///
    /// Sorting the same column again cycles ascending, descending, unsorted.
    /// The selected record stays selected wherever it moves to.
    pub fn sort_by(&mut self, column: usize) {
        if self.sort_column == Some(column) {
            self.sort_order = match self.sort_order {
                SortOrder::None => SortOrder::Ascending,
                SortOrder::Ascending => SortOrder::Descending,
                SortOrder::Descending => SortOrder::None,
            };
        } else {
            self.sort_column = Some(column);
            self.sort_order = SortOrder::Ascending;
        }

        self.apply_sort();
    }

    /// Reset sorting
    pub fn reset_sort(&mut self) {
        self.sort_column = None;
        self.sort_order = SortOrder::None;
        self.apply_sort();
    }

    /// Rebuild the shown order from the data and the sort settings, keep the
    /// selected record selected, and find the search matches again in the
    /// new order.
    pub(super) fn apply_sort(&mut self) {
        let selected = self.sorted_indices.get(self.selected_row).copied();

        let start = self.first_data_row();
        self.sorted_indices = (start..self.data.len().max(start)).collect();
        if let Some(col) = self.sort_column {
            let key = |row: usize| {
                self.data
                    .get(row)
                    .and_then(|r| r.get(col))
                    .map(String::as_str)
                    .unwrap_or("")
            };
            let order = |a: &usize, b: &usize| natural_cmp(key(*a), key(*b));
            match self.sort_order {
                SortOrder::None => {}
                SortOrder::Ascending => self.sorted_indices.sort_by(order),
                SortOrder::Descending => self.sorted_indices.sort_by(|a, b| order(a, b).reverse()),
            }
        }

        self.selected_row = selected
            .and_then(|row| self.sorted_indices.iter().position(|&r| r == row))
            .unwrap_or_else(|| self.selected_row.min(self.row_count().saturating_sub(1)));
        self.rerun_search();
    }
}
