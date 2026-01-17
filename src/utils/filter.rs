//! Filter mode utilities for searchable widgets
//!
//! Provides a shared `FilterMode` enum used by combobox, autocomplete, and other
//! widgets that need to filter items based on user input.

/// Filter mode for matching items against user input
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FilterMode {
    /// Fuzzy matching (typo tolerant, e.g., "hw" matches "Hello World")
    #[default]
    Fuzzy,
    /// Prefix matching (starts with, e.g., "Hel" matches "Hello")
    Prefix,
    /// Contains matching (substring anywhere)
    Contains,
    /// Exact matching (case-insensitive)
    Exact,
    /// No filtering (show all items)
    None,
}

impl FilterMode {
    /// Check if a text matches the pattern using this filter mode
    pub fn matches(&self, text: &str, pattern: &str) -> bool {
        if pattern.is_empty() {
            return true;
        }

        match self {
            FilterMode::Fuzzy => {
                // Simple fuzzy: all pattern chars appear in order
                let mut pattern_chars = pattern.chars().peekable();
                for ch in text.chars() {
                    if let Some(&p) = pattern_chars.peek() {
                        if ch.eq_ignore_ascii_case(&p) {
                            pattern_chars.next();
                        }
                    }
                }
                pattern_chars.peek().is_none()
            }
            FilterMode::Prefix => text
                .to_ascii_lowercase()
                .starts_with(&pattern.to_ascii_lowercase()),
            FilterMode::Contains => text
                .to_ascii_lowercase()
                .contains(&pattern.to_ascii_lowercase()),
            FilterMode::Exact => text.eq_ignore_ascii_case(pattern),
            FilterMode::None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match() {
        assert!(FilterMode::Fuzzy.matches("Hello World", "hw"));
        assert!(FilterMode::Fuzzy.matches("Hello World", "HW"));
        assert!(FilterMode::Fuzzy.matches("Hello World", "helloworld"));
        assert!(!FilterMode::Fuzzy.matches("Hello World", "xyz"));
    }

    #[test]
    fn test_prefix_match() {
        assert!(FilterMode::Prefix.matches("Hello World", "hel"));
        assert!(FilterMode::Prefix.matches("Hello World", "Hello"));
        assert!(!FilterMode::Prefix.matches("Hello World", "World"));
    }

    #[test]
    fn test_contains_match() {
        assert!(FilterMode::Contains.matches("Hello World", "wor"));
        assert!(FilterMode::Contains.matches("Hello World", "llo"));
        assert!(!FilterMode::Contains.matches("Hello World", "xyz"));
    }

    #[test]
    fn test_exact_match() {
        assert!(FilterMode::Exact.matches("Hello", "hello"));
        assert!(FilterMode::Exact.matches("Hello", "HELLO"));
        assert!(!FilterMode::Exact.matches("Hello World", "Hello"));
    }

    #[test]
    fn test_none_match() {
        assert!(FilterMode::None.matches("anything", "pattern"));
        assert!(FilterMode::None.matches("", "pattern"));
    }

    #[test]
    fn test_empty_pattern() {
        assert!(FilterMode::Fuzzy.matches("Hello", ""));
        assert!(FilterMode::Prefix.matches("Hello", ""));
        assert!(FilterMode::Contains.matches("Hello", ""));
        assert!(FilterMode::Exact.matches("Hello", ""));
        assert!(FilterMode::None.matches("Hello", ""));
    }
}
