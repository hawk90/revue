//! Chart marks: each series as lines, Braille lines, area fill and markers

use super::geometry::{bresenham, clip_segment, grid_cell, grid_contains, DataSegment};
use super::render::PlotLayout;
use super::Chart;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::canvas::BrailleGrid;
use crate::widget::data::chart::chart_common::Marker;
use crate::widget::data::chart::types::{ChartType, LineStyle};
use crate::widget::traits::RenderContext;

/// Line segment for drawing
pub(super) struct LineSegment {
    x0: u16,
    y0: u16,
    x1: u16,
    y1: u16,
    color: Color,
    style: LineStyle,
}

impl Chart {
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
    pub(super) fn draw_line(&self, ctx: &mut RenderContext, seg: &LineSegment, bounds: &Rect) {
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
    pub(super) fn draw_braille_lines(
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
    pub(super) fn draw_area_fill(
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

    /// Draw each series
    pub(super) fn draw_series(
        &self,
        ctx: &mut RenderContext,
        plot: &PlotLayout,
        bounds: (f64, f64, f64, f64),
    ) {
        let PlotLayout {
            inner_y,
            inner_h,
            chart_bounds,
            ..
        } = *plot;
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
    }
}
