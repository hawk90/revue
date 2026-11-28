//! Drawing the bar and the open menu's dropdown

use super::super::list::{draw_list, fit_x, list_width, ListColors};
use super::super::types::Menu;
use super::MenuBar;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::unicode::display_width_u16;
use crate::widget::traits::{RenderContext, View};

impl View for MenuBar {
    /// The bar, stretching across the offered width, and the open menu, which
    /// is drawn in place below it.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let list = self
            .menus
            .get(self.selected_menu)
            .filter(|menu| self.open && !menu.items.is_empty())
            .map_or(0, |menu| {
                u16::try_from(menu.items.len())
                    .unwrap_or(u16::MAX)
                    .saturating_add(2)
            });
        Some((max_width, list.saturating_add(1).min(max_height)))
    }

    fn fills(&self) -> crate::widget::Fill {
        crate::widget::Fill::WIDTH
    }
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
            x = x.saturating_add(display_width_u16(&title));
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

        let title_x = self.dropdown_x();
        // The menu's title starts past the right edge: no room for its dropdown
        if title_x >= ctx.area.width {
            return;
        }

        // As wide as its items need, or the area; it starts under its title
        // and moves left to fit, as ContextMenu does, instead of being cut at
        // the right edge (#1050)
        let width = list_width(&menu.items).min(ctx.area.width);
        let x = fit_x(title_x, width, ctx.area.width);
        let height = (u16::try_from(menu.items.len())
            .unwrap_or(u16::MAX)
            .saturating_add(2))
        .min(ctx.area.height - y);

        let colors = ListColors {
            fg,
            bg: self.bg,
            selected_fg: self.selected_fg,
            selected_bg: self.selected_bg,
            disabled_fg: self.disabled_fg,
            shortcut_fg: self.shortcut_fg,
        };
        draw_list(
            ctx,
            &menu.items,
            (x, y),
            (width, height),
            self.selected_item,
            &self.scroll,
            colors,
        );
    }

    /// The dropdown's left column: the selected title's offset in the bar.
    fn dropdown_x(&self) -> u16 {
        // Calculate dropdown position (relative)
        let mut menu_x: u16 = 0;
        for (i, m) in self.menus.iter().enumerate() {
            if i == self.selected_menu {
                break;
            }
            menu_x = menu_x.saturating_add(display_width_u16(&m.title).saturating_add(2));
        }
        menu_x
    }
}
