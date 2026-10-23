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

        // Draw border (relative to overlay area); a borderless toast is
        // filled with its background all the same
        if self.show_border {
            push_frame(&mut entry, toast_width, toast_height, color, bg);
        } else {
            push_fill(&mut entry, toast_width, toast_height, color, bg);
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

/// The box with no border: every cell on the background
fn push_fill(
    entry: &mut OverlayEntry,
    toast_width: u16,
    toast_height: u16,
    color: Color,
    bg: Color,
) {
    for row in 0..toast_height {
        for col in 0..toast_width {
            push(entry, col, row, ' ', color, bg);
        }
    }
}

impl Toast {
    /// The columns left of the content (the border and its padding) and
    /// right of it (the border, or a 2-cell padding without one)
    fn insets(&self) -> (u16, u16) {
        if self.show_border {
            (2, 2)
        } else {
            (0, 2)
        }
    }

    /// The icon's columns: the icon and the space after it
    fn icon_width(&self) -> u16 {
        if self.show_icon {
            2
        } else {
            0
        }
    }

    /// The columns the message gets in a box `width` wide
    fn text_width(&self, width: u16) -> u16 {
        let (left, right) = self.insets();
        width.saturating_sub(left + right + self.icon_width())
    }

    /// The message's lines in a box `width` wide: the shared word wrap at
    /// the columns it gets, or one line (the box's) when it gets none
    fn lines(&self, width: u16) -> Vec<String> {
        match self.text_width(width) {
            0 => vec![String::new()],
            w => {
                let lines = crate::text::wrap_words(&self.message, w as usize);
                if lines.is_empty() {
                    vec![String::new()]
                } else {
                    lines
                }
            }
        }
    }

    /// Calculate toast dimensions
    fn calculate_size(&self, max_width: u16) -> (u16, u16) {
        let (left, right) = self.insets();
        let natural = crate::utils::unicode::display_width(&self.message) as u16
            + self.icon_width()
            + left
            + right;
        let width = self.width.unwrap_or(natural).min(max_width);
        // One row per line the message wraps to, plus the border
        let height = self.lines(width).len() as u16 + if self.show_border { 2 } else { 0 };
        (width, height)
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

        // The margin at the Left and Top positions goes where the box would
        // not fit beside it (the other positions saturate at 0)
        let x = match self.position {
            ToastPosition::TopLeft | ToastPosition::BottomLeft => {
                margin.min(area_width.saturating_sub(toast_width))
            }
            ToastPosition::TopCenter | ToastPosition::BottomCenter => {
                area_width.saturating_sub(toast_width) / 2
            }
            ToastPosition::TopRight | ToastPosition::BottomRight => {
                area_width.saturating_sub(toast_width + margin)
            }
        };

        let y = match self.position {
            ToastPosition::TopLeft | ToastPosition::TopCenter | ToastPosition::TopRight => {
                margin.min(area_height.saturating_sub(toast_height))
            }
            ToastPosition::BottomLeft
            | ToastPosition::BottomCenter
            | ToastPosition::BottomRight => area_height.saturating_sub(toast_height + margin),
        };

        (x, y)
    }

    /// The icon on the first content row and the message's lines beside
    /// it, kept between the insets: an icon or glyph that does not fit
    /// there is not drawn
    fn push_content(&self, entry: &mut OverlayEntry, toast_width: u16, color: Color, bg: Color) {
        let (left, right) = self.insets();
        let end = toast_width.saturating_sub(right);
        let cy = if self.show_border { 1 } else { 0 };

        if self.show_icon && left < end {
            push(entry, left, cy, self.level.icon(), color, bg);
        }

        let msg_x = left + self.icon_width();
        for (row, line) in self.lines(toast_width).iter().enumerate() {
            let mut tx = msg_x;
            for ch in line.chars() {
                let cw = crate::utils::char_width(ch) as u16;
                if tx + cw > end {
                    break;
                }
                push(entry, tx, cy + row as u16, ch, Color::WHITE, bg);
                tx += cw;
            }
        }
    }
}
