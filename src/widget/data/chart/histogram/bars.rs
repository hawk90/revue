//! Histogram bars, vertical and horizontal

use super::Histogram;
use crate::layout::Rect;
use crate::render::Cell;
use crate::widget::traits::RenderContext;

impl Histogram {
    /// Render histogram bars
    pub(super) fn render_bars(&self, ctx: &mut RenderContext, chart_area: Rect) {
        if self.bins.is_empty() {
            return;
        }

        let x_min = self.bins.first().map(|b| b.start).unwrap_or(0.0);
        let x_max = self.bins.last().map(|b| b.end).unwrap_or(1.0);
        let x_range = (x_max - x_min).max(1.0);
        let y_max = self.max_value();

        let mut cumulative_sum = 0.0;

        for bin in &self.bins {
            cumulative_sum += bin.frequency;
            let value = self.bin_value(bin, cumulative_sum);

            // Calculate bar position
            let bar_x_start = ((bin.start - x_min) / x_range * chart_area.width as f64) as u16;
            let bar_x_end = ((bin.end - x_min) / x_range * chart_area.width as f64) as u16;
            let bar_width = (bar_x_end - bar_x_start).max(1);

            let bar_height = ((value / y_max) * chart_area.height as f64) as u16;
            let bar_height = bar_height.min(chart_area.height);

            // Draw bar
            for dx in 0..bar_width {
                for dy in 0..bar_height {
                    let x = chart_area.x + bar_x_start + dx;
                    let y = chart_area.y + chart_area.height - 1 - dy;

                    if x < chart_area.x + chart_area.width && y >= chart_area.y {
                        let ch = if dy == bar_height - 1 {
                            '▀'
                        } else if self.bar_border.is_some() && (dx == 0 || dx == bar_width - 1) {
                            '│'
                        } else {
                            '█'
                        };

                        let mut cell = Cell::new(ch);
                        if self.bar_border.is_some() && (dx == 0 || dx == bar_width - 1) {
                            cell.fg = self.bar_border;
                        } else {
                            cell.fg = Some(ctx.css_color(self.fill_color));
                        }
                        ctx.set(x, y, cell);
                    }
                }
            }
        }
    }

    /// Render histogram bars horizontally: bins top to bottom, bars growing
    /// to the right
    pub(super) fn render_bars_horizontal(&self, ctx: &mut RenderContext, chart_area: Rect) {
        if self.bins.is_empty() {
            return;
        }

        let x_min = self.bins.first().map(|b| b.start).unwrap_or(0.0);
        let x_max = self.bins.last().map(|b| b.end).unwrap_or(1.0);
        let x_range = (x_max - x_min).max(1.0);
        let y_max = self.max_value();

        let mut cumulative_sum = 0.0;

        for bin in &self.bins {
            cumulative_sum += bin.frequency;
            let value = self.bin_value(bin, cumulative_sum);

            // Calculate bar position
            let bar_y_start = ((bin.start - x_min) / x_range * chart_area.height as f64) as u16;
            let bar_y_end = ((bin.end - x_min) / x_range * chart_area.height as f64) as u16;
            let bar_thickness = bar_y_end.saturating_sub(bar_y_start).max(1);

            let bar_length = ((value / y_max) * chart_area.width as f64) as u16;
            let bar_length = bar_length.min(chart_area.width);

            // Draw bar
            for dy in 0..bar_thickness {
                for dx in 0..bar_length {
                    let x = chart_area.x + dx;
                    let y = chart_area.y + bar_y_start + dy;

                    if y < chart_area.y + chart_area.height {
                        let is_border =
                            self.bar_border.is_some() && (dy == 0 || dy == bar_thickness - 1);
                        let ch = if dx == bar_length - 1 {
                            '▌'
                        } else if is_border {
                            '─'
                        } else {
                            '█'
                        };

                        let mut cell = Cell::new(ch);
                        if is_border {
                            cell.fg = self.bar_border;
                        } else {
                            cell.fg = Some(ctx.css_color(self.fill_color));
                        }
                        ctx.set(x, y, cell);
                    }
                }
            }
        }
    }
}
