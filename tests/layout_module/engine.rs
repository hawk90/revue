//! LayoutEngine Public API tests

#![allow(unused_imports)]

use revue::layout::*;

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
