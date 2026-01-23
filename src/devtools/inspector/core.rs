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
