//! Widget tree inspector

pub use core::Inspector;
pub use picker::ComponentPicker;
pub use types::{InspectorConfig, PickerMode, WidgetNode};

mod core;
mod picker;
#[cfg(test)]
mod tests {
//! Tests for widget inspector

#![allow(unused_imports)]

use super::super::{Inspector, PickerMode, WidgetNode};
use crate::layout::Rect;

#[test]
fn test_inspector_add_nodes() {
    let mut inspector = Inspector::new();
    let root = inspector.add_root("VStack");
    let child = inspector.add_child(root, "Text");

    assert!(inspector.get(root).is_some());
    assert!(inspector.get(child).is_some());
    assert_eq!(inspector.get(child).unwrap().parent, Some(root));
}

#[test]
fn test_widget_node_label() {
    let node = WidgetNode::new(0, "Button")
        .widget_id("submit")
        .class("primary");

    assert_eq!(node.label(), "Button#submit.primary");
}

#[test]
fn test_inspector_select() {
    let mut inspector = Inspector::new();
    let id = inspector.add_root("Test");

    inspector.select(Some(id));
    assert!(inspector.selected().is_some());
    assert!(inspector.get(id).unwrap().selected);
}

// ==========================================================================
// PickerMode tests
// ==========================================================================

#[test]
fn test_picker_mode_default() {
    assert_eq!(PickerMode::default(), PickerMode::Disabled);
}

// ==========================================================================
// ComponentPicker tests
// ==========================================================================

#[test]
fn test_component_picker_creation() {
    let picker = super::picker::ComponentPicker::new();
    assert_eq!(picker.mode(), PickerMode::Disabled);
    assert!(!picker.is_active());
    assert!(picker.hovered_node().is_none());
    assert!(picker.mouse_pos().is_none());
}

#[test]
fn test_component_picker_enable_disable() {
    let mut picker = super::picker::ComponentPicker::new();

    picker.enable();
    assert_eq!(picker.mode(), PickerMode::Active);
    assert!(picker.is_active());

    picker.disable();
    assert_eq!(picker.mode(), PickerMode::Disabled);
    assert!(!picker.is_active());
}

#[test]
fn test_component_picker_toggle() {
    let mut picker = super::picker::ComponentPicker::new();

    picker.toggle();
    assert!(picker.is_active());

    picker.toggle();
    assert!(!picker.is_active());
}

#[test]
fn test_component_picker_set_hovered() {
    let mut picker = super::picker::ComponentPicker::new();
    picker.enable();

    picker.set_hovered(Some(42));
    assert_eq!(picker.hovered_node(), Some(42));
    assert_eq!(picker.mode(), PickerMode::Hovering);

    picker.set_hovered(None);
    assert_eq!(picker.hovered_node(), None);
    assert_eq!(picker.mode(), PickerMode::Active);
}

#[test]
fn test_component_picker_mouse_pos() {
    let mut picker = super::picker::ComponentPicker::new();

    picker.set_mouse_pos(10, 20);
    assert_eq!(picker.mouse_pos(), Some((10, 20)));
}

#[test]
fn test_component_picker_disable_clears_state() {
    let mut picker = super::picker::ComponentPicker::new();
    picker.enable();
    picker.set_hovered(Some(5));
    picker.set_mouse_pos(100, 200);

    picker.disable();

    assert!(!picker.is_active());
    assert!(picker.hovered_node().is_none());
    // mouse_pos is not cleared by disable, only hovered_node
}

#[test]
fn test_component_picker_find_node_at() {
    let picker = super::picker::ComponentPicker::new();
    let mut nodes = std::collections::HashMap::new();

    // Create a parent node
    let mut parent = WidgetNode::new(0, "Parent");
    parent.rect = Rect::new(0, 0, 100, 100);
    nodes.insert(0, parent);

    // Create a child node inside parent
    let mut child = WidgetNode::new(1, "Child");
    child.rect = Rect::new(10, 10, 50, 50);
    child.parent = Some(0);
    nodes.insert(1, child);

    // Point inside child should find child (deeper node)
    let found = picker.find_node_at(20, 20, &nodes);
    assert_eq!(found, Some(1));

    // Point outside child but inside parent should find parent
    let found = picker.find_node_at(80, 80, &nodes);
    assert_eq!(found, Some(0));

    // Point outside all nodes
    let found = picker.find_node_at(200, 200, &nodes);
    assert!(found.is_none());
}

#[test]
fn test_component_picker_highlight_rect() {
    let mut picker = super::picker::ComponentPicker::new();
    let mut nodes = std::collections::HashMap::new();

    let mut node = WidgetNode::new(0, "Button");
    node.rect = Rect::new(5, 10, 20, 15);
    nodes.insert(0, node);

    picker.set_hovered(Some(0));
    let rect = picker.highlight_rect(&nodes);

    assert!(rect.is_some());
    let r = rect.unwrap();
    assert_eq!(r.x, 5);
    assert_eq!(r.y, 10);
    assert_eq!(r.width, 20);
    assert_eq!(r.height, 15);
}

#[test]
fn test_component_picker_tooltip_text() {
    let mut picker = super::picker::ComponentPicker::new().show_tooltip(true);
    let mut nodes = std::collections::HashMap::new();

    let mut node = WidgetNode::new(0, "Button");
    node.widget_id = Some("submit".to_string());
    nodes.insert(0, node);

    picker.set_hovered(Some(0));
    let tooltip = picker.tooltip_text(&nodes);

    assert!(tooltip.is_some());
    assert!(tooltip.unwrap().contains("Button"));
}

#[test]
fn test_component_picker_tooltip_disabled() {
    let mut picker = super::picker::ComponentPicker::new().show_tooltip(false);
    let mut nodes = std::collections::HashMap::new();

    let node = WidgetNode::new(0, "Button");
    nodes.insert(0, node);

    picker.set_hovered(Some(0));
    let tooltip = picker.tooltip_text(&nodes);

    assert!(tooltip.is_none());
}

#[test]
fn test_component_picker_highlight_color() {
    let color = crate::style::Color::rgb(255, 100, 50);
    let picker = super::picker::ComponentPicker::new().highlight_color(color);

    // The highlight color is private, so we just verify the builder works
    assert!(!picker.is_active());
}

// ==========================================================================
// Inspector picker integration tests
// ==========================================================================

#[test]
fn test_inspector_toggle_picker() {
    let mut inspector = Inspector::new();

    inspector.toggle_picker();
    assert!(inspector.picker().is_active());

    inspector.toggle_picker();
    assert!(!inspector.picker().is_active());
}

#[test]
fn test_inspector_picker_click() {
    let mut inspector = Inspector::new();

    // Add a node with a rect
    let id = inspector.add_root("Button");
    if let Some(node) = inspector.get_mut(id) {
        node.rect = Rect::new(0, 0, 50, 20);
    }

    inspector.toggle_picker();

    // Click inside the node
    let clicked = inspector.picker_click(10, 10);
    assert_eq!(clicked, Some(id));
    assert!(!inspector.picker().is_active()); // Picker should be disabled after click
}

#[test]
fn test_inspector_reveal_node() {
    let mut inspector = Inspector::new();
    let root = inspector.add_root("Root");
    let child1 = inspector.add_child(root, "Child1");
    let child2 = inspector.add_child(child1, "Child2");

    // Collapse all initially
    if let Some(node) = inspector.get_mut(root) {
        node.expanded = false;
    }
    if let Some(node) = inspector.get_mut(child1) {
        node.expanded = false;
    }

    // Reveal the deepest node
    inspector.reveal_node(child2);

    // Parent nodes should now be expanded
    assert!(inspector.get(root).unwrap().expanded);
    assert!(inspector.get(child1).unwrap().expanded);
}

#[test]
fn test_widget_node_depth() {
    let mut nodes = std::collections::HashMap::new();

    let root = WidgetNode::new(0, "Root");
    nodes.insert(0, root);

    let mut child = WidgetNode::new(1, "Child");
    child.parent = Some(0);
    nodes.insert(1, child);

    let mut grandchild = WidgetNode::new(2, "GrandChild");
    grandchild.parent = Some(1);
    nodes.insert(2, grandchild);

    assert_eq!(nodes.get(&0).unwrap().depth(&nodes), 0);
    assert_eq!(nodes.get(&1).unwrap().depth(&nodes), 1);
    assert_eq!(nodes.get(&2).unwrap().depth(&nodes), 2);
}

}
mod types;
