use super::*;
use crate::event::{Key, KeyEvent};
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::utils::FilterMode;
use crate::widget::traits::RenderContext;

#[test]
fn test_suggestion_new() {
    let s = Suggestion::new("test");
    assert_eq!(s.label, "test");
    assert_eq!(s.value, "test");
}

#[test]
fn test_suggestion_with_value() {
    let s = Suggestion::with_value("Display", "actual_value");
    assert_eq!(s.label, "Display");
    assert_eq!(s.value, "actual_value");
}

#[test]
fn test_autocomplete_new() {
    let ac = Autocomplete::new();
    assert_eq!(ac.get_value(), "");
    assert!(!ac.focused);
}

#[test]
fn test_autocomplete_suggestions() {
    let ac = Autocomplete::new().suggestions(vec!["apple", "banana", "cherry"]);
    assert_eq!(ac.suggestions.len(), 3);
}

#[test]
fn test_autocomplete_filter() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "apricot", "banana"])
        .min_chars(1);

    ac.set_value("ap");
    assert_eq!(ac.filtered.len(), 2); // apple, apricot
}

#[test]
fn test_autocomplete_accept() {
    let mut ac = Autocomplete::new().suggestions(vec!["apple", "banana"]);

    ac.set_value("a");
    ac.accept_selection();
    assert_eq!(ac.get_value(), "apple");
}

#[test]
fn test_autocomplete_key_handling() {
    let mut ac = Autocomplete::new();
    ac.focus();

    ac.handle_key(KeyEvent::new(Key::Char('h')));
    ac.handle_key(KeyEvent::new(Key::Char('i')));
    assert_eq!(ac.get_value(), "hi");

    ac.handle_key(KeyEvent::new(Key::Backspace));
    assert_eq!(ac.get_value(), "h");
}

#[test]
fn test_autocomplete_render() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .value("a");
    ac.render(&mut ctx);
}

#[test]
fn test_autocomplete_helper() {
    let ac = autocomplete().placeholder("Search...");
    assert_eq!(ac.placeholder, "Search...");
}

// =========================================================================
// Additional coverage tests
// =========================================================================

#[test]
fn test_autocomplete_default() {
    let ac = Autocomplete::default();
    assert_eq!(ac.get_value(), "");
}

#[test]
fn test_autocomplete_blur() {
    let mut ac = Autocomplete::new().suggestions(vec!["apple", "banana"]);

    ac.focus();
    ac.set_value("a");
    assert!(ac.is_focused());

    ac.blur();
    assert!(!ac.is_focused());
    assert!(!ac.dropdown_visible);
}

#[test]
fn test_autocomplete_selected_suggestion_empty() {
    let ac = Autocomplete::new().suggestions(vec!["apple", "banana"]);
    // No filter applied yet
    assert!(ac.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_accept_selection_empty() {
    let mut ac = Autocomplete::new();
    // No suggestions, should return false
    assert!(!ac.accept_selection());
}

#[test]
fn test_autocomplete_min_chars() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(3);

    ac.set_value("ap");
    // Should not filter because min_chars is 3
    assert!(ac.filtered.is_empty());

    ac.set_value("app");
    // Now should filter
    assert!(!ac.filtered.is_empty());
}

#[test]
fn test_autocomplete_max_suggestions() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["a1", "a2", "a3", "a4", "a5", "a6"])
        .max_suggestions(3)
        .min_chars(1);

    ac.set_value("a");
    assert_eq!(ac.filtered.len(), 3);
}

#[test]
fn test_autocomplete_filter_mode_prefix() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "application", "banana"])
        .filter_mode(FilterMode::Prefix)
        .min_chars(1);

    ac.set_value("app");
    assert_eq!(ac.filtered.len(), 2); // apple, application
}

#[test]
fn test_autocomplete_filter_mode_contains() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "pineapple", "banana"])
        .filter_mode(FilterMode::Contains)
        .min_chars(1);

    ac.set_value("apple");
    assert_eq!(ac.filtered.len(), 2); // apple, pineapple
}

#[test]
fn test_autocomplete_filter_mode_exact() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "APPLE", "apples"])
        .filter_mode(FilterMode::Exact)
        .min_chars(1);

    ac.set_value("apple");
    assert_eq!(ac.filtered.len(), 2); // apple, APPLE (case insensitive)
}

#[test]
fn test_autocomplete_filter_mode_none() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana", "cherry"])
        .filter_mode(FilterMode::None)
        .min_chars(1);

    ac.set_value("xyz");
    assert_eq!(ac.filtered.len(), 3); // All suggestions shown
}

#[test]
fn test_autocomplete_handle_key_delete() {
    let mut ac = Autocomplete::new();
    ac.focus();

    ac.handle_key(KeyEvent::new(Key::Char('a')));
    ac.handle_key(KeyEvent::new(Key::Char('b')));
    ac.handle_key(KeyEvent::new(Key::Home));
    ac.handle_key(KeyEvent::new(Key::Delete));
    assert_eq!(ac.get_value(), "b");
}

#[test]
fn test_autocomplete_handle_key_cursor_movement() {
    let mut ac = Autocomplete::new();
    ac.focus();

    ac.handle_key(KeyEvent::new(Key::Char('a')));
    ac.handle_key(KeyEvent::new(Key::Char('b')));
    ac.handle_key(KeyEvent::new(Key::Char('c')));

    ac.handle_key(KeyEvent::new(Key::Home));
    assert_eq!(ac.cursor, 0);

    ac.handle_key(KeyEvent::new(Key::End));
    assert_eq!(ac.cursor, 3);

    ac.handle_key(KeyEvent::new(Key::Left));
    assert_eq!(ac.cursor, 2);

    ac.handle_key(KeyEvent::new(Key::Right));
    assert_eq!(ac.cursor, 3);
}

#[test]
fn test_autocomplete_handle_key_dropdown_navigation() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["a1", "a2", "a3"])
        .min_chars(1);

    ac.focus();
    ac.handle_key(KeyEvent::new(Key::Char('a')));

    assert!(ac.dropdown_visible);

    ac.handle_key(KeyEvent::new(Key::Down));
    assert_eq!(ac.selection.index, 1);

    ac.handle_key(KeyEvent::new(Key::Up));
    assert_eq!(ac.selection.index, 0);
}

#[test]
fn test_autocomplete_handle_key_escape() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["a1", "a2"])
        .min_chars(1);

    ac.focus();
    ac.handle_key(KeyEvent::new(Key::Char('a')));
    assert!(ac.dropdown_visible);

    ac.handle_key(KeyEvent::new(Key::Escape));
    assert!(!ac.dropdown_visible);
}

#[test]
fn test_autocomplete_handle_key_tab() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);

    ac.focus();
    ac.handle_key(KeyEvent::new(Key::Char('a')));
    ac.handle_key(KeyEvent::new(Key::Tab));

    assert_eq!(ac.get_value(), "apple");
}

#[test]
fn test_autocomplete_handle_key_unhandled() {
    let mut ac = Autocomplete::new();
    ac.focus();

    let handled = ac.handle_key(KeyEvent::new(Key::F(1)));
    assert!(!handled);
}

#[test]
fn test_autocomplete_colors() {
    let ac = Autocomplete::new()
        .input_style(Color::WHITE, Color::BLACK)
        .dropdown_style(Color::rgb(50, 50, 50), Color::WHITE, Color::BLUE)
        .highlight_fg(Color::YELLOW);

    assert_eq!(ac.input_fg, Color::WHITE);
    assert_eq!(ac.input_bg, Color::BLACK);
    assert_eq!(ac.dropdown_bg, Color::rgb(50, 50, 50));
    assert_eq!(ac.selected_fg, Color::WHITE);
    assert_eq!(ac.selected_bg, Color::BLUE);
    assert_eq!(ac.highlight_fg, Color::YELLOW);
}

#[test]
fn test_autocomplete_render_with_placeholder() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let ac = Autocomplete::new().placeholder("Type here...");
    ac.render(&mut ctx);
}

#[test]
fn test_autocomplete_render_with_cursor() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut ac = Autocomplete::new().value("test");
    ac.focus();
    ac.render(&mut ctx);
}

#[test]
fn test_autocomplete_render_dropdown() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut ac = Autocomplete::new()
        .suggestions(vec![
            Suggestion::new("apple").description("A fruit").icon('🍎'),
            Suggestion::new("banana"),
        ])
        .min_chars(1);

    ac.focus();
    ac.set_value("a");
    ac.render(&mut ctx);
}

#[test]
fn test_autocomplete_render_small_area() {
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let ac = Autocomplete::new().value("test");
    ac.render(&mut ctx);
    // Small area should not panic
}

#[test]
fn test_suggestion_description_and_icon() {
    let s = Suggestion::new("test")
        .description("A test suggestion")
        .icon('✓');

    assert_eq!(s.description, Some("A test suggestion".to_string()));
    assert_eq!(s.icon, Some('✓'));
}

#[test]
fn test_suggestion_from() {
    let s: Suggestion = "quick".into();
    assert_eq!(s.label, "quick");
    assert_eq!(s.value, "quick");
}

#[test]
fn test_autocomplete_set_suggestions() {
    let mut ac = Autocomplete::new();
    ac.set_suggestions(vec![Suggestion::new("one"), Suggestion::new("two")]);
    assert_eq!(ac.suggestions.len(), 2);
}
