//! Skeleton widget tests extracted from src/widget/display/skeleton.rs

use revue::prelude::*;

// Note: Most tests for Skeleton access private fields and methods
// (shape, is_animate, skeleton_char, width, height, color, lines),
// so only a subset of tests that use public APIs are extracted here.

#[test]
fn test_skeleton_new() {
    let s = Skeleton::new();
    // Can't verify private fields, just verify it compiles
}

#[test]
fn test_skeleton_shapes() {
    let s1 = skeleton().circle();
    let s2 = skeleton().paragraph();
    // Can't verify shapes as they're private, just verify they compile
}

#[test]
fn test_skeleton_dimensions() {
    let s = skeleton().width(10).height(3);
    // Can't verify width/height as they're private, just verify they compile
}

#[test]
fn test_skeleton_helper() {
    let s = skeleton();
    let s2 = skeleton().circle();
    let s3 = skeleton().paragraph();
    // Just verify constructors compile
}

#[test]
fn test_skeleton_render() {
    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().width(5).height(2).no_animate();
    s.render(&mut ctx);
    // Just verify it renders without panic
}
