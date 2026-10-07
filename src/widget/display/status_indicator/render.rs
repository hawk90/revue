//! Drawing the indicator: dot, dot with label, label only and badge, with the pulse

use super::{StatusIndicator, StatusSize, StatusStyle};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::{DARK_BG, SECONDARY_TEXT};
use crate::widget::traits::{RenderContext, View};
use unicode_width::UnicodeWidthChar;

impl View for StatusIndicator {
    crate::impl_view_meta!("StatusIndicator");

    /// One row, [`width`](StatusIndicator::width) columns.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((self.width().min(max_width), 1.min(max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 1 || area.height < 1 {
            return;
        }

        // The status palette is this widget's own default. A rule naming this
        // indicator targets the whole node, so the author outranks it.
        let color = ctx.css_color(self.status.color());
        let visible = self.is_visible();

        match self.style {
            StatusStyle::Dot => {
                self.render_dot(ctx, color, visible);
            }
            StatusStyle::DotWithLabel => {
                self.render_dot_with_label(ctx, color, visible);
            }
            StatusStyle::LabelOnly => {
                self.render_label_only(ctx, color);
            }
            StatusStyle::Badge => {
                self.render_badge(ctx, color, visible);
            }
        }
    }
}

impl StatusIndicator {
    /// Check if currently visible (for pulsing animation)
    fn is_visible(&self) -> bool {
        if !self.pulsing {
            return true;
        }
        // Pulse every 8 frames (visible for 6, hidden for 2)
        (self.frame % 8) < 6
    }

    fn render_dot(&self, ctx: &mut RenderContext, color: Color, visible: bool) {
        let area = ctx.area;
        let dot = if visible { self.size.dot() } else { ' ' };

        let mut cell = Cell::new(dot);
        cell.fg = Some(color);
        ctx.set(0, 0, cell);

        // For large size, add extra visual
        if self.size == StatusSize::Large && area.width > 1 {
            let mut cell2 = Cell::new(' ');
            cell2.bg = Some(color);
            ctx.set(1, 0, cell2);
        }
    }

    fn render_dot_with_label(&self, ctx: &mut RenderContext, color: Color, visible: bool) {
        let area = ctx.area;

        // Render dot
        let dot = if visible { self.size.dot() } else { ' ' };
        let mut dot_cell = Cell::new(dot);
        dot_cell.fg = Some(color);
        ctx.set(0, 0, dot_cell);

        // Render label
        let label = self.get_label();
        let label_start = self.size.width() + 1;
        let max_label_width = area.width.saturating_sub(self.size.width() + 1);

        let mut offset = 0u16;
        for ch in label.chars() {
            let char_width = ch.width().unwrap_or(0) as u16;
            if char_width == 0 {
                continue;
            }
            if offset + char_width > max_label_width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(SECONDARY_TEXT);
            ctx.set(label_start + offset, 0, cell);
            for i in 1..char_width {
                ctx.set(label_start + offset + i, 0, Cell::continuation());
            }
            offset += char_width;
        }
    }

    fn render_label_only(&self, ctx: &mut RenderContext, color: Color) {
        let area = ctx.area;
        let label = self.get_label();

        let mut offset = 0u16;
        for ch in label.chars() {
            let char_width = ch.width().unwrap_or(0) as u16;
            if char_width == 0 {
                continue;
            }
            if offset + char_width > area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(color);
            ctx.set(offset, 0, cell);
            for i in 1..char_width {
                ctx.set(offset + i, 0, Cell::continuation());
            }
            offset += char_width;
        }
    }

    fn render_badge(&self, ctx: &mut RenderContext, color: Color, visible: bool) {
        let area = ctx.area;
        let label = self.get_label();

        // Background
        let bg_color = DARK_BG;
        let total_width = self.width().min(area.width);

        for i in 0..total_width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(bg_color);
            ctx.set(i, 0, cell);
        }

        // Dot
        let dot = if visible { '●' } else { ' ' };
        let mut dot_cell = Cell::new(dot);
        dot_cell.fg = Some(color);
        dot_cell.bg = Some(bg_color);
        ctx.set(1, 0, dot_cell);

        // Label
        let label_start: u16 = 3;
        let max_label_width = total_width.saturating_sub(4);
        let mut offset = 0u16;
        for ch in label.chars() {
            let char_width = ch.width().unwrap_or(0) as u16;
            if char_width == 0 {
                continue;
            }
            if offset + char_width > max_label_width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            cell.bg = Some(bg_color);
            ctx.set(label_start + offset, 0, cell);
            for i in 1..char_width {
                let mut cont = Cell::continuation();
                cont.bg = Some(bg_color);
                ctx.set(label_start + offset + i, 0, cont);
            }
            offset += char_width;
        }
    }
}
