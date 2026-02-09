//! Rendering tests for input widgets

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::input;

// =========================================================================
// Basic rendering tests
// =========================================================================

#[test]
fn test_input_render() {
    let i = input();
    let mut b = Buffer::new(30, 5);
    let a = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_with_text() {
    let i = input().value("Hello");
    let mut b = Buffer::new(30, 5);
    let a = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_with_placeholder() {
    let i = input().placeholder("Enter text...");
    let mut b = Buffer::new(30, 5);
    let a = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_focused() {
    let i = input().value("test").focused(true);
    let mut b = Buffer::new(30, 5);
    let a = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_unfocused() {
    let i = input().value("test").focused(false);
    let mut b = Buffer::new(30, 5);
    let a = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_with_colors() {
    let i = input()
        .value("colored")
        .fg(Color::RED)
        .bg(Color::BLUE);
    let mut b = Buffer::new(30, 5);
    let a = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_with_selection() {
    let mut i = input().value("hello world");
    i.selection_anchor = Some(0);
    i.cursor = 5;
    let mut b = Buffer::new(30, 5);
    let a = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_with_unicode() {
    let i = input().value("안녕🎉世界");
    let mut b = Buffer::new(30, 5);
    let a = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_small_area() {
    let i = input().value("test");
    let mut b = Buffer::new(5, 1);
    let a = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_zero_width_area() {
    let i = input().value("test");
    let mut b = Buffer::new(0, 1);
    let a = Rect::new(0, 0, 0, 1);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_zero_height_area() {
    let i = input().value("test");
    let mut b = Buffer::new(10, 0);
    let a = Rect::new(0, 0, 10, 0);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_long_text_truncates() {
    let i = input().value("This is a very long text that should be truncated");
    let mut b = Buffer::new(10, 1);
    let a = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_with_custom_cursor_style() {
    let i = input()
        .value("test")
        .cursor_style(Color::YELLOW, Color::BLACK);
    let mut b = Buffer::new(30, 5);
    let a = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_with_custom_selection_bg() {
    let i = input()
        .value("test")
        .selection_bg(Color::GREEN);
    let mut b = Buffer::new(30, 5);
    let a = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}

#[test]
fn test_input_render_offset_position() {
    let i = input().value("test");
    let mut b = Buffer::new(50, 10);
    let a = Rect::new(10, 5, 30, 1);
    let mut ctx = RenderContext::new(&mut b, a);
    i.render(&mut ctx);
}
