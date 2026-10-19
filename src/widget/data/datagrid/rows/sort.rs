//! DataGrid sorting

use crate::utils::natural_cmp;
use crate::widget::data::datagrid::types::{ColumnType, SortDirection};
use crate::widget::data::datagrid::DataGrid;
use std::cmp::Ordering;

impl DataGrid {
    /// Sort by a single column (cancels any active edit and clears multi-sort)
    pub fn sort(&mut self, column: usize) {
        if !self.begin_sort(column) {
            return;
        }

        // Clear multi-column sort when using single sort
        self.sort_columns.clear();

        if self.sort_column == Some(column) {
            self.sort_direction = self.sort_direction.toggle();
        } else {
            self.sort_column = Some(column);
            self.sort_direction = SortDirection::Ascending;
        }

        self.sort_rows_by(&[(column, self.sort_direction)]);
    }

    /// Add or toggle a column in the multi-column sort stack
    ///
    /// If the column is already in the sort stack, its direction is toggled.
    /// If it is not in the stack, it is appended with ascending direction.
    /// Clears the single-column sort state (`sort_column`).
    pub fn add_sort(&mut self, column: usize) {
        if !self.begin_sort(column) {
            return;
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

        let keys = self.sort_columns.clone();
        self.sort_rows_by(&keys);
    }

    /// Whether `column` can be sorted; cancels an active edit first, since
    /// sorting moves the rows under it
    fn begin_sort(&mut self, column: usize) -> bool {
        if column >= self.columns.len() || !self.columns[column].sortable {
            return false;
        }
        if self.edit_state.active {
            self.cancel_edit();
        }
        true
    }

    /// Stable-sort the rows by `keys` in priority order: a later key only
    /// breaks ties of the earlier ones. Single and multi-column sorting both
    /// go through here.
    fn sort_rows_by(&mut self, keys: &[(usize, SortDirection)]) {
        // Build sort spec: (key, col_type, ascending)
        let sort_spec: Vec<_> = keys
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
                    ColumnType::Number => {
                        let na: f64 = va.parse().unwrap_or(0.0);
                        let nb: f64 = vb.parse().unwrap_or(0.0);
                        na.total_cmp(&nb)
                    }
                    ColumnType::Text | ColumnType::Custom => {
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
}
