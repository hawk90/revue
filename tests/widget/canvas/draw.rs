//! DrawContext tests
//!
//! The basic primitives (dimensions, set, hline, vline, rect, fill_rect, bar,
//! text, diagonal line, a half-cell partial bar, point) are covered in
//! tests/canvas/integration.rs; these cover their edge cases.

use revue::layout::Rect;
use revue::render::{Buffer, Cell, Modifier};
use revue::style::Color;
use revue::widget::DrawContext;

fn sym(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).map(|c| c.symbol).unwrap_or('\0')
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width()).map(|x| sym(buffer, x, y)).collect()
}

fn col(buffer: &Buffer, x: u16) -> String {
    (0..buffer.height()).map(|y| sym(buffer, x, y)).collect()
}

fn is_blank(buffer: &Buffer) -> bool {
    (0..buffer.height()).all(|y| row(buffer, y).trim().is_empty())
}

// =========================================================================
// set / set_styled / set_cell
// =========================================================================

#[test]
fn test_draw_context_set_out_of_bounds() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 5, 5));
    ctx.set(15, 15, 'X');
    // Inside the buffer but outside the context area: also ignored
    ctx.set(5, 0, 'Y');
    ctx.set(0, 5, 'Z');
    assert!(is_blank(&buffer));
}

#[test]
fn test_draw_context_set_at_boundary() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
    ctx.set(0, 0, 'A');
    ctx.set(9, 9, 'B');
    assert_eq!(sym(&buffer, 0, 0), 'A');
    assert_eq!(sym(&buffer, 9, 9), 'B');
}

#[test]
fn test_draw_context_set_styled() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(2, 2, 8, 8));
    ctx.set_styled(5, 5, 'X', Some(Color::RED), Some(Color::BLUE));
    let cell = buffer.get(7, 7).unwrap();
    assert_eq!(cell.symbol, 'X');
    assert_eq!(cell.fg, Some(Color::RED));
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_draw_context_set_styled_no_color() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
    ctx.set_styled(5, 5, 'X', None, None);
    let cell = buffer.get(5, 5).unwrap();
    assert_eq!(cell.symbol, 'X');
    assert_eq!(cell.fg, None);
    assert_eq!(cell.bg, None);
}

#[test]
fn test_draw_context_set_cell() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(1, 1, 9, 9));
    let mut cell = Cell::new('X');
    cell.modifier = Modifier::UNDERLINE;
    ctx.set_cell(5, 5, cell);
    ctx.set_cell(9, 0, Cell::new('Y')); // outside the area
    let drawn = buffer.get(6, 6).unwrap();
    assert_eq!(drawn.symbol, 'X');
    assert!(drawn.modifier.contains(Modifier::UNDERLINE));
    assert_eq!(row(&buffer, 1).trim(), "");
}

// =========================================================================
// hline / vline
// =========================================================================

#[test]
fn test_draw_context_hline_zero_length() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
    ctx.hline(5, 5, 0, '─', Some(Color::WHITE));
    assert!(is_blank(&buffer));
}

#[test]
fn test_draw_context_hline_truncated() {
    let mut buffer = Buffer::new(12, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
    ctx.hline(5, 5, 20, '─', Some(Color::WHITE));
    // Clipped at the context width, not the buffer width
    assert_eq!(row(&buffer, 5), "     ─────  ");
}

#[test]
fn test_draw_context_vline_zero_length() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
    ctx.vline(5, 5, 0, '│', Some(Color::WHITE));
    assert!(is_blank(&buffer));
}

#[test]
fn test_draw_context_vline_truncated() {
    let mut buffer = Buffer::new(10, 12);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
    ctx.vline(5, 5, 20, '│', Some(Color::WHITE));
    assert_eq!(col(&buffer, 5), "     │││││  ");
}

// =========================================================================
// rect / fill_rect
// =========================================================================

#[test]
fn test_draw_context_rect_zero_size() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
    ctx.rect(5, 5, 0, 0, Some(Color::WHITE));
    ctx.rect(5, 5, 3, 0, Some(Color::WHITE));
    ctx.rect(5, 5, 0, 3, Some(Color::WHITE));
    assert!(is_blank(&buffer));
}

#[test]
fn test_draw_context_rect_minimal() {
    let mut buffer = Buffer::new(4, 3);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 4, 3));
    ctx.rect(0, 0, 2, 2, None);
    assert_eq!(row(&buffer, 0), "┌┐  ");
    assert_eq!(row(&buffer, 1), "└┘  ");
}

#[test]
fn test_draw_context_rect_clipped() {
    // A rect hanging off the right/bottom edge keeps its visible part
    let mut buffer = Buffer::new(6, 4);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 6, 4));
    ctx.rect(3, 1, 10, 10, None);
    assert_eq!(row(&buffer, 0), "      ");
    assert_eq!(row(&buffer, 1), "   ┌──");
    assert_eq!(row(&buffer, 2), "   │  ");
    assert_eq!(row(&buffer, 3), "   │  ");
}

#[test]
fn test_draw_context_fill_rect_zero_size() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
    ctx.fill_rect(
        Rect::new(5, 5, 0, 0),
        'X',
        Some(Color::WHITE),
        Some(Color::BLACK),
    );
    assert!(is_blank(&buffer));
}

#[test]
fn test_draw_context_fill_rect_clipped() {
    let mut buffer = Buffer::new(6, 3);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 4, 2));
    ctx.fill_rect(Rect::new(2, 1, 10, 10), '#', None, None);
    assert_eq!(row(&buffer, 0), "      ");
    assert_eq!(row(&buffer, 1), "  ##  ");
    assert_eq!(row(&buffer, 2), "      ");
}

// =========================================================================
// bar / partial_bar
// =========================================================================

#[test]
fn test_draw_context_bar_zero_width() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
    ctx.bar(5, 5, 0, Color::WHITE, None);
    assert!(is_blank(&buffer));
}

#[test]
fn test_draw_context_bar_background() {
    let mut buffer = Buffer::new(10, 1);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 1));
    ctx.bar(2, 0, 3, Color::WHITE, Some(Color::BLACK));
    assert_eq!(row(&buffer, 0), "  ███     ");
    assert_eq!(buffer.get(2, 0).unwrap().bg, Some(Color::BLACK));
}

#[test]
fn test_draw_context_partial_bar_full() {
    let mut buffer = Buffer::new(10, 1);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 1));
    ctx.partial_bar(2, 0, 5.0, Color::WHITE);
    // Whole numbers draw no partial cell
    assert_eq!(row(&buffer, 0), "  █████   ");
}

#[test]
fn test_draw_context_partial_bar_eighths() {
    let cases = [
        (0.125, '▏'),
        (0.25, '▎'),
        (0.375, '▍'),
        (0.5, '▌'),
        (0.625, '▋'),
        (0.75, '▊'),
        (0.875, '▉'),
    ];
    for (fraction, expected) in cases {
        let mut buffer = Buffer::new(5, 1);
        let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 5, 1));
        ctx.partial_bar(0, 0, 2.0 + fraction, Color::WHITE);
        assert_eq!(row(&buffer, 0), format!("██{expected}  "), "{fraction}");
    }
}

#[test]
fn test_draw_context_partial_bar_rounds_up_to_full_block() {
    // 2.97 is closer to three cells than to two and seven eighths
    let mut buffer = Buffer::new(5, 1);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 5, 1));
    ctx.partial_bar(0, 0, 2.97, Color::WHITE);
    assert_eq!(row(&buffer, 0), "███  ");
}

#[test]
fn test_draw_context_partial_bar_zero() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
    ctx.partial_bar(5, 5, 0.0, Color::WHITE);
    assert!(is_blank(&buffer));
}

// =========================================================================
// text / text_bold
// =========================================================================

#[test]
fn test_draw_context_text_empty() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
    ctx.text(5, 5, "", Some(Color::WHITE));
    assert!(is_blank(&buffer));
}

#[test]
fn test_draw_context_text_truncated() {
    let mut buffer = Buffer::new(12, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
    ctx.text(5, 5, "very long text", Some(Color::WHITE));
    assert_eq!(row(&buffer, 5), "     very   ");
}

#[test]
fn test_draw_context_text_bold() {
    let mut buffer = Buffer::new(10, 3);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(1, 1, 8, 2));
    ctx.text_bold(1, 0, "bold", Some(Color::WHITE));
    assert_eq!(row(&buffer, 1), "  bold    ");
    let cell = buffer.get(2, 1).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD));
    assert_eq!(cell.fg, Some(Color::WHITE));
}

#[test]
fn test_draw_context_text_bold_truncated() {
    let mut buffer = Buffer::new(10, 1);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 6, 1));
    ctx.text_bold(3, 0, "bold", None);
    assert_eq!(row(&buffer, 0), "   bol    ");
}

// =========================================================================
// clear
// =========================================================================

#[test]
fn test_draw_context_clear() {
    let mut buffer = Buffer::new(10, 10);
    buffer.set(0, 0, Cell::new('#'));
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(2, 2, 5, 5));
    ctx.set(1, 1, 'X');
    ctx.set_styled(2, 2, 'Y', Some(Color::RED), None);
    ctx.clear();
    assert_eq!(sym(&buffer, 3, 3), ' ');
    assert_eq!(buffer.get(4, 4).unwrap().fg, None);
    // Only the context area is cleared
    assert_eq!(sym(&buffer, 0, 0), '#');
}

// =========================================================================
// line
// =========================================================================

#[test]
fn test_draw_context_line_horizontal() {
    let mut buffer = Buffer::new(20, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 20, 10));
    ctx.line(2, 5, 15, 5, '─', Some(Color::WHITE));
    assert_eq!(row(&buffer, 5), format!("  {}    ", "─".repeat(14)));
}

#[test]
fn test_draw_context_line_vertical() {
    let mut buffer = Buffer::new(10, 20);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 20));
    ctx.line(5, 2, 5, 15, '│', Some(Color::WHITE));
    assert_eq!(col(&buffer, 5), format!("  {}    ", "│".repeat(14)));
}

#[test]
fn test_draw_context_line_reversed() {
    // Drawing from the far end gives the same cells
    let mut forward = Buffer::new(10, 10);
    DrawContext::new(&mut forward, Rect::new(0, 0, 10, 10)).line(1, 1, 8, 4, '*', None);
    let mut backward = Buffer::new(10, 10);
    DrawContext::new(&mut backward, Rect::new(0, 0, 10, 10)).line(8, 4, 1, 1, '*', None);
    for y in 0..10 {
        assert_eq!(row(&forward, y), row(&backward, y), "row {y}");
    }
}

#[test]
fn test_draw_context_line_all_quadrants() {
    let mut buffer = Buffer::new(20, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 20, 10));
    ctx.line(0, 0, 9, 9, 'a', None); // down-right
    ctx.line(19, 0, 10, 9, 'b', None); // down-left
    ctx.line(0, 9, 9, 0, 'c', None); // up-right (overwrites a at its crossing)
    for (x, y, ch) in [
        (0, 0, 'a'),
        (19, 0, 'b'),
        (10, 9, 'b'),
        (0, 9, 'c'),
        (9, 0, 'c'),
    ] {
        assert_eq!(sym(&buffer, x, y), ch, "({x}, {y})");
    }
}

#[test]
fn test_draw_context_line_same_point() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
    ctx.line(5, 5, 5, 5, 'X', Some(Color::WHITE));
    assert_eq!(sym(&buffer, 5, 5), 'X');
    let marked = (0..10)
        .flat_map(|y| (0..10).map(move |x| (x, y)))
        .filter(|&(x, y)| sym(&buffer, x, y) == 'X')
        .count();
    assert_eq!(marked, 1);
}

#[test]
fn test_draw_context_line_no_color() {
    let mut buffer = Buffer::new(20, 20);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 20, 20));
    ctx.line(2, 2, 15, 15, '*', None);
    let cell = buffer.get(8, 8).unwrap();
    assert_eq!(cell.symbol, '*');
    assert_eq!(cell.fg, None);
}

#[test]
fn test_draw_context_line_clipped() {
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 5, 5));
    ctx.line(0, 0, 9, 9, '*', None);
    assert_eq!(sym(&buffer, 4, 4), '*');
    assert_eq!(sym(&buffer, 5, 5), ' ');
}
