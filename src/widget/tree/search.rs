//! Fuzzy search methods for Tree widget

use super::types::TreeNode;
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

    /// Update matches based on current query
    pub fn update_matches(&mut self) {
        self.matches.clear();
        self.current_match = 0;

        if self.query.is_empty() {
            return;
        }

        // Find all visible nodes that match
        fn find_matches(
            nodes: &[TreeNode],
            query: &str,
            visible_index: &mut usize,
            matches: &mut Vec<usize>,
        ) {
            for node in nodes {
                if fuzzy_match(query, &node.label).is_some() {
                    matches.push(*visible_index);
                }
                *visible_index += 1;
                if node.expanded && !node.children.is_empty() {
                    find_matches(&node.children, query, visible_index, matches);
                }
            }
        }

        let mut visible_index = 0;
        find_matches(
            &self.root,
            &self.query,
            &mut visible_index,
            &mut self.matches,
        );

        // Jump to first match
        if let Some(&first) = self.matches.first() {
            self.selection.set(first);
        }
    }

    /// Jump to next match
    pub fn next_match(&mut self) -> bool {
        if self.matches.is_empty() {
            return false;
        }
        self.current_match = (self.current_match + 1) % self.matches.len();
        self.selection.set(self.matches[self.current_match]);
        true
    }

    /// Jump to previous match
    pub fn prev_match(&mut self) -> bool {
        if self.matches.is_empty() {
            return false;
        }
        self.current_match = self
            .current_match
            .checked_sub(1)
            .unwrap_or(self.matches.len() - 1);
        self.selection.set(self.matches[self.current_match]);
        true
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
