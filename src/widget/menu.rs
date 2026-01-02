//! Menu bar and context menu widgets
//!
//! Provides horizontal menu bars and dropdown/context menus.

use super::traits::{View, RenderContext, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::event::Key;
use crate::{impl_styled_view, impl_props_builders};

/// Menu item action
pub type MenuAction = Box<dyn Fn() + 'static>;

/// Menu item
pub struct MenuItem {
    /// Item label
    pub label: String,
    /// Keyboard shortcut display
    pub shortcut: Option<String>,
    /// Item is disabled
    pub disabled: bool,
    /// Item is checked (for toggle items)
    pub checked: Option<bool>,
    /// Submenu items
    pub submenu: Vec<MenuItem>,
    /// Is a separator
    pub separator: bool,
    /// Action callback
    action: Option<MenuAction>,
}

impl MenuItem {
    /// Create a new menu item
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            shortcut: None,
            disabled: false,
            checked: None,
            submenu: Vec::new(),
            separator: false,
            action: None,
        }
    }

    /// Create a separator
    pub fn separator() -> Self {
        Self {
            label: String::new(),
            shortcut: None,
            disabled: false,
            checked: None,
            submenu: Vec::new(),
            separator: true,
            action: None,
        }
    }

    /// Set keyboard shortcut display
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set checked state
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    /// Add submenu items
    pub fn submenu(mut self, items: Vec<MenuItem>) -> Self {
        self.submenu = items;
        self
    }

    /// Set action callback
    pub fn on_select<F: Fn() + 'static>(mut self, action: F) -> Self {
        self.action = Some(Box::new(action));
        self
    }

    /// Execute action if available
    pub fn execute(&self) {
        if let Some(ref action) = self.action {
            if !self.disabled {
                action();
            }
        }
    }

    /// Has submenu
    pub fn has_submenu(&self) -> bool {
        !self.submenu.is_empty()
    }
}

/// Menu (top-level or submenu)
pub struct Menu {
    /// Menu title
    pub title: String,
    /// Menu items
    pub items: Vec<MenuItem>,
}

impl Menu {
    /// Create a new menu
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
        }
    }

    /// Add an item
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add multiple items
    pub fn items(mut self, items: Vec<MenuItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// Add a separator
    pub fn separator(mut self) -> Self {
        self.items.push(MenuItem::separator());
        self
    }
}

/// Menu bar widget
pub struct MenuBar {
    /// Menus
    menus: Vec<Menu>,
    /// Currently selected menu index
    selected_menu: usize,
    /// Currently selected item index (if menu is open)
    selected_item: Option<usize>,
    /// Is a menu currently open
    open: bool,
    /// Colors
    bg: Color,
    fg: Color,
    selected_bg: Color,
    selected_fg: Color,
    disabled_fg: Color,
    shortcut_fg: Color,
    /// Widget properties
    props: WidgetProps,
}

impl MenuBar {
    /// Create a new menu bar
    pub fn new() -> Self {
        Self {
            menus: Vec::new(),
            selected_menu: 0,
            selected_item: None,
            open: false,
            bg: Color::rgb(40, 40, 40),
            fg: Color::WHITE,
            selected_bg: Color::rgb(60, 100, 180),
            selected_fg: Color::WHITE,
            disabled_fg: Color::rgb(100, 100, 100),
            shortcut_fg: Color::rgb(150, 150, 150),
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
        self.fg = color;
        self
    }

    /// Open menu at index
    pub fn open_menu(&mut self, index: usize) {
        if index < self.menus.len() {
            self.selected_menu = index;
            self.open = true;
            self.selected_item = if self.menus[index].items.is_empty() {
                None
            } else {
                Some(0)
            };
        }
    }

    /// Close menu
    pub fn close(&mut self) {
        self.open = false;
        self.selected_item = None;
    }

    /// Toggle menu open state
    pub fn toggle(&mut self) {
        if self.open {
            self.close();
        } else {
            self.open_menu(self.selected_menu);
        }
    }

    /// Select next menu
    pub fn next_menu(&mut self) {
        if !self.menus.is_empty() {
            self.selected_menu = (self.selected_menu + 1) % self.menus.len();
            if self.open {
                self.open_menu(self.selected_menu);
            }
        }
    }

    /// Select previous menu
    pub fn prev_menu(&mut self) {
        if !self.menus.is_empty() {
            self.selected_menu = self.selected_menu
                .checked_sub(1)
                .unwrap_or(self.menus.len() - 1);
            if self.open {
                self.open_menu(self.selected_menu);
            }
        }
    }

    /// Select next item
    pub fn next_item(&mut self) {
        if let Some(menu) = self.menus.get(self.selected_menu) {
            if !menu.items.is_empty() {
                let current = self.selected_item.unwrap_or(0);
                let mut next = (current + 1) % menu.items.len();
                // Skip separators
                while menu.items[next].separator && next != current {
                    next = (next + 1) % menu.items.len();
                }
                self.selected_item = Some(next);
            }
        }
    }

    /// Select previous item
    pub fn prev_item(&mut self) {
        if let Some(menu) = self.menus.get(self.selected_menu) {
            if !menu.items.is_empty() {
                let current = self.selected_item.unwrap_or(0);
                let mut prev = current.checked_sub(1).unwrap_or(menu.items.len() - 1);
                // Skip separators
                while menu.items[prev].separator && prev != current {
                    prev = prev.checked_sub(1).unwrap_or(menu.items.len() - 1);
                }
                self.selected_item = Some(prev);
            }
        }
    }

    /// Execute selected item
    pub fn execute_selected(&mut self) -> bool {
        if let Some(item_idx) = self.selected_item {
            if let Some(menu) = self.menus.get(self.selected_menu) {
                if let Some(item) = menu.items.get(item_idx) {
                    if !item.disabled && !item.separator {
                        item.execute();
                        self.close();
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Left | Key::Char('h') => {
                self.prev_menu();
                true
            }
            Key::Right | Key::Char('l') => {
                self.next_menu();
                true
            }
            Key::Up | Key::Char('k') if self.open => {
                self.prev_item();
                true
            }
            Key::Down | Key::Char('j') if self.open => {
                self.next_item();
                true
            }
            Key::Enter | Key::Char(' ') => {
                if self.open {
                    self.execute_selected()
                } else {
                    self.toggle();
                    true
                }
            }
            Key::Escape if self.open => {
                self.close();
                true
            }
            _ => false,
        }
    }

    /// Check if menu is open
    pub fn is_open(&self) -> bool {
        self.open
    }
}

impl Default for MenuBar {
    fn default() -> Self {
        Self::new()
    }
}

impl View for MenuBar {
    crate::impl_view_meta!("MenuBar");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.height < 1 {
            return;
        }

        // Draw menu bar background
        for x in area.x..area.x + area.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(self.bg);
            ctx.buffer.set(x, area.y, cell);
        }

        // Draw menu titles
        let mut x = area.x;
        for (i, menu) in self.menus.iter().enumerate() {
            let is_selected = i == self.selected_menu;
            let title = format!(" {} ", menu.title);

            for ch in title.chars() {
                if x >= area.x + area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                if is_selected {
                    cell.bg = Some(self.selected_bg);
                    cell.fg = Some(self.selected_fg);
                } else {
                    cell.bg = Some(self.bg);
                    cell.fg = Some(self.fg);
                }
                ctx.buffer.set(x, area.y, cell);
                x += 1;
            }
        }

        // Draw dropdown if open
        if self.open {
            if let Some(menu) = self.menus.get(self.selected_menu) {
                self.render_dropdown(ctx, menu, area.y + 1);
            }
        }
    }
}

impl MenuBar {
    fn render_dropdown(&self, ctx: &mut RenderContext, menu: &Menu, y: u16) {
        if menu.items.is_empty() || y >= ctx.area.y + ctx.area.height {
            return;
        }

        // Calculate dropdown position
        let mut menu_x = ctx.area.x;
        for (i, m) in self.menus.iter().enumerate() {
            if i == self.selected_menu {
                break;
            }
            menu_x += m.title.len() as u16 + 2;
        }

        // Calculate max width
        let max_width = menu.items.iter()
            .filter(|item| !item.separator)
            .map(|item| {
                let shortcut_len = item.shortcut.as_ref().map(|s| s.len() + 2).unwrap_or(0);
                item.label.len() + shortcut_len + 4
            })
            .max()
            .unwrap_or(10) as u16;

        let dropdown_width = max_width.min(ctx.area.width - menu_x);
        let dropdown_height = (menu.items.len() as u16 + 2).min(ctx.area.height - y);

        // Draw border
        for dy in 0..dropdown_height {
            for dx in 0..dropdown_width {
                let px = menu_x + dx;
                let py = y + dy;
                if px >= ctx.area.x + ctx.area.width || py >= ctx.area.y + ctx.area.height {
                    continue;
                }

                let ch = if dy == 0 && dx == 0 {
                    '┌'
                } else if dy == 0 && dx == dropdown_width - 1 {
                    '┐'
                } else if dy == dropdown_height - 1 && dx == 0 {
                    '└'
                } else if dy == dropdown_height - 1 && dx == dropdown_width - 1 {
                    '┘'
                } else if dy == 0 || dy == dropdown_height - 1 {
                    '─'
                } else if dx == 0 || dx == dropdown_width - 1 {
                    '│'
                } else {
                    ' '
                };

                let mut cell = Cell::new(ch);
                cell.bg = Some(self.bg);
                cell.fg = Some(self.fg);
                ctx.buffer.set(px, py, cell);
            }
        }

        // Draw items
        for (i, item) in menu.items.iter().enumerate() {
            let item_y = y + 1 + i as u16;
            if item_y >= y + dropdown_height - 1 {
                break;
            }

            let is_selected = self.selected_item == Some(i);

            if item.separator {
                // Draw separator line
                for dx in 1..dropdown_width - 1 {
                    let mut cell = Cell::new('─');
                    cell.fg = Some(self.disabled_fg);
                    cell.bg = Some(self.bg);
                    ctx.buffer.set(menu_x + dx, item_y, cell);
                }
                // Fix corners
                let mut left = Cell::new('├');
                left.fg = Some(self.fg);
                left.bg = Some(self.bg);
                ctx.buffer.set(menu_x, item_y, left);

                let mut right = Cell::new('┤');
                right.fg = Some(self.fg);
                right.bg = Some(self.bg);
                ctx.buffer.set(menu_x + dropdown_width - 1, item_y, right);
            } else {
                // Draw item
                let bg = if is_selected { self.selected_bg } else { self.bg };
                let fg = if item.disabled {
                    self.disabled_fg
                } else if is_selected {
                    self.selected_fg
                } else {
                    self.fg
                };

                // Fill background
                for dx in 1..dropdown_width - 1 {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(menu_x + dx, item_y, cell);
                }

                // Draw checkbox if present
                let mut text_x = menu_x + 2;
                if let Some(checked) = item.checked {
                    let check_ch = if checked { '✓' } else { ' ' };
                    let mut cell = Cell::new(check_ch);
                    cell.fg = Some(fg);
                    cell.bg = Some(bg);
                    ctx.buffer.set(text_x, item_y, cell);
                    text_x += 2;
                }

                // Draw label
                for ch in item.label.chars() {
                    if text_x >= menu_x + dropdown_width - 2 {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    cell.bg = Some(bg);
                    ctx.buffer.set(text_x, item_y, cell);
                    text_x += 1;
                }

                // Draw shortcut
                if let Some(ref shortcut) = item.shortcut {
                    let shortcut_x = menu_x + dropdown_width - 2 - shortcut.len() as u16;
                    for (j, ch) in shortcut.chars().enumerate() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.shortcut_fg);
                        cell.bg = Some(bg);
                        ctx.buffer.set(shortcut_x + j as u16, item_y, cell);
                    }
                }

                // Draw submenu indicator
                if item.has_submenu() {
                    let mut cell = Cell::new('▶');
                    cell.fg = Some(fg);
                    cell.bg = Some(bg);
                    ctx.buffer.set(menu_x + dropdown_width - 2, item_y, cell);
                }
            }
        }
    }
}

impl_styled_view!(MenuBar);
impl_props_builders!(MenuBar);

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
    fg: Color,
    selected_bg: Color,
    selected_fg: Color,
    /// Widget properties
    props: WidgetProps,
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
            bg: Color::rgb(40, 40, 40),
            fg: Color::WHITE,
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

    /// Handle key
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if !self.visible {
            return false;
        }

        match key {
            Key::Up | Key::Char('k') => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                true
            }
            Key::Down | Key::Char('j') => {
                if self.selected < self.items.len().saturating_sub(1) {
                    self.selected += 1;
                }
                true
            }
            Key::Enter | Key::Char(' ') => {
                if let Some(item) = self.items.get(self.selected) {
                    item.execute();
                }
                self.hide();
                true
            }
            Key::Escape => {
                self.hide();
                true
            }
            _ => false,
        }
    }
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl View for ContextMenu {
    crate::impl_view_meta!("ContextMenu");

    fn render(&self, ctx: &mut RenderContext) {
        if !self.visible || self.items.is_empty() {
            return;
        }

        let width = self.items.iter()
            .map(|i| i.label.len())
            .max()
            .unwrap_or(10) as u16 + 4;
        let height = self.items.len() as u16 + 2;

        // Adjust position to fit in area
        let x = self.x.min(ctx.area.width.saturating_sub(width));
        let y = self.y.min(ctx.area.height.saturating_sub(height));

        // Draw border and background
        for dy in 0..height {
            for dx in 0..width {
                let ch = if dy == 0 && dx == 0 { '┌' }
                else if dy == 0 && dx == width - 1 { '┐' }
                else if dy == height - 1 && dx == 0 { '└' }
                else if dy == height - 1 && dx == width - 1 { '┘' }
                else if dy == 0 || dy == height - 1 { '─' }
                else if dx == 0 || dx == width - 1 { '│' }
                else { ' ' };

                let mut cell = Cell::new(ch);
                cell.bg = Some(self.bg);
                cell.fg = Some(self.fg);
                ctx.buffer.set(x + dx, y + dy, cell);
            }
        }

        // Draw items
        for (i, item) in self.items.iter().enumerate() {
            let item_y = y + 1 + i as u16;
            let is_selected = i == self.selected;

            let bg = if is_selected { self.selected_bg } else { self.bg };
            let fg = if is_selected { self.selected_fg } else { self.fg };

            // Fill row
            for dx in 1..width - 1 {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.buffer.set(x + dx, item_y, cell);
            }

            // Draw label
            for (j, ch) in item.label.chars().enumerate() {
                if j as u16 + 2 >= width - 1 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                ctx.buffer.set(x + 2 + j as u16, item_y, cell);
            }
        }
    }
}

impl_styled_view!(ContextMenu);
impl_props_builders!(ContextMenu);

// Helper functions

/// Create a new dropdown menu
pub fn menu(title: impl Into<String>) -> Menu {
    Menu::new(title)
}

/// Create a new menu item
pub fn menu_item(label: impl Into<String>) -> MenuItem {
    MenuItem::new(label)
}

/// Create a new menu bar
pub fn menu_bar() -> MenuBar {
    MenuBar::new()
}

/// Create a new context menu
pub fn context_menu() -> ContextMenu {
    ContextMenu::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::layout::Rect;

    #[test]
    fn test_menu_item() {
        let item = MenuItem::new("Open")
            .shortcut("Ctrl+O")
            .disabled(false);

        assert_eq!(item.label, "Open");
        assert_eq!(item.shortcut, Some("Ctrl+O".to_string()));
        assert!(!item.disabled);
    }

    #[test]
    fn test_menu() {
        let m = Menu::new("File")
            .item(MenuItem::new("New"))
            .item(MenuItem::new("Open"))
            .separator()
            .item(MenuItem::new("Exit"));

        assert_eq!(m.title, "File");
        assert_eq!(m.items.len(), 4);
        assert!(m.items[2].separator);
    }

    #[test]
    fn test_menu_bar() {
        let mut bar = MenuBar::new()
            .menu(Menu::new("File").item(MenuItem::new("New")))
            .menu(Menu::new("Edit").item(MenuItem::new("Copy")));

        assert_eq!(bar.menus.len(), 2);
        assert!(!bar.is_open());

        bar.open_menu(0);
        assert!(bar.is_open());

        bar.next_menu();
        assert_eq!(bar.selected_menu, 1);

        bar.close();
        assert!(!bar.is_open());
    }

    #[test]
    fn test_context_menu() {
        let mut menu = ContextMenu::new()
            .item(MenuItem::new("Cut"))
            .item(MenuItem::new("Copy"))
            .item(MenuItem::new("Paste"));

        assert!(!menu.is_visible());

        menu.show(10, 10);
        assert!(menu.is_visible());

        menu.handle_key(&Key::Down);
        assert_eq!(menu.selected, 1);

        menu.hide();
        assert!(!menu.is_visible());
    }

    #[test]
    fn test_menu_bar_render() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bar = MenuBar::new()
            .menu(Menu::new("File"))
            .menu(Menu::new("Edit"));

        bar.render(&mut ctx);
        // Smoke test
    }
}
