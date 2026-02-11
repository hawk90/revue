//! Tests for RenderContext text drawing methods
//!
//! Extracted from src/widget/traits/render_context/text.rs

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::render_context::RenderContext;

// =========================================================================
// draw_char tests
// =========================================================================

#[test]
fn test_draw_char() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_char(0, 0, 'A', Color::WHITE);
}

#[test]
fn test_draw_char_offset() {
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_char(10, 10, 'B', Color::CYAN);
}

#[test]
fn test_draw_char_wide() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_char(0, 0, '你', Color::WHITE);
}

// =========================================================================
// draw_char_bg tests
// =========================================================================

#[test]
fn test_draw_char_bg() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_char_bg(0, 0, 'A', Color::WHITE, Color::BLACK);
}

#[test]
fn test_draw_char_bg_offset() {
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_char_bg(10, 10, 'B', Color::CYAN, Color::BLUE);
}

// =========================================================================
// draw_char_bold tests
// =========================================================================

#[test]
fn test_draw_char_bold() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_char_bold(0, 0, 'A', Color::WHITE);
}

#[test]
fn test_draw_char_bold_offset() {
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_char_bold(10, 10, 'B', Color::CYAN);
}

// =========================================================================
// draw_text tests
// =========================================================================

#[test]
fn test_draw_text_empty() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text(0, 0, "", Color::WHITE);
}

#[test]
fn test_draw_text_single() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text(0, 0, "A", Color::WHITE);
}

#[test]
fn test_draw_text_multiple() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text(0, 0, "Hello", Color::WHITE);
}

#[test]
fn test_draw_text_wide() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text(0, 0, "你好", Color::WHITE);
}

#[test]
fn test_draw_text_mixed() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text(0, 0, "Hello你好", Color::WHITE);
}

#[test]
fn test_draw_text_offset() {
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text(10, 10, "Test", Color::CYAN);
}

// =========================================================================
// draw_text_bg tests
// =========================================================================

#[test]
fn test_draw_text_bg() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_bg(0, 0, "Hello", Color::WHITE, Color::BLACK);
}

#[test]
fn test_draw_text_bg_empty() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_bg(0, 0, "", Color::WHITE, Color::BLACK);
}

// =========================================================================
// draw_text_bold tests
// =========================================================================

#[test]
fn test_draw_text_bold() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_bold(0, 0, "Hello", Color::WHITE);
}

#[test]
fn test_draw_text_bold_empty() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_bold(0, 0, "", Color::WHITE);
}

// =========================================================================
// draw_text_bg_bold tests
// =========================================================================

#[test]
fn test_draw_text_bg_bold() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_bg_bold(0, 0, "Hello", Color::WHITE, Color::BLACK);
}

#[test]
fn test_draw_text_bg_bold_empty() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_bg_bold(0, 0, "", Color::WHITE, Color::BLACK);
}

// =========================================================================
// draw_text_clipped tests
// =========================================================================

#[test]
fn test_draw_text_clipped_empty() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_clipped(0, 0, "", Color::WHITE, 10);
}

#[test]
fn test_draw_text_clipped_fit() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_clipped(0, 0, "Hello", Color::WHITE, 10);
}

#[test]
fn test_draw_text_clipped_truncate() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_clipped(0, 0, "Hello World", Color::WHITE, 5);
}

#[test]
fn test_draw_text_clipped_wide() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_clipped(0, 0, "你好世界", Color::WHITE, 5);
}

// =========================================================================
// draw_text_clipped_bold tests
// =========================================================================

#[test]
fn test_draw_text_clipped_bold() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_clipped_bold(0, 0, "Hello", Color::WHITE, 10);
}

#[test]
fn test_draw_text_clipped_bold_truncate() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_clipped_bold(0, 0, "Hello World", Color::WHITE, 5);
}

// =========================================================================
// draw_text_dim tests
// =========================================================================

#[test]
fn test_draw_text_dim() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_dim(0, 0, "Hello", Color::WHITE);
}

#[test]
fn test_draw_text_dim_empty() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_dim(0, 0, "", Color::WHITE);
}

// =========================================================================
// draw_text_italic tests
// =========================================================================

#[test]
fn test_draw_text_italic() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_italic(0, 0, "Hello", Color::WHITE);
}

#[test]
fn test_draw_text_italic_empty() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_italic(0, 0, "", Color::WHITE);
}

// =========================================================================
// draw_text_underline tests
// =========================================================================

#[test]
fn test_draw_text_underline() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_underline(0, 0, "Hello", Color::WHITE);
}

#[test]
fn test_draw_text_underline_empty() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_underline(0, 0, "", Color::WHITE);
}

// =========================================================================
// draw_text_centered tests
// =========================================================================

#[test]
fn test_draw_text_centered_exact_fit() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_centered(0, 0, 5, "Hello", Color::WHITE);
}

#[test]
fn test_draw_text_centered_short() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_centered(0, 0, 10, "Hi", Color::WHITE);
}

#[test]
fn test_draw_text_centered_long() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_centered(0, 0, 3, "Hello", Color::WHITE);
}

#[test]
fn test_draw_text_centered_empty() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_centered(0, 0, 10, "", Color::WHITE);
}

// =========================================================================
// draw_text_right tests
// =========================================================================

#[test]
fn test_draw_text_right_exact_fit() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_right(0, 0, 5, "Hello", Color::WHITE);
}

#[test]
fn test_draw_text_right_short() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_right(0, 0, 10, "Hi", Color::WHITE);
}

#[test]
fn test_draw_text_right_long() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_right(0, 0, 3, "Hello", Color::WHITE);
}

#[test]
fn test_draw_text_right_empty() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_text_right(0, 0, 10, "", Color::WHITE);
}
