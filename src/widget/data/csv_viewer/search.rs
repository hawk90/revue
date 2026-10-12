//! Search: matching cells, visited in the order rows are shown

use super::core::CsvViewer;

impl CsvViewer {
    /// Set search query
    ///
    /// Matches are kept in the order the rows are shown, so
    /// [`next_match`](Self::next_match) walks down the screen.
    pub fn search(&mut self, query: &str) {
        self.search_query = query.to_lowercase();
        self.search_matches.clear();
        self.current_match = 0;

        if self.search_query.is_empty() {
            return;
        }

        let start = self.first_data_row();
        // Only the shown columns: a ragged row's extra cells are never drawn
        let columns = self.column_count();
        for &data_idx in &self.sorted_indices {
            let Some(row) = self.data.get(data_idx) else {
                continue;
            };
            for (col_idx, cell) in row.iter().enumerate().take(columns) {
                if cell.to_lowercase().contains(&self.search_query) {
                    self.search_matches.push((data_idx - start, col_idx));
                }
            }
        }
    }

    /// Find the current query again, after the rows or their order changed
    pub(super) fn rerun_search(&mut self) {
        let query = std::mem::take(&mut self.search_query);
        self.search(&query);
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
            self.select_current_match();
        }
    }

    /// Go to previous search match
    pub fn prev_match(&mut self) {
        if !self.search_matches.is_empty() {
            self.current_match = self
                .current_match
                .checked_sub(1)
                .unwrap_or(self.search_matches.len() - 1);
            self.select_current_match();
        }
    }

    fn select_current_match(&mut self) {
        let (row, col) = self.search_matches[self.current_match];
        if let Some(position) = self.position_of(row) {
            self.selected_row = position;
        }
        self.selected_col = col;
    }
}
