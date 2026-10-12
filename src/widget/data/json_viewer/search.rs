//! Search functionality for JsonViewer

use super::types::JsonNode;
use std::collections::HashSet;

/// Search trait for JsonViewer
pub trait Search {
    /// Set search query
    fn search(&mut self, query: &str);

    /// Clear search
    fn clear_search(&mut self);

    /// Get search match count
    fn match_count(&self) -> usize;

    /// Check if search is active
    fn is_searching(&self) -> bool;

    /// Go to next match
    fn next_match(&mut self);

    /// Go to previous match
    fn prev_match(&mut self);
}

/// Searchable state for JsonViewer
#[derive(Clone, Debug)]
pub struct SearchState {
    pub search_query: String,
    pub search_matches: Vec<String>,
    pub current_match: usize,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match: 0,
        }
    }

    pub fn search_recursive(&mut self, node: &JsonNode) {
        let matches = |text: &str| text.to_lowercase().contains(&self.search_query);
        if matches(&node.key) || node.value.as_deref().is_some_and(matches) {
            self.search_matches.push(node.path.clone());
        }

        for child in &node.children {
            self.search_recursive(child);
        }
    }

    /// Expand the ancestors of the current match in `collapsed` and select
    /// it among the rows that leaves visible
    pub fn go_to_match(
        &self,
        collapsed: &mut HashSet<String>,
        selected: &mut usize,
        get_visible_nodes: impl Fn(&HashSet<String>) -> Vec<JsonNode>,
    ) {
        if let Some(path) = self.search_matches.get(self.current_match) {
            // Expand all ancestors
            let parts: Vec<&str> = path.split('.').collect();
            let mut current_path = String::new();
            for (i, part) in parts.iter().enumerate() {
                if i > 0 {
                    current_path.push('.');
                }
                current_path.push_str(part);
                collapsed.remove(&current_path);
            }

            // Find and select the node
            let visible = get_visible_nodes(collapsed);
            for (idx, node) in visible.iter().enumerate() {
                if &node.path == path {
                    *selected = idx;
                    break;
                }
            }
        }
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}
