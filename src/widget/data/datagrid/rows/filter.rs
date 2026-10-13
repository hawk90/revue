//! DataGrid filtering

use crate::widget::data::datagrid::DataGrid;

impl DataGrid {
    /// Set filter (cancels any active edit)
    pub fn set_filter(&mut self, filter: impl Into<String>) {
        // Cancel any active edit before filtering (row visibility will change)
        if self.edit_state.active {
            self.cancel_edit();
        }
        self.filter = filter.into().to_lowercase();
        self.recompute_cache();
        // Reset selection to avoid pointing past filtered results
        self.selected_row = 0;
        self.scroll_row = 0;
    }

    /// Compute filtered row indices (internal)
    pub(in crate::widget::data::datagrid) fn compute_filtered_indices(&self) -> Vec<usize> {
        if self.filter.is_empty() {
            (0..self.rows.len()).collect()
        } else {
            self.rows
                .iter()
                .enumerate()
                .filter(|(_, row)| match self.filter_column {
                    Some(col_idx) => {
                        if let Some(col) = self.columns.get(col_idx).filter(|c| c.filterable) {
                            row.get(&col.key)
                                .map(|v| v.to_lowercase().contains(&self.filter))
                                .unwrap_or(false)
                        } else {
                            false
                        }
                    }
                    None => row.data.iter().any(|(k, v)| {
                        let filterable = self
                            .columns
                            .iter()
                            .find(|c| &c.key == k)
                            .is_none_or(|c| c.filterable);
                        filterable && v.to_lowercase().contains(&self.filter)
                    }),
                })
                .map(|(i, _)| i)
                .collect()
        }
    }
}
