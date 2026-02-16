//! SearchBar widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{search_bar, SearchBar};

// =============================================================================
// Constructor Tests
// =============================================================================

#[test]
fn test_search_bar_new_creates_empty_search_bar() {
    let s = SearchBar::new();
    assert!(s.get_input().is_empty());
    assert!(s.is_valid());
    assert_eq!(s.placeholder, "Search...");
    assert_eq!(s.width, 40);
    assert!(!s.is_focused());
    assert!(s.show_hints);
    assert_eq!(s.icon, '🔍');
}

#[test]
fn test_search_bar_default_trait() {
    let s = SearchBar::default();
    assert!(s.get_input().is_empty());
    assert!(s.is_valid());
    assert_eq!(s.width, 40);
}

#[test]
fn test_search_bar_helper_function() {
    let s = search_bar();
    assert!(s.get_input().is_empty());
    assert!(s.is_valid());
}

// =============================================================================
// Builder Method Tests
// =============================================================================

#[test]
fn test_search_bar_placeholder_builder() {
    let s = SearchBar::new().placeholder("Type here...");
    assert_eq!(s.placeholder, "Type here...");
}

#[test]
fn test_search_bar_placeholder_builder_with_string() {
    let s = SearchBar::new().placeholder(String::from("Custom placeholder"));
    assert_eq!(s.placeholder, "Custom placeholder");
}

#[test]
fn test_search_bar_width_builder() {
    let s = SearchBar::new().width(60);
    assert_eq!(s.width, 60);
}

#[test]
fn test_search_bar_width_builder_clamps_minimum() {
    let s = SearchBar::new().width(5);
    assert_eq!(s.width, 10); // Minimum is 10
}

#[test]
fn test_search_bar_icon_builder() {
    let s = SearchBar::new().icon('🔎');
    assert_eq!(s.icon, '🔎');
}

#[test]
fn test_search_bar_show_hints_builder() {
    let s = SearchBar::new().show_hints(false);
    assert!(!s.show_hints);
}

#[test]
fn test_search_bar_show_hints_builder_true() {
    let s = SearchBar::new().show_hints(true);
    assert!(s.show_hints);
}

#[test]
fn test_search_bar_colors_builder() {
    let s = SearchBar::new().colors(Color::RED, Color::BLUE, Color::GREEN);
    assert_eq!(s.bg_color, Color::RED);
    assert_eq!(s.border_color, Color::BLUE);
    assert_eq!(s.text_color, Color::GREEN);
}

#[test]
fn test_search_bar_error_color_builder() {
    let s = SearchBar::new().error_color(Color::YELLOW);
    assert_eq!(s.error_color, Color::YELLOW);
}

#[test]
fn test_search_bar_builder_chaining() {
    let s = SearchBar::new()
        .placeholder("Search")
        .width(50)
        .icon('🔍')
        .show_hints(false)
        .colors(Color::WHITE, Color::rgb(128, 128, 128), Color::BLACK)
        .error_color(Color::RED);

    assert_eq!(s.placeholder, "Search");
    assert_eq!(s.width, 50);
    assert_eq!(s.icon, '🔍');
    assert!(!s.show_hints);
    assert_eq!(s.text_color, Color::BLACK);
    assert_eq!(s.error_color, Color::RED);
}

// =============================================================================
// Focus State Tests
// =============================================================================

#[test]
fn test_search_bar_focus_sets_focused() {
    let mut s = SearchBar::new();
    s.focus();
    assert!(s.is_focused());
}

#[test]
fn test_search_bar_focus_can_be_called_multiple_times() {
    let mut s = SearchBar::new();
    s.focus();
    s.focus();
    assert!(s.is_focused());
}

#[test]
fn test_search_bar_blur_clears_focused() {
    let mut s = SearchBar::new();
    s.focus();
    s.blur();
    assert!(!s.is_focused());
}

#[test]
fn test_search_bar_blur_can_be_called_multiple_times() {
    let mut s = SearchBar::new();
    s.blur();
    s.blur();
    assert!(!s.is_focused());
}

#[test]
fn test_search_bar_is_focused_returns_true_when_focused() {
    let mut s = SearchBar::new();
    s.focus();
    assert!(s.is_focused());
}

#[test]
fn test_search_bar_is_focused_returns_false_when_not_focused() {
    let s = SearchBar::new();
    assert!(!s.is_focused());
}

// =============================================================================
// Input Manipulation Tests
// =============================================================================

#[test]
fn test_search_bar_input_inserts_character() {
    let mut s = SearchBar::new();
    s.input('a');
    assert_eq!(s.get_input(), "a");
}

#[test]
fn test_search_bar_input_inserts_multiple_characters() {
    let mut s = SearchBar::new();
    s.input('h');
    s.input('e');
    s.input('l');
    s.input('l');
    s.input('o');
    assert_eq!(s.get_input(), "hello");
}

#[test]
fn test_search_bar_input_moves_cursor() {
    let mut s = SearchBar::new();
    s.input('a');
    s.input('b');
    assert_eq!(s.cursor, 2);
}

#[test]
fn test_search_bar_input_with_unicode() {
    let mut s = SearchBar::new();
    s.input('🎉');
    s.input('한');
    s.input('글');
    assert_eq!(s.get_input(), "🎉한글");
    assert_eq!(s.cursor, 3);
}

#[test]
fn test_search_bar_backspace_deletes_at_end() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    s.backspace();
    assert_eq!(s.get_input(), "hell");
    assert_eq!(s.cursor, 4);
}

#[test]
fn test_search_bar_backspace_moves_cursor() {
    let mut s = SearchBar::new();
    s.set_query("ab");
    s.cursor = 2;
    s.backspace();
    assert_eq!(s.cursor, 1);
}

#[test]
fn test_search_bar_backspace_at_start_does_nothing() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    s.cursor = 0;
    s.backspace();
    assert_eq!(s.get_input(), "hello");
    assert_eq!(s.cursor, 0);
}

#[test]
fn test_search_bar_backspace_on_empty_does_nothing() {
    let mut s = SearchBar::new();
    s.backspace();
    assert!(s.get_input().is_empty());
    assert_eq!(s.cursor, 0);
}

#[test]
fn test_search_bar_delete_deletes_at_cursor() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    s.cursor = 2;
    s.delete();
    assert_eq!(s.get_input(), "helo");
}

#[test]
fn test_search_bar_delete_at_end_does_nothing() {
    let mut s = SearchBar::new();
    s.set_query("hi");
    s.cursor = 2;
    s.delete();
    assert_eq!(s.get_input(), "hi");
}

#[test]
fn test_search_bar_delete_on_empty_does_nothing() {
    let mut s = SearchBar::new();
    s.delete();
    assert!(s.get_input().is_empty());
}

#[test]
fn test_search_bar_cursor_left_moves_left() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    s.cursor_left();
    assert_eq!(s.cursor, 4);
}

#[test]
fn test_search_bar_cursor_left_at_start_stays() {
    let mut s = SearchBar::new();
    s.set_query("hi");
    s.cursor = 0;
    s.cursor_left();
    assert_eq!(s.cursor, 0);
}

#[test]
fn test_search_bar_cursor_right_moves_right() {
    let mut s = SearchBar::new();
    s.set_query("hi");
    s.cursor = 0;
    s.cursor_right();
    assert_eq!(s.cursor, 1);
}

#[test]
fn test_search_bar_cursor_right_at_end_stays() {
    let mut s = SearchBar::new();
    s.set_query("hi");
    s.cursor_right();
    assert_eq!(s.cursor, 2);
}

#[test]
fn test_search_bar_cursor_home_moves_to_start() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    s.cursor = 3;
    s.cursor_home();
    assert_eq!(s.cursor, 0);
}

#[test]
fn test_search_bar_cursor_home_at_start_stays() {
    let mut s = SearchBar::new();
    s.set_query("test");
    s.cursor_home();
    assert_eq!(s.cursor, 0);
}

#[test]
fn test_search_bar_cursor_end_moves_to_end() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    s.cursor = 0;
    s.cursor_end();
    assert_eq!(s.cursor, 5);
}

#[test]
fn test_search_bar_cursor_end_at_end_stays() {
    let mut s = SearchBar::new();
    s.set_query("test");
    s.cursor_end();
    assert_eq!(s.cursor, 4);
}

// =============================================================================
// Query Handling Tests
// =============================================================================

#[test]
fn test_search_bar_set_query_sets_text() {
    let mut s = SearchBar::new();
    s.set_query("test query");
    assert_eq!(s.get_input(), "test query");
}

#[test]
fn test_search_bar_set_query_with_string() {
    let mut s = SearchBar::new();
    s.set_query(String::from("test"));
    assert_eq!(s.get_input(), "test");
}

#[test]
fn test_search_bar_set_query_moves_cursor_to_end() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    assert_eq!(s.cursor, 5);
}

#[test]
fn test_search_bar_set_query_empty() {
    let mut s = SearchBar::new();
    s.set_query("");
    assert!(s.get_input().is_empty());
    assert_eq!(s.cursor, 0);
}

#[test]
fn test_search_bar_set_query_with_unicode() {
    let mut s = SearchBar::new();
    s.set_query("🎉한글");
    assert_eq!(s.get_input(), "🎉한글");
    assert_eq!(s.cursor, 3);
}

#[test]
fn test_search_bar_get_input_returns_current_text() {
    let mut s = SearchBar::new();
    s.set_query("test");
    assert_eq!(s.get_input(), "test");
}

#[test]
fn test_search_bar_get_input_empty() {
    let s = SearchBar::new();
    assert!(s.get_input().is_empty());
}

// =============================================================================
// Query Parsing Tests
// =============================================================================

#[test]
fn test_search_bar_query_returns_some_when_valid() {
    let mut s = SearchBar::new();
    s.set_query("author:john");
    assert!(s.query().is_some());
}

#[test]
fn test_search_bar_query_returns_none_when_invalid() {
    let mut s = SearchBar::new();
    s.set_query("sort:name:invalid"); // Invalid sort direction causes parse error
    assert!(s.query().is_none());
}

#[test]
fn test_search_bar_query_on_empty_returns_default_query() {
    let s = SearchBar::new();
    let query = s.query().unwrap();
    assert!(query.is_empty());
}

#[test]
fn test_search_bar_error_returns_some_when_invalid() {
    let mut s = SearchBar::new();
    s.set_query("sort:name:invalid"); // Invalid sort direction causes parse error
    assert!(s.error().is_some());
}

#[test]
fn test_search_bar_error_returns_none_when_valid() {
    let mut s = SearchBar::new();
    s.set_query("author:john");
    assert!(s.error().is_none());
}

#[test]
fn test_search_bar_is_valid_returns_true_when_valid() {
    let mut s = SearchBar::new();
    s.set_query("author:john");
    assert!(s.is_valid());
}

#[test]
fn test_search_bar_is_valid_returns_false_when_invalid() {
    let mut s = SearchBar::new();
    s.set_query("sort:name:invalid"); // Invalid sort direction causes parse error
    assert!(!s.is_valid());
}

#[test]
fn test_search_bar_is_valid_returns_true_on_empty() {
    let s = SearchBar::new();
    assert!(s.is_valid());
}

// =============================================================================
// Clear Operation Tests
// =============================================================================

#[test]
fn test_search_bar_clear_clears_input() {
    let mut s = SearchBar::new();
    s.set_query("test");
    s.clear();
    assert!(s.get_input().is_empty());
}

#[test]
fn test_search_bar_clear_resets_cursor() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    s.clear();
    assert_eq!(s.cursor, 0);
}

#[test]
fn test_search_bar_clear_clears_parse_error() {
    let mut s = SearchBar::new();
    s.set_query("sort:name:invalid"); // Invalid sort direction causes parse error
    assert!(s.error().is_some());
    s.clear();
    assert!(s.error().is_none());
}

#[test]
fn test_search_bar_clear_restores_valid_query() {
    let mut s = SearchBar::new();
    s.set_query("sort:name:invalid"); // Invalid sort direction causes parse error
    assert!(!s.is_valid());
    s.clear();
    assert!(s.is_valid());
}

#[test]
fn test_search_bar_clear_can_be_called_on_empty() {
    let mut s = SearchBar::new();
    s.clear();
    s.clear();
    assert!(s.get_input().is_empty());
}

// =============================================================================
// Key Handling Tests
// =============================================================================

#[test]
fn test_search_bar_handle_key_char_inserts() {
    let mut s = SearchBar::new();
    let handled = s.handle_key(&Key::Char('a'));
    assert!(handled);
    assert_eq!(s.get_input(), "a");
}

#[test]
fn test_search_bar_handle_key_backspace_deletes() {
    let mut s = SearchBar::new();
    s.set_query("hi");
    let handled = s.handle_key(&Key::Backspace);
    assert!(handled);
    assert_eq!(s.get_input(), "h");
}

#[test]
fn test_search_bar_handle_key_delete_deletes() {
    let mut s = SearchBar::new();
    s.set_query("hi");
    s.cursor = 0;
    let handled = s.handle_key(&Key::Delete);
    assert!(handled);
    assert_eq!(s.get_input(), "i");
}

#[test]
fn test_search_bar_handle_key_left_moves_cursor() {
    let mut s = SearchBar::new();
    s.set_query("hi");
    let handled = s.handle_key(&Key::Left);
    assert!(handled);
    assert_eq!(s.cursor, 1);
}

#[test]
fn test_search_bar_handle_key_right_moves_cursor() {
    let mut s = SearchBar::new();
    s.set_query("hi");
    s.cursor = 0;
    let handled = s.handle_key(&Key::Right);
    assert!(handled);
    assert_eq!(s.cursor, 1);
}

#[test]
fn test_search_bar_handle_key_home_moves_to_start() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    let handled = s.handle_key(&Key::Home);
    assert!(handled);
    assert_eq!(s.cursor, 0);
}

#[test]
fn test_search_bar_handle_key_end_moves_to_end() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    s.cursor = 0;
    let handled = s.handle_key(&Key::End);
    assert!(handled);
    assert_eq!(s.cursor, 5);
}

#[test]
fn test_search_bar_handle_key_escape_clears() {
    let mut s = SearchBar::new();
    s.set_query("test query");
    let handled = s.handle_key(&Key::Escape);
    assert!(handled);
    assert!(s.get_input().is_empty());
}

#[test]
fn test_search_bar_handle_key_unknown_returns_false() {
    let mut s = SearchBar::new();
    let handled = s.handle_key(&Key::F(1));
    assert!(!handled);
}

#[test]
fn test_search_bar_handle_key_page_up_returns_false() {
    let mut s = SearchBar::new();
    let handled = s.handle_key(&Key::PageUp);
    assert!(!handled);
}

#[test]
fn test_search_bar_handle_key_page_down_returns_false() {
    let mut s = SearchBar::new();
    let handled = s.handle_key(&Key::PageDown);
    assert!(!handled);
}

// =============================================================================
// Rendering Tests
// =============================================================================

#[test]
fn test_search_bar_render_without_panic() {
    let mut buffer = Buffer::new(50, 2);
    let area = Rect::new(0, 0, 50, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = search_bar().width(40).placeholder("Search...");
    s.render(&mut ctx);
    // Smoke test - just verify it renders without panic
}

#[test]
fn test_search_bar_render_with_text() {
    let mut buffer = Buffer::new(50, 2);
    let area = Rect::new(0, 0, 50, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut s = search_bar().width(40);
    s.set_query("test query");
    s.render(&mut ctx);
    // Smoke test
}

#[test]
fn test_search_bar_render_with_error() {
    let mut buffer = Buffer::new(50, 2);
    let area = Rect::new(0, 0, 50, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut s = search_bar().width(40);
    s.set_query("sort:name:invalid"); // Invalid sort direction causes parse error
    s.render(&mut ctx);
    // Smoke test
}

#[test]
fn test_search_bar_render_focused() {
    let mut buffer = Buffer::new(50, 2);
    let area = Rect::new(0, 0, 50, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut s = search_bar().width(40);
    s.focus();
    s.render(&mut ctx);
    // Smoke test
}

#[test]
fn test_search_bar_render_small_area() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = search_bar().width(5);
    s.render(&mut ctx);
    // Should handle small area gracefully
}

#[test]
fn test_search_bar_render_with_hints() {
    let mut buffer = Buffer::new(50, 3);
    let area = Rect::new(0, 0, 50, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut s = search_bar().width(40).show_hints(true);
    s.focus();
    s.render(&mut ctx);
    // Should show hints
}

#[test]
fn test_search_bar_render_without_hints() {
    let mut buffer = Buffer::new(50, 3);
    let area = Rect::new(0, 0, 50, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = search_bar().width(40).show_hints(false);
    s.render(&mut ctx);
    // Should not show hints
}

// =============================================================================
// Initial State Tests
// =============================================================================

#[test]
fn test_search_bar_initial_state_is_valid() {
    let s = SearchBar::new();
    assert!(s.is_valid());
    assert!(s.query().is_some());
    assert!(s.error().is_none());
}

#[test]
fn test_search_bar_default_colors() {
    let s = SearchBar::new();
    assert_eq!(s.bg_color, Color::rgb(30, 30, 40));
    assert_eq!(s.border_color, Color::rgb(80, 80, 100));
    assert_eq!(s.text_color, Color::WHITE);
    assert_eq!(s.placeholder_color, Color::rgb(100, 100, 120));
    assert_eq!(s.error_color, Color::RED);
}

#[test]
fn test_search_bar_default_icon() {
    let s = SearchBar::new();
    assert_eq!(s.icon, '🔍');
}

#[test]
fn test_search_bar_default_placeholder() {
    let s = SearchBar::new();
    assert_eq!(s.placeholder, "Search...");
}

#[test]
fn test_search_bar_default_width() {
    let s = SearchBar::new();
    assert_eq!(s.width, 40);
}

#[test]
fn test_search_bar_default_show_hints() {
    let s = SearchBar::new();
    assert!(s.show_hints);
}
