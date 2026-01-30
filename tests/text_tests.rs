//! Text widget integration tests

use revue::style::Color;
use revue::widget::{text, Text};

#[test]
fn test_text_new() {
    let _text = Text::new("Hello");
    // Text was created successfully
}

#[test]
fn test_text_content() {
    let text = Text::new("Hello World");
    assert_eq!(text.content(), "Hello World");
}

#[test]
fn test_text_heading() {
    let text = Text::heading("Title");
    assert_eq!(text.content(), "Title");
}

#[test]
fn test_text_muted() {
    let text = Text::muted("Secondary info");
    assert_eq!(text.content(), "Secondary info");
}

#[test]
fn test_text_error() {
    let text = Text::error("Error message");
    assert_eq!(text.content(), "Error message");
}

#[test]
fn test_text_success() {
    let text = Text::success("Success!");
    assert_eq!(text.content(), "Success!");
}

#[test]
fn test_text_warning() {
    let text = Text::warning("Warning!");
    assert_eq!(text.content(), "Warning!");
}

#[test]
fn test_text_info() {
    let text = Text::info("Info");
    assert_eq!(text.content(), "Info");
}

#[test]
fn test_text_label() {
    let text = Text::label("Label:");
    assert_eq!(text.content(), "Label:");
}

#[test]
fn test_text_fg() {
    let _text = Text::new("Colored").fg(Color::CYAN);
    // Foreground color was set successfully
}

#[test]
fn test_text_bg() {
    let _text = Text::new("Background").bg(Color::BLUE);
    // Background color was set successfully
}

#[test]
fn test_text_bold() {
    let _text = Text::new("Bold").bold();
    // Bold was set successfully
}

#[test]
fn test_text_italic() {
    let _text = Text::new("Italic").italic();
    // Italic was set successfully
}

#[test]
fn test_text_underline() {
    let _text = Text::new("Underline").underline();
    // Underline was set successfully
}

#[test]
fn test_text_reverse() {
    let _text = Text::new("Reverse").reverse();
    // Reverse was set successfully
}

#[test]
fn test_text_helper() {
    let _text = text("Hello World");
    // Helper function works
}

#[test]
fn test_text_empty() {
    let text = Text::new("");
    assert_eq!(text.content(), "");
}

#[test]
fn test_text_multiline() {
    let text = Text::new("Line 1\nLine 2\nLine 3");
    assert_eq!(text.content(), "Line 1\nLine 2\nLine 3");
}

#[test]
fn test_text_with_special_chars() {
    let text = Text::new("Special: ♥♦♣♠");
    assert_eq!(text.content(), "Special: ♥♦♣♠");
}

#[test]
fn test_text_with_unicode() {
    let text = Text::new("Unicode: 你好世界 🌍");
    assert_eq!(text.content(), "Unicode: 你好世界 🌍");
}

#[test]
fn test_text_builder_pattern() {
    let text = Text::new("Styled Text")
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .bold()
        .underline();

    assert_eq!(text.content(), "Styled Text");
}

#[test]
fn test_text_multiple_modifiers() {
    let text = Text::new("Multi").bold().italic().underline().reverse();

    assert_eq!(text.content(), "Multi");
}
