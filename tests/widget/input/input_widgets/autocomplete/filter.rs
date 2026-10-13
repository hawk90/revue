//! Autocomplete filter tests: the filter modes, min_chars and max_suggestions

use super::{dropdown, fruits, key, type_str};
use revue::event::Key;
use revue::utils::FilterMode;
use revue::widget::autocomplete::Autocomplete;

#[test]
fn test_autocomplete_filter_mode_fuzzy_is_default() {
    let mut a = fruits();
    type_str(&mut a, "ae"); // a..e subsequence in "apple" only
    assert_eq!(dropdown(&a, 20, 6), vec!["apple"]);
}

#[test]
fn test_autocomplete_filter_mode_prefix() {
    let mut a = fruits().filter_mode(FilterMode::Prefix);
    type_str(&mut a, "AP");
    assert_eq!(dropdown(&a, 20, 6), vec!["apple", "application"]);
}

#[test]
fn test_autocomplete_filter_mode_contains() {
    let mut a = fruits().filter_mode(FilterMode::Contains);
    type_str(&mut a, "an");
    assert_eq!(dropdown(&a, 20, 6), vec!["banana"]);
}

#[test]
fn test_autocomplete_filter_mode_exact() {
    let mut a = fruits().filter_mode(FilterMode::Exact);
    type_str(&mut a, "appl");
    assert!(dropdown(&a, 20, 6).is_empty());
    type_str(&mut a, "e");
    assert_eq!(dropdown(&a, 20, 6), vec!["apple"]);
}

#[test]
fn test_autocomplete_filter_mode_none() {
    let mut a = fruits().filter_mode(FilterMode::None);
    type_str(&mut a, "xyz");
    assert_eq!(
        dropdown(&a, 20, 6),
        vec!["apple", "application", "banana", "cherry"]
    );
}

#[test]
fn test_autocomplete_filter_mode_override() {
    let mut a = fruits()
        .filter_mode(FilterMode::Contains)
        .filter_mode(FilterMode::Prefix);
    type_str(&mut a, "an");
    assert!(dropdown(&a, 20, 6).is_empty());
}

#[test]
fn test_autocomplete_min_chars() {
    let mut a = fruits().min_chars(2);
    type_str(&mut a, "a");
    assert!(a.selected_suggestion().is_none());
    type_str(&mut a, "p");
    assert_eq!(a.selected_suggestion().unwrap().label, "apple");
}

#[test]
fn test_autocomplete_min_chars_zero() {
    // With no threshold, focusing an empty input lists every suggestion.
    let mut a = fruits().min_chars(0).filter_mode(FilterMode::None);
    a.focus();
    assert_eq!(dropdown(&a, 20, 6).len(), 4);
}

#[test]
fn test_autocomplete_min_chars_threshold() {
    let mut a = fruits().min_chars(3).value("ap");
    a.focus();
    assert!(dropdown(&a, 20, 6).is_empty());
    type_str(&mut a, "p");
    assert_eq!(dropdown(&a, 20, 6), vec!["apple", "application"]);
}

#[test]
fn test_autocomplete_min_chars_large() {
    let mut a = fruits().min_chars(100);
    type_str(&mut a, "apple");
    assert!(a.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_min_chars_counts_characters() {
    // One CJK character is one character, not three bytes.
    let mut a = Autocomplete::new()
        .suggestions(["한국", "한글"])
        .filter_mode(FilterMode::Prefix)
        .min_chars(2);
    type_str(&mut a, "한");
    assert!(a.selected_suggestion().is_none());
    type_str(&mut a, "글");
    assert_eq!(a.selected_suggestion().unwrap().label, "한글");
}

#[test]
fn test_autocomplete_max_suggestions() {
    let mut a = fruits().max_suggestions(1).filter_mode(FilterMode::None);
    type_str(&mut a, "a");
    assert_eq!(dropdown(&a, 20, 6), vec!["apple"]);
}

#[test]
fn test_autocomplete_max_suggestions_zero() {
    let mut a = fruits().max_suggestions(0);
    type_str(&mut a, "a");
    assert!(a.selected_suggestion().is_none());
    assert!(dropdown(&a, 20, 6).is_empty());
}

#[test]
fn test_autocomplete_max_suggestions_limit() {
    let mut a = Autocomplete::new()
        .suggestions((0..100).map(|i| format!("item{}", i)).collect::<Vec<_>>())
        .max_suggestions(5)
        .value("item");
    a.focus();
    assert_eq!(
        dropdown(&a, 20, 20),
        vec!["item0", "item1", "item2", "item3", "item4"]
    );
}

#[test]
fn test_autocomplete_set_value_filters() {
    let mut a = fruits();
    a.set_value("ban");
    assert_eq!(a.selected_suggestion().unwrap().label, "banana");
}

#[test]
fn test_autocomplete_deleting_below_min_chars_hides_dropdown() {
    let mut a = fruits().min_chars(2);
    type_str(&mut a, "ap");
    assert!(!dropdown(&a, 20, 6).is_empty());
    key(&mut a, Key::Backspace);
    assert!(dropdown(&a, 20, 6).is_empty());
    assert!(a.selected_suggestion().is_none());
}
