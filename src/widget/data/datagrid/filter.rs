//! DataGrid sorting and filtering

use super::core::DataGrid;
use super::types::SortDirection;
use crate::utils::natural_cmp;
use std::cmp::Ordering;

impl DataGrid {
    /// Sort by a single column (cancels any active edit and clears multi-sort)
    pub fn sort(&mut self, column: usize) {
        if column >= self.columns.len() || !self.columns[column].sortable {
            return;
        }

        // Cancel any active edit before sorting (row indices will change)
        if self.edit_state.active {
            self.cancel_edit();
        }

        // Clear multi-column sort when using single sort
        self.sort_columns.clear();

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

    /// Add or toggle a column in the multi-column sort stack
    ///
    /// If the column is already in the sort stack, its direction is toggled.
    /// If it is not in the stack, it is appended with ascending direction.
    /// Clears the single-column sort state (`sort_column`).
    pub fn add_sort(&mut self, column: usize) {
        if column >= self.columns.len() || !self.columns[column].sortable {
            return;
        }

        // Cancel any active edit before sorting
        if self.edit_state.active {
            self.cancel_edit();
        }

        // Clear single-column sort
        self.sort_column = None;

        // Check if column is already in multi-sort
        if let Some(pos) = self.sort_columns.iter().position(|(c, _)| *c == column) {
            // Toggle direction
            let (_, ref mut dir) = self.sort_columns[pos];
            *dir = dir.toggle();
        } else {
            // Add new column with ascending direction
            self.sort_columns.push((column, SortDirection::Ascending));
        }

        self.apply_multi_sort();
    }

    /// Remove a column from the multi-column sort stack
    pub fn remove_sort(&mut self, column: usize) {
        self.sort_columns.retain(|(c, _)| *c != column);

        if !self.sort_columns.is_empty() {
            self.apply_multi_sort();
        } else {
            self.recompute_cache();
        }
    }

    /// Clear all multi-column sorts
    pub fn clear_sort(&mut self) {
        self.sort_columns.clear();
        self.sort_column = None;
        self.recompute_cache();
    }

    /// Apply multi-column sort, sorting rows by all columns in priority order
    pub fn apply_multi_sort(&mut self) {
        if self.sort_columns.is_empty() {
            return;
        }

        // Build sort spec: (key, col_type, ascending, use_natural)
        let sort_spec: Vec<_> = self
            .sort_columns
            .iter()
            .filter_map(|&(col_idx, dir)| {
                self.columns.get(col_idx).map(|col| {
                    (
                        col.key.clone(),
                        col.col_type,
                        dir == SortDirection::Ascending,
                    )
                })
            })
            .collect();

        let use_natural = self.options.use_natural_sort;

        self.rows.sort_by(|a, b| {
            for (key, col_type, ascending) in &sort_spec {
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

                let ordered = if *ascending { cmp } else { cmp.reverse() };

                if ordered != Ordering::Equal {
                    return ordered;
                }
            }
            Ordering::Equal
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
