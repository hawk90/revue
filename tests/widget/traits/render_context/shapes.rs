//! Tests for RenderContext shape drawing methods
//!
//! Extracted from src/widget/traits/render_context/shapes.rs

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::render_context::RenderContext;
use revue::widget::View;

// Test widget to create a render context
#[allow(dead_code)]
struct TestWidget;
impl View for TestWidget {
    fn render(&self, _ctx: &mut RenderContext) {}
}

#[test]
fn test_draw_hline() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not panic
    ctx.draw_hline(10, 5, 20, '-', Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_vline() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_vline(10, 5, 10, '|', Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_box_rounded() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_box_rounded(5, 5, 20, 10, Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_box_rounded_too_small() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Width or height < 2 should return early
    ctx.draw_box_rounded(5, 5, 1, 10, Color::rgb(255, 255, 255));
    ctx.draw_box_rounded(5, 5, 10, 1, Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_box_no_top() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_box_no_top(5, 5, 20, 10, Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_header_line() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let parts = &[("Title", Color::rgb(255, 0, 0))];
    ctx.draw_header_line(5, 5, 30, parts, Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_header_line_multiple_parts() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let parts = &[
        ("A", Color::rgb(255, 0, 0)),
        ("B", Color::rgb(0, 255, 0)),
        ("C", Color::rgb(0, 0, 255)),
    ];
    ctx.draw_header_line(5, 5, 40, parts, Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_header_line_too_small() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let parts = &[("Title", Color::rgb(255, 0, 0))];
    // Width < 4 should return early
    ctx.draw_header_line(5, 5, 3, parts, Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_box_single() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_box_single(5, 5, 20, 10, Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_box_double() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_box_double(5, 5, 20, 10, Color::rgb(255, 255, 255));
}

#[test]
fn test_fill() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.fill(10, 10, 5, 3, '*', Color::rgb(255, 255, 255));
}

#[test]
fn test_fill_bg() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.fill_bg(10, 10, 5, 3, Color::rgb(100, 100, 100));
}

#[test]
fn test_clear() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.clear(10, 10, 5, 3);
}

#[test]
fn test_draw_box_titled() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_box_titled(5, 5, 30, 10, "Title", Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_box_titled_empty_title() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_box_titled(5, 5, 30, 10, "", Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_box_titled_unicode() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_box_titled(5, 5, 30, 10, "标题", Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_box_titled_single() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_box_titled_single(5, 5, 30, 10, "Title", Color::rgb(255, 255, 255));
}

#[test]
fn test_draw_box_titled_double() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_box_titled_double(5, 5, 30, 10, "Title", Color::rgb(255, 255, 255));
}

#[test]
fn test_fill_zero_area() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Zero width/height should not panic
    ctx.fill(10, 10, 0, 0, '*', Color::rgb(255, 255, 255));
    ctx.fill_bg(10, 10, 0, 0, Color::rgb(100, 100, 100));
    ctx.clear(10, 10, 0, 0);
}

#[test]
fn test_draw_lines_zero_length() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_hline(10, 5, 0, '-', Color::rgb(255, 255, 255));
    ctx.draw_vline(10, 5, 0, '|', Color::rgb(255, 255, 255));
}
