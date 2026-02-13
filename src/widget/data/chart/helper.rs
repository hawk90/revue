//! Helper functions for chart widget

use super::chart_common::{Axis, AxisFormat, LegendPosition, Marker};
use super::types::{ChartType, LineStyle, Series};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Line segment for drawing
pub(super) struct LineSegment {
    x0: u16,
    y0: u16,
    x1: u16,
    y1: u16,
    color: Color,
    style: LineStyle,
}

/// Chart widget
#[derive(Debug)]
pub struct Chart {
    /// Chart title
    title: Option<String>,
    /// Data series
    series: Vec<Series>,
    /// X axis
    x_axis: Axis,
    /// Y axis
    y_axis: Axis,
    /// Legend position
    legend: LegendPosition,
    /// Background color
    bg_color: Option<Color>,
    /// Border color
    border_color: Option<Color>,
    /// Use Braille for higher resolution
    braille_mode: bool,
    /// Widget properties
    props: WidgetProps,
}

impl Chart {
    /// Create a new chart
    pub fn new() -> Self {
        Self {
            title: None,
            series: Vec::new(),
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            legend: LegendPosition::TopRight,
            bg_color: None,
            border_color: None,
            braille_mode: false,
            props: WidgetProps::new(),
        }
    }

    /// Set chart title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add a series
    pub fn series(mut self, series: Series) -> Self {
        self.series.push(series);
        self
    }

    /// Add multiple series
    pub fn series_vec(mut self, series: Vec<Series>) -> Self {
        self.series.extend(series);
        self
    }

    /// Set X axis
    pub fn x_axis(mut self, axis: Axis) -> Self {
        self.x_axis = axis;
        self
    }

    /// Set Y axis
    pub fn y_axis(mut self, axis: Axis) -> Self {
        self.y_axis = axis;
        self
    }

    /// Set legend position
    pub fn legend(mut self, position: LegendPosition) -> Self {
        self.legend = position;
        self
    }

    /// Hide legend
    pub fn no_legend(mut self) -> Self {
        self.legend = LegendPosition::None;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set border color
    pub fn border(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Enable Braille mode for higher resolution
    pub fn braille(mut self) -> Self {
        self.braille_mode = true;
        self
    }

    /// Compute data bounds
    ///
    /// Returns (x_min, x_max, y_min, y_max) with safe defaults for edge cases.
    fn compute_bounds(&self) -> (f64, f64, f64, f64) {
        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;
        let mut has_data = false;

        for series in &self.series {
            for &(x, y) in &series.data {
                // Skip NaN and infinite values
                if !x.is_finite() || !y.is_finite() {
                    continue;
                }
                has_data = true;
                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }

        // Default bounds for empty data
        if !has_data {
            x_min = self.x_axis.min.unwrap_or(0.0);
            x_max = self.x_axis.max.unwrap_or(1.0);
            y_min = self.y_axis.min.unwrap_or(0.0);
            y_max = self.y_axis.max.unwrap_or(1.0);
        } else {
            // Use axis bounds if specified
            x_min = self.x_axis.min.unwrap_or(x_min);
            x_max = self.x_axis.max.unwrap_or(x_max);
            y_min = self.y_axis.min.unwrap_or(y_min);
            y_max = self.y_axis.max.unwrap_or(y_max);
        }

        // Ensure non-zero ranges (avoid division by zero)
        const EPSILON: f64 = 1e-10;
        if (x_max - x_min).abs() < EPSILON {
            let center = (x_max + x_min) / 2.0;
            x_min = center - 0.5;
            x_max = center + 0.5;
        }
        if (y_max - y_min).abs() < EPSILON {
            let center = (y_max + y_min) / 2.0;
            y_min = center - 0.5;
            y_max = center + 0.5;
        }

        // Add padding for auto bounds
        let y_range = y_max - y_min;
        let y_min = if self.y_axis.min.is_none() {
            y_min - y_range * 0.05
        } else {
            y_min
        };
        let y_max = if self.y_axis.max.is_none() {
            y_max + y_range * 0.05
        } else {
            y_max
        };

        (x_min, x_max, y_min, y_max)
    }

    /// Format axis label
    fn format_label(&self, value: f64, format: &AxisFormat) -> String {
        match format {
            AxisFormat::Auto => {
                if value.abs() >= 1000.0 || (value != 0.0 && value.abs() < 0.01) {
                    format!("{:.1e}", value)
                } else if value.fract() == 0.0 {
                    format!("{:.0}", value)
                } else {
                    format!("{:.2}", value)
                }
            }
            AxisFormat::Integer => format!("{:.0}", value),
            AxisFormat::Fixed(decimals) => format!("{:.1$}", value, *decimals),
            AxisFormat::Percent => format!("{:.0}%", value * 100.0),
            AxisFormat::Custom(fmt) => fmt.replace("{}", &value.to_string()),
        }
    }

    /// Map data coordinates to screen coordinates
    fn map_point(
        &self,
        x: f64,
        y: f64,
        bounds: (f64, f64, f64, f64),
        chart_area: (u16, u16, u16, u16),
    ) -> (u16, u16) {
        let (x_min, x_max, y_min, y_max) = bounds;
        let (cx, cy, cw, ch) = chart_area;

        let x_range = x_max - x_min;
        let y_range = y_max - y_min;

        let px = if x_range > 0.0 {
            cx + ((x - x_min) / x_range * (cw as f64 - 1.0)) as u16
        } else {
            cx + cw / 2
        };

        let py = if y_range > 0.0 {
            cy + ch - 1 - ((y - y_min) / y_range * (ch as f64 - 1.0)) as u16
        } else {
            cy + ch / 2
        };

        (px, py)
    }

    /// Get line character based on direction
    fn get_line_char(&self, dx: i32, dy: i32) -> char {
        match (dx.signum(), dy.signum()) {
            (1, 0) | (-1, 0) => '─',  // Horizontal
            (0, 1) | (0, -1) => '│',  // Vertical
            (1, -1) | (-1, 1) => '╱', // Up-right or down-left
            (1, 1) | (-1, -1) => '╲', // Down-right or up-left
            _ => '·',
        }
    }

    /// Draw line between two points using Bresenham's algorithm
    fn draw_line(&self, ctx: &mut RenderContext, seg: &LineSegment, bounds: &Rect) {
        let LineSegment {
            x0,
            y0,
            x1,
            y1,
            color,
            style,
        } = *seg;

        let dx = (x1 as i32 - x0 as i32).abs();
        let dy = (y1 as i32 - y0 as i32).abs();
        let sx = if x0 < x1 { 1i32 } else { -1i32 };
        let sy = if y0 < y1 { 1i32 } else { -1i32 };
        let mut err = dx - dy;

        let mut x = x0 as i32;
        let mut y = y0 as i32;
        let mut step = 0;

        loop {
            // Check bounds
            if x >= bounds.x as i32
                && x < (bounds.x + bounds.width) as i32
                && y >= bounds.y as i32
                && y < (bounds.y + bounds.height) as i32
            {
                let draw = match style {
                    LineStyle::Solid => true,
                    LineStyle::Dashed => (step / 3) % 2 == 0,
                    LineStyle::Dotted => step % 2 == 0,
                    LineStyle::None => false,
                };

                if draw {
                    let ch = self.get_line_char(x1 as i32 - x0 as i32, y1 as i32 - y0 as i32);
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(color);
                    ctx.buffer.set(x as u16, y as u16, cell);
                }
            }

            if x == x1 as i32 && y == y1 as i32 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
            step += 1;
        }
    }

    /// Draw area fill
    fn draw_area_fill(
        &self,
        ctx: &mut RenderContext,
        points: &[(u16, u16)],
        fill_color: Color,
        chart_area: (u16, u16, u16, u16),
        y_bottom: u16,
    ) {
        let (cx, cy, cw, ch) = chart_area;

        for window in points.windows(2) {
            let (x0, y0) = window[0];
            let (x1, y1) = window[1];

            // Fill vertical columns between the two points
            for x in x0..=x1 {
                if x < cx || x >= cx + cw {
                    continue;
                }

                // Interpolate y value
                let t = if x1 != x0 {
                    (x - x0) as f64 / (x1 - x0) as f64
                } else {
                    0.0
                };
                let y_line = y0 as f64 + t * (y1 as f64 - y0 as f64);

                let y_start = y_line.ceil() as u16;
                let y_end = y_bottom.min(cy + ch - 1);

                for y in y_start..=y_end {
                    if y >= cy && y < cy + ch {
                        // Use a transparent fill character
                        let gradient = (y - y_start) as f64 / (y_end - y_start).max(1) as f64;
                        let ch = if gradient < 0.33 {
                            '░'
                        } else if gradient < 0.66 {
                            '▒'
                        } else {
                            '▓'
                        };
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(fill_color);
                        ctx.buffer.set(x, y, cell);
                    }
                }
            }
        }
    }
}

impl Default for Chart {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Chart {
    crate::impl_view_meta!("Chart");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 10 || area.height < 5 {
            return;
        }

        // Fill background
        if let Some(bg) = self.bg_color {
            for y in area.y..area.y + area.height {
                for x in area.x..area.x + area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Draw border
        if let Some(border_color) = self.border_color {
            // Top and bottom
            for x in area.x..area.x + area.width {
                let mut top = Cell::new('─');
                top.fg = Some(border_color);
                ctx.buffer.set(x, area.y, top);

                let mut bottom = Cell::new('─');
                bottom.fg = Some(border_color);
                ctx.buffer.set(x, area.y + area.height - 1, bottom);
            }
            // Left and right
            for y in area.y..area.y + area.height {
                let mut left = Cell::new('│');
                left.fg = Some(border_color);
                ctx.buffer.set(area.x, y, left);

                let mut right = Cell::new('│');
                right.fg = Some(border_color);
                ctx.buffer.set(area.x + area.width - 1, y, right);
            }
            // Corners
            let corners = [
                (area.x, area.y, '┌'),
                (area.x + area.width - 1, area.y, '┐'),
                (area.x, area.y + area.height - 1, '└'),
                (area.x + area.width - 1, area.y + area.height - 1, '┘'),
            ];
            for (x, y, ch) in corners {
                let mut cell = Cell::new(ch);
                cell.fg = Some(border_color);
                ctx.buffer.set(x, y, cell);
            }
        }

        // Calculate layout
        let has_border = self.border_color.is_some();
        let has_title = self.title.is_some();
        let y_label_width = 8u16; // Space for Y axis labels
        let x_label_height = 2u16; // Space for X axis labels

        let inner_x = area.x + if has_border { 1 } else { 0 } + y_label_width;
        let inner_y = area.y + if has_border { 1 } else { 0 } + if has_title { 1 } else { 0 };
        let inner_w = area
            .width
            .saturating_sub(y_label_width + if has_border { 2 } else { 0 });
        let inner_h = area.height.saturating_sub(
            x_label_height + if has_border { 2 } else { 0 } + if has_title { 1 } else { 0 },
        );

        if inner_w < 5 || inner_h < 3 {
            return;
        }

        let chart_bounds = Rect::new(inner_x, inner_y, inner_w, inner_h);

        // Draw title
        if let Some(ref title) = self.title {
            let title_x = area.x + (area.width.saturating_sub(title.len() as u16)) / 2;
            let title_y = area.y + if has_border { 1 } else { 0 };
            for (i, ch) in title.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.modifier |= crate::render::Modifier::BOLD;
                ctx.buffer.set(title_x + i as u16, title_y, cell);
            }
        }

        // Compute bounds
        let bounds = self.compute_bounds();
        let (x_min, x_max, y_min, y_max) = bounds;

        // Draw Y axis labels
        let y_label_x = area.x + if has_border { 1 } else { 0 };
        for i in 0..=self.y_axis.ticks {
            let t = i as f64 / self.y_axis.ticks as f64;
            let value = y_min + t * (y_max - y_min);
            let label = self.format_label(value, &self.y_axis.format);
            let y = inner_y + inner_h - 1 - ((t * (inner_h as f64 - 1.0)) as u16);

            // Right-align label
            let label_start = y_label_x + y_label_width.saturating_sub(label.len() as u16 + 1);
            for (j, ch) in label.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.y_axis.color);
                ctx.buffer.set(label_start + j as u16, y, cell);
            }

            // Draw grid line
            if self.y_axis.grid && i > 0 && i < self.y_axis.ticks {
                for x in inner_x..inner_x + inner_w {
                    let mut cell = Cell::new('┄');
                    cell.fg = Some(Color::rgb(50, 50, 50));
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Draw X axis labels
        let x_label_y = inner_y + inner_h;
        for i in 0..=self.x_axis.ticks {
            let t = i as f64 / self.x_axis.ticks as f64;
            let value = x_min + t * (x_max - x_min);
            let label = self.format_label(value, &self.x_axis.format);
            let x = inner_x + (t * (inner_w as f64 - 1.0)) as u16;

            // Center label
            let label_start = x.saturating_sub(label.len() as u16 / 2);
            for (j, ch) in label.chars().enumerate() {
                let px = label_start + j as u16;
                if px >= area.x && px < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.x_axis.color);
                    ctx.buffer.set(px, x_label_y, cell);
                }
            }

            // Draw grid line
            if self.x_axis.grid && i > 0 && i < self.x_axis.ticks {
                for y in inner_y..inner_y + inner_h {
                    let mut cell = Cell::new('┊');
                    cell.fg = Some(Color::rgb(50, 50, 50));
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Draw axis titles
        if let Some(ref title) = self.y_axis.title {
            // Draw vertically on the left
            let y_start = inner_y + (inner_h.saturating_sub(title.len() as u16)) / 2;
            for (i, ch) in title.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.y_axis.color);
                ctx.buffer.set(y_label_x, y_start + i as u16, cell);
            }
        }

        if let Some(ref title) = self.x_axis.title {
            let x_start = inner_x + (inner_w.saturating_sub(title.len() as u16)) / 2;
            let y = x_label_y + 1;
            if y < area.y + area.height {
                for (i, ch) in title.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.x_axis.color);
                    ctx.buffer.set(x_start + i as u16, y, cell);
                }
            }
        }

        // Draw each series
        let y_bottom = inner_y + inner_h - 1;

        for series in &self.series {
            if series.data.is_empty() {
                continue;
            }

            // Map all points to screen coordinates
            let chart_area = (
                chart_bounds.x,
                chart_bounds.y,
                chart_bounds.width,
                chart_bounds.height,
            );
            let screen_points: Vec<(u16, u16)> = series
                .data
                .iter()
                .map(|&(x, y)| self.map_point(x, y, bounds, chart_area))
                .collect();

            // Draw area fill first (if applicable)
            if matches!(series.chart_type, ChartType::Area) {
                if let Some(fill_color) = series.fill_color {
                    self.draw_area_fill(ctx, &screen_points, fill_color, chart_area, y_bottom);
                }
            }

            // Draw lines
            if !matches!(series.line_style, LineStyle::None) {
                match series.chart_type {
                    ChartType::Line | ChartType::Area => {
                        for window in screen_points.windows(2) {
                            let (x0, y0) = window[0];
                            let (x1, y1) = window[1];
                            let seg = LineSegment {
                                x0,
                                y0,
                                x1,
                                y1,
                                color: series.color,
                                style: series.line_style,
                            };
                            self.draw_line(ctx, &seg, &chart_bounds);
                        }
                    }
                    ChartType::StepAfter => {
                        for window in screen_points.windows(2) {
                            let (x0, y0) = window[0];
                            let (x1, y1) = window[1];
                            // Horizontal then vertical
                            let horiz = LineSegment {
                                x0,
                                y0,
                                x1,
                                y1: y0,
                                color: series.color,
                                style: series.line_style,
                            };
                            self.draw_line(ctx, &horiz, &chart_bounds);
                            let vert = LineSegment {
                                x0: x1,
                                y0,
                                x1,
                                y1,
                                color: series.color,
                                style: series.line_style,
                            };
                            self.draw_line(ctx, &vert, &chart_bounds);
                        }
                    }
                    ChartType::StepBefore => {
                        for window in screen_points.windows(2) {
                            let (x0, y0) = window[0];
                            let (x1, y1) = window[1];
                            // Vertical then horizontal
                            let vert = LineSegment {
                                x0,
                                y0,
                                x1: x0,
                                y1,
                                color: series.color,
                                style: series.line_style,
                            };
                            self.draw_line(ctx, &vert, &chart_bounds);
                            let horiz = LineSegment {
                                x0,
                                y0: y1,
                                x1,
                                y1,
                                color: series.color,
                                style: series.line_style,
                            };
                            self.draw_line(ctx, &horiz, &chart_bounds);
                        }
                    }
                    ChartType::Scatter => {}
                }
            }

            // Draw markers
            if !matches!(series.marker, Marker::None) {
                let marker_char = series.marker.char();
                for &(x, y) in &screen_points {
                    if x >= inner_x
                        && x < inner_x + inner_w
                        && y >= inner_y
                        && y < inner_y + inner_h
                    {
                        let mut cell = Cell::new(marker_char);
                        cell.fg = Some(series.color);
                        ctx.buffer.set(x, y, cell);
                    }
                }
            }
        }

        // Draw legend
        if !matches!(self.legend, LegendPosition::None) && !self.series.is_empty() {
            let legend_width = self
                .series
                .iter()
                .map(|s| s.name.len() + 4)
                .max()
                .unwrap_or(10) as u16;
            let legend_height = self.series.len() as u16 + 2;

            let (legend_x, legend_y) = match self.legend {
                LegendPosition::TopLeft => (inner_x + 1, inner_y + 1),
                LegendPosition::TopCenter => (inner_x + (inner_w - legend_width) / 2, inner_y + 1),
                LegendPosition::TopRight => (inner_x + inner_w - legend_width - 1, inner_y + 1),
                LegendPosition::BottomLeft => (inner_x + 1, inner_y + inner_h - legend_height - 1),
                LegendPosition::BottomCenter => (
                    inner_x + (inner_w - legend_width) / 2,
                    inner_y + inner_h - legend_height - 1,
                ),
                LegendPosition::BottomRight => (
                    inner_x + inner_w - legend_width - 1,
                    inner_y + inner_h - legend_height - 1,
                ),
                LegendPosition::Left => (inner_x + 1, inner_y + (inner_h - legend_height) / 2),
                LegendPosition::Right => (
                    inner_x + inner_w - legend_width - 1,
                    inner_y + (inner_h - legend_height) / 2,
                ),
                // None is filtered out by the if condition above, but provide fallback
                LegendPosition::None => (inner_x + 1, inner_y + 1),
            };

            // Draw legend background
            for dy in 0..legend_height {
                for dx in 0..legend_width {
                    let x = legend_x + dx;
                    let y = legend_y + dy;
                    if x < inner_x + inner_w && y < inner_y + inner_h {
                        let ch = if dy == 0 && dx == 0 {
                            '┌'
                        } else if dy == 0 && dx == legend_width - 1 {
                            '┐'
                        } else if dy == legend_height - 1 && dx == 0 {
                            '└'
                        } else if dy == legend_height - 1 && dx == legend_width - 1 {
                            '┘'
                        } else if dy == 0 || dy == legend_height - 1 {
                            '─'
                        } else if dx == 0 || dx == legend_width - 1 {
                            '│'
                        } else {
                            ' '
                        };
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(Color::rgb(80, 80, 80));
                        cell.bg = self.bg_color.or(Some(Color::rgb(20, 20, 20)));
                        ctx.buffer.set(x, y, cell);
                    }
                }
            }

            // Draw legend entries
            for (i, series) in self.series.iter().enumerate() {
                let y = legend_y + 1 + i as u16;
                if y >= inner_y + inner_h - 1 {
                    break;
                }

                // Color indicator
                let mut indicator = Cell::new('■');
                indicator.fg = Some(series.color);
                ctx.buffer.set(legend_x + 1, y, indicator);

                // Series name
                for (j, ch) in series.name.chars().enumerate() {
                    let x = legend_x + 3 + j as u16;
                    if x < legend_x + legend_width - 1 {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(Color::WHITE);
                        cell.bg = self.bg_color.or(Some(Color::rgb(20, 20, 20)));
                        ctx.buffer.set(x, y, cell);
                    }
                }
            }
        }
    }
}

impl_styled_view!(Chart);
impl_props_builders!(Chart);

/// Helper function to create a chart
pub fn chart() -> Chart {
    Chart::new()
}

/// Quick line chart from data
pub fn line_chart(data: &[f64]) -> Chart {
    Chart::new().series(Series::new("Data").data_y(data).line())
}

/// Quick scatter plot from data
pub fn scatter_plot(data: &[(f64, f64)]) -> Chart {
    Chart::new().series(Series::new("Data").data(data.to_vec()).scatter())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::style::Color;

    // =========================================================================
    // Private method tests
    // =========================================================================

    #[test]
    fn test_compute_bounds_empty() {
        let chart = Chart::new();
        let (x_min, x_max, y_min, y_max) = chart.compute_bounds();
        // Default bounds with Y padding applied
        assert_eq!(x_min, 0.0);
        assert_eq!(x_max, 1.0);
        // Y gets 5% padding: 0 - (1-0)*0.05 = -0.05
        assert!((y_min + 0.05).abs() < 0.001);
        assert!((y_max - 1.05).abs() < 0.001);
    }

    #[test]
    fn test_compute_bounds_single_point() {
        let chart = Chart::new().series(Series::new("Test").data(vec![(5.0, 10.0)]));
        let (x_min, x_max, y_min, y_max) = chart.compute_bounds();
        // EPSILON padding adds +/- 0.5 for zero range, then Y padding applied
        assert_eq!(x_min, 4.5);
        assert_eq!(x_max, 5.5);
        // Y: (10.0 - 0.5) = 9.5, then padding: 9.5 - (1.0 * 0.05) = 9.45
        assert!((y_min - 9.45).abs() < 0.01);
        assert!((y_max - 10.55).abs() < 0.01);
    }

    #[test]
    fn test_compute_bounds_multiple_points() {
        let chart =
            Chart::new().series(Series::new("Test").data(vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]));
        let (x_min, x_max, y_min, y_max) = chart.compute_bounds();
        // X has no padding
        assert_eq!(x_min, 1.0);
        assert_eq!(x_max, 5.0);
        // Y gets 5% padding: 2 - (6-2)*0.05 = 1.8, 6 + (6-2)*0.05 = 6.2
        assert!((y_min - 1.8).abs() < 0.01);
        assert!((y_max - 6.2).abs() < 0.01);
    }

    #[test]
    fn test_compute_bounds_with_nan() {
        let chart = Chart::new().series(Series::new("Test").data(vec![
            (1.0, 2.0),
            (f64::NAN, 4.0),
            (5.0, 6.0),
        ]));
        let (x_min, x_max, _y_min, _y_max) = chart.compute_bounds();
        // NaN values should be filtered out
        assert_eq!(x_min, 1.0);
        assert_eq!(x_max, 5.0);
    }

    #[test]
    fn test_compute_bounds_with_infinity() {
        let chart = Chart::new().series(Series::new("Test").data(vec![
            (1.0, 2.0),
            (f64::INFINITY, 4.0),
            (5.0, 6.0),
        ]));
        let (x_min, x_max, _y_min, _y_max) = chart.compute_bounds();
        // Infinite values should be filtered out
        assert_eq!(x_min, 1.0);
        assert_eq!(x_max, 5.0);
    }

    #[test]
    fn test_compute_bounds_negative_values() {
        let chart = Chart::new().series(Series::new("Test").data(vec![(-5.0, -10.0), (5.0, 10.0)]));
        let (x_min, x_max, y_min, y_max) = chart.compute_bounds();
        // X has no padding
        assert_eq!(x_min, -5.0);
        assert_eq!(x_max, 5.0);
        // Y gets 5% padding: -10 - (20)*0.05 = -11, 10 + (20)*0.05 = 11
        assert!((y_min - (-11.0)).abs() < 0.1);
        assert!((y_max - 11.0).abs() < 0.1);
    }

    #[test]
    fn test_compute_bounds_axis_override() {
        let chart = Chart::new()
            .series(Series::new("Test").data(vec![(1.0, 2.0), (3.0, 4.0)]))
            .x_axis(Axis::new().min(0.0).max(10.0))
            .y_axis(Axis::new().min(-5.0).max(15.0));

        let (x_min, x_max, y_min, y_max) = chart.compute_bounds();
        assert_eq!(x_min, 0.0);
        assert_eq!(x_max, 10.0);
        assert_eq!(y_min, -5.0);
        assert_eq!(y_max, 15.0);
    }

    #[test]
    fn test_compute_bounds_same_values_adds_padding() {
        let chart = Chart::new().series(Series::new("Test").data(vec![(5.0, 5.0)]));
        let (x_min, x_max, _y_min, _y_max) = chart.compute_bounds();
        // Should add epsilon padding for zero range
        assert!(x_min < 5.0);
        assert!(x_max > 5.0);
    }

    #[test]
    fn test_compute_bounds_auto_padding() {
        let chart = Chart::new().series(Series::new("Test").data(vec![(0.0, 0.0), (100.0, 100.0)]));
        let (x_min, x_max, y_min, y_max) = chart.compute_bounds();
        // Only Y axis gets 5% padding, not X
        assert_eq!(x_min, 0.0);
        assert_eq!(x_max, 100.0);
        // Y: 0 - 100*0.05 = -5, 100 + 100*0.05 = 105
        assert!((y_min - (-5.0)).abs() < 0.1);
        assert!((y_max - 105.0).abs() < 0.1);
    }

    #[test]
    fn test_format_label_auto_integer() {
        let chart = Chart::new();
        let result = chart.format_label(5.0, &AxisFormat::Auto);
        assert_eq!(result, "5");
    }

    #[test]
    fn test_format_label_auto_decimal() {
        let chart = Chart::new();
        let result = chart.format_label(5.67, &AxisFormat::Auto);
        assert_eq!(result, "5.67");
    }

    #[test]
    fn test_format_label_auto_scientific_small() {
        let chart = Chart::new();
        let result = chart.format_label(0.001, &AxisFormat::Auto);
        assert!(result.contains("e"));
    }

    #[test]
    fn test_format_label_auto_scientific_large() {
        let chart = Chart::new();
        let result = chart.format_label(10000.0, &AxisFormat::Auto);
        assert!(result.contains("e"));
    }

    #[test]
    fn test_format_label_integer() {
        let chart = Chart::new();
        // 5.67 rounds to 6 with standard rounding
        let result = chart.format_label(5.67, &AxisFormat::Integer);
        assert_eq!(result, "6");
    }

    #[test]
    fn test_format_label_fixed() {
        let chart = Chart::new();
        let result = chart.format_label(5.6789, &AxisFormat::Fixed(3));
        assert_eq!(result, "5.679");
    }

    #[test]
    fn test_format_label_percent() {
        let chart = Chart::new();
        let result = chart.format_label(0.5, &AxisFormat::Percent);
        assert_eq!(result, "50%");
    }

    #[test]
    fn test_format_label_custom() {
        let chart = Chart::new();
        let result = chart.format_label(42.0, &AxisFormat::Custom("Value: {}".to_string()));
        assert_eq!(result, "Value: 42");
    }

    #[test]
    fn test_map_point_min_bounds() {
        let chart = Chart::new();
        let bounds = (0.0, 10.0, 0.0, 10.0);
        let chart_area = (10, 10, 20, 20);

        let (x, y) = chart.map_point(0.0, 0.0, bounds, chart_area);
        assert_eq!(x, 10);
        assert_eq!(y, 29); // Inverted y
    }

    #[test]
    fn test_map_point_max_bounds() {
        let chart = Chart::new();
        let bounds = (0.0, 10.0, 0.0, 10.0);
        let chart_area = (10, 10, 20, 20);

        let (x, y) = chart.map_point(10.0, 10.0, bounds, chart_area);
        assert_eq!(x, 29);
        assert_eq!(y, 10);
    }

    #[test]
    fn test_map_point_center() {
        let chart = Chart::new();
        let bounds = (0.0, 10.0, 0.0, 10.0);
        let chart_area = (10, 10, 20, 20);

        let (x, _y) = chart.map_point(5.0, 5.0, bounds, chart_area);
        assert_eq!(x, 19);
    }

    #[test]
    fn test_map_point_zero_range_x() {
        let chart = Chart::new();
        let bounds = (5.0, 5.0, 0.0, 10.0);
        let chart_area = (10, 10, 20, 20);

        let (_x, _y) = chart.map_point(5.0, 5.0, bounds, chart_area);
        // Center of x range
    }

    #[test]
    fn test_map_point_zero_range_y() {
        let chart = Chart::new();
        let bounds = (0.0, 10.0, 5.0, 5.0);
        let chart_area = (10, 10, 20, 20);

        let (_x, _y) = chart.map_point(5.0, 5.0, bounds, chart_area);
        // Center of y range
    }

    #[test]
    fn test_get_line_char_horizontal() {
        let chart = Chart::new();
        assert_eq!(chart.get_line_char(1, 0), '─');
        assert_eq!(chart.get_line_char(-1, 0), '─');
    }

    #[test]
    fn test_get_line_char_vertical() {
        let chart = Chart::new();
        assert_eq!(chart.get_line_char(0, 1), '│');
        assert_eq!(chart.get_line_char(0, -1), '│');
    }

    #[test]
    fn test_get_line_char_diagonal_up_right() {
        let chart = Chart::new();
        assert_eq!(chart.get_line_char(1, -1), '╱');
        assert_eq!(chart.get_line_char(-1, 1), '╱');
    }

    #[test]
    fn test_get_line_char_diagonal_down_right() {
        let chart = Chart::new();
        assert_eq!(chart.get_line_char(1, 1), '╲');
        assert_eq!(chart.get_line_char(-1, -1), '╲');
    }

    #[test]
    fn test_get_line_char_zero() {
        let chart = Chart::new();
        assert_eq!(chart.get_line_char(0, 0), '·');
    }

    #[test]
    fn test_draw_line_solid() {
        let chart = Chart::new();
        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        let seg = LineSegment {
            x0: 2,
            y0: 2,
            x1: 10,
            y1: 2,
            color: Color::WHITE,
            style: LineStyle::Solid,
        };

        chart.draw_line(&mut ctx, &seg, &rect);
        // Should render without panic
    }

    #[test]
    fn test_draw_line_dashed() {
        let chart = Chart::new();
        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        let seg = LineSegment {
            x0: 2,
            y0: 2,
            x1: 10,
            y1: 2,
            color: Color::WHITE,
            style: LineStyle::Dashed,
        };

        chart.draw_line(&mut ctx, &seg, &rect);
    }

    #[test]
    fn test_draw_line_dotted() {
        let chart = Chart::new();
        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        let seg = LineSegment {
            x0: 2,
            y0: 2,
            x1: 10,
            y1: 2,
            color: Color::WHITE,
            style: LineStyle::Dotted,
        };

        chart.draw_line(&mut ctx, &seg, &rect);
    }

    #[test]
    fn test_draw_line_none() {
        let chart = Chart::new();
        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        let seg = LineSegment {
            x0: 2,
            y0: 2,
            x1: 10,
            y1: 2,
            color: Color::WHITE,
            style: LineStyle::None,
        };

        chart.draw_line(&mut ctx, &seg, &rect);
        // Should render nothing
    }

    #[test]
    fn test_draw_line_out_of_bounds() {
        let chart = Chart::new();
        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        let seg = LineSegment {
            x0: 100,
            y0: 100,
            x1: 200,
            y1: 200,
            color: Color::WHITE,
            style: LineStyle::Solid,
        };

        chart.draw_line(&mut ctx, &seg, &rect);
        // Should not panic even when out of bounds
    }

    #[test]
    fn test_draw_area_fill_basic() {
        let chart = Chart::new();
        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        let points = vec![(5, 5), (10, 5), (15, 3)];
        chart.draw_area_fill(&mut ctx, &points, Color::BLUE, (5, 5, 15, 5), 8);
        // Should render without panic
    }

    #[test]
    fn test_draw_area_fill_empty_points() {
        let chart = Chart::new();
        let mut buffer = Buffer::new(20, 10);
        let rect = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        let points: Vec<(u16, u16)> = vec![];
        chart.draw_area_fill(&mut ctx, &points, Color::BLUE, (5, 5, 15, 5), 8);
        // Should handle empty points gracefully
    }
}
