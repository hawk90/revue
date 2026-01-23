//! DataGrid row/column selection and navigation

use super::core::DataGrid;

impl DataGrid {
    /// Select next row (with auto-scroll)
    pub fn select_next(&mut self) {
        let count = self.filtered_count();
        if self.selected_row < count.saturating_sub(1) {
            self.selected_row += 1;
            self.ensure_visible();
        }
    }

    /// Select previous row (with auto-scroll)
    pub fn select_prev(&mut self) {
        if self.selected_row > 0 {
            self.selected_row -= 1;
            self.ensure_visible();
        }
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        let count = self.filtered_count();
        self.selected_row = (self.selected_row + page_size).min(count.saturating_sub(1));
        self.ensure_visible();
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.selected_row = self.selected_row.saturating_sub(page_size);
        self.ensure_visible();
    }

    /// Go to first row
    pub fn select_first(&mut self) {
        self.selected_row = 0;
        self.ensure_visible();
    }

    /// Go to last row
    pub fn select_last(&mut self) {
        let count = self.filtered_count();
        self.selected_row = count.saturating_sub(1);
        self.ensure_visible();
    }

    /// Ensure selected row is visible (auto-scroll)
    pub fn ensure_visible(&mut self) {
        // This will be called with viewport_height from render
        // For now, use a reasonable default
        self.ensure_visible_with_height(20);
    }

    /// Ensure selected row is visible with specific viewport height
    pub fn ensure_visible_with_height(&mut self, viewport_height: usize) {
        if self.selected_row < self.scroll_row {
            // Scroll up to show selected row
            self.scroll_row = self.selected_row;
        } else if self.selected_row >= self.scroll_row + viewport_height {
            // Scroll down to show selected row
            self.scroll_row = self.selected_row.saturating_sub(viewport_height - 1);
        }
    }

    /// Set viewport height (called during render)
    pub fn set_viewport_height(&mut self, height: usize) {
        self.ensure_visible_with_height(height);
    }

    /// Get scroll position info (current, total, viewport)
    pub fn scroll_info(&self) -> (usize, usize, usize) {
        let total = self.filtered_count();
        (self.scroll_row, total, 20) // Default viewport, will be updated in render
    }

    /// Get total row count
    pub fn row_count(&self) -> usize {
        self.filtered_count()
    }

    /// Get visible row count
    pub fn visible_row_count(&self) -> usize {
        self.filtered_count()
    }

    /// Select next column
    pub fn select_next_col(&mut self) {
        let visible_cols: Vec<_> = self
            .columns
            .iter()
            .enumerate()
            .filter(|(_, c)| c.visible)
            .collect();
        if let Some(pos) = visible_cols
            .iter()
            .position(|(i, _)| *i == self.selected_col)
        {
            if pos < visible_cols.len() - 1 {
                self.selected_col = visible_cols[pos + 1].0;
            }
        }
    }

    /// Select previous column
    pub fn select_prev_col(&mut self) {
        let visible_cols: Vec<_> = self
            .columns
            .iter()
            .enumerate()
            .filter(|(_, c)| c.visible)
            .collect();
        if let Some(pos) = visible_cols
            .iter()
            .position(|(i, _)| *i == self.selected_col)
        {
            if pos > 0 {
                self.selected_col = visible_cols[pos - 1].0;
            }
        }
    }

    /// Toggle row selection
    pub fn toggle_selection(&mut self) {
        if self.options.multi_select && self.selected_row < self.rows.len() {
            self.rows[self.selected_row].selected = !self.rows[self.selected_row].selected;
        }
    }

    /// Get selected rows
    pub fn selected_rows(&self) -> Vec<&super::types::GridRow> {
        self.rows.iter().filter(|r| r.selected).collect()
    }
}
