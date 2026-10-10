//! Text widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::render::Modifier;
use revue::style::Color;
use revue::widget::text;
use revue::widget::traits::RenderContext;
use revue::widget::traits::View;
use revue::widget::Alignment;
use revue::widget::Text;

fn render(text: &Text, width: u16) -> Buffer {
    let mut buffer = Buffer::new(width, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, 1));
    text.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, 0).unwrap().symbol)
        .collect()
}

#[test]
fn test_text_all_alignments() {
    let cases = [
        (Alignment::Left, "Test      "),
        (Alignment::Center, "   Test   "),
        (Alignment::Right, "      Test"),
    ];
    for (align, expected) in cases {
        let text = Text::new("Test").align(align);
        assert_eq!(row(&render(&text, 10)), expected, "{:?}", align);
    }
}

#[test]
fn test_text_builder_chaining() {
    let text = Text::new("Test")
        .fg(Color::RED)
        .bg(Color::BLUE)
        .bold()
        .italic()
        .underline()
        .dim();
    assert_eq!(text.content(), "Test");

    let buffer = render(&text, 10);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'T');
    assert_eq!(cell.fg, Some(Color::RED));
    assert_eq!(cell.bg, Some(Color::BLUE));
    for m in [
        Modifier::BOLD,
        Modifier::ITALIC,
        Modifier::UNDERLINE,
        Modifier::DIM,
    ] {
        assert!(cell.modifier.contains(m), "{:?}", m);
    }
}

#[test]
fn test_text_plain_has_no_modifiers() {
    let buffer = render(&Text::new("Test"), 10);
    assert!(buffer.get(0, 0).unwrap().modifier.is_empty());
}

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

mod snapshots {
    use revue::prelude::*;
    use revue::testing::{Pilot, TestApp, TestConfig};

    #[test]
    fn test_text_simple() {
        let view = text("Hello, World!");
        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("text_simple");
    }

    #[test]
    fn test_text_multiline() {
        let view = vstack()
            .child(text("Line 1"))
            .child(text("Line 2"))
            .child(text("Line 3"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("text_multiline");
    }

    #[test]
    fn test_text_formatting() {
        let view = vstack()
            .child(Text::new("Normal text"))
            .child(Text::heading("Heading"))
            .child(Text::muted("Muted text"))
            .child(Text::success("Success!"))
            .child(Text::error("Error!"))
            .child(Text::info("Info"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("text_formatting");
    }

    #[test]
    fn test_text_alignment() {
        let config = TestConfig::with_size(40, 10);
        let view = vstack()
            .child(Text::new("Left aligned").align(Alignment::Left))
            .child(Text::new("Centered").align(Alignment::Center))
            .child(Text::new("Right aligned").align(Alignment::Right));

        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("text_alignment");
    }

    #[test]
    fn test_text_reverse() {
        let view = vstack()
            .child(Text::new("Normal text"))
            .child(Text::new("Reversed text").reverse())
            .child(Text::new("Bold + Reversed").bold().reverse());

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("text_reverse");
    }

    #[test]
    fn test_text_justify() {
        let config = TestConfig::with_size(30, 5);
        let view = vstack()
            .child(Text::new("Hello World Test").align(Alignment::Justify))
            .child(Text::new("A B C D E").align(Alignment::Justify))
            .child(Text::new("SingleWord").align(Alignment::Justify));

        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("text_justify");
    }
}
