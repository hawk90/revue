//! Widget tree inspector

use super::DevToolsConfig;
use crate::layout::Rect;
use crate::render::Buffer;
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

/// Widget tree inspector
#[derive(Debug, Default)]
pub struct Inspector {
    /// Widget nodes
    nodes: HashMap<usize, WidgetNode>,
    /// Root node IDs
    roots: Vec<usize>,
    /// Selected node ID
    selected: Option<usize>,
    /// Next node ID
    next_id: usize,
    /// Configuration
    config: InspectorConfig,
    /// Scroll offset
    scroll: usize,
}

impl Inspector {
    /// Create new inspector
    pub fn new() -> Self {
        Self::default()
    }

    /// Set configuration
    pub fn config(mut self, config: InspectorConfig) -> Self {
        self.config = config;
        self
    }

    /// Show bounds
    pub fn show_bounds(mut self, show: bool) -> Self {
        self.config.show_bounds = show;
        self
    }

    /// Show IDs
    pub fn show_ids(mut self, show: bool) -> Self {
        self.config.show_ids = show;
        self
    }

    /// Show classes
    pub fn show_classes(mut self, show: bool) -> Self {
        self.config.show_classes = show;
        self
    }

    /// Clear all nodes
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.roots.clear();
        self.selected = None;
        self.next_id = 0;
    }

    /// Add a root node
    pub fn add_root(&mut self, type_name: impl Into<String>) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let node = WidgetNode::new(id, type_name);
        self.nodes.insert(id, node);
        self.roots.push(id);
        id
    }

    /// Add a child node
    pub fn add_child(&mut self, parent_id: usize, type_name: impl Into<String>) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        let mut node = WidgetNode::new(id, type_name);
        node.parent = Some(parent_id);

        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(id);
        }

        self.nodes.insert(id, node);
        id
    }

    /// Get node by ID
    pub fn get(&self, id: usize) -> Option<&WidgetNode> {
        self.nodes.get(&id)
    }

    /// Get mutable node by ID
    pub fn get_mut(&mut self, id: usize) -> Option<&mut WidgetNode> {
        self.nodes.get_mut(&id)
    }

    /// Select node
    pub fn select(&mut self, id: Option<usize>) {
        // Deselect current
        if let Some(current) = self.selected {
            if let Some(node) = self.nodes.get_mut(&current) {
                node.selected = false;
            }
        }

        // Select new
        self.selected = id;
        if let Some(id) = id {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.selected = true;
            }
        }
    }

    /// Get selected node
    pub fn selected(&self) -> Option<&WidgetNode> {
        self.selected.and_then(|id| self.nodes.get(&id))
    }

    /// Toggle node expansion
    pub fn toggle_expand(&mut self, id: usize) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.expanded = !node.expanded;
        }
    }

    /// Select next visible node
    pub fn select_next(&mut self) {
        let visible = self.visible_nodes();
        if visible.is_empty() {
            return;
        }

        let current_idx = self
            .selected
            .and_then(|id| visible.iter().position(|&i| i == id))
            .unwrap_or(0);

        let next_idx = (current_idx + 1).min(visible.len() - 1);
        self.select(Some(visible[next_idx]));
    }

    /// Select previous visible node
    pub fn select_prev(&mut self) {
        let visible = self.visible_nodes();
        if visible.is_empty() {
            return;
        }

        let current_idx = self
            .selected
            .and_then(|id| visible.iter().position(|&i| i == id))
            .unwrap_or(0);

        let prev_idx = current_idx.saturating_sub(1);
        self.select(Some(visible[prev_idx]));
    }

    /// Get visible node IDs in order
    fn visible_nodes(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for &root in &self.roots {
            self.collect_visible(root, &mut result);
        }
        result
    }

    fn collect_visible(&self, id: usize, result: &mut Vec<usize>) {
        result.push(id);
        if let Some(node) = self.nodes.get(&id) {
            if node.expanded {
                for &child in &node.children {
                    self.collect_visible(child, result);
                }
            }
        }
    }

    /// Render inspector content
    pub fn render_content(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let mut y = area.y;
        let max_y = area.y + area.height;

        // Tree view
        let visible = self.visible_nodes();
        for (_i, &id) in visible.iter().enumerate().skip(self.scroll) {
            if y >= max_y {
                break;
            }

            if let Some(node) = self.nodes.get(&id) {
                self.render_node(buffer, area.x, y, area.width, node, config);
                y += 1;
            }
        }

        // Selected node details
        if let Some(node) = self.selected() {
            if y + 3 < max_y {
                y += 1;
                self.render_separator(buffer, area.x, y, area.width, config);
                y += 1;
                self.render_details(buffer, area.x, y, area.width, node, config);
            }
        }
    }

    fn render_node(
        &self,
        buffer: &mut Buffer,
        x: u16,
        y: u16,
        width: u16,
        node: &WidgetNode,
        config: &DevToolsConfig,
    ) {
        let depth = node.depth(&self.nodes);
        let indent = (depth * 2) as u16;

        let mut px = x + indent;

        // Expand/collapse indicator
        let indicator = if node.children.is_empty() {
            "  "
        } else if node.expanded {
            "▼ "
        } else {
            "▶ "
        };

        for ch in indicator.chars() {
            if px < x + width {
                if let Some(cell) = buffer.get_mut(px, y) {
                    cell.symbol = ch;
                    cell.fg = Some(config.accent_color);
                }
                px += 1;
            }
        }

        // Label
        let label = node.label();
        let fg = if node.selected {
            config.bg_color
        } else {
            config.fg_color
        };
        let bg = if node.selected {
            Some(config.accent_color)
        } else {
            None
        };

        for ch in label.chars() {
            if px < x + width {
                if let Some(cell) = buffer.get_mut(px, y) {
                    cell.symbol = ch;
                    cell.fg = Some(fg);
                    if let Some(b) = bg {
                        cell.bg = Some(b);
                    }
                }
                px += 1;
            }
        }
    }

    fn render_separator(
        &self,
        buffer: &mut Buffer,
        x: u16,
        y: u16,
        width: u16,
        config: &DevToolsConfig,
    ) {
        for px in x..x + width {
            if let Some(cell) = buffer.get_mut(px, y) {
                cell.symbol = '─';
                cell.fg = Some(config.accent_color);
            }
        }
    }

    fn render_details(
        &self,
        buffer: &mut Buffer,
        x: u16,
        y: u16,
        _width: u16,
        node: &WidgetNode,
        config: &DevToolsConfig,
    ) {
        let mut py = y;

        // Type
        let type_line = format!("Type: {}", node.type_name);
        self.draw_text(buffer, x, py, &type_line, config.fg_color);
        py += 1;

        // Rect
        if self.config.show_rect {
            let rect_line = format!(
                "Rect: {}x{} @ ({}, {})",
                node.rect.width, node.rect.height, node.rect.x, node.rect.y
            );
            self.draw_text(buffer, x, py, &rect_line, config.fg_color);
        }
    }

    fn draw_text(&self, buffer: &mut Buffer, x: u16, y: u16, text: &str, color: Color) {
        for (i, ch) in text.chars().enumerate() {
            if let Some(cell) = buffer.get_mut(x + i as u16, y) {
                cell.symbol = ch;
                cell.fg = Some(color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
