//! The frame: background, horizontal rules, side borders and the result count.

use crate::render::Cell;
use crate::utils::{char_width, display_width};
use crate::widget::theme::DARK_GRAY;
use crate::widget::traits::RenderContext;

use super::{CommandPalette, Layout};

impl CommandPalette {
    /// Fill the whole frame with the background color.
    pub(super) fn draw_background(&self, ctx: &mut RenderContext, layout: &Layout) {
        for dy in 0..layout.height {
            for dx in 0..layout.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.bg_color);
                ctx.set(layout.x + dx, layout.y + dy, cell);
            }
        }
    }

    /// A horizontal border row: `left`, `─` across, `right`.
    pub(super) fn draw_rule(
        &self,
        ctx: &mut RenderContext,
        layout: &Layout,
        y: u16,
        left: char,
        right: char,
    ) {
        ctx.set(layout.x, y, self.border_cell(left));

        for dx in 1..layout.width.saturating_sub(1) {
            ctx.set(layout.x + dx, y, self.border_cell('─'));
        }

        ctx.set(layout.right_x, y, self.border_cell(right));
    }

    /// The left border of a row.
    pub(super) fn draw_left(&self, ctx: &mut RenderContext, layout: &Layout, y: u16) {
        ctx.set(layout.x, y, self.border_cell('│'));
    }

    /// The right border of a row.
    pub(super) fn draw_right(&self, ctx: &mut RenderContext, layout: &Layout, y: u16) {
        ctx.set(layout.right_x, y, self.border_cell('│'));
    }

    /// Both side borders of an empty row.
    pub(super) fn draw_sides(&self, ctx: &mut RenderContext, layout: &Layout, y: u16) {
        self.draw_left(ctx, layout, y);
        self.draw_right(ctx, layout, y);
    }

    /// "filtered/total", right-aligned on the bottom border.
    pub(super) fn draw_count(&self, ctx: &mut RenderContext, layout: &Layout, bottom_y: u16) {
        let count_str = format!("{}/{}", self.filtered.len(), self.commands.len());
        let count_x = layout
            .text_end
            .saturating_sub(display_width(&count_str) as u16);
        let mut dx: u16 = 0;
        for ch in count_str.chars() {
            let cw = char_width(ch) as u16;
            let mut cell = Cell::new(ch);
            cell.fg = Some(DARK_GRAY);
            cell.bg = Some(self.bg_color);
            ctx.set(count_x + dx, bottom_y, cell);
            dx += cw;
        }
    }
}
