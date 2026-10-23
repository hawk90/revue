//! Number input widget with increment/decrement controls
//!
//! Provides a numeric input field with:
//! - Up/Down arrow key controls for increment/decrement
//! - Direct numeric entry
//! - Min/max value constraints
//! - Configurable step size and precision
//! - Optional prefix/suffix display (e.g., "$", "%")

mod edit;
mod helper;
mod value;
mod view;

pub use helper::{currency_input, integer_input, number_input, percentage_input};

use crate::utils::EditState;
use crate::widget::traits::{WidgetProps, WidgetState};
use crate::{impl_styled_view, impl_widget_builders};

/// A number input widget with increment/decrement controls
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::{number_input, NumberInput};
///
/// // Basic number input
/// let input = number_input().value(42.0);
///
/// // With constraints
/// let input = number_input()
///     .value(50.0)
///     .min(0.0)
///     .max(100.0)
///     .step(5.0);
///
/// // Currency input
/// let price = number_input()
///     .value(19.99)
///     .prefix("$")
///     .precision(2);
///
/// // Percentage input
/// let percent = number_input()
///     .value(75.0)
///     .suffix("%")
///     .min(0.0)
///     .max(100.0);
/// ```
#[derive(Debug, Clone)]
pub struct NumberInput {
    /// Current numeric value
    value: f64,
    /// Minimum allowed value
    min: Option<f64>,
    /// Maximum allowed value
    max: Option<f64>,
    /// Increment/decrement step
    step: f64,
    /// Decimal precision for display
    precision: u8,
    /// Prefix to display before value (e.g., "$")
    prefix: Option<String>,
    /// Suffix to display after value (e.g., "%")
    suffix: Option<String>,
    /// Whether in direct editing mode
    editing: bool,
    /// The text typed in direct editing mode and its cursor, edited as
    /// every single-line field is (#985)
    edit: EditState,
    /// Width of the input field (characters); `None` fills the area
    width: Option<u16>,
    /// Whether to show +/- buttons
    show_buttons: bool,
    /// Widget state (focused, disabled, colors)
    state: WidgetState,
    /// Widget props (id, classes)
    props: WidgetProps,
}

impl NumberInput {
    /// Create a new number input widget
    pub fn new() -> Self {
        Self {
            value: 0.0,
            min: None,
            max: None,
            step: 1.0,
            precision: 0,
            prefix: None,
            suffix: None,
            editing: false,
            edit: EditState::single_line(),
            width: None,
            show_buttons: true,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Set the current value
    pub fn value(mut self, value: f64) -> Self {
        self.value = self.clamp_value(value);
        self
    }

    /// Set minimum allowed value
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self.value = self.clamp_value(self.value);
        self
    }

    /// Set maximum allowed value
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self.value = self.clamp_value(self.value);
        self
    }

    /// Set the step size for increment/decrement
    pub fn step(mut self, step: f64) -> Self {
        self.step = step.abs();
        self
    }

    /// Set decimal precision (number of decimal places to display)
    pub fn precision(mut self, precision: u8) -> Self {
        self.precision = precision;
        self
    }

    /// Set prefix text (displayed before the value)
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set suffix text (displayed after the value)
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Set the width of the input field
    ///
    /// The field, including its +/- buttons, takes at most `width` columns
    /// (minimum 5). Without it the field fills the area it is given.
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width.max(5));
        self
    }

    /// Show or hide the +/- buttons
    pub fn show_buttons(mut self, show: bool) -> Self {
        self.show_buttons = show;
        self
    }
}

impl Default for NumberInput {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(NumberInput);
impl_widget_builders!(NumberInput);
