//! State getter methods for CSV Viewer widget

/// State getter methods for CsvViewer
impl super::CsvViewer {
    /// Get number of rows (excluding header if present)
    pub fn row_count(&self) -> usize {
        if self.has_header && !self.data.is_empty() {
            self.data.len() - 1
        } else {
            self.data.len()
        }
    }

    /// Get number of columns
    pub fn column_count(&self) -> usize {
        self.data.first().map(|r| r.len()).unwrap_or(0)
    }

    /// Get selected row index
    pub fn selected_row(&self) -> usize {
        self.selected_row
    }

    /// Get selected column index
    pub fn selected_col(&self) -> usize {
        self.selected_col
    }

    /// Get cell value at position
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&str> {
        let actual_row = if self.has_header { row + 1 } else { row };
        self.data
            .get(actual_row)
            .and_then(|r| r.get(col))
            .map(|s| s.as_str())
    }

    /// Get header value at column
    pub fn get_header(&self, col: usize) -> Option<&str> {
        if self.has_header {
            self.data
                .first()
                .and_then(|r| r.get(col))
                .map(|s| s.as_str())
        } else {
            None
        }
    }

    /// Get selected cell value
    pub fn selected_value(&self) -> Option<&str> {
        self.get_cell(self.selected_row, self.selected_col)
    }

    /// Check if search is active
    pub fn is_searching(&self) -> bool {
        !self.search_query.is_empty()
    }

    /// Get search match count
    pub fn match_count(&self) -> usize {
        self.search_matches.len()
    }
}
