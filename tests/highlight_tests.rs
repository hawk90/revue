//! Text highlighting utilities tests

use revue::style::Color;
use revue::utils::highlight::{
    highlight_matches, highlight_range, highlight_ranges, highlight_substring, HighlightSpan,
    Highlighter,
};

#[test]
fn test_highlight_matches_basic() {
    let spans = highlight_matches("Hello", &[0, 4]);

    // "H" (highlighted), "ell" (normal), "o" (highlighted)
    assert_eq!(spans.len(), 3);
    assert!(spans[0].highlighted);
    assert_eq!(spans[0].text, "H");
    assert!(!spans[1].highlighted);
    assert_eq!(spans[1].text, "ell");
    assert!(spans[2].highlighted);
    assert_eq!(spans[2].text, "o");
}

#[test]
fn test_highlight_matches_empty() {
    let spans = highlight_matches("Hello", &[]);
    assert_eq!(spans.len(), 1);
    assert!(!spans[0].highlighted);
    assert_eq!(spans[0].text, "Hello");
}

#[test]
fn test_highlight_matches_consecutive() {
    let spans = highlight_matches("Hello", &[0, 1, 2]);

    assert_eq!(spans.len(), 2);
    assert!(spans[0].highlighted);
    assert_eq!(spans[0].text, "Hel");
    assert!(!spans[1].highlighted);
    assert_eq!(spans[1].text, "lo");
}

#[test]
fn test_highlight_substring() {
    let spans = highlight_substring("Hello World", "World");

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].text, "Hello ");
    assert!(!spans[0].highlighted);
    assert_eq!(spans[1].text, "World");
    assert!(spans[1].highlighted);
}

#[test]
fn test_highlight_substring_case_insensitive() {
    let spans = highlight_substring("Hello WORLD", "world");

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[1].text, "WORLD");
    assert!(spans[1].highlighted);
}

#[test]
fn test_highlight_substring_multiple() {
    let spans = highlight_substring("foo bar foo", "foo");

    assert_eq!(spans.len(), 3);
    assert!(spans[0].highlighted);
    assert!(!spans[1].highlighted);
    assert!(spans[2].highlighted);
}

#[test]
fn test_highlight_range() {
    let spans = highlight_range("Hello World", 6, 11);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].text, "Hello ");
    assert_eq!(spans[1].text, "World");
    assert!(spans[1].highlighted);
}

#[test]
fn test_highlight_ranges_merge() {
    let spans = highlight_ranges("Hello World", &[(0, 3), (2, 5)]);

    // Should merge overlapping ranges
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].text, "Hello");
    assert!(spans[0].highlighted);
}

#[test]
fn test_highlighter_colors() {
    let h = Highlighter::yellow();

    let span_hi = HighlightSpan::highlighted("test", 0, 4);
    let span_no = HighlightSpan::normal("test", 0, 4);

    assert_eq!(h.fg_for(&span_hi), Some(Color::BLACK));
    assert_eq!(h.bg_for(&span_hi), Some(Color::YELLOW));
    assert_eq!(h.fg_for(&span_no), None);
}

#[test]
fn test_unicode_highlighting() {
    let spans = highlight_matches("안녕하세요", &[0, 2]);

    assert!(spans[0].highlighted); // 안
    assert!(!spans[1].highlighted); // 녕
    assert!(spans[2].highlighted); // 하
}
