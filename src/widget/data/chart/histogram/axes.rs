//! Histogram axis labels: bin values and counts, for both orientations

use super::Histogram;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, truncate_to_width};
use crate::widget::traits::RenderContext;

impl Histogram {
    /// Render axis labels for a horizontal histogram: bin values down the
    /// left side, counts along the bottom
    pub(super) fn render_axes_horizontal(
        &self,
        ctx: &mut RenderContext,
        area: Rect,
        chart_area: Rect,
    ) {
        if self.bins.is_empty() {
            return;
        }

        let x_min = self.bins.first().map(|b| b.start).unwrap_or(0.0);
        let x_max = self.bins.last().map(|b| b.end).unwrap_or(1.0);
        let y_max = self.max_value();

        let mut put_str = |text: &str, x: u16, y: u16, max_x: u16, fg: Color| {
            let mut dx: u16 = 0;
            for ch in text.chars() {
                let cx = x + dx;
                if cx < max_x && y < area.y + area.height {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    ctx.set(cx, y, cell);
                }
                dx += char_width(ch) as u16;
            }
        };

        // Bin value labels down the left side, lowest at the top
        let label_width = chart_area.x.saturating_sub(area.x);
        let span = chart_area.height.saturating_sub(1);
        for i in 0..=4u16 {
            let value = x_min + (x_max - x_min) * i as f64 / 4.0;
            let label = self.x_axis.format_value(value);
            let label = truncate_to_width(&label, label_width.saturating_sub(1) as usize);
            let y = chart_area.y + i * span / 4;
            put_str(label, area.x, y, area.x + label_width, self.x_axis.color);
        }

        // Count labels along the bottom, from 0 on the left to the maximum on
        // the right; a label that would touch the previous one is skipped
        let y = area.y + area.height - 1;
        let right_edge = chart_area.x + chart_area.width;
        let span = chart_area.width.saturating_sub(1);
        let mut next_free = chart_area.x;
        for i in 0..=4u16 {
            let value = y_max * i as f64 / 4.0;
            let label = if self.density || self.cumulative {
                format!("{:.2}", value)
            } else {
                format!("{:.0}", value)
            };
            let width = crate::utils::display_width(&label) as u16;
            let tick = chart_area.x + i * span / 4;
            let start = tick
                .saturating_sub(width / 2)
                .max(chart_area.x)
                .min(right_edge.saturating_sub(width));
            if start < next_free {
                continue;
            }
            put_str(&label, start, y, right_edge, self.y_axis.color);
            next_free = start + width + 1;
        }
    }

    /// Render axis labels
    pub(super) fn render_axes(&self, ctx: &mut RenderContext, area: Rect) {
        if self.bins.is_empty() {
            return;
        }

        let x_min = self.bins.first().map(|b| b.start).unwrap_or(0.0);
        let x_max = self.bins.last().map(|b| b.end).unwrap_or(1.0);
        let y_max = self.max_value();

        // Y axis labels
        let y_label_width = 6u16;
        for i in 0..=4 {
            let value = y_max * (1.0 - i as f64 / 4.0);
            let label = if self.density || self.cumulative {
                format!("{:.2}", value)
            } else {
                format!("{:.0}", value)
            };
            let y = area.y + 1 + (i as u16 * (area.height - 3) / 4);

            let label_truncated = truncate_to_width(&label, y_label_width as usize - 1);
            let mut dx: u16 = 0;
            for ch in label_truncated.chars() {
                let x = area.x + dx;
                if x < area.x + y_label_width && y < area.y + area.height {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.y_axis.color);
                    ctx.set(x, y, cell);
                }
                dx += char_width(ch) as u16;
            }
        }

        // X axis labels
        let chart_width = area.width.saturating_sub(y_label_width);
        for i in 0..=4 {
            let value = x_min + (x_max - x_min) * i as f64 / 4.0;
            let label = self.x_axis.format_value(value);
            let x = area.x + y_label_width + (i as u16 * chart_width / 4);
            let y = area.y + area.height - 1;

            let label_truncated = truncate_to_width(&label, 6);
            let mut dx: u16 = 0;
            for ch in label_truncated.chars() {
                let label_x = x + dx;
                if label_x < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.x_axis.color);
                    ctx.set(label_x, y, cell);
                }
                dx += char_width(ch) as u16;
            }
        }
    }
}
