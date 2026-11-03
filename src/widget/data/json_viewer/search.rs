//! Search functionality for JsonViewer

use super::types::{JsonNode, JsonType};
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
    /// Where each match is in the tree: the child index at each level from
    /// the root, so its ancestors are found whatever its path holds
    pub match_places: Vec<Vec<usize>>,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match: 0,
            match_places: Vec::new(),
        }
    }

    /// Find the matches under `root`: nodes whose key or shown value
    /// (`null` for a null) holds the query
    pub fn search_recursive(&mut self, root: &JsonNode) {
        let mut place = Vec::new();
        self.search_node(root, &mut place);
    }

    fn search_node(&mut self, node: &JsonNode, place: &mut Vec<usize>) {
        let matches = |text: &str| text.to_lowercase().contains(&self.search_query);
        let value = match node.value_type {
            JsonType::Null => Some("null"),
            _ => node.value.as_deref(),
        };
        if matches(&node.key) || value.is_some_and(matches) {
            self.search_matches.push(node.path.clone());
            self.match_places.push(place.clone());
        }

        for (i, child) in node.children.iter().enumerate() {
            place.push(i);
            self.search_node(child, place);
            place.pop();
        }
    }

    /// Forget every match
    pub fn clear_matches(&mut self) {
        self.search_matches.clear();
        self.match_places.clear();
        self.current_match = 0;
    }

    /// Expand the ancestors of the current match under `root` in
    /// `collapsed` and select it among the rows that leaves visible
    pub fn go_to_match<'a>(
        &self,
        root: &'a JsonNode,
        collapsed: &mut HashSet<String>,
        selected: &mut usize,
        get_visible_nodes: impl Fn(&HashSet<String>) -> Vec<&'a JsonNode>,
    ) {
        let Some(place) = self.match_places.get(self.current_match) else {
            return;
        };

        // Expand every ancestor, walking down from the root
        let mut node = root;
        for &i in place {
            collapsed.remove(&node.path);
            match node.children.get(i) {
                Some(child) => node = child,
                None => return,
            }
        }

        // Select the node itself
        let visible = get_visible_nodes(collapsed);
        if let Some(idx) = visible.iter().position(|&n| std::ptr::eq(n, node)) {
            *selected = idx;
        }
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}
