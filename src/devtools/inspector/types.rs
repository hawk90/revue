//! Core types and enums for the widget inspector

use crate::layout::Rect;
use crate::style::Color;
use std::collections::HashMap;

/// Widget node in the tree
#[derive(Debug, Clone)]
pub struct WidgetNode {
    /// Node ID
    pub id: usize,
    /// Widget type name
    pub type_name: String,
    /// Optional widget ID
    pub widget_id: Option<String>,
    /// CSS classes
    pub classes: Vec<String>,
    /// Bounding rect
    pub rect: Rect,
    /// Parent node ID
    pub parent: Option<usize>,
    /// Child node IDs
    pub children: Vec<usize>,
    /// Is expanded in tree view
    pub expanded: bool,
    /// Is selected
    pub selected: bool,
    /// Is hovered
    pub hovered: bool,
    /// Is focused
    pub focused: bool,
}

impl WidgetNode {
    /// Create a new widget node
    pub fn new(id: usize, type_name: impl Into<String>) -> Self {
        Self {
            id,
            type_name: type_name.into(),
            widget_id: None,
            classes: Vec::new(),
            rect: Rect::default(),
            parent: None,
            children: Vec::new(),
            expanded: true,
            selected: false,
            hovered: false,
            focused: false,
        }
    }

    /// Set widget ID
    pub fn widget_id(mut self, id: impl Into<String>) -> Self {
        self.widget_id = Some(id.into());
        self
    }

    /// Add class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Set rect
    pub fn rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    /// Get display label
    pub fn label(&self) -> String {
        let mut label = self.type_name.clone();
        if let Some(ref id) = self.widget_id {
            label.push_str(&format!("#{}", id));
        }
        for class in &self.classes {
            label.push_str(&format!(".{}", class));
        }
        label
    }

    /// Get depth in tree
    pub fn depth(&self, nodes: &HashMap<usize, WidgetNode>) -> usize {
        let mut depth = 0;
        let mut current = self.parent;
        while let Some(parent_id) = current {
            depth += 1;
            current = nodes.get(&parent_id).and_then(|n| n.parent);
        }
        depth
    }
}

/// Inspector configuration
#[derive(Debug, Clone)]
pub struct InspectorConfig {
    /// Show bounding boxes
    pub show_bounds: bool,
    /// Show widget IDs
    pub show_ids: bool,
    /// Show classes
    pub show_classes: bool,
    /// Show rect info
    pub show_rect: bool,
    /// Highlight color
    pub highlight_color: Color,
}

impl Default for InspectorConfig {
    fn default() -> Self {
        Self {
            show_bounds: true,
            show_ids: true,
            show_classes: true,
            show_rect: true,
            highlight_color: Color::rgb(100, 200, 255),
        }
    }
}

/// Component picker mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PickerMode {
    /// Picker is disabled
    #[default]
    Disabled,
    /// Picker is active, waiting for user to click
    Active,
    /// Picker is hovering over a component
    Hovering,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_node_new() {
        let node = WidgetNode::new(1, "Button");
        assert_eq!(node.id, 1);
        assert_eq!(node.type_name, "Button");
        assert_eq!(node.widget_id, None);
        assert!(node.classes.is_empty());
        assert_eq!(node.parent, None);
        assert!(node.children.is_empty());
        assert!(node.expanded);
        assert!(!node.selected);
        assert!(!node.hovered);
        assert!(!node.focused);
    }

    #[test]
    fn test_widget_node_builder() {
        let node = WidgetNode::new(1, "Button")
            .widget_id("my-button")
            .class("btn")
            .class("primary")
            .rect(Rect::new(5, 5, 100, 50));

        assert_eq!(node.widget_id, Some("my-button".to_string()));
        assert_eq!(node.classes.len(), 2);
        assert!(node.classes.contains(&"btn".to_string()));
        assert!(node.classes.contains(&"primary".to_string()));
        assert_eq!(node.rect, Rect::new(5, 5, 100, 50));
    }

    #[test]
    fn test_widget_node_label() {
        let node = WidgetNode::new(1, "Button").widget_id("my-btn");
        assert_eq!(node.label(), "Button#my-btn");

        let node = WidgetNode::new(2, "Button")
            .widget_id("my-btn")
            .class("btn")
            .class("primary");
        assert_eq!(node.label(), "Button#my-btn.btn.primary");
    }

    #[test]
    fn test_widget_node_label_no_id() {
        let node = WidgetNode::new(1, "Button");
        assert_eq!(node.label(), "Button");
    }

    #[test]
    fn test_widget_node_depth() {
        let mut nodes = HashMap::new();

        // Create a tree: root -> child1 -> child2
        let root = WidgetNode::new(0, "Root");
        let child1 = WidgetNode::new(1, "Child1");
        let child2 = WidgetNode::new(2, "Child2");

        // Set up parent relationships
        let mut child1_with_parent = child1;
        child1_with_parent.parent = Some(0);
        let mut child2_with_parent = child2;
        child2_with_parent.parent = Some(1);

        nodes.insert(0, root);
        nodes.insert(1, child1_with_parent);
        nodes.insert(2, child2_with_parent);

        // Check depths
        assert_eq!(nodes[&0].depth(&nodes), 0);
        assert_eq!(nodes[&1].depth(&nodes), 1);
        assert_eq!(nodes[&2].depth(&nodes), 2);
    }

    #[test]
    fn test_widget_node_depth_no_parent() {
        let node = WidgetNode::new(1, "Button");
        let mut nodes = HashMap::new();
        nodes.insert(1, node);
        assert_eq!(nodes[&1].depth(&nodes), 0);
    }

    #[test]
    fn test_widget_node_public_fields() {
        let mut node = WidgetNode::new(1, "Button");
        node.id = 5;
        node.type_name = "Text".to_string();
        node.selected = true;
        node.hovered = true;
        node.focused = true;

        assert_eq!(node.id, 5);
        assert_eq!(node.type_name, "Text");
        assert!(node.selected);
        assert!(node.hovered);
        assert!(node.focused);
    }

    #[test]
    fn test_inspector_config_default() {
        let config = InspectorConfig::default();
        assert!(config.show_bounds);
        assert!(config.show_ids);
        assert!(config.show_classes);
        assert!(config.show_rect);
        assert_eq!(config.highlight_color, Color::rgb(100, 200, 255));
    }

    #[test]
    fn test_inspector_config_public_fields() {
        let mut config = InspectorConfig::default();
        config.show_bounds = false;
        config.show_ids = false;
        config.highlight_color = Color::rgb(255, 0, 0);

        assert!(!config.show_bounds);
        assert!(!config.show_ids);
        assert_eq!(config.highlight_color, Color::rgb(255, 0, 0));
    }

    #[test]
    fn test_picker_mode_default() {
        let mode = PickerMode::default();
        assert_eq!(mode, PickerMode::Disabled);
    }

    #[test]
    fn test_picker_mode_equality() {
        assert_eq!(PickerMode::Active, PickerMode::Active);
        assert_ne!(PickerMode::Active, PickerMode::Disabled);
        assert_ne!(PickerMode::Active, PickerMode::Hovering);
    }

    // =========================================================================
    // Additional inspector types tests
    // =========================================================================

    #[test]
    fn test_widget_node_clone() {
        let mut node = WidgetNode::new(1, "Button").widget_id("test").class("btn");
        node.selected = true;
        let cloned = node.clone();
        assert_eq!(node.id, cloned.id);
        assert_eq!(node.type_name, cloned.type_name);
        assert_eq!(node.widget_id, cloned.widget_id);
        assert_eq!(node.selected, cloned.selected);
    }

    #[test]
    fn test_widget_node_label_with_classes_only() {
        let node = WidgetNode::new(1, "Button").class("btn").class("primary");
        assert_eq!(node.label(), "Button.btn.primary");
    }

    #[test]
    fn test_widget_node_label_with_multiple_classes() {
        let node = WidgetNode::new(1, "Div")
            .widget_id("container")
            .class("flex")
            .class("center")
            .class("container");
        assert_eq!(node.label(), "Div#container.flex.center.container");
    }

    #[test]
    fn test_widget_node_depth_with_missing_parent() {
        let mut nodes = HashMap::new();
        let node = WidgetNode::new(1, "Button");
        let mut node_with_parent = node;
        node_with_parent.parent = Some(999); // Non-existent parent
        nodes.insert(1, node_with_parent);
        // Should handle missing parent gracefully
        assert_eq!(nodes[&1].depth(&nodes), 1);
    }

    #[test]
    fn test_widget_node_depth_complex_tree() {
        let mut nodes = HashMap::new();
        // root (0) -> a (1) -> b (2) -> c (3) -> d (4)
        for i in 0..=4 {
            let mut node = WidgetNode::new(i, format!("Node{}", i));
            if i > 0 {
                node.parent = Some(i - 1);
            }
            nodes.insert(i, node);
        }
        assert_eq!(nodes[&0].depth(&nodes), 0);
        assert_eq!(nodes[&1].depth(&nodes), 1);
        assert_eq!(nodes[&2].depth(&nodes), 2);
        assert_eq!(nodes[&3].depth(&nodes), 3);
        assert_eq!(nodes[&4].depth(&nodes), 4);
    }

    #[test]
    fn test_widget_node_expanded_default() {
        let node = WidgetNode::new(1, "Button");
        assert!(node.expanded);
    }

    #[test]
    fn test_widget_node_children_default_empty() {
        let node = WidgetNode::new(1, "Button");
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_widget_node_state_flags() {
        let mut node = WidgetNode::new(1, "Button");
        node.selected = true;
        node.hovered = true;
        node.focused = true;
        node.expanded = false;
        assert!(node.selected);
        assert!(node.hovered);
        assert!(node.focused);
        assert!(!node.expanded);
    }

    #[test]
    fn test_widget_node_with_string_type_name() {
        let type_name = String::from("CustomButton");
        let node = WidgetNode::new(1, type_name.clone());
        assert_eq!(node.type_name, type_name);
    }

    #[test]
    fn test_widget_node_widget_id_with_string() {
        let widget_id = String::from("my-button-id");
        let node = WidgetNode::new(1, "Button").widget_id(widget_id.clone());
        assert_eq!(node.widget_id, Some(widget_id));
    }

    #[test]
    fn test_inspector_config_clone() {
        let config = InspectorConfig::default();
        let cloned = config.clone();
        assert_eq!(config.show_bounds, cloned.show_bounds);
        assert_eq!(config.show_ids, cloned.show_ids);
        assert_eq!(config.show_classes, cloned.show_classes);
        assert_eq!(config.show_rect, cloned.show_rect);
        assert_eq!(config.highlight_color, cloned.highlight_color);
    }

    #[test]
    fn test_inspector_config_custom_color() {
        let config = InspectorConfig {
            show_bounds: true,
            show_ids: false,
            show_classes: false,
            show_rect: false,
            highlight_color: Color::YELLOW,
        };
        assert_eq!(config.highlight_color, Color::YELLOW);
        assert!(!config.show_ids);
        assert!(!config.show_classes);
        assert!(!config.show_rect);
    }

    #[test]
    fn test_picker_mode_all_variants() {
        let modes = [
            PickerMode::Disabled,
            PickerMode::Active,
            PickerMode::Hovering,
        ];
        for (i, m1) in modes.iter().enumerate() {
            for (j, m2) in modes.iter().enumerate() {
                if i == j {
                    assert_eq!(m1, m2);
                } else {
                    assert_ne!(m1, m2);
                }
            }
        }
    }

    #[test]
    fn test_picker_mode_copy_trait() {
        let mode = PickerMode::Active;
        let copied = mode; // Copy trait
        assert_eq!(mode, copied);
    }

    #[test]
    fn test_widget_node_rect_default() {
        let node = WidgetNode::new(1, "Button");
        assert_eq!(node.rect, Rect::default());
    }

    #[test]
    fn test_widget_node_rect_builder() {
        let rect = Rect::new(10, 20, 100, 50);
        let node = WidgetNode::new(1, "Button").rect(rect);
        assert_eq!(node.rect, rect);
    }

    #[test]
    fn test_inspector_config_all_false() {
        let config = InspectorConfig {
            show_bounds: false,
            show_ids: false,
            show_classes: false,
            show_rect: false,
            highlight_color: Color::BLACK,
        };
        assert!(!config.show_bounds);
        assert!(!config.show_ids);
        assert!(!config.show_classes);
        assert!(!config.show_rect);
    }

    #[test]
    fn test_widget_node_debug() {
        let node = WidgetNode::new(1, "Button");
        let debug_str = format!("{:?}", node);
        assert!(debug_str.contains("Button"));
    }
}
