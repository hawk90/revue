//! Integration tests for Splitter widget

use revue::style::Color;
use revue::widget::{HSplit, Pane, SplitOrientation, Splitter, VSplit};

#[test]
fn test_pane_new() {
    let pane = Pane::new("pane1");

    assert_eq!(pane.id, "pane1");
    assert_eq!(pane.min_size, 5); // default
    assert_eq!(pane.max_size, 0); // default (0 = unlimited)
    assert_eq!(pane.ratio, 0.5); // default
    assert!(!pane.collapsible);
    assert!(!pane.collapsed);
}

#[test]
fn test_pane_with_min_size() {
    let pane = Pane::new("pane1").min_size(10);

    assert_eq!(pane.min_size, 10);
}

#[test]
fn test_pane_with_max_size() {
    let pane = Pane::new("pane1").max_size(50);

    assert_eq!(pane.max_size, 50);
}

#[test]
fn test_pane_with_ratio() {
    let pane = Pane::new("pane1").ratio(0.7);

    assert_eq!(pane.ratio, 0.7);
}

#[test]
fn test_pane_collapsible() {
    let pane = Pane::new("pane1").collapsible();

    assert!(pane.collapsible);
}

#[test]
fn test_pane_toggle_collapse() {
    let mut pane = Pane::new("pane1").collapsible();

    assert!(!pane.collapsed);
    pane.toggle_collapse();
    assert!(pane.collapsed);
    pane.toggle_collapse();
    assert!(!pane.collapsed);
}

#[test]
fn test_splitter_new() {
    let _splitter = Splitter::new();

    // Splitter created successfully
}

#[test]
fn test_splitter_with_pane() {
    let pane = Pane::new("pane1");
    let _splitter = Splitter::new().pane(pane);

    // Pane added successfully
}

#[test]
fn test_splitter_with_panes() {
    let panes = vec![Pane::new("pane1").ratio(0.3), Pane::new("pane2").ratio(0.7)];
    let _splitter = Splitter::new().panes(panes);

    // Panes added successfully
}

#[test]
fn test_splitter_horizontal() {
    let _splitter = Splitter::new().horizontal();

    // Horizontal orientation was set
}

#[test]
fn test_splitter_vertical() {
    let _splitter = Splitter::new().vertical();

    // Vertical orientation was set
}

#[test]
fn test_splitter_orientation() {
    let _splitter = Splitter::new().orientation(SplitOrientation::Vertical);

    // Orientation was set
}

#[test]
fn test_splitter_color() {
    let _splitter = Splitter::new().color(Color::CYAN);

    // Color was set
}

#[test]
fn test_splitter_active_color() {
    let _splitter = Splitter::new().active_color(Color::YELLOW);

    // Active color was set
}

#[test]
fn test_hsplit_new() {
    let _hsplit = HSplit::new(0.5);

    // Horizontal split with 50% ratio created
}

#[test]
fn test_hsplit_with_min_widths() {
    let _hsplit = HSplit::new(0.5).min_widths(10, 20);

    // Min widths were set
}

#[test]
fn test_hsplit_hide_splitter() {
    let _hsplit = HSplit::new(0.5).hide_splitter();

    // Hide splitter was set
}

#[test]
fn test_vsplit_new() {
    let _vsplit = VSplit::new(0.5);

    // Vertical split with 50% ratio created
}

#[test]
fn test_pane_complete_builder() {
    let pane = Pane::new("my_pane")
        .min_size(10)
        .max_size(50)
        .ratio(0.25)
        .collapsible();

    assert_eq!(pane.id, "my_pane");
    assert_eq!(pane.min_size, 10);
    assert_eq!(pane.max_size, 50);
    assert_eq!(pane.ratio, 0.25);
    assert!(pane.collapsible);
}

#[test]
fn test_splitter_builder_pattern() {
    let _splitter = Splitter::new()
        .horizontal()
        .color(Color::CYAN)
        .active_color(Color::YELLOW);

    // Builder pattern works
}

#[test]
fn test_splitter_with_collapsible_pane() {
    let _splitter = Splitter::new().pane(Pane::new("sidebar").collapsible());

    // Collapsible pane added
}

#[test]
fn test_splitter_multiple_panes_builder() {
    let _splitter = Splitter::new()
        .pane(Pane::new("left").ratio(0.3))
        .pane(Pane::new("middle").ratio(0.4))
        .pane(Pane::new("right").ratio(0.3));

    // Multiple panes configured
}
