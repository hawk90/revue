//! Polygon tests

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
fn test_polygon_creation() {
    let vertices = vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)];
    let polygon = Polygon::new(vertices.clone(), Color::RED);
    assert_eq!(polygon.vertices, vertices);
}

#[test]
fn test_polygon_regular() {
    let hex = Polygon::regular(20.0, 20.0, 10.0, 6, Color::BLUE);
    assert_eq!(hex.vertices.len(), 6);
}

#[test]
fn test_polygon_regular_triangle() {
    let triangle = Polygon::regular(20.0, 20.0, 10.0, 3, Color::GREEN);
    assert_eq!(triangle.vertices.len(), 3);
}

#[test]
fn test_polygon_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let polygon = Polygon::new(
        vec![(10.0, 10.0), (30.0, 10.0), (30.0, 30.0), (10.0, 30.0)],
        Color::CYAN,
    );
    polygon.draw(&mut grid);
}

#[test]
fn test_polygon_draw_empty() {
    let mut grid = BrailleGrid::new(20, 10);
    let polygon = Polygon::new(vec![], Color::RED);
    polygon.draw(&mut grid);
    // Should not crash with empty vertices
}

#[test]
fn test_polygon_draw_single_point() {
    let mut grid = BrailleGrid::new(20, 10);
    let polygon = Polygon::new(vec![(10.0, 10.0)], Color::RED);
    polygon.draw(&mut grid);
    // Should not crash with single point
}

#[test]
fn test_filled_polygon_creation() {
    let vertices = vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)];
    let polygon = FilledPolygon::new(vertices.clone(), Color::YELLOW);
    assert_eq!(polygon.vertices, vertices);
}

#[test]
fn test_filled_polygon_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let polygon = FilledPolygon::new(
        vec![(10.0, 10.0), (30.0, 10.0), (20.0, 30.0)],
        Color::MAGENTA,
    );
    polygon.draw(&mut grid);
}
