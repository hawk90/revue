//! Histogram axis labels: bin values and counts, for both orientations

use super::Histogram;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, truncate_to_width};
use crate::widget::data::chart::chart_render::put_label;
use crate::widget::traits::RenderContext;

impl Histogram {
    /// The count axis label of `value`
    fn count_label(&self, value: f64) -> String {
        if self.density || self.cumulative {
            format!("{:.2}", value)
        } else {
            format!("{:.0}", value)
        }
    }

    /// The five labels left of the chart, from the top row down: counts
    /// from the maximum down to 0 for a vertical histogram, bin values from
    /// the lowest up for a horizontal one (no labels without bins)
    pub(super) fn left_labels(&self, horizontal: bool) -> Vec<String> {
        if self.bins.is_empty() {
            return Vec::new();
        }
        if horizontal {
            let x_min = self.bins.first().map(|b| b.start).unwrap_or(0.0);
            let x_max = self.bins.last().map(|b| b.end).unwrap_or(1.0);
            (0..=4)
                .map(|i| {
                    self.x_axis
                        .format_value(x_min + (x_max - x_min) * i as f64 / 4.0)
                })
                .collect()
        } else {
            let y_max = self.max_value();
            (0..=4)
                .map(|i| self.count_label(y_max * (1.0 - i as f64 / 4.0)))
                .collect()
        }
    }

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

        // Bin value labels down the left side, lowest at the top, in the
        // label column before the chart's gap column; a wider label is cut
        // with `…`
        let columns = chart_area.x.saturating_sub(area.x + 1);
        let span = chart_area.height.saturating_sub(1);
        let style = Cell {
            fg: Some(self.x_axis.color),
            ..Cell::default()
        };
        for (i, label) in (0u16..).zip(self.left_labels(true)) {
            let y = chart_area.y + i * span / 4;
            if y < area.y + area.height {
                put_label(ctx, &label, (area.x, y), columns, false, style);
            }
        }

        let y_max = self.max_value();
        let mut put_str = |text: &str, x: u16, y: u16, max_x: u16, fg: Color| {
            let mut dx: u16 = 0;
            for ch in text.chars() {
                let cx = x.saturating_add(dx);
                if cx >= max_x {
                    break;
                }
                if y < area.y + area.height {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    ctx.set(cx, y, cell);
                }
                dx = dx.saturating_add(char_width(ch) as u16);
            }
        };

        // Count labels along the bottom, from 0 on the left to the maximum on
        // the right; a label that would touch the previous one is skipped
        let y = area.y + area.height - 1;
        let right_edge = chart_area.x + chart_area.width;
        let span = chart_area.width.saturating_sub(1);
        let mut next_free = chart_area.x;
        for i in 0..=4u16 {
            let label = self.count_label(y_max * i as f64 / 4.0);
            let width = crate::utils::unicode::display_width_u16(&label);
            let tick = chart_area.x + i * span / 4;
            let start = tick
                .saturating_sub(width / 2)
                .max(chart_area.x)
                .min(right_edge.saturating_sub(width));
            if start < next_free {
                continue;
            }
            put_str(&label, start, y, right_edge, self.y_axis.color);
            next_free = start.saturating_add(width).saturating_add(1);
        }
    }

    /// Render axis labels: counts left of `chart_area`, bin values under it
    pub(super) fn render_axes(&self, ctx: &mut RenderContext, area: Rect, chart_area: Rect) {
        if self.bins.is_empty() {
            return;
        }

        let x_min = self.bins.first().map(|b| b.start).unwrap_or(0.0);
        let x_max = self.bins.last().map(|b| b.end).unwrap_or(1.0);

        // Y axis labels, left-aligned in the label column before the
        // chart's gap column; a wider label is cut with `…`
        let y_label_width = chart_area.x.saturating_sub(area.x);
        let columns = y_label_width.saturating_sub(1);
        let style = Cell {
            fg: Some(self.y_axis.color),
            ..Cell::default()
        };
        for (i, label) in (0u16..).zip(self.left_labels(false)) {
            let y = area.y + 1 + (i * (area.height - 3) / 4);
            if y < area.y + area.height {
                put_label(ctx, &label, (area.x, y), columns, false, style);
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
