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
    fn test_fuzzy_match_order_matters() {
        // Pattern chars must appear in order
        assert!(FilterMode::Fuzzy.matches("abcdef", "ace"));
        assert!(!FilterMode::Fuzzy.matches("abcdef", "eca")); // wrong order
    }

    #[test]
    fn test_fuzzy_match_case_insensitive() {
        assert!(FilterMode::Fuzzy.matches("HelloWorld", "hw"));
        assert!(FilterMode::Fuzzy.matches("helloworld", "HW"));
        assert!(FilterMode::Fuzzy.matches("HELLO", "hello"));
    }

    #[test]
    fn test_fuzzy_match_single_char() {
        assert!(FilterMode::Fuzzy.matches("Hello", "h"));
        assert!(FilterMode::Fuzzy.matches("Hello", "e"));
        assert!(FilterMode::Fuzzy.matches("Hello", "o"));
        assert!(!FilterMode::Fuzzy.matches("Hello", "x"));
    }

    #[test]
    fn test_prefix_match() {
        assert!(FilterMode::Prefix.matches("Hello World", "hel"));
        assert!(FilterMode::Prefix.matches("Hello World", "Hello"));
        assert!(!FilterMode::Prefix.matches("Hello World", "World"));
    }

    #[test]
    fn test_prefix_match_case_insensitive() {
        assert!(FilterMode::Prefix.matches("Hello", "HEL"));
        assert!(FilterMode::Prefix.matches("HELLO", "hel"));
        assert!(FilterMode::Prefix.matches("hello", "HEL"));
    }

    #[test]
    fn test_prefix_match_full_string() {
        assert!(FilterMode::Prefix.matches("Hello", "Hello"));
        assert!(FilterMode::Prefix.matches("Hello", "hello"));
    }

    #[test]
    fn test_contains_match() {
        assert!(FilterMode::Contains.matches("Hello World", "wor"));
        assert!(FilterMode::Contains.matches("Hello World", "llo"));
        assert!(!FilterMode::Contains.matches("Hello World", "xyz"));
    }

    #[test]
    fn test_contains_match_at_start() {
        assert!(FilterMode::Contains.matches("Hello World", "Hel"));
        assert!(FilterMode::Contains.matches("Hello World", "hel"));
    }

    #[test]
    fn test_contains_match_at_end() {
        assert!(FilterMode::Contains.matches("Hello World", "rld"));
        assert!(FilterMode::Contains.matches("Hello World", "RLD"));
    }

    #[test]
    fn test_contains_match_full_string() {
        assert!(FilterMode::Contains.matches("Hello", "Hello"));
        assert!(FilterMode::Contains.matches("Hello", "hello"));
    }

    #[test]
    fn test_exact_match() {
        assert!(FilterMode::Exact.matches("Hello", "hello"));
        assert!(FilterMode::Exact.matches("Hello", "HELLO"));
        assert!(!FilterMode::Exact.matches("Hello World", "Hello"));
    }

    #[test]
    fn test_exact_match_partial_fails() {
        assert!(!FilterMode::Exact.matches("Hello World", "Hello"));
        assert!(!FilterMode::Exact.matches("Hello", "Hel"));
        assert!(!FilterMode::Exact.matches("Hello", "ello"));
    }

    #[test]
    fn test_none_match() {
        assert!(FilterMode::None.matches("anything", "pattern"));
        assert!(FilterMode::None.matches("", "pattern"));
    }

    #[test]
    fn test_none_match_always_true() {
        assert!(FilterMode::None.matches("", ""));
        assert!(FilterMode::None.matches("abc", "xyz"));
        assert!(FilterMode::None.matches("短い", "long pattern"));
    }

    #[test]
    fn test_empty_pattern() {
        assert!(FilterMode::Fuzzy.matches("Hello", ""));
        assert!(FilterMode::Prefix.matches("Hello", ""));
        assert!(FilterMode::Contains.matches("Hello", ""));
        assert!(FilterMode::Exact.matches("Hello", ""));
        assert!(FilterMode::None.matches("Hello", ""));
    }

    #[test]
    fn test_empty_text() {
        assert!(!FilterMode::Fuzzy.matches("", "a"));
        assert!(!FilterMode::Prefix.matches("", "a"));
        assert!(!FilterMode::Contains.matches("", "a"));
        assert!(!FilterMode::Exact.matches("", "a"));
        assert!(FilterMode::None.matches("", "a")); // None always matches
    }

    #[test]
    fn test_both_empty() {
        assert!(FilterMode::Fuzzy.matches("", ""));
        assert!(FilterMode::Prefix.matches("", ""));
        assert!(FilterMode::Contains.matches("", ""));
        assert!(FilterMode::Exact.matches("", ""));
        assert!(FilterMode::None.matches("", ""));
    }

    #[test]
    fn test_filter_mode_default() {
        assert_eq!(FilterMode::default(), FilterMode::Fuzzy);
    }

    #[test]
    fn test_filter_mode_clone() {
        let mode = FilterMode::Prefix;
        let cloned = mode;
        assert_eq!(mode, cloned);
    }

    #[test]
    fn test_filter_mode_debug() {
        assert_eq!(format!("{:?}", FilterMode::Fuzzy), "Fuzzy");
        assert_eq!(format!("{:?}", FilterMode::Prefix), "Prefix");
        assert_eq!(format!("{:?}", FilterMode::Contains), "Contains");
        assert_eq!(format!("{:?}", FilterMode::Exact), "Exact");
        assert_eq!(format!("{:?}", FilterMode::None), "None");
    }

    #[test]
    fn test_special_characters() {
        assert!(FilterMode::Contains.matches("hello-world", "-"));
        assert!(FilterMode::Contains.matches("hello_world", "_"));
        assert!(FilterMode::Prefix.matches("[test]", "["));
        assert!(FilterMode::Fuzzy.matches("a.b.c", "abc"));
    }

    #[test]
    fn test_whitespace() {
        assert!(FilterMode::Contains.matches("hello world", " "));
        assert!(FilterMode::Fuzzy.matches("hello world", "hw"));
        assert!(FilterMode::Prefix.matches("  hello", "  "));
    }

    #[test]
    fn test_numbers() {
        assert!(FilterMode::Fuzzy.matches("test123", "t13"));
        assert!(FilterMode::Prefix.matches("123abc", "123"));
        assert!(FilterMode::Contains.matches("abc123def", "123"));
        assert!(FilterMode::Exact.matches("123", "123"));
    }
}
