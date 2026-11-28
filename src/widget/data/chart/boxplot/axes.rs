//! Box plot axis labels: values and group names, for both orientations

use super::render::BoxPlotRender;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, display_width, truncate_to_width};
use crate::widget::traits::RenderContext;

impl<'a> BoxPlotRender<'a> {
    /// Render axis labels for a horizontal plot: group labels to the left of
    /// the chart area, value labels along the bottom
    pub fn render_axes_horizontal(
        &self,
        ctx: &mut RenderContext,
        area: Rect,
        value_axis: &crate::widget::data::chart::chart_common::Axis,
        category_axis: &crate::widget::data::chart::chart_common::Axis,
    ) {
        if self.groups.is_empty() {
            return;
        }

        let chart = self.chart_area;
        let mut put_str = |text: &str, x: u16, y: u16, min_x: u16, max_x: u16, fg: Color| {
            let mut dx: u16 = 0;
            for ch in text.chars() {
                let cx = x.saturating_add(dx);
                if cx >= max_x {
                    break;
                }
                if cx >= min_x && y < area.y + area.height {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    ctx.set(cx, y, cell);
                }
                dx = dx.saturating_add(char_width(ch) as u16);
            }
        };

        // Group labels, left-aligned in the label column on each group's
        // center row; `…` on the first row past them when groups are left out
        let label_width = chart.x.saturating_sub(area.x + 1) as usize;
        let (shown, group_height, marker) = self.group_bands(chart.height);
        for (i, group) in self.groups.iter().enumerate().take(shown) {
            let y = chart.y + (i as u16 * group_height) + group_height / 2;
            let label = truncate_to_width(&group.label, label_width);
            put_str(
                label,
                area.x,
                y,
                area.x,
                chart.x.saturating_sub(1),
                category_axis.color,
            );
        }
        if let Some(offset) = marker {
            put_str(
                "…",
                area.x,
                chart.y + offset,
                area.x,
                chart.x.saturating_sub(1),
                category_axis.color,
            );
        }

        // Five value labels under the chart, from min (left) to max (right),
        // centered on their tick and kept inside the chart columns. A label
        // that would touch the previous one is skipped.
        let (min, max) = self.bounds;
        let y = chart.y + chart.height;
        let right_edge = chart.x + chart.width;
        let span = chart.width.saturating_sub(1);
        let mut next_free = chart.x;
        for i in 0..=4u16 {
            let value = min + (max - min) * i as f64 / 4.0;
            let label = value_axis.format_value(value);
            let width = u16::try_from(display_width(&label)).unwrap_or(u16::MAX);
            let tick = chart.x + i * span / 4;
            let start = tick
                .saturating_sub(width / 2)
                .max(chart.x)
                .min(right_edge.saturating_sub(width));
            if start < next_free {
                continue;
            }
            put_str(&label, start, y, chart.x, right_edge, value_axis.color);
            next_free = start.saturating_add(width).saturating_add(1);
        }
    }

    /// Render axis labels for a vertical plot
    pub fn render_axes(
        &self,
        ctx: &mut RenderContext,
        area: Rect,
        value_axis: &crate::widget::data::chart::chart_common::Axis,
        category_axis: &crate::widget::data::chart::chart_common::Axis,
    ) {
        if self.groups.is_empty() {
            return;
        }

        let (min, max) = self.bounds;

        // Value axis labels (left side)
        let y_label_width = 6u16;
        for i in 0..=4 {
            let value = max - (max - min) * i as f64 / 4.0;
            let label = value_axis.format_value(value);
            let y = area.y + 1 + (i as u16 * (area.height - 3) / 4);

            let label_truncated = truncate_to_width(&label, y_label_width as usize - 1);
            let mut dx: u16 = 0;
            for ch in label_truncated.chars() {
                let x = area.x + dx;
                if x < area.x + y_label_width && y < area.y + area.height {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(value_axis.color);
                    ctx.set(x, y, cell);
                }
                dx += char_width(ch) as u16;
            }
        }

        // Category axis labels (bottom), centered under each group's band
        // of the chart columns. When groups are left out, `…` takes the
        // column after the last band and labels stop before it.
        let chart = self.chart_area;
        let (shown, group_width, marker) = self.group_bands(chart.width);
        let y = area.y + area.height - 1;
        let right_edge = match marker {
            Some(offset) => chart.x + offset,
            None => area.x + area.width,
        };
        let mut put = |x: u16, ch: char| {
            let mut cell = Cell::new(ch);
            cell.fg = Some(category_axis.color);
            ctx.set(x, y, cell);
        };

        for (i, group) in self.groups.iter().enumerate().take(shown) {
            let x = chart.x + (i as u16 * group_width) + group_width / 2;
            let half = u16::try_from(display_width(&group.label) / 2).unwrap_or(u16::MAX);
            let label_start = x.saturating_sub(half);

            let mut dx: u16 = 0;
            for ch in group.label.chars() {
                let label_x = label_start.saturating_add(dx);
                let w = char_width(ch) as u16;
                if label_x >= right_edge {
                    break;
                }
                if label_x >= chart.x && label_x.saturating_add(w) <= right_edge {
                    put(label_x, ch);
                }
                dx = dx.saturating_add(w);
            }
        }
        if marker.is_some() {
            put(right_edge, '…');
        }
    }
}
