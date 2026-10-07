//! Helper functions for chart widget

use super::chart_common::{Axis, AxisFormat, LegendPosition, Marker};
use super::types::{ChartType, LineStyle, Series};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{char_width, display_width};
use crate::widget::canvas::BrailleGrid;
use crate::widget::theme::DARK_GRAY;
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
#[derive(Debug, Clone)]
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
    /// Tooltip configuration
    tooltip: Option<super::chart_common::ChartTooltip>,
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
            tooltip: None,
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
    ///
    /// Series lines (line, area outline and step charts) are drawn with
    /// Braille dots, 2x4 per cell, instead of box-drawing characters.
    /// Markers, area fills, axes and labels still use whole cells.
    pub fn braille(mut self) -> Self {
        self.braille_mode = true;
        self
    }

    /// Set tooltip configuration
    pub fn tooltip(mut self, tooltip: super::chart_common::ChartTooltip) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    /// Enable tooltips with default settings
    pub fn with_tooltip(mut self) -> Self {
        self.tooltip = Some(super::chart_common::ChartTooltip::enabled());
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

    /// Map a data point onto a `gw` x `gh` grid laid over the plot area.
    ///
    /// Returns fractional offsets: columns from the left edge and rows up from
    /// the bottom edge. Points inside the axis bounds land in
    /// `[0, gw - 1] x [0, gh - 1]`; points outside fall outside that range
    /// (they are clipped by `clip_segment` and `grid_contains`, never
    /// cast to `u16` unchecked).
    fn grid_offset(x: f64, y: f64, bounds: (f64, f64, f64, f64), gw: u16, gh: u16) -> GridPoint {
        let (x_min, x_max, y_min, y_max) = bounds;
        let x_range = x_max - x_min;
        let y_range = y_max - y_min;

        let ox = if x_range > 0.0 {
            (x - x_min) / x_range * (gw as f64 - 1.0)
        } else {
            (gw / 2) as f64
        };
        let oy = if y_range > 0.0 {
            (y - y_min) / y_range * (gh as f64 - 1.0)
        } else {
            (gh - 1 - gh / 2) as f64
        };
        (ox, oy)
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

        let ch = self.get_line_char(x1 as i32 - x0 as i32, y1 as i32 - y0 as i32);
        bresenham((x0, y0), (x1, y1), |x, y, step| {
            // Check bounds
            if x >= bounds.x
                && x < bounds.x + bounds.width
                && y >= bounds.y
                && y < bounds.y + bounds.height
            {
                let draw = match style {
                    LineStyle::Solid => true,
                    LineStyle::Dashed => (step / 3) % 2 == 0,
                    LineStyle::Dotted => step % 2 == 0,
                    LineStyle::None => false,
                };

                if draw {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(color);
                    ctx.set(x, y, cell);
                }
            }
        });
    }

    /// Draw a series' line segments (in data coordinates) with Braille dots,
    /// 2x4 dots per cell of the plot area
    fn draw_braille_lines(
        &self,
        ctx: &mut RenderContext,
        segments: &[DataSegment],
        bounds: (f64, f64, f64, f64),
        plot: &Rect,
        color: Color,
        style: LineStyle,
    ) {
        let gw = plot.width.saturating_mul(2);
        let gh = plot.height.saturating_mul(4);
        let mut grid = BrailleGrid::new(plot.width, plot.height);

        for &(a, b) in segments {
            let a = Self::grid_offset(a.0, a.1, bounds, gw, gh);
            let b = Self::grid_offset(b.0, b.1, bounds, gw, gh);
            let Some((a, b)) = clip_segment(a, b, gw, gh) else {
                continue;
            };
            let start = grid_cell(a, gw, gh);
            let end = grid_cell(b, gw, gh);
            bresenham(start, end, |x, y, step| {
                // Dash and dot lengths are doubled to match the cell-mode
                // pattern at twice the horizontal resolution
                let draw = match style {
                    LineStyle::Solid => true,
                    LineStyle::Dashed => (step / 6) % 2 == 0,
                    LineStyle::Dotted => (step / 2) % 2 == 0,
                    LineStyle::None => false,
                };
                if draw {
                    grid.set(x as usize, y as usize, color);
                }
            });
        }

        for cy in 0..plot.height {
            for cx in 0..plot.width {
                let ch = grid.get_char(cx as usize, cy as usize);
                if ch != '\u{2800}' {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(color);
                    ctx.set(plot.x + cx, plot.y + cy, cell);
                }
            }
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
                        ctx.set(x, y, cell);
                    }
                }
            }
        }
    }
}

/// A point on the plot grid: fractional columns from the left edge and rows up
/// from the bottom edge.
type GridPoint = (f64, f64);

/// A line segment between two points in data coordinates.
type DataSegment = ((f64, f64), (f64, f64));

/// Whether a grid point lies inside a `gw` x `gh` grid.
fn grid_contains((x, y): GridPoint, gw: u16, gh: u16) -> bool {
    const EPS: f64 = 1e-9;
    x >= -EPS && x <= gw as f64 - 1.0 + EPS && y >= -EPS && y <= gh as f64 - 1.0 + EPS
}

/// Clip a segment to a `gw` x `gh` grid (Liang-Barsky).
///
/// Returns `None` when the segment misses the grid. Endpoints that are already
/// inside are returned unchanged.
fn clip_segment(p0: GridPoint, p1: GridPoint, gw: u16, gh: u16) -> Option<(GridPoint, GridPoint)> {
    let (x_max, y_max) = (gw as f64 - 1.0, gh as f64 - 1.0);
    let (dx, dy) = (p1.0 - p0.0, p1.1 - p0.1);
    let (mut t0, mut t1) = (0.0f64, 1.0f64);
    for (p, q) in [
        (-dx, p0.0),
        (dx, x_max - p0.0),
        (-dy, p0.1),
        (dy, y_max - p0.1),
    ] {
        if p == 0.0 {
            if q < 0.0 {
                return None;
            }
        } else {
            let r = q / p;
            if p < 0.0 {
                t0 = t0.max(r);
            } else {
                t1 = t1.min(r);
            }
        }
    }
    if t0 > t1 {
        return None;
    }
    let at = |t: f64| (p0.0 + t * dx, p0.1 + t * dy);
    let a = if t0 > 0.0 { at(t0) } else { p0 };
    let b = if t1 < 1.0 { at(t1) } else { p1 };
    Some((a, b))
}

/// Walk the cells of a line from `start` to `end` (Bresenham), calling
/// `visit(x, y, step)` for each one.
fn bresenham(start: (u16, u16), end: (u16, u16), mut visit: impl FnMut(u16, u16, usize)) {
    let (x0, y0) = (start.0 as i32, start.1 as i32);
    let (x1, y1) = (end.0 as i32, end.1 as i32);
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let (mut x, mut y) = (x0, y0);
    let mut step = 0;
    loop {
        visit(x as u16, y as u16, step);
        if x == x1 && y == y1 {
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

/// The grid cell (column, row from the top) holding a point inside a
/// `gw` x `gh` grid.
fn grid_cell((x, y): GridPoint, gw: u16, gh: u16) -> (u16, u16) {
    let col = x.floor().clamp(0.0, gw as f64 - 1.0) as u16;
    let up = y.floor().clamp(0.0, gh as f64 - 1.0) as u16;
    (col, gh - 1 - up)
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

        // The series palette carries the data and each axis has its own color
        // field; the chrome around them - title and legend labels - is
        // ordinary text.
        let chrome = ctx.css_color(Color::WHITE);

        // Fill background
        if let Some(bg) = self.bg_color {
            ctx.fill_box_background(bg);
        }

        // Draw border
        if let Some(border_color) = self.border_color {
            // Top and bottom
            for x in 0..area.width {
                let mut top = Cell::new('─');
                top.fg = Some(border_color);
                ctx.set(x, 0, top);

                let mut bottom = Cell::new('─');
                bottom.fg = Some(border_color);
                ctx.set(x, area.height - 1, bottom);
            }
            // Left and right
            for y in 0..area.height {
                let mut left = Cell::new('│');
                left.fg = Some(border_color);
                ctx.set(0, y, left);

                let mut right = Cell::new('│');
                right.fg = Some(border_color);
                ctx.set(area.width - 1, y, right);
            }
            // Corners
            let corners = [
                (0u16, 0u16, '┌'),
                (area.width - 1, 0, '┐'),
                (0, area.height - 1, '└'),
                (area.width - 1, area.height - 1, '┘'),
            ];
            for (x, y, ch) in corners {
                let mut cell = Cell::new(ch);
                cell.fg = Some(border_color);
                ctx.set(x, y, cell);
            }
        }

        // Calculate layout (relative coordinates)
        let has_border = self.border_color.is_some();
        let has_title = self.title.is_some();
        let y_label_width = 8u16; // Space for Y axis labels
        let x_label_height = 2u16; // Space for X axis labels

        let inner_x = if has_border { 1 } else { 0 } + y_label_width;
        let inner_y = if has_border { 1 } else { 0 } + if has_title { 1u16 } else { 0 };
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
            let title_x = (area.width.saturating_sub(display_width(title) as u16)) / 2;
            let title_y = if has_border { 1u16 } else { 0 };
            let mut dx: u16 = 0;
            for ch in title.chars() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(chrome);
                cell.modifier |= crate::render::Modifier::BOLD;
                ctx.set(title_x + dx, title_y, cell);
                dx += char_width(ch) as u16;
            }
        }

        // Compute bounds
        let bounds = self.compute_bounds();
        let (x_min, x_max, y_min, y_max) = bounds;

        // Draw Y axis labels
        let y_label_x = if has_border { 1u16 } else { 0 };
        for i in 0..=self.y_axis.ticks {
            let t = i as f64 / self.y_axis.ticks as f64;
            let value = y_min + t * (y_max - y_min);
            let label = self.format_label(value, &self.y_axis.format);
            let y = inner_y + inner_h - 1 - ((t * (inner_h as f64 - 1.0)) as u16);

            // Right-align label
            let label_start =
                y_label_x + y_label_width.saturating_sub(display_width(&label) as u16 + 1);
            let mut dx: u16 = 0;
            for ch in label.chars() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.y_axis.color);
                ctx.set(label_start + dx, y, cell);
                dx += char_width(ch) as u16;
            }

            // Draw grid line
            if self.y_axis.grid && i > 0 && i < self.y_axis.ticks {
                for x in inner_x..inner_x + inner_w {
                    let mut cell = Cell::new('┄');
                    cell.fg = Some(Color::rgb(50, 50, 50));
                    ctx.set(x, y, cell);
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
            let label_start = x.saturating_sub(display_width(&label) as u16 / 2);
            let mut dx: u16 = 0;
            for ch in label.chars() {
                let px = label_start + dx;
                if px < area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.x_axis.color);
                    ctx.set(px, x_label_y, cell);
                }
                dx += char_width(ch) as u16;
            }

            // Draw grid line
            if self.x_axis.grid && i > 0 && i < self.x_axis.ticks {
                for y in inner_y..inner_y + inner_h {
                    let mut cell = Cell::new('┊');
                    cell.fg = Some(Color::rgb(50, 50, 50));
                    ctx.set(x, y, cell);
                }
            }
        }

        // Draw axis titles
        if let Some(ref title) = self.y_axis.title {
            // Draw vertically on the left (one char per row, so char count governs height)
            let char_count = title.chars().count() as u16;
            let y_start = inner_y + (inner_h.saturating_sub(char_count)) / 2;
            for (i, ch) in title.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.y_axis.color);
                ctx.set(y_label_x, y_start + i as u16, cell);
            }
        }

        if let Some(ref title) = self.x_axis.title {
            let x_start = inner_x + (inner_w.saturating_sub(display_width(title) as u16)) / 2;
            let y = x_label_y + 1;
            if y < area.height {
                let mut dx: u16 = 0;
                for ch in title.chars() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.x_axis.color);
                    ctx.set(x_start + dx, y, cell);
                    dx += char_width(ch) as u16;
                }
            }
        }

        // Draw each series
        let y_bottom = inner_y + inner_h - 1;

        for series in &self.series {
            if series.data.is_empty() {
                continue;
            }

            let chart_area = (
                chart_bounds.x,
                chart_bounds.y,
                chart_bounds.width,
                chart_bounds.height,
            );
            // Non-finite points are left out of the bounds, so they are left
            // out of the plot too (mapping them would overflow or pin them
            // to the left edge).
            let points: Vec<(f64, f64)> = series
                .data
                .iter()
                .copied()
                .filter(|(x, y)| x.is_finite() && y.is_finite())
                .collect();

            // The pieces of the series line, in data coordinates
            let data_segments: Vec<DataSegment> = match series.chart_type {
                ChartType::Line | ChartType::Area => {
                    points.windows(2).map(|w| (w[0], w[1])).collect()
                }
                // Horizontal then vertical
                ChartType::StepAfter => points
                    .windows(2)
                    .flat_map(|w| {
                        let corner = (w[1].0, w[0].1);
                        [(w[0], corner), (corner, w[1])]
                    })
                    .collect(),
                // Vertical then horizontal
                ChartType::StepBefore => points
                    .windows(2)
                    .flat_map(|w| {
                        let corner = (w[0].0, w[1].1);
                        [(w[0], corner), (corner, w[1])]
                    })
                    .collect(),
                ChartType::Scatter => Vec::new(),
            };

            // Segments clipped to the plot, as screen cells. Points outside
            // fixed axis bounds are cut off at the plot edge.
            let (w, h) = (chart_bounds.width, chart_bounds.height);
            let cell_segments: Vec<((u16, u16), (u16, u16))> = data_segments
                .iter()
                .filter_map(|&(a, b)| {
                    let a = Self::grid_offset(a.0, a.1, bounds, w, h);
                    let b = Self::grid_offset(b.0, b.1, bounds, w, h);
                    clip_segment(a, b, w, h)
                })
                .map(|(a, b)| {
                    let (ax, ay) = grid_cell(a, w, h);
                    let (bx, by) = grid_cell(b, w, h);
                    (
                        (chart_bounds.x + ax, chart_bounds.y + ay),
                        (chart_bounds.x + bx, chart_bounds.y + by),
                    )
                })
                .collect();

            // Draw area fill first (if applicable)
            if matches!(series.chart_type, ChartType::Area) {
                if let Some(fill_color) = series.fill_color {
                    for &(a, b) in &cell_segments {
                        self.draw_area_fill(ctx, &[a, b], fill_color, chart_area, y_bottom);
                    }
                }
            }

            // Draw lines
            if !matches!(series.line_style, LineStyle::None) && self.braille_mode {
                self.draw_braille_lines(
                    ctx,
                    &data_segments,
                    bounds,
                    &chart_bounds,
                    series.color,
                    series.line_style,
                );
            } else if !matches!(series.line_style, LineStyle::None) {
                for &((x0, y0), (x1, y1)) in &cell_segments {
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

            // Draw markers (only for points inside the plot)
            if !matches!(series.marker, Marker::None) {
                let marker_char = series.marker.char();
                for &(x, y) in &points {
                    let p = Self::grid_offset(x, y, bounds, w, h);
                    if !grid_contains(p, w, h) {
                        continue;
                    }
                    let (col, row) = grid_cell(p, w, h);
                    let mut cell = Cell::new(marker_char);
                    cell.fg = Some(series.color);
                    ctx.set(chart_bounds.x + col, chart_bounds.y + row, cell);
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

            // A legend larger than the plot is pinned to its top/left edge
            // (and clipped below) instead of underflowing.
            let left = inner_x + 1;
            let center_x = inner_x + inner_w.saturating_sub(legend_width) / 2;
            let right = inner_x + inner_w.saturating_sub(legend_width + 1);
            let top = inner_y + 1;
            let middle_y = inner_y + inner_h.saturating_sub(legend_height) / 2;
            let bottom = inner_y + inner_h.saturating_sub(legend_height + 1);
            let (legend_x, legend_y) = match self.legend {
                LegendPosition::TopLeft => (left, top),
                LegendPosition::TopCenter => (center_x, top),
                LegendPosition::TopRight => (right, top),
                LegendPosition::BottomLeft => (left, bottom),
                LegendPosition::BottomCenter => (center_x, bottom),
                LegendPosition::BottomRight => (right, bottom),
                LegendPosition::Left => (left, middle_y),
                LegendPosition::Right => (right, middle_y),
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
                        cell.fg = Some(DARK_GRAY);
                        cell.bg = self.bg_color.or(Some(Color::rgb(20, 20, 20)));
                        ctx.set(x, y, cell);
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
                ctx.set(legend_x + 1, y, indicator);

                // Series name
                let mut dx: u16 = 0;
                for ch in series.name.chars() {
                    let x = legend_x + 3 + dx;
                    if x < legend_x + legend_width - 1 {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(chrome);
                        cell.bg = self.bg_color.or(Some(Color::rgb(20, 20, 20)));
                        ctx.set(x, y, cell);
                    }
                    dx += char_width(ch) as u16;
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
