//! Box plot rendering logic

use super::group::BoxGroup;
use super::types::WhiskerStyle;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, display_width, truncate_to_width};
use crate::widget::traits::RenderContext;

/// Box plot rendering state
pub struct BoxPlotRender<'a> {
    /// Box groups
    pub groups: &'a [BoxGroup],
    /// Value to screen coordinate mapping state
    pub bounds: (f64, f64),
    pub chart_area: Rect,
    pub box_width: f64,
    pub whisker_style: WhiskerStyle,
    pub show_outliers: bool,
    pub group_count: usize,
}

impl<'a> BoxPlotRender<'a> {
    /// Create new render state
    pub fn new(
        groups: &'a [BoxGroup],
        bounds: (f64, f64),
        chart_area: Rect,
        box_width: f64,
        whisker_style: WhiskerStyle,
        show_outliers: bool,
    ) -> Self {
        Self {
            groups,
            bounds,
            chart_area,
            box_width,
            whisker_style,
            show_outliers,
            group_count: groups.len(),
        }
    }

    /// Map value to screen coordinate
    pub fn value_to_screen(&self, value: f64, length: u16) -> u16 {
        let (min, max) = self.bounds;
        let range = (max - min).max(1.0);
        ((value - min) / range * (length as f64 - 1.0)) as u16
    }

    /// Get color for group at index
    pub fn group_color(
        &self,
        index: usize,
        colors: &crate::widget::data::chart::chart_common::ColorScheme,
    ) -> Color {
        self.groups
            .get(index)
            .and_then(|g| g.color)
            .unwrap_or_else(|| colors.get(index))
    }

    /// Render all box plots vertically: values run bottom to top and each
    /// group gets a band of columns
    pub fn render_boxes(
        &self,
        ctx: &mut RenderContext,
        colors: &crate::widget::data::chart::chart_common::ColorScheme,
    ) {
        if self.groups.is_empty() {
            return;
        }

        let n_groups = self.group_count;
        let group_width = self.chart_area.width / n_groups as u16;
        let box_width = (group_width as f64 * self.box_width) as u16;

        for (i, group) in self.groups.iter().enumerate() {
            let Some(stats) = group.get_stats(self.whisker_style) else {
                continue;
            };

            let color = self.group_color(i, colors);
            let group_center = self.chart_area.x + (i as u16 * group_width) + group_width / 2;
            let box_left = group_center.saturating_sub(box_width / 2);
            let box_right = box_left + box_width;

            // Calculate y positions (inverted because y increases downward)
            let y_whisker_low = self.chart_area.y + self.chart_area.height
                - 1
                - self.value_to_screen(stats.whisker_low, self.chart_area.height);
            let y_q1 = self.chart_area.y + self.chart_area.height
                - 1
                - self.value_to_screen(stats.q1, self.chart_area.height);
            let y_median = self.chart_area.y + self.chart_area.height
                - 1
                - self.value_to_screen(stats.median, self.chart_area.height);
            let y_q3 = self.chart_area.y + self.chart_area.height
                - 1
                - self.value_to_screen(stats.q3, self.chart_area.height);
            let y_whisker_high = self.chart_area.y + self.chart_area.height
                - 1
                - self.value_to_screen(stats.whisker_high, self.chart_area.height);

            // Draw whiskers (vertical line in center)
            for y in y_whisker_low.min(y_whisker_high)..=y_whisker_low.max(y_whisker_high) {
                if y >= self.chart_area.y && y < self.chart_area.y + self.chart_area.height {
                    let mut cell = Cell::new('│');
                    cell.fg = Some(color);
                    ctx.set(group_center, y, cell);
                }
            }

            // Draw whisker caps
            for x in box_left..=box_right {
                if x >= self.chart_area.x && x < self.chart_area.x + self.chart_area.width {
                    // Lower whisker cap
                    if y_whisker_low >= self.chart_area.y
                        && y_whisker_low < self.chart_area.y + self.chart_area.height
                    {
                        let mut cell = Cell::new('─');
                        cell.fg = Some(color);
                        ctx.set(x, y_whisker_low, cell);
                    }
                    // Upper whisker cap
                    if y_whisker_high >= self.chart_area.y
                        && y_whisker_high < self.chart_area.y + self.chart_area.height
                    {
                        let mut cell = Cell::new('─');
                        cell.fg = Some(color);
                        ctx.set(x, y_whisker_high, cell);
                    }
                }
            }

            // Draw box (Q1 to Q3)
            for y in y_q3.min(y_q1)..=y_q3.max(y_q1) {
                if y < self.chart_area.y || y >= self.chart_area.y + self.chart_area.height {
                    continue;
                }
                for x in box_left..=box_right {
                    if x < self.chart_area.x || x >= self.chart_area.x + self.chart_area.width {
                        continue;
                    }

                    let ch = if y == y_q1.min(y_q3) {
                        if x == box_left {
                            '┌'
                        } else if x == box_right {
                            '┐'
                        } else {
                            '─'
                        }
                    } else if y == y_q1.max(y_q3) {
                        if x == box_left {
                            '└'
                        } else if x == box_right {
                            '┘'
                        } else {
                            '─'
                        }
                    } else if x == box_left || x == box_right {
                        '│'
                    } else {
                        ' '
                    };

                    let mut cell = Cell::new(ch);
                    cell.fg = Some(color);
                    ctx.set(x, y, cell);
                }
            }

            // Draw median line
            for x in box_left..=box_right {
                if x >= self.chart_area.x
                    && x < self.chart_area.x + self.chart_area.width
                    && y_median >= self.chart_area.y
                    && y_median < self.chart_area.y + self.chart_area.height
                {
                    let ch = if x == box_left {
                        '├'
                    } else if x == box_right {
                        '┤'
                    } else {
                        '─'
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::WHITE);
                    ctx.set(x, y_median, cell);
                }
            }

            // Draw outliers
            if self.show_outliers {
                for &outlier in &stats.outliers {
                    let y = self.chart_area.y + self.chart_area.height
                        - 1
                        - self.value_to_screen(outlier, self.chart_area.height);
                    if y >= self.chart_area.y
                        && y < self.chart_area.y + self.chart_area.height
                        && group_center >= self.chart_area.x
                        && group_center < self.chart_area.x + self.chart_area.width
                    {
                        let mut cell = Cell::new('○');
                        cell.fg = Some(color);
                        ctx.set(group_center, y, cell);
                    }
                }
            }
        }
    }

    /// Render all box plots horizontally: values run left to right and each
    /// group gets a band of rows
    pub fn render_boxes_horizontal(
        &self,
        ctx: &mut RenderContext,
        colors: &crate::widget::data::chart::chart_common::ColorScheme,
    ) {
        if self.groups.is_empty() {
            return;
        }

        let area = self.chart_area;
        let in_area = |x: u16, y: u16| {
            x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
        };
        let n_groups = self.group_count;
        let group_height = area.height / n_groups as u16;
        let box_height = (group_height as f64 * self.box_width) as u16;
        let to_x = |value: f64| area.x + self.value_to_screen(value, area.width);

        for (i, group) in self.groups.iter().enumerate() {
            let Some(stats) = group.get_stats(self.whisker_style) else {
                continue;
            };

            let color = self.group_color(i, colors);
            let group_center = area.y + (i as u16 * group_height) + group_height / 2;
            let box_top = group_center.saturating_sub(box_height / 2);
            let box_bottom = box_top + box_height;

            let x_whisker_low = to_x(stats.whisker_low);
            let x_q1 = to_x(stats.q1);
            let x_median = to_x(stats.median);
            let x_q3 = to_x(stats.q3);
            let x_whisker_high = to_x(stats.whisker_high);

            let mut put = |x: u16, y: u16, ch: char, fg: Color| {
                if in_area(x, y) {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(fg);
                    ctx.set(x, y, cell);
                }
            };

            // Whisker (horizontal line through the center row)
            for x in x_whisker_low.min(x_whisker_high)..=x_whisker_low.max(x_whisker_high) {
                put(x, group_center, '─', color);
            }

            // Whisker caps
            for y in box_top..=box_bottom {
                put(x_whisker_low, y, '│', color);
                put(x_whisker_high, y, '│', color);
            }

            // Box (Q1 to Q3)
            let (left, right) = (x_q1.min(x_q3), x_q1.max(x_q3));
            for y in box_top..=box_bottom {
                for x in left..=right {
                    let ch = if y == box_top {
                        if x == left {
                            '┌'
                        } else if x == right {
                            '┐'
                        } else {
                            '─'
                        }
                    } else if y == box_bottom {
                        if x == left {
                            '└'
                        } else if x == right {
                            '┘'
                        } else {
                            '─'
                        }
                    } else if x == left || x == right {
                        '│'
                    } else {
                        ' '
                    };
                    put(x, y, ch, color);
                }
            }

            // Median line
            for y in box_top..=box_bottom {
                let ch = if y == box_top {
                    '┬'
                } else if y == box_bottom {
                    '┴'
                } else {
                    '│'
                };
                put(x_median, y, ch, Color::WHITE);
            }

            // Outliers
            if self.show_outliers {
                for &outlier in &stats.outliers {
                    put(to_x(outlier), group_center, '○', color);
                }
            }
        }
    }

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

// KEEP HERE - accesses private fields (RenderContext::buffer)
// Tests extracted to tests/widget/data/chart_boxplot_render.rs
