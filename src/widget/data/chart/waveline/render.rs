//! Waveline rendering: color gradient, interpolation and the `View` impl

use super::{Interpolation, WaveStyle, Waveline};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

impl Waveline {
    fn get_color(&self, ratio: f64) -> Color {
        if let Some(end) = self.gradient_color {
            let r = (self.color.r as f64 * (1.0 - ratio) + end.r as f64 * ratio).round() as u8;
            let g = (self.color.g as f64 * (1.0 - ratio) + end.g as f64 * ratio).round() as u8;
            let b = (self.color.b as f64 * (1.0 - ratio) + end.b as f64 * ratio).round() as u8;
            Color::rgb(r, g, b)
        } else {
            self.color
        }
    }

    fn get_interpolated_value(&self, data: &[f64], x: usize, width: usize) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        let ratio = x as f64 / (width - 1).max(1) as f64;
        let idx = ratio * (data.len() - 1) as f64;
        let idx_floor = idx.floor() as usize;
        let idx_ceil = (idx_floor + 1).min(data.len() - 1);
        let t = idx - idx_floor as f64;

        match self.interpolation {
            Interpolation::Linear => data[idx_floor] * (1.0 - t) + data[idx_ceil] * t,
            Interpolation::Step => data[idx_floor],
            Interpolation::Bezier | Interpolation::CatmullRom => {
                let p0_idx = idx_floor.saturating_sub(1);
                let p3_idx = (idx_ceil + 1).min(data.len() - 1);

                let p0 = data[p0_idx];
                let p1 = data[idx_floor];
                let p2 = data[idx_ceil];
                let p3 = data[p3_idx];

                let t2 = t * t;
                let t3 = t2 * t;

                0.5 * ((2.0 * p1)
                    + (-p0 + p2) * t
                    + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
                    + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
            }
        }
    }
}

impl View for Waveline {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let height = self.height.unwrap_or(area.height);

        if area.width < 2 || height < 1 {
            return;
        }

        let mut chart_y = 0u16;
        let mut chart_height = height.min(area.height);

        // Background
        if let Some(bg) = self.bg_color {
            for y in 0..chart_height {
                for x in 0..area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.set(x, y, cell);
                }
            }
        }

        // The waveform's own colors are computed per sample from the amplitude,
        // so there is no single color for a rule to set - the same reason
        // `GradientBox` is left alone. The label is ordinary text.
        let chrome = ctx.css_color(Color::WHITE);

        // Label
        if let Some(ref label) = self.label {
            let bg = self.bg_color;
            ctx.put_str_with(0, chart_y, label, u16::MAX, |ch| {
                let mut cell = Cell::new(ch);
                cell.fg = Some(chrome);
                cell.bg = bg;
                cell
            });
            chart_y += 1;
            chart_height = chart_height.saturating_sub(1);
        }

        if chart_height < 1 || self.data.is_empty() {
            return;
        }

        // Determine data range
        let data = if let Some(max) = self.max_points {
            if self.data.len() > max {
                &self.data[self.data.len() - max..]
            } else {
                &self.data[..]
            }
        } else {
            &self.data[..]
        };

        let width = area.width as usize;

        // Draw baseline
        if self.show_baseline {
            let baseline_row = ((1.0 - self.baseline) * (chart_height - 1) as f64) as u16;
            let y = chart_y + baseline_row;
            for x in 0..area.width {
                let mut cell = Cell::new('─');
                cell.bg = self.bg_color;
                cell.fg = Some(self.baseline_color);
                ctx.set(x, y, cell);
            }
        }

        match self.style {
            WaveStyle::Line | WaveStyle::Smooth => {
                for x in 0..width {
                    let val = (self.get_interpolated_value(data, x, width) * self.amplitude)
                        .clamp(-1.0, 1.0);
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
            WaveStyle::Filled => {
                let baseline_row = ((1.0 - self.baseline) * (chart_height - 1) as f64) as u16;

                for x in 0..width {
                    let val = (self.get_interpolated_value(data, x, width) * self.amplitude)
                        .clamp(-1.0, 1.0);
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
            WaveStyle::Mirrored => {
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
            WaveStyle::Bars => {
                let baseline_row = ((1.0 - self.baseline) * (chart_height - 1) as f64) as u16;
                let bar_chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

                for x in 0..width {
                    let val = (self.get_interpolated_value(data, x, width) * self.amplitude)
                        .clamp(-1.0, 1.0);
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
            WaveStyle::Dots => {
                for x in 0..width {
                    let val = (self.get_interpolated_value(data, x, width) * self.amplitude)
                        .clamp(-1.0, 1.0);
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
    }

    crate::impl_view_meta!("Waveline");
}
