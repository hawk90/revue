//! Transform tests

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
fn test_transform_identity() {
    let t = Transform::identity();
    let (x, y) = t.apply(5.0, 10.0);
    assert!((x - 5.0).abs() < 0.001);
    assert!((y - 10.0).abs() < 0.001);
}

#[test]
fn test_transform_translate() {
    let t = Transform::translate(10.0, 20.0);
    let (x, y) = t.apply(5.0, 5.0);
    assert!((x - 15.0).abs() < 0.001);
    assert!((y - 25.0).abs() < 0.001);
}

#[test]
fn test_transform_scale() {
    let t = Transform::scale(2.0, 3.0);
    let (x, y) = t.apply(5.0, 10.0);
    assert!((x - 10.0).abs() < 0.001);
    assert!((y - 30.0).abs() < 0.001);
}

#[test]
fn test_transform_scale_uniform() {
    let t = Transform::scale_uniform(2.0);
    let (x, y) = t.apply(5.0, 10.0);
    assert!((x - 10.0).abs() < 0.001);
    assert!((y - 20.0).abs() < 0.001);
}

#[test]
fn test_transform_rotate() {
    let t = Transform::rotate(std::f64::consts::FRAC_PI_2);
    let (x, y) = t.apply(1.0, 0.0);
    assert!(x.abs() < 0.001);
    assert!((y - 1.0).abs() < 0.001);
}

#[test]
fn test_transform_rotate_degrees() {
    let t = Transform::rotate_degrees(90.0);
    let (x, y) = t.apply(1.0, 0.0);
    assert!(x.abs() < 0.001);
    assert!((y - 1.0).abs() < 0.001);
}

#[test]
fn test_transform_chain() {
    let t = Transform::translate(10.0, 0.0).then(&Transform::scale(2.0, 2.0));
    let (x, y) = t.apply(5.0, 5.0);
    // First scale: (10, 10), then translate: (20, 10)
    assert!((x - 20.0).abs() < 0.001);
    assert!((y - 10.0).abs() < 0.001);
}

#[test]
fn test_transform_with_translate() {
    let t = Transform::identity().with_translate(10.0, 20.0);
    let (x, y) = t.apply(0.0, 0.0);
    assert!((x - 10.0).abs() < 0.001);
    assert!((y - 20.0).abs() < 0.001);
}

#[test]
fn test_transform_with_scale() {
    let t = Transform::identity().with_scale(2.0, 3.0);
    let (x, y) = t.apply(5.0, 5.0);
    assert!((x - 10.0).abs() < 0.001);
    assert!((y - 15.0).abs() < 0.001);
}

#[test]
fn test_transform_with_rotate() {
    let t = Transform::identity().with_rotate(std::f64::consts::PI);
    let (x, y) = t.apply(1.0, 0.0);
    assert!((x + 1.0).abs() < 0.001);
    assert!(y.abs() < 0.001);
}

#[test]
fn test_transform_default() {
    let t = Transform::default();
    let (x, y) = t.apply(5.0, 10.0);
    assert!((x - 5.0).abs() < 0.001);
    assert!((y - 10.0).abs() < 0.001);
}
