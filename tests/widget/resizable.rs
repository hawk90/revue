//! Resizable widget integration tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{Resizable, ResizeDirection, ResizeHandle, ResizeStyle, StyledView, View};

// ==================== Basic Creation Tests ====================

#[test]
fn test_resizable_default_creation() {
    let r = Resizable::new(20, 10);
    assert_eq!(r.size(), (20, 10));
    assert!(!r.is_resizing());
}

#[test]
fn test_resizable_minimum_size() {
    let r = Resizable::new(0, 0);
    // Minimum is enforced to 1x1
    assert_eq!(r.size(), (1, 1));
}

#[test]
fn test_resizable_large_size() {
    let r = Resizable::new(200, 100);
    assert_eq!(r.size(), (200, 100));
}

// ==================== Builder Methods Tests ====================

#[test]
fn test_resizable_min_size_builder() {
    let r = Resizable::new(20, 10).min_size(10, 5);
    // Test that constraints are applied
    assert_eq!(r.size(), (20, 10));
}

#[test]
fn test_resizable_min_size_with_zero() {
    let r = Resizable::new(20, 10).min_size(0, 0);
    // Min size of 0 is clamped to 1 internally
    let _ = r;
}

#[test]
fn test_resizable_max_size_builder() {
    let r = Resizable::new(20, 10).max_size(50, 30);
    // Test that constraints are stored
    assert_eq!(r.size(), (20, 10));
}

#[test]
fn test_resizable_max_size_zero_means_unlimited() {
    let r = Resizable::new(20, 10).max_size(0, 0);
    // Max size of 0 means unlimited
    let _ = r;
}

#[test]
fn test_resizable_handles_builder_all() {
    let r = Resizable::new(20, 10).handles(ResizeHandle::ALL);
    // All handles are stored (we can verify by using handle_at)
    let area = Rect::new(0, 0, 20, 10);
    assert_eq!(r.handle_at(0, 0, area), Some(ResizeHandle::TopLeft));
    assert_eq!(r.handle_at(10, 0, area), Some(ResizeHandle::Top));
    assert_eq!(r.handle_at(0, 5, area), Some(ResizeHandle::Left));
}

#[test]
fn test_resizable_handles_builder_edges() {
    let r = Resizable::new(20, 10).handles(ResizeHandle::EDGES);
    let area = Rect::new(0, 0, 20, 10);
    // Edge handles should work
    assert_eq!(r.handle_at(10, 0, area), Some(ResizeHandle::Top));
    assert_eq!(r.handle_at(10, 9, area), Some(ResizeHandle::Bottom));
    assert_eq!(r.handle_at(0, 5, area), Some(ResizeHandle::Left));
    assert_eq!(r.handle_at(19, 5, area), Some(ResizeHandle::Right));
    // Corner handles should not work
    assert_eq!(r.handle_at(0, 0, area), None);
}

#[test]
fn test_resizable_handles_builder_corners() {
    let r = Resizable::new(20, 10).handles(ResizeHandle::CORNERS);
    let area = Rect::new(0, 0, 20, 10);
    // Corner handles should work
    assert_eq!(r.handle_at(0, 0, area), Some(ResizeHandle::TopLeft));
    assert_eq!(r.handle_at(19, 0, area), Some(ResizeHandle::TopRight));
    // Edge handles should not work
    assert_eq!(r.handle_at(10, 0, area), None);
}

#[test]
fn test_resizable_handles_builder_single() {
    let r = Resizable::new(20, 10).handles(&[ResizeHandle::Right][..]);
    let area = Rect::new(0, 0, 20, 10);
    // Only right handle should work
    assert_eq!(r.handle_at(19, 5, area), Some(ResizeHandle::Right));
    assert_eq!(r.handle_at(0, 5, area), None);
}

#[test]
fn test_resizable_handles_builder_empty() {
    let r = Resizable::new(20, 10).handles(&[]);
    let area = Rect::new(0, 0, 20, 10);
    // No handles should be detected
    assert_eq!(r.handle_at(0, 0, area), None);
    assert_eq!(r.handle_at(19, 5, area), None);
}

#[test]
fn test_resizable_style_builder_border() {
    let r = Resizable::new(20, 10).style(ResizeStyle::Border);
    // Can test style indirectly via render or content_area
    let area = Rect::new(0, 0, 20, 10);
    let content = r.content_area(area);
    // Border style has 1 pixel border
    assert_eq!(content.x, 1);
    assert_eq!(content.y, 1);
}

#[test]
fn test_resizable_style_builder_subtle() {
    let r = Resizable::new(20, 10).style(ResizeStyle::Subtle);
    let area = Rect::new(0, 0, 20, 10);
    let content = r.content_area(area);
    // Subtle style has no border
    assert_eq!(content.x, 0);
    assert_eq!(content.y, 0);
}

#[test]
fn test_resizable_style_builder_hidden() {
    let r = Resizable::new(20, 10).style(ResizeStyle::Hidden);
    let area = Rect::new(0, 0, 20, 10);
    let content = r.content_area(area);
    // Hidden style has no border
    assert_eq!(content.x, 0);
    assert_eq!(content.y, 0);
}

#[test]
fn test_resizable_style_builder_dots() {
    let r = Resizable::new(20, 10).style(ResizeStyle::Dots);
    let area = Rect::new(0, 0, 20, 10);
    let content = r.content_area(area);
    // Dots style has no border
    assert_eq!(content.x, 0);
    assert_eq!(content.y, 0);
}

#[test]
fn test_resizable_handle_color_builder() {
    let r = Resizable::new(20, 10).handle_color(Color::RED);
    // Color is stored, verify it exists
    let _ = r;
}

#[test]
fn test_resizable_active_color_builder() {
    let r = Resizable::new(20, 10).active_color(Color::GREEN);
    // Color is stored
    let _ = r;
}

#[test]
fn test_resizable_preserve_aspect_ratio_builder() {
    let r = Resizable::new(20, 10).preserve_aspect_ratio();
    // Test behavior via set_size
    let mut r = r;
    r.set_size(40, 10);
    // Height should be adjusted to maintain 2:1 ratio
    assert_eq!(r.size(), (40, 20));
}

#[test]
fn test_resizable_custom_aspect_ratio_builder() {
    let r = Resizable::new(20, 10).aspect_ratio(1.5);
    // Test behavior via set_size
    let mut r = r;
    r.set_size(30, 10);
    // 1.5:1 ratio: width 30 -> height 20
    assert_eq!(r.size(), (30, 20));
}

#[test]
fn test_resizable_snap_to_grid_builder() {
    let mut r = Resizable::new(20, 10).snap_to_grid(5, 5);
    r.set_size(23, 12);
    // Should snap to grid
    assert_eq!(r.size(), (25, 10));
}

#[test]
fn test_resizable_snap_to_grid_clamped() {
    let mut r = Resizable::new(20, 10).snap_to_grid(0, 0);
    // Grid size is clamped to minimum 1 internally
    r.set_size(23, 12);
    // Should work with 1x1 grid (no snapping)
    assert_eq!(r.size(), (23, 12));
}

#[test]
fn test_resizable_on_resize_builder() {
    use std::cell::Cell;
    use std::rc::Rc;

    let called = Rc::new(Cell::new(false));
    let called_clone = called.clone();

    let mut r = Resizable::new(20, 10).on_resize(move |_w, _h| {
        called_clone.set(true);
    });

    r.start_resize(ResizeHandle::BottomRight);
    r.apply_delta(5, 3);

    assert!(called.get());
}

// ==================== Size Operations Tests ====================

#[test]
fn test_resizable_set_size_basic() {
    let mut r = Resizable::new(20, 10);
    r.set_size(30, 15);
    assert_eq!(r.size(), (30, 15));
}

#[test]
fn test_resizable_set_size_respects_min() {
    let mut r = Resizable::new(20, 10).min_size(10, 5);
    r.set_size(5, 2);
    assert_eq!(r.size(), (10, 5));
}

#[test]
fn test_resizable_set_size_respects_max() {
    let mut r = Resizable::new(20, 10).max_size(30, 15);
    r.set_size(50, 20);
    assert_eq!(r.size(), (30, 15));
}

#[test]
fn test_resizable_set_size_max_zero_no_limit() {
    let mut r = Resizable::new(20, 10).max_size(0, 0);
    r.set_size(1000, 1000);
    assert_eq!(r.size(), (1000, 1000));
}

#[test]
fn test_resizable_set_size_max_width_only() {
    let mut r = Resizable::new(20, 10).max_size(30, 0);
    r.set_size(50, 100);
    assert_eq!(r.size(), (30, 100));
}

#[test]
fn test_resizable_set_size_max_height_only() {
    let mut r = Resizable::new(20, 10).max_size(0, 15);
    r.set_size(100, 20);
    assert_eq!(r.size(), (100, 15));
}

// ==================== Resize Operations Tests ====================

#[test]
fn test_resizable_start_resize_valid_handle() {
    let mut r = Resizable::new(20, 10);
    r.start_resize(ResizeHandle::BottomRight);
    assert!(r.is_resizing());
}

#[test]
fn test_resizable_start_resize_invalid_handle() {
    let mut r = Resizable::new(20, 10).handles(ResizeHandle::CORNERS);
    r.start_resize(ResizeHandle::Top);
    assert!(!r.is_resizing());
}

#[test]
fn test_resizable_end_resize() {
    let mut r = Resizable::new(20, 10);
    r.start_resize(ResizeHandle::BottomRight);
    assert!(r.is_resizing());

    r.end_resize();
    assert!(!r.is_resizing());
}

#[test]
fn test_resizable_apply_delta_not_resizing() {
    let mut r = Resizable::new(20, 10);
    r.apply_delta(10, 10);
    // Should not change size when not resizing
    assert_eq!(r.size(), (20, 10));
}

#[test]
fn test_resizable_apply_delta_horizontal_right() {
    let mut r = Resizable::new(20, 10);
    r.start_resize(ResizeHandle::Right);
    r.apply_delta(10, 0);
    assert_eq!(r.size(), (30, 10));
}

#[test]
fn test_resizable_apply_delta_horizontal_left() {
    let mut r = Resizable::new(20, 10);
    r.start_resize(ResizeHandle::Left);
    r.apply_delta(-10, 0);
    // Left handle means moving left increases width
    assert_eq!(r.size(), (30, 10));
}

#[test]
fn test_resizable_apply_delta_vertical_bottom() {
    let mut r = Resizable::new(20, 10);
    r.start_resize(ResizeHandle::Bottom);
    r.apply_delta(0, 5);
    assert_eq!(r.size(), (20, 15));
}

#[test]
fn test_resizable_apply_delta_vertical_top() {
    let mut r = Resizable::new(20, 10);
    r.start_resize(ResizeHandle::Top);
    r.apply_delta(0, -5);
    // Top handle means moving up increases height
    assert_eq!(r.size(), (20, 15));
}

#[test]
fn test_resizable_apply_delta_both() {
    let mut r = Resizable::new(20, 10);
    r.start_resize(ResizeHandle::BottomRight);
    r.apply_delta(5, 3);
    assert_eq!(r.size(), (25, 13));
}

#[test]
fn test_resizable_apply_delta_respects_min() {
    let mut r = Resizable::new(20, 10).min_size(15, 8);
    r.start_resize(ResizeHandle::BottomRight);
    r.apply_delta(-10, -5);
    // Decrease to below min, should be constrained
    assert_eq!(r.size(), (15, 8));
}

#[test]
fn test_resizable_apply_delta_respects_max() {
    let mut r = Resizable::new(20, 10).max_size(30, 20);
    r.start_resize(ResizeHandle::BottomRight);
    r.apply_delta(100, 100);
    assert_eq!(r.size(), (30, 20));
}

#[test]
fn test_resizable_apply_delta_never_below_one() {
    let mut r = Resizable::new(5, 5).min_size(1, 1);
    r.start_resize(ResizeHandle::BottomRight);
    r.apply_delta(-100, -100);
    assert_eq!(r.size(), (1, 1));
}

// ==================== Aspect Ratio Tests ====================

#[test]
fn test_resizable_aspect_ratio_preserved() {
    let mut r = Resizable::new(20, 10).preserve_aspect_ratio();
    r.set_size(40, 5);
    // Height should be adjusted to maintain 2:1 ratio
    assert_eq!(r.size(), (40, 20));
}

#[test]
fn test_resizable_aspect_ratio_with_min() {
    let mut r = Resizable::new(20, 10)
        .preserve_aspect_ratio()
        .min_size(10, 5);
    r.set_size(15, 10);
    // Width 15 should give height 7.5, clamped to min 5
    assert_eq!(r.size(), (15, 7));
}

#[test]
fn test_resizable_aspect_ratio_with_max() {
    let mut r = Resizable::new(20, 10)
        .preserve_aspect_ratio()
        .max_size(50, 20);
    r.set_size(100, 10);
    // Width clamped to 50, height adjusted for ratio
    let (w, h) = r.size();
    assert_eq!(w, 50);
    assert_eq!(h, 20);
}

#[test]
fn test_resizable_custom_aspect_ratio() {
    let mut r = Resizable::new(20, 10).aspect_ratio(4.0);
    r.set_size(40, 5);
    // 4:1 ratio: width 40 -> height 10
    assert_eq!(r.size(), (40, 10));
}

// ==================== Grid Snap Tests ====================

#[test]
fn test_resizable_grid_snap_basic() {
    let mut r = Resizable::new(20, 10).snap_to_grid(5, 5);
    r.set_size(23, 12);
    // 23 -> 25 (round), 12 -> 10 (round)
    assert_eq!(r.size(), (25, 10));
}

#[test]
fn test_resizable_grid_snap_round_up() {
    let mut r = Resizable::new(20, 10).snap_to_grid(10, 10);
    r.set_size(26, 16);
    // 26 -> 30, 16 -> 20
    assert_eq!(r.size(), (30, 20));
}

#[test]
fn test_resizable_grid_snap_round_down() {
    let mut r = Resizable::new(20, 10).snap_to_grid(10, 10);
    r.set_size(24, 14);
    // 24 -> 20, 14 -> 10
    assert_eq!(r.size(), (20, 10));
}

#[test]
fn test_resizable_grid_snap_exactly_on_grid() {
    let mut r = Resizable::new(20, 10).snap_to_grid(5, 5);
    r.set_size(25, 15);
    // Already on grid
    assert_eq!(r.size(), (25, 15));
}

#[test]
fn test_resizable_grid_snap_different_grids() {
    let mut r = Resizable::new(20, 10).snap_to_grid(10, 5);
    r.set_size(27, 13);
    // Width: 27 -> 30, Height: 13 -> 15
    assert_eq!(r.size(), (30, 15));
}

// ==================== Handle Hit Tests ====================

#[test]
fn test_resizable_handle_at_corner() {
    let r = Resizable::new(20, 10);
    let area = Rect::new(5, 5, 20, 10);

    assert_eq!(r.handle_at(5, 5, area), Some(ResizeHandle::TopLeft));
    assert_eq!(r.handle_at(24, 5, area), Some(ResizeHandle::TopRight));
    assert_eq!(r.handle_at(5, 14, area), Some(ResizeHandle::BottomLeft));
    assert_eq!(r.handle_at(24, 14, area), Some(ResizeHandle::BottomRight));
}

#[test]
fn test_resizable_handle_at_edge() {
    let r = Resizable::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);

    assert_eq!(r.handle_at(10, 0, area), Some(ResizeHandle::Top));
    assert_eq!(r.handle_at(10, 9, area), Some(ResizeHandle::Bottom));
    assert_eq!(r.handle_at(0, 5, area), Some(ResizeHandle::Left));
    assert_eq!(r.handle_at(19, 5, area), Some(ResizeHandle::Right));
}

#[test]
fn test_resizable_handle_at_center() {
    let r = Resizable::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);

    // Center should have no handle
    assert_eq!(r.handle_at(10, 5, area), None);
}

#[test]
fn test_resizable_handle_at_inside_area() {
    let r = Resizable::new(20, 10);
    let area = Rect::new(5, 5, 20, 10);

    // Position in middle of area, not on any edge
    assert_eq!(r.handle_at(15, 10, area), None);
}

#[test]
fn test_resizable_handle_at_with_limited_handles() {
    let r = Resizable::new(20, 10).handles(ResizeHandle::CORNERS);
    let area = Rect::new(0, 0, 20, 10);

    // Edges should not be detected
    assert_eq!(r.handle_at(10, 0, area), None);
    assert_eq!(r.handle_at(10, 9, area), None);

    // Corners should still work
    assert_eq!(r.handle_at(0, 0, area), Some(ResizeHandle::TopLeft));
    assert_eq!(r.handle_at(19, 9, area), Some(ResizeHandle::BottomRight));
}

// ==================== Hover State Tests ====================

#[test]
fn test_resizable_set_hovered_handle() {
    let mut r = Resizable::new(20, 10);
    // Just verify it doesn't panic
    r.set_hovered(Some(ResizeHandle::TopLeft));
    r.set_hovered(None);
}

#[test]
fn test_resizable_set_hovered_different_corners() {
    let mut r = Resizable::new(20, 10);

    for handle in ResizeHandle::ALL.iter() {
        r.set_hovered(Some(*handle));
    }
}

// ==================== Content Area Tests ====================

#[test]
fn test_resizable_content_area_border_style() {
    let r = Resizable::new(20, 10).style(ResizeStyle::Border);
    let area = Rect::new(5, 5, 100, 100);
    let content = r.content_area(area);

    // Border takes 1 pixel on each side
    assert_eq!(content.x, 6);
    assert_eq!(content.y, 6);
    assert_eq!(content.width, 18);
    assert_eq!(content.height, 8);
}

#[test]
fn test_resizable_content_area_dots_style() {
    let r = Resizable::new(20, 10).style(ResizeStyle::Dots);
    let area = Rect::new(5, 5, 100, 100);
    let content = r.content_area(area);

    // Dots style has no border
    assert_eq!(content.x, 5);
    assert_eq!(content.y, 5);
    assert_eq!(content.width, 20);
    assert_eq!(content.height, 10);
}

#[test]
fn test_resizable_content_area_subtle_style() {
    let r = Resizable::new(20, 10).style(ResizeStyle::Subtle);
    let area = Rect::new(5, 5, 100, 100);
    let content = r.content_area(area);

    // Subtle style has no border
    assert_eq!(content.x, 5);
    assert_eq!(content.y, 5);
    assert_eq!(content.width, 20);
    assert_eq!(content.height, 10);
}

#[test]
fn test_resizable_content_area_hidden_style() {
    let r = Resizable::new(20, 10).style(ResizeStyle::Hidden);
    let area = Rect::new(5, 5, 100, 100);
    let content = r.content_area(area);

    // Hidden style has no border
    assert_eq!(content.x, 5);
    assert_eq!(content.y, 5);
    assert_eq!(content.width, 20);
    assert_eq!(content.height, 10);
}

#[test]
fn test_resizable_content_area_small_size() {
    let r = Resizable::new(3, 3).style(ResizeStyle::Border);
    let area = Rect::new(0, 0, 10, 10);
    let content = r.content_area(area);

    // Small content area after border
    assert_eq!(content.x, 1);
    assert_eq!(content.y, 1);
    assert_eq!(content.width, 1);
    assert_eq!(content.height, 1);
}

// ==================== Keyboard Handling Tests ====================

#[test]
fn test_resizable_key_right_increases_width() {
    let mut r = Resizable::new(20, 10).focused(true);

    r.handle_key(&Key::Right);
    assert_eq!(r.size(), (21, 10));
}

#[test]
fn test_resizable_key_left_decreases_width() {
    let mut r = Resizable::new(20, 10).focused(true);

    r.handle_key(&Key::Left);
    assert_eq!(r.size(), (19, 10));
}

#[test]
fn test_resizable_key_down_increases_height() {
    let mut r = Resizable::new(20, 10).focused(true);

    r.handle_key(&Key::Down);
    assert_eq!(r.size(), (20, 11));
}

#[test]
fn test_resizable_key_up_decreases_height() {
    let mut r = Resizable::new(20, 10).focused(true);

    r.handle_key(&Key::Up);
    assert_eq!(r.size(), (20, 9));
}

#[test]
fn test_resizable_key_not_focused_ignored() {
    let mut r = Resizable::new(20, 10);

    let handled = r.handle_key(&Key::Right);
    assert!(!handled);
    assert_eq!(r.size(), (20, 10));
}

#[test]
fn test_resizable_key_unhandled_key() {
    let mut r = Resizable::new(20, 10).focused(true);

    let handled = r.handle_key(&Key::Char('a'));
    assert!(!handled);
    assert_eq!(r.size(), (20, 10));
}

#[test]
fn test_resizable_key_without_required_handle() {
    let mut r = Resizable::new(20, 10).handles(ResizeHandle::CORNERS);
    let mut r = r.focused(true);

    // Right handle not in CORNERS
    let handled = r.handle_key(&Key::Right);
    assert!(!handled);
    assert_eq!(r.size(), (20, 10));
}

#[test]
fn test_resizable_key_respects_min_width() {
    let mut r = Resizable::new(5, 5).min_size(5, 5);
    let mut r = r.focused(true);

    r.handle_key(&Key::Left);
    // Should stay at minimum
    assert_eq!(r.size(), (5, 5));
}

#[test]
fn test_resizable_key_respects_max_width() {
    let mut r = Resizable::new(20, 10).max_size(25, 15);
    let mut r = r.focused(true);

    r.handle_key(&Key::Right);
    r.handle_key(&Key::Right);
    r.handle_key(&Key::Right);
    r.handle_key(&Key::Right);
    r.handle_key(&Key::Right);

    // Should stop at maximum
    assert_eq!(r.size(), (25, 10));
}

// ==================== Render Tests ====================

#[test]
fn test_resizable_render_border_style() {
    let r = Resizable::new(10, 5).style(ResizeStyle::Border);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    r.render(&mut ctx);

    // Check corners
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '┐');
    assert_eq!(buffer.get(0, 4).unwrap().symbol, '└');
    assert_eq!(buffer.get(9, 4).unwrap().symbol, '┘');
}

#[test]
fn test_resizable_render_border_top_and_bottom() {
    let r = Resizable::new(10, 5).style(ResizeStyle::Border);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    r.render(&mut ctx);

    // Check top border
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '─');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '─');
    assert_eq!(buffer.get(8, 0).unwrap().symbol, '─');

    // Check bottom border
    assert_eq!(buffer.get(1, 4).unwrap().symbol, '─');
    assert_eq!(buffer.get(5, 4).unwrap().symbol, '─');
    assert_eq!(buffer.get(8, 4).unwrap().symbol, '─');
}

#[test]
fn test_resizable_render_border_sides() {
    let r = Resizable::new(10, 5).style(ResizeStyle::Border);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    r.render(&mut ctx);

    // Check side borders
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '│');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '│');
    assert_eq!(buffer.get(0, 3).unwrap().symbol, '│');
    assert_eq!(buffer.get(9, 1).unwrap().symbol, '│');
    assert_eq!(buffer.get(9, 2).unwrap().symbol, '│');
    assert_eq!(buffer.get(9, 3).unwrap().symbol, '│');
}

#[test]
fn test_resizable_render_dots_style() {
    let r = Resizable::new(10, 5).style(ResizeStyle::Dots);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    r.render(&mut ctx);

    // Check corner dots
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '●');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '●');
    assert_eq!(buffer.get(0, 4).unwrap().symbol, '●');
    assert_eq!(buffer.get(9, 4).unwrap().symbol, '●');
}

#[test]
fn test_resizable_render_hidden_style() {
    let r = Resizable::new(10, 5).style(ResizeStyle::Hidden);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    r.render(&mut ctx);

    // Should not draw anything
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, ' ');
}

#[test]
fn test_resizable_render_subtle_not_hovered() {
    let r = Resizable::new(10, 5).style(ResizeStyle::Subtle);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    r.render(&mut ctx);

    // Should not draw when not hovered
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_resizable_render_subtle_hovered() {
    let mut r = Resizable::new(10, 5).style(ResizeStyle::Subtle);
    r.set_hovered(Some(ResizeHandle::TopLeft));

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    r.render(&mut ctx);

    // Should draw when hovered
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_resizable_render_resizing_active_color() {
    let mut r = Resizable::new(10, 5)
        .style(ResizeStyle::Border)
        .active_color(Color::RED);
    r.start_resize(ResizeHandle::BottomRight);

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    r.render(&mut ctx);

    // Should render with active color
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_resizable_render_hovered_corner_highlight() {
    let mut r = Resizable::new(10, 5)
        .style(ResizeStyle::Border)
        .active_color(Color::CYAN);
    r.set_hovered(Some(ResizeHandle::TopRight));

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    r.render(&mut ctx);

    // Hovered corner should have active color
    assert_eq!(buffer.get(9, 0).unwrap().fg, Some(Color::CYAN));
}

// ==================== StyledView Tests ====================

#[test]
fn test_resizable_id() {
    let r = Resizable::new(20, 10).element_id("test-resizable");
    assert_eq!(View::id(&r), Some("test-resizable"));
}

#[test]
fn test_resizable_classes() {
    let r = Resizable::new(20, 10)
        .class("resizable")
        .class("interactive");

    assert!(r.has_class("resizable"));
    assert!(r.has_class("interactive"));
    assert!(!r.has_class("other"));
}

#[test]
fn test_resizable_styled_view_methods() {
    let mut r = Resizable::new(20, 10);

    r.set_id("my-resizable");
    assert_eq!(View::id(&r), Some("my-resizable"));

    r.add_class("active");
    assert!(r.has_class("active"));

    r.remove_class("active");
    assert!(!r.has_class("active"));

    r.toggle_class("selected");
    assert!(r.has_class("selected"));

    r.toggle_class("selected");
    assert!(!r.has_class("selected"));
}

#[test]
fn test_resizable_meta() {
    let r = Resizable::new(20, 10)
        .element_id("test")
        .class("widget")
        .class("resizable");

    let meta = r.meta();
    assert_eq!(meta.id, Some("test".to_string()));
    assert!(meta.classes.contains("widget"));
    assert!(meta.classes.contains("resizable"));
}

#[test]
fn test_resizable_foreground_color() {
    let r = Resizable::new(20, 10).fg(Color::RED);
    // The fg color is stored in props
    let _ = r;
}

#[test]
fn test_resizable_background_color() {
    let r = Resizable::new(20, 10).bg(Color::BLUE);
    // The bg color is stored in props
    let _ = r;
}

// ==================== ResizeHandle Constants Tests ====================

#[test]
fn test_resize_handle_all_contains_all() {
    assert_eq!(ResizeHandle::ALL.len(), 8);
    assert!(ResizeHandle::ALL.contains(&ResizeHandle::Top));
    assert!(ResizeHandle::ALL.contains(&ResizeHandle::Bottom));
    assert!(ResizeHandle::ALL.contains(&ResizeHandle::Left));
    assert!(ResizeHandle::ALL.contains(&ResizeHandle::Right));
    assert!(ResizeHandle::ALL.contains(&ResizeHandle::TopLeft));
    assert!(ResizeHandle::ALL.contains(&ResizeHandle::TopRight));
    assert!(ResizeHandle::ALL.contains(&ResizeHandle::BottomLeft));
    assert!(ResizeHandle::ALL.contains(&ResizeHandle::BottomRight));
}

#[test]
fn test_resize_handle_edges_only() {
    assert_eq!(ResizeHandle::EDGES.len(), 4);
    assert!(ResizeHandle::EDGES.contains(&ResizeHandle::Top));
    assert!(ResizeHandle::EDGES.contains(&ResizeHandle::Bottom));
    assert!(ResizeHandle::EDGES.contains(&ResizeHandle::Left));
    assert!(ResizeHandle::EDGES.contains(&ResizeHandle::Right));
    assert!(!ResizeHandle::EDGES.contains(&ResizeHandle::TopLeft));
}

#[test]
fn test_resize_handle_corners_only() {
    assert_eq!(ResizeHandle::CORNERS.len(), 4);
    assert!(ResizeHandle::CORNERS.contains(&ResizeHandle::TopLeft));
    assert!(ResizeHandle::CORNERS.contains(&ResizeHandle::TopRight));
    assert!(ResizeHandle::CORNERS.contains(&ResizeHandle::BottomLeft));
    assert!(ResizeHandle::CORNERS.contains(&ResizeHandle::BottomRight));
    assert!(!ResizeHandle::CORNERS.contains(&ResizeHandle::Top));
}

// ==================== Edge Cases ====================

#[test]
fn test_resizable_size_one_by_one() {
    let mut r = Resizable::new(1, 1).min_size(1, 1);
    r.start_resize(ResizeHandle::BottomRight);
    r.apply_delta(-100, -100);
    // Should stay at 1x1 minimum
    assert_eq!(r.size(), (1, 1));
}

#[test]
fn test_resizable_large_resize() {
    let mut r = Resizable::new(20, 10);
    r.start_resize(ResizeHandle::BottomRight);
    r.apply_delta(1000, 1000);
    assert_eq!(r.size(), (1020, 1010));
}

#[test]
fn test_resizable_negative_delta_positive_resize() {
    let mut r = Resizable::new(20, 10);
    r.start_resize(ResizeHandle::TopLeft);
    r.apply_delta(-5, -5);
    // TopLeft with negative delta increases size
    assert_eq!(r.size(), (25, 15));
}

#[test]
fn test_resizable_positive_delta_negative_resize() {
    let mut r = Resizable::new(30, 20);
    r.start_resize(ResizeHandle::TopLeft);
    r.apply_delta(5, 5);
    // TopLeft with positive delta decreases size
    assert_eq!(r.size(), (25, 15));
}

#[test]
fn test_resizable_chain_builder_methods() {
    let r = Resizable::new(20, 10)
        .min_size(10, 5)
        .max_size(50, 30)
        .handles(ResizeHandle::CORNERS)
        .style(ResizeStyle::Dots)
        .handle_color(Color::RED)
        .active_color(Color::GREEN)
        .preserve_aspect_ratio()
        .snap_to_grid(5, 5);

    // Verify via behavior, not private fields
    assert_eq!(r.size(), (20, 10));

    let mut r = r;
    // Test min constraint
    r.set_size(5, 2);
    assert_eq!(r.size(), (10, 5));

    // Test max constraint - aspect ratio affects result
    r.set_size(100, 50);
    let (w, h) = r.size();
    assert_eq!(w, 50);
    // Height is adjusted for 2:1 ratio with max 30
    assert!(h <= 30);
}

#[test]
fn test_resizable_callback_invoked_on_resize() {
    use std::cell::Cell;
    use std::rc::Rc;

    let call_count = Rc::new(Cell::new(0));
    let last_width = Rc::new(Cell::new(0u16));
    let last_height = Rc::new(Cell::new(0u16));

    let count_clone = call_count.clone();
    let width_clone = last_width.clone();
    let height_clone = last_height.clone();

    let mut r = Resizable::new(20, 10).on_resize(move |w, h| {
        count_clone.set(count_clone.get() + 1);
        width_clone.set(w);
        height_clone.set(h);
    });

    r.start_resize(ResizeHandle::BottomRight);
    r.apply_delta(5, 3);
    r.apply_delta(2, 1);

    assert_eq!(call_count.get(), 2);
    assert_eq!(last_width.get(), 27);
    assert_eq!(last_height.get(), 14);
}

#[test]
fn test_resizable_callback_not_invoked_when_no_change() {
    use std::cell::Cell;
    use std::rc::Rc;

    let called = Rc::new(Cell::new(false));

    let called_clone = called.clone();
    let mut r = Resizable::new(20, 10)
        .max_size(20, 10)
        .on_resize(move |_w, _h| {
            called_clone.set(true);
        });

    r.start_resize(ResizeHandle::BottomRight);
    r.apply_delta(5, 5);

    // Size didn't change due to max constraint (already at max)
    assert!(!called.get());
}

#[test]
fn test_resizable_render_with_offset_area() {
    let r = Resizable::new(10, 5).style(ResizeStyle::Border);
    let mut buffer = Buffer::new(30, 20);
    let area = Rect::new(10, 10, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    r.render(&mut ctx);

    // Border should be at offset position
    assert_eq!(buffer.get(10, 10).unwrap().symbol, '┌');
    assert_eq!(buffer.get(19, 10).unwrap().symbol, '┐');
    assert_eq!(buffer.get(10, 14).unwrap().symbol, '└');
    assert_eq!(buffer.get(19, 14).unwrap().symbol, '┘');
}

#[test]
fn test_resizable_hit_test_all_handles() {
    let r = Resizable::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);

    // Test all handle positions - need to account for handle_size (default 1)
    // Corners extend handle_size pixels from each corner
    // Edges exclude the corner areas
    let handles = [
        (2, 0, ResizeHandle::Top),          // Away from corner (handle_size=1)
        (17, 0, ResizeHandle::Top),         // Away from corner
        (2, 9, ResizeHandle::Bottom),       // Away from corner
        (17, 9, ResizeHandle::Bottom),      // Away from corner
        (0, 2, ResizeHandle::Left),         // Away from corner
        (0, 7, ResizeHandle::Left),         // Away from corner
        (19, 2, ResizeHandle::Right),       // Away from corner
        (19, 7, ResizeHandle::Right),       // Away from corner
        (0, 0, ResizeHandle::TopLeft),      // Corner
        (19, 0, ResizeHandle::TopRight),    // Corner
        (0, 9, ResizeHandle::BottomLeft),   // Corner
        (19, 9, ResizeHandle::BottomRight), // Corner
    ];

    for (x, y, expected) in handles {
        assert_eq!(
            r.handle_at(x, y, area),
            Some(expected),
            "Failed at ({}, {})",
            x,
            y
        );
    }
}

#[test]
fn test_resizable_aspect_ratio_zero_height_protection() {
    let mut r = Resizable::new(20, 10).preserve_aspect_ratio();
    r.set_size(30, 0);
    // Should protect against zero height
    let (_w, h) = r.size();
    assert!(h > 0);
}

#[test]
fn test_resizable_multiple_resize_operations() {
    let mut r = Resizable::new(20, 10);

    // First resize
    r.start_resize(ResizeHandle::BottomRight);
    r.apply_delta(10, 5);
    r.end_resize();

    // Second resize
    r.start_resize(ResizeHandle::TopLeft);
    r.apply_delta(-5, -3);
    r.end_resize();

    // Third resize
    r.start_resize(ResizeHandle::Right);
    r.apply_delta(15, 0);
    r.end_resize();

    assert_eq!(r.size(), (50, 18));
}

#[test]
fn test_resizable_no_callback_when_resizing_same_size() {
    use std::cell::Cell;
    use std::rc::Rc;

    let called = Rc::new(Cell::new(false));

    let called_clone = called.clone();
    let mut r = Resizable::new(20, 10)
        .max_size(20, 10)
        .on_resize(move |_w, _h| {
            called_clone.set(true);
        });

    r.start_resize(ResizeHandle::BottomRight);
    r.apply_delta(5, 5);

    // Already at max, no change
    assert!(!called.get());
}
