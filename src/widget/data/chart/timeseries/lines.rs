//! The series: points, the lines between them (solid, dashed, dotted or
//! stepped) and the filled area under them

use super::types::{TimeLineStyle, TimeSeriesData};
use super::view::Plot;
use super::TimeSeries;
use crate::render::Cell;
use crate::widget::traits::RenderContext;

impl TimeSeries {
    /// Render data series
    pub(super) fn render_series(&self, ctx: &mut RenderContext, plot: &Plot) {
        let Plot {
            x: plot_x,
            y: plot_y,
            width: plot_width,
            height: plot_height,
            time_min,
            time_max,
            time_range,
            val_min,
            val_range,
            ..
        } = *plot;

        for series in &self.series {
            let filtered_points: Vec<_> = series
                .points
                .iter()
                .filter(|p| p.timestamp >= time_min && p.timestamp <= time_max)
                .collect();

            if filtered_points.is_empty() {
                continue;
            }

            // Map points to screen coordinates
            let screen_points: Vec<(u16, u16)> = filtered_points
                .iter()
                .map(|p| {
                    let x_ratio = if time_range > 0 {
                        (p.timestamp - time_min) as f64 / time_range as f64
                    } else {
                        0.5
                    };
                    let y_ratio = if val_range > 0.0 {
                        (p.value - val_min) / val_range
                    } else {
                        0.5
                    };

                    let x = plot_x + (x_ratio * (plot_width - 1) as f64) as u16;
                    let y = plot_y + plot_height - 1 - (y_ratio * (plot_height - 1) as f64) as u16;
                    (x, y)
                })
                .collect();

            // Draw lines between points
            for i in 0..screen_points.len().saturating_sub(1) {
                let (x1, y1) = screen_points[i];
                let (x2, y2) = screen_points[i + 1];

                match series.line_style {
                    TimeLineStyle::Step => {
                        self.draw_step_line(ctx, x1, y1, x2, y2, series.color);
                    }
                    _ => {
                        self.draw_line(ctx, x1, y1, x2, y2, series);
                    }
                }

                // Fill area if enabled
                if series.fill {
                    self.fill_area(ctx, x1, y1, x2, y2, plot_y, plot_height, series.color);
                }
            }

            // Draw points
            for &(x, y) in &screen_points {
                let mut cell = Cell::new('●');
                cell.fg = Some(series.color);
                ctx.set(x, y, cell);
            }
        }
    }

    /// Draw a step line between two points
    fn draw_step_line(
        &self,
        ctx: &mut RenderContext,
        x1: u16,
        y1: u16,
        x2: u16,
        y2: u16,
        color: crate::style::Color,
    ) {
        for x in x1..=x2 {
            let mut cell = Cell::new('─');
            cell.fg = Some(color);
            ctx.set(x, y1, cell);
        }
        let (start_y, end_y) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
        for y in start_y..=end_y {
            let mut cell = Cell::new('│');
            cell.fg = Some(color);
            ctx.set(x2, y, cell);
        }
    }

    /// Draw a line between two points using Bresenham's algorithm
    fn draw_line(
        &self,
        ctx: &mut RenderContext,
        x1: u16,
        y1: u16,
        x2: u16,
        y2: u16,
        series: &TimeSeriesData,
    ) {
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();
        let sx = if x1 < x2 { 1i32 } else { -1i32 };
        let sy = if y1 < y2 { 1i32 } else { -1i32 };
        let mut err = dx - dy;
        let mut x = x1 as i32;
        let mut y = y1 as i32;
        let mut step = 0;

        loop {
            let ch = match series.line_style {
                TimeLineStyle::Solid => {
                    if dx > dy {
                        '─'
                    } else {
                        '│'
                    }
                }
                TimeLineStyle::Dashed => {
                    if step % 2 == 0 {
                        if dx > dy {
                            '╌'
                        } else {
                            '╎'
                        }
                    } else {
                        ' '
                    }
                }
                TimeLineStyle::Dotted => {
                    if step % 2 == 0 {
                        '·'
                    } else {
                        ' '
                    }
                }
                TimeLineStyle::Step => '─',
            };

            if ch != ' ' {
                let mut cell = Cell::new(ch);
                cell.fg = Some(series.color);
                ctx.set(x as u16, y as u16, cell);
            }

            if x == x2 as i32 && y == y2 as i32 {
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

    #[allow(clippy::too_many_arguments)]
    /// Fill area under a line segment
    fn fill_area(
        &self,
        ctx: &mut RenderContext,
        x1: u16,
        y1: u16,
        x2: u16,
        y2: u16,
        plot_y: u16,
        plot_height: u16,
        color: crate::style::Color,
    ) {
        let bottom_y = plot_y + plot_height - 1;
        for x in x1..=x2 {
            let y_at_x = if x2 != x1 {
                let t = (x - x1) as f64 / (x2 - x1) as f64;
                (y1 as f64 + t * (y2 as f64 - y1 as f64)) as u16
            } else {
                y1
            };
            for y in y_at_x..=bottom_y {
                let fill_color = crate::style::Color::rgb(
                    (color.r as u16 * 3 / 10) as u8,
                    (color.g as u16 * 3 / 10) as u8,
                    (color.b as u16 * 3 / 10) as u8,
                );
                let mut cell = Cell::new(' ');
                cell.bg = Some(fill_color);
                ctx.set(x, y, cell);
            }
        }
    }
}
