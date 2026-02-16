//! Tests for autocomplete/core.rs
//!
//! Extracted from src/widget/input/input_widgets/autocomplete/core.rs

use crate::event::{Key, KeyEvent};
use revue::widget::input::input_widgets::autocomplete::Autocomplete;
use revue::widget::input::input_widgets::autocomplete::types::Suggestion;
use revue::style::Color;
use revue::utils::FilterMode;

// =========================================================================
// Autocomplete::new tests
// =========================================================================

#[test]
fn test_autocomplete_new() {
    let ac = Autocomplete::new();
    assert_eq!(ac.get_value(), "");
    assert!(!ac.is_focused());
    assert_eq!(ac.get_min_chars_value(), 1);
    assert_eq!(ac.get_max_suggestions_value(), 10);
}

#[test]
fn test_autocomplete_default() {
    let ac = Autocomplete::default();
    assert_eq!(ac.get_value(), "");
    assert!(!ac.is_focused());
}

// =========================================================================
// Autocomplete::value tests
// =========================================================================

#[test]
fn test_autocomplete_value_str() {
    let ac = Autocomplete::new().value("test");
    assert_eq!(ac.get_value(), "test");
}

#[test]
fn test_autocomplete_value_string() {
    let ac = Autocomplete::new().value(String::from("owned"));
    assert_eq!(ac.get_value(), "owned");
}

#[test]
fn test_autocomplete_value_empty() {
    let ac = Autocomplete::new().value("");
    assert_eq!(ac.get_value(), "");
}

#[test]
fn test_autocomplete_value_unicode() {
    let ac = Autocomplete::new().value("你好");
    assert_eq!(ac.get_value(), "你好");
}

#[test]
fn test_autocomplete_value_emoji() {
    let ac = Autocomplete::new().value("🎉");
    assert_eq!(ac.get_value(), "🎉");
}

// =========================================================================
// Autocomplete::placeholder tests
// =========================================================================

#[test]
fn test_autocomplete_placeholder() {
    let ac = Autocomplete::new().placeholder("Search...");
    let _ = ac;
}

#[test]
fn test_autocomplete_placeholder_string() {
    let ac = Autocomplete::new().placeholder(String::from("Owned"));
    let _ = ac;
}

#[test]
fn test_autocomplete_placeholder_empty() {
    let ac = Autocomplete::new().placeholder("");
    let _ = ac;
}

// =========================================================================
// Autocomplete::filter_mode tests
// =========================================================================

#[test]
fn test_autocomplete_filter_mode_fuzzy() {
    let ac = Autocomplete::new().filter_mode(FilterMode::Fuzzy);
    let _ = ac;
}

#[test]
fn test_autocomplete_filter_mode_prefix() {
    let ac = Autocomplete::new().filter_mode(FilterMode::Prefix);
    let _ = ac;
}

#[test]
fn test_autocomplete_filter_mode_contains() {
    let ac = Autocomplete::new().filter_mode(FilterMode::Contains);
    let _ = ac;
}

#[test]
fn test_autocomplete_filter_mode_exact() {
    let ac = Autocomplete::new().filter_mode(FilterMode::Exact);
    let _ = ac;
}

#[test]
fn test_autocomplete_filter_mode_none() {
    let ac = Autocomplete::new().filter_mode(FilterMode::None);
    let _ = ac;
}

// =========================================================================
// Autocomplete::min_chars tests
// =========================================================================

#[test]
fn test_autocomplete_min_chars_zero() {
    let ac = Autocomplete::new().min_chars(0);
    assert_eq!(ac.get_min_chars_value(), 0);
}

#[test]
fn test_autocomplete_min_chars_one() {
    let ac = Autocomplete::new().min_chars(1);
    assert_eq!(ac.get_min_chars_value(), 1);
}

#[test]
fn test_autocomplete_min_chars_large() {
    let ac = Autocomplete::new().min_chars(100);
    assert_eq!(ac.get_min_chars_value(), 100);
}

// =========================================================================
// Autocomplete::max_suggestions tests
// =========================================================================

#[test]
fn test_autocomplete_max_suggestions_zero() {
    let ac = Autocomplete::new().max_suggestions(0);
    assert_eq!(ac.get_max_suggestions_value(), 0);
}

#[test]
fn test_autocomplete_max_suggestions_five() {
    let ac = Autocomplete::new().max_suggestions(5);
    assert_eq!(ac.get_max_suggestions_value(), 5);
}

#[test]
fn test_autocomplete_max_suggestions_large() {
    let ac = Autocomplete::new().max_suggestions(1000);
    assert_eq!(ac.get_max_suggestions_value(), 1000);
}

// =========================================================================
// Autocomplete::suggestions tests
// =========================================================================

#[test]
fn test_autocomplete_suggestions_vec() {
    let ac = Autocomplete::new()
        .suggestions(vec![Suggestion::new("Item 1"), Suggestion::new("Item 2")]);
    assert_eq!(ac.get_suggestions_list().len(), 2);
}

#[test]
fn test_autocomplete_suggestions_slice() {
    let items = ["Item 1", "Item 2"];
    let ac = Autocomplete::new().suggestions(items);
    assert_eq!(ac.get_suggestions_list().len(), 2);
}

#[test]
fn test_autocomplete_suggestions_empty() {
    let ac = Autocomplete::new().suggestions(Vec::<Suggestion>::new());
    assert_eq!(ac.get_suggestions_list().len(), 0);
}

// =========================================================================
// Autocomplete::input_style tests
// =========================================================================

#[test]
fn test_autocomplete_input_style() {
    let ac = Autocomplete::new().input_style(Color::RED, Color::BLUE);
    assert_eq!(ac.get_input_fg_color(), Color::RED);
    assert_eq!(ac.get_input_bg_color(), Color::BLUE);
}

// =========================================================================
// Autocomplete::dropdown_style tests
// =========================================================================

#[test]
fn test_autocomplete_dropdown_style() {
    let ac = Autocomplete::new().dropdown_style(Color::RED, Color::GREEN, Color::BLUE);
    assert_eq!(ac.get_dropdown_bg_color(), Color::RED);
    assert_eq!(ac.get_selected_fg_color(), Color::GREEN);
    assert_eq!(ac.get_selected_bg_color(), Color::BLUE);
}

// =========================================================================
// Autocomplete::highlight_fg tests
// =========================================================================

#[test]
fn test_autocomplete_highlight_fg() {
    let ac = Autocomplete::new().highlight_fg(Color::YELLOW);
    assert_eq!(ac.get_highlight_fg_color(), Color::YELLOW);
}

// =========================================================================
// Autocomplete setter methods tests
// =========================================================================

#[test]
fn test_autocomplete_set_value() {
    let mut ac = Autocomplete::new();
    ac.set_value("new value");
    assert_eq!(ac.get_value(), "new value");
}

#[test]
fn test_autocomplete_set_value_updates_cursor() {
    let mut ac = Autocomplete::new();
    ac.set_value("test");
    assert_eq!(ac.cursor(), 4);
}

#[test]
fn test_autocomplete_set_value_string() {
    let mut ac = Autocomplete::new();
    ac.set_value(String::from("owned"));
    assert_eq!(ac.get_value(), "owned");
}

#[test]
fn test_autocomplete_set_suggestions() {
    let mut ac = Autocomplete::new();
    ac.set_suggestions(vec![Suggestion::new("Item 1"), Suggestion::new("Item 2")]);
    assert_eq!(ac.get_suggestions_list().len(), 2);
}

// =========================================================================
// Autocomplete focus methods tests
// =========================================================================

#[test]
fn test_autocomplete_focus() {
    let mut ac = Autocomplete::new();
    assert!(!ac.is_focused());
    ac.focus();
    assert!(ac.is_focused());
}

#[test]
fn test_autocomplete_blur() {
    let mut ac = Autocomplete::new();
    ac.focus();
    assert!(ac.is_focused());
    ac.blur();
    assert!(!ac.is_focused());
}

#[test]
fn test_autocomplete_blur_hides_dropdown() {
    let mut ac = Autocomplete::new();
    ac.set_suggestions(vec![Suggestion::new("Test")]);
    ac.set_value("T");
    ac.focus();
    ac.update_filter();
    ac.blur();
    assert!(!ac.is_dropdown_visible());
}

#[test]
fn test_autocomplete_is_focused() {
    let ac = Autocomplete::new();
    assert!(!ac.is_focused());
}

// =========================================================================
// Autocomplete::selected_suggestion tests
// =========================================================================

#[test]
fn test_autocomplete_selected_suggestion_none() {
    let ac = Autocomplete::new();
    assert!(ac.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_selected_suggestion_with_filter() {
    let mut ac = Autocomplete::new();
    ac.set_suggestions(vec![Suggestion::new("Item 1"), Suggestion::new("Item 2")]);
    ac.set_value("Item");
    ac.focus();
    ac.update_filter();
    // Should have filtered suggestions
    let selected = ac.selected_suggestion();
    let _ = selected;
}

#[test]
fn test_autocomplete_selected_suggestion_empty_filtered() {
    let mut ac = Autocomplete::new();
    ac.set_suggestions(vec![Suggestion::new("Item")]);
    ac.set_value("X");
    ac.focus();
    ac.update_filter();
    assert!(ac.selected_suggestion().is_none());
}

// =========================================================================
// Autocomplete::accept_selection tests
// =========================================================================

#[test]
fn test_autocomplete_accept_selection_none() {
    let mut ac = Autocomplete::new();
    assert!(!ac.accept_selection());
}

#[test]
fn test_autocomplete_accept_selection_with_match() {
    let mut ac = Autocomplete::new();
    ac.set_suggestions(vec![Suggestion::new("Test")]);
    ac.set_value("T");
    ac.focus();
    ac.update_filter();
    // If there's a match, accept it
    let accepted = ac.accept_selection();
    let _ = accepted;
}

#[test]
fn test_autocomplete_accept_selection_hides_dropdown() {
    let mut ac = Autocomplete::new();
    ac.set_suggestions(vec![Suggestion::with_value("Test", "test-value")]);
    ac.set_value("T");
    ac.focus();
    ac.update_filter();
    ac.accept_selection();
    assert!(!ac.is_dropdown_visible());
}

// =========================================================================
// Autocomplete::handle_key tests
// =========================================================================

#[test]
fn test_autocomplete_handle_key_char() {
    let mut ac = Autocomplete::new();
    assert!(ac.handle_key(KeyEvent::new(Key::Char('a'))));
    assert_eq!(ac.get_value(), "a");
}

#[test]
fn test_autocomplete_handle_key_backspace() {
    let mut ac = Autocomplete::new();
    ac.set_value("ab");
    assert!(ac.handle_key(KeyEvent::new(Key::Backspace)));
    assert_eq!(ac.get_value(), "a");
}

#[test]
fn test_autocomplete_handle_key_backspace_empty() {
    let mut ac = Autocomplete::new();
    assert!(ac.handle_key(KeyEvent::new(Key::Backspace)));
    assert_eq!(ac.get_value(), "");
}

#[test]
fn test_autocomplete_handle_key_delete() {
    let mut ac = Autocomplete::new();
    ac.set_value("ab");
    ac.cursor = 0;
    assert!(ac.handle_key(KeyEvent::new(Key::Delete)));
    assert_eq!(ac.get_value(), "b");
}

#[test]
fn test_autocomplete_handle_key_delete_at_end() {
    let mut ac = Autocomplete::new();
    ac.set_value("ab");
    assert!(ac.handle_key(KeyEvent::new(Key::Delete)));
    assert_eq!(ac.get_value(), "ab");
}

#[test]
fn test_autocomplete_handle_key_left() {
    let mut ac = Autocomplete::new();
    ac.set_value("ab");
    assert!(ac.handle_key(KeyEvent::new(Key::Left)));
    assert_eq!(ac.cursor(), 1);
}

#[test]
fn test_autocomplete_handle_key_left_at_start() {
    let mut ac = Autocomplete::new();
    ac.set_value("ab");
    ac.cursor = 0;
    assert!(ac.handle_key(KeyEvent::new(Key::Left)));
    assert_eq!(ac.cursor(), 0);
}

#[test]
fn test_autocomplete_handle_key_right() {
    let mut ac = Autocomplete::new();
    ac.set_value("ab");
    ac.cursor = 0;
    assert!(ac.handle_key(KeyEvent::new(Key::Right)));
    assert_eq!(ac.cursor(), 1);
}

#[test]
fn test_autocomplete_handle_key_right_at_end() {
    let mut ac = Autocomplete::new();
    ac.set_value("ab");
    assert!(ac.handle_key(KeyEvent::new(Key::Right)));
    assert_eq!(ac.cursor(), 2);
}

#[test]
fn test_autocomplete_handle_key_home() {
    let mut ac = Autocomplete::new();
    ac.set_value("abc");
    ac.cursor = 2;
    assert!(ac.handle_key(KeyEvent::new(Key::Home)));
    assert_eq!(ac.cursor(), 0);
}

#[test]
fn test_autocomplete_handle_key_end() {
    let mut ac = Autocomplete::new();
    ac.set_value("abc");
    assert!(ac.handle_key(KeyEvent::new(Key::End)));
    assert_eq!(ac.cursor(), 3);
}

#[test]
fn test_autocomplete_handle_key_unhandled() {
    let mut ac = Autocomplete::new();
    assert!(!ac.handle_key(KeyEvent::new(Key::Tab)));
}

#[test]
fn test_autocomplete_handle_key_escape_without_dropdown() {
    let mut ac = Autocomplete::new();
    assert!(!ac.handle_key(KeyEvent::new(Key::Escape)));
}

#[test]
fn test_autocomplete_handle_key_up_without_dropdown() {
    let mut ac = Autocomplete::new();
    assert!(!ac.handle_key(KeyEvent::new(Key::Up)));
}

#[test]
fn test_autocomplete_handle_key_down_without_dropdown() {
    let mut ac = Autocomplete::new();
    assert!(!ac.handle_key(KeyEvent::new(Key::Down)));
}

// =========================================================================
// Autocomplete builder chain tests
// =========================================================================

#[test]
fn test_autocomplete_full_builder_chain() {
    let ac = Autocomplete::new()
        .value("test")
        .placeholder("Search...")
        .filter_mode(FilterMode::Prefix)
        .min_chars(2)
        .max_suggestions(5)
        .input_style(Color::WHITE, Color::BLACK)
        .dropdown_style(Color::rgb(128, 128, 128), Color::WHITE, Color::BLUE)
        .highlight_fg(Color::YELLOW)
        .suggestions(vec![Suggestion::new("Item 1"), Suggestion::new("Item 2")]);
    assert_eq!(ac.get_value(), "test");
    assert_eq!(ac.get_min_chars_value(), 2);
    assert_eq!(ac.get_max_suggestions_value(), 5);
}

// =========================================================================
// Autocomplete edge case tests
// =========================================================================

#[test]
fn test_autocomplete_unicode_input() {
    let mut ac = Autocomplete::new();
    ac.handle_key(KeyEvent::new(Key::Char('你')));
    assert_eq!(ac.get_value(), "你");
}

#[test]
fn test_autocomplete_emoji_input() {
    let mut ac = Autocomplete::new();
    ac.handle_key(KeyEvent::new(Key::Char('🎉')));
    assert_eq!(ac.get_value(), "🎉");
}

#[test]
fn test_autocomplete_newline_in_value() {
    let ac = Autocomplete::new().value("line1\nline2");
    assert_eq!(ac.get_value(), "line1\nline2");
}

#[test]
fn test_autocomplete_clone() {
    let ac1 = Autocomplete::new()
        .value("test")
        .placeholder("Search...")
        .suggestions(vec![Suggestion::new("Item")]);
    let ac2 = ac1.clone();
    assert_eq!(ac1.get_value(), ac2.get_value());
}

#[test]
fn test_autocomplete_debug() {
    let ac = Autocomplete::new().value("test");
    let debug_str = format!("{:?}", ac);
    assert!(debug_str.contains("Autocomplete"));
}
