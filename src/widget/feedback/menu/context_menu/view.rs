//! Drawing the menu at its position, kept inside the area

use super::super::frame::draw_frame;
use super::ContextMenu;
use crate::render::Cell;
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

        let width = self
            .items
            .iter()
            .map(|i| crate::utils::display_width(&i.label))
            .max()
            .unwrap_or(10) as u16
            + 4;
        // As tall as the items and the border, or the area; the items scroll
        // to keep the selected one in view (#973)
        let height = (self.items.len() as u16 + 2).min(ctx.area.height);
        if height < 3 {
            return;
        }
        self.scroll.set_content(self.items.len());
        self.scroll.set_viewport(usize::from(height - 2));
        self.scroll.track(self.selected..self.selected + 1);
        let shown = self.scroll.visible();

        // Adjust position to fit in area (relative coordinates)
        let x = self.x.min(ctx.area.width.saturating_sub(width));
        let y = self.y.min(ctx.area.height.saturating_sub(height));

        draw_frame(ctx, x, y, width, height, fg, self.bg);

        // Draw items
        for (row, (i, item)) in self
            .items
            .iter()
            .enumerate()
            .skip(shown.start)
            .take(shown.len())
            .enumerate()
        {
            let item_y = y + 1 + row as u16;
            let is_selected = i == self.selected;

            let bg = if is_selected {
                self.selected_bg
            } else {
                self.bg
            };
            let fg = if is_selected { self.selected_fg } else { fg };

            // Fill row
            for dx in 1..width - 1 {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.set(x + dx, item_y, cell);
            }

            // Draw label (wide glyphs get their continuation cell, so the
            // row fill does not show through their right half)
            ctx.put_str_with(x + 2, item_y, &item.label, x + width - 2, |ch| {
                Cell::new(ch).fg(fg).bg(bg)
            });
        }
    }
}
