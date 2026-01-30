//! Widget tree inspector core implementation

use super::super::helpers::draw_text_overlay;
use super::super::DevToolsConfig;
use super::picker::ComponentPicker;
use super::types::{InspectorConfig, WidgetNode};
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use std::collections::HashMap;

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
    /// Component picker
    picker: ComponentPicker,
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

    // =========================================================================
    // Component Picker Methods
    // =========================================================================

    /// Get picker reference
    pub fn picker(&self) -> &ComponentPicker {
        &self.picker
    }

    /// Get mutable picker reference
    pub fn picker_mut(&mut self) -> &mut ComponentPicker {
        &mut self.picker
    }

    /// Toggle picker mode (Ctrl+Shift+C shortcut)
    pub fn toggle_picker(&mut self) {
        self.picker.toggle();
    }

    /// Enable picker mode
    pub fn enable_picker(&mut self) {
        self.picker.enable();
    }

    /// Disable picker mode
    pub fn disable_picker(&mut self) {
        self.picker.disable();
    }

    /// Check if picker is active
    pub fn is_picker_active(&self) -> bool {
        self.picker.is_active()
    }

    /// Handle mouse move in picker mode
    pub fn picker_mouse_move(&mut self, x: u16, y: u16) {
        if !self.picker.is_active() {
            return;
        }

        self.picker.set_mouse_pos(x, y);
        let hovered = self.picker.find_node_at(x, y, &self.nodes);
        self.picker.set_hovered(hovered);
    }

    /// Handle mouse click in picker mode
    ///
    /// Returns the selected node ID if a component was clicked
    pub fn picker_click(&mut self, x: u16, y: u16) -> Option<usize> {
        if !self.picker.is_active() {
            return None;
        }

        if let Some(node_id) = self.picker.find_node_at(x, y, &self.nodes) {
            // Select the clicked component
            self.select(Some(node_id));
            // Disable picker after selection
            self.picker.disable();
            // Expand parents to show selected node
            self.reveal_node(node_id);
            return Some(node_id);
        }

        None
    }

    /// Reveal a node by expanding all its parents
    pub fn reveal_node(&mut self, node_id: usize) {
        // Collect parent chain
        let mut parents = Vec::new();
        let mut current = self.nodes.get(&node_id).and_then(|n| n.parent);

        while let Some(parent_id) = current {
            parents.push(parent_id);
            current = self.nodes.get(&parent_id).and_then(|n| n.parent);
        }

        // Expand all parents
        for parent_id in parents {
            if let Some(node) = self.nodes.get_mut(&parent_id) {
                node.expanded = true;
            }
        }
    }

    /// Render picker overlay
    pub fn render_picker_overlay(&self, buffer: &mut Buffer, area: Rect) {
        self.picker.render_overlay(buffer, area, &self.nodes);
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
        draw_text_overlay(buffer, x, y, text, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn create_test_inspector() -> Inspector {
        Inspector::new()
    }

    #[test]
    fn test_inspector_new() {
        let inspector = Inspector::new();
        assert!(inspector.nodes.is_empty());
        assert!(inspector.roots.is_empty());
        assert!(inspector.selected.is_none());
        assert_eq!(inspector.next_id, 0);
    }

    #[test]
    fn test_inspector_default() {
        let inspector = Inspector::default();
        assert!(inspector.nodes.is_empty());
        assert!(inspector.roots.is_empty());
    }

    #[test]
    fn test_show_bounds() {
        let inspector = Inspector::new().show_bounds(true);
        assert!(inspector.config.show_bounds);
    }

    #[test]
    fn test_show_ids() {
        let inspector = Inspector::new().show_ids(true);
        assert!(inspector.config.show_ids);
    }

    #[test]
    fn test_show_classes() {
        let inspector = Inspector::new().show_classes(true);
        assert!(inspector.config.show_classes);
    }

    #[test]
    fn test_config() {
        let config = InspectorConfig {
            show_bounds: true,
            show_ids: true,
            show_classes: true,
            show_rect: true,
            highlight_color: Color::rgb(255, 255, 255),
        };
        let inspector = Inspector::new().config(config);
        assert!(inspector.config.show_bounds);
        assert!(inspector.config.show_ids);
        assert!(inspector.config.show_classes);
        assert!(inspector.config.show_rect);
    }

    #[test]
    fn test_clear() {
        let mut inspector = Inspector::new();
        inspector.add_root("Root");
        inspector.select(Some(0));
        assert_eq!(inspector.nodes.len(), 1);
        assert!(inspector.selected.is_some());

        inspector.clear();
        assert!(inspector.nodes.is_empty());
        assert!(inspector.roots.is_empty());
        assert!(inspector.selected.is_none());
        assert_eq!(inspector.next_id, 0);
    }

    #[test]
    fn test_add_root() {
        let mut inspector = Inspector::new();

        let id1 = inspector.add_root("Root1");
        assert_eq!(id1, 0);
        assert_eq!(inspector.nodes.len(), 1);
        assert_eq!(inspector.roots.len(), 1);
        assert_eq!(inspector.next_id, 1);

        let id2 = inspector.add_root("Root2");
        assert_eq!(id2, 1);
        assert_eq!(inspector.nodes.len(), 2);
        assert_eq!(inspector.roots.len(), 2);
        assert_eq!(inspector.next_id, 2);
    }

    #[test]
    fn test_add_child() {
        let mut inspector = Inspector::new();

        let root = inspector.add_root("Root");
        assert_eq!(root, 0);

        let child = inspector.add_child(root, "Child");
        assert_eq!(child, 1);
        assert_eq!(inspector.nodes.len(), 2);

        // Check parent-child relationship
        let parent = inspector.get(root).unwrap();
        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.children[0], child);

        let child_node = inspector.get(child).unwrap();
        assert_eq!(child_node.parent, Some(root));
    }

    #[test]
    fn test_get() {
        let mut inspector = Inspector::new();

        let id = inspector.add_root("Root");
        let node = inspector.get(id);
        assert!(node.is_some());
        assert_eq!(node.unwrap().type_name, "Root");

        let non_existent = inspector.get(999);
        assert!(non_existent.is_none());
    }

    #[test]
    fn test_get_mut() {
        let mut inspector = Inspector::new();

        let id = inspector.add_root("Root");
        let node = inspector.get_mut(id);
        assert!(node.is_some());
        assert_eq!(node.unwrap().type_name, "Root");

        let non_existent = inspector.get_mut(999);
        assert!(non_existent.is_none());
    }

    #[test]
    fn test_select() {
        let mut inspector = Inspector::new();

        let id1 = inspector.add_root("Root1");
        let id2 = inspector.add_root("Root2");

        // Select first node
        inspector.select(Some(id1));
        assert_eq!(inspector.selected, Some(id1));
        let node = inspector.get(id1).unwrap();
        assert!(node.selected);

        // Select second node (first should be deselected)
        inspector.select(Some(id2));
        assert_eq!(inspector.selected, Some(id2));
        let node1 = inspector.get(id1).unwrap();
        assert!(!node1.selected);
        let node2 = inspector.get(id2).unwrap();
        assert!(node2.selected);

        // Deselect
        inspector.select(None);
        assert!(inspector.selected.is_none());
        let node2 = inspector.get(id2).unwrap();
        assert!(!node2.selected);
    }

    #[test]
    fn test_selected() {
        let mut inspector = Inspector::new();

        assert!(inspector.selected().is_none());

        let id = inspector.add_root("Root");
        inspector.select(Some(id));

        let selected = inspector.selected();
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().type_name, "Root");
    }

    #[test]
    fn test_toggle_expand() {
        let mut inspector = Inspector::new();

        let root = inspector.add_root("Root");
        inspector.add_child(root, "Child");

        let node = inspector.get(root).unwrap();
        // Nodes start expanded by default
        assert!(node.expanded);

        // Collapse it
        inspector.toggle_expand(root);
        let node = inspector.get(root).unwrap();
        assert!(!node.expanded);

        // Expand it again
        inspector.toggle_expand(root);
        let node = inspector.get(root).unwrap();
        assert!(node.expanded);
    }

    #[test]
    fn test_select_next() {
        let mut inspector = Inspector::new();

        let _id1 = inspector.add_root("Root1");
        let id2 = inspector.add_root("Root2");
        let id3 = inspector.add_root("Root3");

        // No selection - select_next advances to second node
        inspector.select_next();
        assert_eq!(inspector.selected, Some(id2));

        // Select next (now at third)
        inspector.select_next();
        assert_eq!(inspector.selected, Some(id3));

        // Select next again
        inspector.select_next();
        assert_eq!(inspector.selected, Some(id3));

        // Already at last - should stay at last
        inspector.select_next();
        assert_eq!(inspector.selected, Some(id3));
    }

    #[test]
    fn test_select_prev() {
        let mut inspector = Inspector::new();

        let id1 = inspector.add_root("Root1");
        let id2 = inspector.add_root("Root2");
        let id3 = inspector.add_root("Root3");

        // Select last first
        inspector.select(Some(id3));

        // Select previous
        inspector.select_prev();
        assert_eq!(inspector.selected, Some(id2));

        // Select previous again
        inspector.select_prev();
        assert_eq!(inspector.selected, Some(id1));

        // Already at first - should stay at first
        inspector.select_prev();
        assert_eq!(inspector.selected, Some(id1));
    }

    #[test]
    fn test_select_next_with_children() {
        let mut inspector = Inspector::new();

        let root = inspector.add_root("Root");
        let child1 = inspector.add_child(root, "Child1");
        let child2 = inspector.add_child(root, "Child2");

        // Parent starts expanded by default, so children are visible
        // No selection -> select_next advances to second visible node
        inspector.select_next();
        assert_eq!(inspector.selected, Some(child1));

        // Next should be third node (child2)
        inspector.select_next();
        assert_eq!(inspector.selected, Some(child2));
    }

    #[test]
    fn test_picker() {
        let inspector = Inspector::new();
        let _picker = inspector.picker();
    }

    #[test]
    fn test_picker_mut() {
        let mut inspector = Inspector::new();
        let _picker = inspector.picker_mut();
    }

    #[test]
    fn test_is_picker_active() {
        let inspector = Inspector::new();
        assert!(!inspector.is_picker_active());
    }

    #[test]
    fn test_toggle_picker() {
        let mut inspector = Inspector::new();
        assert!(!inspector.is_picker_active());

        inspector.toggle_picker();
        // The picker state might be internal, just check it doesn't crash

        inspector.toggle_picker();
        // Just check it doesn't crash
    }

    #[test]
    fn test_enable_picker() {
        let mut inspector = Inspector::new();
        inspector.enable_picker();
        // Just check it doesn't crash
    }

    #[test]
    fn test_disable_picker() {
        let mut inspector = Inspector::new();
        inspector.disable_picker();
        // Just check it doesn't crash
    }

    #[test]
    fn test_picker_mouse_move() {
        let mut inspector = Inspector::new();
        inspector.picker_mouse_move(10, 20);
        // Just check it doesn't crash when picker is inactive
    }

    #[test]
    fn test_picker_click() {
        let mut inspector = Inspector::new();
        let result = inspector.picker_click(10, 20);
        // Should return None when picker is inactive
        assert!(result.is_none());
    }

    #[test]
    fn test_reveal_node() {
        let mut inspector = Inspector::new();

        let root = inspector.add_root("Root");
        let child1 = inspector.add_child(root, "Child1");
        let child2 = inspector.add_child(child1, "Child2");

        // All nodes start expanded by default
        let root_node = inspector.get(root).unwrap();
        assert!(root_node.expanded);
        let child1_node = inspector.get(child1).unwrap();
        assert!(child1_node.expanded);

        // Collapse them first to test reveal
        inspector.toggle_expand(root);
        inspector.toggle_expand(child1);

        // Verify collapsed
        assert!(!inspector.get(root).unwrap().expanded);
        assert!(!inspector.get(child1).unwrap().expanded);

        // Reveal the nested child
        inspector.reveal_node(child2);

        // Parents should now be expanded
        let root_node = inspector.get(root).unwrap();
        assert!(root_node.expanded);
        let child1_node = inspector.get(child1).unwrap();
        assert!(child1_node.expanded);
    }

    #[test]
    fn test_reveal_root_node() {
        let mut inspector = Inspector::new();

        let root = inspector.add_root("Root");
        inspector.reveal_node(root);

        // Should not crash for root node (no parents)
        // Root should remain expanded (its default state)
        let root_node = inspector.get(root).unwrap();
        assert!(root_node.expanded);
    }

    #[test]
    fn test_visible_nodes_single() {
        let mut inspector = Inspector::new();
        inspector.add_root("Root");

        // Use public methods that depend on visible_nodes
        let visible_count = {
            let inspector = &inspector;
            inspector.visible_nodes().len()
        };

        assert_eq!(visible_count, 1);
    }

    #[test]
    fn test_visible_nodes_with_hierarchy() {
        let mut inspector = Inspector::new();

        let root = inspector.add_root("Root");
        inspector.add_child(root, "Child1");
        inspector.add_child(root, "Child2");

        // Children are visible by default (parent is expanded)
        let visible = inspector.visible_nodes();
        assert_eq!(visible.len(), 3);
        assert_eq!(visible[0], root);

        // Collapse root
        inspector.toggle_expand(root);

        // Now children should not be visible
        let visible = inspector.visible_nodes();
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0], root);
    }

    #[test]
    fn test_multiple_roots() {
        let mut inspector = Inspector::new();

        let root1 = inspector.add_root("Root1");
        let _root2 = inspector.add_root("Root2");
        let _child1 = inspector.add_child(root1, "Child1");

        // First root is expanded by default
        let visible = inspector.visible_nodes();
        assert_eq!(visible.len(), 3); // root1 (expanded with child1), root2
    }

    #[test]
    fn test_select_wrapping() {
        let mut inspector = Inspector::new();

        let id = inspector.add_root("Root");

        // At single node, next/prev should stay at that node
        inspector.select_next();
        assert_eq!(inspector.selected, Some(id));

        inspector.select_prev();
        assert_eq!(inspector.selected, Some(id));
    }
}
