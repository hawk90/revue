//! Skeleton widget tests extracted from src/widget/display/skeleton.rs

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::display::skeleton::{
    skeleton, skeleton_avatar, skeleton_paragraph, skeleton_text, Skeleton, SkeletonShape,
};
use revue::widget::traits::{RenderContext, View};

// =========================================================================
// Basic tests
// =========================================================================

#[test]
fn test_skeleton_new() {
    let s = Skeleton::new();
    assert_eq!(s.shape(), SkeletonShape::Rectangle);
    assert!(s.is_animate());
}

#[test]
fn test_skeleton_shapes() {
    let s = skeleton().circle();
    assert_eq!(s.shape(), SkeletonShape::Circle);

    let s = skeleton().paragraph();
    assert_eq!(s.shape(), SkeletonShape::Paragraph);
}

#[test]
fn test_skeleton_dimensions() {
    let s = skeleton().width(10).height(3);
    assert_eq!(s.width(), 10);
    assert_eq!(s.height(), 3);
}

#[test]
fn test_skeleton_animation() {
    let s = skeleton().frame(0);
    assert_eq!(s.skeleton_char(), '░');

    let s = skeleton().frame(1);
    assert_eq!(s.skeleton_char(), '▒');

    let s = skeleton().no_animate();
    assert!(!s.is_animate());
}

#[test]
fn test_skeleton_render_rectangle() {
    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().width(5).height(2).no_animate();
    s.render(&mut ctx);

    // Should fill the area with skeleton chars
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('░'));
}

#[test]
fn test_skeleton_render_paragraph() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton_paragraph().lines(3);
    s.render(&mut ctx);

    // Should have 3 lines
    assert!(buffer.get(0, 0).map(|c| c.symbol).is_some());
    assert!(buffer.get(0, 1).map(|c| c.symbol).is_some());
    assert!(buffer.get(0, 2).map(|c| c.symbol).is_some());
}

#[test]
fn test_helper_functions() {
    let s = skeleton();
    assert_eq!(s.shape(), SkeletonShape::Rectangle);

    let s = skeleton_text();
    assert_eq!(s.height(), 1);

    let s = skeleton_avatar();
    assert_eq!(s.shape(), SkeletonShape::Circle);

    let s = skeleton_paragraph();
    assert_eq!(s.shape(), SkeletonShape::Paragraph);
}

// =========================================================================
// skeleton_char edge cases
// =========================================================================

#[test]
fn test_skeleton_char_frame_0() {
    let s = Skeleton::new().frame(0);
    // Default animate is true
    assert_eq!(s.skeleton_char(), '░');
}

#[test]
fn test_skeleton_char_frame_1() {
    let s = Skeleton::new().frame(1);
    assert_eq!(s.skeleton_char(), '▒');
}

#[test]
fn test_skeleton_char_frame_2() {
    let s = Skeleton::new().frame(2);
    assert_eq!(s.skeleton_char(), '▓');
}

#[test]
fn test_skeleton_char_frame_3() {
    let s = Skeleton::new().frame(3);
    // Frame 3 (3 % 4 = 3) maps to '▒' (the default case)
    assert_eq!(s.skeleton_char(), '▒');
}

#[test]
fn test_skeleton_char_frame_4() {
    let s = Skeleton::new().frame(4);
    // Frame 4 (4 % 4 = 0) maps to '░'
    assert_eq!(s.skeleton_char(), '░');
}

#[test]
fn test_skeleton_char_frame_5() {
    let s = Skeleton::new().frame(5);
    // Frame 5 (5 % 4 = 1) maps to '▒'
    assert_eq!(s.skeleton_char(), '▒');
}

#[test]
fn test_skeleton_char_no_animate() {
    let s = Skeleton::new().no_animate();
    assert_eq!(s.skeleton_char(), '░'); // default wave_char
}

#[test]
fn test_skeleton_char_custom_wave_char() {
    let mut s = Skeleton::new().no_animate();
    s.set_wave_char('█');
    assert_eq!(s.skeleton_char(), '█');
}

#[test]
fn test_skeleton_color() {
    let s = Skeleton::new().color(Color::CYAN);
    assert_eq!(s.color(), Color::CYAN);
}

#[test]
fn test_skeleton_lines() {
    let s = Skeleton::new().lines(5);
    assert_eq!(s.lines(), 5);
}

#[test]
fn test_skeleton_rectangle_shorthand() {
    let s = Skeleton::new().circle().rectangle();
    assert_eq!(s.shape(), SkeletonShape::Rectangle);
}

#[test]
fn test_skeleton_default() {
    let s = Skeleton::default();
    assert_eq!(s.shape(), SkeletonShape::Rectangle);
    assert_eq!(s.width(), 0);
    assert_eq!(s.height(), 1);
    assert!(s.is_animate());
}

#[test]
fn test_skeleton_shape_default() {
    let shape = SkeletonShape::default();
    assert_eq!(shape, SkeletonShape::Rectangle);
}

// =========================================================================
// render circle edge cases
// =========================================================================

#[test]
fn test_skeleton_render_circle_size_1() {
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Skeleton::new().circle().height(1).no_animate();
    s.render(&mut ctx);

    // Size 1 circle uses filled dot
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('●'));
}

#[test]
fn test_skeleton_render_circle_size_2() {
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Skeleton::new().circle().height(2).no_animate();
    s.render(&mut ctx);

    // Size 2 uses 4 corner characters
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('╭'));
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('╮'));
    assert_eq!(buffer.get(0, 1).map(|c| c.symbol), Some('╰'));
    assert_eq!(buffer.get(1, 1).map(|c| c.symbol), Some('╯'));
}

#[test]
fn test_skeleton_render_circle_size_3() {
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Skeleton::new().circle().height(3).no_animate();
    s.render(&mut ctx);

    // Size 3+ uses box drawing with filled center
    // Corners
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('╭'));
    assert_eq!(buffer.get(2, 0).map(|c| c.symbol), Some('╮'));
    assert_eq!(buffer.get(0, 2).map(|c| c.symbol), Some('╰'));
    assert_eq!(buffer.get(2, 2).map(|c| c.symbol), Some('╯'));

    // Top/bottom edges
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('─'));
    assert_eq!(buffer.get(1, 2).map(|c| c.symbol), Some('─'));

    // Sides
    assert_eq!(buffer.get(0, 1).map(|c| c.symbol), Some('│'));
    assert_eq!(buffer.get(2, 1).map(|c| c.symbol), Some('│'));

    // Center should be skeleton char
    assert_eq!(buffer.get(1, 1).map(|c| c.symbol), Some('░'));
}

#[test]
fn test_skeleton_render_with_animation() {
    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Skeleton::new().width(5).height(1).frame(1);
    s.render(&mut ctx);

    // Should use frame 1 char (▒)
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('▒'));
}

#[test]
fn test_skeleton_render_width_0_fills_area() {
    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Skeleton::new().width(0).height(2).no_animate();
    s.render(&mut ctx);

    // Width 0 means fill available area
    assert_eq!(buffer.get(9, 0).map(|c| c.symbol), Some('░'));
}

#[test]
fn test_skeleton_render_clamps_to_area_size() {
    let mut buffer = Buffer::new(5, 2);
    let area = Rect::new(0, 0, 5, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Request larger than available
    let s = Skeleton::new().width(10).height(10).no_animate();
    s.render(&mut ctx);

    // Should clamp to area size
    assert!(buffer.get(4, 0).map(|c| c.symbol).is_some());
    assert!(buffer.get(4, 1).map(|c| c.symbol).is_some());
}

#[test]
fn test_skeleton_render_rectangle_full_area() {
    let mut buffer = Buffer::new(5, 3);
    let area = Rect::new(0, 0, 5, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Skeleton::new().rectangle().height(3).no_animate();
    s.render(&mut ctx);

    // Check corners are filled (width 0 means fill to area width)
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('░'));
    assert_eq!(buffer.get(4, 0).map(|c| c.symbol), Some('░'));
    assert_eq!(buffer.get(0, 2).map(|c| c.symbol), Some('░'));
    assert_eq!(buffer.get(4, 2).map(|c| c.symbol), Some('░'));
}

#[test]
fn test_skeleton_render_paragraph_line_widths() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Skeleton::new().paragraph().width(15).lines(3).no_animate();
    s.render(&mut ctx);

    // Line 0: full width (15) - positions 0-14 should be filled
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('░'));
    assert_eq!(buffer.get(14, 0).map(|c| c.symbol), Some('░'));

    // Line 1: width - 4 (11) - alternate lines
    assert_eq!(buffer.get(0, 1).map(|c| c.symbol), Some('░'));
    assert_eq!(buffer.get(10, 1).map(|c| c.symbol), Some('░'));

    // Line 2: 2/3 of width (10) - last line shorter
    assert_eq!(buffer.get(0, 2).map(|c| c.symbol), Some('░'));
    assert_eq!(buffer.get(9, 2).map(|c| c.symbol), Some('░'));
}

#[test]
fn test_skeleton_render_paragraph_clamps_to_area_height() {
    let mut buffer = Buffer::new(20, 2);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Request more lines than available
    let s = skeleton_paragraph().lines(5);
    s.render(&mut ctx);

    // Should only render 2 lines (area height)
    assert!(buffer.get(0, 0).map(|c| c.symbol).is_some());
    assert!(buffer.get(0, 1).map(|c| c.symbol).is_some());
}
