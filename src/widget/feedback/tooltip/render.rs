//! Drawing the tooltip as an overlay: background, border, title, text and arrow

use super::{Tooltip, TooltipArrow, TooltipPosition};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

impl TooltipArrow {
    fn chars(&self, position: TooltipPosition) -> (char, char) {
        match (self, position) {
            (TooltipArrow::None, _) => (' ', ' '),
            (TooltipArrow::Simple, TooltipPosition::Top) => ('v', 'v'),
            (TooltipArrow::Simple, TooltipPosition::Bottom) => ('^', '^'),
            (TooltipArrow::Simple, TooltipPosition::Left) => ('>', '>'),
            (TooltipArrow::Simple, TooltipPosition::Right) => ('<', '<'),
            (TooltipArrow::Simple, TooltipPosition::Auto) => ('v', 'v'),
            (TooltipArrow::Unicode, TooltipPosition::Top) => ('▼', '▽'),
            (TooltipArrow::Unicode, TooltipPosition::Bottom) => ('▲', '△'),
            (TooltipArrow::Unicode, TooltipPosition::Left) => ('▶', '▷'),
            (TooltipArrow::Unicode, TooltipPosition::Right) => ('◀', '◁'),
            (TooltipArrow::Unicode, TooltipPosition::Auto) => ('▼', '▽'),
        }
    }
}

impl View for Tooltip {
    crate::impl_view_meta!("Tooltip");

    fn render(&self, ctx: &mut RenderContext) {
        if !self.visible || (self.delay > 0 && self.delay_counter < self.delay) {
            return;
        }

        let area = ctx.area;
        let (tooltip_w, tooltip_h) = self.calculate_dimensions();
        let (tooltip_x, tooltip_y, actual_position) =
            self.calculate_position(area.width, area.height);

        // Builder, then the stylesheet, then the tooltip style's own palette.
        let (default_fg, default_bg) = self.style.colors();
        let fg = self.fg.unwrap_or_else(|| ctx.css_color(default_fg));
        let bg = self.bg.unwrap_or_else(|| ctx.css_background(default_bg));

        // Build overlay entry (all coordinates relative to tooltip area)
        let overlay_area = crate::layout::Rect::new(tooltip_x, tooltip_y, tooltip_w, tooltip_h);
        let mut entry = crate::widget::traits::OverlayEntry::new(150, overlay_area);

        // Helper to create styled cell
        let cell_with = |ch: char, cell_fg: Color, cell_bg: Color| -> Cell {
            let mut c = Cell::new(ch);
            c.fg = Some(cell_fg);
            c.bg = Some(cell_bg);
            c
        };

        // Background
        for dy in 0..tooltip_h {
            for dx in 0..tooltip_w {
                entry.push(dx, dy, cell_with(' ', fg, bg));
            }
        }

        // Border
        let content_rx;
        let content_ry;

        if let Some(border) = self.style.border_chars() {
            content_rx = 2u16;
            content_ry = 1u16;

            entry.push(0, 0, cell_with(border.top_left, fg, bg));
            for dx in 1..tooltip_w.saturating_sub(1) {
                entry.push(dx, 0, cell_with(border.horizontal, fg, bg));
            }
            entry.push(
                tooltip_w.saturating_sub(1),
                0,
                cell_with(border.top_right, fg, bg),
            );

            // Title (bold)
            if let Some(ref title) = self.title {
                entry.push_str_with(2, 1, title, tooltip_w.saturating_sub(2), |ch| {
                    let mut c = cell_with(ch, fg, bg);
                    c.modifier |= Modifier::BOLD;
                    c
                });
            }

            // Side borders
            for dy in 1..tooltip_h.saturating_sub(1) {
                entry.push(0, dy, cell_with(border.vertical, fg, bg));
                entry.push(
                    tooltip_w.saturating_sub(1),
                    dy,
                    cell_with(border.vertical, fg, bg),
                );
            }

            // Bottom border
            let by = tooltip_h.saturating_sub(1);
            entry.push(0, by, cell_with(border.bottom_left, fg, bg));
            for dx in 1..tooltip_w.saturating_sub(1) {
                entry.push(dx, by, cell_with(border.horizontal, fg, bg));
            }
            entry.push(
                tooltip_w.saturating_sub(1),
                by,
                cell_with(border.bottom_right, fg, bg),
            );
        } else {
            content_rx = 1;
            content_ry = 0;
        }

        // Text content
        let lines = self.wrap_text();
        let text_y_off = if self.title.is_some() && self.style.border_chars().is_some() {
            1u16
        } else {
            0
        };

        // The last row is the bottom border, if there is one.
        let text_end = if self.style.border_chars().is_some() {
            tooltip_h.saturating_sub(1)
        } else {
            tooltip_h
        };
        for (i, line) in lines.iter().enumerate() {
            let ry = content_ry + text_y_off + i as u16;
            if ry >= text_end {
                break;
            }
            entry.push_str_with(content_rx, ry, line, tooltip_w.saturating_sub(1), |ch| {
                cell_with(ch, fg, bg)
            });
        }

        // Arrow — queue as separate 1-cell overlay at higher z-index
        if !matches!(self.arrow, TooltipArrow::None) {
            let (arrow_char, _) = self.arrow.chars(actual_position);
            let (arrow_abs_x, arrow_abs_y) = match actual_position {
                TooltipPosition::Top => (self.anchor.0, tooltip_y + tooltip_h),
                TooltipPosition::Bottom => (self.anchor.0, tooltip_y.saturating_sub(1)),
                TooltipPosition::Left => (tooltip_x + tooltip_w, self.anchor.1),
                TooltipPosition::Right => (tooltip_x.saturating_sub(1), self.anchor.1),
                TooltipPosition::Auto => (self.anchor.0, tooltip_y + tooltip_h),
            };

            let inside = arrow_abs_x >= tooltip_x
                && arrow_abs_x < tooltip_x + tooltip_w
                && arrow_abs_y >= tooltip_y
                && arrow_abs_y < tooltip_y + tooltip_h;

            let buf_w = ctx.buffer.width();
            let buf_h = ctx.buffer.height();
            if !inside && arrow_abs_x < buf_w && arrow_abs_y < buf_h {
                let arrow_area = crate::layout::Rect::new(arrow_abs_x, arrow_abs_y, 1, 1);
                let mut arrow_entry = crate::widget::traits::OverlayEntry::new(151, arrow_area);
                let mut cell = Cell::new(arrow_char);
                cell.fg = Some(fg);
                arrow_entry.push(0, 0, cell);
                // Without an overlay layer, draw inline like the body below.
                if !ctx.queue_overlay(arrow_entry) {
                    ctx.set(arrow_abs_x, arrow_abs_y, cell);
                }
            }
        }

        // Queue tooltip as overlay; fallback to inline
        if !ctx.queue_overlay(entry.clone()) {
            for oc in &entry.cells {
                ctx.set(tooltip_x + oc.x, tooltip_y + oc.y, oc.cell);
            }
        }
    }
}
