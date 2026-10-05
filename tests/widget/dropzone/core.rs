//! Drop zone widget tests

use revue::event::drag::DragData;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{Draggable, RenderContext, StyledView, View};
use revue::widget::{DropZone, DropZoneStyle};

// =========================================================================
// Helpers
// =========================================================================

fn render(zone: &impl View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    zone.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

fn symbol(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

fn fg(buffer: &Buffer, x: u16, y: u16) -> Option<Color> {
    buffer.get(x, y).unwrap().fg
}

// =========================================================================
// Constructor tests
// =========================================================================

#[test]
fn test_drop_zone_new_shows_placeholder() {
    let zone = DropZone::new("Drop files here");
    let buffer = render(&zone, 30, 3);
    assert!(row(&buffer, 1).contains("Drop files here"));
}

#[test]
fn test_drop_zone_new_with_string_owned() {
    let zone = DropZone::new("Owned text".to_string());
    let buffer = render(&zone, 30, 3);
    assert!(row(&buffer, 1).contains("Owned text"));
}

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
// Styles
// =========================================================================

#[test]
fn test_drop_zone_style_solid() {
    let zone = DropZone::new("Test").style(DropZoneStyle::Solid);
    let buffer = render(&zone, 10, 3);
    assert_eq!(row(&buffer, 0), "┌────────┐");
    assert_eq!(symbol(&buffer, 0, 1), '│');
    assert_eq!(row(&buffer, 2), "└────────┘");
}

#[test]
fn test_drop_zone_style_dashed() {
    let zone = DropZone::new("Test").style(DropZoneStyle::Dashed);
    let buffer = render(&zone, 10, 3);
    assert_eq!(row(&buffer, 0), "┌╌╌╌╌╌╌╌╌┐");
    assert_eq!(symbol(&buffer, 0, 1), '╎');
    assert_eq!(symbol(&buffer, 9, 1), '╎');
}

#[test]
fn test_drop_zone_style_highlight() {
    let mut zone = DropZone::new("Test").style(DropZoneStyle::Highlight);
    let buffer = render(&zone, 10, 3);
    // No border at rest
    assert_eq!(row(&buffer, 0).trim(), "");
    assert_eq!(buffer.get(0, 0).unwrap().bg, None);

    // Filled while an acceptable drag is over it
    zone.set_hovered(true, true);
    let accept = render(&zone, 10, 3);
    assert!(accept.get(0, 0).unwrap().bg.is_some());

    // ... in a different color when the drag would be rejected
    zone.set_hovered(true, false);
    let reject = render(&zone, 10, 3);
    assert!(reject.get(0, 0).unwrap().bg.is_some());
    assert_ne!(accept.get(0, 0).unwrap().bg, reject.get(0, 0).unwrap().bg);
}

#[test]
fn test_drop_zone_style_minimal() {
    let mut zone = DropZone::new("Test").style(DropZoneStyle::Minimal);
    let buffer = render(&zone, 10, 3);
    assert_eq!(symbol(&buffer, 0, 0), '│');
    assert_eq!(symbol(&buffer, 9, 0), ' ');

    zone.set_hovered(true, true);
    assert_eq!(symbol(&render(&zone, 10, 3), 0, 0), '▶');

    zone.set_hovered(true, false);
    assert_eq!(symbol(&render(&zone, 10, 3), 0, 0), '✗');
}

#[test]
fn test_drop_zone_border_color() {
    let red = Color::rgb(255, 0, 0);
    let zone = DropZone::new("Test").border_color(red);
    let buffer = render(&zone, 10, 3);
    assert_eq!(fg(&buffer, 0, 0), Some(red));
    assert_eq!(fg(&buffer, 9, 2), Some(red));
}

#[test]
fn test_drop_zone_hover_color() {
    let render_states = |zone: &mut DropZone| {
        let mut out = Vec::new();
        for (hovered, accept) in [(false, false), (true, true), (true, false)] {
            zone.set_hovered(hovered, accept);
            let buffer = render(&*zone, 10, 3);
            out.push((0..10).map(|x| fg(&buffer, x, 0)).collect::<Vec<_>>());
        }
        out
    };
    let mut plain = DropZone::new("Test");
    let green = Color::rgb(0, 255, 0);
    let mut custom = DropZone::new("Test").hover_color(green);
    let plain_states = render_states(&mut plain);
    let custom_states = render_states(&mut custom);
    assert_ne!(plain_states, custom_states);
    // At rest and on a rejected drag nothing changes
    assert_eq!(plain_states[0], custom_states[0]);
    assert_eq!(plain_states[2], custom_states[2]);
    // An acceptable drag draws the whole border in the hover color
    assert!(custom_states[1].iter().all(|c| *c == Some(green)));

    // The minimal style's indicator follows it too
    let mut minimal = DropZone::new("Test")
        .style(DropZoneStyle::Minimal)
        .hover_color(green);
    minimal.set_hovered(true, true);
    assert_eq!(fg(&render(&minimal, 10, 3), 0, 0), Some(green));
}

#[test]
fn test_drop_zone_min_height() {
    let zone = DropZone::new("Test").min_height(10);
    assert_eq!(zone.get_min_height(), 10);
    // Default minimum is 3 rows
    assert_eq!(DropZone::new("Test").get_min_height(), 3);
}

#[test]
fn test_drop_zone_min_height_zero() {
    let zone = DropZone::new("Test").min_height(0);
    assert_eq!(zone.get_min_height(), 0);
    // The area height still applies
    let buffer = render(&zone, 10, 3);
    assert_eq!(symbol(&buffer, 0, 2), '└');
}

#[test]
fn test_drop_zone_builder_chain() {
    let border = Color::rgb(100, 100, 100);
    let zone = DropZone::new("Test")
        .accepts(&["file", "text"])
        .style(DropZoneStyle::Dashed)
        .border_color(border)
        .hover_color(Color::rgb(150, 150, 255))
        .min_height(5);
    assert_eq!(zone.accepted_types(), &["file", "text"]);
    assert_eq!(zone.get_min_height(), 5);
    let buffer = render(&zone, 10, 5);
    assert_eq!(symbol(&buffer, 1, 0), '╌');
    assert_eq!(fg(&buffer, 1, 0), Some(border));
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

// =========================================================================
// StyledView
// =========================================================================

#[test]
fn test_drop_zone_set_id() {
    let mut zone = DropZone::new("Test");
    zone.set_id("my-dropzone");
    assert_eq!(View::id(&zone), Some("my-dropzone"));
}

#[test]
fn test_drop_zone_add_class() {
    let mut zone = DropZone::new("Test");
    zone.add_class("dropzone-active");
    zone.add_class("dropzone-hover");
    assert!(zone.has_class("dropzone-active"));
    assert!(zone.has_class("dropzone-hover"));
}

#[test]
fn test_drop_zone_add_class_duplicate() {
    let mut zone = DropZone::new("Test");
    zone.add_class("active");
    zone.add_class("active");
    assert_eq!(View::classes(&zone), ["active"]);
}

#[test]
fn test_drop_zone_remove_class() {
    let mut zone = DropZone::new("Test");
    zone.add_class("active");
    zone.remove_class("active");
    assert!(!zone.has_class("active"));
}

#[test]
fn test_drop_zone_toggle_class() {
    let mut zone = DropZone::new("Test");
    zone.toggle_class("active");
    assert!(zone.has_class("active"));
    zone.toggle_class("active");
    assert!(!zone.has_class("active"));
}

#[test]
fn test_drop_zone_has_class_false() {
    let zone = DropZone::new("Test");
    assert!(!zone.has_class("active"));
}

// =========================================================================
// WidgetState builders
// =========================================================================

#[test]
fn test_drop_zone_focused() {
    assert!(DropZone::new("Test").focused(true).is_focused());
    assert!(!DropZone::new("Test").focused(false).is_focused());
}

#[test]
fn test_drop_zone_disabled() {
    assert!(DropZone::new("Test").disabled(true).is_disabled());
    assert!(!DropZone::new("Test").disabled(false).is_disabled());
}

#[test]
fn test_drop_zone_fg_color_does_not_change_state() {
    let zone = DropZone::new("Test").fg(Color::rgb(255, 0, 0));
    assert!(!zone.is_focused());
    assert!(!zone.is_disabled());
}

#[test]
fn test_drop_zone_set_focused() {
    let mut zone = DropZone::new("Test");
    zone.set_focused(true);
    assert!(zone.is_focused());
    zone.set_focused(false);
    assert!(!zone.is_focused());
}

// =========================================================================
// Integration
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
    assert_eq!(zone.get_min_height(), 10);
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
    assert_eq!(View::id(&zone), Some("upload-zone"));
    assert_eq!(View::classes(&zone), ["primary", "large"]);
}
