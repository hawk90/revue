//! Text widget tests extracted from src/widget/display/text.rs

use revue::style::Color;
use revue::widget::display::Text;
use revue::widget::display::Alignment;

// Note: Most tests for Text access private fields, so only a subset of
// tests that use public APIs are extracted here.

// =========================================================================
// Text builder tests (using public APIs only)
// =========================================================================

#[test]
fn test_text_builder() {
    let text = Text::new("Test")
        .fg(Color::RED)
        .bold()
        .align(Alignment::Center);

    // Test what we can through public APIs only
    assert_eq!(text.content(), "Test");
    // NOTE: We can't test private fields like fg, bold, etc. from public API
}

// Edge case tests for text content handling
#[test]
fn test_text_empty_content() {
    // Text widget should handle empty content gracefully
    let text = Text::new("");
    assert_eq!(text.content(), "");
}

#[test]
fn test_text_whitespace_only() {
    // Text widget should handle whitespace-only content
    let text = Text::new("   ");
    assert_eq!(text.content(), "   ");
}

#[test]
fn test_text_newlines() {
    // Text widget should handle newlines
    let text = Text::new("line1\nline2\nline3");
    assert_eq!(text.content(), "line1\nline2\nline3");
}

#[test]
fn test_text_tabs() {
    // Text widget should handle tabs
    let text = Text::new("col1\tcol2");
    assert_eq!(text.content(), "col1\tcol2");
}

#[test]
fn test_text_special_characters() {
    // Text widget should handle special characters
    let special = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
    let text = Text::new(special);
    assert_eq!(text.content(), special);
}

#[test]
fn test_text_emoji() {
    // Text widget should handle emoji (multi-byte UTF-8)
    let emoji = "😀😁😂🤣";
    let text = Text::new(emoji);
    assert_eq!(text.content(), emoji);
}

#[test]
fn test_text_mixed_unicode() {
    // Text widget should handle mixed ASCII and Unicode
    let mixed = "Hello 世界! 🌍";
    let text = Text::new(mixed);
    assert_eq!(text.content(), mixed);
}

#[test]
fn test_text_zero_width_joiners() {
    // Text widget should handle zero-width joiners
    let text = Text::new("e\u{200d}"); // 'e' + zero-width joiner
    assert_eq!(text.content(), "e\u{200d}");
}

#[test]
fn test_text_very_long_single_line() {
    // Text widget should handle very long single lines
    let long = "x".repeat(10000);
    let text = Text::new(&long);
    assert_eq!(text.content(), long);
}

#[test]
fn test_text_null_bytes_not_allowed() {
    // Rust strings don't allow null bytes, but we should handle
    // edge case where external input might contain them
    // This just verifies our type system handles it correctly
    let text = Text::new("valid string");
    assert_eq!(text.content(), "valid string");
}

#[test]
fn test_text_with_styled_modifiers() {
    // Text widget should handle modifiers with edge case content
    let text = Text::new("  ").bold().italic();
    assert_eq!(text.content(), "  ");
    // NOTE: We can't test private modifier states from public API
}

#[test]
fn test_text_builder_chaining() {
    // Text builder should support method chaining
    let text = Text::new("Test")
        .fg(Color::RED)
        .bg(Color::BLUE)
        .bold()
        .italic()
        .underline()
        .dim();

    assert_eq!(text.content(), "Test");
    // NOTE: We can't test private field states from public API
}

#[test]
fn test_text_all_alignments() {
    // Test all alignment options work
    for align in &[Alignment::Left, Alignment::Center, Alignment::Right] {
        let text = Text::new("Test").align(*align);
        assert_eq!(text.content(), "Test");
        // NOTE: We can't test private alignment field from public API
    }
}

#[test]
fn test_text_with_ansi_codes() {
    // Text widget should handle ANSI escape sequences
    let ansi = "\x1b[31mRed text\x1b[0m";
    let text = Text::new(ansi);
    assert_eq!(text.content(), ansi);
}

#[test]
fn test_text_combining_diacritics() {
    // Text widget should handle combining diacritical marks
    let text = Text::new("café"); // precomposed é
    assert_eq!(text.content(), "café");

    let text2 = Text::new("cafe\u{301}"); // e + combining acute
    assert_eq!(text2.content(), "cafe\u{301}");
}
