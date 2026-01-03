//! Chart widget for data visualization
//!
//! Supports line charts, scatter plots, area charts, and step charts
//! with multiple series, axes, legends, and grid lines.

use super::traits::{RenderContext, View, WidgetProps};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Line segment for drawing
struct LineSegment {
    x0: u16,
    y0: u16,
    x1: u16,
    y1: u16,
    color: Color,
    style: LineStyle,
}

/// Chart type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartType {
    /// Line chart connecting points
    #[default]
    Line,
    /// Scatter plot (points only)
    Scatter,
    /// Area chart (filled below line)
    Area,
    /// Step chart (horizontal then vertical)
    StepAfter,
    /// Step chart (vertical then horizontal)
    StepBefore,
}

/// Marker style for data points
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Marker {
    /// No marker
    #[default]
    None,
    /// Dot marker (•)
    Dot,
    /// Circle marker (○)
    Circle,
    /// Square marker (□)
    Square,
    /// Diamond marker (◇)
    Diamond,
    /// Triangle marker (△)
    Triangle,
    /// Cross marker (+)
    Cross,
    /// X marker (×)
    X,
    /// Star marker (★)
    Star,
    /// Braille dots for high resolution
    Braille,
}

impl Marker {
    fn char(&self) -> char {
        match self {
            Marker::None => ' ',
            Marker::Dot => '•',
            Marker::Circle => '○',
            Marker::Square => '□',
            Marker::Diamond => '◇',
            Marker::Triangle => '△',
            Marker::Cross => '+',
            Marker::X => '×',
            Marker::Star => '★',
            Marker::Braille => '⣿',
        }
    }
}

/// Line style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LineStyle {
    /// Solid line
    #[default]
    Solid,
    /// Dashed line
    Dashed,
    /// Dotted line
    Dotted,
    /// No line (for scatter)
    None,
}

/// A data series
#[derive(Clone, Debug)]
pub struct Series {
    /// Series name (for legend)
    pub name: String,
    /// Data points (x, y)
    pub data: Vec<(f64, f64)>,
    /// Chart type for this series
    pub chart_type: ChartType,
    /// Line color
    pub color: Color,
    /// Line style
    pub line_style: LineStyle,
    /// Marker style
    pub marker: Marker,
    /// Fill color (for area charts)
    pub fill_color: Option<Color>,
}

impl Series {
    /// Create a new series
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            chart_type: ChartType::Line,
            color: Color::WHITE,
            line_style: LineStyle::Solid,
            marker: Marker::None,
            fill_color: None,
        }
    }

    /// Set data points
    pub fn data(mut self, data: Vec<(f64, f64)>) -> Self {
        self.data = data;
        self
    }

    /// Set data from y values (x = index)
    pub fn data_y(mut self, ys: &[f64]) -> Self {
        self.data = ys.iter().enumerate().map(|(i, &y)| (i as f64, y)).collect();
        self
    }

    /// Set chart type
    pub fn chart_type(mut self, chart_type: ChartType) -> Self {
        self.chart_type = chart_type;
        self
    }

    /// Set color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set line style
    pub fn line_style(mut self, style: LineStyle) -> Self {
        self.line_style = style;
        self
    }

    /// Set marker
    pub fn marker(mut self, marker: Marker) -> Self {
        self.marker = marker;
        self
    }

    /// Set fill color (for area charts)
    pub fn fill(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self.chart_type = ChartType::Area;
        self
    }

    /// Make this a scatter plot
    pub fn scatter(mut self) -> Self {
        self.chart_type = ChartType::Scatter;
        self.line_style = LineStyle::None;
        if matches!(self.marker, Marker::None) {
            self.marker = Marker::Dot;
        }
        self
    }

    /// Make this a line chart
    pub fn line(mut self) -> Self {
        self.chart_type = ChartType::Line;
        self.line_style = LineStyle::Solid;
        self
    }

    /// Make this an area chart
    pub fn area(mut self, fill_color: Color) -> Self {
        self.chart_type = ChartType::Area;
        self.fill_color = Some(fill_color);
        self
    }

    /// Make this a step chart
    pub fn step(mut self) -> Self {
        self.chart_type = ChartType::StepAfter;
        self
    }
}

/// Axis configuration
#[derive(Clone, Debug)]
pub struct Axis {
    /// Axis title
    pub title: Option<String>,
    /// Minimum value (auto if None)
    pub min: Option<f64>,
    /// Maximum value (auto if None)
    pub max: Option<f64>,
    /// Number of tick marks
    pub ticks: usize,
    /// Show grid lines
    pub grid: bool,
    /// Axis color
    pub color: Color,
    /// Label formatter
    pub format: AxisFormat,
}

/// Axis label format
#[derive(Clone, Debug, Default)]
pub enum AxisFormat {
    /// Auto format
    #[default]
    Auto,
    /// Integer format
    Integer,
    /// Fixed decimal places
    Fixed(usize),
    /// Percentage
    Percent,
    /// Custom format string
    Custom(String),
}

impl Default for Axis {
    fn default() -> Self {
        Self {
            title: None,
            min: None,
            max: None,
            ticks: 5,
            grid: true,
            color: Color::rgb(100, 100, 100),
            format: AxisFormat::Auto,
        }
    }
}

impl Axis {
    /// Create a new axis
    pub fn new() -> Self {
        Self::default()
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set bounds
    pub fn bounds(mut self, min: f64, max: f64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    /// Set number of ticks
    pub fn ticks(mut self, ticks: usize) -> Self {
        self.ticks = ticks;
        self
    }

    /// Enable/disable grid
    pub fn grid(mut self, show: bool) -> Self {
        self.grid = show;
        self
    }

    /// Set axis color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set format
    pub fn format(mut self, format: AxisFormat) -> Self {
        self.format = format;
        self
    }
}

/// Legend position
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LegendPosition {
    /// Top left
    TopLeft,
    /// Top center
    TopCenter,
    /// Top right
    #[default]
    TopRight,
    /// Bottom left
    BottomLeft,
    /// Bottom center
    BottomCenter,
    /// Bottom right
    BottomRight,
    /// Hidden
    None,
}

/// Chart widget
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
                LegendPosition::None => unreachable!(),
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

    #[test]
    fn test_chart_new() {
        let c = Chart::new();
        assert!(c.title.is_none());
        assert!(c.series.is_empty());
    }

    #[test]
    fn test_series_builder() {
        let s = Series::new("Test")
            .data(vec![(0.0, 1.0), (1.0, 2.0)])
            .color(Color::RED)
            .marker(Marker::Dot);

        assert_eq!(s.name, "Test");
        assert_eq!(s.data.len(), 2);
        assert_eq!(s.color, Color::RED);
    }

    #[test]
    fn test_series_data_y() {
        let s = Series::new("Test").data_y(&[1.0, 2.0, 3.0]);
        assert_eq!(s.data, vec![(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)]);
    }

    #[test]
    fn test_chart_bounds() {
        let c = Chart::new().series(Series::new("A").data(vec![(0.0, 0.0), (10.0, 100.0)]));

        let bounds = c.compute_bounds();
        assert_eq!(bounds.0, 0.0); // x_min
        assert_eq!(bounds.1, 10.0); // x_max
    }

    #[test]
    fn test_axis_builder() {
        let axis = Axis::new()
            .title("Value")
            .bounds(0.0, 100.0)
            .ticks(10)
            .grid(true);

        assert_eq!(axis.title, Some("Value".to_string()));
        assert_eq!(axis.min, Some(0.0));
        assert_eq!(axis.max, Some(100.0));
        assert_eq!(axis.ticks, 10);
        assert!(axis.grid);
    }

    #[test]
    fn test_chart_render() {
        let mut buffer = Buffer::new(60, 20);
        let area = Rect::new(0, 0, 60, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Chart::new().title("Test Chart").series(
            Series::new("Data")
                .data_y(&[1.0, 4.0, 2.0, 5.0, 3.0])
                .line(),
        );

        c.render(&mut ctx);
        // Basic smoke test - chart renders without panic
    }

    #[test]
    fn test_quick_line_chart() {
        let c = line_chart(&[1.0, 2.0, 3.0, 2.0, 1.0]);
        assert_eq!(c.series.len(), 1);
        assert_eq!(c.series[0].data.len(), 5);
    }

    #[test]
    fn test_quick_scatter_plot() {
        let c = scatter_plot(&[(0.0, 1.0), (1.0, 2.0), (2.0, 1.5)]);
        assert_eq!(c.series.len(), 1);
        assert!(matches!(c.series[0].chart_type, ChartType::Scatter));
    }

    #[test]
    fn test_multiple_series() {
        let c = Chart::new()
            .series(Series::new("A").data_y(&[1.0, 2.0, 3.0]).color(Color::RED))
            .series(Series::new("B").data_y(&[3.0, 2.0, 1.0]).color(Color::BLUE));

        assert_eq!(c.series.len(), 2);
    }

    #[test]
    fn test_area_chart() {
        let s = Series::new("Area")
            .data_y(&[1.0, 3.0, 2.0])
            .area(Color::CYAN);

        assert!(matches!(s.chart_type, ChartType::Area));
        assert_eq!(s.fill_color, Some(Color::CYAN));
    }

    #[test]
    fn test_step_chart() {
        let s = Series::new("Step").data_y(&[1.0, 2.0, 3.0]).step();
        assert!(matches!(s.chart_type, ChartType::StepAfter));
    }

    #[test]
    fn test_marker_chars() {
        assert_eq!(Marker::Dot.char(), '•');
        assert_eq!(Marker::Circle.char(), '○');
        assert_eq!(Marker::Square.char(), '□');
        assert_eq!(Marker::Diamond.char(), '◇');
        assert_eq!(Marker::Cross.char(), '+');
    }

    #[test]
    fn test_format_labels() {
        let c = Chart::new();

        assert_eq!(c.format_label(5.0, &AxisFormat::Integer), "5");
        assert_eq!(c.format_label(5.0, &AxisFormat::Fixed(2)), "5.00");
        assert_eq!(c.format_label(0.5, &AxisFormat::Percent), "50%");
    }

    #[test]
    fn test_legend_positions() {
        let c = Chart::new()
            .series(Series::new("Test").data_y(&[1.0, 2.0]))
            .legend(LegendPosition::BottomLeft);

        assert!(matches!(c.legend, LegendPosition::BottomLeft));
    }

    #[test]
    fn test_chart_with_all_options() {
        let mut buffer = Buffer::new(80, 30);
        let area = Rect::new(0, 0, 80, 30);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Chart::new()
            .title("Full Chart")
            .border(Color::WHITE)
            .bg(Color::rgb(20, 20, 20))
            .x_axis(Axis::new().title("Time").ticks(10))
            .y_axis(Axis::new().title("Value").bounds(0.0, 100.0))
            .legend(LegendPosition::TopRight)
            .series(
                Series::new("Line")
                    .data_y(&[10.0, 30.0, 20.0, 50.0, 40.0, 60.0])
                    .color(Color::GREEN)
                    .marker(Marker::Circle),
            )
            .series(
                Series::new("Area")
                    .data_y(&[5.0, 15.0, 10.0, 25.0, 20.0, 30.0])
                    .area(Color::CYAN),
            );

        c.render(&mut ctx);
        // Smoke test passes
    }

    #[test]
    fn test_compute_bounds_empty_data() {
        let c = Chart::new();
        let (x_min, x_max, y_min, y_max) = c.compute_bounds();
        // Default bounds for empty data
        assert_eq!(x_min, 0.0);
        assert_eq!(x_max, 1.0);
        assert!(y_min < y_max); // With padding
    }

    #[test]
    fn test_compute_bounds_single_point() {
        let c = Chart::new().series(Series::new("Single").data(vec![(5.0, 10.0)]));
        let (x_min, x_max, y_min, y_max) = c.compute_bounds();
        // Should create range around single point
        assert!(x_min < x_max);
        assert!(y_min < y_max);
    }

    #[test]
    fn test_compute_bounds_zero_range() {
        let c = Chart::new().series(Series::new("Flat").data_y(&[5.0, 5.0, 5.0]));
        let (_, _, y_min, y_max) = c.compute_bounds();
        // Should create range around flat line
        assert!(y_min < y_max);
    }

    #[test]
    fn test_compute_bounds_nan_values() {
        let c = Chart::new().series(Series::new("WithNaN").data(vec![
            (1.0, 2.0),
            (f64::NAN, 3.0),
            (3.0, f64::INFINITY),
        ]));
        let (x_min, x_max, y_min, y_max) = c.compute_bounds();
        // Should ignore NaN/Infinity and use valid data
        assert!(x_min.is_finite());
        assert!(x_max.is_finite());
        assert!(y_min.is_finite());
        assert!(y_max.is_finite());
    }
}
