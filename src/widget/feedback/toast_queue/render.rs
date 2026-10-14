//! Placing the visible toasts and drawing each one

use super::{StackDirection, ToastEntry, ToastPosition, ToastQueue};
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, truncate_to_width};
use crate::widget::theme::DISABLED_FG;
use crate::widget::traits::{RenderContext, View};

impl ToastQueue {
    /// Calculate toast height
    fn toast_height(&self) -> u16 {
        3 // border + content
    }

    /// Calculate base position
    fn calculate_base_position(&self, area_width: u16, area_height: u16) -> (u16, u16) {
        let margin = 1u16;
        let toast_w = self.toast_width;
        let total_height = (self.visible.len() as u16) * (self.toast_height() + self.gap);

        let x = match self.position {
            ToastPosition::TopLeft | ToastPosition::BottomLeft => margin,
            ToastPosition::TopCenter | ToastPosition::BottomCenter => {
                area_width.saturating_sub(toast_w) / 2
            }
            ToastPosition::TopRight | ToastPosition::BottomRight => {
                area_width.saturating_sub(toast_w + margin)
            }
        };

        let y = match self.position {
            ToastPosition::TopLeft | ToastPosition::TopCenter | ToastPosition::TopRight => margin,
            ToastPosition::BottomLeft
            | ToastPosition::BottomCenter
            | ToastPosition::BottomRight => area_height.saturating_sub(total_height + margin),
        };

        (x, y)
    }

    /// Render a single toast
    fn render_toast(&self, ctx: &mut RenderContext, entry: &ToastEntry, x: u16, y: u16) {
        let area = ctx.area;
        let toast_w = self.toast_width.min(area.width.saturating_sub(x));
        let toast_h = self.toast_height();

        if x >= area.width || y >= area.height {
            return;
        }

        let color = entry.level.color();
        let bg = entry.level.bg_color();

        draw_frame(ctx, x, y, toast_w, toast_h, color, bg);
        draw_content(ctx, entry, x, y, toast_w, color, bg);
    }
}

/// A cell with the toast's background
fn cell(ch: char, fg: Option<Color>, bg: Color) -> Cell {
    let mut cell = Cell::new(ch);
    cell.fg = fg;
    cell.bg = Some(bg);
    cell
}

/// The rounded border, with the inside filled with the background
fn draw_frame(
    ctx: &mut RenderContext,
    x: u16,
    y: u16,
    toast_w: u16,
    toast_h: u16,
    color: Color,
    bg: Color,
) {
    // Draw border
    // Top
    ctx.set(x, y, cell('╭', Some(color), bg));

    for i in 1..toast_w.saturating_sub(1) {
        ctx.set(x + i, y, cell('─', Some(color), bg));
    }

    ctx.set(x + toast_w - 1, y, cell('╮', Some(color), bg));

    // Bottom
    ctx.set(x, y + toast_h - 1, cell('╰', Some(color), bg));

    for i in 1..toast_w.saturating_sub(1) {
        ctx.set(x + i, y + toast_h - 1, cell('─', Some(color), bg));
    }

    ctx.set(x + toast_w - 1, y + toast_h - 1, cell('╯', Some(color), bg));

    // Sides and fill
    for row in 1..toast_h.saturating_sub(1) {
        ctx.set(x, y + row, cell('│', Some(color), bg));
        ctx.set(x + toast_w - 1, y + row, cell('│', Some(color), bg));

        for col in 1..toast_w.saturating_sub(1) {
            ctx.set(x + col, y + row, cell(' ', None, bg));
        }
    }
}

/// The icon, the message and, for a dismissible toast, the close mark
fn draw_content(
    ctx: &mut RenderContext,
    entry: &ToastEntry,
    x: u16,
    y: u16,
    toast_w: u16,
    color: Color,
    bg: Color,
) {
    // Content
    let content_x = x + 2;
    let content_y = y + 1;

    // Icon
    ctx.set(
        content_x,
        content_y,
        cell(entry.level.icon(), Some(color), bg),
    );

    // Message
    let msg_x = content_x + 2;
    let max_msg_width = toast_w.saturating_sub(5) as usize;
    let truncated_msg = truncate_to_width(&entry.message, max_msg_width);
    let mut dx: u16 = 0;
    for ch in truncated_msg.chars() {
        let cw = char_width(ch) as u16;
        if dx + cw > max_msg_width as u16 {
            break;
        }
        let fg = ctx.css_color(Color::WHITE);
        ctx.set(msg_x + dx, content_y, cell(ch, Some(fg), bg));
        dx += cw;
    }

    // Dismiss hint for dismissible toasts
    if entry.dismissible && toast_w > 10 {
        let dismiss_x = x + toast_w - 3;
        ctx.set(dismiss_x, content_y, cell('×', Some(DISABLED_FG), bg));
    }
}

impl View for ToastQueue {
    crate::impl_view_meta!("ToastQueue");

    fn render(&self, ctx: &mut RenderContext) {
        if self.visible.is_empty() {
            return;
        }

        let area = ctx.area;
        let (base_x, base_y) = self.calculate_base_position(area.width, area.height);

        // The stack fills the same block below `base_y` either way; `Up` only
        // reverses the order, so the newest (last) toast is on top.
        let count = self.visible.len();
        for (i, entry) in self.visible.iter().enumerate() {
            let slot = match self.stack_direction {
                StackDirection::Down => i,
                StackDirection::Up => count - 1 - i,
            };
            let y = base_y.saturating_add((slot as u16) * (self.toast_height() + self.gap));

            if y < area.height {
                self.render_toast(ctx, entry, base_x, y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    /// The row each toast's message is drawn on, oldest first.
    fn message_rows(position: ToastPosition, direction: StackDirection) -> Vec<Option<u16>> {
        let mut queue = ToastQueue::new()
            .position(position)
            .stack_direction(direction)
            .toast_width(12);
        queue.info("A");
        queue.info("B");
        queue.tick();
        let mut buf = Buffer::new(14, 12);
        let mut ctx = RenderContext::new(&mut buf, Rect::new(0, 0, 14, 12));
        queue.render(&mut ctx);
        // The message starts 4 cells into a toast at x = 1 (the margin).
        ['A', 'B']
            .iter()
            .map(|&m| (0..12).find(|&y| buf.get(5, y).unwrap().symbol == m))
            .collect()
    }

    #[test]
    fn stacking_up_puts_the_newest_toast_on_top_within_the_area() {
        // Toasts are 3 rows with a 1-row gap; the margin is 1.
        assert_eq!(
            message_rows(ToastPosition::TopLeft, StackDirection::Down),
            [Some(2), Some(6)]
        );
        assert_eq!(
            message_rows(ToastPosition::TopLeft, StackDirection::Up),
            [Some(6), Some(2)]
        );
        assert_eq!(
            message_rows(ToastPosition::BottomLeft, StackDirection::Down),
            [Some(4), Some(8)]
        );
        assert_eq!(
            message_rows(ToastPosition::BottomLeft, StackDirection::Up),
            [Some(8), Some(4)]
        );
    }
}
