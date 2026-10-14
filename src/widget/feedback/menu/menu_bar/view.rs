//! Drawing the bar and the open menu's dropdown

use super::super::frame::draw_frame;
use super::super::types::{Menu, MenuItem};
use super::MenuBar;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::unicode::display_width;
use crate::widget::traits::{RenderContext, View};

impl View for MenuBar {
    crate::impl_view_meta!("MenuBar");

    fn render(&self, ctx: &mut RenderContext) {
        // Unselected entries take the stylesheet; the selected one keeps its
        // highlight, which a rule cannot address separately.
        let fg = self.fg.unwrap_or_else(|| ctx.css_color(Color::WHITE));
        let area = ctx.area;
        if area.height < 1 {
            return;
        }

        // Draw menu bar background
        for x in 0..area.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(self.bg);
            ctx.set(x, 0, cell);
        }

        // Draw menu titles
        let mut x: u16 = 0;
        for (i, menu) in self.menus.iter().enumerate() {
            let is_selected = i == self.selected_menu;
            let title = format!(" {} ", menu.title);
            let (title_fg, title_bg) = if is_selected {
                (self.selected_fg, self.selected_bg)
            } else {
                (fg, self.bg)
            };
            ctx.put_str_with(x, 0, &title, area.width, |ch| {
                Cell::new(ch).fg(title_fg).bg(title_bg)
            });
            // Advance by the title's columns, the offset the dropdown uses.
            x = x.saturating_add(display_width(&title) as u16);
        }

        // Draw dropdown if open
        if self.open {
            if let Some(menu) = self.menus.get(self.selected_menu) {
                self.render_dropdown(ctx, menu, 1);
            }
        }
    }
}

impl MenuBar {
    fn render_dropdown(&self, ctx: &mut RenderContext, menu: &Menu, y: u16) {
        // Resolved once: the builder's color if it named one, else the
        // stylesheet's, else white.
        let fg = self.fg.unwrap_or_else(|| ctx.css_color(Color::WHITE));
        if menu.items.is_empty() || y >= ctx.area.height {
            return;
        }

        let menu_x = self.dropdown_x();
        // The menu's title starts past the right edge: no room for its dropdown
        if menu_x >= ctx.area.width {
            return;
        }

        let max_width = dropdown_width(menu);

        let dropdown_width = max_width.min(ctx.area.width - menu_x);
        let dropdown_height = (menu.items.len() as u16 + 2).min(ctx.area.height - y);

        draw_frame(ctx, menu_x, y, dropdown_width, dropdown_height, fg, self.bg);

        // Draw items
        for (i, item) in menu.items.iter().enumerate() {
            let item_y = y + 1 + i as u16;
            if item_y >= y + dropdown_height - 1 {
                break;
            }

            let is_selected = self.selected_item == Some(i);

            if item.separator {
                self.draw_separator_row(ctx, menu_x, dropdown_width, item_y, fg);
            } else {
                self.draw_item_row(ctx, item, is_selected, menu_x, dropdown_width, item_y, fg);
            }
        }
    }

    /// The dropdown's left column: the selected title's offset in the bar.
    fn dropdown_x(&self) -> u16 {
        // Calculate dropdown position (relative)
        let mut menu_x: u16 = 0;
        for (i, m) in self.menus.iter().enumerate() {
            if i == self.selected_menu {
                break;
            }
            menu_x = menu_x.saturating_add(display_width(&m.title) as u16 + 2);
        }
        menu_x
    }

    /// A separator: a dim line across the dropdown, joined to its border.
    fn draw_separator_row(
        &self,
        ctx: &mut RenderContext,
        menu_x: u16,
        dropdown_width: u16,
        item_y: u16,
        fg: Color,
    ) {
        // Draw separator line
        for dx in 1..dropdown_width - 1 {
            let mut cell = Cell::new('─');
            cell.fg = Some(self.disabled_fg);
            cell.bg = Some(self.bg);
            ctx.set(menu_x + dx, item_y, cell);
        }
        // Fix corners
        let mut left = Cell::new('├');
        left.fg = Some(fg);
        left.bg = Some(self.bg);
        ctx.set(menu_x, item_y, left);

        let mut right = Cell::new('┤');
        right.fg = Some(fg);
        right.bg = Some(self.bg);
        ctx.set(menu_x + dropdown_width - 1, item_y, right);
    }

    /// An item: its check mark, label, right-aligned shortcut and submenu
    /// marker, in the selected, disabled or plain colors.
    #[allow(clippy::too_many_arguments)]
    fn draw_item_row(
        &self,
        ctx: &mut RenderContext,
        item: &MenuItem,
        is_selected: bool,
        menu_x: u16,
        dropdown_width: u16,
        item_y: u16,
        fg: Color,
    ) {
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
            fg
        };

        // Fill background
        for dx in 1..dropdown_width - 1 {
            let mut cell = Cell::new(' ');
            cell.bg = Some(bg);
            ctx.set(menu_x + dx, item_y, cell);
        }

        // Draw checkbox if present
        let mut text_x = menu_x + 2;
        if let Some(checked) = item.checked {
            let check_ch = if checked { '✓' } else { ' ' };
            let mut cell = Cell::new(check_ch);
            cell.fg = Some(fg);
            cell.bg = Some(bg);
            ctx.set(text_x, item_y, cell);
            text_x += 2;
        }

        // Draw label
        ctx.put_str_with(
            text_x,
            item_y,
            &item.label,
            menu_x + dropdown_width - 2,
            |ch| Cell::new(ch).fg(fg).bg(bg),
        );

        // Draw shortcut, right-aligned by its columns
        if let Some(ref shortcut) = item.shortcut {
            let shortcut_x =
                (menu_x + dropdown_width - 2).saturating_sub(display_width(shortcut) as u16);
            let shortcut_fg = self.shortcut_fg;
            ctx.put_str_with(
                shortcut_x,
                item_y,
                shortcut,
                menu_x + dropdown_width - 1,
                |ch| Cell::new(ch).fg(shortcut_fg).bg(bg),
            );
        }

        // Draw submenu indicator
        if item.has_submenu() {
            let mut cell = Cell::new('▶');
            cell.fg = Some(fg);
            cell.bg = Some(bg);
            ctx.set(menu_x + dropdown_width - 2, item_y, cell);
        }
    }
}

/// The columns the widest item needs: label, shortcut and padding.
fn dropdown_width(menu: &Menu) -> u16 {
    // Calculate max width
    menu.items
        .iter()
        .filter(|item| !item.separator)
        .map(|item| {
            let shortcut_width = item
                .shortcut
                .as_ref()
                .map(|s| display_width(s) + 2)
                .unwrap_or(0);
            display_width(&item.label) + shortcut_width + 4
        })
        .max()
        .unwrap_or(10) as u16
}
