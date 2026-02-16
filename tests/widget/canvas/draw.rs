//! DrawContext tests

use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::style::Color;
use revue::widget::canvas::DrawContext;

// =========================================================================
// DrawContext::new tests
// =========================================================================

#[test]
fn test_draw_context_new() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let ctx = DrawContext::new(&mut buffer, area);
    assert_eq!(ctx.width(), 10);
    assert_eq!(ctx.height(), 10);
}

// =========================================================================
// width and height tests
// =========================================================================

#[test]
fn test_draw_context_width() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let ctx = DrawContext::new(&mut buffer, area);
    assert_eq!(ctx.width(), 20);
}

#[test]
fn test_draw_context_height() {
    let mut buffer = Buffer::new(10, 20);
    let area = Rect::new(0, 0, 10, 20);
    let ctx = DrawContext::new(&mut buffer, area);
    assert_eq!(ctx.height(), 20);
}

#[test]
fn test_draw_context_area() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(5, 5, 10, 10);
    let ctx = DrawContext::new(&mut buffer, area);
    assert_eq!(ctx.area(), area);
}

// =========================================================================
// set tests
// =========================================================================

#[test]
fn test_draw_context_set_char() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.set(5, 5, 'X');
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_set_out_of_bounds() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.set(15, 15, 'X');
    // Should not panic, just ignore
}

#[test]
fn test_draw_context_set_at_boundary() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.set(0, 0, 'A');
    ctx.set(9, 9, 'B');
    // Just verify it doesn't panic
}

// =========================================================================
// set_styled tests
// =========================================================================

#[test]
fn test_draw_context_set_styled() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.set_styled(5, 5, 'X', Some(Color::RED), Some(Color::BLUE));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_set_styled_no_color() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.set_styled(5, 5, 'X', None, None);
    // Just verify it doesn't panic
}

// =========================================================================
// set_cell tests
// =========================================================================

#[test]
fn test_draw_context_set_cell() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    let cell = Cell::new('X');
    ctx.set_cell(5, 5, cell);
    // Just verify it doesn't panic
}

// =========================================================================
// hline tests
// =========================================================================

#[test]
fn test_draw_context_hline() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.hline(2, 5, 10, '─', Some(Color::WHITE));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_hline_zero_length() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.hline(5, 5, 0, '─', Some(Color::WHITE));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_hline_truncated() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.hline(5, 5, 20, '─', Some(Color::WHITE));
    // Should be truncated to fit
}

// =========================================================================
// vline tests
// =========================================================================

#[test]
fn test_draw_context_vline() {
    let mut buffer = Buffer::new(10, 20);
    let area = Rect::new(0, 0, 10, 20);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.vline(5, 2, 10, '│', Some(Color::WHITE));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_vline_zero_length() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.vline(5, 5, 0, '│', Some(Color::WHITE));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_vline_truncated() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.vline(5, 5, 20, '│', Some(Color::WHITE));
    // Should be truncated to fit
}

// =========================================================================
// rect tests
// =========================================================================

#[test]
fn test_draw_context_rect() {
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.rect(2, 2, 10, 8, Some(Color::WHITE));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_rect_zero_size() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.rect(5, 5, 0, 0, Some(Color::WHITE));
    // Just verify it doesn't panic (should return early)
}

// =========================================================================
// fill_rect tests
// =========================================================================

#[test]
fn test_draw_context_fill_rect() {
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = DrawContext::new(&mut buffer, area);
    let rect = Rect::new(2, 2, 10, 8);
    ctx.fill_rect(rect, 'X', Some(Color::WHITE), Some(Color::BLACK));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_fill_rect_zero_size() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    let rect = Rect::new(5, 5, 0, 0);
    ctx.fill_rect(rect, 'X', Some(Color::WHITE), Some(Color::BLACK));
    // Just verify it doesn't panic
}

// =========================================================================
// bar tests
// =========================================================================

#[test]
fn test_draw_context_bar() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.bar(2, 5, 10, Color::WHITE, Some(Color::BLACK));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_bar_zero_width() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.bar(5, 5, 0, Color::WHITE, None);
    // Just verify it doesn't panic
}

// =========================================================================
// partial_bar tests
// =========================================================================

#[test]
fn test_draw_context_partial_bar_full() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.partial_bar(2, 5, 5.0, Color::WHITE);
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_partial_bar_fractional() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.partial_bar(2, 5, 3.5, Color::WHITE);
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_partial_bar_zero() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.partial_bar(5, 5, 0.0, Color::WHITE);
    // Just verify it doesn't panic
}

// =========================================================================
// text tests
// =========================================================================

#[test]
fn test_draw_context_text() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.text(2, 5, "hello", Some(Color::WHITE));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_text_empty() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.text(5, 5, "", Some(Color::WHITE));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_text_truncated() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.text(5, 5, "very long text", Some(Color::WHITE));
    // Should be truncated to fit
}

// =========================================================================
// text_bold tests
// =========================================================================

#[test]
fn test_draw_context_text_bold() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.text_bold(2, 5, "bold", Some(Color::WHITE));
    // Just verify it doesn't panic
}

// =========================================================================
// clear tests
// =========================================================================

#[test]
fn test_draw_context_clear() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.set(5, 5, 'X');
    ctx.clear();
    // Just verify it doesn't panic
}

// =========================================================================
// point tests
// =========================================================================

#[test]
fn test_draw_context_point() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.point(5, 5, Color::WHITE);
    // Just verify it doesn't panic
}

// =========================================================================
// line tests
// =========================================================================

#[test]
fn test_draw_context_line_horizontal() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.line(2, 5, 15, 5, '─', Some(Color::WHITE));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_line_vertical() {
    let mut buffer = Buffer::new(10, 20);
    let area = Rect::new(0, 0, 10, 20);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.line(5, 2, 5, 15, '│', Some(Color::WHITE));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_line_diagonal() {
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.line(2, 2, 15, 15, '*', Some(Color::WHITE));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_line_same_point() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.line(5, 5, 5, 5, 'X', Some(Color::WHITE));
    // Just verify it doesn't panic
}

#[test]
fn test_draw_context_line_no_color() {
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = DrawContext::new(&mut buffer, area);
    ctx.line(2, 2, 15, 15, '*', None);
    // Just verify it doesn't panic
}
