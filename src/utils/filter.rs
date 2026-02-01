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
    ///
    /// Uses Unicode-aware case folding for non-ASCII characters,
    /// ensuring proper matching for text like "É" vs "é".
    pub fn matches(&self, text: &str, pattern: &str) -> bool {
        if pattern.is_empty() {
            return true;
        }

        match self {
            FilterMode::Fuzzy => {
                // Simple fuzzy: all pattern chars appear in order
                // Use Unicode-aware case comparison for non-ASCII
                let mut pattern_chars = pattern.chars().peekable();
                for ch in text.chars() {
                    if let Some(&p) = pattern_chars.peek() {
                        // Try ASCII case compare first (fast path)
                        if ch.eq_ignore_ascii_case(&p) {
                            pattern_chars.next();
                        } else {
                            // Fallback to Unicode case folding for non-ASCII
                            let ch_lower = ch.to_lowercase().next().unwrap_or(ch);
                            let p_lower = p.to_lowercase().next().unwrap_or(p);
                            if ch_lower == p_lower {
                                pattern_chars.next();
                            }
                        }
                    }
                }
                pattern_chars.peek().is_none()
            }
            FilterMode::Prefix => {
                // Use Unicode to_lowercase for proper case-insensitive comparison
                let text_lower = text.to_lowercase();
                let pattern_lower = pattern.to_lowercase();
                text_lower.starts_with(&pattern_lower)
            }
            FilterMode::Contains => {
                // Use Unicode to_lowercase for proper case-insensitive comparison
                let text_lower = text.to_lowercase();
                let pattern_lower = pattern.to_lowercase();
                text_lower.contains(&pattern_lower)
            }
            FilterMode::Exact => {
                // Use Unicode to_lowercase for proper case-insensitive comparison
                let text_lower = text.to_lowercase();
                let pattern_lower = pattern.to_lowercase();
                text_lower == pattern_lower
            }
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

    // Non-ASCII Unicode tests - proper case folding

    #[test]
    fn test_unicode_accented_e() {
        // É (U+00C9) should match é (U+00E9) case-insensitively
        assert!(FilterMode::Exact.matches("Éléphant", "éléphant"));
        assert!(FilterMode::Exact.matches("ÉLÉPHANT", "éléphant"));
        assert!(FilterMode::Exact.matches("éléphant", "ÉLÉPHANT"));
        assert!(FilterMode::Prefix.matches("Éléphant", "él"));
        assert!(FilterMode::Contains.matches("Éléphant", "lé"));
    }

    #[test]
    fn test_unicode_german_umlaut() {
        // Ä should match ä case-insensitively
        assert!(FilterMode::Exact.matches("Ärger", "ärger"));
        assert!(FilterMode::Prefix.matches("Ärger", "är"));
        assert!(FilterMode::Contains.matches("Ärger", "rg"));
    }

    #[test]
    fn test_unicode_cjk() {
        // CJK characters don't have case, but should still match
        assert!(FilterMode::Exact.matches("你好", "你好"));
        assert!(FilterMode::Prefix.matches("你好世界", "你好"));
        assert!(FilterMode::Contains.matches("你好世界", "好"));
    }

    #[test]
    fn test_unicode_mixed_script() {
        // Mixed ASCII and non-ASCII
        assert!(FilterMode::Exact.matches("Café", "café"));
        assert!(FilterMode::Contains.matches("Café au lait", "fé"));
        assert!(FilterMode::Prefix.matches("Éclair", "écl"));
    }

    #[test]
    fn test_unicode_fuzzy() {
        // Fuzzy matching with Unicode
        assert!(FilterMode::Fuzzy.matches("Éléphant", "éph"));
        assert!(FilterMode::Fuzzy.matches("Café au lait", "cal"));
    }
}
