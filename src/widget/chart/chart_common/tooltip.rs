/// Tooltip configuration
#[derive(Clone, Debug, Default)]
#[allow(dead_code)] // Part of public API for chart widgets
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
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an enabled tooltip
    #[allow(dead_code)]
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// Enable/disable tooltip
    #[allow(dead_code)]
    pub fn enable(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set tooltip format
    #[allow(dead_code)]
    pub fn format(mut self, format: ChartTooltipFormat) -> Self {
        self.format = format;
        self
    }

    /// Set custom format string
    #[allow(dead_code)]
    pub fn custom_format(mut self, format: impl Into<String>) -> Self {
        self.format = ChartTooltipFormat::Custom(format.into());
        self
    }

    /// Set tooltip position mode
    #[allow(dead_code)]
    pub fn position(mut self, position: ChartTooltipPosition) -> Self {
        self.position = position;
        self
    }
}

/// Tooltip format
#[derive(Clone, Debug, Default)]
#[allow(dead_code)] // Part of public API for chart widgets
pub enum ChartTooltipFormat {
    /// Automatic format based on data
    #[default]
    Auto,
    /// Custom format string (e.g., "{label}: {value}")
    Custom(String),
}

/// Tooltip position mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)] // Part of public API for chart widgets
pub enum ChartTooltipPosition {
    /// Follow cursor position
    #[default]
    Follow,
    /// Fixed position near data point
    Fixed,
}
