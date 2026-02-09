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
