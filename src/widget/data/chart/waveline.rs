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

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

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
            ctx.buffer
                .put_str_styled(area.x, chart_y, label, Some(Color::WHITE), self.bg_color);
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
                    let val = (self.get_interpolated_value(data, x, width) * self.amplitude)
                        .clamp(-1.0, 1.0);
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
                    let val = (self.get_interpolated_value(data, x, width) * self.amplitude)
                        .clamp(-1.0, 1.0);
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
                    let val = (self.get_interpolated_value(data, x, width).abs() * self.amplitude)
                        .clamp(0.0, 1.0);
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
                    let val = (self.get_interpolated_value(data, x, width) * self.amplitude)
                        .clamp(-1.0, 1.0);
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
                    let val = (self.get_interpolated_value(data, x, width) * self.amplitude)
                        .clamp(-1.0, 1.0);
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
        .gradient(Color::CYAN, Color::BLUE)
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
            if t.fract() < 0.5 {
                amplitude
            } else {
                -amplitude
            }
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
        let wave = waveline(vec![0.0, 1.0, 0.0]).interpolation(Interpolation::CatmullRom);

        assert_eq!(wave.interpolation, Interpolation::CatmullRom);
    }

    // =========================================================================
    // WaveStyle enum tests
    // =========================================================================

    #[test]
    fn test_wave_style_default() {
        assert_eq!(WaveStyle::default(), WaveStyle::Line);
    }

    #[test]
    fn test_wave_style_clone() {
        let style1 = WaveStyle::Filled;
        let style2 = style1.clone();
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_wave_style_copy() {
        let style1 = WaveStyle::Bars;
        let style2 = style1;
        assert_eq!(style2, WaveStyle::Bars);
    }

    #[test]
    fn test_wave_style_partial_eq() {
        assert_eq!(WaveStyle::Line, WaveStyle::Line);
        assert_eq!(WaveStyle::Filled, WaveStyle::Filled);
        assert_ne!(WaveStyle::Line, WaveStyle::Mirrored);
    }

    #[test]
    fn test_wave_style_debug() {
        let debug_str = format!("{:?}", WaveStyle::Line);
        assert!(debug_str.contains("Line"));
    }

    // =========================================================================
    // Interpolation enum tests
    // =========================================================================

    #[test]
    fn test_interpolation_default() {
        assert_eq!(Interpolation::default(), Interpolation::Linear);
    }

    #[test]
    fn test_interpolation_clone() {
        let interp1 = Interpolation::Bezier;
        let interp2 = interp1.clone();
        assert_eq!(interp1, interp2);
    }

    #[test]
    fn test_interpolation_copy() {
        let interp1 = Interpolation::CatmullRom;
        let interp2 = interp1;
        assert_eq!(interp2, Interpolation::CatmullRom);
    }

    #[test]
    fn test_interpolation_partial_eq() {
        assert_eq!(Interpolation::Linear, Interpolation::Linear);
        assert_eq!(Interpolation::Bezier, Interpolation::Bezier);
        assert_ne!(Interpolation::Linear, Interpolation::Step);
    }

    #[test]
    fn test_interpolation_debug() {
        let debug_str = format!("{:?}", Interpolation::Bezier);
        assert!(debug_str.contains("Bezier"));
    }

    // =========================================================================
    // Waveline::new tests
    // =========================================================================

    #[test]
    fn test_waveline_new_empty() {
        let wave = Waveline::new(vec![]);
        assert!(wave.data.is_empty());
        assert_eq!(wave.style, WaveStyle::Line);
        assert_eq!(wave.color, Color::CYAN);
    }

    #[test]
    fn test_waveline_new_with_data() {
        let data = vec![0.5, 0.7, 0.3, 0.9];
        let wave = Waveline::new(data.clone());
        assert_eq!(wave.data, data);
    }

    // =========================================================================
    // Waveline::data tests
    // =========================================================================

    #[test]
    fn test_waveline_data_builder() {
        let wave = Waveline::new(vec![0.5]).data(vec![0.1, 0.2, 0.3]);
        assert_eq!(wave.data, vec![0.1, 0.2, 0.3]);
    }

    // =========================================================================
    // Waveline::style tests
    // =========================================================================

    #[test]
    fn test_waveline_style_filled() {
        let wave = Waveline::new(vec![0.5]).style(WaveStyle::Filled);
        assert_eq!(wave.style, WaveStyle::Filled);
    }

    #[test]
    fn test_waveline_style_mirrored() {
        let wave = Waveline::new(vec![0.5]).style(WaveStyle::Mirrored);
        assert_eq!(wave.style, WaveStyle::Mirrored);
    }

    #[test]
    fn test_waveline_style_bars() {
        let wave = Waveline::new(vec![0.5]).style(WaveStyle::Bars);
        assert_eq!(wave.style, WaveStyle::Bars);
    }

    #[test]
    fn test_waveline_style_dots() {
        let wave = Waveline::new(vec![0.5]).style(WaveStyle::Dots);
        assert_eq!(wave.style, WaveStyle::Dots);
    }

    #[test]
    fn test_waveline_style_smooth() {
        let wave = Waveline::new(vec![0.5]).style(WaveStyle::Smooth);
        assert_eq!(wave.style, WaveStyle::Smooth);
    }

    // =========================================================================
    // Waveline::interpolation tests
    // =========================================================================

    #[test]
    fn test_waveline_interpolation_linear() {
        let wave = Waveline::new(vec![0.5]).interpolation(Interpolation::Linear);
        assert_eq!(wave.interpolation, Interpolation::Linear);
    }

    #[test]
    fn test_waveline_interpolation_bezier() {
        let wave = Waveline::new(vec![0.5]).interpolation(Interpolation::Bezier);
        assert_eq!(wave.interpolation, Interpolation::Bezier);
    }

    #[test]
    fn test_waveline_interpolation_step() {
        let wave = Waveline::new(vec![0.5]).interpolation(Interpolation::Step);
        assert_eq!(wave.interpolation, Interpolation::Step);
    }

    // =========================================================================
    // Waveline::color tests
    // =========================================================================

    #[test]
    fn test_waveline_color() {
        let wave = Waveline::new(vec![0.5]).color(Color::RED);
        assert_eq!(wave.color, Color::RED);
    }

    // =========================================================================
    // Waveline::gradient tests
    // =========================================================================

    #[test]
    fn test_waveline_gradient() {
        let wave = Waveline::new(vec![0.5]).gradient(Color::BLUE, Color::GREEN);
        assert_eq!(wave.color, Color::BLUE);
        assert_eq!(wave.gradient_color, Some(Color::GREEN));
    }

    // =========================================================================
    // Waveline::baseline tests
    // =========================================================================

    #[test]
    fn test_waveline_baseline() {
        let wave = Waveline::new(vec![0.5]).baseline(0.7);
        assert_eq!(wave.baseline, 0.7);
    }

    #[test]
    fn test_waveline_baseline_clamp_low() {
        let wave = Waveline::new(vec![0.5]).baseline(-0.5);
        assert_eq!(wave.baseline, 0.0);
    }

    #[test]
    fn test_waveline_baseline_clamp_high() {
        let wave = Waveline::new(vec![0.5]).baseline(1.5);
        assert_eq!(wave.baseline, 1.0);
    }

    // =========================================================================
    // Waveline::amplitude tests
    // =========================================================================

    #[test]
    fn test_waveline_amplitude() {
        let wave = Waveline::new(vec![0.5]).amplitude(2.0);
        assert_eq!(wave.amplitude, 2.0);
    }

    // =========================================================================
    // Waveline::show_baseline tests
    // =========================================================================

    #[test]
    fn test_waveline_show_baseline_true() {
        let wave = Waveline::new(vec![0.5]).show_baseline(true);
        assert!(wave.show_baseline);
    }

    #[test]
    fn test_waveline_show_baseline_false() {
        let wave = Waveline::new(vec![0.5]).show_baseline(false);
        assert!(!wave.show_baseline);
    }

    // =========================================================================
    // Waveline::baseline_color tests
    // =========================================================================

    #[test]
    fn test_waveline_baseline_color() {
        let wave = Waveline::new(vec![0.5]).baseline_color(Color::YELLOW);
        assert_eq!(wave.baseline_color, Color::YELLOW);
    }

    // =========================================================================
    // Waveline::bg tests
    // =========================================================================

    #[test]
    fn test_waveline_bg() {
        let wave = Waveline::new(vec![0.5]).bg(Color::BLACK);
        assert_eq!(wave.bg_color, Some(Color::BLACK));
    }

    // =========================================================================
    // Waveline::height tests
    // =========================================================================

    #[test]
    fn test_waveline_height() {
        let wave = Waveline::new(vec![0.5]).height(10);
        assert_eq!(wave.height, Some(10));
    }

    // =========================================================================
    // Waveline::max_points tests
    // =========================================================================

    #[test]
    fn test_waveline_max_points() {
        let wave = Waveline::new(vec![0.5]).max_points(100);
        assert_eq!(wave.max_points, Some(100));
    }

    // =========================================================================
    // Waveline::label tests
    // =========================================================================

    #[test]
    fn test_waveline_label_str() {
        let wave = Waveline::new(vec![0.5]).label("Audio");
        assert_eq!(wave.label, Some("Audio".to_string()));
    }

    #[test]
    fn test_waveline_label_string() {
        let wave = Waveline::new(vec![0.5]).label(String::from("Wave"));
        assert_eq!(wave.label, Some("Wave".to_string()));
    }

    // =========================================================================
    // Waveline Default trait
    // =========================================================================

    #[test]
    fn test_waveline_default() {
        let wave = Waveline::default();
        assert!(wave.data.is_empty());
        assert_eq!(wave.style, WaveStyle::Line);
    }

    // =========================================================================
    // Waveline Clone trait
    // =========================================================================

    #[test]
    fn test_waveline_clone() {
        let wave1 = Waveline::new(vec![0.5, 0.7]).color(Color::RED);
        let wave2 = wave1.clone();
        assert_eq!(wave1.data, wave2.data);
        assert_eq!(wave1.color, wave2.color);
    }

    // =========================================================================
    // Waveline Debug trait
    // =========================================================================

    #[test]
    fn test_waveline_debug() {
        let wave = Waveline::new(vec![0.5]);
        let debug_str = format!("{:?}", wave);
        assert!(debug_str.contains("Waveline"));
    }

    // =========================================================================
    // Waveline::get_color tests
    // =========================================================================

    #[test]
    fn test_get_color_no_gradient() {
        let wave = Waveline::new(vec![0.5]).color(Color::BLUE);
        let color = wave.get_color(0.5);
        assert_eq!(color, Color::BLUE);
    }

    #[test]
    fn test_get_color_with_gradient_start() {
        let wave = Waveline::new(vec![0.5]).gradient(Color::RED, Color::BLUE);
        let color = wave.get_color(0.0);
        assert_eq!(color, Color::RED);
    }

    #[test]
    fn test_get_color_with_gradient_end() {
        let wave = Waveline::new(vec![0.5]).gradient(Color::RED, Color::BLUE);
        let color = wave.get_color(1.0);
        assert_eq!(color, Color::BLUE);
    }

    #[test]
    fn test_get_color_with_gradient_mid() {
        let wave = Waveline::new(vec![0.5]).gradient(Color::BLACK, Color::WHITE);
        let color = wave.get_color(0.5);
        // Midpoint should be gray
        assert_eq!(color.r, 128);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 128);
    }

    // =========================================================================
    // Wave generation helper tests
    // =========================================================================

    #[test]
    fn test_sine_wave_frequency() {
        let data = sine_wave(100, 1.0, 1.0);
        assert_eq!(data.len(), 100);
    }

    #[test]
    fn test_sine_wave_amplitude() {
        let data = sine_wave(100, 1.0, 0.5);
        // All values should be within amplitude
        assert!(data.iter().all(|&v| v >= -0.5 && v <= 0.5));
    }

    #[test]
    fn test_square_wave_generation() {
        let data = square_wave(100, 1.0, 1.0);
        assert_eq!(data.len(), 100);
        // Square wave should only have two values
        let unique_vals: std::collections::HashSet<_> =
            data.iter().map(|&v| (v * 10.0).round() as i32).collect();
        assert_eq!(unique_vals.len(), 2);
    }

    #[test]
    fn test_sawtooth_wave_generation() {
        let data = sawtooth_wave(100, 1.0, 1.0);
        assert_eq!(data.len(), 100);
        assert!(data.iter().all(|&v| v >= -1.0 && v <= 1.0));
    }

    // =========================================================================
    // Convenience constructor tests
    // =========================================================================

    #[test]
    fn test_audio_waveform() {
        let wave = audio_waveform(vec![0.5, 0.7, 0.3]);
        assert_eq!(wave.style, WaveStyle::Mirrored);
        assert_eq!(wave.color, Color::CYAN);
        assert!(wave.gradient_color.is_some());
    }

    #[test]
    fn test_signal_wave() {
        let wave = signal_wave(vec![0.5, 0.7, 0.3]);
        assert_eq!(wave.style, WaveStyle::Line);
        assert_eq!(wave.interpolation, Interpolation::CatmullRom);
        assert_eq!(wave.color, Color::GREEN);
        assert!(wave.show_baseline);
    }

    #[test]
    fn test_area_wave() {
        let wave = area_wave(vec![0.5, 0.7, 0.3]);
        assert_eq!(wave.style, WaveStyle::Filled);
        assert_eq!(wave.color, Color::MAGENTA);
        assert_eq!(wave.baseline, 1.0);
    }

    #[test]
    fn test_spectrum() {
        let wave = spectrum(vec![0.5, 0.7, 0.3]);
        assert_eq!(wave.style, WaveStyle::Bars);
        assert_eq!(wave.color, Color::YELLOW);
        assert_eq!(wave.baseline, 1.0);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_waveline_builder_chain() {
        let wave = Waveline::new(vec![0.5, 0.7, 0.3])
            .style(WaveStyle::Mirrored)
            .interpolation(Interpolation::Bezier)
            .color(Color::RED)
            .gradient(Color::RED, Color::YELLOW)
            .baseline(0.3)
            .amplitude(1.5)
            .show_baseline(true)
            .baseline_color(Color::rgb(128, 128, 128))
            .bg(Color::BLACK)
            .height(15)
            .max_points(200)
            .label("Test Wave");

        assert_eq!(wave.style, WaveStyle::Mirrored);
        assert_eq!(wave.interpolation, Interpolation::Bezier);
        assert_eq!(wave.color, Color::RED);
        assert_eq!(wave.baseline, 0.3);
        assert_eq!(wave.amplitude, 1.5);
        assert!(wave.show_baseline);
        assert_eq!(wave.height, Some(15));
        assert_eq!(wave.max_points, Some(200));
        assert_eq!(wave.label, Some("Test Wave".to_string()));
    }

    // =========================================================================
    // Render tests
    // =========================================================================

    #[test]
    fn test_waveline_render_basic() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![0.2, 0.5, 0.8, 0.5, 0.2];
        let wave = Waveline::new(data);

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        wave.render(&mut ctx);
        // Should render without panic
    }

    #[test]
    fn test_waveline_render_with_label() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![0.5, 0.7, 0.3];
        let wave = Waveline::new(data).label("Audio");

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        wave.render(&mut ctx);

        // Label should be rendered
        let mut label_found = false;
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, 0) {
                if cell.symbol == 'A' {
                    label_found = true;
                    break;
                }
            }
        }
        assert!(label_found);
    }

    #[test]
    fn test_waveline_render_filled() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![0.3, 0.7, 0.5, 0.9, 0.4];
        let wave = Waveline::new(data).style(WaveStyle::Filled);

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        wave.render(&mut ctx);
        // Should render filled area
    }

    #[test]
    fn test_waveline_render_mirrored() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![0.3, 0.7, 0.5, 0.9, 0.4];
        let wave = Waveline::new(data).style(WaveStyle::Mirrored);

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        wave.render(&mut ctx);
        // Should render mirrored wave
    }

    #[test]
    fn test_waveline_render_bars() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![0.3, 0.7, 0.5];
        let wave = Waveline::new(data).style(WaveStyle::Bars);

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        wave.render(&mut ctx);
        // Should render bars
    }

    #[test]
    fn test_waveline_render_dots() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![0.3, 0.7, 0.5];
        let wave = Waveline::new(data).style(WaveStyle::Dots);

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        wave.render(&mut ctx);
        // Should render dots
    }

    #[test]
    fn test_waveline_render_smooth() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![0.2, 0.5, 0.8, 0.5, 0.2];
        let wave = Waveline::new(data).style(WaveStyle::Smooth);

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        wave.render(&mut ctx);
        // Should render smooth curve
    }

    #[test]
    fn test_waveline_render_with_background() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![0.5, 0.7, 0.3];
        let wave = Waveline::new(data).bg(Color::BLUE);

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        wave.render(&mut ctx);
        // Should render with background
    }

    #[test]
    fn test_waveline_render_with_baseline() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![0.5, 0.7, 0.3];
        let wave = Waveline::new(data).show_baseline(true);

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        wave.render(&mut ctx);
        // Should render baseline
    }

    #[test]
    fn test_waveline_render_empty() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let wave = Waveline::new(vec![]);

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        wave.render(&mut ctx);
        // Should handle empty data gracefully
    }

    #[test]
    fn test_waveline_render_small_area() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![0.5, 0.7, 0.3];
        let wave = Waveline::new(data);

        let mut buffer = Buffer::new(5, 3);
        let area = Rect::new(0, 0, 5, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        wave.render(&mut ctx);
        // Should handle small area
    }

    #[test]
    fn test_waveline_render_negative_values() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data = vec![-0.5, 0.0, 0.5, 0.0, -0.5];
        let wave = Waveline::new(data);

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        wave.render(&mut ctx);
        // Should handle negative values
    }

    #[test]
    fn test_waveline_render_max_points() {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::RenderContext;

        let data: Vec<f64> = (0..100).map(|i| i as f64 / 100.0).collect();
        let wave = Waveline::new(data).max_points(20);

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        wave.render(&mut ctx);
        // Should only render last 20 points
    }
}
