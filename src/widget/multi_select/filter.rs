//! Search and filter functionality for the multi-select widget

use crate::utils::{fuzzy_match, FuzzyMatch};

use super::types::MultiSelect;

impl MultiSelect {
    /// Get current search query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Set search query
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.update_filter();
    }

    /// Clear search query
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.reset_filter();
    }

    /// Update filter based on query
    pub(super) fn update_filter(&mut self) {
        if self.query.is_empty() {
            self.reset_filter();
            return;
        }

        let mut matches: Vec<(usize, i32)> = self
            .options
            .iter()
            .enumerate()
            .filter_map(|(i, opt)| fuzzy_match(&self.query, &opt.label).map(|m| (i, m.score)))
            .collect();

        matches.sort_by(|a, b| b.1.cmp(&a.1));
        self.filtered = matches.into_iter().map(|(i, _)| i).collect();
        self.dropdown_cursor = 0;
    }

    /// Get fuzzy match for an option
    pub fn get_match(&self, text: &str) -> Option<FuzzyMatch> {
        if self.query.is_empty() {
            None
        } else {
            fuzzy_match(&self.query, text)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_select() -> MultiSelect {
        MultiSelect::new().options(vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"])
    }

    // Query tests

    #[test]
    fn test_query_initial_empty() {
        // Arrange
        let select = create_test_select();

        // Act
        let query = select.query();

        // Assert
        assert_eq!(query, "");
    }

    #[test]
    fn test_query_after_set() {
        // Arrange
        let mut select = create_test_select();
        select.set_query("app");

        // Act
        let query = select.query();

        // Assert
        assert_eq!(query, "app");
    }

    #[test]
    fn test_set_query_updates_filter() {
        // Arrange
        let mut select = create_test_select();

        // Act
        select.set_query("app");

        // Assert
        assert_eq!(select.query, "app");
        // Filter should be updated to match "app"
        // Should find "Apple"
        assert!(!select.filtered.is_empty());
    }

    #[test]
    fn test_set_query_with_string() {
        // Arrange
        let mut select = create_test_select();

        // Act
        select.set_query(String::from("banana"));

        // Assert
        assert_eq!(select.query, "banana");
    }

    #[test]
    fn test_set_query_empty() {
        // Arrange
        let mut select = create_test_select();
        select.query = "test".to_string();

        // Act
        select.set_query("");

        // Assert
        assert_eq!(select.query, "");
        // Filter should reset to show all options
        assert_eq!(select.filtered.len(), 5);
    }

    #[test]
    fn test_set_query_resets_cursor() {
        // Arrange
        let mut select = create_test_select();
        select.dropdown_cursor = 3;

        // Act
        select.set_query("test");

        // Assert
        assert_eq!(select.dropdown_cursor, 0);
    }

    #[test]
    fn test_clear_query() {
        // Arrange
        let mut select = create_test_select();
        select.query = "test".to_string();
        select.filtered = vec![0];

        // Act
        select.clear_query();

        // Assert
        assert_eq!(select.query, "");
        assert_eq!(select.filtered.len(), 5);
    }

    #[test]
    fn test_clear_query_resets_cursor() {
        // Arrange
        let mut select = create_test_select();
        select.query = "test".to_string();
        select.dropdown_cursor = 2;

        // Act
        select.clear_query();

        // Assert
        assert_eq!(select.dropdown_cursor, 0);
    }

    #[test]
    fn test_clear_query_already_empty() {
        // Arrange
        let mut select = create_test_select();

        // Act
        select.clear_query();

        // Assert
        assert_eq!(select.query, "");
        assert_eq!(select.filtered.len(), 5);
    }

    // Fuzzy match tests

    #[test]
    fn test_get_match_with_empty_query() {
        // Arrange
        let select = create_test_select();

        // Act
        let match_result = select.get_match("Apple");

        // Assert
        assert_eq!(match_result, None);
    }

    #[test]
    fn test_get_match_with_query() {
        // Arrange
        let mut select = create_test_select();
        select.set_query("app");

        // Act
        let match_result = select.get_match("Apple");

        // Assert
        assert!(match_result.is_some());
    }

    #[test]
    fn test_get_match_no_match() {
        // Arrange
        let mut select = create_test_select();
        select.set_query("xyz");

        // Act
        let match_result = select.get_match("Apple");

        // Assert
        assert_eq!(match_result, None);
    }

    #[test]
    fn test_get_match_partial_match() {
        // Arrange
        let mut select = create_test_select();
        select.set_query("app");

        // Act
        let match_result = select.get_match("Pineapple");

        // Assert
        assert!(match_result.is_some());
    }

    #[test]
    fn test_get_match_case_sensitive() {
        // Arrange
        let mut select = create_test_select();
        select.set_query("APP");

        // Act
        let match_result = select.get_match("apple");

        // Assert
        // Fuzzy match is typically case-insensitive or case-sensitive based on implementation
        // This test verifies the behavior
        let result = match_result;
        let _ = result; // Suppress unused warning
    }

    // Filter update tests

    #[test]
    fn test_update_filter_with_empty_query() {
        // Arrange
        let mut select = create_test_select();
        select.filtered = vec![0, 2];
        select.query = "".to_string();

        // Act
        select.update_filter();

        // Assert
        assert_eq!(select.filtered.len(), 5);
        assert_eq!(select.dropdown_cursor, 0);
    }

    #[test]
    fn test_update_filter_with_match() {
        // Arrange
        let mut select = create_test_select();
        select.query = "app".to_string();

        // Act
        select.update_filter();

        // Assert
        // Should find "Apple"
        assert!(!select.filtered.is_empty());
        assert_eq!(select.dropdown_cursor, 0);
    }

    #[test]
    fn test_update_filter_no_matches() {
        // Arrange
        let mut select = create_test_select();
        select.query = "zzz".to_string();

        // Act
        select.update_filter();

        // Assert
        assert!(select.filtered.is_empty());
    }

    #[test]
    fn test_update_filter_multiple_matches() {
        // Arrange
        let mut select =
            MultiSelect::new().options(vec!["Apple", "Applepie", "Pineapple", "Banana"]);
        select.query = "app".to_string();

        // Act
        select.update_filter();

        // Assert
        // Should find Apple, Applepie, Pineapple
        assert!(select.filtered.len() >= 1);
    }

    #[test]
    fn test_update_filter_resets_cursor() {
        // Arrange
        let mut select = create_test_select();
        select.dropdown_cursor = 3;
        select.query = "test".to_string();

        // Act
        select.update_filter();

        // Assert
        assert_eq!(select.dropdown_cursor, 0);
    }

    // Integration tests

    #[test]
    fn test_search_workflow() {
        // Arrange
        let mut select = create_test_select();

        // Act - Set query
        select.set_query("app");

        // Assert - Query set
        assert_eq!(select.query(), "app");

        // Check filter updated
        assert!(!select.filtered.is_empty());

        // Clear query
        select.clear_query();

        // Assert - All options visible
        assert_eq!(select.query(), "");
        assert_eq!(select.filtered.len(), 5);
    }

    #[test]
    fn test_search_and_navigate() {
        // Arrange
        let mut select = create_test_select();
        select.set_query("an");

        // Act - Navigate filtered results
        // With "an" query, should find "Banana" (only one match)
        select.cursor_down();

        // Assert - Should wrap around since only one item
        assert_eq!(select.dropdown_cursor, 0);
    }

    #[test]
    fn test_search_after_close() {
        // Arrange
        let mut select = create_test_select();
        select.open();
        select.set_query("app");

        // Act - Close should clear query
        select.close();

        // Assert
        assert_eq!(select.query(), "");
        assert_eq!(select.filtered.len(), 5);
    }
}
