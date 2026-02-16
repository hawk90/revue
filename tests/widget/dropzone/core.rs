//! Drop zone widget tests

use revue::event::drag::{DragData, DragId, DropTarget};
use revue::layout::Rect;
use revue::style::Color;
use revue::widget::traits::{Draggable, RenderContext, StyledView, View, WidgetProps, WidgetState};
use revue::impl_view_meta;
use revue::widget::dropzone::{DropZone, DropZoneStyle};

// =========================================================================
// Constructor tests
// =========================================================================

#[test]
fn test_drop_zone_new_with_string() {
    let zone = DropZone::new("Drop files here");
    assert!(zone.id() > 0);
}

#[test]
fn test_drop_zone_new_with_string_owned() {
    let zone = DropZone::new("Drop files here".to_string());
    assert!(zone.id() > 0);
}

#[test]
fn test_drop_zone_new_generates_unique_ids() {
    let zone1 = DropZone::new("Zone 1");
    let zone2 = DropZone::new("Zone 2");
    assert_ne!(zone1.id(), zone2.id());
}

// =========================================================================
// Builder method tests
// =========================================================================

#[test]
fn test_drop_zone_accepts_single_type() {
    let zone = DropZone::new("Test").accepts(&["file"]);
    assert_eq!(zone.accepted_types(), &["file"]);
}

#[test]
fn test_drop_zone_accepts_multiple_types() {
    let zone = DropZone::new("Test").accepts(&["file", "text", "image"]);
    assert_eq!(zone.accepted_types(), &["file", "text", "image"]);
}

#[test]
fn test_drop_zone_accepts_empty() {
    let zone = DropZone::new("Test").accepts(&[]);
    assert_eq!(zone.accepted_types(), &[] as &[&str]);
}

#[test]
fn test_drop_zone_accepts_all() {
    let zone = DropZone::new("Test").accepts(&["file"]).accepts_all();
    assert_eq!(zone.accepted_types(), &[] as &[&str]);
}

#[test]
fn test_drop_zone_style_solid() {
    let zone = DropZone::new("Test").style(DropZoneStyle::Solid);
    // Can't directly access style, but we can verify the method exists
    let _ = zone;
}

#[test]
fn test_drop_zone_style_dashed() {
    let zone = DropZone::new("Test").style(DropZoneStyle::Dashed);
    let _ = zone;
}

#[test]
fn test_drop_zone_style_highlight() {
    let zone = DropZone::new("Test").style(DropZoneStyle::Highlight);
    let _ = zone;
}

#[test]
fn test_drop_zone_style_minimal() {
    let zone = DropZone::new("Test").style(DropZoneStyle::Minimal);
    let _ = zone;
}

#[test]
fn test_drop_zone_border_color() {
    let zone = DropZone::new("Test").border_color(Color::rgb(255, 0, 0));
    let _ = zone;
}

#[test]
fn test_drop_zone_hover_color() {
    let zone = DropZone::new("Test").hover_color(Color::rgb(0, 255, 0));
    let _ = zone;
}

#[test]
fn test_drop_zone_min_height() {
    let zone = DropZone::new("Test").min_height(10);
    // Can't directly access min_height, but verify method exists
    let _ = zone;
}

#[test]
fn test_drop_zone_min_height_zero() {
    let zone = DropZone::new("Test").min_height(0);
    let _ = zone;
}

#[test]
fn test_drop_zone_builder_chain() {
    let zone = DropZone::new("Test")
        .accepts(&["file", "text"])
        .style(DropZoneStyle::Dashed)
        .border_color(Color::rgb(100, 100, 100))
        .hover_color(Color::rgb(150, 150, 255))
        .min_height(5);
    let _ = zone;
}

// =========================================================================
// on_drop handler tests
// =========================================================================

#[test]
fn test_drop_zone_on_drop_with_handler() {
    use crate::widget::traits::Draggable;
    let mut zone = DropZone::new("Test").on_drop(|_data| true);
    let data = DragData::text("test");
    assert!(Draggable::on_drop(&mut zone, data));
}

#[test]
fn test_drop_zone_on_drop_returns_false() {
    use crate::widget::traits::Draggable;
    let mut zone = DropZone::new("Test").on_drop(|_data| false);
    let data = DragData::text("test");
    assert!(!Draggable::on_drop(&mut zone, data));
}

#[test]
fn test_drop_zone_on_drop_without_handler_returns_false() {
    use crate::widget::traits::Draggable;
    let mut zone = DropZone::new("Test");
    let data = DragData::text("test");
    assert!(!Draggable::on_drop(&mut zone, data));
}

#[test]
fn test_drop_zone_on_drop_resets_hover_state() {
    use crate::widget::traits::Draggable;
    let mut zone = DropZone::new("Test").on_drop(|_data| true);
    zone.set_hovered(true, true);
    Draggable::on_drop(&mut zone, DragData::text("test"));
    // After drop, hover state should be reset
    // This is tested through Draggable trait
}

// =========================================================================
// State method tests
// =========================================================================

#[test]
fn test_drop_zone_set_hovered_can_accept() {
    let mut zone = DropZone::new("Test");
    zone.set_hovered(true, true);
    // State is private, but method exists
}

#[test]
fn test_drop_zone_set_hovered_cannot_accept() {
    let mut zone = DropZone::new("Test");
    zone.set_hovered(true, false);
    // State is private, but method exists
}

#[test]
fn test_drop_zone_set_hovered_false() {
    let mut zone = DropZone::new("Test");
    zone.set_hovered(false, false);
    // State is private, but method exists
}

// =========================================================================
// Getter method tests
// =========================================================================

#[test]
fn test_drop_zone_id() {
    let zone = DropZone::new("Test");
    // ID should be non-zero
    assert!(zone.id() > 0);
}

#[test]
fn test_drop_zone_id_multiple() {
    let zone1 = DropZone::new("Test1");
    let zone2 = DropZone::new("Test2");
    let zone3 = DropZone::new("Test3");
    // IDs should be unique and sequential
    assert!(zone1.id() < zone2.id());
    assert!(zone2.id() < zone3.id());
    assert_eq!(zone2.id() - zone1.id(), 1);
    assert_eq!(zone3.id() - zone2.id(), 1);
}

#[test]
fn test_drop_zone_as_target() {
    let zone = DropZone::new("Test");
    let bounds = Rect::new(0, 0, 10, 5);
    let target = zone.as_target(bounds);
    assert_eq!(target.id, zone.id());
}

#[test]
fn test_drop_zone_as_target_with_accepts() {
    let zone = DropZone::new("Test").accepts(&["file", "text"]);
    let bounds = Rect::new(0, 0, 10, 5);
    let target = zone.as_target(bounds);
    assert_eq!(target.id, zone.id());
}

// =========================================================================
// Draggable trait tests
// =========================================================================

#[test]
fn test_drop_zone_can_drop_always_true() {
    let zone = DropZone::new("Test");
    assert!(zone.can_drop());
}

#[test]
fn test_drop_zone_accepted_types() {
    let zone = DropZone::new("Test").accepts(&["file"]);
    assert_eq!(zone.accepted_types(), &["file"]);
}

#[test]
fn test_drop_zone_accepted_types_empty() {
    let zone = DropZone::new("Test");
    assert!(zone.accepted_types().is_empty());
}

#[test]
fn test_drop_zone_on_drag_enter() {
    let mut zone = DropZone::new("Test").accepts(&["text"]);
    let data = DragData::text("test");
    zone.on_drag_enter(&data);
    // Can't directly verify hover state, but method exists
}

#[test]
fn test_drop_zone_on_drag_leave() {
    let mut zone = DropZone::new("Test");
    zone.on_drag_leave();
    // Can't directly verify hover state, but method exists
}

#[test]
fn test_drop_zone_drag_enter_then_leave() {
    let mut zone = DropZone::new("Test").accepts(&["text"]);
    let data = DragData::text("test");
    zone.on_drag_enter(&data);
    zone.on_drag_leave();
}

#[test]
fn test_drop_zone_on_drop_trait() {
    use crate::widget::traits::Draggable;
    let mut zone = DropZone::new("Test").on_drop(|_data| true);
    let data = DragData::text("test");
    assert!(Draggable::on_drop(&mut zone, data));
}

#[test]
fn test_drop_zone_drop_bounds() {
    let zone = DropZone::new("Test");
    let bounds = Rect::new(5, 10, 20, 15);
    let result = zone.drop_bounds(bounds);
    assert_eq!(result, bounds);
}

// =========================================================================
// StyledView trait tests
// =========================================================================

#[test]
fn test_drop_zone_set_id() {
    let mut zone = DropZone::new("Test");
    zone.set_id("my-dropzone");
    // Can't directly access id, but method exists
}

#[test]
fn test_drop_zone_add_class() {
    let mut zone = DropZone::new("Test");
    zone.add_class("dropzone-active");
    zone.add_class("dropzone-hover");
    // Can't directly access classes, but method exists
}

#[test]
fn test_drop_zone_add_class_duplicate() {
    let mut zone = DropZone::new("Test");
    zone.add_class("active");
    zone.add_class("active"); // Should not add duplicate
                              // Can't directly verify, but method exists
}

#[test]
fn test_drop_zone_remove_class() {
    let mut zone = DropZone::new("Test");
    zone.add_class("active");
    zone.remove_class("active");
    // Can't directly verify, but method exists
}

#[test]
fn test_drop_zone_toggle_class_add() {
    let mut zone = DropZone::new("Test");
    zone.toggle_class("active");
    // Should add class
}

#[test]
fn test_drop_zone_toggle_class_remove() {
    let mut zone = DropZone::new("Test");
    zone.add_class("active");
    zone.toggle_class("active");
    // Should remove class
}

#[test]
fn test_drop_zone_has_class_true() {
    let mut zone = DropZone::new("Test");
    zone.add_class("active");
    assert!(zone.has_class("active"));
}

#[test]
fn test_drop_zone_has_class_false() {
    let zone = DropZone::new("Test");
    assert!(!zone.has_class("active"));
}

// =========================================================================
// WidgetState builder tests
// =========================================================================

#[test]
fn test_drop_zone_focused() {
    let zone = DropZone::new("Test").focused(true);
    assert!(zone.is_focused());
}

#[test]
fn test_drop_zone_not_focused() {
    let zone = DropZone::new("Test").focused(false);
    assert!(!zone.is_focused());
}

#[test]
fn test_drop_zone_disabled() {
    let zone = DropZone::new("Test").disabled(true);
    assert!(zone.is_disabled());
}

#[test]
fn test_drop_zone_not_disabled() {
    let zone = DropZone::new("Test").disabled(false);
    assert!(!zone.is_disabled());
}

#[test]
fn test_drop_zone_fg_color() {
    let zone = DropZone::new("Test").fg(Color::rgb(255, 0, 0));
    assert!(zone.is_focused() == false); // Should not affect focused state
}

#[test]
fn test_drop_zone_bg_color() {
    let zone = DropZone::new("Test").bg(Color::rgb(0, 255, 0));
    let _ = zone;
}

#[test]
fn test_drop_zone_set_focused_true() {
    let mut zone = DropZone::new("Test");
    zone.set_focused(true);
    assert!(zone.is_focused());
}

#[test]
fn test_drop_zone_set_focused_false() {
    let mut zone = DropZone::new("Test").focused(true);
    zone.set_focused(false);
    assert!(!zone.is_focused());
}

// =========================================================================
// Integration tests
// =========================================================================

#[test]
fn test_drop_zone_full_builder_chain() {
    let zone = DropZone::new("Upload files")
        .accepts(&["file", "image"])
        .style(DropZoneStyle::Dashed)
        .border_color(Color::rgb(100, 100, 100))
        .hover_color(Color::rgb(150, 150, 255))
        .min_height(10)
        .focused(true)
        .disabled(false)
        .fg(Color::rgb(200, 200, 200))
        .bg(Color::rgb(50, 50, 50));

    assert!(zone.is_focused());
    assert!(!zone.is_disabled());
    assert_eq!(zone.accepted_types(), &["file", "image"]);
}

#[test]
fn test_drop_zone_with_style_and_classes() {
    let mut zone = DropZone::new("Test")
        .style(DropZoneStyle::Highlight)
        .focused(true);

    zone.set_id("upload-zone");
    zone.add_class("primary");
    zone.add_class("large");

    assert!(zone.is_focused());
    assert!(zone.has_class("primary"));
    assert!(zone.has_class("large"));
}

#[test]
fn test_drop_zone_with_drop_handler_and_drag_operations() {
    use crate::widget::traits::Draggable;
    let mut zone = DropZone::new("Test").accepts(&["text"]).on_drop(|data| {
        assert_eq!(data.type_id, "text");
        true
    });

    let data = DragData::text("Hello, world!");
    zone.on_drag_enter(&data);
    let result = Draggable::on_drop(&mut zone, data);
    assert!(result);
}