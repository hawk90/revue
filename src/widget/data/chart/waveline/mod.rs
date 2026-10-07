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

mod generators;
mod render;
mod types;

pub use generators::{sawtooth_wave, sine_wave, square_wave};
pub use types::{Interpolation, WaveStyle};

use crate::style::Color;
use crate::widget::theme::DARK_GRAY;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

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
            baseline_color: DARK_GRAY,
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
        .baseline(0.0)
}

/// Create a bar spectrum visualization
pub fn spectrum(data: Vec<f64>) -> Waveline {
    Waveline::new(data)
        .style(WaveStyle::Bars)
        .color(Color::YELLOW)
        .baseline(0.0)
}
