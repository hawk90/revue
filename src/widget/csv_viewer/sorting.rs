//! Sorting functionality for CSV Viewer widget

use crate::utils::natural_cmp;

use super::types::SortOrder;

/// Sorting methods for CsvViewer
impl super::CsvViewer {
    /// Sort by column
    pub fn sort_by(&mut self, column: usize) {
        if self.sort_column == Some(column) {
            // Toggle sort order
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
        let start = if self.has_header { 1 } else { 0 };
        self.sorted_indices = (start..self.data.len()).collect();
    }

    /// Apply current sort
    fn apply_sort(&mut self) {
        let start = if self.has_header { 1 } else { 0 };
        self.sorted_indices = (start..self.data.len()).collect();

        if let Some(col) = self.sort_column {
            match self.sort_order {
                SortOrder::None => {}
                SortOrder::Ascending => {
                    self.sorted_indices.sort_by(|&a, &b| {
                        let val_a = self
                            .data
                            .get(a)
                            .and_then(|r| r.get(col))
                            .map(|s| s.as_str())
                            .unwrap_or("");
                        let val_b = self
                            .data
                            .get(b)
                            .and_then(|r| r.get(col))
                            .map(|s| s.as_str())
                            .unwrap_or("");
                        natural_cmp(val_a, val_b)
                    });
                }
                SortOrder::Descending => {
                    self.sorted_indices.sort_by(|&a, &b| {
                        let val_a = self
                            .data
                            .get(a)
                            .and_then(|r| r.get(col))
                            .map(|s| s.as_str())
                            .unwrap_or("");
                        let val_b = self
                            .data
                            .get(b)
                            .and_then(|r| r.get(col))
                            .map(|s| s.as_str())
                            .unwrap_or("");
                        natural_cmp(val_b, val_a)
                    });
                }
            }
        }
    }
}
