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
        self.step_match(true);
    }

    fn prev_match(&mut self) {
        self.step_match(false);
    }
}

impl JsonViewer {
    /// Move to the next or previous match, wrapping at either end
    fn step_match(&mut self, forward: bool) {
        let count = self.search_state.search_matches.len();
        if count == 0 {
            return;
        }
        let current = self.search_state.current_match;
        self.search_state.current_match = if forward {
            (current + 1) % count
        } else {
            current.checked_sub(1).unwrap_or(count - 1)
        };
        self.go_to_match();
    }

    fn go_to_match(&mut self) {
        let root = &self.root;
        let mut selected = self.selection.index().unwrap_or(0);
        self.search_state
            .go_to_match(&mut self.collapsed, &mut selected, |collapsed| {
                root.as_ref()
                    .map(|root| flatten_tree(root, collapsed))
                    .unwrap_or_default()
            });
        self.select_row(selected);
    }
}
