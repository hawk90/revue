//! Shape tests

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
fn test_line_creation() {
    let line = Line::new(0.0, 0.0, 10.0, 10.0, Color::RED);
    assert_eq!(line.x0, 0.0);
    assert_eq!(line.y0, 0.0);
    assert_eq!(line.x1, 10.0);
    assert_eq!(line.y1, 10.0);
}

#[test]
fn test_line_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let line = Line::new(0.0, 0.0, 10.0, 10.0, Color::WHITE);
    line.draw(&mut grid);
    // Line should be drawn (visual verification would require more setup)
}

#[test]
fn test_circle_creation() {
    let circle = Circle::new(20.0, 20.0, 10.0, Color::BLUE);
    assert_eq!(circle.x, 20.0);
    assert_eq!(circle.y, 20.0);
    assert_eq!(circle.radius, 10.0);
}

#[test]
fn test_circle_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let circle = Circle::new(20.0, 20.0, 10.0, Color::CYAN);
    circle.draw(&mut grid);
}

#[test]
fn test_filled_circle_creation() {
    let circle = FilledCircle::new(15.0, 15.0, 8.0, Color::GREEN);
    assert_eq!(circle.x, 15.0);
    assert_eq!(circle.y, 15.0);
    assert_eq!(circle.radius, 8.0);
}

#[test]
fn test_filled_circle_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let circle = FilledCircle::new(20.0, 20.0, 5.0, Color::YELLOW);
    circle.draw(&mut grid);
}

#[test]
fn test_rectangle_creation() {
    let rect = Rectangle::new(5.0, 5.0, 20.0, 10.0, Color::MAGENTA);
    assert_eq!(rect.x, 5.0);
    assert_eq!(rect.y, 5.0);
    assert_eq!(rect.width, 20.0);
    assert_eq!(rect.height, 10.0);
}

#[test]
fn test_rectangle_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let rect = Rectangle::new(5.0, 5.0, 20.0, 15.0, Color::WHITE);
    rect.draw(&mut grid);
}

#[test]
fn test_filled_rectangle_creation() {
    let rect = FilledRectangle::new(0.0, 0.0, 10.0, 5.0, Color::RED);
    assert_eq!(rect.x, 0.0);
    assert_eq!(rect.y, 0.0);
    assert_eq!(rect.width, 10.0);
    assert_eq!(rect.height, 5.0);
}

#[test]
fn test_filled_rectangle_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let rect = FilledRectangle::new(5.0, 5.0, 10.0, 8.0, Color::BLUE);
    rect.draw(&mut grid);
}

#[test]
fn test_points_creation() {
    let coords = vec![(0.0, 0.0), (5.0, 5.0), (10.0, 0.0)];
    let points = Points::new(coords.clone(), Color::CYAN);
    assert_eq!(points.coords, coords);
}

#[test]
fn test_points_from_slices() {
    let xs = [0.0, 5.0, 10.0, 15.0];
    let ys = [0.0, 5.0, 0.0, 5.0];
    let points = Points::from_slices(&xs, &ys, Color::WHITE);
    assert_eq!(points.coords.len(), 4);
}

#[test]
fn test_points_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let points = Points::new(vec![(0.0, 0.0), (20.0, 40.0), (40.0, 0.0)], Color::MAGENTA);
    points.draw(&mut grid);
}
