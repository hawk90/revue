use revue::widget::{autocomplete, Autocomplete};
use revue::event::{Key, KeyEvent};
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
    assert_eq!(ac.min_chars, 1);
    assert_eq!(ac.max_suggestions, 10);
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
    assert_eq!(ac.min_chars, 0);
}

#[test]
fn test_autocomplete_min_chars_one() {
    let ac = Autocomplete::new().min_chars(1);
    assert_eq!(ac.min_chars, 1);
}

#[test]
fn test_autocomplete_min_chars_large() {
    let ac = Autocomplete::new().min_chars(100);
    assert_eq!(ac.min_chars, 100);
}

// =========================================================================
// Autocomplete::max_suggestions tests
// =========================================================================

#[test]
fn test_autocomplete_max_suggestions_zero() {
    let ac = Autocomplete::new().max_suggestions(0);
    assert_eq!(ac.max_suggestions, 0);
}

#[test]
fn test_autocomplete_max_suggestions_five() {
    let ac = Autocomplete::new().max_suggestions(5);
    assert_eq!(ac.max_suggestions, 5);
}

#[test]
fn test_autocomplete_max_suggestions_large() {
    let ac = Autocomplete::new().max_suggestions(1000);
    assert_eq!(ac.max_suggestions, 1000);
}

// =========================================================================
// Autocomplete::suggestions tests
// =========================================================================

#[test]
fn test_autocomplete_suggestions_vec() {
    let ac = Autocomplete::new()
        .suggestions(vec!["Item 1", "Item 2"]);
    assert_eq!(ac.suggestions.len(), 2);
}

#[test]
fn test_autocomplete_suggestions_slice() {
    let items = ["Item 1", "Item 2"];
    let ac = Autocomplete::new().suggestions(items);
    assert_eq!(ac.suggestions.len(), 2);
}

#[test]
fn test_autocomplete_suggestions_empty() {
    let ac = Autocomplete::new().suggestions(Vec::<String>::new());
    assert_eq!(ac.suggestions.len(), 0);
}

// =========================================================================
// Autocomplete::input_style tests
// =========================================================================

#[test]
fn test_autocomplete_input_style() {
    let ac = Autocomplete::new().input_style(Color::RED, Color::BLUE);
    assert_eq!(ac.input_fg, Color::RED);
    assert_eq!(ac.input_bg, Color::BLUE);
}

// =========================================================================
// Autocomplete::dropdown_style tests
// =========================================================================

#[test]
fn test_autocomplete_dropdown_style() {
    let ac = Autocomplete::new().dropdown_style(Color::RED, Color::GREEN, Color::BLUE);
    assert_eq!(ac.dropdown_bg, Color::RED);
    assert_eq!(ac.selected_fg, Color::GREEN);
    assert_eq!(ac.selected_bg, Color::BLUE);
}

// =========================================================================
// Autocomplete::highlight_fg tests
// =========================================================================

#[test]
fn test_autocomplete_highlight_fg() {
    let ac = Autocomplete::new().highlight_fg(Color::YELLOW);
    assert_eq!(ac.highlight_fg, Color::YELLOW);
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
    assert_eq!(ac.cursor, 4);
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
    ac.set_suggestions(vec!["Item 1", "Item 2"]);
    assert_eq!(ac.suggestions.len(), 2);
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
    ac.set_suggestions(vec!["Test"]);
    ac.set_value("T");
    ac.focus();
    // Trigger filter update
    ac.update_filter();
    ac.blur();
    assert!(!ac.dropdown_visible);
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
    ac.set_suggestions(vec!["Item 1", "Item 2"]);
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
    ac.set_suggestions(vec!["Item"]);
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
    ac.set_suggestions(vec!["Test"]);
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
    ac.set_suggestions(vec!["Test"]);
    ac.set_value("T");
    ac.focus();
    ac.update_filter();
    ac.accept_selection();
    assert!(!ac.dropdown_visible);
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
    assert_eq!(ac.cursor, 1);
}

#[test]
fn test_autocomplete_handle_key_left_at_start() {
    let mut ac = Autocomplete::new();
    ac.set_value("ab");
    ac.cursor = 0;
    assert!(ac.handle_key(KeyEvent::new(Key::Left)));
    assert_eq!(ac.cursor, 0);
}

#[test]
fn test_autocomplete_handle_key_right() {
    let mut ac = Autocomplete::new();
    ac.set_value("ab");
    ac.cursor = 0;
    assert!(ac.handle_key(KeyEvent::new(Key::Right)));
    assert_eq!(ac.cursor, 1);
}

#[test]
fn test_autocomplete_handle_key_right_at_end() {
    let mut ac = Autocomplete::new();
    ac.set_value("ab");
    assert!(ac.handle_key(KeyEvent::new(Key::Right)));
    assert_eq!(ac.cursor, 2);
}

#[test]
fn test_autocomplete_handle_key_home() {
    let mut ac = Autocomplete::new();
    ac.set_value("abc");
    ac.cursor = 2;
    assert!(ac.handle_key(KeyEvent::new(Key::Home)));
    assert_eq!(ac.cursor, 0);
}

#[test]
fn test_autocomplete_handle_key_end() {
    let mut ac = Autocomplete::new();
    ac.set_value("abc");
    assert!(ac.handle_key(KeyEvent::new(Key::End)));
    assert_eq!(ac.cursor, 3);
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
        .suggestions(vec!["Item 1", "Item 2"]);
    assert_eq!(ac.get_value(), "test");
    assert_eq!(ac.min_chars, 2);
    assert_eq!(ac.max_suggestions, 5);
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
        .suggestions(vec!["Item"]);
    let ac2 = ac1.clone();
    assert_eq!(ac1.get_value(), ac2.get_value());
}

#[test]
fn test_autocomplete_debug() {
    let ac = Autocomplete::new().value("test");
    let debug_str = format!("{:?}", ac);
    assert!(debug_str.contains("Autocomplete"));
}

// =========================================================================
// autocomplete helper tests
// =========================================================================

#[test]
fn test_autocomplete_helper_creates_widget() {
    let widget = autocomplete();
    // Should create a widget with default values
    assert!(!widget.is_focused());
}

#[test]
fn test_autocomplete_helper_default_value() {
    let widget = autocomplete();
    assert_eq!(widget.get_value(), "");
}

#[test]
fn test_autocomplete_helper_chainable() {
    let widget = autocomplete()
        .value("test")
        .placeholder("Enter text...")
        .min_chars(1)
        .max_suggestions(10);
    assert_eq!(widget.get_value(), "test");
}

#[test]
fn test_autocomplete_helper_with_value() {
    let widget = autocomplete().value("hello");
    assert_eq!(widget.get_value(), "hello");
}

#[test]
fn test_autocomplete_helper_with_placeholder() {
    let widget = autocomplete().placeholder("Search...");
    let _ = widget;
}

#[test]
fn test_autocomplete_helper_with_filter_mode() {
    let widget = autocomplete().filter_mode(FilterMode::Prefix);
    let _ = widget;
}

#[test]
fn test_autocomplete_helper_with_min_chars() {
    let widget = autocomplete().min_chars(2);
    let _ = widget;
}

#[test]
fn test_autocomplete_helper_with_max_suggestions() {
    let widget = autocomplete().max_suggestions(5);
    let _ = widget;
}

#[test]
fn test_autocomplete_helper_focus() {
    let mut widget = autocomplete();
    assert!(!widget.is_focused());
    widget.focus();
    assert!(widget.is_focused());
}

#[test]
fn test_autocomplete_helper_blur() {
    let mut widget = autocomplete();
    widget.focus();
    assert!(widget.is_focused());
    widget.blur();
    assert!(!widget.is_focused());
}

#[test]
fn test_autocomplete_helper_set_value() {
    let mut widget = autocomplete();
    widget.set_value("new value");
    assert_eq!(widget.get_value(), "new value");
}

#[test]
fn test_autocomplete_helper_set_suggestions() {
    let mut widget = autocomplete();
    let suggestions = vec!["Option 1", "Option 2"];
    widget.set_suggestions(suggestions);
    let _ = widget;
}

#[test]
fn test_autocomplete_helper_no_selection_initially() {
    let widget = autocomplete();
    assert!(widget.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_helper_input_style() {
    let widget = autocomplete().input_style(Color::WHITE, Color::BLACK);
    let _ = widget;
}

#[test]
fn test_autocomplete_helper_dropdown_style() {
    let widget =
        autocomplete().dropdown_style(Color::BLUE, Color::WHITE, Color::rgb(0, 0, 139));
    let _ = widget;
}

#[test]
fn test_autocomplete_helper_highlight_fg() {
    let widget = autocomplete().highlight_fg(Color::YELLOW);
    let _ = widget;
}

#[test]
fn test_autocomplete_helper_handle_key() {
    let mut widget = autocomplete();
    widget.set_suggestions(vec!["Test"]);
    widget.focus();
    let handled = widget.handle_key(KeyEvent::new(Key::Char('t')));
    // Key should be handled
    assert!(handled);
}

#[test]
fn test_autocomplete_helper_accept_selection() {
    let mut widget = autocomplete();
    widget.set_suggestions(vec!["Item"]);
    widget.set_value("i");
    widget.focus();
    // Set up selection by typing and matching
    let accepted = widget.accept_selection();
    // May accept a selection if there's a match
    let _ = accepted;
}

#[test]
fn test_autocomplete_helper_with_string_value() {
    let widget = autocomplete().value(String::from("test"));
    assert_eq!(widget.get_value(), "test");
}

#[test]
fn test_autocomplete_helper_empty_value() {
    let widget = autocomplete().value("");
    assert_eq!(widget.get_value(), "");
}

#[test]
fn test_autocomplete_helper_chained_builders() {
    let widget = autocomplete()
        .value("search")
        .placeholder("Type to search...")
        .min_chars(1)
        .max_suggestions(8)
        .filter_mode(FilterMode::Prefix)
        .input_style(Color::rgb(255, 255, 255), Color::rgb(0, 0, 0))
        .dropdown_style(
            Color::rgb(50, 50, 50),
            Color::rgb(255, 255, 255),
            Color::rgb(100, 100, 255),
        )
        .highlight_fg(Color::YELLOW);
    assert_eq!(widget.get_value(), "search");
}