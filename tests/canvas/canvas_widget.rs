//! Canvas Widget tests

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{
    braille_canvas, canvas, Arc, BrailleGrid, Circle, ClipRegion, FilledCircle, FilledPolygon,
    FilledRectangle, Layer, Line, Points, Polygon, Rectangle, Shape, Transform,
};

#[test]
fn test_canvas_creation() {
    let c = canvas(|_ctx| {});
    let _ = c;
}

#[test]
fn test_canvas_draw_basic() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.set(5, 5, 'X');
        ctx.text(0, 0, "Hello", Some(Color::WHITE));
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_canvas_draw_shapes() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.hline(0, 0, 10, '-', Some(Color::WHITE));
        ctx.vline(0, 0, 5, '|', Some(Color::WHITE));
        ctx.rect(2, 2, 8, 4, Some(Color::CYAN));
        ctx.bar(0, 5, 10, Color::GREEN, None);
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_canvas_partial_bar() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.partial_bar(0, 0, 5.5, Color::BLUE);
    });

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_canvas_line_drawing() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.line(0, 0, 15, 8, '*', Some(Color::YELLOW));
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_canvas_fill_rect() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.fill_rect(
            Rect::new(2, 2, 5, 3),
            '#',
            Some(Color::RED),
            Some(Color::BLACK),
        );
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_canvas_point() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.point(5, 5, Color::MAGENTA);
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_canvas_clear() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.set(5, 5, 'X');
        ctx.clear();
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}
