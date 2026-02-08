//! Component picker for click-to-inspect functionality

use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use std::collections::HashMap;

use super::types::{PickerMode, WidgetNode};

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
        let width = crate::utils::unicode::display_width(text) as u16 + padding * 2;

        for dx in 0..width {
            if let Some(cell) = buffer.get_mut(tooltip_x + dx, tooltip_y) {
                cell.symbol = ' ';
                cell.bg = Some(Color::rgb(40, 40, 50));
            }
        }

        // Draw tooltip text (wide-char safe)
        let _ = buffer.put_str_styled(
            tooltip_x + padding,
            tooltip_y,
            text,
            Some(Color::WHITE),
            Some(Color::rgb(40, 40, 50)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use std::collections::HashMap;

    #[test]
    fn test_component_picker_new() {
        let mut picker = ComponentPicker::new();
        assert_eq!(picker.mode(), PickerMode::Disabled);
        assert!(!picker.is_active());
        assert_eq!(picker.hovered_node(), None);
        assert_eq!(picker.mouse_pos(), None);
    }

    #[test]
    fn test_component_picker_highlight_color() {
        let color = Color::rgb(255, 0, 0);
        let picker = ComponentPicker::new().highlight_color(color);
        // Just verify it doesn't panic - the highlight_color is private
        let _ = picker;
    }

    #[test]
    fn test_component_picker_show_tooltip() {
        let picker = ComponentPicker::new().show_tooltip(false);
        // Just verify it doesn't panic - show_tooltip is private
        let _ = picker;
    }

    #[test]
    fn test_component_picker_enable() {
        let mut picker = ComponentPicker::new();
        picker.enable();
        assert!(picker.is_active());
        assert_eq!(picker.mode(), PickerMode::Active);
    }

    #[test]
    fn test_component_picker_disable() {
        let mut picker = ComponentPicker::new();
        picker.enable();
        picker.disable();
        assert!(!picker.is_active());
        assert_eq!(picker.mode(), PickerMode::Disabled);
        assert_eq!(picker.hovered_node(), None);
        assert_eq!(picker.mouse_pos(), None);
    }

    #[test]
    fn test_component_picker_toggle() {
        let mut picker = ComponentPicker::new();
        // Toggle from disabled to active
        picker.toggle();
        assert!(picker.is_active());
        // Toggle from active to disabled
        picker.toggle();
        assert!(!picker.is_active());
    }

    #[test]
    fn test_component_picker_set_hovered() {
        let mut picker = ComponentPicker::new();
        picker.set_hovered(Some(5));
        assert_eq!(picker.hovered_node(), Some(5));
        assert_eq!(picker.mode(), PickerMode::Hovering);

        picker.set_hovered(None);
        assert_eq!(picker.hovered_node(), None);
        assert_eq!(picker.mode(), PickerMode::Active);
    }

    #[test]
    fn test_component_picker_set_mouse_pos() {
        let mut picker = ComponentPicker::new();
        picker.set_mouse_pos(10, 20);
        assert_eq!(picker.mouse_pos(), Some((10, 20)));

        picker.set_mouse_pos(5, 15);
        assert_eq!(picker.mouse_pos(), Some((5, 15)));
    }

    #[test]
    fn test_component_picker_find_node_at() {
        use super::super::types::WidgetNode;

        let mut picker = ComponentPicker::new();
        let mut nodes = HashMap::new();

        // Create a node at (5, 5) with size 10x10
        let node1 = WidgetNode::new(1, "Button")
            .widget_id("btn1")
            .rect(Rect::new(5, 5, 10, 10));

        nodes.insert(1, node1);

        // Find node at a position inside the rect
        let found = picker.find_node_at(10, 10, &nodes);
        assert_eq!(found, Some(1));

        // Find node at a position outside the rect
        let found = picker.find_node_at(0, 0, &nodes);
        assert_eq!(found, None);
    }

    #[test]
    fn test_component_picker_highlight_rect() {
        use super::super::types::WidgetNode;

        let mut picker = ComponentPicker::new();
        let mut nodes = HashMap::new();

        let node = WidgetNode::new(1, "Button")
            .widget_id("btn1")
            .rect(Rect::new(5, 5, 10, 10));

        nodes.insert(1, node);

        picker.set_hovered(Some(1));
        let rect = picker.highlight_rect(&nodes);
        assert_eq!(rect, Some(Rect::new(5, 5, 10, 10)));

        picker.set_hovered(None);
        let rect = picker.highlight_rect(&nodes);
        assert_eq!(rect, None);
    }

    #[test]
    fn test_component_picker_tooltip_text() {
        use super::super::types::WidgetNode;

        let mut picker = ComponentPicker::new();
        let mut nodes = HashMap::new();

        let node = WidgetNode::new(1, "Button")
            .widget_id("btn1")
            .rect(Rect::new(0, 0, 10, 10));

        nodes.insert(1, node);

        picker.set_hovered(Some(1));
        let tooltip = picker.tooltip_text(&nodes);
        assert_eq!(tooltip, Some("Button#btn1".to_string()));

        picker.set_hovered(None);
        let tooltip = picker.tooltip_text(&nodes);
        assert_eq!(tooltip, None);
    }

    #[test]
    fn test_component_picker_tooltip_text_no_widget_id() {
        use super::super::types::WidgetNode;

        let mut picker = ComponentPicker::new();
        let mut nodes = HashMap::new();

        let node = WidgetNode::new(1, "Text").rect(Rect::new(0, 0, 10, 10));

        nodes.insert(1, node);

        picker.set_hovered(Some(1));
        let tooltip = picker.tooltip_text(&nodes);
        assert_eq!(tooltip, Some("Text".to_string()));
    }

    #[test]
    fn test_component_picker_tooltip_hidden() {
        let picker = ComponentPicker::new().show_tooltip(false);
        // With tooltip disabled, should return None
        let nodes = HashMap::new();
        let tooltip = picker.tooltip_text(&nodes);
        assert_eq!(tooltip, None);
    }

    #[test]
    fn test_component_picker_render_overlay_disabled() {
        let mut picker = ComponentPicker::new();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let nodes = HashMap::new();

        // Should not panic when picker is disabled
        picker.render_overlay(&mut buffer, area, &nodes);
    }

    #[test]
    fn test_component_picker_render_overlay_active() {
        let mut picker = ComponentPicker::new();
        picker.enable();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let nodes = HashMap::new();

        // Should not panic when picker is active
        picker.render_overlay(&mut buffer, area, &nodes);
    }

    #[test]
    fn test_component_picker_render_overlay_with_hovered() {
        use super::super::types::WidgetNode;

        let mut picker = ComponentPicker::new();
        picker.enable();
        picker.set_mouse_pos(10, 10);

        let mut nodes = HashMap::new();
        let node = WidgetNode::new(1, "Button")
            .widget_id("btn1")
            .rect(Rect::new(5, 5, 10, 10));
        nodes.insert(1, node);

        picker.set_hovered(Some(1));

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);

        // Should not panic
        picker.render_overlay(&mut buffer, area, &nodes);
    }

    // =========================================================================
    // Additional picker tests
    // =========================================================================

    #[test]
    fn test_component_picker_default() {
        let picker = ComponentPicker::default();
        assert_eq!(picker.mode(), PickerMode::Disabled);
    }

    #[test]
    fn test_component_picker_mode() {
        let picker = ComponentPicker::new();
        assert_eq!(picker.mode(), PickerMode::Disabled);
    }

    #[test]
    fn test_component_picker_is_active() {
        let picker = ComponentPicker::new();
        assert!(!picker.is_active());
    }

    #[test]
    fn test_component_picker_find_node_at_empty() {
        let picker = ComponentPicker::new();
        let nodes = HashMap::new();
        let found = picker.find_node_at(10, 10, &nodes);
        assert!(found.is_none());
    }

    #[test]
    fn test_component_picker_find_node_at_multiple() {
        let mut nodes = HashMap::new();

        // Create overlapping nodes - parent contains child
        let mut child = WidgetNode::new(2, "Button").rect(Rect::new(10, 10, 20, 20));
        child.parent = Some(1);
        let parent = WidgetNode::new(1, "Container").rect(Rect::new(0, 0, 50, 50));

        nodes.insert(1, parent);
        nodes.insert(2, child);

        let picker = ComponentPicker::new();
        // Should find the child (deeper node)
        let found = picker.find_node_at(15, 15, &nodes);
        assert_eq!(found, Some(2));
    }

    #[test]
    fn test_component_picker_find_node_at_edge() {
        let mut nodes = HashMap::new();
        let node = WidgetNode::new(1, "Button").rect(Rect::new(10, 10, 10, 10));
        nodes.insert(1, node);

        let picker = ComponentPicker::new();
        // Test edge cases
        assert_eq!(picker.find_node_at(9, 15, &nodes), None); // Just outside
        assert_eq!(picker.find_node_at(10, 10, &nodes), Some(1)); // Top-left corner
        assert_eq!(picker.find_node_at(19, 19, &nodes), Some(1)); // Bottom-right corner
        assert_eq!(picker.find_node_at(20, 20, &nodes), None); // Just outside
    }

    #[test]
    fn test_component_picker_set_hovered_mode_transition() {
        let mut picker = ComponentPicker::new();
        picker.enable();

        // Setting hovered when Active transitions to Hovering
        picker.set_hovered(Some(1));
        assert_eq!(picker.mode(), PickerMode::Hovering);

        // Clearing hovered when Hovering returns to Active
        picker.set_hovered(None);
        assert_eq!(picker.mode(), PickerMode::Active);
    }

    #[test]
    fn test_component_picker_set_hovered_disabled_changes_to_hovering() {
        let mut picker = ComponentPicker::new();

        // Setting hovered when Disabled changes mode to Hovering
        picker.set_hovered(Some(1));
        assert_eq!(picker.mode(), PickerMode::Hovering);
    }

    #[test]
    fn test_component_picker_render_overlay_no_mouse() {
        use super::super::types::WidgetNode;

        let mut picker = ComponentPicker::new();
        picker.enable();

        let mut nodes = HashMap::new();
        let node = WidgetNode::new(1, "Button")
            .widget_id("btn1")
            .rect(Rect::new(5, 5, 10, 10));
        nodes.insert(1, node);

        picker.set_hovered(Some(1));

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);

        // No mouse position - should still render highlight
        picker.render_overlay(&mut buffer, area, &nodes);
    }

    #[test]
    fn test_component_picker_multiple_enable() {
        let mut picker = ComponentPicker::new();
        picker.enable();
        picker.enable();
        assert!(picker.is_active());
    }

    #[test]
    fn test_component_picker_multiple_disable() {
        let mut picker = ComponentPicker::new();
        picker.enable();
        picker.disable();
        picker.disable();
        assert!(!picker.is_active());
    }

    #[test]
    fn test_component_picker_disable_clears_state() {
        let mut picker = ComponentPicker::new();
        picker.enable();
        picker.set_hovered(Some(1));
        picker.set_mouse_pos(10, 20);

        picker.disable();
        assert_eq!(picker.hovered_node(), None);
        assert_eq!(picker.mouse_pos(), None);
    }

    #[test]
    fn test_component_picker_highlight_rect_no_hover() {
        let picker = ComponentPicker::new();
        let nodes = HashMap::new();
        let rect = picker.highlight_rect(&nodes);
        assert!(rect.is_none());
    }

    #[test]
    fn test_component_picker_highlight_rect_invalid_node() {
        let mut picker = ComponentPicker::new();
        picker.set_hovered(Some(999));
        let nodes = HashMap::new();
        let rect = picker.highlight_rect(&nodes);
        assert!(rect.is_none());
    }

    #[test]
    fn test_component_picker_tooltip_text_no_hover() {
        let picker = ComponentPicker::new();
        let nodes = HashMap::new();
        let tooltip = picker.tooltip_text(&nodes);
        assert!(tooltip.is_none());
    }

    #[test]
    fn test_component_picker_tooltip_text_empty_nodes() {
        let mut picker = ComponentPicker::new();
        picker.set_hovered(Some(1));
        let nodes = HashMap::new();
        let tooltip = picker.tooltip_text(&nodes);
        assert!(tooltip.is_none());
    }

    #[test]
    fn test_component_picker_tooltip_with_show_tooltip() {
        use super::super::types::WidgetNode;

        let mut picker = ComponentPicker::new().show_tooltip(true);
        picker.enable();

        let mut nodes = HashMap::new();
        let node = WidgetNode::new(1, "Button")
            .widget_id("test")
            .rect(Rect::new(0, 0, 10, 10));
        nodes.insert(1, node);

        picker.set_hovered(Some(1));
        picker.set_mouse_pos(5, 5);

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);

        // Should render with tooltip
        picker.render_overlay(&mut buffer, area, &nodes);
    }

    #[test]
    fn test_component_picker_clone_debug() {
        // ComponentPicker derives Debug
        let picker = ComponentPicker::new();
        let debug_str = format!("{:?}", picker);
        assert!(debug_str.contains("ComponentPicker"));
    }

    #[test]
    fn test_component_picker_mode_cycle() {
        let mut picker = ComponentPicker::new();

        // Disabled -> Active
        picker.enable();
        assert_eq!(picker.mode(), PickerMode::Active);

        // Active -> Hovering (via set_hovered)
        picker.set_hovered(Some(1));
        assert_eq!(picker.mode(), PickerMode::Hovering);

        // Hovering -> Active (via set_hovered None)
        picker.set_hovered(None);
        assert_eq!(picker.mode(), PickerMode::Active);

        // Active -> Disabled
        picker.disable();
        assert_eq!(picker.mode(), PickerMode::Disabled);
    }
}
