//! Pie chart style, label style and slice types

use crate::style::Color;

/// Pie chart style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PieStyle {
    /// Standard pie chart
    #[default]
    Pie,
    /// Donut chart with hollow center
    Donut,
}

/// Label display style for pie slices
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PieLabelStyle {
    /// No labels
    #[default]
    None,
    /// Show value
    Value,
    /// Show percentage
    Percent,
    /// Show slice label
    Label,
    /// Show label and percentage
    LabelPercent,
}

/// A single slice in the pie chart
#[derive(Clone, Debug)]
pub struct PieSlice {
    /// Slice label
    pub label: String,
    /// Slice value
    pub value: f64,
    /// Custom color (uses palette if None)
    pub color: Option<Color>,
}

impl PieSlice {
    /// Create a new slice
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            color: None,
        }
    }

    /// Create a slice with custom color
    pub fn with_color(label: impl Into<String>, value: f64, color: Color) -> Self {
        Self {
            label: label.into(),
            value,
            color: Some(color),
        }
    }
}
