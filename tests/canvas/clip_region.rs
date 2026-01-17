//! ClipRegion tests

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
fn test_clip_region_creation() {
    let clip = ClipRegion::new(10.0, 20.0, 30.0, 40.0);
    assert_eq!(clip.x_min, 10.0);
    assert_eq!(clip.y_min, 20.0);
    assert_eq!(clip.x_max, 40.0);
    assert_eq!(clip.y_max, 60.0);
}

#[test]
fn test_clip_region_from_bounds() {
    let clip = ClipRegion::from_bounds(0.0, 0.0, 100.0, 100.0);
    assert_eq!(clip.x_min, 0.0);
    assert_eq!(clip.y_min, 0.0);
    assert_eq!(clip.x_max, 100.0);
    assert_eq!(clip.y_max, 100.0);
}

#[test]
fn test_clip_region_contains() {
    let clip = ClipRegion::new(10.0, 10.0, 20.0, 20.0);

    assert!(clip.contains(15.0, 15.0)); // Inside
    assert!(clip.contains(10.0, 10.0)); // On min edge
    assert!(clip.contains(30.0, 30.0)); // On max edge
    assert!(!clip.contains(5.0, 15.0)); // Left of region
    assert!(!clip.contains(35.0, 15.0)); // Right of region
    assert!(!clip.contains(15.0, 5.0)); // Above region
    assert!(!clip.contains(15.0, 35.0)); // Below region
}

#[test]
fn test_clip_region_intersect() {
    let clip1 = ClipRegion::new(0.0, 0.0, 20.0, 20.0);
    let clip2 = ClipRegion::new(10.0, 10.0, 20.0, 20.0);

    let intersection = clip1.intersect(&clip2).unwrap();
    assert_eq!(intersection.x_min, 10.0);
    assert_eq!(intersection.y_min, 10.0);
    assert_eq!(intersection.x_max, 20.0);
    assert_eq!(intersection.y_max, 20.0);
}

#[test]
fn test_clip_region_no_intersect() {
    let clip1 = ClipRegion::new(0.0, 0.0, 10.0, 10.0);
    let clip2 = ClipRegion::new(20.0, 20.0, 10.0, 10.0);

    assert!(clip1.intersect(&clip2).is_none());
}
