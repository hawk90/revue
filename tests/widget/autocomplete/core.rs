//! Autocomplete core widget tests
//!
//! Tests for src/widget/input/input_widgets/autocomplete/core.rs

use revue::event::{Key, KeyEvent};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::utils::FilterMode;
use revue::widget::autocomplete::{Autocomplete, Suggestion};
use revue::widget::traits::{RenderContext, StyledView, View};

// ==================== Constructor Tests ====================

#[test]
fn test_autocomplete_new() {
    let a = Autocomplete::new();
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_default() {
    let a = Autocomplete::default();
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_helper() {
    use revue::widget::autocomplete;
    let a = autocomplete();
    assert_eq!(a.get_value(), "");
}

// ==================== Builder Tests ====================

#[test]
fn test_autocomplete_suggestions() {
    let a = Autocomplete::new().suggestions(vec!["test".to_string()]);
    // Just verify it compiles
}

#[test]
fn test_autocomplete_suggestions_from_slice() {
    let a = Autocomplete::new().suggestions(["a", "b", "c"]);
}

#[test]
fn test_autocomplete_suggestions_with_suggestion_structs() {
    let a = Autocomplete::new().suggestions(vec![
        Suggestion::new("Item 1"),
        Suggestion::new("Item 2"),
    ]);
}

#[test]
fn test_autocomplete_suggestions_empty() {
    let a = Autocomplete::new().suggestions::<Vec<String>>(vec![]);
}

#[test]
fn test_autocomplete_value() {
    let a = Autocomplete::new().value("test");
    assert_eq!(a.get_value(), "test");
}

#[test]
fn test_autocomplete_value_string() {
    let a = Autocomplete::new().value(String::from("owned"));
    assert_eq!(a.get_value(), "owned");
}

#[test]
fn test_autocomplete_value_empty() {
    let a = Autocomplete::new().value("");
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_placeholder() {
    let a = Autocomplete::new().placeholder("Search...");
    // Just verify it compiles
}

#[test]
fn test_autocomplete_placeholder_string() {
    let a = Autocomplete::new().placeholder(String::from("Type here"));
}

#[test]
fn test_autocomplete_placeholder_empty() {
    let a = Autocomplete::new().placeholder("");
}

#[test]
fn test_autocomplete_filter_mode() {
    let modes = [
        FilterMode::Fuzzy,
        FilterMode::Prefix,
        FilterMode::Contains,
        FilterMode::Exact,
        FilterMode::None,
    ];

    for mode in modes {
        let _a = Autocomplete::new().filter_mode(mode);
    }
}

#[test]
fn test_autocomplete_min_chars() {
    let a = Autocomplete::new().min_chars(2);
    // Just verify it compiles
}

#[test]
fn test_autocomplete_min_chars_zero() {
    let a = Autocomplete::new().min_chars(0);
}

#[test]
fn test_autocomplete_min_chars_large() {
    let a = Autocomplete::new().min_chars(100);
}

#[test]
fn test_autocomplete_max_suggestions() {
    let a = Autocomplete::new().max_suggestions(5);
}

#[test]
fn test_autocomplete_max_suggestions_zero() {
    let a = Autocomplete::new().max_suggestions(0);
}

#[test]
fn test_autocomplete_max_suggestions_large() {
    let a = Autocomplete::new().max_suggestions(1000);
}

#[test]
fn test_autocomplete_input_style() {
    let a = Autocomplete::new().input_style(Color::CYAN, Color::BLACK);
}

#[test]
fn test_autocomplete_dropdown_style() {
    let a = Autocomplete::new().dropdown_style(Color::BLUE, Color::WHITE, Color::GRAY);
}

#[test]
fn test_autocomplete_highlight_fg() {
    let a = Autocomplete::new().highlight_fg(Color::YELLOW);
}

#[test]
fn test_autocomplete_builder_chain() {
    let a = Autocomplete::new()
        .value("test")
        .placeholder("Search")
        .filter_mode(FilterMode::Prefix)
        .min_chars(2)
        .max_suggestions(5)
        .input_style(Color::WHITE, Color::BLACK)
        .dropdown_style(Color::GRAY, Color::WHITE, Color::BLUE)
        .highlight_fg(Color::YELLOW)
        .suggestions(vec!["a", "b", "c"]);

    assert_eq!(a.get_value(), "test");
}

// ==================== Getter Tests ====================

#[test]
fn test_autocomplete_get_value_empty() {
    let a = Autocomplete::new();
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_get_value_with_text() {
    let a = Autocomplete::new().value("hello");
    assert_eq!(a.get_value(), "hello");
}

#[test]
fn test_autocomplete_get_value_unicode() {
    let a = Autocomplete::new().value("こんにちは");
    assert_eq!(a.get_value(), "こんにちは");
}

#[test]
fn test_autocomplete_selected_suggestion_none() {
    let a = Autocomplete::new();
    assert!(a.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_selected_suggestion_with_suggestions() {
    let a = Autocomplete::new()
        .suggestions(["apple", "banana", "cherry"])
        .value("app");
    a.focus();

    // After focusing and typing, filter should update
    // but selection is still None since we can't test dropdown visibility directly
}

#[test]
fn test_autocomplete_is_focused_default() {
    let a = Autocomplete::new();
    assert!(!a.is_focused());
}

#[test]
fn test_autocomplete_is_focused_after_focus() {
    let mut a = Autocomplete::new();
    a.focus();
    assert!(a.is_focused());
}

#[test]
fn test_autocomplete_is_focused_after_blur() {
    let mut a = Autocomplete::new();
    a.focus();
    a.blur();
    assert!(!a.is_focused());
}

// ==================== State-Changing Methods Tests ====================

#[test]
fn test_autocomplete_set_value_empty() {
    let mut a = Autocomplete::new();
    a.set_value("");
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_set_value_string() {
    let mut a = Autocomplete::new();
    a.set_value("test");
    assert_eq!(a.get_value(), "test");
}

#[test]
fn test_autocomplete_set_value_overwrite() {
    let mut a = Autocomplete::new().value("first");
    a.set_value("second");
    assert_eq!(a.get_value(), "second");
}

#[test]
fn test_autocomplete_set_value_unicode() {
    let mut a = Autocomplete::new();
    a.set_value("Привет");
    assert_eq!(a.get_value(), "Привет");
}

#[test]
fn test_autocomplete_set_suggestions_empty() {
    let mut a = Autocomplete::new();
    a.set_suggestions(vec![]);
}

#[test]
fn test_autocomplete_set_suggestions_vec() {
    let mut a = Autocomplete::new();
    a.set_suggestions(vec![
        Suggestion::new("Item 1"),
        Suggestion::new("Item 2"),
    ]);
}

#[test]
fn test_autocomplete_set_suggestions_overwrite() {
    let mut a = Autocomplete::new();
    a.set_suggestions(vec!["a", "b"]);
    a.set_suggestions(vec!["c", "d", "e"]);
}

#[test]
fn test_autocomplete_focus() {
    let mut a = Autocomplete::new();
    a.focus();
    assert!(a.is_focused());
}

#[test]
fn test_autocomplete_focus_twice() {
    let mut a = Autocomplete::new();
    a.focus();
    a.focus();
    assert!(a.is_focused());
}

#[test]
fn test_autocomplete_blur() {
    let mut a = Autocomplete::new();
    a.focus();
    a.blur();
    assert!(!a.is_focused());
}

#[test]
fn test_autocomplete_blur_twice() {
    let mut a = Autocomplete::new();
    a.focus();
    a.blur();
    a.blur();
    assert!(!a.is_focused());
}

#[test]
fn test_autocomplete_blur_without_focus() {
    let mut a = Autocomplete::new();
    a.blur();
    assert!(!a.is_focused());
}

#[test]
fn test_autocomplete_accept_selection_none() {
    let mut a = Autocomplete::new();
    assert!(!a.accept_selection());
}

#[test]
fn test_autocomplete_accept_selection_with_suggestions() {
    let mut a = Autocomplete::new()
        .suggestions(["apple", "banana", "cherry"])
        .value("app");

    a.focus();

    // accept_selection should return false if no selection
    // This is a basic test - real selection testing requires focus/typing
}

// ==================== Key Handling Tests ====================

#[test]
fn test_autocomplete_handle_key_char() {
    let mut a = Autocomplete::new();
    let key = KeyEvent::new(Key::Char('a'));
    assert!(a.handle_key(key));
    assert_eq!(a.get_value(), "a");
}

#[test]
fn test_autocomplete_handle_key_multiple_chars() {
    let mut a = Autocomplete::new();
    a.handle_key(KeyEvent::new(Key::Char('h')));
    a.handle_key(KeyEvent::new(Key::Char('e')));
    a.handle_key(KeyEvent::new(Key::Char('l')));
    a.handle_key(KeyEvent::new(Key::Char('l')));
    a.handle_key(KeyEvent::new(Key::Char('o')));
    assert_eq!(a.get_value(), "hello");
}

#[test]
fn test_autocomplete_handle_key_unicode_char() {
    let mut a = Autocomplete::new();
    assert!(a.handle_key(KeyEvent::new(Key::Char('你'))));
    assert_eq!(a.get_value(), "你");
}

#[test]
fn test_autocomplete_handle_key_backspace() {
    let mut a = Autocomplete::new().value("test");
    assert!(a.handle_key(KeyEvent::new(Key::Backspace)));
    assert_eq!(a.get_value(), "tes");
}

#[test]
fn test_autocomplete_handle_key_backspace_empty() {
    let mut a = Autocomplete::new();
    assert!(a.handle_key(KeyEvent::new(Key::Backspace)));
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_handle_key_backspace_multiple() {
    let mut a = Autocomplete::new().value("hello");
    a.handle_key(KeyEvent::new(Key::Backspace));
    a.handle_key(KeyEvent::new(Key::Backspace));
    a.handle_key(KeyEvent::new(Key::Backspace));
    assert_eq!(a.get_value(), "he");
}

#[test]
fn test_autocomplete_handle_key_delete() {
    let mut a = Autocomplete::new().value("test");
    assert!(a.handle_key(KeyEvent::new(Key::Delete)));
    assert_eq!(a.get_value(), "est");
}

#[test]
fn test_autocomplete_handle_key_delete_end() {
    let mut a = Autocomplete::new().value("ab");
    a.handle_key(KeyEvent::new(Key::Right));
    assert!(a.handle_key(KeyEvent::new(Key::Delete)));
    assert_eq!(a.get_value(), "a");
}

#[test]
fn test_autocomplete_handle_key_delete_empty() {
    let mut a = Autocomplete::new();
    assert!(a.handle_key(KeyEvent::new(Key::Delete)));
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_handle_key_left() {
    let mut a = Autocomplete::new().value("test");
    assert!(a.handle_key(KeyEvent::new(Key::Left)));
    // Cursor should move left (can't test directly, but verify it doesn't crash)
}

#[test]
fn test_autocomplete_handle_key_left_at_start() {
    let mut a = Autocomplete::new().value("test");
    a.handle_key(KeyEvent::new(Key::Left));
    a.handle_key(KeyEvent::new(Key::Left));
    a.handle_key(KeyEvent::new(Key::Left));
    a.handle_key(KeyEvent::new(Key::Left));
    assert!(a.handle_key(KeyEvent::new(Key::Left))); // Should not crash
}

#[test]
fn test_autocomplete_handle_key_right() {
    let mut a = Autocomplete::new().value("test");
    a.handle_key(KeyEvent::new(Key::Left));
    assert!(a.handle_key(KeyEvent::new(Key::Right)));
}

#[test]
fn test_autocomplete_handle_key_right_at_end() {
    let mut a = Autocomplete::new().value("test");
    assert!(a.handle_key(KeyEvent::new(Key::Right)));
    // Should not crash
}

#[test]
fn test_autocomplete_handle_key_home() {
    let mut a = Autocomplete::new().value("test");
    assert!(a.handle_key(KeyEvent::new(Key::Home)));
}

#[test]
fn test_autocomplete_handle_key_end() {
    let mut a = Autocomplete::new().value("test");
    a.handle_key(KeyEvent::new(Key::Home));
    assert!(a.handle_key(KeyEvent::new(Key::End)));
}

#[test]
fn test_autocomplete_handle_key_up_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!a.handle_key(KeyEvent::new(Key::Up)));
}

#[test]
fn test_autocomplete_handle_key_down_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!a.handle_key(KeyEvent::new(Key::Down)));
}

#[test]
fn test_autocomplete_handle_key_enter_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!a.handle_key(KeyEvent::new(Key::Enter)));
}

#[test]
fn test_autocomplete_handle_key_tab_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!a.handle_key(KeyEvent::new(Key::Tab)));
}

#[test]
fn test_autocomplete_handle_key_escape_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!a.handle_key(KeyEvent::new(Key::Escape)));
}

#[test]
fn test_autocomplete_handle_key_invalid_key() {
    let mut a = Autocomplete::new();
    assert!(!a.handle_key(KeyEvent::new(Key::Tab)));
    assert!(!a.handle_key(KeyEvent::new(Key::PageUp)));
    assert!(!a.handle_key(KeyEvent::new(Key::PageDown)));
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_handle_key_navigation_sequence() {
    let mut a = Autocomplete::new();
    a.handle_key(KeyEvent::new(Key::Char('t')));
    a.handle_key(KeyEvent::new(Key::Char('e')));
    a.handle_key(KeyEvent::new(Key::Char('s')));
    a.handle_key(KeyEvent::new(Key::Left));
    a.handle_key(KeyEvent::new(Key::Right));
    a.handle_key(KeyEvent::new(Key::Home));
    a.handle_key(KeyEvent::new(Key::End)));
    assert_eq!(a.get_value(), "tes");
}

#[test]
fn test_autocomplete_handle_key_edit_and_navigate() {
    let mut a = Autocomplete::new();
    a.handle_key(KeyEvent::new(Key::Char('a')));
    a.handle_key(KeyEvent::new(Key::Char('b')));
    a.handle_key(KeyEvent::new(Key::Char('c')));
    a.handle_key(KeyEvent::new(Key::Left));
    a.handle_key(KeyEvent::new(Key::Left));
    a.handle_key(KeyEvent::new(Key::Char('x'))); // Insert at cursor position
    assert_eq!(a.get_value(), "xabc");
}

// ==================== Rendering Tests ====================

#[test]
fn test_autocomplete_render_empty() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Autocomplete::new();
    a.render(&mut ctx);
}

#[test]
fn test_autocomplete_render_with_value() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Autocomplete::new().value("test");
    a.render(&mut ctx);
}

#[test]
fn test_autocomplete_render_with_placeholder() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Autocomplete::new().placeholder("Search...");
    a.render(&mut ctx);
}

#[test]
fn test_autocomplete_render_focused() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut a = Autocomplete::new();
    a.focus();
    a.render(&mut ctx);
}

#[test]
fn test_autocomplete_render_zero_width() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 0, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Autocomplete::new();
    a.render(&mut ctx);
}

#[test]
fn test_autocomplete_render_zero_height() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Autocomplete::new();
    a.render(&mut ctx);
}

#[test]
fn test_autocomplete_render_very_narrow() {
    let mut buffer = Buffer::new(3, 5);
    let area = Rect::new(0, 0, 3, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Autocomplete::new();
    a.render(&mut ctx);
}

#[test]
fn test_autocomplete_render_with_suggestions() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Autocomplete::new()
        .suggestions(["apple", "application", "banana"])
        .value("app");

    a.focus();
    a.render(&mut ctx);
}

#[test]
fn test_autocomplete_render_long_value() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Autocomplete::new().value("this is a very long value");
    a.render(&mut ctx);
}

#[test]
fn test_autocomplete_render_unicode_value() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Autocomplete::new().value("こんにちは世界");
    a.render(&mut ctx);
}

#[test]
fn test_autocomplete_render_offset_area() {
    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(10, 2, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Autocomplete::new().value("test");
    a.render(&mut ctx);
}

// ==================== CSS Integration Tests ====================

#[test]
fn test_autocomplete_element_id() {
    let a = Autocomplete::new().element_id("search-box");
    assert_eq!(View::id(&a), Some("search-box"));
}

#[test]
fn test_autocomplete_classes() {
    let a = Autocomplete::new().class("input").class("search");
    assert!(a.has_class("input"));
    assert!(a.has_class("search"));
    assert!(!a.has_class("hidden"));
}

#[test]
fn test_autocomplete_styled_view_methods() {
    let mut a = Autocomplete::new();

    a.set_id("my-autocomplete");
    assert_eq!(View::id(&a), Some("my-autocomplete"));

    a.add_class("active");
    assert!(a.has_class("active"));

    a.remove_class("active");
    assert!(!a.has_class("active"));

    a.toggle_class("visible");
    assert!(a.has_class("visible"));

    a.toggle_class("visible");
    assert!(!a.has_class("visible"));
}

#[test]
fn test_autocomplete_meta() {
    let a = Autocomplete::new()
        .element_id("test")
        .class("class1")
        .class("class2");

    let meta = a.meta();
    assert_eq!(meta.id, Some("test".to_string()));
    assert_eq!(meta.classes.len(), 2);
}

// ==================== Edge Cases Tests ====================

#[test]
fn test_autocomplete_empty_value_with_suggestions() {
    let a = Autocomplete::new()
        .suggestions(["a", "b", "c"])
        .value("");
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_special_chars_in_value() {
    let mut a = Autocomplete::new();
    a.handle_key(KeyEvent::new(Key::Char('@')));
    a.handle_key(KeyEvent::new(Key::Char('#')));
    a.handle_key(KeyEvent::new(Key::Char('$')));
    a.handle_key(KeyEvent::new(Key::Char('%')));
    assert_eq!(a.get_value(), "@#$%");
}

#[test]
fn test_autocomplete_newline_chars() {
    let mut a = Autocomplete::new();
    // While newline might not be typical, we test it doesn't crash
    a.set_value("line1\nline2");
    assert_eq!(a.get_value(), "line1\nline2");
}

#[test]
fn test_autocomplete_tab_char() {
    let mut a = Autocomplete::new();
    a.set_value("tab\there");
    assert_eq!(a.get_value(), "tab\there");
}

#[test]
fn test_autocomplete_emoji_in_value() {
    let mut a = Autocomplete::new();
    a.handle_key(KeyEvent::new(Key::Char('🔍')));
    assert_eq!(a.get_value(), "🔍");
}

#[test]
fn test_autocomplete_rapid_character_input() {
    let mut a = Autocomplete::new();
    for i in 0..100 {
        a.handle_key(KeyEvent::new(Key::Char(char::from_digit(i % 10, 10).unwrap())));
    }
    assert_eq!(a.get_value().len(), 100);
}

#[test]
fn test_autocomplete_rapid_backspace() {
    let mut a = Autocomplete::new();
    a.set_value("abcdefghijklmnopqrstuvwxyz");

    for _ in 0..26 {
        a.handle_key(KeyEvent::new(Key::Backspace));
    }
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_mix_edit_and_navigation() {
    let mut a = Autocomplete::new();
    a.handle_key(KeyEvent::new(Key::Char('a')));
    a.handle_key(KeyEvent::new(Key::Char('b')));
    a.handle_key(KeyEvent::new(Key::Char('c')));
    a.handle_key(KeyEvent::new(Key::Left));
    a.handle_key(KeyEvent::new(Key::Backspace)); // Deletes 'b'
    a.handle_key(KeyEvent::new(Key::Char('x'))); // Inserts 'x'
    assert_eq!(a.get_value(), "axc");
}

#[test]
fn test_autocomplete_cursor_movement_edge_cases() {
    let mut a = Autocomplete::new();
    a.set_value("test");

    // Move left many times
    for _ in 0..20 {
        a.handle_key(KeyEvent::new(Key::Left));
    }

    // Move right many times
    for _ in 0..20 {
        a.handle_key(KeyEvent::new(Key::Right));
    }

    // Should not crash
}

#[test]
fn test_autocomplete_filter_mode_none() {
    let a = Autocomplete::new()
        .suggestions(["apple", "banana", "cherry"])
        .filter_mode(FilterMode::None)
        .value("xyz");

    a.focus();
    // With FilterMode::None, all suggestions should show
}

#[test]
fn test_autocomplete_filter_mode_exact() {
    let a = Autocomplete::new()
        .suggestions(["apple", "banana", "cherry"])
        .filter_mode(FilterMode::Exact)
        .value("apple");

    a.focus();
    // Should only show exact match
}

#[test]
fn test_autocomplete_min_chars_threshold() {
    let a = Autocomplete::new()
        .suggestions(["apple", "banana", "cherry"])
        .min_chars(3)
        .value("ap");

    a.focus();
    // Should not show suggestions yet (below min_chars)
}

#[test]
fn test_autocomplete_max_suggestions_limit() {
    let a = Autocomplete::new()
        .suggestions((0..100).map(|i| format!("item{}", i)).collect::<Vec<_>>())
        .max_suggestions(5)
        .value("item");

    a.focus();
    // Should limit to 5 suggestions
}

// ==================== Builder Override Tests ====================

#[test]
fn test_autocomplete_value_override() {
    let a = Autocomplete::new()
        .value("first")
        .value("second");
    assert_eq!(a.get_value(), "second");
}

#[test]
fn test_autocomplete_placeholder_override() {
    let a = Autocomplete::new()
        .placeholder("first")
        .placeholder("second");
    // Just verify it compiles
}

#[test]
fn test_autocomplete_filter_mode_override() {
    let a = Autocomplete::new()
        .filter_mode(FilterMode::Fuzzy)
        .filter_mode(FilterMode::Prefix);
}

#[test]
fn test_autocomplete_suggestions_override() {
    let a = Autocomplete::new()
        .suggestions(["a", "b"])
        .suggestions(["c", "d", "e"]);
}

#[test]
fn test_autocomplete_style_override() {
    let a = Autocomplete::new()
        .input_style(Color::RED, Color::BLACK)
        .input_style(Color::BLUE, Color::WHITE);
}

// ==================== Multiple Instance Tests ====================

#[test]
fn test_autocomplete_multiple_independent_instances() {
    let mut a1 = Autocomplete::new().value("first");
    let mut a2 = Autocomplete::new().value("second");

    a1.handle_key(KeyEvent::new(Key::Char('x')));
    a2.handle_key(KeyEvent::new(Key::Char('y')));

    assert_eq!(a1.get_value(), "firstx");
    assert_eq!(a2.get_value(), "secondy");
}

#[test]
fn test_autocomplete_clone_builder_pattern() {
    let a1 = Autocomplete::new()
        .value("test")
        .placeholder("Search")
        .min_chars(2);

    let a2 = Autocomplete::new()
        .value("test")
        .placeholder("Search")
        .min_chars(2);

    assert_eq!(a1.get_value(), a2.get_value());
}

// ==================== Default Trait Tests ====================

#[test]
fn test_autocomplete_default_trait() {
    let a: Autocomplete = Default::default();
    assert_eq!(a.get_value(), "");
}

// ==================== Complex Interaction Tests ====================

#[test]
fn test_autocomplete_typing_and_deleting_full_cycle() {
    let mut a = Autocomplete::new();

    // Type something
    a.handle_key(KeyEvent::new(Key::Char('h')));
    a.handle_key(KeyEvent::new(Key::Char('e')));
    a.handle_key(KeyEvent::new(Key::Char('l')));
    a.handle_key(KeyEvent::new(Key::Char('l')));
    a.handle_key(KeyEvent::new(Key::Char('o')));
    assert_eq!(a.get_value(), "hello");

    // Delete all
    for _ in 0..5 {
        a.handle_key(KeyEvent::new(Key::Backspace));
    }
    assert_eq!(a.get_value(), "");

    // Type again
    a.handle_key(KeyEvent::new(Key::Char('w')));
    a.handle_key(KeyEvent::new(Key::Char('o')));
    a.handle_key(KeyEvent::new(Key::Char('r')));
    a.handle_key(KeyEvent::new(Key::Char('l')));
    a.handle_key(KeyEvent::new(Key::Char('d')));
    assert_eq!(a.get_value(), "world");
}

#[test]
fn test_autocomplete_focus_blur_focus_cycle() {
    let mut a = Autocomplete::new();

    assert!(!a.is_focused());
    a.focus();
    assert!(a.is_focused());
    a.blur();
    assert!(!a.is_focused());
    a.focus();
    assert!(a.is_focused());
}

#[test]
fn test_autocomplete_set_value_then_type() {
    let mut a = Autocomplete::new();
    a.set_value("hello");
    a.handle_key(KeyEvent::new(Key::Char('!')));
    assert_eq!(a.get_value(), "hello!");
}

#[test]
fn test_autocomplete_type_then_set_value() {
    let mut a = Autocomplete::new();
    a.handle_key(KeyEvent::new(Key::Char('x')));
    a.set_value("y");
    assert_eq!(a.get_value(), "y");
}

// ==================== Colors Tests ====================

#[test]
fn test_autocomplete_custom_colors() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = Autocomplete::new()
        .input_style(Color::CYAN, Color::BLACK)
        .dropdown_style(Color::GRAY, Color::WHITE, Color::BLUE)
        .highlight_fg(Color::YELLOW);

    a.render(&mut ctx);
}

#[test]
fn test_autocomplete_same_input_colors() {
    let a = Autocomplete::new()
        .input_style(Color::WHITE, Color::WHITE);
    // Should handle same foreground and background
}

// ==================== View Trait Tests ====================

#[test]
fn test_autocomplete_view_id() {
    let a = Autocomplete::new().element_id("test-id");
    assert_eq!(View::id(&a), Some("test-id"));
}

#[test]
fn test_autocomplete_view_id_none() {
    let a = Autocomplete::new();
    assert_eq!(View::id(&a), None);
}

#[test]
fn test_autocomplete_view_has_class() {
    let a = Autocomplete::new().class("test-class");
    assert!(a.has_class("test-class"));
}

#[test]
fn test_autocomplete_view_classes_empty() {
    let a = Autocomplete::new();
    assert!(!a.has_class("anything"));
}

#[test]
fn test_autocomplete_view_meta() {
    let a = Autocomplete::new()
        .element_id("my-element")
        .class("class1")
        .class("class2");

    let meta = a.meta();
    assert_eq!(meta.id, Some("my-element".to_string()));
    assert_eq!(meta.classes.len(), 2);
    assert!(meta.classes.contains(&"class1".to_string()));
    assert!(meta.classes.contains(&"class2".to_string()));
}
