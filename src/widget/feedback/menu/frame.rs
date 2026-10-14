//! The box both menus draw their items in

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::RenderContext;

/// A single-line box at relative `(x, y)`, its inside filled with blanks, all
/// in `fg` on `bg`. Cells outside the area are dropped by `RenderContext::set`.
pub(super) fn draw_frame(
    ctx: &mut RenderContext,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    fg: Color,
    bg: Color,
) {
    // Draw border and background
    for dy in 0..height {
        for dx in 0..width {
            let ch = if dy == 0 && dx == 0 {
                '┌'
            } else if dy == 0 && dx == width - 1 {
                '┐'
            } else if dy == height - 1 && dx == 0 {
                '└'
            } else if dy == height - 1 && dx == width - 1 {
                '┘'
            } else if dy == 0 || dy == height - 1 {
                '─'
            } else if dx == 0 || dx == width - 1 {
                '│'
            } else {
                ' '
            };

            let mut cell = Cell::new(ch);
            cell.bg = Some(bg);
            cell.fg = Some(fg);
            ctx.set(x + dx, y + dy, cell);
        }
    }
}
