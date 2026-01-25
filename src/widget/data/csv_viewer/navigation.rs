//! Navigation functionality for CSV Viewer widget

/// Navigation methods for CsvViewer
impl super::CsvViewer {
    /// Move selection down
    pub fn select_down(&mut self) {
        let max_row = self.row_count().saturating_sub(1);
        self.selected_row = (self.selected_row + 1).min(max_row);
        self.ensure_visible();
    }

    /// Move selection up
    pub fn select_up(&mut self) {
        self.selected_row = self.selected_row.saturating_sub(1);
        self.ensure_visible();
    }

    /// Move selection right
    pub fn select_right(&mut self) {
        let max_col = self.column_count().saturating_sub(1);
        self.selected_col = (self.selected_col + 1).min(max_col);
    }

    /// Move selection left
    pub fn select_left(&mut self) {
        self.selected_col = self.selected_col.saturating_sub(1);
    }

    /// Select first row
    pub fn select_first_row(&mut self) {
        self.selected_row = 0;
        self.ensure_visible();
    }

    /// Select last row
    pub fn select_last_row(&mut self) {
        self.selected_row = self.row_count().saturating_sub(1);
        self.ensure_visible();
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        let max_row = self.row_count().saturating_sub(1);
        self.selected_row = (self.selected_row + page_size).min(max_row);
        self.ensure_visible();
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.selected_row = self.selected_row.saturating_sub(page_size);
        self.ensure_visible();
    }

    /// Ensure selected cell is visible
    fn ensure_visible(&mut self) {
        // Vertical scrolling handled during render
    }
}
