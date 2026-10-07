//! LogViewer search: query, match list and match navigation

use super::LogViewer;
use crate::widget::data::log_viewer::entry::SearchMatch;

impl LogViewer {
    /// Set search query
    pub fn search(&mut self, query: &str) {
        self.search_query = query.to_string();
        self.search_index = 0;
        self.update_search();
    }

    /// Clear search
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.search_matches.clear();
        self.search_index = 0;
    }

    /// Go to next search match
    pub fn next_match(&mut self) {
        if !self.search_matches.is_empty() {
            self.search_index = (self.search_index + 1) % self.search_matches.len();
            self.scroll_to_match(self.search_index);
        }
    }

    /// Go to previous search match
    pub fn prev_match(&mut self) {
        if !self.search_matches.is_empty() {
            self.search_index = if self.search_index == 0 {
                self.search_matches.len() - 1
            } else {
                self.search_index - 1
            };
            self.scroll_to_match(self.search_index);
        }
    }

    /// Scroll to specific search match
    fn scroll_to_match(&mut self, match_index: usize) {
        if let Some(m) = self.search_matches.get(match_index) {
            // Find position in filtered view
            let filtered: Vec<_> = self.filtered_entries().collect();
            for (view_idx, (entry_idx, _)) in filtered.iter().enumerate() {
                if *entry_idx == m.entry_index {
                    self.selected = view_idx;
                    self.ensure_visible(view_idx);
                    break;
                }
            }
        }
    }

    /// Update search matches
    pub(super) fn update_search(&mut self) {
        self.search_matches.clear();

        if self.search_query.is_empty() {
            return;
        }

        let query_lower = self.search_query.to_lowercase();

        for (idx, entry) in self.entries.iter().enumerate() {
            let msg_lower = entry.message.to_lowercase();

            let mut start = 0;
            while let Some(pos) = msg_lower[start..].find(&query_lower) {
                let actual_start = start + pos;
                self.search_matches.push(SearchMatch {
                    entry_index: idx,
                    start: actual_start,
                    end: actual_start + self.search_query.len(),
                });
                start = actual_start + 1;
            }
        }
    }
}
