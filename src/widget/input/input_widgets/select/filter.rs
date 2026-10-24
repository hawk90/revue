//! Fuzzy search and filtering for Select

use crate::event::KeyEvent;
use crate::utils::{fuzzy_match, EditOutcome, FuzzyMatch};

use super::Select;

impl Select {
    /// Reset filter to show all options
    pub(crate) fn reset_filter(&mut self) {
        self.filtered = (0..self.options.len()).collect();
        self.filtered_selection.set_len(self.filtered.len());
        self.filtered_selection.first();
    }

    /// Apply an editing key to the search query, filtering again when the
    /// text changed; whether the key was an editing key
    pub(crate) fn edit_query(&mut self, event: &KeyEvent) -> bool {
        match self.query.edit_key(event, 1) {
            EditOutcome::Edited => {
                self.update_filter();
                true
            }
            EditOutcome::Moved => true,
            EditOutcome::Unhandled => false,
        }
    }

    /// Update filter based on current query
    pub(crate) fn update_filter(&mut self) {
        if self.query.is_empty() {
            self.reset_filter();
            return;
        }

        // Collect matches with scores
        let mut matches: Vec<(usize, i32)> = self
            .options
            .iter()
            .enumerate()
            .filter_map(|(i, opt)| fuzzy_match(self.query(), opt).map(|m| (i, m.score)))
            .collect();

        // Sort by score descending
        matches.sort_by_key(|b| std::cmp::Reverse(b.1));

        self.filtered = matches.into_iter().map(|(i, _)| i).collect();
        self.filtered_selection.set_len(self.filtered.len());
        self.filtered_selection.first();

        // Update selected to first filtered item if available
        if let Some(&first) = self.filtered.first() {
            self.selection.set(first);
        }
    }

    /// Get fuzzy match for an option
    pub fn get_match(&self, option: &str) -> Option<FuzzyMatch> {
        if self.query.is_empty() {
            None
        } else {
            fuzzy_match(self.query(), option)
        }
    }

    /// Select next in filtered results
    pub(crate) fn select_next_filtered(&mut self) {
        if !self.filtered.is_empty() {
            self.filtered_selection.next();
            self.selection
                .set(self.filtered[self.filtered_selection.index]);
        }
    }

    /// Select previous in filtered results
    pub(crate) fn select_prev_filtered(&mut self) {
        if !self.filtered.is_empty() {
            self.filtered_selection.prev();
            self.selection
                .set(self.filtered[self.filtered_selection.index]);
        }
    }
}
