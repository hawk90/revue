//! Time Series widget rendering

use super::super::traits::{RenderContext, View};
use super::types::{TimeLineStyle, TimeSeriesData};
use super::TimeSeries;
use crate::render::Cell;

impl View for TimeSeries {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let height = self.height.unwrap_or(area.height);

        if area.width < 10 || height < 5 {
            return;
        }

        let mut current_y = area.y;

        // Background
        if let Some(bg) = self.bg_color {
            for y in area.y..area.y + height.min(area.height) {
                for x in area.x..area.x + area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Title
        if let Some(ref title) = self.title {
            let title_x = area.x + (area.width.saturating_sub(title.len() as u16)) / 2;
            ctx.buffer.put_str_styled(
                title_x,
                current_y,
                title,
                Some(crate::style::Color::WHITE),
                self.bg_color,
            );
            current_y += 1;
        }

        // Legend
        if self.show_legend && !self.series.is_empty() {
            let mut x = area.x + 2;
            for series in &self.series {
                let marker = match series.line_style {
                    TimeLineStyle::Solid => "─",
                    TimeLineStyle::Dashed => "╌",
                    TimeLineStyle::Dotted => "·",
                    TimeLineStyle::Step => "┐",
                };
                ctx.buffer
                    .put_str_styled(x, current_y, marker, Some(series.color), self.bg_color);
                x += 2;
                ctx.buffer.put_str_styled(
                    x,
                    current_y,
                    &series.name,
                    Some(crate::style::Color::WHITE),
                    self.bg_color,
                );
                x += series.name.len() as u16 + 3;
            }
            current_y += 1;
        }

        let y_label_width = 8u16;
        let plot_x = area.x + y_label_width;
        let plot_width = area.width.saturating_sub(y_label_width + 1);
        let plot_y = current_y;
        let plot_height = height.saturating_sub(current_y - area.y + 2);

        if plot_width < 5 || plot_height < 3 {
            return;
        }

        let (time_min, time_max) = self.get_time_bounds();
        let (val_min, val_max) = self.get_value_bounds();
        let time_range = time_max.saturating_sub(time_min);
        let val_range = val_max - val_min;

        // Draw grid
        if self.show_grid {
            let grid_rows = 4.min(plot_height as usize);
            for i in 0..=grid_rows {
                let y = plot_y + (i * plot_height as usize / grid_rows) as u16;
                for x in plot_x..plot_x + plot_width {
                    let ch = if i == grid_rows { '─' } else { '┄' };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.grid_color);
                    ctx.buffer.set(x, y, cell);
                }

                // Y-axis labels
                let val = val_max - (i as f64 * val_range / grid_rows as f64);
                let label = if val.abs() >= 1000.0 {
                    format!("{:.1}k", val / 1000.0)
                } else if val.abs() >= 1.0 {
                    format!("{:.1}", val)
                } else {
                    format!("{:.2}", val)
                };
                let label_x = area.x + y_label_width.saturating_sub(label.len() as u16 + 1);
                ctx.buffer.put_str_styled(
                    label_x,
                    y,
                    &label,
                    Some(crate::style::Color::WHITE),
                    self.bg_color,
                );
            }
        }

        // Draw markers
        for marker in &self.markers {
            if marker.timestamp >= time_min && marker.timestamp <= time_max && time_range > 0 {
                let x_pos = ((marker.timestamp - time_min) as f64 / time_range as f64
                    * (plot_width - 1) as f64) as u16;
                let x = plot_x + x_pos;

                match marker.style {
                    super::types::MarkerStyle::Line => {
                        for y in plot_y..plot_y + plot_height {
                            let mut cell = Cell::new('│');
                            cell.fg = Some(marker.color);
                            ctx.buffer.set(x, y, cell);
                        }
                    }
                    super::types::MarkerStyle::Point | super::types::MarkerStyle::Region => {
                        let mut cell = Cell::new('▼');
                        cell.fg = Some(marker.color);
                        ctx.buffer.set(x, plot_y, cell);
                    }
                }

                if !marker.label.is_empty() {
                    let label_x = x.saturating_sub(marker.label.len() as u16 / 2);
                    ctx.buffer.put_str_styled(
                        label_x,
                        plot_y + plot_height + 1,
                        &marker.label,
                        Some(marker.color),
                        self.bg_color,
                    );
                }
            }
        }

        // Draw series
        self.render_series(
            ctx,
            plot_x,
            plot_y,
            plot_width,
            plot_height,
            time_min,
            time_max,
            time_range,
            val_min,
            val_range,
        );

        // X-axis time labels
        let x_label_y = plot_y + plot_height + 1;
        if x_label_y < area.y + height {
            let num_labels = (plot_width / 12).max(2) as usize;
            for i in 0..num_labels {
                let ratio = i as f64 / (num_labels - 1) as f64;
                let ts = time_min + (ratio * time_range as f64) as u64;
                let label = self.format_time(ts, time_range);
                let x = plot_x + (ratio * (plot_width - 1) as f64) as u16;
                let label_x = x.saturating_sub(label.len() as u16 / 2);
                ctx.buffer.put_str_styled(
                    label_x,
                    x_label_y,
                    &label,
                    Some(crate::style::Color::WHITE),
                    self.bg_color,
                );
            }
        }
    }

    crate::impl_view_meta!("TimeSeries");
}

impl TimeSeries {
    #[allow(clippy::too_many_arguments)]
    /// Render data series
    fn render_series(
        &self,
        ctx: &mut RenderContext,
        plot_x: u16,
        plot_y: u16,
        plot_width: u16,
        plot_height: u16,
        time_min: u64,
        time_max: u64,
        time_range: u64,
        val_min: f64,
        val_range: f64,
    ) {
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
                ctx.buffer.set(x, y, cell);
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
            ctx.buffer.set(x, y1, cell);
        }
        let (start_y, end_y) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
        for y in start_y..=end_y {
            let mut cell = Cell::new('│');
            cell.fg = Some(color);
            ctx.buffer.set(x2, y, cell);
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
                ctx.buffer.set(x as u16, y as u16, cell);
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
                ctx.buffer.set(x, y, cell);
            }
        }
    }
}
