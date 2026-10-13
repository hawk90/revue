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
                let cx = x + dx;
                if cx >= min_x && cx < max_x && y < area.y + area.height {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    ctx.set(cx, y, cell);
                }
                dx += char_width(ch) as u16;
            }
        };

        // Group labels, left-aligned in the label column on each group's center row
        let label_width = chart.x.saturating_sub(area.x + 1) as usize;
        let group_height = chart.height / self.group_count as u16;
        for (i, group) in self.groups.iter().enumerate() {
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
            let width = display_width(&label) as u16;
            let tick = chart.x + i * span / 4;
            let start = tick
                .saturating_sub(width / 2)
                .max(chart.x)
                .min(right_edge.saturating_sub(width));
            if start < next_free {
                continue;
            }
            put_str(&label, start, y, chart.x, right_edge, value_axis.color);
            next_free = start + width + 1;
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

        // Category axis labels (bottom)
        let n_groups = self.group_count;
        let chart_width = area.width.saturating_sub(y_label_width);
        let group_width = chart_width / n_groups as u16;

        for (i, group) in self.groups.iter().enumerate() {
            let x = area.x + y_label_width + (i as u16 * group_width) + group_width / 2;
            let y = area.y + area.height - 1;
            let label_start = x.saturating_sub(display_width(&group.label) as u16 / 2);

            let mut dx: u16 = 0;
            for ch in group.label.chars() {
                let label_x = label_start + dx;
                if label_x >= area.x + y_label_width && label_x < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(category_axis.color);
                    ctx.set(label_x, y, cell);
                }
                dx += char_width(ch) as u16;
            }
        }
    }
}
