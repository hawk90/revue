//! Integration tests for diff utilities
//! Extracted from src/utils/diff.rs

use revue::utils::diff::*;

#[test]
fn test_diff_lines_equal() {
    let changes = diff_lines("a\nb\nc", "a\nb\nc");
    assert_eq!(changes.len(), 3);
    assert!(changes.iter().all(|c| c.is_equal()));
}

#[test]
fn test_diff_lines_insert() {
    let changes = diff_lines("a\nc", "a\nb\nc");
    assert_eq!(changes.len(), 3);
    assert!(changes[0].is_equal());
    assert!(changes[1].is_insert());
    assert_eq!(changes[1].text, "b");
    assert!(changes[2].is_equal());
}

#[test]
fn test_diff_lines_delete() {
    let changes = diff_lines("a\nb\nc", "a\nc");
    assert_eq!(changes.len(), 3);
    assert!(changes[0].is_equal());
    assert!(changes[1].is_delete());
    assert_eq!(changes[1].text, "b");
    assert!(changes[2].is_equal());
}

#[test]
fn test_diff_lines_replace() {
    let changes = diff_lines("a\nb\nc", "a\nx\nc");
    assert_eq!(changes.len(), 4);
    assert!(changes[0].is_equal());
    assert!(changes[1].is_delete());
    assert!(changes[2].is_insert());
    assert!(changes[3].is_equal());
}

#[test]
fn test_diff_chars() {
    let changes = diff_chars("hello", "hallo");
    // Should have: h (equal), e (delete), a (insert), llo (equal)
    assert!(changes.iter().any(|c| c.is_delete() && c.text == "e"));
    assert!(changes.iter().any(|c| c.is_insert() && c.text == "a"));
}

#[test]
fn test_diff_words() {
    let changes = diff_words("hello world", "hello rust world");
    assert!(changes.iter().any(|c| c.is_insert() && c.text == "rust"));
}

#[test]
fn test_diff_stats() {
    let changes = diff_lines("a\nb\nc", "a\nx\nc");
    let stats = DiffStats::from_changes(&changes);
    assert_eq!(stats.equal, 2);
    assert_eq!(stats.insertions, 1);
    assert_eq!(stats.deletions, 1);
}

#[test]
fn test_diff_similarity() {
    let changes = diff_lines("a\nb\nc\nd", "a\nb\nc\nd");
    let stats = DiffStats::from_changes(&changes);
    assert!((stats.similarity() - 1.0).abs() < 0.001);

    let changes = diff_lines("a", "b");
    let stats = DiffStats::from_changes(&changes);
    assert!(stats.similarity() < 0.5);
}

#[test]
fn test_empty_diff() {
    let changes = diff_lines("", "");
    assert!(changes.is_empty());

    let changes = diff_lines("", "a");
    assert_eq!(changes.len(), 1);
    assert!(changes[0].is_insert());

    let changes = diff_lines("a", "");
    assert_eq!(changes.len(), 1);
    assert!(changes[0].is_delete());
}
