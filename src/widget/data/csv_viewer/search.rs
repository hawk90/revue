//! Search functionality for CSV Viewer widget

/// Search methods for CsvViewer
impl super::CsvViewer {
    /// Set search query
    pub fn search(&mut self, query: &str) {
        self.search_query = query.to_lowercase();
        self.search_matches.clear();
        self.current_match = 0;

        if self.search_query.is_empty() {
            return;
        }

        let start = if self.has_header { 1 } else { 0 };
        for (row_idx, row) in self.data.iter().enumerate().skip(start) {
            for (col_idx, cell) in row.iter().enumerate() {
                if cell.to_lowercase().contains(&self.search_query) {
                    self.search_matches.push((row_idx - start, col_idx));
                }
            }
        }
    }

    /// Clear search
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.search_matches.clear();
        self.current_match = 0;
    }

    /// Go to next search match
    pub fn next_match(&mut self) {
        if !self.search_matches.is_empty() {
            self.current_match = (self.current_match + 1) % self.search_matches.len();
            let (row, col) = self.search_matches[self.current_match];
            self.selected_row = row;
            self.selected_col = col;
        }
    }

    /// Go to previous search match
    pub fn prev_match(&mut self) {
        if !self.search_matches.is_empty() {
            self.current_match = self
                .current_match
                .checked_sub(1)
                .unwrap_or(self.search_matches.len() - 1);
            let (row, col) = self.search_matches[self.current_match];
            self.selected_row = row;
            self.selected_col = col;
        }
    }
}
