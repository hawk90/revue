//! Box plot rendering logic

use super::group::BoxGroup;
use super::types::WhiskerStyle;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
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

    /// Render all box plots
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
                    ctx.buffer.set(group_center, y, cell);
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
                        ctx.buffer.set(x, y_whisker_low, cell);
                    }
                    // Upper whisker cap
                    if y_whisker_high >= self.chart_area.y
                        && y_whisker_high < self.chart_area.y + self.chart_area.height
                    {
                        let mut cell = Cell::new('─');
                        cell.fg = Some(color);
                        ctx.buffer.set(x, y_whisker_high, cell);
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
                    ctx.buffer.set(x, y, cell);
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
                    ctx.buffer.set(x, y_median, cell);
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
                        ctx.buffer.set(group_center, y, cell);
                    }
                }
            }
        }
    }

    /// Render axis labels
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

            for (j, ch) in label.chars().take(y_label_width as usize - 1).enumerate() {
                let x = area.x + j as u16;
                if x < area.x + y_label_width && y < area.y + area.height {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(value_axis.color);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Category axis labels (bottom)
        let n_groups = self.group_count;
        let chart_width = area.width.saturating_sub(y_label_width);
        let group_width = chart_width / n_groups as u16;

        for (i, group) in self.groups.iter().enumerate() {
            let x = area.x + y_label_width + (i as u16 * group_width) + group_width / 2;
            let y = area.y + area.height - 1;
            let label_start = x.saturating_sub(group.label.len() as u16 / 2);

            for (j, ch) in group.label.chars().enumerate() {
                let label_x = label_start + j as u16;
                if label_x >= area.x + y_label_width && label_x < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(category_axis.color);
                    ctx.buffer.set(label_x, y, cell);
                }
            }
        }
    }
}
