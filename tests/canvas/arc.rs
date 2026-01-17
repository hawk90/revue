//! Arc tests

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
fn test_arc_creation() {
    let arc = Arc::new(20.0, 20.0, 10.0, 0.0, std::f64::consts::PI, Color::RED);
    assert_eq!(arc.x, 20.0);
    assert_eq!(arc.y, 20.0);
    assert_eq!(arc.radius, 10.0);
    assert_eq!(arc.start_angle, 0.0);
    assert_eq!(arc.end_angle, std::f64::consts::PI);
}

#[test]
fn test_arc_from_degrees() {
    let arc = Arc::from_degrees(20.0, 20.0, 10.0, 0.0, 180.0, Color::BLUE);
    assert_eq!(arc.x, 20.0);
    assert_eq!(arc.y, 20.0);
    assert!((arc.end_angle - std::f64::consts::PI).abs() < 0.001);
}

#[test]
fn test_arc_draw() {
    let mut grid = BrailleGrid::new(20, 10);
    let arc = Arc::new(
        20.0,
        20.0,
        10.0,
        0.0,
        std::f64::consts::FRAC_PI_2,
        Color::CYAN,
    );
    arc.draw(&mut grid);
}

#[test]
fn test_arc_full_circle() {
    let mut grid = BrailleGrid::new(20, 10);
    let arc = Arc::new(20.0, 20.0, 10.0, 0.0, std::f64::consts::TAU, Color::GREEN);
    arc.draw(&mut grid);
}

#[test]
fn test_arc_reverse_direction() {
    let mut grid = BrailleGrid::new(20, 10);
    // End angle less than start angle should still work
    let arc = Arc::new(20.0, 20.0, 10.0, std::f64::consts::PI, 0.0, Color::YELLOW);
    arc.draw(&mut grid);
}
