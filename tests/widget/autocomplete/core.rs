//! Autocomplete widget tests

use revue::event::{Key, KeyEvent};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::utils::FilterMode;
use revue::widget::autocomplete::{autocomplete, Autocomplete, Suggestion};
use revue::widget::traits::{RenderContext, StyledView, View};

fn key(a: &mut Autocomplete, k: Key) -> bool {
    a.handle_key(KeyEvent::new(k))
}

fn type_str(a: &mut Autocomplete, s: &str) {
    for c in s.chars() {
        key(a, Key::Char(c));
    }
}

/// Render `a` into a `width` x `height` buffer.
fn render(a: &Autocomplete, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16, width: u16) -> String {
    (0..width)
        .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
        .collect()
}

fn render_rows(a: &Autocomplete, width: u16, height: u16) -> Vec<String> {
    let buffer = render(a, width, height);
    (0..height).map(|y| row(&buffer, y, width)).collect()
}

/// Labels currently shown in the dropdown (rows below the input line).
fn dropdown(a: &Autocomplete, width: u16, height: u16) -> Vec<String> {
    render_rows(a, width, height)
        .into_iter()
        .skip(1)
        .map(|r| r.trim_end().to_string())
        .filter(|r| !r.is_empty())
        .collect()
}

fn fruits() -> Autocomplete {
    Autocomplete::new().suggestions(["apple", "application", "banana", "cherry"])
}

// ==================== Constructor Tests ====================

#[test]
fn test_autocomplete_new() {
    let a = Autocomplete::new();
    assert_eq!(a.get_value(), "");
    assert!(!a.is_focused());
    assert!(a.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_default() {
    let a = Autocomplete::default();
    assert_eq!(a.get_value(), "");
    assert!(!a.is_focused());
}

#[test]
fn test_autocomplete_helper() {
    let a = autocomplete();
    assert_eq!(a.get_value(), "");
}

// ==================== Builder Tests ====================

#[test]
fn test_autocomplete_suggestions_from_strings() {
    let mut a = Autocomplete::new().suggestions(vec!["test".to_string()]);
    type_str(&mut a, "t");
    assert_eq!(a.selected_suggestion().unwrap().label, "test");
}

#[test]
fn test_autocomplete_suggestions_from_slice() {
    let mut a = Autocomplete::new()
        .suggestions(["a", "b", "c"])
        .filter_mode(FilterMode::None);
    type_str(&mut a, "x");
    assert_eq!(dropdown(&a, 10, 5), vec!["a", "b", "c"]);
}

#[test]
fn test_autocomplete_suggestions_with_suggestion_structs() {
    let mut a = Autocomplete::new().suggestions(vec![
        Suggestion::with_value("Item 1", "one"),
        Suggestion::new("Item 2"),
    ]);
    type_str(&mut a, "Item");
    assert_eq!(dropdown(&a, 10, 5), vec!["Item 1", "Item 2"]);
    assert!(a.accept_selection());
    assert_eq!(a.get_value(), "one");
}

#[test]
fn test_autocomplete_suggestions_empty() {
    let mut a = Autocomplete::new().suggestions::<Vec<String>, String>(vec![]);
    type_str(&mut a, "a");
    assert!(a.selected_suggestion().is_none());
    assert!(dropdown(&a, 10, 5).is_empty());
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
fn test_autocomplete_value_puts_cursor_at_end() {
    let mut a = Autocomplete::new().value("test");
    type_str(&mut a, "!");
    assert_eq!(a.get_value(), "test!");
}

#[test]
fn test_autocomplete_placeholder() {
    let a = Autocomplete::new().placeholder("Search...");
    assert_eq!(render_rows(&a, 12, 1)[0], "Search...   ");
}

#[test]
fn test_autocomplete_placeholder_string() {
    let a = Autocomplete::new().placeholder(String::from("Type here"));
    assert_eq!(render_rows(&a, 12, 1)[0].trim_end(), "Type here");
}

#[test]
fn test_autocomplete_placeholder_empty() {
    let a = Autocomplete::new().placeholder("");
    assert_eq!(render_rows(&a, 12, 1)[0].trim(), "");
}

#[test]
fn test_autocomplete_placeholder_hidden_by_value() {
    let a = Autocomplete::new().placeholder("Search...").value("abc");
    assert_eq!(render_rows(&a, 12, 1)[0].trim_end(), "abc");
}

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
fn test_autocomplete_dropdown_clipped_to_height() {
    let mut a = fruits().filter_mode(FilterMode::None);
    type_str(&mut a, "x");
    // One row for the input, two for the dropdown
    assert_eq!(dropdown(&a, 20, 3), vec!["apple", "application"]);
}

#[test]
fn test_autocomplete_input_style() {
    let a = Autocomplete::new()
        .input_style(Color::CYAN, Color::BLACK)
        .value("ab");
    let buffer = render(&a, 5, 1);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::CYAN));
    assert_eq!(cell.bg, Some(Color::BLACK));
    // The rest of the input line is filled with the background
    assert_eq!(buffer.get(4, 0).unwrap().bg, Some(Color::BLACK));
}

#[test]
fn test_autocomplete_style_override() {
    let a = Autocomplete::new()
        .input_style(Color::RED, Color::BLACK)
        .input_style(Color::BLUE, Color::WHITE)
        .value("x");
    let buffer = render(&a, 5, 1);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::BLUE));
    assert_eq!(cell.bg, Some(Color::WHITE));
}

#[test]
fn test_autocomplete_dropdown_style() {
    let mut a = Autocomplete::new()
        .suggestions(["ab", "ac"])
        .dropdown_style(Color::BLUE, Color::WHITE, Color::GREEN)
        .highlight_fg(Color::YELLOW);
    type_str(&mut a, "a");
    let buffer = render(&a, 10, 3);
    // Selected row: matched 'a' highlighted, the rest in selected colors
    let first = buffer.get(0, 1).unwrap();
    assert_eq!(first.fg, Some(Color::YELLOW));
    assert_eq!(first.bg, Some(Color::GREEN));
    let second = buffer.get(1, 1).unwrap();
    assert_eq!(second.fg, Some(Color::WHITE));
    assert_eq!(second.bg, Some(Color::GREEN));
    // Unselected row uses the dropdown background
    assert_eq!(buffer.get(1, 2).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_autocomplete_highlight_fg() {
    let mut a = Autocomplete::new()
        .suggestions(["apple"])
        .highlight_fg(Color::YELLOW);
    type_str(&mut a, "ae");
    let buffer = render(&a, 10, 2);
    let highlighted: Vec<bool> = (0..5)
        .map(|x| buffer.get(x, 1).unwrap().fg == Some(Color::YELLOW))
        .collect();
    assert_eq!(highlighted, vec![true, false, false, false, true]);
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
        .dropdown_style(Color::GREEN, Color::WHITE, Color::BLUE)
        .highlight_fg(Color::YELLOW)
        .suggestions(vec!["a", "b", "c"]);

    assert_eq!(a.get_value(), "test");
}

// ==================== Getter Tests ====================

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
    let mut a = fruits().value("app");
    // The builder value does not filter until focus
    assert!(a.selected_suggestion().is_none());
    a.focus();
    assert_eq!(a.selected_suggestion().unwrap().label, "apple");
}

#[test]
fn test_autocomplete_is_focused_default() {
    let a = Autocomplete::new();
    assert!(!a.is_focused());
}

// ==================== State-Changing Methods Tests ====================

#[test]
fn test_autocomplete_set_value_string() {
    let mut a = Autocomplete::new();
    a.set_value("test");
    assert_eq!(a.get_value(), "test");
}

#[test]
fn test_autocomplete_set_value_empty() {
    let mut a = Autocomplete::new().value("x");
    a.set_value("");
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_set_value_overwrite() {
    let mut a = Autocomplete::new().value("first");
    a.set_value("second");
    assert_eq!(a.get_value(), "second");
}

#[test]
fn test_autocomplete_set_value_filters() {
    let mut a = fruits();
    a.set_value("ban");
    assert_eq!(a.selected_suggestion().unwrap().label, "banana");
}

#[test]
fn test_autocomplete_set_value_unicode() {
    let mut a = Autocomplete::new();
    a.set_value("Привет");
    assert_eq!(a.get_value(), "Привет");
}

#[test]
fn test_autocomplete_set_value_unicode_then_edit() {
    // The cursor lands after the last character, not past the last byte.
    let mut a = Autocomplete::new();
    a.set_value("Привет");
    key(&mut a, Key::Backspace);
    assert_eq!(a.get_value(), "Приве");
    key(&mut a, Key::Left);
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "Привxе");
}

#[test]
fn test_autocomplete_set_value_unicode_cursor_rendered_at_end() {
    let mut a = Autocomplete::new().input_style(Color::WHITE, Color::BLACK);
    a.focus();
    a.set_value("Привет");
    let buffer = render(&a, 20, 1);
    // The cursor cell is drawn inverted, right after the text
    assert_eq!(buffer.get(6, 0).unwrap().bg, Some(Color::WHITE));
    assert_eq!(buffer.get(5, 0).unwrap().bg, Some(Color::BLACK));
}

#[test]
fn test_autocomplete_set_suggestions_empty() {
    let mut a = fruits().value("a");
    a.focus();
    assert!(a.selected_suggestion().is_some());
    a.set_suggestions(vec![]);
    assert!(a.selected_suggestion().is_none());
}

#[test]
fn test_autocomplete_set_suggestions_vec() {
    let mut a = Autocomplete::new().value("Item");
    a.set_suggestions(vec![Suggestion::new("Item 1"), Suggestion::new("Item 2")]);
    assert_eq!(dropdown(&a, 10, 5), vec!["Item 1", "Item 2"]);
}

#[test]
fn test_autocomplete_set_suggestions_overwrite() {
    let mut a = Autocomplete::new().value("x").filter_mode(FilterMode::None);
    a.set_suggestions(vec!["a".into(), "b".into()]);
    a.set_suggestions(vec!["c".into(), "d".into(), "e".into()]);
    assert_eq!(dropdown(&a, 10, 5), vec!["c", "d", "e"]);
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
fn test_autocomplete_blur_hides_dropdown() {
    let mut a = fruits().value("app");
    a.focus();
    assert!(!dropdown(&a, 20, 6).is_empty());
    a.blur();
    assert!(dropdown(&a, 20, 6).is_empty());
    // Navigation keys are no longer consumed
    assert!(!key(&mut a, Key::Down));
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
fn test_autocomplete_accept_selection_none() {
    let mut a = Autocomplete::new();
    assert!(!a.accept_selection());
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_accept_selection_with_suggestions() {
    let mut a = fruits().value("app");
    a.focus();
    assert!(a.accept_selection());
    assert_eq!(a.get_value(), "apple");
    // Dropdown closes after accepting
    assert!(dropdown(&a, 20, 6).is_empty());
}

#[test]
fn test_autocomplete_accept_unicode_then_edit() {
    let mut a = Autocomplete::new().suggestions(["日本語"]);
    type_str(&mut a, "日");
    assert!(a.accept_selection());
    assert_eq!(a.get_value(), "日本語");
    key(&mut a, Key::Backspace);
    assert_eq!(a.get_value(), "日本");
}

// ==================== Key Handling Tests ====================

#[test]
fn test_autocomplete_handle_key_char() {
    let mut a = Autocomplete::new();
    assert!(key(&mut a, Key::Char('a')));
    assert_eq!(a.get_value(), "a");
}

#[test]
fn test_autocomplete_handle_key_multiple_chars() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "hello");
    assert_eq!(a.get_value(), "hello");
}

#[test]
fn test_autocomplete_handle_key_unicode_char() {
    let mut a = Autocomplete::new();
    assert!(key(&mut a, Key::Char('你')));
    assert_eq!(a.get_value(), "你");
}

#[test]
fn test_autocomplete_handle_key_backspace() {
    let mut a = Autocomplete::new().value("test");
    assert!(key(&mut a, Key::Backspace));
    assert_eq!(a.get_value(), "tes");
}

#[test]
fn test_autocomplete_handle_key_backspace_empty() {
    let mut a = Autocomplete::new();
    assert!(key(&mut a, Key::Backspace));
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_handle_key_backspace_at_start() {
    let mut a = Autocomplete::new().value("ab");
    key(&mut a, Key::Home);
    assert!(key(&mut a, Key::Backspace));
    assert_eq!(a.get_value(), "ab");
}

#[test]
fn test_autocomplete_handle_key_backspace_multiple() {
    let mut a = Autocomplete::new().value("hello");
    key(&mut a, Key::Backspace);
    key(&mut a, Key::Backspace);
    key(&mut a, Key::Backspace);
    assert_eq!(a.get_value(), "he");
}

#[test]
fn test_autocomplete_handle_key_delete() {
    let mut a = Autocomplete::new().value("test");
    key(&mut a, Key::Home);
    assert!(key(&mut a, Key::Delete));
    assert_eq!(a.get_value(), "est");
}

#[test]
fn test_autocomplete_handle_key_delete_at_end() {
    // The builder value leaves the cursor at the end: nothing to delete.
    let mut a = Autocomplete::new().value("ab");
    assert!(key(&mut a, Key::Delete));
    assert_eq!(a.get_value(), "ab");
    key(&mut a, Key::Left);
    assert!(key(&mut a, Key::Delete));
    assert_eq!(a.get_value(), "a");
}

#[test]
fn test_autocomplete_handle_key_delete_empty() {
    let mut a = Autocomplete::new();
    assert!(key(&mut a, Key::Delete));
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_handle_key_left() {
    let mut a = Autocomplete::new().value("test");
    assert!(key(&mut a, Key::Left));
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "tesxt");
}

#[test]
fn test_autocomplete_handle_key_left_at_start() {
    let mut a = Autocomplete::new().value("test");
    for _ in 0..4 {
        key(&mut a, Key::Left);
    }
    assert!(key(&mut a, Key::Left));
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "xtest");
}

#[test]
fn test_autocomplete_handle_key_right() {
    let mut a = Autocomplete::new().value("test");
    key(&mut a, Key::Home);
    assert!(key(&mut a, Key::Right));
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "txest");
}

#[test]
fn test_autocomplete_handle_key_right_at_end() {
    let mut a = Autocomplete::new().value("test");
    assert!(key(&mut a, Key::Right));
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "testx");
}

#[test]
fn test_autocomplete_handle_key_home() {
    let mut a = Autocomplete::new().value("test");
    assert!(key(&mut a, Key::Home));
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "xtest");
}

#[test]
fn test_autocomplete_handle_key_end() {
    let mut a = Autocomplete::new().value("test");
    key(&mut a, Key::Home);
    assert!(key(&mut a, Key::End));
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "testx");
}

#[test]
fn test_autocomplete_handle_key_up_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!key(&mut a, Key::Up));
}

#[test]
fn test_autocomplete_handle_key_down_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!key(&mut a, Key::Down));
}

#[test]
fn test_autocomplete_handle_key_enter_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!key(&mut a, Key::Enter));
}

#[test]
fn test_autocomplete_handle_key_tab_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!key(&mut a, Key::Tab));
}

#[test]
fn test_autocomplete_handle_key_escape_without_dropdown() {
    let mut a = Autocomplete::new();
    assert!(!key(&mut a, Key::Escape));
}

#[test]
fn test_autocomplete_handle_key_invalid_key() {
    let mut a = Autocomplete::new();
    assert!(!key(&mut a, Key::Tab));
    assert!(!key(&mut a, Key::PageUp));
    assert!(!key(&mut a, Key::PageDown));
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_handle_key_down_up_with_dropdown() {
    let mut a = fruits();
    type_str(&mut a, "app");
    assert_eq!(a.selected_suggestion().unwrap().label, "apple");
    assert!(key(&mut a, Key::Down));
    assert_eq!(a.selected_suggestion().unwrap().label, "application");
    // Down at the last item stays there
    assert!(key(&mut a, Key::Down));
    assert_eq!(a.selected_suggestion().unwrap().label, "application");
    assert!(key(&mut a, Key::Up));
    assert_eq!(a.selected_suggestion().unwrap().label, "apple");
    assert!(key(&mut a, Key::Up));
    assert_eq!(a.selected_suggestion().unwrap().label, "apple");
}

#[test]
fn test_autocomplete_handle_key_enter_accepts() {
    let mut a = fruits();
    type_str(&mut a, "app");
    key(&mut a, Key::Down);
    assert!(key(&mut a, Key::Enter));
    assert_eq!(a.get_value(), "application");
    assert!(dropdown(&a, 20, 6).is_empty());
}

#[test]
fn test_autocomplete_handle_key_tab_accepts() {
    let mut a = fruits();
    type_str(&mut a, "che");
    assert!(key(&mut a, Key::Tab));
    assert_eq!(a.get_value(), "cherry");
}

#[test]
fn test_autocomplete_handle_key_escape_closes_dropdown() {
    let mut a = fruits();
    type_str(&mut a, "app");
    assert!(key(&mut a, Key::Escape));
    assert!(dropdown(&a, 20, 6).is_empty());
    assert_eq!(a.get_value(), "app");
    assert!(!key(&mut a, Key::Escape));
}

#[test]
fn test_autocomplete_typing_reopens_dropdown() {
    let mut a = fruits();
    type_str(&mut a, "ap");
    key(&mut a, Key::Escape);
    type_str(&mut a, "p");
    assert_eq!(dropdown(&a, 20, 6), vec!["apple", "application"]);
}

#[test]
fn test_autocomplete_handle_key_navigation_sequence() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "tes");
    key(&mut a, Key::Left);
    key(&mut a, Key::Right);
    key(&mut a, Key::Home);
    key(&mut a, Key::End);
    assert_eq!(a.get_value(), "tes");
}

#[test]
fn test_autocomplete_handle_key_edit_and_navigate() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "abc");
    key(&mut a, Key::Left);
    key(&mut a, Key::Left);
    type_str(&mut a, "x"); // Insert at cursor position
    assert_eq!(a.get_value(), "axbc");
}

// ==================== Rendering Tests ====================

#[test]
fn test_autocomplete_render_empty() {
    let a = Autocomplete::new();
    for r in render_rows(&a, 20, 5) {
        assert_eq!(r.trim(), "");
    }
}

#[test]
fn test_autocomplete_render_with_value() {
    let a = Autocomplete::new().value("test");
    assert_eq!(render_rows(&a, 20, 5)[0].trim_end(), "test");
}

#[test]
fn test_autocomplete_render_focused() {
    // The focused cursor is drawn with inverted colors.
    let mut a = Autocomplete::new()
        .input_style(Color::WHITE, Color::BLACK)
        .value("ab");
    a.focus();
    let buffer = render(&a, 20, 5);
    let cursor = buffer.get(2, 0).unwrap();
    assert_eq!(cursor.fg, Some(Color::BLACK));
    assert_eq!(cursor.bg, Some(Color::WHITE));

    a.blur();
    let buffer = render(&a, 20, 5);
    assert_eq!(buffer.get(2, 0).unwrap().bg, Some(Color::BLACK));
}

#[test]
fn test_autocomplete_render_cursor_on_char() {
    let mut a = Autocomplete::new()
        .input_style(Color::WHITE, Color::BLACK)
        .value("ab");
    a.focus();
    key(&mut a, Key::Home);
    let buffer = render(&a, 20, 1);
    let cursor = buffer.get(0, 0).unwrap();
    assert_eq!(cursor.symbol, 'a');
    assert_eq!(cursor.bg, Some(Color::WHITE));
}

#[test]
fn test_autocomplete_render_zero_width() {
    let a = Autocomplete::new().value("x");
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 0, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_autocomplete_render_zero_height() {
    let a = Autocomplete::new().value("x");
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_autocomplete_render_very_narrow() {
    // Below three columns nothing is drawn; at three the input appears.
    let a = Autocomplete::new().value("abcd");
    assert_eq!(render_rows(&a, 2, 1)[0], "  ");
    assert_eq!(render_rows(&a, 3, 1)[0], "abc");
}

#[test]
fn test_autocomplete_render_with_suggestions() {
    let mut a = fruits().value("app");
    a.focus();
    let rows = render_rows(&a, 30, 10);
    assert_eq!(rows[0].trim_end(), "app");
    assert_eq!(rows[1].trim_end(), "apple");
    assert_eq!(rows[2].trim_end(), "application");
    assert_eq!(rows[3].trim_end(), "");
}

#[test]
fn test_autocomplete_render_icon_and_description() {
    let mut a = Autocomplete::new().suggestions(vec![Suggestion::new("main.rs")
        .icon('#')
        .description("entry")]);
    type_str(&mut a, "main");
    assert_eq!(render_rows(&a, 20, 2)[1].trim_end(), "# main.rs entry");
}

#[test]
fn test_autocomplete_render_long_value() {
    let a = Autocomplete::new().value("this is a very long value");
    assert_eq!(render_rows(&a, 20, 5)[0], "this is a very long ");
}

#[test]
fn test_autocomplete_render_unicode_value() {
    let a = Autocomplete::new().value("こんにちは世界");
    assert!(render_rows(&a, 20, 5)[0].starts_with("こんにちは世界"));
}

#[test]
fn test_autocomplete_render_offset_area() {
    let a = Autocomplete::new().value("test");
    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(10, 2, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    assert_eq!(row(&buffer, 2, 50)[10..14].to_string(), "test");
    assert_eq!(row(&buffer, 0, 50).trim(), "");
}

// ==================== CSS Integration Tests ====================

#[test]
fn test_autocomplete_element_id() {
    let a = Autocomplete::new().element_id("search-box");
    assert_eq!(View::id(&a), Some("search-box"));
}

#[test]
fn test_autocomplete_view_id_none() {
    let a = Autocomplete::new();
    assert_eq!(View::id(&a), None);
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
        .element_id("my-element")
        .class("class1")
        .class("class2");

    let meta = a.meta();
    assert_eq!(meta.id, Some("my-element".to_string()));
    assert_eq!(meta.classes.len(), 2);
    assert!(meta.classes.contains("class1"));
    assert!(meta.classes.contains("class2"));
}

// ==================== Edge Cases Tests ====================

#[test]
fn test_autocomplete_special_chars_in_value() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "@#$%");
    assert_eq!(a.get_value(), "@#$%");
}

#[test]
fn test_autocomplete_newline_chars() {
    let mut a = Autocomplete::new();
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
    key(&mut a, Key::Char('🔍'));
    assert_eq!(a.get_value(), "🔍");
}

#[test]
fn test_autocomplete_rapid_character_input() {
    let mut a = Autocomplete::new();
    for i in 0..100 {
        key(&mut a, Key::Char(char::from_digit(i % 10, 10).unwrap()));
    }
    assert_eq!(a.get_value().len(), 100);
}

#[test]
fn test_autocomplete_rapid_backspace() {
    let mut a = Autocomplete::new();
    a.set_value("abcdefghijklmnopqrstuvwxyz");
    for _ in 0..26 {
        key(&mut a, Key::Backspace);
    }
    assert_eq!(a.get_value(), "");
}

#[test]
fn test_autocomplete_mix_edit_and_navigation() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "abc");
    key(&mut a, Key::Left);
    key(&mut a, Key::Backspace); // Deletes 'b'
    type_str(&mut a, "x");
    assert_eq!(a.get_value(), "axc");
}

#[test]
fn test_autocomplete_cursor_movement_edge_cases() {
    let mut a = Autocomplete::new();
    a.set_value("test");
    for _ in 0..20 {
        key(&mut a, Key::Left);
    }
    type_str(&mut a, "<");
    for _ in 0..20 {
        key(&mut a, Key::Right);
    }
    type_str(&mut a, ">");
    assert_eq!(a.get_value(), "<test>");
}

#[test]
fn test_autocomplete_multiple_independent_instances() {
    let mut a1 = Autocomplete::new().value("first");
    let mut a2 = Autocomplete::new().value("second");

    key(&mut a1, Key::Char('x'));
    key(&mut a2, Key::Char('y'));

    assert_eq!(a1.get_value(), "firstx");
    assert_eq!(a2.get_value(), "secondy");
}

#[test]
fn test_autocomplete_typing_and_deleting_full_cycle() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "hello");
    assert_eq!(a.get_value(), "hello");
    for _ in 0..5 {
        key(&mut a, Key::Backspace);
    }
    assert_eq!(a.get_value(), "");
    type_str(&mut a, "world");
    assert_eq!(a.get_value(), "world");
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

#[test]
fn test_autocomplete_set_value_then_type() {
    let mut a = Autocomplete::new();
    a.set_value("hello");
    type_str(&mut a, "!");
    assert_eq!(a.get_value(), "hello!");
}

#[test]
fn test_autocomplete_type_then_set_value() {
    let mut a = Autocomplete::new();
    type_str(&mut a, "x");
    a.set_value("y");
    assert_eq!(a.get_value(), "y");
}
