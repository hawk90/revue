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
    /// Pinch each box in around the median's confidence interval
    pub notched: bool,
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
        notched: bool,
    ) -> Self {
        Self {
            groups,
            bounds,
            chart_area,
            box_width,
            whisker_style,
            show_outliers,
            notched,
            group_count: groups.len(),
        }
    }

    /// Map value to screen coordinate
    pub fn value_to_screen(&self, value: f64, length: u16) -> u16 {
        let (min, max) = self.bounds;
        let range = (max - min).max(1.0);
        ((value - min) / range * (length as f64 - 1.0)) as u16
    }

    /// The notch of a group: the approximate 95% confidence interval of the
    /// median, `median ± 1.57 * IQR / sqrt(n)`.
    ///
    /// `None` when notches are off, when the group was built from
    /// precomputed [`BoxStats`](super::BoxStats) (its sample size is
    /// unknown), or when the interval is empty.
    pub fn notch_bounds(&self, group: &BoxGroup, stats: &super::BoxStats) -> Option<(f64, f64)> {
        if !self.notched || group.stats.is_some() {
            return None;
        }
        let n = group.data.iter().filter(|v| v.is_finite()).count();
        if n == 0 {
            return None;
        }
        let half = 1.57 * (stats.q3 - stats.q1) / (n as f64).sqrt();
        (half > 0.0).then_some((stats.median - half, stats.median + half))
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

            // Notch: the box sides step in by one column over the median's
            // confidence interval. Only drawn when it fits strictly inside
            // the box with the median between its ends.
            let (y_top, y_bottom) = (y_q3.min(y_q1), y_q3.max(y_q1));
            let notch_rows = self.notch_bounds(group, &stats).and_then(|(low, high)| {
                let to_y = |v: f64| {
                    let pos = self
                        .value_to_screen(v, self.chart_area.height)
                        .min(self.chart_area.height - 1);
                    self.chart_area.y + self.chart_area.height - 1 - pos
                };
                let (y_high, y_low) = (
                    to_y(high).max(y_top + 1),
                    to_y(low).min(y_bottom.saturating_sub(1)),
                );
                (box_right - box_left >= 4 && y_high < y_median && y_median < y_low)
                    .then_some((y_high, y_low))
            });
            if let Some((y_high, y_low)) = notch_rows {
                let mut put = |x: u16, y: u16, ch: char| {
                    if x >= self.chart_area.x
                        && x < self.chart_area.x + self.chart_area.width
                        && y >= self.chart_area.y
                        && y < self.chart_area.y + self.chart_area.height
                    {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(color);
                        ctx.set(x, y, cell);
                    }
                };
                for y in y_high..=y_low {
                    if y == y_high {
                        put(box_left, y, '╲');
                        put(box_right, y, '╱');
                    } else if y == y_low {
                        put(box_left, y, '╱');
                        put(box_right, y, '╲');
                    } else {
                        put(box_left, y, ' ');
                        put(box_right, y, ' ');
                        put(box_left + 1, y, '│');
                        put(box_right - 1, y, '│');
                    }
                }
            }
            let (median_left, median_right) = if notch_rows.is_some() {
                (box_left + 1, box_right - 1)
            } else {
                (box_left, box_right)
            };

            // Draw median line
            for x in median_left..=median_right {
                if x >= self.chart_area.x
                    && x < self.chart_area.x + self.chart_area.width
                    && y_median >= self.chart_area.y
                    && y_median < self.chart_area.y + self.chart_area.height
                {
                    let ch = if x == median_left {
                        '├'
                    } else if x == median_right {
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

            // Notch: the top and bottom edges step in by one row over the
            // median's confidence interval. Only drawn when it fits strictly
            // inside the box with the median between its ends.
            let notch_cols = self.notch_bounds(group, &stats).and_then(|(low, high)| {
                let to_x =
                    |v: f64| area.x + self.value_to_screen(v, area.width).min(area.width - 1);
                let (x_low, x_high) = (
                    to_x(low).max(left + 1),
                    to_x(high).min(right.saturating_sub(1)),
                );
                (box_bottom - box_top >= 4 && x_low < x_median && x_median < x_high)
                    .then_some((x_low, x_high))
            });
            if let Some((x_low, x_high)) = notch_cols {
                for x in x_low..=x_high {
                    if x == x_low {
                        put(x, box_top, '╲', color);
                        put(x, box_bottom, '╱', color);
                    } else if x == x_high {
                        put(x, box_top, '╱', color);
                        put(x, box_bottom, '╲', color);
                    } else {
                        put(x, box_top, ' ', color);
                        put(x, box_bottom, ' ', color);
                        put(x, box_top + 1, '─', color);
                        put(x, box_bottom - 1, '─', color);
                    }
                }
            }
            let (median_top, median_bottom) = if notch_cols.is_some() {
                (box_top + 1, box_bottom - 1)
            } else {
                (box_top, box_bottom)
            };

            // Median line
            for y in median_top..=median_bottom {
                let ch = if y == median_top {
                    '┬'
                } else if y == median_bottom {
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
