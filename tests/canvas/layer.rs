//! Layer tests

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
fn test_layer_creation() {
    let layer = Layer::new(40, 20);
    assert_eq!(layer.width(), 80); // 40 * 2
    assert_eq!(layer.height(), 80); // 20 * 4
    assert!(layer.is_visible());
    assert!((layer.opacity() - 1.0).abs() < 0.001);
}

#[test]
fn test_layer_visibility() {
    let mut layer = Layer::new(40, 20);
    assert!(layer.is_visible());

    layer.set_visible(false);
    assert!(!layer.is_visible());

    layer.set_visible(true);
    assert!(layer.is_visible());
}

#[test]
fn test_layer_opacity() {
    let mut layer = Layer::new(40, 20);
    assert!((layer.opacity() - 1.0).abs() < 0.001);

    layer.set_opacity(0.5);
    assert!((layer.opacity() - 0.5).abs() < 0.001);

    layer.set_opacity(0.0);
    assert!((layer.opacity() - 0.0).abs() < 0.001);

    // Test clamping
    layer.set_opacity(2.0);
    assert!((layer.opacity() - 1.0).abs() < 0.001);

    layer.set_opacity(-1.0);
    assert!((layer.opacity() - 0.0).abs() < 0.001);
}

#[test]
fn test_layer_draw_shape() {
    let mut layer = Layer::new(40, 20);
    layer.draw(&Circle::new(20.0, 20.0, 10.0, Color::RED));
    // Shape should be drawn on the layer
}

#[test]
fn test_layer_clear() {
    let mut layer = Layer::new(40, 20);
    layer.draw(&Circle::new(20.0, 20.0, 10.0, Color::RED));
    layer.clear();
    // Layer should be cleared
}

#[test]
fn test_layer_set_dot() {
    let mut layer = Layer::new(40, 20);
    layer.set(10, 10, Color::BLUE);
    // Dot should be set
}

#[test]
fn test_layer_composite() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut layer = Layer::new(40, 20);

    layer.draw(&Circle::new(20.0, 20.0, 10.0, Color::RED));
    grid.composite_layer(&layer);
    // Layer should be composited onto grid
}

#[test]
fn test_layer_composite_invisible() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut layer = Layer::new(40, 20);

    layer.draw(&Circle::new(20.0, 20.0, 10.0, Color::RED));
    layer.set_visible(false);

    // Pre-draw something on the grid
    grid.draw(&Line::new(0.0, 0.0, 10.0, 10.0, Color::WHITE));

    grid.composite_layer(&layer);
    // Invisible layer should not affect grid
}

#[test]
fn test_layer_composite_zero_opacity() {
    let mut grid = BrailleGrid::new(40, 20);
    let mut layer = Layer::new(40, 20);

    layer.draw(&Circle::new(20.0, 20.0, 10.0, Color::RED));
    layer.set_opacity(0.0);

    grid.composite_layer(&layer);
    // Zero opacity layer should not affect grid
}
