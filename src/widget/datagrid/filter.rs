//! DataGrid sorting and filtering

use super::core::DataGrid;
use super::types::SortDirection;
use crate::utils::natural_cmp;
use std::cmp::Ordering;

impl DataGrid {
    /// Sort by column (cancels any active edit)
    pub fn sort(&mut self, column: usize) {
        if column >= self.columns.len() || !self.columns[column].sortable {
            return;
        }

        // Cancel any active edit before sorting (row indices will change)
        if self.edit_state.active {
            self.cancel_edit();
        }

        if self.sort_column == Some(column) {
            self.sort_direction = self.sort_direction.toggle();
        } else {
            self.sort_column = Some(column);
            self.sort_direction = SortDirection::Ascending;
        }

        let key = &self.columns[column].key;
        let col_type = self.columns[column].col_type;
        let ascending = self.sort_direction == SortDirection::Ascending;
        let use_natural = self.options.use_natural_sort;

        self.rows.sort_by(|a, b| {
            let va = a.get(key).unwrap_or("");
            let vb = b.get(key).unwrap_or("");

            let cmp = match col_type {
                super::types::ColumnType::Number => {
                    let na: f64 = va.parse().unwrap_or(0.0);
                    let nb: f64 = vb.parse().unwrap_or(0.0);
                    na.partial_cmp(&nb).unwrap_or(Ordering::Equal)
                }
                super::types::ColumnType::Text | super::types::ColumnType::Custom => {
                    if use_natural {
                        natural_cmp(va, vb)
                    } else {
                        va.cmp(vb)
                    }
                }
                _ => va.cmp(vb),
            };

            if ascending {
                cmp
            } else {
                cmp.reverse()
            }
        });
        self.recompute_cache();
    }

    /// Set filter (cancels any active edit)
    pub fn set_filter(&mut self, filter: impl Into<String>) {
        // Cancel any active edit before filtering (row visibility will change)
        if self.edit_state.active {
            self.cancel_edit();
        }
        self.filter = filter.into().to_lowercase();
        self.recompute_cache();
    }
}
