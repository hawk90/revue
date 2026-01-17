//! Responsive tests (from src/layout/responsive.rs)

#![allow(unused_imports)]

use revue::layout::*;

#[test]
fn test_breakpoint_new() {
    let bp = Breakpoint::new("custom", 100);
    assert_eq!(bp.name, "custom");
    assert_eq!(bp.min_width, 100);
}

#[test]
fn test_breakpoints_current() {
    let bp = Breakpoints::terminal();

    assert_eq!(bp.current(30).name, "xs");
    assert_eq!(bp.current(50).name, "sm");
    assert_eq!(bp.current(80).name, "md");
    assert_eq!(bp.current(120).name, "lg");
    assert_eq!(bp.current(200).name, "xl");
}

#[test]
fn test_breakpoints_at_least() {
    let bp = Breakpoints::terminal();

    assert!(bp.at_least(80, "md"));
    assert!(bp.at_least(100, "md"));
    assert!(!bp.at_least(60, "md"));
}

#[test]
fn test_breakpoints_below() {
    let bp = Breakpoints::terminal();

    assert!(bp.below(60, "md"));
    assert!(!bp.below(80, "md"));
    assert!(!bp.below(100, "md"));
}

#[test]
fn test_responsive_value() {
    let bp = Breakpoints::terminal();
    let value = ResponsiveValue::new(1).at("sm", 2).at("md", 3).at("lg", 4);

    assert_eq!(value.resolve(&bp, 30), 1); // xs
    assert_eq!(value.resolve(&bp, 50), 2); // sm
    assert_eq!(value.resolve(&bp, 80), 3); // md
    assert_eq!(value.resolve(&bp, 120), 4); // lg
}

#[test]
fn test_responsive_value_cascade() {
    let bp = Breakpoints::terminal();
    // Only define value for "md", should cascade up
    let value = ResponsiveValue::new(1).at("md", 5);

    assert_eq!(value.resolve(&bp, 30), 1); // xs - uses default
    assert_eq!(value.resolve(&bp, 80), 5); // md - uses md value
    assert_eq!(value.resolve(&bp, 120), 5); // lg - cascades md value
}

#[test]
fn test_responsive_layout() {
    let layout = ResponsiveLayout::new(100, 30);

    assert_eq!(layout.breakpoint_name(), "md");
    assert!(layout.at_least("sm"));
    assert!(layout.at_least("md"));
    assert!(!layout.at_least("lg"));
    assert!(layout.below("lg"));
}

#[test]
fn test_responsive_layout_orientation() {
    let portrait = ResponsiveLayout::new(80, 100);
    assert!(portrait.is_portrait());
    assert!(!portrait.is_landscape());

    let landscape = ResponsiveLayout::new(120, 40);
    assert!(landscape.is_landscape());
    assert!(!landscape.is_portrait());
}

#[test]
fn test_media_query_basic() {
    let layout = ResponsiveLayout::new(100, 30);

    assert!(MediaQuery::MinWidth(80).matches(&layout));
    assert!(!MediaQuery::MinWidth(120).matches(&layout));
    assert!(MediaQuery::MaxWidth(120).matches(&layout));
    assert!(!MediaQuery::MaxWidth(80).matches(&layout));
}

#[test]
fn test_media_query_range() {
    let layout = ResponsiveLayout::new(100, 30);

    assert!(MediaQuery::WidthRange(80, 120).matches(&layout));
    assert!(!MediaQuery::WidthRange(120, 160).matches(&layout));
}

#[test]
fn test_media_query_combined() {
    let layout = ResponsiveLayout::new(100, 30);

    let query = MediaQuery::MinWidth(80).and(MediaQuery::MaxWidth(120));
    assert!(query.matches(&layout));

    let query = MediaQuery::MinWidth(120).or(MediaQuery::MaxWidth(80));
    assert!(!query.matches(&layout));

    let query = MediaQuery::MinWidth(120).not();
    assert!(query.matches(&layout));
}

#[test]
fn test_media_query_breakpoint() {
    let layout = ResponsiveLayout::new(100, 30);

    assert!(MediaQuery::Breakpoint("md").matches(&layout));
    assert!(!MediaQuery::Breakpoint("lg").matches(&layout));
}

#[test]
fn test_responsive_helpers() {
    let small = ResponsiveLayout::new(30, 24);
    let medium = ResponsiveLayout::new(100, 30);
    let large = ResponsiveLayout::new(150, 40);

    // Columns - use the re-exported module
    assert_eq!(responsive_value::columns(&small), 1);
    assert_eq!(responsive_value::columns(&medium), 3);
    assert_eq!(responsive_value::columns(&large), 4);

    // Sidebar
    assert!(!responsive_value::show_sidebar(&small));
    assert!(responsive_value::show_sidebar(&medium));

    // Compact mode
    assert!(responsive_value::compact_mode(&small));
    assert!(!responsive_value::compact_mode(&medium));
}

#[test]
fn test_custom_breakpoints() {
    let bp = Breakpoints::new()
        .add(Breakpoint::new("tiny", 0))
        .add(Breakpoint::new("normal", 60))
        .add(Breakpoint::new("wide", 100));

    assert_eq!(bp.current(50).name, "tiny");
    assert_eq!(bp.current(60).name, "normal");
    assert_eq!(bp.current(100).name, "wide");
}

#[test]
fn test_resize() {
    let mut layout = ResponsiveLayout::new(60, 24);
    assert_eq!(layout.breakpoint_name(), "sm");

    layout.resize(100, 30);
    assert_eq!(layout.breakpoint_name(), "md");
}
