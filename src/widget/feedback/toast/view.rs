//! Sizing, placing and drawing the toast, as an overlay or inline

use super::{Toast, ToastPosition};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{OverlayEntry, RenderContext, View};

impl View for Toast {
    crate::impl_view_meta!("Toast");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 3 {
            return;
        }

        // As an overlay the toast is placed on the screen (buffer). Without an
        // overlay layer it is drawn inline, placed inside its own area.
        let inline = !ctx.has_overlay_support();
        let (space_w, space_h) = if inline {
            (area.width, area.height)
        } else {
            (ctx.buffer.width(), ctx.buffer.height())
        };
        let (toast_width, toast_height) = self.calculate_size(space_w);
        // `width(0)`: no box to draw
        if toast_width == 0 {
            return;
        }
        let (abs_x, abs_y) = self.calculate_position(space_w, space_h, toast_width, toast_height);

        // The level palette is this toast's own default, and a rule naming
        // this toast targets the whole node - so the author outranks it.
        let color = ctx.css_color(self.level.color());
        let bg = self.level.bg_color();

        // Build overlay entry for toast (renders on top of everything)
        let overlay_area = crate::layout::Rect::new(abs_x, abs_y, toast_width, toast_height);
        let mut entry = crate::widget::traits::OverlayEntry::new(200, overlay_area); // z=200: above dropdowns

        // Draw border (relative to overlay area)
        if self.show_border {
            push_frame(&mut entry, toast_width, toast_height, color, bg);
        }
        self.push_content(&mut entry, toast_width, color, bg);

        // Queue as overlay; fallback to inline, at the area-relative position
        if inline || !ctx.queue_overlay(entry.clone()) {
            for oc in &entry.cells {
                ctx.set(abs_x + oc.x, abs_y + oc.y, oc.cell);
            }
        }
    }
}

/// Push one cell, relative to the overlay area
fn push(entry: &mut OverlayEntry, rx: u16, ry: u16, ch: char, fg: Color, bg_c: Color) {
    let mut cell = Cell::new(ch);
    cell.fg = Some(fg);
    cell.bg = Some(bg_c);
    entry.push(rx, ry, cell);
}

/// The rounded border, with the inside filled with the background
fn push_frame(
    entry: &mut OverlayEntry,
    toast_width: u16,
    toast_height: u16,
    color: Color,
    bg: Color,
) {
    push(entry, 0, 0, '╭', color, bg);
    for i in 1..toast_width.saturating_sub(1) {
        push(entry, i, 0, '─', color, bg);
    }
    push(entry, toast_width - 1, 0, '╮', color, bg);

    push(entry, 0, toast_height - 1, '╰', color, bg);
    for i in 1..toast_width.saturating_sub(1) {
        push(entry, i, toast_height - 1, '─', color, bg);
    }
    push(entry, toast_width - 1, toast_height - 1, '╯', color, bg);

    for row in 1..toast_height.saturating_sub(1) {
        push(entry, 0, row, '│', color, bg);
        push(entry, toast_width - 1, row, '│', color, bg);
        for col in 1..toast_width.saturating_sub(1) {
            push(entry, col, row, ' ', color, bg);
        }
    }
}

impl Toast {
    /// Calculate toast dimensions
    fn calculate_size(&self, max_width: u16) -> (u16, u16) {
        let icon_width = if self.show_icon { 2 } else { 0 };
        let border_width = if self.show_border { 2 } else { 0 };
        let padding = 2; // 1 char padding on each side

        let content_width = crate::utils::unicode::display_width(&self.message) as u16 + icon_width;
        let total_width = self.width.unwrap_or(content_width + border_width + padding);
        let width = total_width.min(max_width);

        // Calculate height based on message wrapping
        let inner_width = width.saturating_sub(border_width + padding + icon_width);
        let msg_cols = crate::utils::unicode::display_width(&self.message) as u16;
        let lines = msg_cols
            .saturating_add(inner_width.saturating_sub(1))
            .checked_div(inner_width)
            .unwrap_or(1);
        let height = lines + if self.show_border { 2 } else { 0 };

        (width, height.max(if self.show_border { 3 } else { 1 }))
    }

    /// Calculate toast position
    fn calculate_position(
        &self,
        area_width: u16,
        area_height: u16,
        toast_width: u16,
        toast_height: u16,
    ) -> (u16, u16) {
        let margin = 1u16;

        let x = match self.position {
            ToastPosition::TopLeft | ToastPosition::BottomLeft => margin,
            ToastPosition::TopCenter | ToastPosition::BottomCenter => {
                area_width.saturating_sub(toast_width) / 2
            }
            ToastPosition::TopRight | ToastPosition::BottomRight => {
                area_width.saturating_sub(toast_width + margin)
            }
        };

        let y = match self.position {
            ToastPosition::TopLeft | ToastPosition::TopCenter | ToastPosition::TopRight => margin,
            ToastPosition::BottomLeft
            | ToastPosition::BottomCenter
            | ToastPosition::BottomRight => area_height.saturating_sub(toast_height + margin),
        };

        (x, y)
    }

    /// The icon and the message on the first content row
    fn push_content(&self, entry: &mut OverlayEntry, toast_width: u16, color: Color, bg: Color) {
        // Content position (relative)
        let cx = if self.show_border { 2 } else { 0 };
        let cy = if self.show_border { 1 } else { 0 };

        if self.show_icon {
            push(entry, cx, cy, self.level.icon(), color, bg);
        }

        // Message text
        let msg_x = cx + if self.show_icon { 2 } else { 0 };
        let max_text = toast_width.saturating_sub(msg_x + if self.show_border { 1 } else { 0 });
        let truncated = crate::utils::truncate_to_width(&self.message, max_text as usize);
        let mut tx = msg_x;
        for ch in truncated.chars() {
            let cw = crate::utils::char_width(ch) as u16;
            if tx + cw > msg_x + max_text {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            cell.bg = Some(bg);
            entry.push(tx, cy, cell);
            tx += cw;
        }
    }
}
