//! Context menu widget for right-click and action menus
//!
//! Displays a popup menu with selectable items, submenus, and keyboard navigation.

use super::traits::{View, RenderContext, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::event::{KeyEvent, Key};
use crate::{impl_styled_view, impl_props_builders};

/// Menu item separator
pub const SEPARATOR: &str = "---";

/// Menu item type
#[derive(Clone, Debug)]
pub enum MenuItem {
    /// Regular menu item
    Item {
        /// Item label
        label: String,
        /// Keyboard shortcut hint
        shortcut: Option<String>,
        /// Is item enabled
        enabled: bool,
        /// Is item checked (for toggle items)
        checked: Option<bool>,
        /// Item ID for event handling
        id: String,
    },
    /// Submenu
    Submenu {
        /// Submenu label
        label: String,
        /// Submenu items
        items: Vec<MenuItem>,
        /// Is submenu enabled
        enabled: bool,
    },
    /// Separator line
    Separator,
}

impl MenuItem {
    /// Create a new menu item
    pub fn new(label: impl Into<String>, id: impl Into<String>) -> Self {
        MenuItem::Item {
            label: label.into(),
            shortcut: None,
            enabled: true,
            checked: None,
            id: id.into(),
        }
    }

    /// Create a menu item with shortcut
    pub fn with_shortcut(label: impl Into<String>, id: impl Into<String>, shortcut: impl Into<String>) -> Self {
        MenuItem::Item {
            label: label.into(),
            shortcut: Some(shortcut.into()),
            enabled: true,
            checked: None,
            id: id.into(),
        }
    }

    /// Create a toggle menu item
    pub fn toggle(label: impl Into<String>, id: impl Into<String>, checked: bool) -> Self {
        MenuItem::Item {
            label: label.into(),
            shortcut: None,
            enabled: true,
            checked: Some(checked),
            id: id.into(),
        }
    }

    /// Create a submenu
    pub fn submenu(label: impl Into<String>, items: Vec<MenuItem>) -> Self {
        MenuItem::Submenu {
            label: label.into(),
            items,
            enabled: true,
        }
    }

    /// Create a separator
    pub fn separator() -> Self {
        MenuItem::Separator
    }

    /// Set enabled state
    pub fn enabled(mut self, enabled: bool) -> Self {
        match &mut self {
            MenuItem::Item { enabled: e, .. } => *e = enabled,
            MenuItem::Submenu { enabled: e, .. } => *e = enabled,
            _ => {}
        }
        self
    }

    /// Check if item is selectable
    pub fn is_selectable(&self) -> bool {
        match self {
            MenuItem::Item { enabled, .. } => *enabled,
            MenuItem::Submenu { enabled, .. } => *enabled,
            MenuItem::Separator => false,
        }
    }

    /// Get label
    pub fn label(&self) -> Option<&str> {
        match self {
            MenuItem::Item { label, .. } => Some(label),
            MenuItem::Submenu { label, .. } => Some(label),
            MenuItem::Separator => None,
        }
    }

    /// Get ID
    pub fn id(&self) -> Option<&str> {
        match self {
            MenuItem::Item { id, .. } => Some(id),
            _ => None,
        }
    }
}

/// Context menu widget
pub struct ContextMenu {
    /// Menu items
    items: Vec<MenuItem>,
    /// Position (x, y)
    position: (u16, u16),
    /// Currently selected index
    selected: usize,
    /// Is menu visible
    visible: bool,
    /// Background color
    bg: Color,
    /// Foreground color
    fg: Color,
    /// Selected background
    selected_bg: Color,
    /// Selected foreground
    selected_fg: Color,
    /// Disabled color
    disabled_fg: Color,
    /// Border color
    border_fg: Color,
    /// Minimum width
    min_width: u16,
    /// Show border
    show_border: bool,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl ContextMenu {
    /// Create a new context menu
    pub fn new(items: Vec<MenuItem>) -> Self {
        let mut menu = Self {
            items,
            position: (0, 0),
            selected: 0,
            visible: false,
            bg: Color::rgb(40, 40, 40),
            fg: Color::WHITE,
            selected_bg: Color::rgb(60, 100, 180),
            selected_fg: Color::WHITE,
            disabled_fg: Color::rgb(100, 100, 100),
            border_fg: Color::rgb(80, 80, 80),
            min_width: 15,
            show_border: true,
            props: WidgetProps::new(),
        };
        menu.select_first_selectable();
        menu
    }

    /// Set menu items
    pub fn items(mut self, items: Vec<MenuItem>) -> Self {
        self.items = items;
        self.select_first_selectable();
        self
    }

    /// Set position
    pub fn position(mut self, x: u16, y: u16) -> Self {
        self.position = (x, y);
        self
    }

    /// Set visibility
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Set selected style
    pub fn selected_style(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = fg;
        self.selected_bg = bg;
        self
    }

    /// Set minimum width
    pub fn min_width(mut self, width: u16) -> Self {
        self.min_width = width;
        self
    }

    /// Enable/disable border
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Show the menu at position
    pub fn show(&mut self, x: u16, y: u16) {
        self.position = (x, y);
        self.visible = true;
        self.select_first_selectable();
    }

    /// Hide the menu
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get selected item
    pub fn selected_item(&self) -> Option<&MenuItem> {
        self.items.get(self.selected)
    }

    /// Get selected item ID
    pub fn selected_id(&self) -> Option<&str> {
        self.selected_item().and_then(|item| item.id())
    }

    /// Select next item
    pub fn select_next(&mut self) {
        let len = self.items.len();
        if len == 0 {
            return;
        }

        let start = self.selected;
        loop {
            self.selected = (self.selected + 1) % len;
            if self.items[self.selected].is_selectable() || self.selected == start {
                break;
            }
        }
    }

    /// Select previous item
    pub fn select_prev(&mut self) {
        let len = self.items.len();
        if len == 0 {
            return;
        }

        let start = self.selected;
        loop {
            self.selected = if self.selected == 0 { len - 1 } else { self.selected - 1 };
            if self.items[self.selected].is_selectable() || self.selected == start {
                break;
            }
        }
    }

    /// Select first selectable item
    fn select_first_selectable(&mut self) {
        for (i, item) in self.items.iter().enumerate() {
            if item.is_selectable() {
                self.selected = i;
                return;
            }
        }
        self.selected = 0;
    }

    /// Calculate menu dimensions
    fn dimensions(&self) -> (u16, u16) {
        let mut width = self.min_width as usize;
        for item in &self.items {
            let item_width = match item {
                MenuItem::Item { label, shortcut, .. } => {
                    let shortcut_len = shortcut.as_ref().map(|s| s.len() + 2).unwrap_or(0);
                    label.len() + shortcut_len + 4 // padding + checkbox space
                }
                MenuItem::Submenu { label, .. } => label.len() + 4, // padding + arrow
                MenuItem::Separator => 3,
            };
            width = width.max(item_width);
        }

        let height = self.items.len() + if self.show_border { 2 } else { 0 };
        (width as u16 + if self.show_border { 2 } else { 0 }, height as u16)
    }

    /// Handle key event
    pub fn handle_key(&mut self, key: KeyEvent) -> Option<String> {
        if !self.visible {
            return None;
        }

        match key.key {
            Key::Up => {
                self.select_prev();
                None
            }
            Key::Down => {
                self.select_next();
                None
            }
            Key::Enter => {
                // Return selected item ID and hide menu
                let id = self.selected_id().map(|s| s.to_string());
                if id.is_some() {
                    self.hide();
                }
                id
            }
            Key::Escape => {
                self.hide();
                None
            }
            _ => None,
        }
    }
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl View for ContextMenu {
    crate::impl_view_meta!("ContextMenu");

    fn render(&self, ctx: &mut RenderContext) {
        if !self.visible || self.items.is_empty() {
            return;
        }

        let (width, height) = self.dimensions();
        let (pos_x, pos_y) = self.position;

        // Clamp to screen bounds
        let area = ctx.area;
        let x = pos_x.min(area.width.saturating_sub(width));
        let y = pos_y.min(area.height.saturating_sub(height));

        // Draw background
        for dy in 0..height {
            for dx in 0..width {
                ctx.buffer.set(x + dx, y + dy, Cell::new(' ').bg(self.bg));
            }
        }

        // Draw border if enabled
        let content_x;
        let content_y;
        let content_width;

        if self.show_border {
            // Top border
            ctx.buffer.set(x, y, Cell::new('╭').fg(self.border_fg).bg(self.bg));
            for dx in 1..width - 1 {
                ctx.buffer.set(x + dx, y, Cell::new('─').fg(self.border_fg).bg(self.bg));
            }
            ctx.buffer.set(x + width - 1, y, Cell::new('╮').fg(self.border_fg).bg(self.bg));

            // Side borders
            for dy in 1..height - 1 {
                ctx.buffer.set(x, y + dy, Cell::new('│').fg(self.border_fg).bg(self.bg));
                ctx.buffer.set(x + width - 1, y + dy, Cell::new('│').fg(self.border_fg).bg(self.bg));
            }

            // Bottom border
            ctx.buffer.set(x, y + height - 1, Cell::new('╰').fg(self.border_fg).bg(self.bg));
            for dx in 1..width - 1 {
                ctx.buffer.set(x + dx, y + height - 1, Cell::new('─').fg(self.border_fg).bg(self.bg));
            }
            ctx.buffer.set(x + width - 1, y + height - 1, Cell::new('╯').fg(self.border_fg).bg(self.bg));

            content_x = x + 1;
            content_y = y + 1;
            content_width = width - 2;
        } else {
            content_x = x;
            content_y = y;
            content_width = width;
        }

        // Draw items
        for (i, item) in self.items.iter().enumerate() {
            let item_y = content_y + i as u16;
            let is_selected = i == self.selected;

            match item {
                MenuItem::Item { label, shortcut, enabled, checked, .. } => {
                    let (fg, bg) = if is_selected && *enabled {
                        (self.selected_fg, self.selected_bg)
                    } else if *enabled {
                        (self.fg, self.bg)
                    } else {
                        (self.disabled_fg, self.bg)
                    };

                    // Fill background for selected item
                    for dx in 0..content_width {
                        ctx.buffer.set(content_x + dx, item_y, Cell::new(' ').bg(bg));
                    }

                    let mut cx = content_x + 1;

                    // Checkbox if toggle item
                    if let Some(is_checked) = checked {
                        let check_char = if *is_checked { '✓' } else { ' ' };
                        ctx.buffer.set(cx, item_y, Cell::new(check_char).fg(fg).bg(bg));
                        cx += 2;
                    }

                    // Label
                    for ch in label.chars() {
                        if cx >= content_x + content_width - 1 {
                            break;
                        }
                        ctx.buffer.set(cx, item_y, Cell::new(ch).fg(fg).bg(bg));
                        cx += 1;
                    }

                    // Shortcut (right-aligned)
                    if let Some(shortcut) = shortcut {
                        let shortcut_x = content_x + content_width - shortcut.len() as u16 - 1;
                        for (j, ch) in shortcut.chars().enumerate() {
                            let sx = shortcut_x + j as u16;
                            if sx > cx {
                                ctx.buffer.set(sx, item_y, Cell::new(ch).fg(self.disabled_fg).bg(bg));
                            }
                        }
                    }
                }
                MenuItem::Submenu { label, enabled, .. } => {
                    let (fg, bg) = if is_selected && *enabled {
                        (self.selected_fg, self.selected_bg)
                    } else if *enabled {
                        (self.fg, self.bg)
                    } else {
                        (self.disabled_fg, self.bg)
                    };

                    // Fill background
                    for dx in 0..content_width {
                        ctx.buffer.set(content_x + dx, item_y, Cell::new(' ').bg(bg));
                    }

                    // Label
                    let mut cx = content_x + 1;
                    for ch in label.chars() {
                        if cx >= content_x + content_width - 2 {
                            break;
                        }
                        ctx.buffer.set(cx, item_y, Cell::new(ch).fg(fg).bg(bg));
                        cx += 1;
                    }

                    // Submenu arrow
                    ctx.buffer.set(content_x + content_width - 2, item_y, Cell::new('▶').fg(fg).bg(bg));
                }
                MenuItem::Separator => {
                    for dx in 0..content_width {
                        let ch = if dx == 0 || dx == content_width - 1 { '─' } else { '─' };
                        ctx.buffer.set(content_x + dx, item_y, Cell::new(ch).fg(self.border_fg).bg(self.bg));
                    }
                }
            }
        }
    }
}

impl_styled_view!(ContextMenu);
impl_props_builders!(ContextMenu);

/// Helper function to create a context menu
pub fn context_menu(items: Vec<MenuItem>) -> ContextMenu {
    ContextMenu::new(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::layout::Rect;

    #[test]
    fn test_menu_item_new() {
        let item = MenuItem::new("Open", "open");
        assert_eq!(item.label(), Some("Open"));
        assert_eq!(item.id(), Some("open"));
        assert!(item.is_selectable());
    }

    #[test]
    fn test_menu_item_with_shortcut() {
        let item = MenuItem::with_shortcut("Save", "save", "Ctrl+S");
        assert_eq!(item.label(), Some("Save"));
    }

    #[test]
    fn test_menu_item_toggle() {
        let item = MenuItem::toggle("Auto Save", "autosave", true);
        if let MenuItem::Item { checked, .. } = item {
            assert_eq!(checked, Some(true));
        }
    }

    #[test]
    fn test_menu_item_separator() {
        let sep = MenuItem::separator();
        assert!(!sep.is_selectable());
        assert_eq!(sep.label(), None);
    }

    #[test]
    fn test_context_menu_new() {
        let items = vec![
            MenuItem::new("Cut", "cut"),
            MenuItem::new("Copy", "copy"),
            MenuItem::separator(),
            MenuItem::new("Paste", "paste"),
        ];
        let menu = ContextMenu::new(items);
        assert_eq!(menu.selected, 0);
    }

    #[test]
    fn test_context_menu_navigation() {
        let items = vec![
            MenuItem::new("One", "one"),
            MenuItem::separator(),
            MenuItem::new("Two", "two"),
        ];
        let mut menu = ContextMenu::new(items);

        assert_eq!(menu.selected, 0);
        menu.select_next();
        assert_eq!(menu.selected, 2); // Skips separator
        menu.select_prev();
        assert_eq!(menu.selected, 0);
    }

    #[test]
    fn test_context_menu_visibility() {
        let mut menu = ContextMenu::new(vec![MenuItem::new("Test", "test")]);

        assert!(!menu.is_visible());
        menu.show(10, 5);
        assert!(menu.is_visible());
        assert_eq!(menu.position, (10, 5));

        menu.hide();
        assert!(!menu.is_visible());
    }

    #[test]
    fn test_context_menu_dimensions() {
        let items = vec![
            MenuItem::new("Short", "short"),
            MenuItem::new("A much longer item", "long"),
        ];
        let menu = ContextMenu::new(items).min_width(10);
        let (w, h) = menu.dimensions();

        assert!(w >= 10);
        assert_eq!(h, 4); // 2 items + 2 border
    }

    #[test]
    fn test_context_menu_render() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let items = vec![
            MenuItem::new("Open", "open"),
            MenuItem::new("Save", "save"),
        ];
        let menu = ContextMenu::new(items).position(5, 2).visible(true);
        menu.render(&mut ctx);

        // Check that menu border is rendered
        assert_eq!(buffer.get(5, 2).unwrap().symbol, '╭');
    }

    #[test]
    fn test_context_menu_helper() {
        let menu = context_menu(vec![MenuItem::new("Test", "test")]);
        assert!(!menu.is_visible());
    }
}
