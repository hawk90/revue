//! Integration tests for fuzzy utilities
//! Extracted from src/utils/fuzzy.rs

use revue::utils::fuzzy::*;

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
