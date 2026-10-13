//! One bar (label, value, color) and the orientation of the bars

use crate::style::Color;

/// Bar orientation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BarOrientation {
    /// Horizontal bars (default)
    #[default]
    Horizontal,
    /// Vertical bars
    Vertical,
}

/// A single bar in the chart
#[derive(Clone, Debug)]
pub struct Bar {
    /// Label for the bar
    pub label: String,
    /// Value of the bar
    pub value: f64,
    /// Optional color for this bar
    pub color: Option<Color>,
}

impl Bar {
    /// Create a new bar
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Set bar color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}
