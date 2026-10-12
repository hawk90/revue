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

        if let Some(root) = &self.root {
            self.search_state.search_recursive(root);
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
        let root = &self.root;
        self.search_state
            .go_to_match(&mut self.collapsed, &mut self.selected, |collapsed| {
                root.as_ref()
                    .map(|root| flatten_tree(root, collapsed))
                    .unwrap_or_default()
            });
    }
}
