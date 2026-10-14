//! Large digit display widget
//!
//! Displays numbers using large 7-segment style characters, perfect for
//! dashboards, clocks, timers, and counters.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Digits, DigitStyle};
//!
//! // Simple counter display
//! let counter = Digits::new(42);
//!
//! // Clock display with colons
//! let clock = Digits::new("12:34:56")
//!     .style(DigitStyle::Block)
//!     .fg(Color::CYAN);
//!
//! // Price display
//! let price = digits(1234.56)
//!     .prefix("$")
//!     .separator(',');
//! ```

mod builders;
mod format;
mod patterns;
mod view;

use crate::style::Color;
use crate::widget::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Digit display style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DigitStyle {
    /// Block characters (███)
    #[default]
    Block,
    /// Thin box drawing characters (┌─┐)
    Thin,
    /// ASCII characters (+-)
    Ascii,
    /// Braille characters (more compact)
    Braille,
}

/// Large digit display widget
#[derive(Clone, Debug)]
pub struct Digits {
    /// The value to display
    value: String,
    /// Display style
    style: DigitStyle,
    /// Foreground color
    fg: Option<Color>,
    /// Background color
    bg: Option<Color>,
    /// Prefix (e.g., "$", "€")
    prefix: Option<String>,
    /// Suffix (e.g., "%", "kg")
    suffix: Option<String>,
    /// Thousands separator
    separator: Option<char>,
    /// Minimum width (pad with zeros)
    min_width: Option<usize>,
    /// Show leading zeros when padding to `min_width`.
    ///
    /// `None` keeps the default zero padding; `Some(false)` pads with blanks.
    leading_zeros: Option<bool>,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Digits {
    /// Create new digits display from a number
    pub fn new(value: impl ToString) -> Self {
        Self {
            value: value.to_string(),
            style: DigitStyle::default(),
            fg: None,
            bg: None,
            prefix: None,
            suffix: None,
            separator: None,
            min_width: None,
            leading_zeros: None,
            props: WidgetProps::new(),
        }
    }

    /// Create from integer
    pub fn from_int(value: i64) -> Self {
        Self::new(value)
    }

    /// Create from float with decimal places
    pub fn from_float(value: f64, decimals: usize) -> Self {
        Self::new(format!("{:.prec$}", value, prec = decimals))
    }

    /// Create time display (HH:MM:SS)
    pub fn time(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self::new(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
    }

    /// Create clock display (HH:MM)
    pub fn clock(hours: u32, minutes: u32) -> Self {
        Self::new(format!("{:02}:{:02}", hours, minutes))
    }

    /// Create timer display from seconds
    pub fn timer(total_seconds: u64) -> Self {
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            Self::time(hours as u32, minutes as u32, seconds as u32)
        } else {
            Self::new(format!("{:02}:{:02}", minutes, seconds))
        }
    }
}

impl_styled_view!(Digits);
impl_props_builders!(Digits);

/// Create a digits display
pub fn digits(value: impl ToString) -> Digits {
    Digits::new(value)
}

/// Create a clock display
pub fn clock(hours: u32, minutes: u32) -> Digits {
    Digits::clock(hours, minutes)
}

/// Create a timer display
pub fn timer(seconds: u64) -> Digits {
    Digits::timer(seconds)
}
