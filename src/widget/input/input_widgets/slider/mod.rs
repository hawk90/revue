//! Slider widget for selecting numeric values
//!
//! Provides horizontal and vertical sliders with customizable
//! ranges, steps, and visual styles.

mod render;
mod types;

pub use types::{SliderOrientation, SliderStyle};

use crate::style::Color;
use crate::widget::theme::SEPARATOR_COLOR;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Slider widget
#[derive(Clone)]
pub struct Slider {
    /// Current value
    value: f64,
    /// Minimum value
    min: f64,
    /// Maximum value
    max: f64,
    /// Step size (0 = continuous)
    step: f64,
    /// Orientation
    orientation: SliderOrientation,
    /// Visual style
    style: SliderStyle,
    /// Length (width for horizontal, height for vertical)
    length: u16,
    /// Show value label
    show_value: bool,
    /// Value format string
    value_format: Option<String>,
    /// Track color
    track_color: Color,
    /// Fill color
    /// The color the builder named, if it named one - see #656.
    fill_color: Option<Color>,
    /// Knob color
    knob_color: Color,
    /// Focused state
    focused: bool,
    /// Disabled state
    disabled: bool,
    /// Label
    label: Option<String>,
    /// Show tick marks
    show_ticks: bool,
    /// Number of ticks
    tick_count: u16,
    /// Widget properties
    props: WidgetProps,
}

impl Slider {
    /// Create a new slider
    pub fn new() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 0.0,
            orientation: SliderOrientation::Horizontal,
            style: SliderStyle::Block,
            length: 20,
            show_value: true,
            value_format: None,
            track_color: SEPARATOR_COLOR,
            fill_color: None,
            knob_color: Color::WHITE,
            focused: false,
            disabled: false,
            label: None,
            show_ticks: false,
            tick_count: 5,
            props: WidgetProps::new(),
        }
    }

    /// Set value
    pub fn value(mut self, value: f64) -> Self {
        self.value = self.clamp_value(value);
        self
    }

    /// Set range
    ///
    /// The bounds may be given in either order: a range given high to low
    /// (`min > max`) is swapped. A NaN bound is ignored and the current
    /// bound kept.
    pub fn range(mut self, min: f64, max: f64) -> Self {
        let min = if min.is_nan() { self.min } else { min };
        let max = if max.is_nan() { self.max } else { max };
        self.min = min.min(max);
        self.max = min.max(max);
        self.value = self.clamp_value(self.value);
        self
    }

    /// Set step size
    pub fn step(mut self, step: f64) -> Self {
        self.step = step.abs();
        self
    }

    /// Set orientation
    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Make horizontal
    pub fn horizontal(mut self) -> Self {
        self.orientation = SliderOrientation::Horizontal;
        self
    }

    /// Make vertical
    pub fn vertical(mut self) -> Self {
        self.orientation = SliderOrientation::Vertical;
        self
    }

    /// Set style
    pub fn style(mut self, style: SliderStyle) -> Self {
        self.style = style;
        self
    }

    /// Set length
    pub fn length(mut self, length: u16) -> Self {
        self.length = length.max(3);
        self
    }

    /// Show/hide value
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Set value format
    pub fn value_format(mut self, format: impl Into<String>) -> Self {
        self.value_format = Some(format.into());
        self
    }

    /// Set track color
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = color;
        self
    }

    /// Set fill color
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set knob color
    pub fn knob_color(mut self, color: Color) -> Self {
        self.knob_color = color;
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Show tick marks
    pub fn ticks(mut self, count: u16) -> Self {
        self.show_ticks = true;
        self.tick_count = count.max(2);
        self
    }

    /// Clamp value to range and step
    fn clamp_value(&self, value: f64) -> f64 {
        // NaN is no position on the track (and clamps to NaN): keep the
        // current value
        if value.is_nan() {
            return self.value;
        }
        let clamped = value.clamp(self.min, self.max);
        if self.step > 0.0 {
            let steps = ((clamped - self.min) / self.step).round();
            (self.min + steps * self.step).clamp(self.min, self.max)
        } else {
            clamped
        }
    }

    /// Get normalized value (0.0 - 1.0)
    fn normalized(&self) -> f64 {
        if (self.max - self.min).abs() < f64::EPSILON {
            0.0
        } else {
            (self.value - self.min) / (self.max - self.min)
        }
    }

    /// Set value
    pub fn set_value(&mut self, value: f64) {
        self.value = self.clamp_value(value);
    }

    /// Get current value
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// Increment value
    pub fn increment(&mut self) {
        let step = if self.step > 0.0 {
            self.step
        } else {
            (self.max - self.min) / 100.0
        };
        self.value = self.clamp_value(self.value + step);
    }

    /// Decrement value
    pub fn decrement(&mut self) {
        let step = if self.step > 0.0 {
            self.step
        } else {
            (self.max - self.min) / 100.0
        };
        self.value = self.clamp_value(self.value - step);
    }

    /// Set to minimum
    pub fn set_min(&mut self) {
        self.value = self.min;
    }

    /// Set to maximum
    pub fn set_max(&mut self) {
        self.value = self.max;
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        if self.disabled || !self.focused {
            return false;
        }

        match (&self.orientation, key) {
            (SliderOrientation::Horizontal, Key::Right | Key::Char('l'))
            | (SliderOrientation::Vertical, Key::Up | Key::Char('k')) => {
                self.increment();
                true
            }
            (SliderOrientation::Horizontal, Key::Left | Key::Char('h'))
            | (SliderOrientation::Vertical, Key::Down | Key::Char('j')) => {
                self.decrement();
                true
            }
            (_, Key::Home) => {
                self.set_min();
                true
            }
            (_, Key::End) => {
                self.set_max();
                true
            }
            _ => false,
        }
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(Slider);
impl_props_builders!(Slider);

/// Helper to create a slider
pub fn slider() -> Slider {
    Slider::new()
}

/// Helper to create a slider with range
pub fn slider_range(min: f64, max: f64) -> Slider {
    Slider::new().range(min, max)
}

/// Helper to create a percentage slider
pub fn percentage_slider() -> Slider {
    Slider::new().range(0.0, 100.0).value_format("{}%")
}

/// Helper to create a volume slider
pub fn volume_slider() -> Slider {
    Slider::new()
        .range(0.0, 100.0)
        .label("Vol")
        .style(SliderStyle::Block)
}
