//! RenderContext segment drawing tests
//!
//! Two-segment draw_segments/draw_segments_sep, draw_text_selectable and
//! the three metric_color bands are covered by the in-source tests in
//! src/widget/traits/render_context/tests.rs. These cover empty input,
//! offsets, wide text, multi-character separators, draw_key_hints and the
//! metric_color boundaries.

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::RenderContext;

const SEP: Color = Color::rgb(100, 100, 100);

fn row(buffer: &Buffer) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, 0).unwrap().symbol)
        .collect()
}

fn draw(width: u16, f: impl FnOnce(&mut RenderContext) -> u16) -> (Buffer, u16) {
    let mut buffer = Buffer::new(width, 1);
    let end = {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, 1));
        f(&mut ctx)
    };
    (buffer, end)
}

// =========================================================================
// draw_segments / draw_segments_sep
// =========================================================================

#[test]
fn test_draw_segments_empty_returns_start() {
    let (buffer, end) = draw(10, |ctx| ctx.draw_segments(3, 0, &[]));
    assert_eq!(end, 3);
    assert_eq!(row(&buffer), "          ");
}

#[test]
fn test_draw_segments_offset_and_colors() {
    let (buffer, end) = draw(12, |ctx| {
        ctx.draw_segments(5, 0, &[("Te", Color::RED), ("st", Color::CYAN)])
    });
    assert_eq!(end, 9);
    assert_eq!(row(&buffer), "     Test   ");
    assert_eq!(buffer.get(6, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(7, 0).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_draw_segments_wide_text_advances_by_display_width() {
    let (buffer, end) = draw(10, |ctx| {
        ctx.draw_segments(0, 0, &[("你好", Color::WHITE), ("!", Color::WHITE)])
    });
    assert_eq!(end, 5);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '你');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '好');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '!');
}

#[test]
fn test_draw_segments_sep_empty_and_single_have_no_separator() {
    let (buffer, end) = draw(10, |ctx| ctx.draw_segments_sep(0, 0, &[], " | ", SEP));
    assert_eq!(end, 0);
    assert_eq!(row(&buffer), "          ");

    let (buffer, end) = draw(10, |ctx| {
        ctx.draw_segments_sep(0, 0, &[("Hello", Color::WHITE)], " | ", SEP)
    });
    assert_eq!(end, 5);
    assert_eq!(row(&buffer), "Hello     ");
}

#[test]
fn test_draw_segments_sep_multi_char_separator() {
    let (buffer, end) = draw(12, |ctx| {
        ctx.draw_segments_sep(
            0,
            0,
            &[("A", Color::WHITE), ("B", Color::CYAN), ("C", Color::RED)],
            " | ",
            SEP,
        )
    });
    assert_eq!(end, 9);
    assert_eq!(row(&buffer), "A | B | C   ");
    assert_eq!(buffer.get(2, 0).unwrap().fg, Some(SEP));
    assert_eq!(buffer.get(4, 0).unwrap().fg, Some(Color::CYAN));
}

// =========================================================================
// draw_key_hints
// =========================================================================

#[test]
fn test_draw_key_hints_empty() {
    let (buffer, end) = draw(10, |ctx| {
        ctx.draw_key_hints(2, 0, &[], Color::CYAN, Color::WHITE)
    });
    assert_eq!(end, 2);
    assert_eq!(row(&buffer), "          ");
}

#[test]
fn test_draw_key_hints_layout_and_styles() {
    let (buffer, end) = draw(20, |ctx| {
        ctx.draw_key_hints(
            0,
            0,
            &[("q", "quit"), ("Ctrl", "save")],
            Color::CYAN,
            Color::WHITE,
        )
    });
    // "q quit" + 2 spaces, then "Ctrl save" + 2 spaces
    assert_eq!(end, 8 + 11);
    assert_eq!(row(&buffer), "q quit  Ctrl save   ");

    let key = buffer.get(0, 0).unwrap();
    assert_eq!(key.fg, Some(Color::CYAN));
    assert!(key.modifier.contains(Modifier::BOLD));

    let action = buffer.get(2, 0).unwrap();
    assert_eq!(action.fg, Some(Color::WHITE));
    assert!(!action.modifier.contains(Modifier::BOLD));

    assert!(buffer.get(11, 0).unwrap().modifier.contains(Modifier::BOLD));
}

// =========================================================================
// metric_color boundaries
// =========================================================================

fn metric(value: u8, mid: u8, high: u8) -> Color {
    RenderContext::metric_color(value, mid, high, Color::GREEN, Color::YELLOW, Color::RED)
}

#[test]
fn test_metric_color_boundaries() {
    assert_eq!(metric(0, 50, 80), Color::GREEN);
    assert_eq!(metric(49, 50, 80), Color::GREEN);
    // Thresholds are inclusive lower bounds
    assert_eq!(metric(50, 50, 80), Color::YELLOW);
    assert_eq!(metric(79, 50, 80), Color::YELLOW);
    assert_eq!(metric(80, 50, 80), Color::RED);
    assert_eq!(metric(255, 50, 80), Color::RED);
}

#[test]
fn test_metric_color_equal_thresholds_skip_mid() {
    assert_eq!(metric(49, 50, 50), Color::GREEN);
    assert_eq!(metric(50, 50, 50), Color::RED);
}
