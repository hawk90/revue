//! Search and filter pattern
//!
//! Provides reusable search/filter state for list-based interfaces.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::patterns::SearchState;
//!
//! let items = vec!["apple", "banana", "cherry"];
//! let mut search = SearchState::new();
//!
//! search.set_query("an");
//! let filtered: Vec<_> = search.filter(&items, |item| item.to_string());
//! assert_eq!(filtered, vec!["banana"]);
//! ```

use crate::constants::DEBOUNCE_SEARCH;
use crate::utils::fuzzy_match;
use std::time::{Duration, Instant};

/// Search mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SearchMode {
    /// Simple contains search
    #[default]
    Contains,
    /// Fuzzy matching
    Fuzzy,
    /// Prefix matching
    Prefix,
    /// Exact match
    Exact,
}

/// Search state for filtering lists
#[derive(Clone, Debug)]
pub struct SearchState {
    /// Current search query
    query: String,
    /// Search mode
    mode: SearchMode,
    /// Whether search is active (input visible)
    active: bool,
    /// Debounce timer for search updates
    last_update: Option<Instant>,
    /// Debounce duration
    debounce: Duration,
    /// Case sensitive search
    case_sensitive: bool,
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchState {
    /// Create a new search state
    pub fn new() -> Self {
        Self {
            query: String::new(),
            mode: SearchMode::Contains,
            active: false,
            last_update: None,
            debounce: DEBOUNCE_SEARCH,
            case_sensitive: false,
        }
    }

    /// Set search mode
    pub fn mode(mut self, mode: SearchMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set case sensitivity
    pub fn case_sensitive(mut self, sensitive: bool) -> Self {
        self.case_sensitive = sensitive;
        self
    }

    /// Set debounce duration
    pub fn debounce(mut self, duration: Duration) -> Self {
        self.debounce = duration;
        self
    }

    /// Get current query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Check if search has a query
    pub fn has_query(&self) -> bool {
        !self.query.is_empty()
    }

    /// Check if search is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Activate search
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Deactivate search
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Toggle search active state
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    /// Set search query
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.last_update = Some(Instant::now());
    }

    /// Clear search query
    pub fn clear(&mut self) {
        self.query.clear();
        self.last_update = None;
    }

    /// Push a character to the query
    pub fn push(&mut self, ch: char) {
        self.query.push(ch);
        self.last_update = Some(Instant::now());
    }

    /// Pop a character from the query
    pub fn pop(&mut self) -> Option<char> {
        let ch = self.query.pop();
        self.last_update = Some(Instant::now());
        ch
    }

    /// Check if debounce period has elapsed
    pub fn is_ready(&self) -> bool {
        match self.last_update {
            Some(t) => t.elapsed() >= self.debounce,
            None => true,
        }
    }

    /// Check if a string matches the current query
    pub fn matches(&self, text: &str) -> bool {
        if self.query.is_empty() {
            return true;
        }

        let query = if self.case_sensitive {
            self.query.clone()
        } else {
            self.query.to_lowercase()
        };

        let text = if self.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };

        match self.mode {
            SearchMode::Contains => text.contains(&query),
            SearchMode::Prefix => text.starts_with(&query),
            SearchMode::Exact => text == query,
            SearchMode::Fuzzy => fuzzy_match(&query, &text).is_some(),
        }
    }

    /// Filter a collection based on the search query
    pub fn filter<'a, T, F>(&self, items: &'a [T], to_string: F) -> Vec<&'a T>
    where
        F: Fn(&T) -> String,
    {
        if self.query.is_empty() {
            return items.iter().collect();
        }

        items
            .iter()
            .filter(|item| self.matches(&to_string(item)))
            .collect()
    }

    /// Filter and return indices of matching items
    pub fn filter_indices<T, F>(&self, items: &[T], to_string: F) -> Vec<usize>
    where
        F: Fn(&T) -> String,
    {
        if self.query.is_empty() {
            return (0..items.len()).collect();
        }

        items
            .iter()
            .enumerate()
            .filter(|(_, item)| self.matches(&to_string(item)))
            .map(|(i, _)| i)
            .collect()
    }

    /// Get match score for ranking (higher is better)
    pub fn score(&self, text: &str) -> Option<i32> {
        if self.query.is_empty() {
            return Some(0);
        }

        let query = if self.case_sensitive {
            self.query.clone()
        } else {
            self.query.to_lowercase()
        };

        let text = if self.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };

        match self.mode {
            SearchMode::Fuzzy => fuzzy_match(&query, &text).map(|m| m.score),
            SearchMode::Exact if text == query => Some(100),
            SearchMode::Prefix if text.starts_with(&query) => {
                Some(50 + (query.len() as i32 * 100 / text.len().max(1) as i32))
            }
            SearchMode::Contains if text.contains(&query) => {
                let pos = text.find(&query).unwrap_or(0);
                Some(25 - pos as i32)
            }
            _ => None,
        }
    }

    /// Filter and sort by score (best matches first)
    pub fn filter_ranked<'a, T, F>(&self, items: &'a [T], to_string: F) -> Vec<&'a T>
    where
        F: Fn(&T) -> String,
    {
        if self.query.is_empty() {
            return items.iter().collect();
        }

        let mut scored: Vec<_> = items
            .iter()
            .filter_map(|item| {
                let text = to_string(item);
                self.score(&text).map(|score| (item, score))
            })
            .collect();

        scored.sort_by(|a, b| b.1.cmp(&a.1));
        scored.into_iter().map(|(item, _)| item).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_contains() {
        let mut search = SearchState::new();
        search.set_query("an");

        assert!(search.matches("banana"));
        assert!(search.matches("mango"));
        assert!(!search.matches("apple"));
    }

    #[test]
    fn test_search_prefix() {
        let mut search = SearchState::new().mode(SearchMode::Prefix);
        search.set_query("app");

        assert!(search.matches("apple"));
        assert!(search.matches("application"));
        assert!(!search.matches("pineapple"));
    }

    #[test]
    fn test_search_exact() {
        let mut search = SearchState::new().mode(SearchMode::Exact);
        search.set_query("apple");

        assert!(search.matches("apple"));
        assert!(!search.matches("apples"));
        assert!(search.matches("Apple")); // case insensitive by default, so matches

        // With case sensitive
        let mut search_cs = SearchState::new()
            .mode(SearchMode::Exact)
            .case_sensitive(true);
        search_cs.set_query("apple");
        assert!(!search_cs.matches("Apple")); // now it doesn't match
    }

    #[test]
    fn test_search_case_sensitive() {
        let mut search = SearchState::new().case_sensitive(true);
        search.set_query("Apple");

        assert!(search.matches("Apple"));
        assert!(!search.matches("apple"));
    }

    #[test]
    fn test_filter() {
        let items = vec!["apple", "banana", "cherry", "date"];
        let mut search = SearchState::new();
        search.set_query("a");

        let filtered: Vec<_> = search.filter(&items, |s| s.to_string());
        assert_eq!(filtered.len(), 3); // apple, banana, date
    }

    #[test]
    fn test_filter_indices() {
        let items = vec!["apple", "banana", "cherry"];
        let mut search = SearchState::new();
        search.set_query("a");

        let indices = search.filter_indices(&items, |s| s.to_string());
        assert!(indices.contains(&0)); // apple
        assert!(indices.contains(&1)); // banana
        assert!(!indices.contains(&2)); // cherry has no 'a'
    }

    #[test]
    fn test_empty_query() {
        let items = vec!["a", "b", "c"];
        let search = SearchState::new();

        let filtered: Vec<_> = search.filter(&items, |s| s.to_string());
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_push_pop() {
        let mut search = SearchState::new();

        search.push('a');
        search.push('p');
        assert_eq!(search.query(), "ap");

        search.pop();
        assert_eq!(search.query(), "a");
    }

    #[test]
    fn test_active_toggle() {
        let mut search = SearchState::new();
        assert!(!search.is_active());

        search.activate();
        assert!(search.is_active());

        search.toggle();
        assert!(!search.is_active());
    }

    #[test]
    fn test_fuzzy_mode() {
        let mut search = SearchState::new().mode(SearchMode::Fuzzy);
        search.set_query("apl");

        // Fuzzy should match "apple" with query "apl"
        assert!(search.matches("apple"));
    }
}
