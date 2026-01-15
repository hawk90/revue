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

/// Component picker for click-to-inspect functionality
#[derive(Debug, Default)]
pub struct ComponentPicker {
    /// Current mode
    mode: PickerMode,
    /// Currently hovered node ID
    hovered_node: Option<usize>,
    /// Highlight color
    highlight_color: Color,
    /// Tooltip visible
    show_tooltip: bool,
    /// Mouse position
    mouse_pos: Option<(u16, u16)>,
}

impl ComponentPicker {
    /// Create a new component picker
    pub fn new() -> Self {
        Self {
            mode: PickerMode::Disabled,
            hovered_node: None,
            highlight_color: Color::rgb(100, 200, 255),
            show_tooltip: true,
            mouse_pos: None,
        }
    }

    /// Set highlight color
    pub fn highlight_color(mut self, color: Color) -> Self {
        self.highlight_color = color;
        self
    }

    /// Toggle tooltip visibility
    pub fn show_tooltip(mut self, show: bool) -> Self {
        self.show_tooltip = show;
        self
    }

    /// Get current mode
    pub fn mode(&self) -> PickerMode {
        self.mode
    }

    /// Check if picker is active
    pub fn is_active(&self) -> bool {
        self.mode != PickerMode::Disabled
    }

    /// Enable picker mode
    pub fn enable(&mut self) {
        self.mode = PickerMode::Active;
    }

    /// Disable picker mode
    pub fn disable(&mut self) {
        self.mode = PickerMode::Disabled;
        self.hovered_node = None;
        self.mouse_pos = None;
    }

    /// Toggle picker mode
    pub fn toggle(&mut self) {
        if self.is_active() {
            self.disable();
        } else {
            self.enable();
        }
    }

    /// Get hovered node ID
    pub fn hovered_node(&self) -> Option<usize> {
        self.hovered_node
    }

    /// Set hovered node
    pub fn set_hovered(&mut self, node_id: Option<usize>) {
        self.hovered_node = node_id;
        if node_id.is_some() {
            self.mode = PickerMode::Hovering;
        } else if self.mode == PickerMode::Hovering {
            self.mode = PickerMode::Active;
        }
    }

    /// Update mouse position
    pub fn set_mouse_pos(&mut self, x: u16, y: u16) {
        self.mouse_pos = Some((x, y));
    }

    /// Get mouse position
    pub fn mouse_pos(&self) -> Option<(u16, u16)> {
        self.mouse_pos
    }

    /// Find node at position
    pub fn find_node_at(
        &self,
        x: u16,
        y: u16,
        nodes: &HashMap<usize, WidgetNode>,
    ) -> Option<usize> {
        // Find the deepest (most specific) node containing this point
        let mut best_match: Option<(usize, usize)> = None; // (node_id, depth)

        for (id, node) in nodes {
            if node.rect.contains(x, y) {
                let depth = node.depth(nodes);
                if best_match.is_none_or(|(_, d)| depth > d) {
                    best_match = Some((*id, depth));
                }
            }
        }

        best_match.map(|(id, _)| id)
    }

    /// Get highlight rect for rendering
    pub fn highlight_rect(&self, nodes: &HashMap<usize, WidgetNode>) -> Option<Rect> {
        self.hovered_node
            .and_then(|id| nodes.get(&id).map(|n| n.rect))
    }

    /// Get tooltip text for hovered node
    pub fn tooltip_text(&self, nodes: &HashMap<usize, WidgetNode>) -> Option<String> {
        if !self.show_tooltip {
            return None;
        }

        self.hovered_node.and_then(|id| {
            nodes.get(&id).map(|node| {
                let mut text = node.type_name.clone();
                if let Some(ref widget_id) = node.widget_id {
                    text.push_str(&format!("#{}", widget_id));
                }
                text
            })
        })
    }

    /// Render picker overlay (highlight and tooltip)
    pub fn render_overlay(
        &self,
        buffer: &mut Buffer,
        _area: Rect,
        nodes: &HashMap<usize, WidgetNode>,
    ) {
        if !self.is_active() {
            return;
        }

        // Draw highlight box around hovered component
        if let Some(rect) = self.highlight_rect(nodes) {
            self.draw_highlight(buffer, rect);
        }

        // Draw tooltip
        if let Some(tooltip) = self.tooltip_text(nodes) {
            if let Some((mx, my)) = self.mouse_pos {
                self.draw_tooltip(buffer, mx, my, &tooltip);
            }
        }
    }

    fn draw_highlight(&self, buffer: &mut Buffer, rect: Rect) {
        // Top and bottom borders
        for x in rect.x..rect.x.saturating_add(rect.width) {
            if let Some(cell) = buffer.get_mut(x, rect.y) {
                cell.bg = Some(self.highlight_color);
            }
            if let Some(cell) =
                buffer.get_mut(x, rect.y.saturating_add(rect.height).saturating_sub(1))
            {
                cell.bg = Some(self.highlight_color);
            }
        }

        // Left and right borders
        for y in rect.y..rect.y.saturating_add(rect.height) {
            if let Some(cell) = buffer.get_mut(rect.x, y) {
                cell.bg = Some(self.highlight_color);
            }
            if let Some(cell) =
                buffer.get_mut(rect.x.saturating_add(rect.width).saturating_sub(1), y)
            {
                cell.bg = Some(self.highlight_color);
            }
        }
    }

    fn draw_tooltip(&self, buffer: &mut Buffer, x: u16, y: u16, text: &str) {
        // Position tooltip below and to the right of cursor
        let tooltip_x = x.saturating_add(2);
        let tooltip_y = y.saturating_add(1);

        // Draw tooltip background
        let padding = 1u16;
        let width = text.len() as u16 + padding * 2;

        for dx in 0..width {
            if let Some(cell) = buffer.get_mut(tooltip_x + dx, tooltip_y) {
                cell.symbol = ' ';
                cell.bg = Some(Color::rgb(40, 40, 50));
            }
        }

        // Draw tooltip text
        for (i, ch) in text.chars().enumerate() {
            if let Some(cell) = buffer.get_mut(tooltip_x + padding + i as u16, tooltip_y) {
                cell.symbol = ch;
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(Color::rgb(40, 40, 50));
            }
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
