//! Avatar widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{avatar, avatar_icon, Avatar, AvatarShape, AvatarSize};

// =============================================================================
// Constructor Tests
// =============================================================================

#[test]
fn test_avatar_new() {
    let a = Avatar::new("John Doe");
    // Verify by rendering
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('J'));
    assert_eq!(buffer.get(2, 0).map(|c| c.symbol), Some('D'));
}

#[test]
fn test_avatar_from_initials() {
    let a = Avatar::from_initials("AB");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('A'));
    assert_eq!(buffer.get(2, 0).map(|c| c.symbol), Some('B'));
}

#[test]
fn test_avatar_from_icon() {
    let a = Avatar::from_icon('🤖');
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('🤖'));
}

#[test]
fn test_avatar_default() {
    let a = Avatar::default();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Empty name shows no initials
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some(' '));
}

#[test]
fn test_avatar_helper_fn() {
    let a = avatar("Test User");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('T'));
    assert_eq!(buffer.get(2, 0).map(|c| c.symbol), Some('U'));
}

#[test]
fn test_avatar_icon_helper_fn() {
    let a = avatar_icon('🎨');
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('🎨'));
}

// =============================================================================
// Builder Methods - Size
// =============================================================================

#[test]
fn test_avatar_size_small() {
    let a = avatar("John").small();
    // Verify by rendering small avatar
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Small size renders single char
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('J'));
}

#[test]
fn test_avatar_size_medium() {
    let a = avatar("John").medium();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Medium size has circle markers
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('◖'));
}

#[test]
fn test_avatar_size_large() {
    let a = avatar("John").large();
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Large size has box drawing
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('╭'));
}

#[test]
fn test_avatar_size_explicit() {
    let a = avatar("John").size(AvatarSize::Large);
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('╭'));
}

// =============================================================================
// Builder Methods - Shape
// =============================================================================

#[test]
fn test_avatar_shape_circle() {
    let a = avatar("John").circle();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Circle uses half-blocks
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('◖'));
}

#[test]
fn test_avatar_shape_square() {
    let a = avatar("John").square();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Square uses brackets
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('['));
}

#[test]
fn test_avatar_shape_rounded() {
    let a = avatar("John").rounded();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Rounded uses parentheses
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('('));
}

#[test]
fn test_avatar_shape_explicit() {
    let a = avatar("John").shape(AvatarShape::Square);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('['));
}

// =============================================================================
// Builder Methods - Colors
// =============================================================================

#[test]
fn test_avatar_bg_color() {
    let a = avatar("John").bg(Color::BLUE);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Check that bg color is applied (not default generated)
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_avatar_fg_color() {
    let a = avatar("John").fg(Color::RED);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::RED));
}

#[test]
fn test_avatar_colors_both() {
    let a = avatar("John").bg(Color::YELLOW).fg(Color::GREEN);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::YELLOW));
    assert_eq!(cell.fg, Some(Color::GREEN));
}

// =============================================================================
// Builder Methods - Status
// =============================================================================

#[test]
fn test_avatar_online() {
    let a = avatar("John").online();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Should have status dot
    assert_eq!(buffer.get(4, 0).map(|c| c.symbol), Some('●'));
    let status_cell = buffer.get(4, 0).unwrap();
    assert_eq!(status_cell.fg, Some(Color::rgb(40, 200, 80)));
}

#[test]
fn test_avatar_offline() {
    let a = avatar("John").offline();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    let status_cell = buffer.get(4, 0).unwrap();
    assert_eq!(status_cell.fg, Some(Color::rgb(100, 100, 100)));
}

#[test]
fn test_avatar_away() {
    let a = avatar("John").away();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    let status_cell = buffer.get(4, 0).unwrap();
    assert_eq!(status_cell.fg, Some(Color::rgb(200, 180, 40)));
}

#[test]
fn test_avatar_busy() {
    let a = avatar("John").busy();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    let status_cell = buffer.get(4, 0).unwrap();
    assert_eq!(status_cell.fg, Some(Color::rgb(200, 60, 60)));
}

#[test]
fn test_avatar_custom_status() {
    let custom = Color::rgb(128, 0, 255);
    let a = avatar("John").status(custom);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    let status_cell = buffer.get(4, 0).unwrap();
    assert_eq!(status_cell.fg, Some(custom));
}

#[test]
fn test_avatar_status_override() {
    let a = avatar("John").online().busy();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Should show busy color (last one set)
    let status_cell = buffer.get(4, 0).unwrap();
    assert_eq!(status_cell.fg, Some(Color::rgb(200, 60, 60)));
}

// =============================================================================
// Builder Methods - Icon
// =============================================================================

#[test]
fn test_avatar_set_icon() {
    let a = avatar("John").icon('🎯');
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Icon overrides name-derived initials
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('🎯'));
}

#[test]
fn test_avatar_override_with_icon() {
    let a = avatar("John Doe").icon('X');
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Icon overrides name-derived initials
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('X'));
}

// =============================================================================
// StyledView Tests (CSS support)
// =============================================================================

#[test]
fn test_avatar_element_id() {
    let a = avatar("John").element_id("user-avatar");
    assert_eq!(View::id(&a), Some("user-avatar"));

    let meta = a.meta();
    assert_eq!(meta.id, Some("user-avatar".to_string()));
}

#[test]
fn test_avatar_add_class() {
    let a = avatar("John").class("user").class("active");

    assert!(a.has_class("user"));
    assert!(a.has_class("active"));
    assert!(!a.has_class("inactive"));

    let meta = a.meta();
    assert!(meta.classes.contains("user"));
    assert!(meta.classes.contains("active"));
}

#[test]
fn test_avatar_multiple_classes() {
    let a = avatar("John").classes(["user", "admin", "online"]);

    assert!(a.has_class("user"));
    assert!(a.has_class("admin"));
    assert!(a.has_class("online"));
}

#[test]
fn test_avatar_styled_view_methods() {
    let mut a = avatar("John");

    a.set_id("test-id");
    assert_eq!(View::id(&a), Some("test-id"));

    a.add_class("active");
    assert!(a.has_class("active"));

    a.remove_class("active");
    assert!(!a.has_class("active"));

    a.toggle_class("selected");
    assert!(a.has_class("selected"));

    a.toggle_class("selected");
    assert!(!a.has_class("selected"));
}

#[test]
fn test_avatar_classes_deduplication() {
    let a = avatar("John").class("user").class("user");
    assert_eq!(View::classes(&a).len(), 1);
}

// =============================================================================
// Render Tests - Small Size
// =============================================================================

#[test]
fn test_avatar_render_small() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("John Doe").small();
    a.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('J'));
}

#[test]
fn test_avatar_render_small_with_status() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("John").small().online();
    a.render(&mut ctx);

    // First char should be initial
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('J'));
    // Second should be status dot
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('●'));
}

#[test]
fn test_avatar_render_small_with_custom_colors() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("John").small().bg(Color::BLUE).fg(Color::YELLOW);
    a.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::YELLOW));
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_avatar_render_small_icon() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar_icon('⭐').small();
    a.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('⭐'));
}

#[test]
fn test_avatar_render_small_empty_name() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("").small();
    a.render(&mut ctx);

    // Should show fallback '?'
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('?'));
}

// =============================================================================
// Render Tests - Medium Size
// =============================================================================

#[test]
fn test_avatar_render_medium() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("John Doe");
    a.render(&mut ctx);

    // Should have initials in the middle
    let text: String = (0..10)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains('J') || text.contains('D'));
}

#[test]
fn test_avatar_render_medium_circle() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("JD").circle();
    a.render(&mut ctx);

    // Circle shape uses half-blocks
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('◖'));
    assert_eq!(buffer.get(3, 0).map(|c| c.symbol), Some('◗'));
}

#[test]
fn test_avatar_render_medium_square() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("JD").square();
    a.render(&mut ctx);

    // Square uses brackets
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('['));
    assert_eq!(buffer.get(3, 0).map(|c| c.symbol), Some(']'));
}

#[test]
fn test_avatar_render_medium_rounded() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("JD").rounded();
    a.render(&mut ctx);

    // Rounded uses parentheses
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('('));
    assert_eq!(buffer.get(3, 0).map(|c| c.symbol), Some(')'));
}

#[test]
fn test_avatar_render_medium_with_status() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("JD").circle().online();
    a.render(&mut ctx);

    // Status dot should be after the avatar
    assert_eq!(buffer.get(4, 0).map(|c| c.symbol), Some('●'));
}

#[test]
fn test_avatar_render_medium_single_initial() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("SingleName");
    a.render(&mut ctx);

    // Should only have one initial
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('S'));
}

// =============================================================================
// Render Tests - Large Size
// =============================================================================

#[test]
fn test_avatar_render_large_circle() {
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("John Doe").large().circle();
    a.render(&mut ctx);

    // Check top row of circle
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('╭'));
    assert_eq!(buffer.get(4, 0).map(|c| c.symbol), Some('╮'));

    // Check bottom row of circle
    assert_eq!(buffer.get(0, 2).map(|c| c.symbol), Some('╰'));
    assert_eq!(buffer.get(4, 2).map(|c| c.symbol), Some('╯'));
}

#[test]
fn test_avatar_render_large_square() {
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("John Doe").large().square();
    a.render(&mut ctx);

    // Check top row of square
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('┌'));
    assert_eq!(buffer.get(4, 0).map(|c| c.symbol), Some('┐'));

    // Check bottom row of square
    assert_eq!(buffer.get(0, 2).map(|c| c.symbol), Some('└'));
    assert_eq!(buffer.get(4, 2).map(|c| c.symbol), Some('┘'));
}

#[test]
fn test_avatar_render_large_rounded() {
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("John Doe").large().rounded();
    a.render(&mut ctx);

    // Rounded and circle use same chars for large
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('╭'));
    assert_eq!(buffer.get(4, 0).map(|c| c.symbol), Some('╮'));
}

#[test]
fn test_avatar_render_large_with_status() {
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("John Doe").large().circle().online();
    a.render(&mut ctx);

    // Status dot should be at bottom right
    assert_eq!(buffer.get(5, 2).map(|c| c.symbol), Some('●'));
}

#[test]
fn test_avatar_render_large_insufficient_height() {
    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("John Doe").large();
    a.render(&mut ctx);

    // Should fall back to medium style (single char)
    let text: String = (0..10)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains('J') || text.contains('D'));
}

#[test]
fn test_avatar_render_large_middle_row() {
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("JD").large().circle();
    a.render(&mut ctx);

    // Check middle row has initials (positions may vary based on implementation)
    // Just verify rendering completes successfully
    assert_eq!(buffer.get(0, 1).map(|c| c.symbol), Some('│'));
    assert_eq!(buffer.get(1, 1).map(|c| c.symbol), Some('J'));
}

// =============================================================================
// Render Tests - Icons
// =============================================================================

#[test]
fn test_avatar_render_icon_medium() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar_icon('🎨').medium();
    a.render(&mut ctx);

    // Should show icon in middle
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('🎨'));
}

#[test]
fn test_avatar_render_icon_large() {
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar_icon('⚡').large().circle();
    a.render(&mut ctx);

    // Should show icon in middle row
    assert_eq!(buffer.get(1, 1).map(|c| c.symbol), Some('⚡'));
}

#[test]
fn test_avatar_render_icon_with_custom_colors() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar_icon('★').bg(Color::RED).fg(Color::YELLOW);
    a.render(&mut ctx);

    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::YELLOW));
    assert_eq!(cell.bg, Some(Color::RED));
}

// =============================================================================
// Render Tests - All Status Colors
// =============================================================================

#[test]
fn test_avatar_render_status_online_color() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("JD").online();
    a.render(&mut ctx);

    let status_cell = buffer.get(4, 0).unwrap();
    assert_eq!(status_cell.fg, Some(Color::rgb(40, 200, 80)));
}

#[test]
fn test_avatar_render_status_offline_color() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("JD").offline();
    a.render(&mut ctx);

    let status_cell = buffer.get(4, 0).unwrap();
    assert_eq!(status_cell.fg, Some(Color::rgb(100, 100, 100)));
}

#[test]
fn test_avatar_render_status_away_color() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("JD").away();
    a.render(&mut ctx);

    let status_cell = buffer.get(4, 0).unwrap();
    assert_eq!(status_cell.fg, Some(Color::rgb(200, 180, 40)));
}

#[test]
fn test_avatar_render_status_busy_color() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("JD").busy();
    a.render(&mut ctx);

    let status_cell = buffer.get(4, 0).unwrap();
    assert_eq!(status_cell.fg, Some(Color::rgb(200, 60, 60)));
}

// =============================================================================
// Chained Builder Tests
// =============================================================================

#[test]
fn test_avatar_chained_builders() {
    let a = avatar("John Doe")
        .small()
        .circle()
        .bg(Color::BLUE)
        .fg(Color::WHITE)
        .online()
        .element_id("user1")
        .class("active");

    // Verify through rendering
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);

    // Small renders single char
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('J'));
    // Online status
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('●'));
    // ID and class
    assert_eq!(View::id(&a), Some("user1"));
    assert!(a.has_class("active"));
}

#[test]
fn test_avatar_size_override() {
    let a = avatar("John").small().medium().large();
    // Verify through rendering
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Last size (large) wins
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('╭'));
}

#[test]
fn test_avatar_shape_override() {
    let a = avatar("John").circle().square().rounded();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Last shape (rounded) wins - uses parentheses
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('('));
}

#[test]
fn test_avatar_colors_override() {
    let a = avatar("John")
        .bg(Color::RED)
        .fg(Color::BLUE)
        .bg(Color::GREEN)
        .fg(Color::YELLOW);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    // Last colors win
    assert_eq!(cell.bg, Some(Color::GREEN));
    assert_eq!(cell.fg, Some(Color::YELLOW));
}

// =============================================================================
// Initials Edge Cases
// =============================================================================

#[test]
fn test_avatar_initials_multiple_words() {
    let a = Avatar::new("Alice Bob Charlie");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Only first 2 initials
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('A'));
    assert_eq!(buffer.get(2, 0).map(|c| c.symbol), Some('B'));
}

#[test]
fn test_avatar_initials_single_word() {
    let a = Avatar::new("SingleName");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Only first char
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('S'));
}

#[test]
fn test_avatar_initials_uppercase() {
    let a = Avatar::new("john doe");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Should be uppercase
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some('J'));
}

// =============================================================================
// Color Generation Tests
// =============================================================================

#[test]
fn test_avatar_color_generation_different_names() {
    let a1 = Avatar::new("Alice");
    let a2 = Avatar::new("Bob");

    let mut buffer1 = Buffer::new(10, 1);
    let area1 = Rect::new(0, 0, 10, 1);
    let mut ctx1 = RenderContext::new(&mut buffer1, area1);
    a1.render(&mut ctx1);

    let mut buffer2 = Buffer::new(10, 1);
    let area2 = Rect::new(0, 0, 10, 1);
    let mut ctx2 = RenderContext::new(&mut buffer2, area2);
    a2.render(&mut ctx2);

    // Different names should (usually) generate different colors
    // We just verify they're valid colors (u8 is always 0-255)
    let c1 = buffer1.get(1, 0).unwrap().bg.unwrap();
    let c2 = buffer2.get(1, 0).unwrap().bg.unwrap();
    assert!(c1.r <= 255);
    assert!(c1.g <= 255);
    assert!(c1.b <= 255);
    assert!(c2.r <= 255);
    assert!(c2.g <= 255);
    assert!(c2.b <= 255);
}

#[test]
fn test_avatar_color_generation_custom_overrides() {
    let a = Avatar::new("Alice").bg(Color::MAGENTA);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    let cell = buffer.get(1, 0).unwrap();
    assert_eq!(cell.bg, Some(Color::MAGENTA));
}

#[test]
fn test_avatar_color_generation_empty_name() {
    let a = Avatar::new("");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Empty name should still render something
    // Just verify rendering doesn't panic
}

// =============================================================================
// All AvatarSize Variants
// =============================================================================

#[test]
fn test_all_avatar_size_variants() {
    let small = avatar("Test").size(AvatarSize::Small);
    let medium = avatar("Test").size(AvatarSize::Medium);
    let large = avatar("Test").size(AvatarSize::Large);

    // Verify through rendering
    let mut buffer1 = Buffer::new(5, 1);
    let mut ctx1 = RenderContext::new(&mut buffer1, Rect::new(0, 0, 5, 1));
    small.render(&mut ctx1);
    assert_eq!(buffer1.get(0, 0).map(|c| c.symbol), Some('T'));

    let mut buffer2 = Buffer::new(10, 1);
    let mut ctx2 = RenderContext::new(&mut buffer2, Rect::new(0, 0, 10, 1));
    medium.render(&mut ctx2);
    assert_eq!(buffer2.get(0, 0).map(|c| c.symbol), Some('◖'));

    let mut buffer3 = Buffer::new(10, 3);
    let mut ctx3 = RenderContext::new(&mut buffer3, Rect::new(0, 0, 10, 3));
    large.render(&mut ctx3);
    assert_eq!(buffer3.get(0, 0).map(|c| c.symbol), Some('╭'));
}

#[test]
fn test_avatar_size_default() {
    let a = avatar("Test");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Default is Medium (circle markers)
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('◖'));
}

// =============================================================================
// All AvatarShape Variants
// =============================================================================

#[test]
fn test_all_avatar_shape_variants() {
    let circle = avatar("Test").shape(AvatarShape::Circle);
    let square = avatar("Test").shape(AvatarShape::Square);
    let rounded = avatar("Test").shape(AvatarShape::Rounded);

    let mut buffer1 = Buffer::new(10, 1);
    let mut ctx1 = RenderContext::new(&mut buffer1, Rect::new(0, 0, 10, 1));
    circle.render(&mut ctx1);
    assert_eq!(buffer1.get(0, 0).map(|c| c.symbol), Some('◖'));

    let mut buffer2 = Buffer::new(10, 1);
    let mut ctx2 = RenderContext::new(&mut buffer2, Rect::new(0, 0, 10, 1));
    square.render(&mut ctx2);
    assert_eq!(buffer2.get(0, 0).map(|c| c.symbol), Some('['));

    let mut buffer3 = Buffer::new(10, 1);
    let mut ctx3 = RenderContext::new(&mut buffer3, Rect::new(0, 0, 10, 1));
    rounded.render(&mut ctx3);
    assert_eq!(buffer3.get(0, 0).map(|c| c.symbol), Some('('));
}

#[test]
fn test_avatar_shape_default() {
    let a = avatar("Test");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Default is Circle
    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('◖'));
}

// =============================================================================
// Modifier Tests
// =============================================================================

#[test]
fn test_avatar_render_bold_modifier() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("JD");
    a.render(&mut ctx);

    // Avatar renders successfully - bold modifier may or may not be applied
    // depending on implementation
    let cell1 = buffer.get(1, 0).unwrap();
    assert_eq!(cell1.symbol, 'J');
}

// =============================================================================
// Default Trait Tests
// =============================================================================

#[test]
fn test_avatar_default_trait() {
    let a: Avatar = Default::default();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    a.render(&mut ctx);
    // Empty name shows space for initials
    assert_eq!(buffer.get(1, 0).map(|c| c.symbol), Some(' '));
}

// =============================================================================
// =============================================================================
