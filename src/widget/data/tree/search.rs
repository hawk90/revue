//! Fuzzy search methods for Tree widget

use super::Tree;
use crate::utils::{fuzzy_match, FuzzyMatch};

impl Tree {
    // --- Fuzzy search methods ---

    /// Get current search query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Set search query and find matches
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.update_matches();
    }

    /// Clear search query
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.matches.clear();
        self.current_match = 0;
    }

    /// Check if searchable mode is enabled
    pub fn is_searchable(&self) -> bool {
        self.searchable
    }

    /// Get number of matches
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Get current match index (1-based for display)
    pub fn current_match_index(&self) -> usize {
        if self.matches.is_empty() {
            0
        } else {
            self.current_match + 1
        }
    }

    /// Update matches based on current query, and select the first one that
    /// can be selected
    pub fn update_matches(&mut self) {
        self.find_matches();
        self.current_match = 0;
        if let Some(at) = self.selectable_match_from(0, 1) {
            self.current_match = at;
            self.selection.set(self.matches[at]);
        }
    }

    /// The visible rows whose labels match the query
    fn find_matches(&mut self) {
        self.matches.clear();
        if self.query.is_empty() {
            return;
        }
        let query = &self.query;
        self.matches = self
            .rows()
            .iter()
            .enumerate()
            .filter(|(_, row)| fuzzy_match(query, &row.node.label).is_some())
            .map(|(index, _)| index)
            .collect();
    }

    /// After rows came or went: the matches again (#859). The selection
    /// stays; the current match is the selected row when it matches, else
    /// the last match above it, so `n` goes to the first match below it.
    pub(super) fn refresh_matches(&mut self) {
        self.find_matches();
        let selected = self.selection.index;
        let n = self.matches.len();
        self.current_match = match self.matches.iter().position(|&row| row >= selected) {
            Some(at) if self.matches[at] == selected => at,
            Some(at) => (at + n - 1) % n,
            None => n.saturating_sub(1),
        };
    }

    /// The first match from position `start`, stepping by `step` (1 or the
    /// count minus 1) and wrapping, whose node can be selected (#979)
    fn selectable_match_from(&self, start: usize, step: usize) -> Option<usize> {
        let n = self.matches.len();
        (0..n)
            .map(|k| (start + k * step) % n)
            .find(|&at| self.is_selectable_at(self.matches[at]))
    }

    /// Jump to the next match that can be selected
    pub fn next_match(&mut self) -> bool {
        if self.matches.is_empty() {
            return false;
        }
        let n = self.matches.len();
        match self.selectable_match_from((self.current_match + 1) % n, 1) {
            Some(at) => {
                self.current_match = at;
                self.selection.set(self.matches[at]);
                true
            }
            None => false,
        }
    }

    /// Jump to the previous match that can be selected
    pub fn prev_match(&mut self) -> bool {
        if self.matches.is_empty() {
            return false;
        }
        let n = self.matches.len();
        match self.selectable_match_from((self.current_match + n - 1) % n, n - 1) {
            Some(at) => {
                self.current_match = at;
                self.selection.set(self.matches[at]);
                true
            }
            None => false,
        }
    }

    /// Get fuzzy match for a label
    pub fn get_match(&self, label: &str) -> Option<FuzzyMatch> {
        if self.query.is_empty() {
            None
        } else {
            fuzzy_match(&self.query, label)
        }
    }

    /// Check if a visible index is a match
    pub fn is_match(&self, visible_index: usize) -> bool {
        self.matches.contains(&visible_index)
    }
}
