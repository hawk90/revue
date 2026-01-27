//! Fuzzy matching utilities
//!
//! Provides fuzzy string matching for search, autocomplete, and command palettes.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::fuzzy_match;
//!
//! let result = fuzzy_match("fzf", "fuzzy finder");
//! assert!(result.is_some());
//! assert!(result.unwrap().score > 0);
//!
//! // Get matched indices for highlighting
//! let result = fuzzy_match("cmd", "CommandPalette").unwrap();
//! assert_eq!(result.indices, vec![0, 3, 7]); // C, m, d
//! ```

/// Result of a fuzzy match
#[derive(Clone, Debug, PartialEq)]
pub struct FuzzyMatch {
    /// Match score (higher is better)
    pub score: i32,
    /// Indices of matched characters in the target string
    pub indices: Vec<usize>,
}

impl FuzzyMatch {
    /// Create a new fuzzy match result
    pub fn new(score: i32, indices: Vec<usize>) -> Self {
        Self { score, indices }
    }
}

/// Fuzzy match a pattern against a target string
///
/// Returns `Some(FuzzyMatch)` if the pattern matches, `None` otherwise.
/// The match is case-insensitive and allows characters to be non-contiguous.
///
/// # Scoring
///
/// - Consecutive matches: +3 bonus
/// - Match at word start: +2 bonus
/// - Match at string start: +2 bonus
/// - Case-sensitive match: +1 bonus
/// - Each matched character: +1
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::fuzzy_match;
///
/// // Matches "fzf" in "fuzzy finder"
/// let m = fuzzy_match("fzf", "fuzzy finder").unwrap();
/// println!("Score: {}, Indices: {:?}", m.score, m.indices);
///
/// // No match
/// assert!(fuzzy_match("xyz", "fuzzy finder").is_none());
/// ```
pub fn fuzzy_match(pattern: &str, target: &str) -> Option<FuzzyMatch> {
    if pattern.is_empty() {
        return Some(FuzzyMatch::new(0, vec![]));
    }

    let pattern_lower: Vec<char> = pattern.to_lowercase().chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let target_lower: Vec<char> = target.to_lowercase().chars().collect();

    let mut indices = Vec::with_capacity(pattern_lower.len());
    let mut score = 0i32;
    let mut pattern_idx = 0;
    let mut prev_match_idx: Option<usize> = None;

    for (i, &ch) in target_lower.iter().enumerate() {
        if pattern_idx < pattern_lower.len() && ch == pattern_lower[pattern_idx] {
            indices.push(i);

            // Base score for match
            score += 1;

            // Bonus for case-sensitive match
            if target_chars[i] == pattern_chars.get(pattern_idx).copied().unwrap_or(' ') {
                score += 1;
            }

            // Bonus for consecutive matches
            if let Some(prev) = prev_match_idx {
                if i == prev + 1 {
                    score += 3;
                }
            }

            // Bonus for match at word boundary
            if i == 0 {
                score += 2; // Start of string
            } else {
                let prev_char = target_chars[i - 1];
                if !prev_char.is_alphanumeric()
                    || (prev_char.is_lowercase() && target_chars[i].is_uppercase())
                {
                    score += 2; // Word boundary or camelCase
                }
            }

            prev_match_idx = Some(i);
            pattern_idx += 1;
        }
    }

    if pattern_idx == pattern_lower.len() {
        Some(FuzzyMatch::new(score, indices))
    } else {
        None
    }
}

/// Fuzzy match with a minimum score threshold
pub fn fuzzy_match_threshold(pattern: &str, target: &str, min_score: i32) -> Option<FuzzyMatch> {
    fuzzy_match(pattern, target).filter(|m| m.score >= min_score)
}

/// Score a fuzzy match (returns 0 if no match)
pub fn fuzzy_score(pattern: &str, target: &str) -> i32 {
    fuzzy_match(pattern, target).map(|m| m.score).unwrap_or(0)
}

/// Check if pattern matches target (simple boolean check)
pub fn fuzzy_matches(pattern: &str, target: &str) -> bool {
    fuzzy_match(pattern, target).is_some()
}

/// Filter and sort items by fuzzy match score
///
/// Returns items sorted by score (highest first), filtering out non-matches.
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::fuzzy_filter;
///
/// let items = vec!["apple", "application", "banana", "appetite"];
/// let results = fuzzy_filter("app", &items);
/// // Returns: ["application", "appetite", "apple"] (sorted by score)
/// ```
pub fn fuzzy_filter<'a, T: AsRef<str>>(pattern: &str, items: &'a [T]) -> Vec<(&'a T, FuzzyMatch)> {
    let mut matches: Vec<_> = items
        .iter()
        .filter_map(|item| fuzzy_match(pattern, item.as_ref()).map(|m| (item, m)))
        .collect();

    matches.sort_by(|a, b| b.1.score.cmp(&a.1.score));
    matches
}

/// Filter items and return only the strings (without match info)
pub fn fuzzy_filter_simple<'a, T: AsRef<str>>(pattern: &str, items: &'a [T]) -> Vec<&'a T> {
    fuzzy_filter(pattern, items)
        .into_iter()
        .map(|(item, _)| item)
        .collect()
}

/// A fuzzy matcher that can be reused for multiple matches
#[derive(Clone, Debug)]
pub struct FuzzyMatcher {
    /// Pattern to match (lowercase)
    pattern: Vec<char>,
    /// Original pattern for case-sensitive bonus
    original_chars: Vec<char>,
    /// Minimum score threshold
    min_score: i32,
}

impl FuzzyMatcher {
    /// Create a new fuzzy matcher
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_lowercase().chars().collect(),
            original_chars: pattern.chars().collect(),
            min_score: 0,
        }
    }

    /// Set minimum score threshold
    pub fn min_score(mut self, score: i32) -> Self {
        self.min_score = score;
        self
    }

    /// Check if the pattern is empty
    pub fn is_empty(&self) -> bool {
        self.pattern.is_empty()
    }

    /// Match against a target string
    pub fn match_str(&self, target: &str) -> Option<FuzzyMatch> {
        if self.pattern.is_empty() {
            return Some(FuzzyMatch::new(0, vec![]));
        }

        let target_chars: Vec<char> = target.chars().collect();
        let target_lower: Vec<char> = target.to_lowercase().chars().collect();

        let mut indices = Vec::with_capacity(self.pattern.len());
        let mut score = 0i32;
        let mut pattern_idx = 0;
        let mut prev_match_idx: Option<usize> = None;

        for (i, &ch) in target_lower.iter().enumerate() {
            if pattern_idx < self.pattern.len() && ch == self.pattern[pattern_idx] {
                indices.push(i);
                score += 1;

                // Case-sensitive bonus
                if let Some(orig_ch) = self.original_chars.get(pattern_idx).copied() {
                    if target_chars[i] == orig_ch {
                        score += 1;
                    }
                }

                // Consecutive bonus
                if let Some(prev) = prev_match_idx {
                    if i == prev + 1 {
                        score += 3;
                    }
                }

                // Word boundary bonus
                if i == 0 {
                    score += 2;
                } else {
                    let prev_char = target_chars[i - 1];
                    if !prev_char.is_alphanumeric()
                        || (prev_char.is_lowercase() && target_chars[i].is_uppercase())
                    {
                        score += 2;
                    }
                }

                prev_match_idx = Some(i);
                pattern_idx += 1;
            }
        }

        if pattern_idx == self.pattern.len() && score >= self.min_score {
            Some(FuzzyMatch::new(score, indices))
        } else {
            None
        }
    }

    /// Filter and sort items
    pub fn filter<'a, T: AsRef<str>>(&self, items: &'a [T]) -> Vec<(&'a T, FuzzyMatch)> {
        let mut matches: Vec<_> = items
            .iter()
            .filter_map(|item| self.match_str(item.as_ref()).map(|m| (item, m)))
            .collect();

        matches.sort_by(|a, b| b.1.score.cmp(&a.1.score));
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_match() {
        let m = fuzzy_match("abc", "abc").unwrap();
        assert!(m.score > 0);
        assert_eq!(m.indices, vec![0, 1, 2]);
    }

    #[test]
    fn test_non_contiguous_match() {
        let m = fuzzy_match("fzf", "fuzzy finder").unwrap();
        assert!(m.score > 0);
        assert_eq!(m.indices, vec![0, 2, 6]);
    }

    #[test]
    fn test_case_insensitive() {
        let m = fuzzy_match("ABC", "abcdef").unwrap();
        assert!(m.score > 0);

        let m = fuzzy_match("abc", "ABCDEF").unwrap();
        assert!(m.score > 0);
    }

    #[test]
    fn test_no_match() {
        assert!(fuzzy_match("xyz", "abcdef").is_none());
        assert!(fuzzy_match("ab", "dcef").is_none());
    }

    #[test]
    fn test_empty_pattern() {
        let m = fuzzy_match("", "anything").unwrap();
        assert_eq!(m.score, 0);
        assert!(m.indices.is_empty());
    }

    #[test]
    fn test_consecutive_bonus() {
        let consecutive = fuzzy_match("abc", "abcxyz").unwrap();
        let non_consecutive = fuzzy_match("abc", "axbxcx").unwrap();
        assert!(consecutive.score > non_consecutive.score);
    }

    #[test]
    fn test_word_boundary_bonus() {
        // "cmd" in "CommandPalette" should match C, m, d at word boundaries
        let m = fuzzy_match("cp", "CommandPalette").unwrap();
        assert!(m.score > 0);
        // C at start, P at camelCase boundary
        assert_eq!(m.indices, vec![0, 7]);
    }

    #[test]
    fn test_fuzzy_filter() {
        let items = vec!["apple", "application", "banana", "appetite"];
        let results = fuzzy_filter("app", &items);

        assert_eq!(results.len(), 3);
        // All should contain "app"
        for (item, _) in &results {
            assert!(item.contains("app"));
        }
    }

    #[test]
    fn test_fuzzy_matcher() {
        let matcher = FuzzyMatcher::new("cmd");

        assert!(matcher.match_str("command").is_some());
        assert!(matcher.match_str("CommandPalette").is_some());
        assert!(matcher.match_str("xyz").is_none());
    }

    #[test]
    fn test_fuzzy_matches() {
        assert!(fuzzy_matches("fzf", "fuzzy finder"));
        assert!(!fuzzy_matches("xyz", "fuzzy finder"));
    }

    #[test]
    fn test_fuzzy_score() {
        assert!(fuzzy_score("abc", "abc") > 0);
        assert_eq!(fuzzy_score("xyz", "abc"), 0);
    }

    #[test]
    fn test_real_world_commands() {
        let commands = vec![
            "File: Open",
            "File: Save",
            "File: Save As",
            "Edit: Copy",
            "Edit: Paste",
            "View: Toggle Sidebar",
            "Git: Commit",
            "Git: Push",
        ];

        // "fs" should match "File: Save" well
        let results = fuzzy_filter("fs", &commands);
        assert!(!results.is_empty());

        // "gp" should match "Git: Push"
        let results = fuzzy_filter("gp", &commands);
        assert!(!results.is_empty());
        assert!(results[0].0.contains("Git"));
    }

    // =============================================================================
    // Edge Case Tests
    // =============================================================================

    #[test]
    fn test_unicode_pattern() {
        // Pattern with unicode characters
        let m = fuzzy_match("你好", "你好世界").unwrap();
        assert!(m.score > 0);
        assert_eq!(m.indices, vec![0, 1]);
    }

    #[test]
    fn test_unicode_target() {
        // ASCII pattern in unicode target
        let m = fuzzy_match("hw", "Hello 世界 World").unwrap();
        assert!(m.score > 0);
    }

    #[test]
    fn test_pattern_longer_than_target() {
        assert!(fuzzy_match("abcdefgh", "abc").is_none());
    }

    #[test]
    fn test_empty_target() {
        assert!(fuzzy_match("abc", "").is_none());
    }

    #[test]
    fn test_both_empty() {
        let m = fuzzy_match("", "").unwrap();
        assert_eq!(m.score, 0);
        assert!(m.indices.is_empty());
    }

    #[test]
    fn test_min_score_threshold() {
        // Low threshold should match
        let m = fuzzy_match_threshold("a", "apple", 1);
        assert!(m.is_some());

        // Very high threshold should not match
        let m = fuzzy_match_threshold("a", "zzzzza", 100);
        assert!(m.is_none());
    }

    #[test]
    fn test_filter_empty_items() {
        let items: Vec<&str> = vec![];
        let results = fuzzy_filter("abc", &items);
        assert!(results.is_empty());
    }

    #[test]
    fn test_filter_sort_order() {
        let items = vec!["abc", "abcd", "ab"];
        let results = fuzzy_filter("ab", &items);

        // "ab" should be first (exact match at start)
        // Results should be sorted by score descending
        assert_eq!(results.len(), 3);
        // Verify scores are in descending order
        for i in 0..results.len() - 1 {
            assert!(results[i].1.score >= results[i + 1].1.score);
        }
    }

    #[test]
    fn test_matcher_min_score() {
        let matcher = FuzzyMatcher::new("a").min_score(10);
        // Single 'a' in long string may not meet threshold
        assert!(matcher.match_str("zzzzzzzzza").is_none());

        // But 'a' at start should have higher score
        assert!(
            matcher.match_str("apple").is_none() || matcher.match_str("apple").unwrap().score >= 10
        );
    }

    #[test]
    fn test_matcher_is_empty() {
        let empty_matcher = FuzzyMatcher::new("");
        assert!(empty_matcher.is_empty());

        let matcher = FuzzyMatcher::new("abc");
        assert!(!matcher.is_empty());
    }

    #[test]
    fn test_matcher_filter() {
        let matcher = FuzzyMatcher::new("ab");
        let items = vec!["abc", "xyz", "aab"];
        let results = matcher.filter(&items);

        assert_eq!(results.len(), 2); // "abc" and "aab"
    }

    #[test]
    fn test_fuzzy_filter_simple() {
        let items = vec!["apple", "banana", "apricot"];
        let results = fuzzy_filter_simple("ap", &items);

        assert_eq!(results.len(), 2); // "apple" and "apricot"
        for item in results {
            assert!(item.starts_with("ap"));
        }
    }

    #[test]
    fn test_case_sensitive_bonus() {
        // Exact case match should score higher
        let exact = fuzzy_match("Abc", "Abc").unwrap();
        let lower = fuzzy_match("abc", "Abc").unwrap();
        // Note: exact case match gets bonus points
        assert!(exact.score >= lower.score);
    }

    #[test]
    fn test_special_characters() {
        let m = fuzzy_match("f:o", "File: Open").unwrap();
        assert!(m.score > 0);

        let m = fuzzy_match("->", "a->b->c").unwrap();
        assert!(m.score > 0);
    }
}
