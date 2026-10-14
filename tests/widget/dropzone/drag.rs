//! DropZone's drag-and-drop side: accepted types, hover state, dropping
//! and the drop target

use revue::event::drag::DragData;
use revue::layout::Rect;
use revue::widget::traits::Draggable;
use revue::widget::DropZone;

use super::{fg, render, row};

// =========================================================================
// Constructor
// =========================================================================

#[test]
fn test_drop_zone_ids_are_unique_and_increasing() {
    let zone1 = DropZone::new("Zone 1");
    let zone2 = DropZone::new("Zone 2");
    let zone3 = DropZone::new("Zone 3");
    assert!(zone1.id() > 0);
    assert!(zone1.id() < zone2.id());
    assert!(zone2.id() < zone3.id());
}
// =========================================================================
// Accepted types
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
fn test_drop_zone_accepts_nothing_by_default() {
    let zone = DropZone::new("Test");
    assert!(zone.accepted_types().is_empty());
    // An empty list accepts any type
    assert!(zone.can_accept(&DragData::text("x")));
    assert!(zone.can_accept(&DragData::file("/tmp/x")));
}
#[test]
fn test_drop_zone_accepts_all() {
    let zone = DropZone::new("Test").accepts(&["file"]).accepts_all();
    assert!(zone.accepted_types().is_empty());
    assert!(zone.can_accept(&DragData::text("x")));
}
#[test]
fn test_drop_zone_accepts_filters_types() {
    let zone = DropZone::new("Test").accepts(&["file"]);
    assert!(zone.can_accept(&DragData::file("/tmp/x")));
    assert!(!zone.can_accept(&DragData::text("x")));
}
// =========================================================================
// Hover state and feedback
// =========================================================================

#[test]
fn test_drop_zone_set_hovered_can_accept() {
    let mut zone = DropZone::new("Test");
    zone.set_hovered(true, true);
    assert!(zone.is_hovered());
    assert!(zone.can_accept_current());
    let buffer = render(&zone, 30, 3);
    assert!(row(&buffer, 1).contains("Drop here!"));
}
#[test]
fn test_drop_zone_set_hovered_cannot_accept() {
    let mut zone = DropZone::new("Test");
    zone.set_hovered(true, false);
    assert!(zone.is_hovered());
    assert!(!zone.can_accept_current());
    let buffer = render(&zone, 30, 3);
    assert!(row(&buffer, 1).contains("Cannot drop here"));
}
#[test]
fn test_drop_zone_set_hovered_false() {
    let mut zone = DropZone::new("Test");
    zone.set_hovered(true, true);
    zone.set_hovered(false, false);
    assert!(!zone.is_hovered());
    let buffer = render(&zone, 30, 3);
    assert!(row(&buffer, 1).contains("Test"));
}
#[test]
fn test_drop_zone_hover_border_colors() {
    let mut zone = DropZone::new("Test");
    let rest = fg(&render(&zone, 10, 3), 0, 0);
    zone.set_hovered(true, true);
    let accept = fg(&render(&zone, 10, 3), 0, 0);
    zone.set_hovered(true, false);
    let reject = fg(&render(&zone, 10, 3), 0, 0);
    assert_ne!(rest, accept);
    assert_ne!(rest, reject);
    assert_ne!(accept, reject);
}
#[test]
fn test_drop_zone_on_drag_enter_accepted_type() {
    let mut zone = DropZone::new("Test").accepts(&["text"]);
    zone.on_drag_enter(&DragData::text("test"));
    assert!(zone.is_hovered());
    assert!(zone.can_accept_current());
}
#[test]
fn test_drop_zone_on_drag_enter_rejected_type() {
    let mut zone = DropZone::new("Test").accepts(&["file"]);
    zone.on_drag_enter(&DragData::text("test"));
    assert!(zone.is_hovered());
    assert!(!zone.can_accept_current());
}
#[test]
fn test_drop_zone_drag_enter_then_leave() {
    let mut zone = DropZone::new("Test").accepts(&["text"]);
    zone.on_drag_enter(&DragData::text("test"));
    zone.on_drag_leave();
    assert!(!zone.is_hovered());
    assert!(!zone.can_accept_current());
}
// =========================================================================
// Dropping
// =========================================================================

#[test]
fn test_drop_zone_on_drop_with_handler() {
    let mut zone = DropZone::new("Test").on_drop(|_data| true);
    assert!(Draggable::on_drop(&mut zone, DragData::text("test")));
}
#[test]
fn test_drop_zone_on_drop_returns_false() {
    let mut zone = DropZone::new("Test").on_drop(|_data| false);
    assert!(!Draggable::on_drop(&mut zone, DragData::text("test")));
}
#[test]
fn test_drop_zone_on_drop_without_handler_returns_false() {
    let mut zone = DropZone::new("Test");
    assert!(!Draggable::on_drop(&mut zone, DragData::text("test")));
}
#[test]
fn test_drop_zone_on_drop_resets_hover_state() {
    let mut zone = DropZone::new("Test");
    zone.set_hovered(true, true);
    Draggable::on_drop(&mut zone, DragData::text("test"));
    assert!(!zone.is_hovered());
    assert!(!zone.can_accept_current());
}
#[test]
fn test_drop_zone_with_drop_handler_and_drag_operations() {
    let mut received = None;
    let mut zone = DropZone::new("Test").accepts(&["text"]).on_drop(|data| {
        received = data.as_text().map(str::to_string);
        true
    });

    let data = DragData::text("Hello, world!");
    zone.on_drag_enter(&data);
    assert!(Draggable::on_drop(&mut zone, data));
    drop(zone);
    assert_eq!(received.as_deref(), Some("Hello, world!"));
}
// =========================================================================
// Drop target registration
// =========================================================================

#[test]
fn test_drop_zone_as_target() {
    let zone = DropZone::new("Test");
    let bounds = Rect::new(0, 0, 10, 5);
    let target = zone.as_target(bounds);
    assert_eq!(target.id, zone.id());
    assert_eq!(target.bounds, bounds);
    assert!(target.accepts.is_empty());
}
#[test]
fn test_drop_zone_as_target_with_accepts() {
    let zone = DropZone::new("Test").accepts(&["file", "text"]);
    let target = zone.as_target(Rect::new(0, 0, 10, 5));
    assert_eq!(target.accepts, ["file", "text"]);
    assert!(target.can_accept(&DragData::text("x")));
}
#[test]
fn test_drop_zone_can_drop_always_true() {
    let zone = DropZone::new("Test");
    assert!(zone.can_drop());
}
#[test]
fn test_drop_zone_drop_bounds() {
    let zone = DropZone::new("Test");
    let bounds = Rect::new(5, 10, 20, 15);
    assert_eq!(zone.drop_bounds(bounds), bounds);
}
