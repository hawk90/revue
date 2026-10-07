//! Gauge widget for displaying metrics and progress
//!
//! Advanced progress indicators with various styles including
//! speedometer, arc, battery, and more.

mod render;
mod types;

pub use types::{GaugeStyle, LabelPosition};

use crate::style::Color;
use crate::widget::theme::SEPARATOR_COLOR;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Gauge widget
#[derive(Clone)]
pub struct Gauge {
    /// Current value (0.0 - 1.0)
    value: f64,
    /// Minimum value for display
    min: f64,
    /// Maximum value for display
    max: f64,
    /// Visual style
    style: GaugeStyle,
    /// Width (for horizontal styles)
    width: u16,
    /// Height (for vertical styles)
    height: u16,
    /// Label format
    label: Option<String>,
    /// Label position
    label_position: LabelPosition,
    /// Show percentage
    show_percent: bool,
    /// Filled color
    fill_color: Option<Color>,
    /// Filled background color
    fill_bg: Option<Color>,
    /// Empty/track color
    empty_color: Color,
    /// Empty background color
    empty_bg: Option<Color>,
    /// Border color
    border_color: Option<Color>,
    /// Warning threshold (0.0-1.0)
    warning_threshold: Option<f64>,
    /// Critical threshold (0.0-1.0)
    critical_threshold: Option<f64>,
    /// Warning color
    warning_color: Color,
    /// Critical color
    critical_color: Color,
    /// Segments count (for segmented style)
    segments: u16,
    /// Title
    title: Option<String>,
    /// Widget properties
    props: WidgetProps,
}

impl Gauge {
    /// Create a new gauge
    pub fn new() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            style: GaugeStyle::Bar,
            width: 20,
            height: 5,
            label: None,
            label_position: LabelPosition::Inside,
            show_percent: true,
            fill_color: None,
            fill_bg: None,
            empty_color: SEPARATOR_COLOR,
            empty_bg: None,
            border_color: None,
            warning_threshold: None,
            critical_threshold: None,
            warning_color: Color::YELLOW,
            critical_color: Color::RED,
            segments: 10,
            title: None,
            props: WidgetProps::new(),
        }
    }

    /// Set value (0.0 - 1.0)
    pub fn value(mut self, value: f64) -> Self {
        self.value = value.clamp(0.0, 1.0);
        self
    }

    /// Set value with custom range
    ///
    /// If min >= max, they will be swapped to ensure valid range.
    /// If min == max after potential swap, value defaults to 0.0.
    pub fn value_range(mut self, value: f64, min: f64, max: f64) -> Self {
        // Ensure min < max
        let (min, max) = if min < max { (min, max) } else { (max, min) };

        self.min = min;
        self.max = max;

        // Avoid division by zero
        let range = max - min;
        if range.abs() < f64::EPSILON {
            self.value = 0.0;
        } else {
            self.value = ((value - min) / range).clamp(0.0, 1.0);
        }
        self
    }

    /// Set percentage (0-100)
    pub fn percent(mut self, percent: f64) -> Self {
        self.value = (percent / 100.0).clamp(0.0, 1.0);
        self
    }

    /// Set style
    pub fn style(mut self, style: GaugeStyle) -> Self {
        self.style = style;
        self
    }

    /// Set width
    pub fn width(mut self, width: u16) -> Self {
        self.width = width.max(4);
        self
    }

    /// Set height
    pub fn height(mut self, height: u16) -> Self {
        self.height = height.max(1);
        self
    }

    /// Set custom label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set label position
    pub fn label_position(mut self, position: LabelPosition) -> Self {
        self.label_position = position;
        self
    }

    /// Show/hide percentage
    pub fn show_percent(mut self, show: bool) -> Self {
        self.show_percent = show;
        self
    }

    /// Set fill color
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set fill background color
    pub fn fill_background(mut self, color: Color) -> Self {
        self.fill_bg = Some(color);
        self
    }

    /// Set empty color
    pub fn empty_color(mut self, color: Color) -> Self {
        self.empty_color = color;
        self
    }

    /// Set empty background color
    pub fn empty_background(mut self, color: Color) -> Self {
        self.empty_bg = Some(color);
        self
    }

    /// Set border color
    pub fn border(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Set thresholds for color changes
    ///
    /// Warning threshold should be less than critical threshold.
    /// If warning >= critical, they will be swapped.
    pub fn thresholds(mut self, warning: f64, critical: f64) -> Self {
        let warning = warning.clamp(0.0, 1.0);
        let critical = critical.clamp(0.0, 1.0);
        // Ensure warning < critical
        let (warning, critical) = if warning < critical {
            (warning, critical)
        } else {
            (critical, warning)
        };
        self.warning_threshold = Some(warning);
        self.critical_threshold = Some(critical);
        self
    }

    /// Set warning color
    pub fn warning_color(mut self, color: Color) -> Self {
        self.warning_color = color;
        self
    }

    /// Set critical color
    pub fn critical_color(mut self, color: Color) -> Self {
        self.critical_color = color;
        self
    }

    /// Set segments count
    pub fn segments(mut self, count: u16) -> Self {
        self.segments = count.max(2);
        self
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Update value
    pub fn set_value(&mut self, value: f64) {
        self.value = value.clamp(0.0, 1.0);
    }

    /// Get current value
    pub fn get_value(&self) -> f64 {
        self.value
    }
}

impl Default for Gauge {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(Gauge);
impl_props_builders!(Gauge);

/// Helper to create a gauge
pub fn gauge() -> Gauge {
    Gauge::new()
}

/// Helper to create a percentage gauge
pub fn percentage(value: f64) -> Gauge {
    Gauge::new().percent(value)
}

/// Helper to create a battery gauge
pub fn battery(level: f64) -> Gauge {
    Gauge::new()
        .percent(level)
        .style(GaugeStyle::Battery)
        .thresholds(0.5, 0.2)
}

// Most tests moved to tests/widget_tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauge_new() {
        let g = Gauge::new();
        assert_eq!(g.value, 0.0);
    }

    #[test]
    fn test_gauge_value() {
        let g = Gauge::new().value(0.75);
        assert_eq!(g.value, 0.75);
    }

    #[test]
    fn test_gauge_value_clamped() {
        let g = Gauge::new().value(1.5);
        assert_eq!(g.value, 1.0);
        let g = Gauge::new().value(-0.5);
        assert_eq!(g.value, 0.0);
    }

    #[test]
    fn test_gauge_percent() {
        let g = Gauge::new().percent(50.0);
        assert!((g.value - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_gauge_styles() {
        let _ = Gauge::new().style(GaugeStyle::Bar);
        let _ = Gauge::new().style(GaugeStyle::Battery);
        let _ = Gauge::new().style(GaugeStyle::Arc);
        let _ = Gauge::new().style(GaugeStyle::Vertical);
    }

    #[test]
    fn test_gauge_helpers() {
        let g = gauge().value(0.7);
        assert_eq!(g.value, 0.7);
        let b = battery(80.0);
        assert!((b.value - 0.8).abs() < 0.01);
    }
}
