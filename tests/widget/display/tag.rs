//! Tag widget tests

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{chip, tag, Tag, TagStyle, DARK_GRAY, SEPARATOR_COLOR, SUBTLE_GRAY};

fn render_in(t: &Tag, width: u16) -> Buffer {
    let mut buffer = Buffer::new(20, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, 1));
    t.render(&mut ctx);
    buffer
}

fn render(t: &Tag) -> Buffer {
    render_in(t, 20)
}

fn text(buffer: &Buffer) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, 0).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// Background of the first text cell.
fn bg(buffer: &Buffer) -> Option<Color> {
    buffer.get(1, 0).unwrap().bg
}

// =========================================================================
// TagStyle enum tests
// =========================================================================

#[test]
fn test_tag_style_default() {
    assert_eq!(TagStyle::default(), TagStyle::Filled);
}

#[test]
fn test_tag_style_debug() {
    assert_eq!(format!("{:?}", TagStyle::Subtle), "Subtle");
}

// =========================================================================
// Rendering of the builders
// =========================================================================

#[test]
fn test_tag_filled_default() {
    let buffer = render(&tag("Test"));
    assert_eq!(text(&buffer), " Test");
    assert_eq!(bg(&buffer), Some(DARK_GRAY));
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(Color::WHITE));
}

#[test]
fn test_tag_outlined() {
    let buffer = render(&tag("Test").outlined().blue());
    assert_eq!(text(&buffer), "⟨Test⟩");
    assert_eq!(bg(&buffer), None);
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(Color::rgb(60, 120, 200)));
}

#[test]
fn test_tag_subtle() {
    let buffer = render(&tag("Test").subtle().red());
    // Text in the tag color on a lightened background
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(Color::rgb(200, 60, 60)));
    assert_eq!(bg(&buffer), Some(Color::rgb(255, 240, 240)));
}

#[test]
fn test_tag_color_presets() {
    let cases = [
        (tag("Test").blue(), Color::rgb(60, 120, 200)),
        (tag("Test").green(), Color::rgb(40, 160, 80)),
        (tag("Test").red(), Color::rgb(200, 60, 60)),
        (tag("Test").yellow(), Color::rgb(200, 180, 40)),
        (tag("Test").purple(), Color::rgb(140, 80, 180)),
    ];
    for (t, expected) in cases {
        assert_eq!(bg(&render(&t)), Some(expected));
    }
}

#[test]
fn test_tag_custom_colors() {
    let t = Tag::new("Test")
        .style(TagStyle::Filled)
        .color(Color::RED)
        .text_color(Color::BLACK);
    let buffer = render(&t);
    assert_eq!(bg(&buffer), Some(Color::RED));
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(Color::BLACK));
}

#[test]
fn test_tag_closable() {
    assert_eq!(text(&render(&tag("Test").closable())), " Test ×");
}

#[test]
fn test_tag_icon() {
    assert_eq!(text(&render(&tag("Rust").icon('R'))), " R Rust");
}

#[test]
fn test_tag_selected() {
    let buffer = render(&tag("Test").selected());
    assert!(buffer.get(1, 0).unwrap().modifier.contains(Modifier::BOLD));
    assert!(!render(&tag("Test"))
        .get(1, 0)
        .unwrap()
        .modifier
        .contains(Modifier::BOLD));
}

#[test]
fn test_tag_disabled() {
    let buffer = render(&tag("Test").blue().disabled());
    let cell = buffer.get(1, 0).unwrap();
    assert!(cell.modifier.contains(Modifier::DIM));
    // Disabled overrides the tag colors
    assert_eq!(cell.bg, Some(SEPARATOR_COLOR));
    assert_eq!(cell.fg, Some(SUBTLE_GRAY));
}

#[test]
fn test_helper_functions() {
    assert_eq!(text(&render(&tag("A"))), text(&render(&Tag::new("A"))));
    assert_eq!(text(&render(&chip("B"))), text(&render(&Tag::new("B"))));
}

#[test]
fn test_tag_default() {
    let buffer = render(&Tag::default());
    // Empty text: just the two padding cells
    assert_eq!(text(&buffer), "");
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(DARK_GRAY));
    assert_eq!(buffer.get(1, 0).unwrap().bg, Some(DARK_GRAY));
    assert_eq!(buffer.get(2, 0).unwrap().bg, None);
}

#[test]
fn test_tag_builder_chain() {
    let t = Tag::new("Chained")
        .blue()
        .outlined()
        .closable()
        .icon('T')
        .selected();
    let buffer = render(&t);
    assert_eq!(text(&buffer), "⟨T Chained ×⟩");
    assert!(buffer.get(1, 0).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_tag_truncated_to_area() {
    let buffer = render_in(&tag("A long tag text"), 6);
    assert_eq!(text(&buffer), " A lo");
}

#[test]
fn test_tag_zero_width_area() {
    let buffer = render_in(&tag("Test"), 0);
    assert_eq!(text(&buffer), "");
}
