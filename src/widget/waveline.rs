//! Waveline Chart Widget
//!
//! A smooth, flowing line chart for visualizing continuous data like audio waveforms,
//! signal processing data, or any oscillating values.
//!
//! # Features
//!
//! - Smooth curve interpolation
//! - Multiple display modes (line, filled, mirrored)
//! - Configurable amplitude and baseline
//! - Gradient fills
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{waveline, WaveStyle};
//!
//! let audio_data: Vec<f64> = get_audio_samples();
//! let wave = waveline(audio_data)
//!     .style(WaveStyle::Filled)
//!     .color(Color::CYAN)
//!     .baseline(0.5);
//! ```

use super::traits::{View, RenderContext, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_styled_view, impl_props_builders};

/// Display style for the waveline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WaveStyle {
    /// Simple line
    #[default]
    Line,
    /// Filled area under the line
    Filled,
    /// Mirrored (centered, like audio visualization)
    Mirrored,
    /// Bars instead of smooth line
    Bars,
    /// Dots at data points
    Dots,
    /// Smooth bezier curve
    Smooth,
}

/// Interpolation method for smooth curves
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Interpolation {
    /// No interpolation (connect points directly)
    #[default]
    Linear,
    /// Smooth bezier curves
    Bezier,
    /// Catmull-Rom spline
    CatmullRom,
    /// Step function
    Step,
}

/// Waveline chart widget
#[derive(Debug, Clone)]
pub struct Waveline {
    /// Data points (values between 0.0 and 1.0 work best)
    data: Vec<f64>,
    /// Display style
    style: WaveStyle,
    /// Interpolation method
    interpolation: Interpolation,
    /// Primary color
    color: Color,
    /// Secondary color for gradients
    gradient_color: Option<Color>,
    /// Baseline position (0.0 = bottom, 1.0 = top, 0.5 = center)
    baseline: f64,
    /// Amplitude multiplier
    amplitude: f64,
    /// Show zero line
    show_baseline: bool,
    /// Baseline color
    baseline_color: Color,
    /// Background color
    bg_color: Option<Color>,
    /// Height in rows
    height: Option<u16>,
    /// Maximum data points to display
    max_points: Option<usize>,
    /// Label
    label: Option<String>,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Default for Waveline {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl Waveline {
    /// Create a new waveline chart
    pub fn new(data: Vec<f64>) -> Self {
        Self {
            data,
            style: WaveStyle::Line,
            interpolation: Interpolation::Linear,
            color: Color::CYAN,
            gradient_color: None,
            baseline: 0.5,
            amplitude: 1.0,
            show_baseline: false,
            baseline_color: Color::rgb(80, 80, 80),
            bg_color: None,
            height: None,
            max_points: None,
            label: None,
            props: WidgetProps::new(),
        }
    }

    /// Set data points
    pub fn data(mut self, data: Vec<f64>) -> Self {
        self.data = data;
        self
    }

    /// Set display style
    pub fn style(mut self, style: WaveStyle) -> Self {
        self.style = style;
        self
    }

    /// Set interpolation method
    pub fn interpolation(mut self, method: Interpolation) -> Self {
        self.interpolation = method;
        self
    }

    /// Set line/fill color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set gradient colors
    pub fn gradient(mut self, start: Color, end: Color) -> Self {
        self.color = start;
        self.gradient_color = Some(end);
        self
    }

    /// Set baseline position (0.0 = bottom, 1.0 = top)
    pub fn baseline(mut self, position: f64) -> Self {
        self.baseline = position.clamp(0.0, 1.0);
        self
    }

    /// Set amplitude multiplier
    pub fn amplitude(mut self, amp: f64) -> Self {
        self.amplitude = amp;
        self
    }

    /// Show or hide baseline
    pub fn show_baseline(mut self, show: bool) -> Self {
        self.show_baseline = show;
        self
    }

    /// Set baseline color
    pub fn baseline_color(mut self, color: Color) -> Self {
        self.baseline_color = color;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set height
    pub fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }

    /// Set maximum points to display
    pub fn max_points(mut self, max: usize) -> Self {
        self.max_points = Some(max);
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    fn get_color(&self, ratio: f64) -> Color {
        if let Some(end) = self.gradient_color {
            let r = (self.color.r as f64 * (1.0 - ratio) + end.r as f64 * ratio) as u8;
            let g = (self.color.g as f64 * (1.0 - ratio) + end.g as f64 * ratio) as u8;
            let b = (self.color.b as f64 * (1.0 - ratio) + end.b as f64 * ratio) as u8;
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
            Interpolation::Linear => {
                data[idx_floor] * (1.0 - t) + data[idx_ceil] * t
            }
            Interpolation::Step => {
                data[idx_floor]
            }
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

        let mut chart_y = area.y;
        let mut chart_height = height.min(area.height);

        // Background
        if let Some(bg) = self.bg_color {
            for y in area.y..area.y + chart_height {
                for x in area.x..area.x + area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Label
        if let Some(ref label) = self.label {
            ctx.buffer.put_str_styled(area.x, chart_y, label, Some(Color::WHITE), self.bg_color);
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
            for x in area.x..area.x + area.width {
                let mut cell = Cell::new('─');
                cell.fg = Some(self.baseline_color);
                ctx.buffer.set(x, y, cell);
            }
        }

        match self.style {
            WaveStyle::Line | WaveStyle::Smooth => {
                for x in 0..width {
                    let val = (self.get_interpolated_value(data, x, width) * self.amplitude).clamp(-1.0, 1.0);
                    let y_ratio = self.baseline + val * (1.0 - self.baseline);
                    let y = chart_y + ((1.0 - y_ratio) * (chart_height - 1) as f64) as u16;

                    if y >= chart_y && y < chart_y + chart_height {
                        let screen_x = area.x + x as u16;
                        let mut cell = Cell::new('●');
                        cell.fg = Some(self.get_color(y_ratio));
                        ctx.buffer.set(screen_x, y, cell);
                    }
                }
            }
            WaveStyle::Filled => {
                let baseline_row = ((1.0 - self.baseline) * (chart_height - 1) as f64) as u16;

                for x in 0..width {
                    let val = (self.get_interpolated_value(data, x, width) * self.amplitude).clamp(-1.0, 1.0);
                    let y_ratio = self.baseline + val * (1.0 - self.baseline);
                    let y = ((1.0 - y_ratio) * (chart_height - 1) as f64) as u16;

                    let screen_x = area.x + x as u16;

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
                            cell.fg = Some(self.get_color(ratio));
                            ctx.buffer.set(screen_x, screen_y, cell);
                        }
                    }
                }
            }
            WaveStyle::Mirrored => {
                let center_y = chart_height / 2;

                for x in 0..width {
                    let val = (self.get_interpolated_value(data, x, width).abs() * self.amplitude).clamp(0.0, 1.0);
                    let half_height = (val * center_y as f64) as u16;

                    let screen_x = area.x + x as u16;

                    // Draw upper half
                    for dy in 0..=half_height {
                        let screen_y = chart_y + center_y.saturating_sub(dy);
                        if screen_y >= chart_y {
                            let intensity = 1.0 - dy as f64 / center_y as f64;
                            let ch = if dy == half_height { '▀' } else { '█' };
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(self.get_color(0.5 + intensity * 0.5));
                            ctx.buffer.set(screen_x, screen_y, cell);
                        }
                    }

                    // Draw lower half
                    for dy in 0..=half_height {
                        let screen_y = chart_y + center_y + dy;
                        if screen_y < chart_y + chart_height {
                            let intensity = 1.0 - dy as f64 / center_y as f64;
                            let ch = if dy == half_height { '▄' } else { '█' };
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(self.get_color(0.5 + intensity * 0.5));
                            ctx.buffer.set(screen_x, screen_y, cell);
                        }
                    }
                }
            }
            WaveStyle::Bars => {
                let baseline_row = ((1.0 - self.baseline) * (chart_height - 1) as f64) as u16;
                let bar_chars = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

                for x in 0..width {
                    let val = (self.get_interpolated_value(data, x, width) * self.amplitude).clamp(-1.0, 1.0);
                    let y_ratio = self.baseline + val * (1.0 - self.baseline);
                    let target_y = ((1.0 - y_ratio) * (chart_height - 1) as f64) as u16;

                    let screen_x = area.x + x as u16;

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
                                cell.fg = Some(self.get_color(y_ratio));
                                ctx.buffer.set(screen_x, screen_y, cell);
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
                                cell.fg = Some(self.get_color(y_ratio));
                                ctx.buffer.set(screen_x, screen_y, cell);
                            }
                        }
                    }
                }
            }
            WaveStyle::Dots => {
                for x in 0..width {
                    let val = (self.get_interpolated_value(data, x, width) * self.amplitude).clamp(-1.0, 1.0);
                    let y_ratio = self.baseline + val * (1.0 - self.baseline);
                    let y = chart_y + ((1.0 - y_ratio) * (chart_height - 1) as f64) as u16;

                    if y >= chart_y && y < chart_y + chart_height {
                        let screen_x = area.x + x as u16;
                        let mut cell = Cell::new('⣿');
                        cell.fg = Some(self.get_color(y_ratio));
                        ctx.buffer.set(screen_x, y, cell);
                    }
                }
            }
        }
    }

    crate::impl_view_meta!("Waveline");
}

impl_styled_view!(Waveline);
impl_props_builders!(Waveline);

// Convenience constructors

/// Create a new waveline chart
pub fn waveline(data: Vec<f64>) -> Waveline {
    Waveline::new(data)
}

/// Create an audio waveform visualization
pub fn audio_waveform(samples: Vec<f64>) -> Waveline {
    Waveline::new(samples)
        .style(WaveStyle::Mirrored)
        .color(Color::CYAN)
        .gradient(Color::BLUE, Color::CYAN)
}

/// Create a signal wave visualization
pub fn signal_wave(data: Vec<f64>) -> Waveline {
    Waveline::new(data)
        .style(WaveStyle::Line)
        .interpolation(Interpolation::CatmullRom)
        .color(Color::GREEN)
        .show_baseline(true)
}

/// Create a filled area wave
pub fn area_wave(data: Vec<f64>) -> Waveline {
    Waveline::new(data)
        .style(WaveStyle::Filled)
        .color(Color::MAGENTA)
        .baseline(1.0)
}

/// Create a bar spectrum visualization
pub fn spectrum(data: Vec<f64>) -> Waveline {
    Waveline::new(data)
        .style(WaveStyle::Bars)
        .color(Color::YELLOW)
        .baseline(1.0)
}

/// Generate sine wave data
pub fn sine_wave(samples: usize, frequency: f64, amplitude: f64) -> Vec<f64> {
    (0..samples)
        .map(|i| {
            let t = i as f64 / samples as f64 * std::f64::consts::PI * 2.0 * frequency;
            t.sin() * amplitude
        })
        .collect()
}

/// Generate square wave data
pub fn square_wave(samples: usize, frequency: f64, amplitude: f64) -> Vec<f64> {
    (0..samples)
        .map(|i| {
            let t = i as f64 / samples as f64 * frequency;
            if t.fract() < 0.5 { amplitude } else { -amplitude }
        })
        .collect()
}

/// Generate sawtooth wave data
pub fn sawtooth_wave(samples: usize, frequency: f64, amplitude: f64) -> Vec<f64> {
    (0..samples)
        .map(|i| {
            let t = i as f64 / samples as f64 * frequency;
            (t.fract() * 2.0 - 1.0) * amplitude
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waveline_creation() {
        let data = vec![0.0, 0.5, 1.0, 0.5, 0.0];
        let wave = waveline(data.clone());

        assert_eq!(wave.data, data);
    }

    #[test]
    fn test_waveline_styles() {
        let data = vec![0.5; 10];
        let wave = waveline(data)
            .style(WaveStyle::Mirrored)
            .color(Color::RED)
            .amplitude(0.8);

        assert_eq!(wave.style, WaveStyle::Mirrored);
        assert_eq!(wave.color, Color::RED);
        assert_eq!(wave.amplitude, 0.8);
    }

    #[test]
    fn test_sine_wave_generation() {
        let data = sine_wave(100, 2.0, 1.0);
        assert_eq!(data.len(), 100);
        assert!(data.iter().all(|&v| v >= -1.0 && v <= 1.0));
    }

    #[test]
    fn test_interpolation() {
        let wave = waveline(vec![0.0, 1.0, 0.0])
            .interpolation(Interpolation::CatmullRom);

        assert_eq!(wave.interpolation, Interpolation::CatmullRom);
    }
}
