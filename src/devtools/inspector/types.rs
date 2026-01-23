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
