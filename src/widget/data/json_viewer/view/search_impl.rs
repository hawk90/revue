//! JsonViewer search: the `Search` impl and match navigation

use super::JsonViewer;
use crate::widget::data::json_viewer::helpers::flatten_tree;
use crate::widget::data::json_viewer::search::Search;

impl Search for JsonViewer {
    fn search(&mut self, query: &str) {
        self.search_state.search_query = query.to_lowercase();
        self.search_state.search_matches.clear();
        self.search_state.current_match = 0;

        if self.search_state.search_query.is_empty() {
            return;
        }

        if let Some(root) = self.root.clone() {
            self.sync_collapsed_to_search();
            self.search_state.search_recursive(&root);
            self.sync_collapsed_from_search();
        }
    }

    fn clear_search(&mut self) {
        self.search_state.search_query.clear();
        self.search_state.search_matches.clear();
        self.search_state.current_match = 0;
    }

    fn match_count(&self) -> usize {
        self.search_state.search_matches.len()
    }

    fn is_searching(&self) -> bool {
        !self.search_state.search_query.is_empty()
    }

    fn next_match(&mut self) {
        if !self.search_state.search_matches.is_empty() {
            self.search_state.current_match =
                (self.search_state.current_match + 1) % self.search_state.search_matches.len();
            self.go_to_match();
        }
    }

    fn prev_match(&mut self) {
        if !self.search_state.search_matches.is_empty() {
            self.search_state.current_match = self
                .search_state
                .current_match
                .checked_sub(1)
                .unwrap_or(self.search_state.search_matches.len() - 1);
            self.go_to_match();
        }
    }
}

impl JsonViewer {
    fn go_to_match(&mut self) {
        self.sync_collapsed_to_search();
        self.search_state.go_to_match(&mut self.selected, |state| {
            let collapsed = state.collapsed.clone();
            if let Some(root) = &self.root {
                flatten_tree(root, &collapsed)
            } else {
                Vec::new()
            }
        });
        self.sync_collapsed_from_search();
    }

    /// Sync collapsed set to search state
    fn sync_collapsed_to_search(&mut self) {
        self.search_state.collapsed = self.collapsed.clone();
    }

    /// Sync collapsed set from search state
    fn sync_collapsed_from_search(&mut self) {
        self.collapsed = self.search_state.collapsed.clone();
    }
}
