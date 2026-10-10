//! Canvas widget integration tests
//!
//! Draw through the public API, then check the cells that land in the buffer.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{
    braille_canvas, canvas, BrailleGrid, Canvas, Circle, DrawContext, FilledCircle,
    FilledRectangle, Line, Points, Rectangle,
};

fn sym(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).map(|c| c.symbol).unwrap_or('\0')
}

fn fg(buffer: &Buffer, x: u16, y: u16) -> Option<Color> {
    buffer.get(x, y).and_then(|c| c.fg)
}

/// Number of raised braille dots across the whole grid.
fn dot_count(grid: &BrailleGrid) -> u32 {
    grid.cells().iter().map(|b| b.count_ones()).sum()
}

// =========================================================================
// DrawContext
// =========================================================================

#[test]
fn test_canvas_new_renders_its_closure() {
    let c = Canvas::new(|ctx| ctx.set(0, 0, 'Z'));
    let mut buffer = Buffer::new(4, 2);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 4, 2));
    c.render(&mut ctx);
    assert_eq!(sym(&buffer, 0, 0), 'Z');
}

#[test]
fn test_draw_context_dimensions() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(5, 5, 30, 10);
    let ctx = DrawContext::new(&mut buffer, area);

    assert_eq!(ctx.width(), 30);
    assert_eq!(ctx.height(), 10);
    assert_eq!(ctx.area(), area);
}

#[test]
fn test_draw_context_set_is_relative_and_clipped() {
    let mut buffer = Buffer::new(20, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(2, 3, 5, 5));
    ctx.set(1, 1, 'X');
    // Outside the context area: ignored.
    ctx.set(5, 0, 'Y');
    assert_eq!(sym(&buffer, 3, 4), 'X');
    assert_eq!(sym(&buffer, 7, 3), ' ');
}

#[test]
fn test_draw_context_hline() {
    let mut buffer = Buffer::new(20, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 20, 10));
    ctx.hline(2, 5, 10, '-', Some(Color::WHITE));

    for x in 2..12 {
        assert_eq!(sym(&buffer, x, 5), '-');
        assert_eq!(fg(&buffer, x, 5), Some(Color::WHITE));
    }
    assert_eq!(sym(&buffer, 1, 5), ' ');
    assert_eq!(sym(&buffer, 12, 5), ' ');
}

#[test]
fn test_draw_context_vline() {
    let mut buffer = Buffer::new(20, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 20, 10));
    ctx.vline(5, 2, 6, '|', Some(Color::WHITE));

    for y in 2..8 {
        assert_eq!(sym(&buffer, 5, y), '|');
    }
    assert_eq!(sym(&buffer, 5, 1), ' ');
    assert_eq!(sym(&buffer, 5, 8), ' ');
}

#[test]
fn test_draw_context_rect() {
    let mut buffer = Buffer::new(20, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 20, 10));
    ctx.rect(2, 2, 10, 5, Some(Color::CYAN));

    assert_eq!(sym(&buffer, 2, 2), '┌');
    assert_eq!(sym(&buffer, 11, 2), '┐');
    assert_eq!(sym(&buffer, 2, 6), '└');
    assert_eq!(sym(&buffer, 11, 6), '┘');
    assert_eq!(sym(&buffer, 5, 2), '─');
    assert_eq!(sym(&buffer, 5, 6), '─');
    assert_eq!(sym(&buffer, 2, 4), '│');
    assert_eq!(sym(&buffer, 11, 4), '│');
    // Interior stays empty.
    assert_eq!(sym(&buffer, 5, 4), ' ');
    assert_eq!(fg(&buffer, 2, 2), Some(Color::CYAN));
}

#[test]
fn test_draw_context_fill_rect() {
    let mut buffer = Buffer::new(20, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 20, 10));
    ctx.fill_rect(
        Rect::new(3, 3, 5, 3),
        '#',
        Some(Color::RED),
        Some(Color::BLACK),
    );

    for y in 3..6 {
        for x in 3..8 {
            assert_eq!(sym(&buffer, x, y), '#');
            let cell = buffer.get(x, y).unwrap();
            assert_eq!(cell.fg, Some(Color::RED));
            assert_eq!(cell.bg, Some(Color::BLACK));
        }
    }
    assert_eq!(sym(&buffer, 8, 3), ' ');
    assert_eq!(sym(&buffer, 3, 6), ' ');
}

#[test]
fn test_draw_context_bar() {
    let mut buffer = Buffer::new(30, 5);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 30, 5));
    ctx.bar(5, 2, 15, Color::GREEN, None);

    for x in 5..20 {
        assert_eq!(sym(&buffer, x, 2), '█');
        assert_eq!(fg(&buffer, x, 2), Some(Color::GREEN));
    }
    assert_eq!(sym(&buffer, 20, 2), ' ');
}

#[test]
fn test_draw_context_text() {
    let mut buffer = Buffer::new(30, 5);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 30, 5));
    ctx.text(5, 2, "Hello World", Some(Color::WHITE));

    let row: String = (5..16).map(|x| sym(&buffer, x, 2)).collect();
    assert_eq!(row, "Hello World");
    assert_eq!(fg(&buffer, 5, 2), Some(Color::WHITE));
}

#[test]
fn test_draw_context_line_diagonal() {
    let mut buffer = Buffer::new(20, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 20, 10));
    ctx.line(0, 0, 9, 9, '*', Some(Color::YELLOW));

    for i in 0..10 {
        assert_eq!(sym(&buffer, i, i), '*');
    }
    assert_eq!(sym(&buffer, 1, 0), ' ');
}

#[test]
fn test_draw_context_line_reaches_both_endpoints() {
    let mut buffer = Buffer::new(20, 10);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 20, 10));
    ctx.line(0, 0, 19, 9, '*', None);

    assert_eq!(sym(&buffer, 0, 0), '*');
    assert_eq!(sym(&buffer, 19, 9), '*');
    // Bresenham draws one cell per column on a shallow line.
    for x in 0..20 {
        let hits = (0..10).filter(|&y| sym(&buffer, x, y) == '*').count();
        assert_eq!(hits, 1, "column {x}");
    }
}

#[test]
fn test_canvas_render() {
    let c = canvas(|ctx| {
        ctx.bar(0, 0, 10, Color::BLUE, None);
        ctx.text(0, 1, "Test", Some(Color::WHITE));
    });

    let mut buffer = Buffer::new(20, 5);
    let mut render_ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 5));
    c.render(&mut render_ctx);

    assert_eq!(sym(&buffer, 0, 0), '█');
    assert_eq!(sym(&buffer, 9, 0), '█');
    assert_eq!(sym(&buffer, 10, 0), ' ');
    let row: String = (0..4).map(|x| sym(&buffer, x, 1)).collect();
    assert_eq!(row, "Test");
}

#[test]
fn test_canvas_helper_point() {
    let c = canvas(|ctx| ctx.point(5, 3, Color::RED));
    let mut buffer = Buffer::new(10, 5);
    let mut render_ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 10, 5));
    c.render(&mut render_ctx);

    assert_eq!(sym(&buffer, 5, 3), '●');
    assert_eq!(fg(&buffer, 5, 3), Some(Color::RED));
}

#[test]
fn test_partial_bar() {
    let mut buffer = Buffer::new(20, 5);
    let mut ctx = DrawContext::new(&mut buffer, Rect::new(0, 0, 20, 5));
    ctx.partial_bar(0, 0, 5.5, Color::GREEN);

    for x in 0..5 {
        assert_eq!(sym(&buffer, x, 0), '█');
    }
    // Half a cell is four eighths: the left-half block.
    assert_eq!(sym(&buffer, 5, 0), '▌');
    assert_eq!(sym(&buffer, 6, 0), ' ');
}

// =========================================================================
// Braille
// =========================================================================

#[test]
fn test_braille_grid_new() {
    let grid = BrailleGrid::new(40, 20);
    assert_eq!(grid.width(), 80); // 40 * 2
    assert_eq!(grid.height(), 80); // 20 * 4
    assert_eq!(dot_count(&grid), 0);
}

#[test]
fn test_braille_line() {
    let mut grid = BrailleGrid::new(20, 10);
    grid.draw(&Line::new(0.0, 0.0, 39.0, 39.0, Color::CYAN));
    // A 45-degree line through a 40x40 dot space sets one dot per step.
    assert_eq!(dot_count(&grid), 40);
    // Both ends are in the grid: top-left cell and the cell holding (39, 39).
    assert_ne!(grid.get_char(0, 0), '\u{2800}');
    assert_ne!(grid.get_char(19, 9), '\u{2800}');
}

#[test]
fn test_braille_circle_is_hollow() {
    let mut grid = BrailleGrid::new(20, 10);
    grid.draw(&Circle::new(20.0, 20.0, 10.0, Color::YELLOW));
    let outline = dot_count(&grid);
    assert!(outline > 0);
    // The centre cell holds no dots.
    assert_eq!(grid.get_char(10, 5), '\u{2800}');

    let mut filled = BrailleGrid::new(20, 10);
    filled.draw(&FilledCircle::new(20.0, 20.0, 10.0, Color::GREEN));
    assert!(dot_count(&filled) > outline);
    assert_ne!(filled.get_char(10, 5), '\u{2800}');
}

#[test]
fn test_braille_rectangle_vs_filled() {
    let mut outline = BrailleGrid::new(20, 10);
    outline.draw(&Rectangle::new(5.0, 5.0, 20.0, 15.0, Color::RED));
    let mut filled = BrailleGrid::new(20, 10);
    filled.draw(&FilledRectangle::new(5.0, 5.0, 20.0, 15.0, Color::BLUE));

    assert!(dot_count(&outline) > 0);
    assert!(dot_count(&filled) > dot_count(&outline));
    // Every outline dot is also part of the filled shape.
    for (o, f) in outline.cells().iter().zip(filled.cells()) {
        assert_eq!(o & f, *o);
    }
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
    grid.draw(&Points::new(coords, Color::MAGENTA));
    // Points are joined into a polyline, so every terminal column the series
    // crosses holds at least one dot, and there are at least as many dots as
    // points.
    assert!(dot_count(&grid) >= 80);
    for x in 0..40 {
        assert!(
            (0..20).any(|y| grid.get_char(x, y) != '\u{2800}'),
            "column {x} has no dots"
        );
    }
    assert!(grid.colors().contains(&Some(Color::MAGENTA)));
}

#[test]
fn test_braille_canvas_widget() {
    let bc = braille_canvas(|ctx| {
        ctx.line(0.0, 0.0, 20.0, 40.0, Color::WHITE);
        ctx.circle(30.0, 30.0, 10.0, Color::CYAN);
    });

    let mut buffer = Buffer::new(20, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 10));
    bc.render(&mut render_ctx);

    // The line starts at the top-left dot.
    let first = sym(&buffer, 0, 0);
    assert!(('\u{2801}'..='\u{28FF}').contains(&first), "got {first:?}");
    assert_eq!(fg(&buffer, 0, 0), Some(Color::WHITE));
    let braille_cells = (0..10)
        .flat_map(|y| (0..20).map(move |x| (x, y)))
        .filter(|&(x, y)| ('\u{2801}'..='\u{28FF}').contains(&sym(&buffer, x, y)))
        .count();
    assert!(braille_cells > 10);
}

#[test]
fn test_braille_context_methods() {
    let bc = braille_canvas(|ctx| {
        assert_eq!(ctx.width(), 80);
        assert_eq!(ctx.height(), 40);
        ctx.line(0.0, 0.0, 10.0, 10.0, Color::WHITE);
        ctx.circle(20.0, 20.0, 5.0, Color::RED);
        ctx.filled_circle(30.0, 20.0, 5.0, Color::GREEN);
        ctx.rect(40.0, 10.0, 10.0, 10.0, Color::BLUE);
        ctx.filled_rect(55.0, 10.0, 10.0, 10.0, Color::YELLOW);
        ctx.set(79, 39, Color::MAGENTA);
    });

    let mut buffer = Buffer::new(40, 10);
    let mut render_ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 10));
    bc.render(&mut render_ctx);

    // The single dot set at (79, 39) is the bottom-right dot of the last cell.
    assert_eq!(sym(&buffer, 39, 9), '\u{2880}');
    assert_eq!(fg(&buffer, 39, 9), Some(Color::MAGENTA));
    // Each shape leaves its colour somewhere.
    let colors: Vec<Option<Color>> = (0..10)
        .flat_map(|y| (0..40).map(move |x| (x, y)))
        .map(|(x, y)| fg(&buffer, x, y))
        .collect();
    for c in [
        Color::WHITE,
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
    ] {
        assert!(colors.contains(&Some(c)), "missing {c:?}");
    }
}
