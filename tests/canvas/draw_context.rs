//! DrawContext tests

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
fn test_draw_context_dimensions() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        // Just verify we can call these methods without panicking
        let w = ctx.width();
        let h = ctx.height();
        assert!(w > 0);
        assert!(h > 0);
    });

    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_draw_context_area() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        let area = ctx.area();
        assert!(area.width > 0);
        assert!(area.height > 0);
    });

    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(5, 5, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_draw_context_styled() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.set_styled(5, 5, 'X', Some(Color::RED), Some(Color::BLUE));
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}

#[test]
fn test_draw_context_text_bold() {
    use revue::widget::View;

    let c = canvas(|ctx| {
        ctx.text_bold(0, 0, "Bold Text", Some(Color::WHITE));
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut render_ctx);
}
