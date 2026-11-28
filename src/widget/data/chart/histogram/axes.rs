//! Histogram axis labels: bin values and counts, for both orientations

use super::Histogram;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::char_width;
use crate::widget::data::chart::chart_render::{
    label_ticks, put_label, put_x_labels, x_label_slot,
};
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

    /// `value` as its count axis label shows it: a whole count, or two
    /// decimals for density and cumulative histograms
    fn shown_count(&self, value: f64) -> f64 {
        if self.density || self.cumulative {
            (value * 100.0).round() / 100.0
        } else {
            value.round()
        }
    }

    /// The count axis values labeled, from the maximum down to 0, with
    /// their labels: the five ticks at the maximum times 1, 3/4, 1/2, 1/4
    /// and 0, each taken at the value its label shows, and a tick whose
    /// label is the one before it again skipped. The labels fall with the
    /// ticks, so none repeats; the maximum count is whole and so is 0, so
    /// both ends are kept (#1624).
    fn count_ticks(&self) -> Vec<(f64, String)> {
        let y_max = self.max_value();
        let mut ticks: Vec<(f64, String)> = Vec::new();
        for i in 0..=4 {
            let value = self.shown_count(y_max * (1.0 - f64::from(i) / 4.0));
            let label = self.count_label(value);
            if ticks.last().is_some_and(|(_, last)| *last == label) {
                continue;
            }
            ticks.push((value, label));
        }
        ticks
    }

    /// The labels left of the chart, from the top row down: counts from
    /// the maximum down to 0 for a vertical histogram (each once), the
    /// five bin values from the lowest up for a horizontal one (no labels
    /// without bins)
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
            self.count_ticks()
                .into_iter()
                .map(|(_, label)| label)
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
        // the right, each once, centered on the last cell of a bar of that
        // count (#1624); a label that would touch the previous one is
        // skipped
        let y = area.y + area.height - 1;
        let right_edge = chart_area.x + chart_area.width;
        let mut next_free = chart_area.x;
        for (value, label) in self.count_ticks().into_iter().rev() {
            let width = crate::utils::unicode::display_width_u16(&label);
            let tick = chart_area.x + self.bar_cells(value, chart_area.width).max(1) - 1;
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

        // Count labels, left-aligned in the label column before the
        // chart's gap column, each on the top row of a bar of its count
        // and 0 on the bottom row; a label on the row of the one above it
        // is skipped, except 0, which takes that row (#1624). A wider
        // label is cut with `…`.
        let y_label_width = chart_area.x.saturating_sub(area.x);
        let columns = y_label_width.saturating_sub(1);
        let style = Cell {
            fg: Some(self.y_axis.color),
            ..Cell::default()
        };
        let rows = chart_area.height;
        let mut labels: Vec<(u16, String)> = Vec::new();
        let ticks = self.count_ticks();
        let last = ticks.len().saturating_sub(1);
        for (n, (value, label)) in ticks.into_iter().enumerate() {
            let y = chart_area.y + rows - self.bar_cells(value, rows).max(1);
            if labels.last().is_some_and(|&(above, _)| above == y) {
                if n < last {
                    continue;
                }
                labels.pop();
            }
            labels.push((y, label));
        }
        for (y, label) in labels {
            put_label(ctx, &label, (area.x, y), columns, false, style);
        }

        // Bin value labels under the chart: the first bin's start on its
        // first column and the last bin's end on its last, centered on
        // their ticks and cut with `…` to the columns between labeled
        // ticks less one (#1623)
        let span = chart_area.width.saturating_sub(1);
        let labeled = label_ticks(4, span, 2);
        let slot = x_label_slot(&labeled, span);
        let labels = labeled.into_iter().map(|(i, offset)| {
            let value = x_min + (x_max - x_min) * i as f64 / 4.0;
            (chart_area.x + offset, self.x_axis.format_value(value))
        });
        let style = Cell {
            fg: Some(self.x_axis.color),
            ..Cell::default()
        };
        let right = area.x.saturating_add(area.width);
        put_x_labels(
            ctx,
            labels,
            area.y + area.height - 1,
            (area.x, right),
            slot,
            style,
        );
    }
}
