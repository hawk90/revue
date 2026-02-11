//! Search and filter functionality for the multi-select widget

use crate::utils::{fuzzy_match, FuzzyMatch};

use super::types::MultiSelect;

impl MultiSelect {
    /// Get current search query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Set search query
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.update_filter();
    }

    /// Clear search query
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.reset_filter();
    }

    /// Update filter based on query
    pub(super) fn update_filter(&mut self) {
        if self.query.is_empty() {
            self.reset_filter();
            return;
        }

        let mut matches: Vec<(usize, i32)> = self
            .options
            .iter()
            .enumerate()
            .filter_map(|(i, opt)| fuzzy_match(&self.query, &opt.label).map(|m| (i, m.score)))
            .collect();

        matches.sort_by(|a, b| b.1.cmp(&a.1));
        self.filtered = matches.into_iter().map(|(i, _)| i).collect();
        self.dropdown_cursor = 0;
    }

    /// Get fuzzy match for an option
    pub fn get_match(&self, text: &str) -> Option<FuzzyMatch> {
        if self.query.is_empty() {
            None
        } else {
            fuzzy_match(&self.query, text)
        }
    }
}
