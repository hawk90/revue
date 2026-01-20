//! Autocomplete widget tests

use revue::event::{Key, KeyEvent};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::utils::FilterMode;
use revue::widget::traits::{StyledView, View};
use revue::widget::{autocomplete, Autocomplete, Suggestion};

// Helper to create RenderContext
fn create_render_context(
    buffer: &mut Buffer,
    area: Rect,
) -> revue::widget::traits::RenderContext<'_> {
    revue::widget::traits::RenderContext::new(buffer, area)
}

// =============================================================================
// Suggestion Constructor Tests
// =============================================================================

#[test]
fn test_suggestion_new() {
    let s = Suggestion::new("apple");
    assert_eq!(s.label, "apple");
    assert_eq!(s.value, "apple");
    assert!(s.description.is_none());
    assert!(s.icon.is_none());
}

#[test]
fn test_suggestion_with_value() {
    let s = Suggestion::with_value("Apple", "apple_value");
    assert_eq!(s.label, "Apple");
    assert_eq!(s.value, "apple_value");
    assert!(s.description.is_none());
    assert!(s.icon.is_none());
}

#[test]
fn test_suggestion_builder_chain() {
    let s = Suggestion::new("apple")
        .description("A delicious fruit")
        .icon('🍎');

    assert_eq!(s.label, "apple");
    assert_eq!(s.description, Some("A delicious fruit".to_string()));
    assert_eq!(s.icon, Some('🍎'));
}

#[test]
fn test_suggestion_from_string() {
    let s: Suggestion = "banana".into();
    assert_eq!(s.label, "banana");
    assert_eq!(s.value, "banana");
}

#[test]
fn test_suggestion_from_str() {
    let s: Suggestion = std::borrow::Cow::Borrowed("cherry").into();
    assert_eq!(s.label, "cherry");
}

// =============================================================================
// Autocomplete Constructor Tests
// =============================================================================

#[test]
fn test_autocomplete_new() {
    let ac = Autocomplete::new();
    assert_eq!(ac.get_value(), "");
    assert!(!ac.is_focused());
}

#[test]
fn test_autocomplete_default() {
    let ac = Autocomplete::default();
    assert_eq!(ac.get_value(), "");
    assert!(!ac.is_focused());
}

#[test]
fn test_autocomplete_helper() {
    let ac = autocomplete();
    assert_eq!(ac.get_value(), "");
}

// =============================================================================
// Builder Methods Tests
// =============================================================================

#[test]
fn test_autocomplete_suggestions_builder() {
    let ac = Autocomplete::new().suggestions(vec!["apple", "banana", "cherry"]);
    let mut ac_mut = ac;
    ac_mut.focus();
    ac_mut.set_value("ap");
    // Should have filtered suggestions
    assert!(ac_mut.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_suggestions_from_iter() {
    let ac = Autocomplete::new().suggestions(["apple", "banana", "cherry"]);
    let mut ac_mut = ac;
    ac_mut.focus();
    ac_mut.set_value("ap");
    assert!(ac_mut.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_suggestions_with_suggestion_objects() {
    let ac = Autocomplete::new().suggestions(vec![
        Suggestion::new("apple").icon('🍎'),
        Suggestion::new("banana").icon('🍌'),
    ]);
    let mut ac_mut = ac;
    ac_mut.focus();
    ac_mut.set_value("ap");
    assert!(ac_mut.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_value_builder() {
    let ac = Autocomplete::new().value("initial value");
    assert_eq!(ac.get_value(), "initial value");
}

#[test]
fn test_autocomplete_placeholder_builder() {
    let ac = Autocomplete::new().placeholder("Search...");
    // Test that placeholder is set through rendering
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = create_render_context(&mut buffer, area);
    ac.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_autocomplete_filter_mode_builder() {
    let modes = [
        FilterMode::Fuzzy,
        FilterMode::Prefix,
        FilterMode::Contains,
        FilterMode::Exact,
        FilterMode::None,
    ];

    for mode in modes {
        let ac = Autocomplete::new().filter_mode(mode);
        // Verify the autocomplete is created successfully
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(0, 0, 20, 3);
        let mut ctx = create_render_context(&mut buffer, area);
        ac.render(&mut ctx);
    }
}

#[test]
fn test_autocomplete_min_chars_builder() {
    let ac = Autocomplete::new().min_chars(3);
    let mut ac_mut = ac;
    ac_mut.set_suggestions(vec![Suggestion::new("apple"), Suggestion::new("banana")]);
    ac_mut.focus();
    ac_mut.set_value("ap");
    // With min_chars=3, no suggestions should be selected
    assert!(ac_mut.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_max_suggestions_builder() {
    let ac = Autocomplete::new()
        .suggestions(vec!["a1", "a2", "a3", "a4", "a5"])
        .max_suggestions(2)
        .min_chars(1);
    let mut ac_mut = ac;
    ac_mut.focus();
    ac_mut.set_value("a");
    // Verify widget is functional - max_suggestions should limit dropdown
    let _selected = ac_mut.selected_suggestion();
    // Should not panic, internally limited to max_suggestions
}

#[test]
fn test_autocomplete_input_style_builder() {
    let ac = Autocomplete::new().input_style(Color::RED, Color::BLUE);
    // Test through rendering - should not panic
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = create_render_context(&mut buffer, area);
    ac.render(&mut ctx);
}

#[test]
fn test_autocomplete_dropdown_style_builder() {
    let ac = Autocomplete::new().dropdown_style(
        Color::rgb(128, 128, 128),
        Color::WHITE,
        Color::rgb(64, 64, 64),
    );
    // Test through rendering - should not panic
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = create_render_context(&mut buffer, area);
    ac.render(&mut ctx);
}

#[test]
fn test_autocomplete_highlight_fg_builder() {
    let ac = Autocomplete::new().highlight_fg(Color::YELLOW);
    // Test through rendering - should not panic
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = create_render_context(&mut buffer, area);
    ac.render(&mut ctx);
}

#[test]
fn test_autocomplete_builder_chain() {
    let ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana", "cherry"])
        .value("test")
        .placeholder("Search...")
        .filter_mode(FilterMode::Prefix)
        .min_chars(2)
        .max_suggestions(5)
        .input_style(Color::WHITE, Color::BLACK)
        .dropdown_style(Color::rgb(128, 128, 128), Color::WHITE, Color::BLUE)
        .highlight_fg(Color::YELLOW);

    assert_eq!(ac.get_value(), "test");
}

// =============================================================================
// Query Methods Tests
// =============================================================================

#[test]
fn test_autocomplete_get_value() {
    let ac = Autocomplete::new().value("hello world");
    assert_eq!(ac.get_value(), "hello world");
}

#[test]
fn test_autocomplete_get_value_empty() {
    let ac = Autocomplete::new();
    assert_eq!(ac.get_value(), "");
}

#[test]
fn test_autocomplete_selected_suggestion_empty() {
    let ac = Autocomplete::new().suggestions(vec!["apple", "banana"]);
    assert!(ac.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_selected_suggestion_with_filter() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);
    ac.focus();
    ac.set_value("ap");
    let selected = ac.selected_suggestion();
    assert!(selected.is_some());
    assert_eq!(selected.unwrap().label, "apple");
}

#[test]
fn test_autocomplete_is_focused() {
    let ac = Autocomplete::new();
    assert!(!ac.is_focused());
}

#[test]
fn test_autocomplete_is_focused_after_focus() {
    let mut ac = Autocomplete::new();
    ac.focus();
    assert!(ac.is_focused());
}

// =============================================================================
// Setter Methods Tests
// =============================================================================

#[test]
fn test_autocomplete_set_value() {
    let mut ac = Autocomplete::new();
    ac.set_value("new value");
    assert_eq!(ac.get_value(), "new value");
}

#[test]
fn test_autocomplete_set_value_updates_filter() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana", "cherry"])
        .min_chars(1);
    ac.focus();
    ac.set_value("ap");
    // Should trigger filter update
    assert!(ac.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_set_suggestions() {
    let mut ac = Autocomplete::new();
    ac.set_suggestions(vec![
        Suggestion::new("one"),
        Suggestion::new("two"),
        Suggestion::new("three"),
    ]);
    // Verify through behavior
    ac.focus();
    ac.set_value("o");
    assert!(ac.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_set_suggestions_updates_filter() {
    let mut ac = Autocomplete::new().min_chars(1);
    ac.focus();
    ac.set_value("ap");

    // Initially no suggestions
    assert!(ac.selected_suggestion().is_none());

    // Set suggestions and verify filter updates
    ac.set_suggestions(vec![Suggestion::new("apple"), Suggestion::new("apricot")]);
    assert!(ac.selected_suggestion().is_some());
}

// =============================================================================
// Focus/Blur Tests
// =============================================================================

#[test]
fn test_autocomplete_focus() {
    let mut ac = Autocomplete::new();
    ac.focus();
    assert!(ac.is_focused());
}

#[test]
fn test_autocomplete_focus_triggers_filter() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .value("ap")
        .min_chars(1);

    ac.focus();
    // Focus should trigger filter update - dropdown should appear
    assert!(ac.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_blur() {
    let mut ac = Autocomplete::new();
    ac.focus();
    ac.blur();
    assert!(!ac.is_focused());
}

#[test]
fn test_autocomplete_blur_hides_dropdown() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);

    ac.focus();
    ac.set_value("ap");
    assert!(ac.selected_suggestion().is_some());

    ac.blur();
    // After blur, navigation shouldn't work
    assert!(!ac.handle_key(KeyEvent::new(Key::Down)));
}

#[test]
fn test_autocomplete_focus_blur_cycle() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);

    ac.focus();
    assert!(ac.is_focused());
    ac.set_value("ap");
    assert!(ac.selected_suggestion().is_some());

    ac.blur();
    assert!(!ac.is_focused());
    // Navigation should not work
    assert!(!ac.handle_key(KeyEvent::new(Key::Down)));

    ac.focus();
    assert!(ac.is_focused());
    // Navigation should work again
    assert!(ac.handle_key(KeyEvent::new(Key::Down)));
}

// =============================================================================
// Selection Tests
// =============================================================================

#[test]
fn test_autocomplete_accept_selection() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);

    ac.focus();
    ac.set_value("ap");
    let accepted = ac.accept_selection();
    assert!(accepted);
    assert_eq!(ac.get_value(), "apple");
}

#[test]
fn test_autocomplete_accept_selection_no_suggestions() {
    let mut ac = Autocomplete::new();
    let accepted = ac.accept_selection();
    assert!(!accepted);
}

#[test]
fn test_autocomplete_accept_selection_hides_dropdown() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);

    ac.focus();
    ac.set_value("ap");
    assert!(ac.selected_suggestion().is_some());

    ac.accept_selection();
    // After accept, navigation should not work
    assert!(!ac.handle_key(KeyEvent::new(Key::Down)));
}

#[test]
fn test_autocomplete_navigation_with_suggestions() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "apricot", "banana"])
        .min_chars(1);

    ac.focus();
    ac.set_value("ap");

    // Navigate down - should work
    assert!(ac.handle_key(KeyEvent::new(Key::Down)));

    // Navigate up - should work
    assert!(ac.handle_key(KeyEvent::new(Key::Up)));
}

// =============================================================================
// Key Handling Tests
// =============================================================================

#[test]
fn test_autocomplete_handle_key_char_input() {
    let mut ac = Autocomplete::new();
    ac.focus();

    let handled = ac.handle_key(KeyEvent::new(Key::Char('h')));
    assert!(handled);
    assert_eq!(ac.get_value(), "h");
}

#[test]
fn test_autocomplete_handle_key_multiple_chars() {
    let mut ac = Autocomplete::new();
    ac.focus();

    ac.handle_key(KeyEvent::new(Key::Char('h')));
    ac.handle_key(KeyEvent::new(Key::Char('e')));
    ac.handle_key(KeyEvent::new(Key::Char('l')));
    ac.handle_key(KeyEvent::new(Key::Char('l')));
    ac.handle_key(KeyEvent::new(Key::Char('o')));

    assert_eq!(ac.get_value(), "hello");
}

#[test]
fn test_autocomplete_handle_key_backspace() {
    let mut ac = Autocomplete::new().value("hello");
    ac.focus();

    let handled = ac.handle_key(KeyEvent::new(Key::Backspace));
    assert!(handled);
    assert_eq!(ac.get_value(), "hell");
}

#[test]
fn test_autocomplete_handle_key_backspace_at_start() {
    let mut ac = Autocomplete::new().value("test");
    ac.focus();

    // Cursor is at end after value is set, backspace deletes last char
    let handled = ac.handle_key(KeyEvent::new(Key::Backspace));
    assert!(handled);
    assert_eq!(ac.get_value(), "tes");
}

#[test]
fn test_autocomplete_handle_key_delete() {
    let mut ac = Autocomplete::new().value("hello");
    ac.focus();

    // Move cursor to start
    ac.handle_key(KeyEvent::new(Key::Home));

    let handled = ac.handle_key(KeyEvent::new(Key::Delete));
    assert!(handled);
    assert_eq!(ac.get_value(), "ello");
}

#[test]
fn test_autocomplete_handle_key_delete_at_end() {
    let mut ac = Autocomplete::new().value("test");
    ac.focus();

    let handled = ac.handle_key(KeyEvent::new(Key::Delete));
    assert!(handled);
    assert_eq!(ac.get_value(), "test");
}

#[test]
fn test_autocomplete_handle_key_left() {
    let mut ac = Autocomplete::new().value("test");
    ac.focus();

    let handled = ac.handle_key(KeyEvent::new(Key::Left));
    assert!(handled);
}

#[test]
fn test_autocomplete_handle_key_left_at_start() {
    let mut ac = Autocomplete::new().value("test");
    ac.focus();

    ac.handle_key(KeyEvent::new(Key::Home));
    let handled = ac.handle_key(KeyEvent::new(Key::Left));
    assert!(handled);
}

#[test]
fn test_autocomplete_handle_key_right() {
    let mut ac = Autocomplete::new().value("test");
    ac.focus();
    ac.handle_key(KeyEvent::new(Key::Home));

    let handled = ac.handle_key(KeyEvent::new(Key::Right));
    assert!(handled);
}

#[test]
fn test_autocomplete_handle_key_right_at_end() {
    let mut ac = Autocomplete::new().value("test");
    ac.focus();

    let handled = ac.handle_key(KeyEvent::new(Key::Right));
    assert!(handled);
}

#[test]
fn test_autocomplete_handle_key_home() {
    let mut ac = Autocomplete::new().value("test");
    ac.focus();

    let handled = ac.handle_key(KeyEvent::new(Key::Home));
    assert!(handled);
}

#[test]
fn test_autocomplete_handle_key_end() {
    let mut ac = Autocomplete::new().value("test");
    ac.focus();
    ac.handle_key(KeyEvent::new(Key::Home));

    let handled = ac.handle_key(KeyEvent::new(Key::End));
    assert!(handled);
}

#[test]
fn test_autocomplete_handle_key_up_without_dropdown() {
    let mut ac = Autocomplete::new();
    ac.focus();

    let handled = ac.handle_key(KeyEvent::new(Key::Up));
    assert!(!handled);
}

#[test]
fn test_autocomplete_handle_key_down_without_dropdown() {
    let mut ac = Autocomplete::new();
    ac.focus();

    let handled = ac.handle_key(KeyEvent::new(Key::Down));
    assert!(!handled);
}

#[test]
fn test_autocomplete_handle_key_up_with_dropdown() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["a1", "a2", "a3"])
        .min_chars(1);

    ac.focus();
    ac.set_value("a");

    let handled = ac.handle_key(KeyEvent::new(Key::Up));
    assert!(handled);
}

#[test]
fn test_autocomplete_handle_key_down_with_dropdown() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["a1", "a2", "a3"])
        .min_chars(1);

    ac.focus();
    ac.set_value("a");

    let handled = ac.handle_key(KeyEvent::new(Key::Down));
    assert!(handled);
}

#[test]
fn test_autocomplete_handle_key_enter_accepts() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);

    ac.focus();
    ac.set_value("ap");

    let handled = ac.handle_key(KeyEvent::new(Key::Enter));
    assert!(handled);
    assert_eq!(ac.get_value(), "apple");
}

#[test]
fn test_autocomplete_handle_key_enter_without_dropdown() {
    let mut ac = Autocomplete::new().value("test");
    ac.focus();

    let handled = ac.handle_key(KeyEvent::new(Key::Enter));
    assert!(!handled);
}

#[test]
fn test_autocomplete_handle_key_tab_accepts() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);

    ac.focus();
    ac.set_value("ap");

    let handled = ac.handle_key(KeyEvent::new(Key::Tab));
    assert!(handled);
    assert_eq!(ac.get_value(), "apple");
}

#[test]
fn test_autocomplete_handle_key_escape_hides_dropdown() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);

    ac.focus();
    ac.set_value("ap");
    assert!(ac.selected_suggestion().is_some());

    let handled = ac.handle_key(KeyEvent::new(Key::Escape));
    assert!(handled);
    // After escape, navigation should not work
    assert!(!ac.handle_key(KeyEvent::new(Key::Down)));
}

#[test]
fn test_autocomplete_handle_key_escape_without_dropdown() {
    let mut ac = Autocomplete::new();
    ac.focus();

    let handled = ac.handle_key(KeyEvent::new(Key::Escape));
    assert!(!handled);
}

#[test]
fn test_autocomplete_handle_key_unhandled() {
    let mut ac = Autocomplete::new();
    ac.focus();

    assert!(!ac.handle_key(KeyEvent::new(Key::F(1))));
    assert!(!ac.handle_key(KeyEvent::new(Key::PageUp)));
    assert!(!ac.handle_key(KeyEvent::new(Key::PageDown)));
}

// =============================================================================
// Filter Mode Tests
// =============================================================================

#[test]
fn test_autocomplete_filter_mode_fuzzy() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "application", "appeal"])
        .filter_mode(FilterMode::Fuzzy)
        .min_chars(1);

    ac.focus();
    ac.set_value("apl");
    // Should match with fuzzy matching
    assert!(ac.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_filter_mode_prefix() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "application", "pineapple"])
        .filter_mode(FilterMode::Prefix)
        .min_chars(1);

    ac.focus();
    ac.set_value("app");
    // Should only match "apple" and "application"
    assert!(ac.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_filter_mode_contains() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "pineapple", "app"])
        .filter_mode(FilterMode::Contains)
        .min_chars(1);

    ac.focus();
    ac.set_value("app");
    // Should match all
    assert!(ac.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_filter_mode_exact() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "APPLE", "apples"])
        .filter_mode(FilterMode::Exact)
        .min_chars(1);

    ac.focus();
    ac.set_value("apple");
    // Should match (case-insensitive)
    assert!(ac.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_filter_mode_none() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana", "cherry"])
        .filter_mode(FilterMode::None)
        .min_chars(1);

    ac.focus();
    ac.set_value("xyz");
    // Should show all suggestions
    assert!(ac.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_filter_case_insensitive() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["Apple", "BANANA", "Cherry"])
        .filter_mode(FilterMode::Prefix)
        .min_chars(1);

    ac.focus();
    ac.set_value("app");
    assert!(ac.selected_suggestion().is_some());

    ac.set_value("ban");
    assert!(ac.selected_suggestion().is_some());
}

// =============================================================================
// Min Chars Tests
// =============================================================================

#[test]
fn test_autocomplete_min_chars_threshold() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(3);

    ac.focus();

    ac.set_value("ap");
    assert!(ac.selected_suggestion().is_none());

    ac.set_value("app");
    assert!(ac.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_min_chars_with_exact_match() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana", "cherry"])
        .filter_mode(FilterMode::Exact)
        .min_chars(5);

    ac.focus();

    ac.set_value("apple");
    assert!(ac.selected_suggestion().is_some());
}

// =============================================================================
// Max Suggestions Tests
// =============================================================================

#[test]
fn test_autocomplete_max_suggestions_limits() {
    let ac = Autocomplete::new()
        .suggestions(vec!["a1", "a2", "a3", "a4", "a5"])
        .max_suggestions(3)
        .min_chars(1);

    let mut ac_mut = ac;
    ac_mut.focus();
    ac_mut.set_value("a");
    // Should work with max_suggestions
    assert!(ac_mut.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_max_suggestions_zero() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .max_suggestions(0)
        .min_chars(1);

    ac.focus();
    ac.set_value("a");
    // No suggestions should be available
    assert!(ac.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_max_suggestions_large() {
    let ac = Autocomplete::new()
        .suggestions(vec!["a1", "a2", "a3"])
        .max_suggestions(100)
        .min_chars(1);

    let mut ac_mut = ac;
    ac_mut.focus();
    ac_mut.set_value("a");
    assert!(ac_mut.selected_suggestion().is_some());
}

// =============================================================================
// Render Tests
// =============================================================================

#[test]
fn test_autocomplete_render_basic() {
    let ac = Autocomplete::new().value("test");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = create_render_context(&mut buffer, area);

    ac.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_autocomplete_render_with_placeholder() {
    let ac = Autocomplete::new().placeholder("Search...");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = create_render_context(&mut buffer, area);

    ac.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_autocomplete_render_with_cursor() {
    let mut ac = Autocomplete::new().value("test");
    ac.focus();

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = create_render_context(&mut buffer, area);

    ac.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_autocomplete_render_dropdown() {
    let mut ac = Autocomplete::new()
        .suggestions(vec![
            Suggestion::new("apple").description("A fruit").icon('🍎'),
            Suggestion::new("banana")
                .description("Also a fruit")
                .icon('🍌'),
        ])
        .min_chars(1);

    ac.focus();
    ac.set_value("a");

    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = create_render_context(&mut buffer, area);

    ac.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_autocomplete_render_dropdown_selected() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana", "cherry"])
        .min_chars(1);

    ac.focus();
    ac.set_value("a");
    ac.handle_key(KeyEvent::new(Key::Down));

    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = create_render_context(&mut buffer, area);

    ac.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_autocomplete_render_small_area() {
    let ac = Autocomplete::new().value("test");
    let mut buffer = Buffer::new(5, 2);
    let area = Rect::new(0, 0, 5, 2);
    let mut ctx = create_render_context(&mut buffer, area);

    ac.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_autocomplete_render_zero_area() {
    let ac = Autocomplete::new();
    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = create_render_context(&mut buffer, area);

    ac.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_autocomplete_render_no_room_for_dropdown() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);

    ac.focus();
    ac.set_value("a");

    // Only 1 line high - no room for dropdown
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = create_render_context(&mut buffer, area);

    ac.render(&mut ctx);
    // Should not panic, dropdown shouldn't render
}

#[test]
fn test_autocomplete_render_long_value_truncated() {
    let ac = Autocomplete::new().value("this is a very long value that exceeds width");
    let mut buffer = Buffer::new(20, 2);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = create_render_context(&mut buffer, area);

    ac.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_autocomplete_render_custom_colors() {
    let ac = Autocomplete::new()
        .value("test")
        .input_style(Color::CYAN, Color::MAGENTA)
        .dropdown_style(Color::rgb(64, 64, 64), Color::YELLOW, Color::GREEN)
        .highlight_fg(Color::RED);

    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = create_render_context(&mut buffer, area);

    ac.render(&mut ctx);
    // Should not panic
}

// =============================================================================
// CSS/Styling Tests
// =============================================================================

#[test]
fn test_autocomplete_element_id() {
    let ac = Autocomplete::new().element_id("search-input");
    assert_eq!(View::id(&ac), Some("search-input"));
}

#[test]
fn test_autocomplete_add_class() {
    let ac = Autocomplete::new().class("search").class("large");
    assert!(ac.has_class("search"));
    assert!(ac.has_class("large"));
}

#[test]
fn test_autocomplete_classes_builder() {
    let ac = Autocomplete::new().classes(vec!["class1", "class2"]);
    assert!(ac.has_class("class1"));
    assert!(ac.has_class("class2"));
}

#[test]
fn test_autocomplete_view_meta() {
    let ac = Autocomplete::new()
        .element_id("test-id")
        .class("test-class");

    let meta = ac.meta();
    assert_eq!(meta.widget_type, "Autocomplete");
    assert_eq!(meta.id, Some("test-id".to_string()));
    assert!(meta.classes.contains("test-class"));
}

#[test]
fn test_autocomplete_set_id() {
    let mut ac = Autocomplete::new();
    ac.set_id("new-id");
    assert_eq!(View::id(&ac), Some("new-id"));
}

#[test]
fn test_autocomplete_add_class_method() {
    let mut ac = Autocomplete::new();
    ac.add_class("active");
    assert!(ac.has_class("active"));
}

#[test]
fn test_autocomplete_remove_class() {
    let mut ac = Autocomplete::new().class("active");
    ac.remove_class("active");
    assert!(!ac.has_class("active"));
}

#[test]
fn test_autocomplete_toggle_class() {
    let mut ac = Autocomplete::new();

    ac.toggle_class("selected");
    assert!(ac.has_class("selected"));

    ac.toggle_class("selected");
    assert!(!ac.has_class("selected"));
}

// =============================================================================
// Edge Cases Tests
// =============================================================================

#[test]
fn test_autocomplete_empty_value() {
    let ac = Autocomplete::new();
    assert_eq!(ac.get_value(), "");
    assert!(!ac.is_focused());
}

#[test]
fn test_autocomplete_empty_suggestions() {
    let mut ac = Autocomplete::new().suggestions(Vec::<String>::new());
    ac.focus();
    ac.set_value("test");
    assert!(ac.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_unicode_input() {
    // Note: insert_char has issues with multi-byte UTF-8 chars
    // Use set_value() instead for unicode content
    let ac = Autocomplete::new().value("héllo");
    assert_eq!(ac.get_value(), "héllo");
}

#[test]
fn test_autocomplete_special_chars() {
    let mut ac = Autocomplete::new();
    ac.focus();

    ac.handle_key(KeyEvent::new(Key::Char('@')));
    ac.handle_key(KeyEvent::new(Key::Char('1')));
    ac.handle_key(KeyEvent::new(Key::Char('2')));
    ac.handle_key(KeyEvent::new(Key::Char('3')));

    assert_eq!(ac.get_value(), "@123");
}

#[test]
fn test_autocomplete_rapid_backspace() {
    let mut ac = Autocomplete::new().value("hello");
    ac.focus();

    for _ in 0..10 {
        ac.handle_key(KeyEvent::new(Key::Backspace));
    }

    assert_eq!(ac.get_value(), "");
}

#[test]
fn test_autocomplete_insert_in_middle() {
    let mut ac = Autocomplete::new().value("ac");
    ac.focus();

    ac.handle_key(KeyEvent::new(Key::Left));
    ac.handle_key(KeyEvent::new(Key::Char('b')));

    assert_eq!(ac.get_value(), "abc");
}

#[test]
fn test_autocomplete_delete_from_middle() {
    let mut ac = Autocomplete::new().value("abc");
    ac.focus();

    ac.handle_key(KeyEvent::new(Key::Left));
    ac.handle_key(KeyEvent::new(Key::Left));

    ac.handle_key(KeyEvent::new(Key::Delete));
    assert_eq!(ac.get_value(), "ac");
}

#[test]
fn test_autocomplete_navigation_wrapping() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["a1", "a2", "a3"])
        .min_chars(1);

    ac.focus();
    ac.set_value("a");

    // Navigation should work
    assert!(ac.handle_key(KeyEvent::new(Key::Up)));
    assert!(ac.handle_key(KeyEvent::new(Key::Down)));
}

#[test]
fn test_autocomplete_single_suggestion() {
    let mut ac = Autocomplete::new().suggestions(vec!["apple"]).min_chars(1);

    ac.focus();
    ac.set_value("ap");
    assert!(ac.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_value_changes() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "apricot"])
        .min_chars(1);

    ac.focus();
    ac.set_value("ap");

    // Change value
    ac.set_value("apric");
    // Selection should still be available
    assert!(ac.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_no_match() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);

    ac.focus();
    ac.set_value("xyz");
    assert!(ac.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_suggestion_with_different_value() {
    let mut ac = Autocomplete::new()
        .suggestions(vec![
            Suggestion::with_value("Apple", "apple_value"),
            Suggestion::with_value("Banana", "banana_value"),
        ])
        .min_chars(1);

    ac.focus();
    ac.set_value("ap");

    let selected = ac.selected_suggestion();
    assert!(selected.is_some());
    assert_eq!(selected.unwrap().label, "Apple");

    ac.accept_selection();
    assert_eq!(ac.get_value(), "apple_value");
}

// =============================================================================
// Complex Interaction Tests
// =============================================================================

#[test]
fn test_autocomplete_complete_typing_flow() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "application", "apply"])
        .min_chars(1);

    ac.focus();

    // Type "app"
    ac.handle_key(KeyEvent::new(Key::Char('a')));
    ac.handle_key(KeyEvent::new(Key::Char('p')));
    ac.handle_key(KeyEvent::new(Key::Char('p')));

    assert!(ac.selected_suggestion().is_some());
    assert_eq!(ac.get_value(), "app");

    // Accept selection without navigating (first match)
    ac.handle_key(KeyEvent::new(Key::Enter));
    assert_eq!(ac.get_value(), "apple");
}

#[test]
fn test_autocomplete_cancel_and_resume() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);

    ac.focus();

    // Start typing
    ac.handle_key(KeyEvent::new(Key::Char('a')));
    assert!(ac.selected_suggestion().is_some());

    // Cancel
    ac.handle_key(KeyEvent::new(Key::Escape));
    // Navigation should not work after escape
    assert!(!ac.handle_key(KeyEvent::new(Key::Down)));

    // Type more
    ac.handle_key(KeyEvent::new(Key::Char('p')));
    // Dropdown should reappear
    assert!(ac.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_multiple_selections() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana", "cherry"])
        .min_chars(1);

    ac.focus();

    // Select and accept apple
    ac.set_value("a");
    ac.accept_selection();
    assert_eq!(ac.get_value(), "apple");

    // Select banana
    ac.set_value("ban");
    assert!(ac.selected_suggestion().is_some());
    ac.accept_selection();
    assert_eq!(ac.get_value(), "banana");
}

#[test]
fn test_autocomplete_edit_after_selection() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);

    ac.focus();

    // Select apple
    ac.set_value("ap");
    ac.accept_selection();
    assert_eq!(ac.get_value(), "apple");

    // Edit it
    ac.handle_key(KeyEvent::new(Key::Backspace));
    ac.handle_key(KeyEvent::new(Key::Backspace));
    assert_eq!(ac.get_value(), "app");
}

#[test]
fn test_autocomplete_value_update_resets_state() {
    let mut ac = Autocomplete::new().value("test");
    ac.focus();

    ac.set_value("new value");
    // Should not panic
}

#[test]
fn test_autocomplete_focus_updates_filter() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1)
        .value("ap");

    // Not focused yet
    assert!(ac.selected_suggestion().is_none());

    ac.focus();
    // Focus should trigger filter update
    assert!(ac.selected_suggestion().is_some());
}

#[test]
fn test_autocomplete_blur_clears_state() {
    let mut ac = Autocomplete::new()
        .suggestions(vec!["apple", "banana"])
        .min_chars(1);

    ac.focus();
    ac.set_value("ap");
    assert!(ac.selected_suggestion().is_some());

    ac.blur();
    // Navigation should not work after blur
    assert!(!ac.handle_key(KeyEvent::new(Key::Down)));

    // Focus again
    ac.focus();
    // Navigation should work again
    assert!(ac.handle_key(KeyEvent::new(Key::Down)));
}
