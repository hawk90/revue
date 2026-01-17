//! BrailleCanvas Widget tests

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
fn test_braille_canvas_creation() {
    let bc = braille_canvas(|_ctx| {});
    let _ = bc;
}

#[test]
fn test_braille_canvas_draw_basic() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.set(10, 10, Color::WHITE);
        ctx.line(0.0, 0.0, 20.0, 40.0, Color::CYAN);
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_shapes() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.circle(20.0, 20.0, 10.0, Color::RED);
        ctx.filled_circle(40.0, 20.0, 8.0, Color::GREEN);
        ctx.rect(5.0, 5.0, 15.0, 10.0, Color::BLUE);
        ctx.filled_rect(50.0, 5.0, 15.0, 10.0, Color::YELLOW);
    });

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_points() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.points(
            vec![(0.0, 0.0), (10.0, 20.0), (20.0, 0.0), (30.0, 20.0)],
            Color::MAGENTA,
        );
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_arc() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.arc(20.0, 20.0, 10.0, 0.0, std::f64::consts::PI, Color::CYAN);
        ctx.arc_degrees(40.0, 20.0, 10.0, 0.0, 270.0, Color::YELLOW);
    });

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_polygon() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.polygon(vec![(10.0, 10.0), (30.0, 10.0), (20.0, 30.0)], Color::RED);
        ctx.regular_polygon(50.0, 20.0, 10.0, 6, Color::BLUE);
    });

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_filled_polygon() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.filled_polygon(vec![(10.0, 10.0), (30.0, 10.0), (20.0, 30.0)], Color::GREEN);
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_dimensions() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        let w = ctx.width();
        let h = ctx.height();
        // Draw border using dimensions
        ctx.rect(0.0, 0.0, w as f64 - 1.0, h as f64 - 1.0, Color::WHITE);
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_canvas_clear() {
    use revue::widget::View;

    let bc = braille_canvas(|ctx| {
        ctx.circle(20.0, 20.0, 10.0, Color::RED);
        ctx.clear();
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut render_ctx);
}
