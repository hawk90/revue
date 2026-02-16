//! Canvas widget integration tests
//!
//! Tests that use only public APIs without accessing private fields.

#![allow(unused_imports)]

use revue::layout::Rect;
use revue::prelude::RenderContext;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::canvas::*;
use revue::widget::traits::View;

// Standard canvas tests

#[test]
fn test_canvas_new() {
    let c = Canvas::new(|_ctx| {});
    let _ = c;
}

#[test]
fn test_draw_context_dimensions() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(5, 5, 30, 10);
    let ctx = DrawContext::new(&mut buffer, area);

    assert_eq!(ctx.width(), 30);
    assert_eq!(ctx.height(), 10);
}

#[test]
fn test_draw_context_set() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);

    ctx.set(5, 5, 'X');
}

#[test]
fn test_draw_context_hline() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);

    ctx.hline(2, 5, 10, '-', Some(Color::WHITE));
}

#[test]
fn test_draw_context_vline() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);

    ctx.vline(5, 2, 6, '|', Some(Color::WHITE));
}

#[test]
fn test_draw_context_rect() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);

    ctx.rect(2, 2, 10, 5, Some(Color::CYAN));
}

#[test]
fn test_draw_context_fill_rect() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);

    ctx.fill_rect(
        Rect::new(3, 3, 5, 3),
        '#',
        Some(Color::RED),
        Some(Color::BLACK),
    );
}

#[test]
fn test_draw_context_bar() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = DrawContext::new(&mut buffer, area);

    ctx.bar(5, 2, 15, Color::GREEN, None);
}

#[test]
fn test_draw_context_text() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = DrawContext::new(&mut buffer, area);

    ctx.text(5, 2, "Hello World", Some(Color::WHITE));
}

#[test]
fn test_draw_context_line() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = DrawContext::new(&mut buffer, area);

    ctx.line(0, 0, 19, 9, '*', Some(Color::YELLOW));
}

#[test]
fn test_canvas_render() {
    let c = canvas(|ctx| {
        ctx.bar(0, 0, 10, Color::BLUE, None);
        ctx.text(0, 1, "Test", Some(Color::WHITE));
    });

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut render_ctx = RenderContext::new(&mut buffer, area);

    c.render(&mut render_ctx);
}

#[test]
fn test_canvas_helper() {
    let c = canvas(|ctx| {
        ctx.point(5, 5, Color::RED);
    });
    let _ = c;
}

#[test]
fn test_partial_bar() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = DrawContext::new(&mut buffer, area);

    ctx.partial_bar(0, 0, 5.5, Color::GREEN);
}

// Braille tests using public APIs only

#[test]
fn test_braille_grid_new() {
    let grid = BrailleGrid::new(40, 20);
    assert_eq!(grid.width(), 80); // 40 * 2
    assert_eq!(grid.height(), 80); // 20 * 4
}

#[test]
fn test_braille_line() {
    let mut grid = BrailleGrid::new(20, 10);
    grid.draw(&braille::Line::new(0.0, 0.0, 39.0, 39.0, Color::CYAN));
    // Line should be drawn
}

#[test]
fn test_braille_circle() {
    let mut grid = BrailleGrid::new(20, 10);
    grid.draw(&braille::Circle::new(20.0, 20.0, 10.0, Color::YELLOW));
    // Circle should be drawn
}

#[test]
fn test_braille_filled_circle() {
    let mut grid = BrailleGrid::new(20, 10);
    grid.draw(&braille::FilledCircle::new(20.0, 20.0, 10.0, Color::GREEN));
    // Filled circle should be drawn
}

#[test]
fn test_braille_rectangle() {
    let mut grid = BrailleGrid::new(20, 10);
    grid.draw(&braille::Rectangle::new(5.0, 5.0, 20.0, 15.0, Color::RED));
    // Rectangle should be drawn
}

#[test]
fn test_braille_filled_rectangle() {
    let mut grid = BrailleGrid::new(20, 10);
    grid.draw(&braille::FilledRectangle::new(
        5.0,
        5.0,
        20.0,
        15.0,
        Color::BLUE,
    ));
    // Filled rectangle should be drawn
}

#[test]
fn test_braille_points() {
    let mut grid = BrailleGrid::new(40, 20);
    let coords: Vec<(f64, f64)> = (0..80)
        .map(|x| {
            let y = (x as f64 * 0.1).sin() * 30.0 + 40.0;
            (x as f64, y)
        })
        .collect();
    grid.draw(&braille::Points::new(coords, Color::MAGENTA));
    // Points should be drawn
}

#[test]
fn test_braille_canvas_widget() {
    let bc = braille_canvas(|ctx| {
        ctx.line(0.0, 0.0, 20.0, 40.0, Color::WHITE);
        ctx.circle(30.0, 30.0, 10.0, Color::CYAN);
    });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);

    bc.render(&mut render_ctx);
}

#[test]
fn test_braille_context_methods() {
    let bc = braille_canvas(|ctx| {
        ctx.line(0.0, 0.0, 10.0, 10.0, Color::WHITE);
        ctx.circle(20.0, 20.0, 5.0, Color::RED);
        ctx.filled_circle(30.0, 20.0, 5.0, Color::GREEN);
        ctx.rect(40.0, 10.0, 10.0, 10.0, Color::BLUE);
        ctx.filled_rect(55.0, 10.0, 10.0, 10.0, Color::YELLOW);
        ctx.set(0, 0, Color::MAGENTA);
    });

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, area);

    bc.render(&mut render_ctx);
}