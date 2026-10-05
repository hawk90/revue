//! RenderContext text drawing tests
//!
//! Basic ASCII drawing for each method (draw_char*, draw_text*, clipping
//! at max_width, centering/right-aligning short text, offsets) is covered
//! by the in-source tests in src/widget/traits/render_context/tests.rs.
//! These cover wide characters, empty input, the combined styles and text
//! that does not fit.

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::RenderContext;

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

fn draw(width: u16, f: impl FnOnce(&mut RenderContext)) -> Buffer {
    let mut buffer = Buffer::new(width, 2);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, 2));
        f(&mut ctx);
    }
    buffer
}

// =========================================================================
// Wide characters
// =========================================================================

#[test]
fn test_draw_char_wide() {
    let buffer = draw(4, |ctx| ctx.draw_char(0, 0, '你', Color::WHITE));
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '你');
}

#[test]
fn test_draw_text_wide_chars() {
    let buffer = draw(20, |ctx| ctx.draw_text(0, 0, "한글", Color::WHITE));
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '한');
    assert!(buffer.get(1, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '글');
    assert!(buffer.get(3, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(4, 0).unwrap().symbol, ' ');
}

#[test]
fn test_draw_text_mixed_width() {
    let buffer = draw(20, |ctx| ctx.draw_text(0, 0, "A한B", Color::WHITE));
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '한');
    assert!(buffer.get(2, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 'B');
}

#[test]
fn test_draw_text_stops_before_a_wide_char_that_does_not_fit() {
    let buffer = draw(2, |ctx| ctx.draw_text(0, 0, "A你B", Color::WHITE));
    assert_eq!(row(&buffer, 0), "A ");
    assert!(!buffer.get(1, 0).unwrap().is_continuation());
}

#[test]
fn test_draw_text_clipped_wide() {
    let buffer = draw(10, |ctx| {
        ctx.draw_text_clipped(0, 0, "你好世界", Color::WHITE, 5)
    });
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '你');
    assert!(buffer.get(1, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '好');
    assert!(buffer.get(3, 0).unwrap().is_continuation());
    // '世' would need columns 4-5 but max_width is 5
    assert_eq!(buffer.get(4, 0).unwrap().symbol, ' ');
}

#[test]
fn test_draw_text_centered_wide_chars() {
    let buffer = draw(10, |ctx| {
        ctx.draw_text_centered(0, 0, 10, "한글", Color::WHITE)
    });
    assert_eq!(buffer.get(3, 0).unwrap().symbol, '한');
    assert!(buffer.get(4, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '글');
    assert!(buffer.get(6, 0).unwrap().is_continuation());
}

#[test]
fn test_draw_text_right_wide_chars() {
    let buffer = draw(10, |ctx| {
        ctx.draw_text_right(0, 0, 10, "한글", Color::WHITE)
    });
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '한');
    assert!(buffer.get(7, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(8, 0).unwrap().symbol, '글');
    assert!(buffer.get(9, 0).unwrap().is_continuation());
}

// =========================================================================
// Empty text and rows outside the area
// =========================================================================

#[test]
fn test_empty_text_draws_nothing() {
    let buffer = draw(6, |ctx| {
        ctx.draw_text(0, 0, "", Color::WHITE);
        ctx.draw_text_bg(0, 0, "", Color::WHITE, Color::BLUE);
        ctx.draw_text_bold(0, 0, "", Color::WHITE);
        ctx.draw_text_bg_bold(0, 0, "", Color::WHITE, Color::BLUE);
        ctx.draw_text_dim(0, 0, "", Color::WHITE);
        ctx.draw_text_italic(0, 0, "", Color::WHITE);
        ctx.draw_text_underline(0, 0, "", Color::WHITE);
        ctx.draw_text_clipped(0, 0, "", Color::WHITE, 6);
        ctx.draw_text_centered(0, 0, 6, "", Color::WHITE);
        ctx.draw_text_right(0, 0, 6, "", Color::WHITE);
    });
    for x in 0..6 {
        let cell = buffer.get(x, 0).unwrap();
        assert_eq!(cell.symbol, ' ');
        assert!(cell.fg.is_none());
        assert!(cell.bg.is_none());
    }
}

#[test]
fn test_draw_text_below_the_area_draws_nothing() {
    let mut buffer = Buffer::new(6, 4);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 6, 2));
        ctx.draw_text(0, 2, "Hello", Color::WHITE);
    }
    assert_eq!(row(&buffer, 2), "      ");
}

// =========================================================================
// Combined styles
// =========================================================================

#[test]
fn test_draw_text_bg_bold() {
    let buffer = draw(6, |ctx| {
        ctx.draw_text_bg_bold(0, 0, "Hi", Color::WHITE, Color::BLUE)
    });
    assert_eq!(row(&buffer, 0), "Hi    ");
    for x in 0..2 {
        let cell = buffer.get(x, 0).unwrap();
        assert_eq!(cell.fg, Some(Color::WHITE));
        assert_eq!(cell.bg, Some(Color::BLUE));
        assert!(cell.modifier.contains(Modifier::BOLD));
    }
}

#[test]
fn test_draw_text_clipped_bold_truncates() {
    let buffer = draw(12, |ctx| {
        ctx.draw_text_clipped_bold(0, 0, "Hello World", Color::WHITE, 5)
    });
    assert_eq!(row(&buffer, 0), "Hello       ");
    assert!(buffer.get(4, 0).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_draw_text_clipped_bg_variants() {
    let buffer = draw(12, |ctx| {
        ctx.draw_text_clipped_bg(0, 0, "Hello World", Color::WHITE, Color::BLUE, 4);
        ctx.draw_text_clipped_bg_bold(0, 1, "Hello World", Color::WHITE, Color::RED, 3);
    });
    assert_eq!(row(&buffer, 0), "Hell        ");
    assert_eq!(buffer.get(3, 0).unwrap().bg, Some(Color::BLUE));
    assert!(buffer.get(4, 0).unwrap().bg.is_none());
    assert!(!buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));

    assert_eq!(row(&buffer, 1), "Hel         ");
    assert_eq!(buffer.get(2, 1).unwrap().bg, Some(Color::RED));
    assert!(buffer.get(2, 1).unwrap().modifier.contains(Modifier::BOLD));
    assert!(buffer.get(3, 1).unwrap().bg.is_none());
}

#[test]
fn test_draw_text_clipped_fits_within_max_width() {
    let buffer = draw(8, |ctx| {
        ctx.draw_text_clipped(0, 0, "Hello", Color::WHITE, 10)
    });
    assert_eq!(row(&buffer, 0), "Hello   ");
}

// =========================================================================
// Alignment when the text fills or overflows the width
// =========================================================================

#[test]
fn test_draw_text_centered_exact_fit() {
    let buffer = draw(7, |ctx| {
        ctx.draw_text_centered(1, 0, 5, "Hello", Color::WHITE)
    });
    assert_eq!(row(&buffer, 0), " Hello ");
}

#[test]
fn test_draw_text_centered_too_long_is_left_aligned_and_clipped() {
    let buffer = draw(6, |ctx| {
        ctx.draw_text_centered(1, 0, 3, "Hello", Color::WHITE)
    });
    assert_eq!(row(&buffer, 0), " Hel  ");
}

#[test]
fn test_draw_text_centered_odd_gap_rounds_left() {
    let buffer = draw(6, |ctx| {
        ctx.draw_text_centered(0, 0, 6, "abc", Color::WHITE)
    });
    assert_eq!(row(&buffer, 0), " abc  ");
}

#[test]
fn test_draw_text_right_exact_fit() {
    let buffer = draw(7, |ctx| ctx.draw_text_right(1, 0, 5, "Hello", Color::WHITE));
    assert_eq!(row(&buffer, 0), " Hello ");
}

#[test]
fn test_draw_text_right_too_long_is_left_aligned_and_clipped() {
    let buffer = draw(6, |ctx| ctx.draw_text_right(1, 0, 3, "Hello", Color::WHITE));
    assert_eq!(row(&buffer, 0), " Hel  ");
}
