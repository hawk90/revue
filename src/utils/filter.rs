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
