//! Drawing the menu at its position, kept inside the area

use super::super::list::{draw_list, fit_x, list_width, ListColors};
use super::ContextMenu;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

impl View for ContextMenu {
    crate::impl_view_meta!("ContextMenu");

    fn render(&self, ctx: &mut RenderContext) {
        // Unselected entries take `color`; the selected one keeps its highlight.
        let fg = self.fg.unwrap_or_else(|| ctx.css_color(Color::WHITE));
        if !self.visible || self.items.is_empty() {
            return;
        }

        // As wide as its items need (labels, check marks, shortcuts), or
        // the area, and as tall as the items and the border, or the area;
        // the items scroll to keep the selected one in view (#973)
        let width = list_width(&self.items).min(ctx.area.width);
        let height = (u16::try_from(self.items.len())
            .unwrap_or(u16::MAX)
            .saturating_add(2))
        .min(ctx.area.height);
        if height < 3 {
            return;
        }

        // Adjust position to fit in area (relative coordinates)
        let x = fit_x(self.x, width, ctx.area.width);
        let y = self.y.min(ctx.area.height.saturating_sub(height));

        // The same rows as MenuBar's dropdown: separators, disabled items,
        // check marks, shortcuts and submenu markers (#1051)
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
            &self.items,
            (x, y),
            (width, height),
            Some(self.selected),
            &self.scroll,
            colors,
        );
    }
}
