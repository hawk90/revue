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
