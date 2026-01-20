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
    assert_eq!(buffer.get(9, 0).unwrap().symbol, 'y');
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

    let cell = buffer.get(0, 0).unwrap();
    // Should have modifiers applied
    assert!(cell.modifier.contains(Modifier::BOLD));
}
