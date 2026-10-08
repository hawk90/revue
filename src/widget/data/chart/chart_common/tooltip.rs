//! Chart tooltip configuration (deprecated: no chart draws a tooltip)

// The types below are deprecated together and refer to each other.
#![allow(deprecated)]

/// Tooltip configuration
#[deprecated(
    since = "3.5.0",
    note = "never drawn: no chart shows a tooltip; draw the value yourself, e.g. in a `Text` beside the chart"
)]
#[derive(Clone, Debug, Default)]
pub struct ChartTooltip {
    /// Whether tooltip is enabled
    pub enabled: bool,
    /// Tooltip format
    pub format: ChartTooltipFormat,
    /// Tooltip position mode
    pub position: ChartTooltipPosition,
}

impl ChartTooltip {
    /// Create a new tooltip (disabled by default)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an enabled tooltip
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// Enable/disable tooltip
    pub fn enable(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set tooltip format
    pub fn format(mut self, format: ChartTooltipFormat) -> Self {
        self.format = format;
        self
    }

    /// Set custom format string
    pub fn custom_format(mut self, format: impl Into<String>) -> Self {
        self.format = ChartTooltipFormat::Custom(format.into());
        self
    }

    /// Set tooltip position mode
    pub fn position(mut self, position: ChartTooltipPosition) -> Self {
        self.position = position;
        self
    }
}

/// Tooltip format
#[deprecated(
    since = "3.5.0",
    note = "never drawn: no chart shows a tooltip; draw the value yourself, e.g. in a `Text` beside the chart"
)]
#[derive(Clone, Debug, Default)]
pub enum ChartTooltipFormat {
    /// Automatic format based on data
    #[default]
    Auto,
    /// Custom format string (e.g., "{label}: {value}")
    Custom(String),
}

/// Tooltip position mode
#[deprecated(
    since = "3.5.0",
    note = "never drawn: no chart shows a tooltip; draw the value yourself, e.g. in a `Text` beside the chart"
)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartTooltipPosition {
    /// Follow cursor position
    #[default]
    Follow,
    /// Fixed position near data point
    Fixed,
}
