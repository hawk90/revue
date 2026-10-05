//! Rendering tests for the Input widget

use revue::event::{Key, KeyEvent};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{input, Input, View};

fn render_in(i: &Input, buffer: &mut Buffer, area: Rect) {
    let mut ctx = RenderContext::new(buffer, area);
    i.render(&mut ctx);
}

fn render(i: &Input, width: u16) -> Buffer {
    let mut b = Buffer::new(width, 1);
    render_in(i, &mut b, Rect::new(0, 0, width, 1));
    b
}

fn text(b: &Buffer, width: u16) -> String {
    (0..width)
        .map(|x| b.get(x, 0).map(|c| c.symbol).unwrap_or(' '))
        .collect::<String>()
        .trim_end()
        .to_string()
}

// =========================================================================
// Basic rendering tests
// =========================================================================

#[test]
fn test_input_render_empty_unfocused() {
    let i = input().focused(false);
    let b = render(&i, 30);
    assert_eq!(text(&b, 30), "");
    assert_eq!(*b.get(0, 0).unwrap(), Default::default());
}

#[test]
fn test_input_render_empty_focused_draws_cursor() {
    // Input::new() starts focused, so the cursor is drawn at column 0.
    let b = render(&input(), 30);
    let cell = b.get(0, 0).unwrap();
    assert_eq!(cell.symbol, ' ');
    assert_eq!(cell.fg, Some(Color::BLACK));
    assert_eq!(cell.bg, Some(Color::WHITE));
}

#[test]
fn test_input_render_with_text() {
    let i = input().value("Hello").focused(false);
    assert_eq!(text(&render(&i, 30), 30), "Hello");
}

#[test]
fn test_input_render_with_placeholder() {
    let i = input().placeholder("Enter text...").focused(false);
    let b = render(&i, 30);
    assert_eq!(text(&b, 30), "Enter text...");
    // The placeholder is drawn dimmed, not in the text color.
    assert!(b.get(0, 0).unwrap().fg.is_some());
}

#[test]
fn test_input_render_placeholder_with_string() {
    let i = input().placeholder(String::from("custom")).focused(false);
    assert_eq!(text(&render(&i, 30), 30), "custom");
}

#[test]
fn test_input_render_placeholder_shown_when_focused() {
    // An empty input shows its placeholder whether or not it is focused;
    // the cursor is drawn over the first placeholder cell.
    let i = input().placeholder("Enter text...").focused(true);
    assert_eq!(text(&render(&i, 30), 30), "Enter text...");
}

#[test]
fn test_input_render_placeholder_hidden_with_value() {
    let i = input()
        .placeholder("Enter text...")
        .value("hi")
        .focused(false);
    assert_eq!(text(&render(&i, 30), 30), "hi");
}

#[test]
fn test_input_render_focused_cursor_at_end() {
    let i = input().value("test").focused(true);
    let b = render(&i, 30);
    assert_eq!(text(&b, 30), "test");
    assert_eq!(b.get(4, 0).unwrap().bg, Some(Color::WHITE));
    assert_ne!(b.get(3, 0).unwrap().bg, Some(Color::WHITE));
}

#[test]
fn test_input_render_focused_cursor_mid_text() {
    let mut i = input().value("test").focused(true);
    i.handle_key(&Key::Left);
    let b = render(&i, 30);
    let cursor = b.get(3, 0).unwrap();
    assert_eq!(cursor.symbol, 't');
    assert_eq!(cursor.bg, Some(Color::WHITE));
    assert_ne!(b.get(4, 0).unwrap().bg, Some(Color::WHITE));
}

#[test]
fn test_input_render_unfocused_has_no_cursor() {
    let i = input().value("test").focused(false);
    let b = render(&i, 30);
    for x in 0..30 {
        assert_ne!(b.get(x, 0).unwrap().bg, Some(Color::WHITE));
    }
}

#[test]
fn test_input_render_with_colors() {
    let i = input()
        .value("colored")
        .fg(Color::RED)
        .bg(Color::BLUE)
        .focused(false);
    let cell = *render(&i, 30).get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'c');
    assert_eq!(cell.fg, Some(Color::RED));
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_input_render_last_color_builder_wins() {
    let i = input()
        .value("x")
        .fg(Color::RED)
        .fg(Color::BLUE)
        .bg(Color::GREEN)
        .focused(false);
    let cell = *render(&i, 5).get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::BLUE));
    assert_eq!(cell.bg, Some(Color::GREEN));
}

#[test]
fn test_input_render_with_selection() {
    let mut i = input().value("hello world").selection_bg(Color::GREEN);
    // Select "hello" by moving to the start and shift-selecting five chars.
    i.handle_key(&Key::Home);
    for _ in 0..5 {
        i.handle_key_event(&KeyEvent {
            shift: true,
            ..KeyEvent::new(Key::Right)
        });
    }
    assert_eq!(i.selected_text(), Some("hello"));
    let b = render(&i, 30);
    for x in 0..5 {
        let cell = b.get(x, 0).unwrap();
        assert_eq!(cell.bg, Some(Color::GREEN), "column {x}");
        assert_eq!(cell.fg, Some(Color::WHITE), "column {x}");
    }
    // The cursor sits after the selection, then unselected text.
    assert_eq!(b.get(5, 0).unwrap().bg, Some(Color::WHITE));
    assert_ne!(b.get(6, 0).unwrap().bg, Some(Color::GREEN));
}

#[test]
fn test_input_render_with_unicode() {
    let i = input().value("안녕🎉世界").focused(false);
    let b = render(&i, 30);
    // Wide characters take two columns each.
    assert_eq!(b.get(0, 0).unwrap().symbol, '안');
    assert_eq!(b.get(2, 0).unwrap().symbol, '녕');
    assert_eq!(b.get(4, 0).unwrap().symbol, '🎉');
    assert_eq!(b.get(6, 0).unwrap().symbol, '世');
    assert_eq!(b.get(8, 0).unwrap().symbol, '界');
}

#[test]
fn test_input_render_small_area() {
    let i = input().value("test").focused(false);
    let mut b = Buffer::new(5, 1);
    render_in(&i, &mut b, Rect::new(0, 0, 3, 1));
    // The cursor sits after "test"; three columns show the tail plus the
    // cursor cell, and nothing is drawn outside the area.
    assert_eq!(text(&b, 5), "st");
}

#[test]
fn test_input_render_zero_width_area() {
    let i = input().value("test");
    let mut b = Buffer::new(5, 1);
    render_in(&i, &mut b, Rect::new(0, 0, 0, 1));
    assert_eq!(text(&b, 5), "");
}

#[test]
fn test_input_render_zero_height_area() {
    let i = input().value("test");
    let mut b = Buffer::new(10, 1);
    render_in(&i, &mut b, Rect::new(0, 0, 10, 0));
    assert_eq!(text(&b, 10), "");
}

#[test]
fn test_input_render_long_text_scrolls_to_cursor() {
    // value() leaves the cursor at the end, and the line scrolls so the
    // cursor stays in view: the tail of the text is shown, in the last
    // columns before the cursor cell, and nothing is drawn past the width.
    let i = input()
        .value("This is a very long text that should be truncated")
        .focused(false);
    let b = render(&i, 10);
    assert_eq!(text(&b, 10), "truncated");
    assert_eq!(b.get(8, 0).unwrap().symbol, 'd');
    assert_eq!(b.get(9, 0).unwrap().symbol, ' ');
}

#[test]
fn test_input_render_wide_char_does_not_split_at_edge() {
    // "a한" is three columns plus the cursor cell. In two columns the line
    // scrolls to the cursor, which leaves only the right half of "한" in
    // view; a wide glyph is never split, so it is dropped rather than drawn
    // as a stray half cell.
    let i = input().value("a한").focused(false);
    assert_eq!(text(&render(&i, 2), 2), "");
    // With room for the glyph and the cursor, it is drawn whole: the glyph
    // in column 0 and its continuation cell in column 1.
    let b = render(&i, 3);
    assert_eq!(b.get(0, 0).unwrap().symbol, '한');
    assert!(b.get(1, 0).unwrap().is_continuation());
}

#[test]
fn test_input_render_with_custom_cursor_style() {
    let i = input()
        .value("test")
        .cursor_style(Color::YELLOW, Color::BLACK)
        .focused(true);
    let cell = *render(&i, 30).get(4, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::YELLOW));
    assert_eq!(cell.bg, Some(Color::BLACK));
}

#[test]
fn test_input_render_offset_position() {
    let i = input().value("test").focused(false);
    let mut b = Buffer::new(50, 10);
    render_in(&i, &mut b, Rect::new(10, 5, 30, 1));
    assert_eq!(b.get(10, 5).unwrap().symbol, 't');
    assert_eq!(b.get(13, 5).unwrap().symbol, 't');
    assert_eq!(b.get(9, 5).unwrap().symbol, ' ');
    assert_eq!(b.get(10, 4).unwrap().symbol, ' ');
}
