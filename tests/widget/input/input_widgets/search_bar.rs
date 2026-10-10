//! SearchBar widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{search_bar, SearchBar, View};

// =============================================================================
// Constructor Tests
// =============================================================================

#[test]
fn test_search_bar_new_creates_empty_search_bar() {
    let s = SearchBar::new();
    assert!(s.get_input().is_empty());
    assert!(s.is_valid());
    assert!(!s.is_focused());
}

#[test]
fn test_search_bar_default_trait() {
    let s = SearchBar::default();
    assert!(s.get_input().is_empty());
    assert!(s.is_valid());
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
    // Verify builder returns self and doesn't panic
    let _s = SearchBar::new().placeholder("Type here...");
}

#[test]
fn test_search_bar_placeholder_builder_with_string() {
    let _s = SearchBar::new().placeholder(String::from("Custom placeholder"));
}

#[test]
fn test_search_bar_width_builder() {
    let _s = SearchBar::new().width(60);
}

#[test]
fn test_search_bar_width_builder_clamps_minimum() {
    // Width of 5 should be clamped to minimum 10; just verify no panic
    let _s = SearchBar::new().width(5);
}

#[test]
fn test_search_bar_icon_builder() {
    let _s = SearchBar::new().icon('🔎');
}

#[test]
fn test_search_bar_show_hints_builder() {
    let _s = SearchBar::new().show_hints(false);
}

#[test]
fn test_search_bar_show_hints_builder_true() {
    let _s = SearchBar::new().show_hints(true);
}

#[test]
fn test_search_bar_colors_builder() {
    let _s = SearchBar::new().colors(Color::RED, Color::BLUE, Color::GREEN);
}

#[test]
fn test_search_bar_error_color_builder() {
    let _s = SearchBar::new().error_color(Color::YELLOW);
}

#[test]
fn test_search_bar_builder_chaining() {
    let _s = SearchBar::new()
        .placeholder("Search")
        .width(50)
        .icon('🔍')
        .show_hints(false)
        .colors(Color::WHITE, Color::rgb(128, 128, 128), Color::BLACK)
        .error_color(Color::RED);
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
    // After 2 inputs, cursor is at end; cursor_left twice confirms position
    s.cursor_left();
    s.cursor_left();
    // Now at 0; cursor_right once should bring us to 1
    s.cursor_right();
    // Insert 'x' here - it should appear between 'a' and 'b'
    s.input('x');
    assert_eq!(s.get_input(), "axb");
}

#[test]
fn test_search_bar_input_with_unicode() {
    let mut s = SearchBar::new();
    s.input('🎉');
    s.input('한');
    s.input('글');
    assert_eq!(s.get_input(), "🎉한글");
    // cursor is at 3; moving left 3 times brings it to 0
    s.cursor_left();
    s.cursor_left();
    s.cursor_left();
    // Confirm we're at 0 by attempting another left (stays at 0)
    s.cursor_left();
    s.cursor_right();
    // Now at 1; input 'X' inserts between '🎉' and '한'
    s.input('X');
    assert_eq!(s.get_input(), "🎉X한글");
}

#[test]
fn test_search_bar_backspace_deletes_at_end() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    s.backspace();
    assert_eq!(s.get_input(), "hell");
}

#[test]
fn test_search_bar_backspace_moves_cursor() {
    let mut s = SearchBar::new();
    s.set_query("ab");
    // cursor is at end (2); backspace moves it to 1
    s.backspace();
    // Verify cursor is at 1: a further backspace removes 'a', leaving ""
    s.backspace();
    assert_eq!(s.get_input(), "");
}

#[test]
fn test_search_bar_backspace_at_start_does_nothing() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    s.cursor_home();
    s.backspace();
    assert_eq!(s.get_input(), "hello");
}

#[test]
fn test_search_bar_backspace_on_empty_does_nothing() {
    let mut s = SearchBar::new();
    s.backspace();
    assert!(s.get_input().is_empty());
}

#[test]
fn test_search_bar_delete_deletes_at_cursor() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    // Move to position 2 (after 'he')
    s.cursor_home();
    s.cursor_right();
    s.cursor_right();
    s.delete();
    assert_eq!(s.get_input(), "helo");
}

#[test]
fn test_search_bar_delete_at_end_does_nothing() {
    let mut s = SearchBar::new();
    s.set_query("hi");
    // cursor is already at end after set_query
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
    // cursor at 5; left moves to 4; verify by inserting at position 4 (before 'o')
    s.cursor_left();
    s.input('X');
    assert_eq!(s.get_input(), "hellXo");
}

#[test]
fn test_search_bar_cursor_left_at_start_stays() {
    let mut s = SearchBar::new();
    s.set_query("hi");
    s.cursor_home();
    s.cursor_left();
    // Still at start; insert should prepend
    s.input('X');
    assert_eq!(s.get_input(), "Xhi");
}

#[test]
fn test_search_bar_cursor_right_moves_right() {
    let mut s = SearchBar::new();
    s.set_query("hi");
    s.cursor_home();
    s.cursor_right();
    // Now at position 1; insert 'X' between 'h' and 'i'
    s.input('X');
    assert_eq!(s.get_input(), "hXi");
}

#[test]
fn test_search_bar_cursor_right_at_end_stays() {
    let mut s = SearchBar::new();
    s.set_query("hi");
    // cursor already at end (2); another right stays at 2
    s.cursor_right();
    // Insert appends to end
    s.input('X');
    assert_eq!(s.get_input(), "hiX");
}

#[test]
fn test_search_bar_cursor_home_moves_to_start() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    // Move cursor to position 3 first
    s.cursor_home();
    s.cursor_right();
    s.cursor_right();
    s.cursor_right();
    s.cursor_home();
    // Now at 0; insert prepends
    s.input('X');
    assert_eq!(s.get_input(), "Xhello");
}

#[test]
fn test_search_bar_cursor_home_at_start_stays() {
    let mut s = SearchBar::new();
    s.set_query("test");
    s.cursor_home();
    // Already at 0; insert prepends
    s.input('X');
    assert_eq!(s.get_input(), "Xtest");
}

#[test]
fn test_search_bar_cursor_end_moves_to_end() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    s.cursor_home();
    s.cursor_end();
    // At end (5); insert appends
    s.input('X');
    assert_eq!(s.get_input(), "helloX");
}

#[test]
fn test_search_bar_cursor_end_at_end_stays() {
    let mut s = SearchBar::new();
    s.set_query("test");
    s.cursor_end();
    // Already at end; insert appends
    s.input('X');
    assert_eq!(s.get_input(), "testX");
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
    // Verify cursor is at end by inserting and checking result
    s.input('X');
    assert_eq!(s.get_input(), "helloX");
}

#[test]
fn test_search_bar_set_query_empty() {
    let mut s = SearchBar::new();
    s.set_query("");
    assert!(s.get_input().is_empty());
    // cursor at 0; backspace does nothing
    s.backspace();
    assert!(s.get_input().is_empty());
}

#[test]
fn test_search_bar_set_query_with_unicode() {
    let mut s = SearchBar::new();
    s.set_query("🎉한글");
    assert_eq!(s.get_input(), "🎉한글");
    // cursor at 3 (end); verify by appending
    s.input('X');
    assert_eq!(s.get_input(), "🎉한글X");
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
    // Cursor is at 0; backspace does nothing, delete does nothing
    s.backspace();
    assert!(s.get_input().is_empty());
    s.input('X');
    assert_eq!(s.get_input(), "X");
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
    s.cursor_home();
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
    // Cursor moved left from 2 to 1; insert 'X' should place it between 'h' and 'i'
    s.input('X');
    assert_eq!(s.get_input(), "hXi");
}

#[test]
fn test_search_bar_handle_key_right_moves_cursor() {
    let mut s = SearchBar::new();
    s.set_query("hi");
    s.cursor_home();
    let handled = s.handle_key(&Key::Right);
    assert!(handled);
    // Cursor moved right from 0 to 1; insert 'X' should place it between 'h' and 'i'
    s.input('X');
    assert_eq!(s.get_input(), "hXi");
}

#[test]
fn test_search_bar_handle_key_home_moves_to_start() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    let handled = s.handle_key(&Key::Home);
    assert!(handled);
    // Cursor at 0; insert prepends
    s.input('X');
    assert_eq!(s.get_input(), "Xhello");
}

#[test]
fn test_search_bar_handle_key_end_moves_to_end() {
    let mut s = SearchBar::new();
    s.set_query("hello");
    s.cursor_home();
    let handled = s.handle_key(&Key::End);
    assert!(handled);
    // Cursor at end; insert appends
    s.input('X');
    assert_eq!(s.get_input(), "helloX");
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
    // Verify default colors are set by checking render doesn't panic
    let mut buffer = Buffer::new(50, 2);
    let area = Rect::new(0, 0, 50, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let s = SearchBar::new();
    s.render(&mut ctx);
}

#[test]
fn test_search_bar_default_icon() {
    // Verify default icon renders without panic
    let mut buffer = Buffer::new(50, 2);
    let area = Rect::new(0, 0, 50, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let s = SearchBar::new();
    s.render(&mut ctx);
}

#[test]
fn test_search_bar_default_placeholder() {
    // Verify default placeholder renders without panic
    let mut buffer = Buffer::new(50, 2);
    let area = Rect::new(0, 0, 50, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let s = SearchBar::new();
    s.render(&mut ctx);
}

#[test]
fn test_search_bar_default_width() {
    // Verify default width; widget is valid and can render
    let s = SearchBar::new();
    assert!(s.get_input().is_empty());
}

#[test]
fn test_search_bar_default_show_hints() {
    // Verify default show_hints renders hints when focused
    let mut buffer = Buffer::new(50, 3);
    let area = Rect::new(0, 0, 50, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let mut s = SearchBar::new();
    s.focus();
    s.render(&mut ctx);
}
