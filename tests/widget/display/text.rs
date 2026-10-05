//! Text widget tests extracted from src/widget/display/text.rs
//!
//! The content round-trip cases live in tests/text_tests.rs; these check
//! what the builders do to the rendered cells.

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{Alignment, Text};

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
