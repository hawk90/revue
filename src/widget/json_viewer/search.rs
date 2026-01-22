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
    pub collapsed: HashSet<String>,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match: 0,
            collapsed: HashSet::new(),
        }
    }

    pub fn search_recursive(&mut self, node: &JsonNode) {
        // Check key
        if node.key.to_lowercase().contains(&self.search_query) {
            self.search_matches.push(node.path.clone());
        }
        // Check value
        else if let Some(value) = &node.value {
            if value.to_lowercase().contains(&self.search_query) {
                self.search_matches.push(node.path.clone());
            }
        }

        for child in &node.children {
            self.search_recursive(child);
        }
    }

    pub fn go_to_match(
        &mut self,
        selected: &mut usize,
        get_visible_nodes: impl Fn(&Self) -> Vec<JsonNode>,
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
                self.collapsed.remove(&current_path);
            }

            // Find and select the node
            let visible = get_visible_nodes(self);
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
