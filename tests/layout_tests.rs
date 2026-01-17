//! Integration tests for the layout module
//!
//! Tests migrated from src/layout/*.rs inline test modules.
//! Only tests using public API are included here.
//! Tests accessing private internals remain inline in source files.

use revue::layout::*;

// ============================================================================
// Rect Tests (from src/layout/mod.rs)
// ============================================================================

#[test]
fn test_rect_new() {
    let rect = Rect::new(10, 20, 30, 40);
    assert_eq!(rect.x, 10);
    assert_eq!(rect.y, 20);
    assert_eq!(rect.width, 30);
    assert_eq!(rect.height, 40);
}

#[test]
fn test_rect_contains() {
    let rect = Rect::new(10, 10, 20, 20);

    assert!(rect.contains(10, 10)); // Top-left
    assert!(rect.contains(15, 15)); // Center
    assert!(rect.contains(29, 29)); // Just inside
    assert!(!rect.contains(30, 30)); // Just outside
    assert!(!rect.contains(5, 15)); // Left of rect
}

#[test]
fn test_rect_edges() {
    let rect = Rect::new(10, 20, 30, 40);
    assert_eq!(rect.right(), 40);
    assert_eq!(rect.bottom(), 60);
}

#[test]
fn test_rect_intersects() {
    let r1 = Rect::new(0, 0, 20, 20);
    let r2 = Rect::new(10, 10, 20, 20);
    let r3 = Rect::new(100, 100, 10, 10);

    assert!(r1.intersects(&r2));
    assert!(r2.intersects(&r1));
    assert!(!r1.intersects(&r3));
}

#[test]
fn test_rect_intersection() {
    let r1 = Rect::new(0, 0, 20, 20);
    let r2 = Rect::new(10, 10, 20, 20);

    let intersection = r1.intersection(&r2).unwrap();
    assert_eq!(intersection, Rect::new(10, 10, 10, 10));

    let r3 = Rect::new(100, 100, 10, 10);
    assert!(r1.intersection(&r3).is_none());
}

#[test]
fn test_rect_union() {
    let r1 = Rect::new(0, 0, 20, 20);
    let r2 = Rect::new(10, 10, 30, 30);

    let union = r1.union(&r2);
    assert_eq!(union, Rect::new(0, 0, 40, 40));
}

#[test]
fn test_merge_rects_empty() {
    let rects: Vec<Rect> = vec![];
    let merged = merge_rects(&rects);
    assert!(merged.is_empty());
}

#[test]
fn test_merge_rects_single() {
    let rects = vec![Rect::new(0, 0, 10, 10)];
    let merged = merge_rects(&rects);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0], Rect::new(0, 0, 10, 10));
}

#[test]
fn test_merge_rects_overlapping() {
    let rects = vec![Rect::new(0, 0, 20, 20), Rect::new(10, 10, 20, 20)];
    let merged = merge_rects(&rects);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0], Rect::new(0, 0, 30, 30));
}

#[test]
fn test_merge_rects_non_overlapping() {
    let rects = vec![Rect::new(0, 0, 10, 10), Rect::new(50, 50, 10, 10)];
    let merged = merge_rects(&rects);
    assert_eq!(merged.len(), 2);
    assert!(merged.contains(&Rect::new(0, 0, 10, 10)));
    assert!(merged.contains(&Rect::new(50, 50, 10, 10)));
}

#[test]
fn test_merge_rects_multiple_overlapping() {
    let rects = vec![
        Rect::new(0, 0, 10, 10),
        Rect::new(5, 5, 10, 10),
        Rect::new(10, 10, 10, 10),
    ];
    let merged = merge_rects(&rects);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0], Rect::new(0, 0, 20, 20));
}

// ============================================================================
// Responsive Tests (from src/layout/responsive.rs)
// ============================================================================

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

// ============================================================================
// LayoutEngine Public API Tests
// ============================================================================

use revue::dom::DomId;
use revue::style::{Display, FlexDirection, Size, Style};

#[test]
fn test_layout_engine_new() {
    let engine = LayoutEngine::new();
    // Can only test via public API - try_layout returns None for missing node
    assert!(engine.try_layout(DomId::new(1)).is_none());
}

#[test]
fn test_create_node() {
    let mut engine = LayoutEngine::new();
    let style = Style::default();
    let dom_id = DomId::new(1);

    engine.create_node(dom_id, &style).unwrap();
    // Verify via layout - node exists if compute works
    engine.compute(dom_id, 100, 100).unwrap();
    assert!(engine.layout(dom_id).is_ok());
}

#[test]
fn test_create_multiple_nodes() {
    let mut engine = LayoutEngine::new();
    let style = Style::default();

    let id1 = DomId::new(1);
    let id2 = DomId::new(2);
    let id3 = DomId::new(3);

    engine.create_node(id1, &style).unwrap();
    engine.create_node(id2, &style).unwrap();
    engine.create_node(id3, &style).unwrap();

    // Verify all nodes exist via try_layout after compute
    engine.compute(id1, 100, 100).unwrap();
    engine.compute(id2, 100, 100).unwrap();
    engine.compute(id3, 100, 100).unwrap();

    assert!(engine.try_layout(id1).is_some());
    assert!(engine.try_layout(id2).is_some());
    assert!(engine.try_layout(id3).is_some());
}

#[test]
fn test_compute_layout_single_node() {
    let mut engine = LayoutEngine::new();
    let mut style = Style::default();
    style.sizing.width = Size::Fixed(100);
    style.sizing.height = Size::Fixed(50);

    let id = DomId::new(1);
    engine.create_node(id, &style).unwrap();
    engine.compute(id, 200, 200).unwrap();

    let layout = engine.layout(id).unwrap();
    assert_eq!(layout.width, 200); // Root takes available space
    assert_eq!(layout.height, 200);
}

#[test]
fn test_compute_layout_with_children() {
    let mut engine = LayoutEngine::new();

    // Create children first
    let mut child_style = Style::default();
    child_style.sizing.width = Size::Fixed(50);
    child_style.sizing.height = Size::Fixed(30);
    let child1 = DomId::new(1);
    let child2 = DomId::new(2);
    engine.create_node(child1, &child_style).unwrap();
    engine.create_node(child2, &child_style).unwrap();

    // Create parent with children
    let mut parent_style = Style::default();
    parent_style.layout.display = Display::Flex;
    parent_style.layout.flex_direction = FlexDirection::Row;
    parent_style.sizing.width = Size::Fixed(200);
    parent_style.sizing.height = Size::Fixed(100);
    let parent = DomId::new(3);
    engine
        .create_node_with_children(parent, &parent_style, &[child1, child2])
        .unwrap();

    engine.compute(parent, 300, 300).unwrap();

    let parent_layout = engine.layout(parent).unwrap();
    assert_eq!(parent_layout.width, 300);
    assert_eq!(parent_layout.height, 300);

    let child1_layout = engine.layout(child1).unwrap();
    let child2_layout = engine.layout(child2).unwrap();

    // Children should be laid out in a row
    assert_eq!(child1_layout.x, 0);
    assert_eq!(child2_layout.x, 50); // After first child
}

#[test]
fn test_remove_node() {
    let mut engine = LayoutEngine::new();
    let style = Style::default();

    let id = DomId::new(1);
    engine.create_node(id, &style).unwrap();
    engine.compute(id, 100, 100).unwrap();
    assert!(engine.try_layout(id).is_some());

    engine.remove_node(id).unwrap();
    assert!(engine.try_layout(id).is_none());
}

#[test]
fn test_clear() {
    let mut engine = LayoutEngine::new();
    let style = Style::default();

    let id1 = DomId::new(1);
    let id2 = DomId::new(2);
    let id3 = DomId::new(3);

    engine.create_node(id1, &style).unwrap();
    engine.create_node(id2, &style).unwrap();
    engine.create_node(id3, &style).unwrap();

    engine.compute(id1, 100, 100).unwrap();
    assert!(engine.try_layout(id1).is_some());

    engine.clear();
    assert!(engine.try_layout(id1).is_none());
    assert!(engine.try_layout(id2).is_none());
    assert!(engine.try_layout(id3).is_none());
}

#[test]
fn test_layout_error_node_not_found() {
    let engine = LayoutEngine::new();
    let result = engine.layout(DomId::new(999));
    assert!(matches!(result, Err(LayoutError::NodeNotFound(_))));
}

#[test]
fn test_try_layout_returns_none_for_missing_node() {
    let engine = LayoutEngine::new();
    assert!(engine.try_layout(DomId::new(999)).is_none());
}
