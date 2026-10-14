//! Context menu widget

mod input;
mod view;

use super::types::MenuItem;
use crate::style::Color;
use crate::widget::theme::DARK_BG;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Context menu (popup menu)
pub struct ContextMenu {
    /// Menu items
    items: Vec<MenuItem>,
    /// Position
    x: u16,
    y: u16,
    /// Selected item
    selected: usize,
    /// Visible
    visible: bool,
    /// Colors
    bg: Color,
    fg: Option<Color>,
    selected_bg: Color,
    selected_fg: Color,
    /// Widget properties
    props: WidgetProps,
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextMenu {
    /// Create a new context menu
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            x: 0,
            y: 0,
            selected: 0,
            visible: false,
            bg: DARK_BG,
            fg: None,
            selected_bg: Color::rgb(60, 100, 180),
            selected_fg: Color::WHITE,
            props: WidgetProps::new(),
        }
    }

    /// Add an item
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add items
    pub fn items(mut self, items: Vec<MenuItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// Show at position
    pub fn show(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
        self.visible = true;
        self.selected = 0;
    }

    /// Hide menu
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

impl_styled_view!(ContextMenu);
impl_props_builders!(ContextMenu);

#[cfg(test)]
mod tests;
