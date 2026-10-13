//! DataGrid row view: which rows are shown, and in what order

mod aggregate;
mod filter;
mod sort;
pub(crate) mod tree;

use crate::widget::data::datagrid::types::GridRow;
use crate::widget::data::datagrid::DataGrid;

impl DataGrid {
    /// Recompute the filtered rows cache (called on mutation)
    pub fn recompute_cache(&mut self) {
        self.filtered_cache = self.compute_filtered_indices();
        // Fewer rows than before (rows removed, an edit that no longer
        // matches the filter): keep the selection and scroll on a row
        let last = self.filtered_cache.len().saturating_sub(1);
        self.selected_row = self.selected_row.min(last);
        self.scroll_row = self.scroll_row.min(last);
    }

    /// Get cached filtered row indices (zero-cost, no allocation)
    #[inline]
    pub fn filtered_indices(&self) -> &[usize] {
        &self.filtered_cache
    }

    /// Get filtered rows count (uses cache)
    pub fn filtered_count(&self) -> usize {
        self.filtered_indices().len()
    }

    /// Get filtered rows (uses cached indices)
    /// Note: For large datasets, prefer using filtered_indices() with index-based access
    pub fn filtered_rows(&self) -> Vec<&GridRow> {
        self.filtered_indices()
            .iter()
            .filter_map(|&i| self.rows.get(i))
            .collect()
    }
}
