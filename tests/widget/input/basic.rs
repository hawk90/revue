//! Basic input widget tests for public API coverage

use revue::widget::input;
use revue::style::Color;

// =========================================================================
// Constructor tests
// =========================================================================

#[test]
fn test_input_new_creates_empty_input() {
    let i = input();
    assert_eq!(i.text(), "");
}

#[test]
fn test_input_default_trait() {
    let i = revue::widget::Input::default();
    assert_eq!(i.text(), "");
}

// =========================================================================
// Builder method tests
// =========================================================================

#[test]
fn test_input_placeholder() {
    let i = input().placeholder("test");
    assert_eq!(i.placeholder, "test");
}

#[test]
fn test_input_placeholder_with_string() {
    let i = input().placeholder(String::from("custom"));
    assert_eq!(i.placeholder, "custom");
}

#[test]
fn test_input_value() {
    let i = input().value("hello");
    assert_eq!(i.text(), "hello");
}

#[test]
fn test_input_value_with_unicode() {
    let i = input().value("안녕🎉");
    assert_eq!(i.text(), "안녕🎉");
}

#[test]
fn test_input_fg() {
    let i = input().fg(Color::RED);
    assert_eq!(i.fg, Some(Color::RED));
}

#[test]
fn test_input_bg() {
    let i = input().bg(Color::BLUE);
    assert_eq!(i.bg, Some(Color::BLUE));
}

#[test]
fn test_input_cursor_style() {
    let i = input().cursor_style(Color::YELLOW, Color::BLACK);
    assert_eq!(i.cursor_fg, Some(Color::YELLOW));
    assert_eq!(i.cursor_bg, Some(Color::BLACK));
}

#[test]
fn test_input_selection_bg() {
    let i = input().selection_bg(Color::GREEN);
    assert_eq!(i.selection_bg, Some(Color::GREEN));
}

#[test]
fn test_input_focused() {
    let i = input().focused(true);
    assert!(i.focused);
}

#[test]
fn test_input_not_focused() {
    let i = input().focused(false);
    assert!(!i.focused);
}

// =========================================================================
// Builder method chaining tests
// =========================================================================

#[test]
fn test_input_builder_chaining() {
    let i = input()
        .value("test")
        .placeholder("enter text")
        .fg(Color::RED)
        .bg(Color::BLUE)
        .cursor_style(Color::YELLOW, Color::BLACK)
        .selection_bg(Color::GREEN)
        .focused(false);

    assert_eq!(i.text(), "test");
    assert_eq!(i.placeholder, "enter text");
    assert_eq!(i.fg, Some(Color::RED));
    assert_eq!(i.bg, Some(Color::BLUE));
    assert_eq!(i.cursor_fg, Some(Color::YELLOW));
    assert_eq!(i.cursor_bg, Some(Color::BLACK));
    assert_eq!(i.selection_bg, Some(Color::GREEN));
    assert!(!i.focused);
}

#[test]
fn test_input_multiple_builder_calls() {
    let i = input()
        .fg(Color::RED)
        .fg(Color::BLUE)
        .bg(Color::GREEN);

    assert_eq!(i.fg, Some(Color::BLUE)); // Last call wins
    assert_eq!(i.bg, Some(Color::GREEN));
}

// =========================================================================
// Getter method tests
// =========================================================================

#[test]
fn test_input_text_getter() {
    let i = input().value("hello world");
    assert_eq!(i.text(), "hello world");
}

#[test]
fn test_input_text_getter_empty() {
    let i = input();
    assert_eq!(i.text(), "");
}

#[test]
fn test_input_cursor_getter() {
    let i = input().value("hello");
    assert_eq!(i.cursor(), 5);
}

#[test]
fn test_input_cursor_getter_empty() {
    let i = input();
    assert_eq!(i.cursor(), 0);
}

// =========================================================================
// Clone trait tests
// =========================================================================

#[test]
fn test_input_clone() {
    let i1 = input()
        .value("test")
        .placeholder("placeholder")
        .fg(Color::RED)
        .bg(Color::BLUE)
        .focused(true);

    let i2 = i1.clone();

    assert_eq!(i2.text(), "test");
    assert_eq!(i2.placeholder, "placeholder");
    assert_eq!(i2.fg, Some(Color::RED));
    assert_eq!(i2.bg, Some(Color::BLUE));
    assert!(i2.focused);
}

#[test]
fn test_input_clone_independence() {
    let mut i1 = input().value("test");
    let i2 = i1.clone();

    // Modifying i1 should not affect i2
    i1.set_value("changed");

    assert_eq!(i1.text(), "changed");
    assert_eq!(i2.text(), "test");
}
