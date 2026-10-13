//! The wave in each style, one method per `WaveStyle`

use super::Waveline;
use crate::render::Cell;
use crate::widget::traits::RenderContext;

impl Waveline {
    /// Line and Smooth: one dot per column at the value's height
    pub(super) fn draw_line(
        &self,
        ctx: &mut RenderContext,
        data: &[f64],
        chart_y: u16,
        chart_height: u16,
    ) {
        let width = ctx.area.width as usize;

        for x in 0..width {
            let val =
                (self.get_interpolated_value(data, x, width) * self.amplitude).clamp(-1.0, 1.0);
            let y_ratio = self.baseline + val * (1.0 - self.baseline);
            let y = chart_y + ((1.0 - y_ratio) * (chart_height - 1) as f64) as u16;

            if y >= chart_y && y < chart_y + chart_height {
                let screen_x = x as u16;
                let mut cell = Cell::new('●');
                cell.bg = self.bg_color;
                cell.fg = Some(self.get_color(y_ratio));
                ctx.set(screen_x, y, cell);
            }
        }
    }

    /// Filled: each column from the value to the baseline
    pub(super) fn draw_filled(
        &self,
        ctx: &mut RenderContext,
        data: &[f64],
        chart_y: u16,
        chart_height: u16,
    ) {
        let width = ctx.area.width as usize;

        let baseline_row = ((1.0 - self.baseline) * (chart_height - 1) as f64) as u16;

        for x in 0..width {
            let val =
                (self.get_interpolated_value(data, x, width) * self.amplitude).clamp(-1.0, 1.0);
            let y_ratio = self.baseline + val * (1.0 - self.baseline);
            let y = ((1.0 - y_ratio) * (chart_height - 1) as f64) as u16;

            let screen_x = x as u16;

            let (start_y, end_y) = if y <= baseline_row {
                (y, baseline_row)
            } else {
                (baseline_row, y)
            };

            for dy in start_y..=end_y {
                if dy < chart_height {
                    let screen_y = chart_y + dy;
                    let ch = if dy == y { '█' } else { '▓' };
                    let ratio = 1.0 - dy as f64 / (chart_height - 1) as f64;
                    let mut cell = Cell::new(ch);
                    cell.bg = self.bg_color;
                    cell.fg = Some(self.get_color(ratio));
                    ctx.set(screen_x, screen_y, cell);
                }
            }
        }
    }

    /// Mirrored: each column's magnitude above and below the center
    pub(super) fn draw_mirrored(
        &self,
        ctx: &mut RenderContext,
        data: &[f64],
        chart_y: u16,
        chart_height: u16,
    ) {
        let width = ctx.area.width as usize;

        let center_y = chart_height / 2;

        for x in 0..width {
            let val = (self.get_interpolated_value(data, x, width).abs() * self.amplitude)
                .clamp(0.0, 1.0);
            let half_height = (val * center_y as f64) as u16;

            let screen_x = x as u16;

            // Draw upper half
            for dy in 0..=half_height {
                let screen_y = chart_y + center_y.saturating_sub(dy);
                if screen_y >= chart_y {
                    let intensity = 1.0 - dy as f64 / center_y as f64;
                    let ch = if dy == half_height { '▀' } else { '█' };
                    let mut cell = Cell::new(ch);
                    cell.bg = self.bg_color;
                    cell.fg = Some(self.get_color(0.5 + intensity * 0.5));
                    ctx.set(screen_x, screen_y, cell);
                }
            }

            // Draw lower half
            for dy in 0..=half_height {
                let screen_y = chart_y + center_y + dy;
                if screen_y < chart_y + chart_height {
                    let intensity = 1.0 - dy as f64 / center_y as f64;
                    let ch = if dy == half_height { '▄' } else { '█' };
                    let mut cell = Cell::new(ch);
                    cell.bg = self.bg_color;
                    cell.fg = Some(self.get_color(0.5 + intensity * 0.5));
                    ctx.set(screen_x, screen_y, cell);
                }
            }
        }
    }

    /// Bars: each column from the baseline to the value, with an eighth-block tip
    pub(super) fn draw_bars(
        &self,
        ctx: &mut RenderContext,
        data: &[f64],
        chart_y: u16,
        chart_height: u16,
    ) {
        let width = ctx.area.width as usize;

        let baseline_row = ((1.0 - self.baseline) * (chart_height - 1) as f64) as u16;
        let bar_chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

        for x in 0..width {
            let val =
                (self.get_interpolated_value(data, x, width) * self.amplitude).clamp(-1.0, 1.0);
            let y_ratio = self.baseline + val * (1.0 - self.baseline);
            let target_y = ((1.0 - y_ratio) * (chart_height - 1) as f64) as u16;

            let screen_x = x as u16;

            if val >= 0.0 {
                for dy in target_y..=baseline_row {
                    if dy < chart_height {
                        let screen_y = chart_y + dy;
                        let ch = if dy == target_y {
                            let frac = (y_ratio * 8.0).fract();
                            bar_chars[(frac * 8.0) as usize % 8]
                        } else {
                            '█'
                        };
                        let mut cell = Cell::new(ch);
                        cell.bg = self.bg_color;
                        cell.fg = Some(self.get_color(y_ratio));
                        ctx.set(screen_x, screen_y, cell);
                    }
                }
            } else {
                for dy in baseline_row..=target_y {
                    if dy < chart_height {
                        let screen_y = chart_y + dy;
                        let ch = if dy == target_y {
                            let frac = 1.0 - (y_ratio * 8.0).fract();
                            bar_chars[(frac * 8.0) as usize % 8]
                        } else {
                            '█'
                        };
                        let mut cell = Cell::new(ch);
                        cell.bg = self.bg_color;
                        cell.fg = Some(self.get_color(y_ratio));
                        ctx.set(screen_x, screen_y, cell);
                    }
                }
            }
        }
    }

    /// Dots: one braille cell per column at the value's height
    pub(super) fn draw_dots(
        &self,
        ctx: &mut RenderContext,
        data: &[f64],
        chart_y: u16,
        chart_height: u16,
    ) {
        let width = ctx.area.width as usize;

        for x in 0..width {
            let val =
                (self.get_interpolated_value(data, x, width) * self.amplitude).clamp(-1.0, 1.0);
            let y_ratio = self.baseline + val * (1.0 - self.baseline);
            let y = chart_y + ((1.0 - y_ratio) * (chart_height - 1) as f64) as u16;

            if y >= chart_y && y < chart_y + chart_height {
                let screen_x = x as u16;
                let mut cell = Cell::new('⣿');
                cell.bg = self.bg_color;
                cell.fg = Some(self.get_color(y_ratio));
                ctx.set(screen_x, y, cell);
            }
        }
    }
}
