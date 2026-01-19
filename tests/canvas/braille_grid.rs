//! BrailleGrid tests

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
fn test_braille_grid_creation() {
    let grid = BrailleGrid::new(40, 20);
    assert_eq!(grid.width(), 80); // 40 * 2
    assert_eq!(grid.height(), 80); // 20 * 4
}

#[test]
fn test_braille_grid_set() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(5, 5, Color::RED);
    // Dot should be set
}

#[test]
fn test_braille_grid_set_bounds() {
    let mut grid = BrailleGrid::new(10, 10);
    // Should not crash when setting out of bounds
    grid.set(1000, 1000, Color::RED);
}

#[test]
fn test_braille_grid_clear() {
    let mut grid = BrailleGrid::new(10, 10);
    grid.set(5, 5, Color::RED);
    grid.clear();
    // Grid should be cleared
}

#[test]
fn test_braille_grid_draw_shape() {
    let mut grid = BrailleGrid::new(20, 10);
    grid.draw(&Line::new(0.0, 0.0, 20.0, 20.0, Color::WHITE));
}

#[test]
fn test_braille_grid_render() {
    let mut grid = BrailleGrid::new(20, 10);
    grid.draw(&Circle::new(10.0, 10.0, 5.0, Color::CYAN));

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    grid.render(&mut buffer, area);
}
