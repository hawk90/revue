//! Width/height constraint tests

use revue::layout::{max_width, min_width, MediaQuery, Rect};

// Convenience wrapper function for responsive tests
fn layout_with_width(width: u16) -> revue::layout::ResponsiveLayout {
    revue::layout::ResponsiveLayout::new(width, 24)
}

#[test]
fn test_min_width_function() {
    let mq = min_width(80);
    assert!(matches!(mq, MediaQuery::MinWidth(80)));
}

#[test]
fn test_max_width_function() {
    let mq = max_width(80);
    assert!(matches!(mq, MediaQuery::MaxWidth(80)));
}

#[test]
fn test_rect_new() {
    let rect = Rect::new(10, 20, 100, 50);
    assert_eq!(rect.x, 10);
    assert_eq!(rect.y, 20);
    assert_eq!(rect.width, 100);
    assert_eq!(rect.height, 50);
}

#[test]
fn test_rect_default() {
    let rect = Rect::default();
    assert_eq!(rect.x, 0);
    assert_eq!(rect.y, 0);
    assert_eq!(rect.width, 0);
    assert_eq!(rect.height, 0);
}

#[test]
fn test_rect_right() {
    let rect = Rect::new(10, 20, 100, 50);
    assert_eq!(rect.right(), 110);
}

#[test]
fn test_rect_right_saturating() {
    let rect = Rect::new(u16::MAX - 10, 0, 20, 10);
    assert_eq!(rect.right(), u16::MAX);
}

#[test]
fn test_rect_bottom() {
    let rect = Rect::new(10, 20, 100, 50);
    assert_eq!(rect.bottom(), 70);
}

#[test]
fn test_rect_bottom_saturating() {
    let rect = Rect::new(0, u16::MAX - 10, 10, 20);
    assert_eq!(rect.bottom(), u16::MAX);
}

#[test]
fn test_rect_intersects_true() {
    let rect1 = Rect::new(0, 0, 50, 50);
    let rect2 = Rect::new(25, 25, 50, 50);
    assert!(rect1.intersects(&rect2));
    assert!(rect2.intersects(&rect1));
}

#[test]
fn test_rect_intersects_false() {
    let rect1 = Rect::new(0, 0, 20, 20);
    let rect2 = Rect::new(30, 30, 20, 20);
    assert!(!rect1.intersects(&rect2));
}

#[test]
fn test_rect_intersects_adjacent() {
    let rect1 = Rect::new(0, 0, 20, 20);
    let rect2 = Rect::new(20, 0, 20, 20);
    // Adjacent rectangles do NOT intersect
    assert!(!rect1.intersects(&rect2));
}

#[test]
fn test_rect_intersection_some() {
    let rect1 = Rect::new(0, 0, 50, 50);
    let rect2 = Rect::new(25, 25, 50, 50);
    let result = rect1.intersection(&rect2);

    assert!(result.is_some());
    let intersection = result.unwrap();
    assert_eq!(intersection.x, 25);
    assert_eq!(intersection.y, 25);
    assert_eq!(intersection.width, 25);
    assert_eq!(intersection.height, 25);
}

#[test]
fn test_rect_intersection_none() {
    let rect1 = Rect::new(0, 0, 20, 20);
    let rect2 = Rect::new(30, 30, 20, 20);
    let result = rect1.intersection(&rect2);
    assert!(result.is_none());
}

#[test]
fn test_rect_union() {
    let rect1 = Rect::new(0, 0, 20, 20);
    let rect2 = Rect::new(10, 10, 20, 20);
    let result = rect1.union(&rect2);

    assert_eq!(result.x, 0);
    assert_eq!(result.y, 0);
    assert_eq!(result.width, 30);
    assert_eq!(result.height, 30);
}

#[test]
fn test_rect_union_non_overlapping() {
    let rect1 = Rect::new(0, 0, 10, 10);
    let rect2 = Rect::new(20, 20, 10, 10);
    let result = rect1.union(&rect2);

    assert_eq!(result.x, 0);
    assert_eq!(result.y, 0);
    assert_eq!(result.width, 30);
    assert_eq!(result.height, 30);
}

#[test]
fn test_rect_contains_point_inside() {
    let rect = Rect::new(10, 10, 50, 50);
    assert!(rect.contains(15, 15));
    assert!(rect.contains(10, 10));
    assert!(rect.contains(59, 59));
}

#[test]
fn test_rect_contains_point_outside() {
    let rect = Rect::new(10, 10, 50, 50);
    assert!(!rect.contains(9, 15));
    assert!(!rect.contains(15, 9));
    assert!(!rect.contains(60, 15));
    assert!(!rect.contains(15, 60));
}

#[test]
fn test_rect_contains_point_on_edge() {
    let rect = Rect::new(10, 10, 50, 50);
    // Points on right/bottom edges are NOT contained
    assert!(!rect.contains(60, 15));
    assert!(!rect.contains(15, 60));
}
