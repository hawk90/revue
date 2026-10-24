//! Search and filter functionality for the multi-select widget

use crate::event::KeyEvent;
use crate::utils::{fuzzy_match, EditOutcome, FuzzyMatch};

use super::MultiSelect;

impl MultiSelect {
    /// Get current search query
    pub fn query(&self) -> &str {
        self.query.line(0).unwrap_or("")
    }

    /// Set search query, the cursor at its end
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query.set_text(query.into());
        let end = self.query.end();
        self.query.set_cursor(end);
        self.update_filter();
    }

    /// Clear search query
    pub fn clear_query(&mut self) {
        self.query.set_text("");
        self.reset_filter();
    }

    /// Apply an editing key to the search query, filtering again when the
    /// text changed; whether the key was an editing key
    pub(super) fn edit_query(&mut self, event: &KeyEvent) -> bool {
        match self.query.edit_key(event, 1) {
            EditOutcome::Edited => {
                self.update_filter();
                true
            }
            EditOutcome::Moved => true,
            EditOutcome::Unhandled => false,
        }
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
            .filter_map(|(i, opt)| fuzzy_match(self.query(), &opt.label).map(|m| (i, m.score)))
            .collect();

        matches.sort_by_key(|b| std::cmp::Reverse(b.1));
        self.filtered = matches.into_iter().map(|(i, _)| i).collect();
        self.dropdown_cursor = 0;
    }

    /// Get fuzzy match for an option
    pub fn get_match(&self, text: &str) -> Option<FuzzyMatch> {
        if self.query.is_empty() {
            None
        } else {
            fuzzy_match(self.query(), text)
        }
    }
}
