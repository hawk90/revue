//! Menu bar widget

mod nav;
mod view;

use super::types::Menu;
use crate::style::Color;
use crate::utils::ScrollState;
use crate::widget::theme::{DARK_BG, LIGHT_GRAY};
use crate::widget::traits::{WidgetProps, DISABLED_FG};
use crate::{impl_props_builders, impl_styled_view};

/// Menu bar widget
pub struct MenuBar {
    /// Menus
    menus: Vec<Menu>,
    /// Currently selected menu index
    selected_menu: usize,
    /// Currently selected item index (if menu is open)
    selected_item: Option<usize>,
    /// Where the item rows are scrolled to
    scroll: ScrollState,
    /// Is a menu currently open
    open: bool,
    /// Colors
    bg: Color,
    fg: Option<Color>,
    selected_bg: Color,
    selected_fg: Color,
    disabled_fg: Color,
    shortcut_fg: Color,
    /// Widget properties
    props: WidgetProps,
}

impl Default for MenuBar {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuBar {
    /// Create a new menu bar
    pub fn new() -> Self {
        Self {
            menus: Vec::new(),
            selected_menu: 0,
            selected_item: None,
            scroll: ScrollState::new(),
            open: false,
            bg: DARK_BG,
            fg: None,
            selected_bg: Color::rgb(60, 100, 180),
            selected_fg: Color::WHITE,
            disabled_fg: DISABLED_FG,
            shortcut_fg: LIGHT_GRAY,
            props: WidgetProps::new(),
        }
    }

    /// Add a menu
    pub fn menu(mut self, menu: Menu) -> Self {
        self.menus.push(menu);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Check if menu is open
    pub fn is_open(&self) -> bool {
        self.open
    }
}

impl_styled_view!(MenuBar);
impl_props_builders!(MenuBar);

#[cfg(test)]
mod tests;
