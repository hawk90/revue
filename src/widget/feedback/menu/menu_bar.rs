//! Menu bar widget

use super::types::Menu;
use crate::event::Key;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps, DISABLED_FG};
use crate::{impl_props_builders, impl_styled_view};

/// Menu bar widget
pub struct MenuBar {
    /// Menus
    pub(crate) menus: Vec<Menu>,
    /// Currently selected menu index
    pub(crate) selected_menu: usize,
    /// Currently selected item index (if menu is open)
    pub(crate) selected_item: Option<usize>,
    /// Is a menu currently open
    pub(crate) open: bool,
    /// Colors
    pub(crate) bg: Color,
    pub(crate) fg: Color,
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
            open: false,
            bg: Color::rgb(40, 40, 40),
            fg: Color::WHITE,
            selected_bg: Color::rgb(60, 100, 180),
            selected_fg: Color::WHITE,
            disabled_fg: DISABLED_FG,
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
            self.selected_menu = self
                .selected_menu
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
        let max_width = menu
            .items
            .iter()
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
                let bg = if is_selected {
                    self.selected_bg
                } else {
                    self.bg
                };
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
