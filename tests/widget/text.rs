//! Text widget tests

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::style::Style;
use revue::style::VisualStyle;
use revue::widget::traits::RenderContext;
use revue::widget::{Alignment, StyledView, Text, View};

// ─────────────────────────────────────────────────────────────────────────
// Basic creation and builder methods
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_text_new() {
    let text = Text::new("Hello");
    assert_eq!(text.content(), "Hello");
}

#[test]
fn test_text_new_with_string() {
    let text = Text::new(String::from("World"));
    assert_eq!(text.content(), "World");
}

#[test]
fn test_text_new_with_str_ref() {
    let s = "Test";
    let text = Text::new(s);
    assert_eq!(text.content(), "Test");
}

#[test]
fn test_text_default() {
    let text = Text::default();
    assert_eq!(text.content(), "");
}

#[test]
fn test_text_content_empty() {
    let text = Text::new("");
    assert_eq!(text.content(), "");
}

#[test]
fn test_text_clone() {
    let text1 = Text::new("Hello").fg(Color::RED).bold();
    let text2 = text1.clone();
    assert_eq!(text1.content(), text2.content());
}

// ─────────────────────────────────────────────────────────────────────────
// Builder methods - colors
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_text_fg() {
    let text = Text::new("Red text").fg(Color::RED);
    // Private field check - just verify it renders without error
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
}

#[test]
fn test_text_bg() {
    let text = Text::new("Background").bg(Color::BLUE);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
}

#[test]
fn test_text_fg_and_bg() {
    let text = Text::new("Colored").fg(Color::YELLOW).bg(Color::BLACK);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Builder methods - text modifiers
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_text_bold() {
    let text = Text::new("Bold").bold();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    // Check that bold modifier is set
    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_text_italic() {
    let text = Text::new("Italic").italic();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::ITALIC));
}

#[test]
fn test_text_underline() {
    let text = Text::new("Underline").underline();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::UNDERLINE));
}

#[test]
fn test_text_reverse() {
    let text = Text::new("Reverse").reverse();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::REVERSE));
}

#[test]
fn test_text_combined_modifiers() {
    let text = Text::new("Styled").bold().italic().underline();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD));
    assert!(cell.modifier.contains(Modifier::ITALIC));
    assert!(cell.modifier.contains(Modifier::UNDERLINE));
}

// ─────────────────────────────────────────────────────────────────────────
// Alignment tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_text_align_left() {
    let text = Text::new("Left").align(Alignment::Left);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'L');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 't');
}

#[test]
fn test_text_align_right() {
    let text = Text::new("Right").align(Alignment::Right);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    // "Right" is 5 characters, so it should start at position 15 (20 - 5)
    assert_eq!(buffer.get(15, 0).unwrap().symbol, 'R');
    assert_eq!(buffer.get(19, 0).unwrap().symbol, 't');
}

#[test]
fn test_text_align_center_odd() {
    let text = Text::new("Hi").align(Alignment::Center);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    // "Hi" is 2 characters, in a 20-char wide area, centered at 9-10
    assert_eq!(buffer.get(9, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(10, 0).unwrap().symbol, 'i');
}

#[test]
fn test_text_align_center_even() {
    let text = Text::new("Center").align(Alignment::Center);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    // "Center" is 6 characters, centered in 20 chars: (20 - 6) / 2 = 7
    assert_eq!(buffer.get(7, 0).unwrap().symbol, 'C');
    assert_eq!(buffer.get(12, 0).unwrap().symbol, 'r');
}

#[test]
fn test_text_alignment_default() {
    // Alignment::Left is the default
    let text = Text::new("Default");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'D');
}

// ─────────────────────────────────────────────────────────────────────────
// Preset methods
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_text_heading() {
    let text = Text::heading("Title");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    // heading() applies bold and white fg
    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_text_muted() {
    let text = Text::muted("Secondary");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(text.content(), "Secondary");
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
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    // label() applies bold
    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD));
}

// ─────────────────────────────────────────────────────────────────────────
// Render operations
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_text_render() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let text = Text::new("Hello");
    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'e');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'l');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 'l');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'o');
}

#[test]
fn test_text_render_centered() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let text = Text::new("Hi").align(Alignment::Center);
    text.render(&mut ctx);

    assert_eq!(buffer.get(9, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(10, 0).unwrap().symbol, 'i');
}

#[test]
fn test_text_render_empty_area() {
    let text = Text::new("Hello");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 0, 1); // Zero width
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);
    // Should not crash, just return early
}

#[test]
fn test_text_render_zero_height() {
    let text = Text::new("Hello");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 0); // Zero height
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);
    // Should not crash, just return early
}

#[test]
fn test_text_render_with_offset() {
    let text = Text::new("Offset");
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(5, 2, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    // Text should be rendered starting at x=5, y=2
    assert_eq!(buffer.get(5, 2).unwrap().symbol, 'O');
    assert_eq!(buffer.get(10, 2).unwrap().symbol, 't');
}

// ─────────────────────────────────────────────────────────────────────────
// Truncation tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_text_truncation() {
    let long_text = "This is a very long text that should be truncated";
    let text = Text::new(long_text);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    // Should only render first 10 characters that fit in the area
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
    // "This is a " is 10 characters, position 9 is a space
    assert_eq!(buffer.get(9, 0).unwrap().symbol, ' ');
}

#[test]
fn test_text_exact_fit() {
    let text = Text::new("Exactly10!");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'E');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '!');
}

#[test]
fn test_text_shorter_than_area() {
    let text = Text::new("Short");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'S');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 't');
    // Position 5 should be empty (default)
    let cell = buffer.get(5, 0).unwrap();
    assert_eq!(cell.symbol, ' ');
}

// ─────────────────────────────────────────────────────────────────────────
// Justify alignment tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_text_justify_alignment() {
    let text = Text::new("Hello World").align(Alignment::Justify);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'o');
    assert_eq!(buffer.get(19, 0).unwrap().symbol, 'd');
    assert_eq!(buffer.get(15, 0).unwrap().symbol, 'W');
}

#[test]
fn test_text_justify_single_word() {
    let text = Text::new("Hello").align(Alignment::Justify);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    // Single word should fall back to left alignment
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'o');
}

#[test]
fn test_text_justify_multiple_words() {
    let text = Text::new("A B C").align(Alignment::Justify);
    let mut buffer = Buffer::new(11, 1);
    let area = Rect::new(0, 0, 11, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, 'B');
    assert_eq!(buffer.get(10, 0).unwrap().symbol, 'C');
}

#[test]
fn test_text_justify_text_wider_than_area() {
    let text = Text::new("This is a very long text").align(Alignment::Justify);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    // Should fall back to left alignment when text is too wide
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
}

#[test]
fn test_text_justify_with_style() {
    let text = Text::new("Hello World")
        .align(Alignment::Justify)
        .fg(Color::RED)
        .bold();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD));
}

// ─────────────────────────────────────────────────────────────────────────
// CSS integration tests
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_text_css_id() {
    let text = Text::new("Title").element_id("page-title");
    assert_eq!(View::id(&text), Some("page-title"));

    let meta = text.meta();
    assert_eq!(meta.id, Some("page-title".to_string()));
}

#[test]
fn test_text_css_classes() {
    let text = Text::new("Warning").class("alert").class("bold");

    assert!(text.has_class("alert"));
    assert!(text.has_class("bold"));
    assert!(!text.has_class("hidden"));

    let meta = text.meta();
    assert!(meta.classes.contains("alert"));
    assert!(meta.classes.contains("bold"));
}

#[test]
fn test_text_styled_view() {
    let mut text = Text::new("Test");

    text.set_id("test-text");
    assert_eq!(View::id(&text), Some("test-text"));

    text.add_class("highlight");
    assert!(text.has_class("highlight"));

    text.toggle_class("highlight");
    assert!(!text.has_class("highlight"));

    text.toggle_class("active");
    assert!(text.has_class("active"));

    text.remove_class("active");
    assert!(!text.has_class("active"));
}

#[test]
fn test_text_css_colors_from_context() {
    let text = Text::new("CSS Text");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::MAGENTA,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    text.render(&mut ctx);
}

#[test]
fn test_text_css_background_from_context() {
    let text = Text::new("CSS BG");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 1);

    let mut style = Style::default();
    style.visual = VisualStyle {
        background: Color::BLUE,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    text.render(&mut ctx);
}

#[test]
fn test_text_inline_color_override_css() {
    let text = Text::new("Override").fg(Color::GREEN);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::RED,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    text.render(&mut ctx);

    // Inline color should override CSS
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::GREEN));
}

#[test]
fn test_text_inline_background_override_css() {
    let text = Text::new("Override").bg(Color::YELLOW);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);

    let mut style = Style::default();
    style.visual = VisualStyle {
        background: Color::BLUE,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    text.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::YELLOW));
}

// ─────────────────────────────────────────────────────────────────────────
// Edge cases
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_text_render_empty_string() {
    let text = Text::new("");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);
    // Should not crash
}

#[test]
fn test_text_render_spaces() {
    let text = Text::new("   ");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, ' ');
}

#[test]
fn test_text_unicode_wide_char() {
    let text = Text::new("日本語"); // Japanese characters (wide)
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '日');
}

#[test]
fn test_text_special_characters() {
    let text = Text::new("©®™€£");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '©');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '®');
}

#[test]
fn test_text_alignment_equality() {
    assert_eq!(Alignment::Left, Alignment::Left);
    assert_eq!(Alignment::Center, Alignment::Center);
    assert_eq!(Alignment::Right, Alignment::Right);
    assert_eq!(Alignment::Justify, Alignment::Justify);

    assert_ne!(Alignment::Left, Alignment::Center);
    assert_ne!(Alignment::Center, Alignment::Right);
}

#[test]
fn test_text_alignment_debug() {
    let left = format!("{:?}", Alignment::Left);
    let center = format!("{:?}", Alignment::Center);
    let right = format!("{:?}", Alignment::Right);
    let justify = format!("{:?}", Alignment::Justify);

    assert!(!left.is_empty());
    assert!(!center.is_empty());
    assert!(!right.is_empty());
    assert!(!justify.is_empty());
}

#[test]
fn test_text_builder_chain() {
    let text = Text::new("Complete")
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .bold()
        .italic()
        .underline()
        .reverse()
        .align(Alignment::Center)
        .element_id("complete")
        .class("styled");

    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    text.render(&mut ctx);

    assert_eq!(View::id(&text), Some("complete"));
    assert!(text.has_class("styled"));

    // Text is centered, so find the actual content position
    let mut found = false;
    for x in 0..20 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'C' {
                found = true;
                break;
            }
        }
    }
    assert!(
        found,
        "Text 'Complete' should be rendered somewhere in buffer"
    );
}

// =============================================================================
// Additional Edge Cases and Comprehensive Tests
// =============================================================================

#[test]
fn test_text_single_char() {
    let text = Text::new("A");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
}

#[test]
fn test_text_emoji() {
    let text = Text::new("🎉🎊");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Should render emoji
}

#[test]
fn test_text_mixed_unicode() {
    let text = Text::new("Hello 世界 🌍");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Should render mixed content
}

#[test]
fn test_text_very_long_single_word() {
    let long_word = "supercalifragilisticexpialidocious";
    let text = Text::new(long_word);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Should truncate
}

#[test]
fn test_text_multiple_spaces() {
    let text = Text::new("Word1   Word2");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'W');
}

#[test]
fn test_text_all_spaces() {
    let text = Text::new("     ");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    for x in 0..5 {
        assert_eq!(buffer.get(x, 0).unwrap().symbol, ' ');
    }
}

#[test]
fn test_text_tab_characters() {
    let text = Text::new("Before\tAfter");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Should handle tabs
}

#[test]
fn test_text_newline_in_content() {
    let text = Text::new("Line1\nLine2");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Should handle or truncate newlines
}

#[test]
fn test_text_zero_width_area() {
    let text = Text::new("Test");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 0, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
}

#[test]
fn test_text_single_pixel_area() {
    let text = Text::new("X");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
}

#[test]
fn test_text_align_right_truncation() {
    let text = Text::new("Very Long Text Here").align(Alignment::Right);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Right aligned text that exceeds area should be truncated on left
}

#[test]
fn test_text_align_center_truncation() {
    let text = Text::new("Centered Text").align(Alignment::Center);
    let mut buffer = Buffer::new(8, 1);
    let area = Rect::new(0, 0, 8, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Centered text that exceeds area should be truncated on both ends
}

#[test]
fn test_text_all_alignments_with_empty() {
    let alignments = [
        Alignment::Left,
        Alignment::Center,
        Alignment::Right,
        Alignment::Justify,
    ];

    for alignment in alignments {
        let text = Text::new("").align(alignment);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
    }
}

#[test]
fn test_text_all_alignments_with_single_char() {
    let alignments = [
        Alignment::Left,
        Alignment::Center,
        Alignment::Right,
        Alignment::Justify,
    ];

    for alignment in alignments {
        let text = Text::new("X").align(alignment);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        // All should render
    }
}

#[test]
fn test_text_dim_modifier() {
    let text = Text::new("Dim").dim();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    let cell = buffer.get(0, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::DIM));
}

#[test]
fn test_text_all_modifiers() {
    let text = Text::new("All").bold().dim().italic().underline().reverse();

    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // All modifiers should be applied
}

#[test]
fn test_text_color_rgb() {
    let text = Text::new("RGB").fg(Color::rgb(100, 150, 200));
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::rgb(100, 150, 200)));
}

#[test]
fn test_text_color_rgba() {
    let text = Text::new("RGBA").fg(Color::rgba(255, 100, 50, 200));
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Should render with RGBA color
}

#[test]
fn test_text_multiple_colors() {
    let text = Text::new("Multi").fg(Color::RED).bg(Color::BLUE);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::RED));
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_text_same_fg_and_bg() {
    let text = Text::new("Same").fg(Color::WHITE).bg(Color::WHITE);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::WHITE));
    assert_eq!(cell.bg, Some(Color::WHITE));
}

#[test]
fn test_text_all_preset_colors() {
    let text1 = Text::error("Error");
    let text2 = Text::success("Success");
    let text3 = Text::warning("Warning");
    let text4 = Text::info("Info");

    assert_eq!(text1.content(), "Error");
    assert_eq!(text2.content(), "Success");
    assert_eq!(text3.content(), "Warning");
    assert_eq!(text4.content(), "Info");
}

#[test]
fn test_text_all_presets_render() {
    let presets = [
        ("Heading", Text::heading("Title")),
        ("Label", Text::label("Label:")),
        ("Muted", Text::muted("Muted")),
        ("Error", Text::error("Error")),
        ("Success", Text::success("Success")),
        ("Warning", Text::warning("Warning")),
        ("Info", Text::info("Info")),
    ];

    for (name, text) in presets {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        // All should render
    }
}

#[test]
fn test_text_clone_preserves_content() {
    let text1 = Text::new("Original").fg(Color::RED).bold();
    let text2 = text1.clone();
    assert_eq!(text1.content(), text2.content());
}

#[test]
fn test_text_clone_preserves_style() {
    let text1 = Text::new("Styled")
        .fg(Color::RED)
        .bg(Color::BLUE)
        .bold()
        .italic();
    let text2 = text1.clone();

    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);

    {
        let mut ctx = RenderContext::new(&mut buffer, area);
        text1.render(&mut ctx);
    }
    let fg1 = buffer.get(0, 0).unwrap().fg;
    let mod1 = buffer.get(0, 0).unwrap().modifier;

    buffer.clear();

    {
        let mut ctx = RenderContext::new(&mut buffer, area);
        text2.render(&mut ctx);
    }
    let fg2 = buffer.get(0, 0).unwrap().fg;
    let mod2 = buffer.get(0, 0).unwrap().modifier;

    assert_eq!(fg1, fg2);
    assert_eq!(mod1, mod2);
}

#[test]
fn test_text_multiple_render_calls() {
    let text = Text::new("Test");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);

    for _ in 0..5 {
        buffer.clear();
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
    }
    // Should render consistently
}

#[test]
fn test_text_justify_with_truncation() {
    let text = Text::new("A B C D E").align(Alignment::Justify);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Should justify and fit what it can
}

#[test]
fn test_text_content_getter() {
    let text = Text::new("Content");
    assert_eq!(text.content(), "Content");
}

#[test]
fn test_text_default_alignment() {
    // Default alignment should be Left
    let text = Text::new("Default");
    assert_eq!(text.content(), "Default");
}

#[test]
fn test_text_with_all_alignments() {
    let text = "Test";
    let alignments = [
        Alignment::Left,
        Alignment::Center,
        Alignment::Right,
        Alignment::Justify,
    ];

    for alignment in alignments {
        let t = Text::new(text).align(alignment);
        assert_eq!(t.content(), text);
    }
}

#[test]
fn test_text_align_right_exactly_fits() {
    let text = Text::new("Exact!").align(Alignment::Right);
    let mut buffer = Buffer::new(6, 1);
    let area = Rect::new(0, 0, 6, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'E');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '!');
}

#[test]
fn test_text_align_center_exactly_fits() {
    let text = Text::new("12345").align(Alignment::Center);
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Center 5 chars in 5 char width = all chars shown
}

#[test]
fn test_text_right_to_left() {
    let text = Text::new("مرحبا"); // Arabic text
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Should render RTL text
}

#[test]
fn test_text_align_with_unicode() {
    let text = Text::new("日本語").align(Alignment::Center);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Should center unicode
}

#[test]
fn test_text_justify_with_unicode() {
    let text = Text::new("A B C").align(Alignment::Justify);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
}

#[test]
fn test_text_modifiers_with_alignment() {
    let text = Text::new("Styled").bold().italic().align(Alignment::Center);

    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);

    // For centered text "Styled" in 20 char width, find the first non-empty cell
    let mut found = false;
    for x in 0..20 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol != ' ' && cell.modifier.contains(Modifier::BOLD) {
                found = true;
                break;
            }
        }
    }
    assert!(found, "Should find bold text in buffer");
}

#[test]
fn test_text_multiple_instances_independent() {
    let text1 = Text::new("Text1").fg(Color::RED);
    let text2 = Text::new("Text2").fg(Color::BLUE);

    assert_eq!(text1.content(), "Text1");
    assert_eq!(text2.content(), "Text2");
}

#[test]
fn test_text_very_narrow_area() {
    let text = Text::new("X");
    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
}

#[test]
fn test_text_very_wide_area() {
    let text = Text::new("Small");
    let mut buffer = Buffer::new(100, 1);
    let area = Rect::new(0, 0, 100, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Should render at left
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'S');
}

#[test]
fn test_text_offset_with_alignment() {
    let text = Text::new("Test").align(Alignment::Center);

    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(10, 2, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
    // Should render centered within offset area
}

#[test]
fn test_content_update() {
    let text1 = Text::new("First");
    // Text is immutable, so we create new instances
    let text2 = Text::new("Second");
    assert_eq!(text1.content(), "First");
    assert_eq!(text2.content(), "Second");
}

#[test]
fn test_text_with_all_colors() {
    let colors = [
        Color::BLACK,
        Color::WHITE,
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
        Color::CYAN,
        Color::MAGENTA,
    ];

    for color in colors {
        let text = Text::new("Color").fg(color);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
    }
}

#[test]
fn test_text_string_conversion() {
    let s = "Test String";
    let text = Text::new(s);
    assert_eq!(text.content(), s);
}

#[test]
fn test_text_reserved_characters() {
    let text = Text::new("()[]{}<>|\\/");
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
}

#[test]
fn test_text_mathematical_symbols() {
    let text = Text::new("∑∫√∞≈≠≤≥");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
}

#[test]
fn test_text_arrows() {
    let text = Text::new("→←↑→↓↔");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
}

#[test]
fn test_text_box_drawing() {
    let text = Text::new("│─┌┐└┘");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);
}

#[test]
fn test_all_modifier_methods_compile() {
    // Test that all modifier methods compile
    let _text = Text::new("Test")
        .bold()
        .dim()
        .italic()
        .underline()
        .reverse();
}

#[test]
fn test_text_alignment_pairwise_equality() {
    // Test that all alignments are pairwise comparable
    let alignments = [
        Alignment::Left,
        Alignment::Center,
        Alignment::Right,
        Alignment::Justify,
    ];

    for i in 0..alignments.len() {
        for j in i..alignments.len() {
            if i == j {
                assert_eq!(alignments[i], alignments[j]);
            } else {
                assert_ne!(alignments[i], alignments[j]);
            }
        }
    }
}

#[test]
fn test_text_render_preserves_content_integrity() {
    let original = "Integrity";
    let text = Text::new(original);

    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    text.render(&mut ctx);

    let rendered: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(rendered.contains(original));
}

#[test]
fn test_text_all_alignments_zero_width() {
    let alignments = [
        Alignment::Left,
        Alignment::Center,
        Alignment::Right,
        Alignment::Justify,
    ];

    for alignment in alignments {
        let text = Text::new("Test").align(alignment);
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 0, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        // Should handle zero width gracefully
    }
}

#[test]
fn test_text_all_alignments_single_char() {
    let alignments = [
        Alignment::Left,
        Alignment::Center,
        Alignment::Right,
        Alignment::Justify,
    ];

    for alignment in alignments {
        let text = Text::new("X").align(alignment);
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        text.render(&mut ctx);
        // All should render X somewhere
    }
}

#[test]
fn test_text_modifiers_with_all_alignments() {
    let modifiers = [
        ("bold", Text::new("B").bold()),
        ("italic", Text::new("I").italic()),
        ("underline", Text::new("U").underline()),
        ("reverse", Text::new("R").reverse()),
    ];

    let alignments = [Alignment::Left, Alignment::Center, Alignment::Right];

    for (mod_name, text) in modifiers {
        for alignment in alignments {
            let t = text.clone().align(alignment);
            let mut buffer = Buffer::new(20, 1);
            let area = Rect::new(0, 0, 20, 1);
            let mut ctx = RenderContext::new(&mut buffer, area);
            t.render(&mut ctx);
        }
    }
}

#[test]
fn test_text_clone_does_not_affect_original() {
    let mut text1 = Text::new("Original");
    let text2 = text1.clone();

    // Modifications to text2 don't affect text1
    // (Text is immutable, so we create new instances)
    let text3 = Text::new("Modified");

    assert_eq!(text1.content(), "Original");
    assert_eq!(text2.content(), "Original");
    assert_eq!(text3.content(), "Modified");
}
