//! DropZone builders, widget-state setters and getters, and CSS classes

use revue::style::Color;
use revue::widget::traits::{Draggable, StyledView, View};
use revue::widget::{DropZone, DropZoneStyle};

use super::{fg, render, row, symbol};

// =========================================================================
// Constructor
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
// =========================================================================
// Builders
// =========================================================================

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

#[test]
fn on_drop_keeps_every_builder_setting() {
    let build = || {
        DropZone::new("Keep")
            .accepts(&["file"])
            .style(DropZoneStyle::Dashed)
            .border_color(Color::YELLOW)
            .hover_color(Color::CYAN)
            .min_height(0)
            .focused(true)
            .fg(Color::MAGENTA)
            .bg(Color::BLUE)
    };
    let mut plain = build();
    let mut handled = build().on_drop(|_| true);
    assert_eq!(handled.id(), handled.as_target(Default::default()).id);
    assert_eq!(handled.accepted_types(), &["file"]);
    for (hovered, accept) in [(false, false), (true, true), (true, false)] {
        plain.set_hovered(hovered, accept);
        handled.set_hovered(hovered, accept);
        // Two rows show min_height(0): the default 3 would leave the frame
        // open at the bottom.
        for (width, height) in [(12, 2), (12, 4)] {
            let (a, b) = (
                render(&plain, width, height),
                render(&handled, width, height),
            );
            for y in 0..height {
                for x in 0..width {
                    assert_eq!(a.get(x, y), b.get(x, y), "({x}, {y}) {hovered} {accept}");
                }
            }
        }
    }
}
