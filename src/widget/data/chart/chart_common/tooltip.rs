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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ChartTooltip::new tests
    // =========================================================================

    #[test]
    fn test_tooltip_new() {
        let tooltip = ChartTooltip::new();
        assert!(!tooltip.enabled);
        assert!(matches!(tooltip.format, ChartTooltipFormat::Auto));
        assert_eq!(tooltip.position, ChartTooltipPosition::Follow);
    }

    #[test]
    fn test_tooltip_default() {
        let tooltip = ChartTooltip::default();
        assert!(!tooltip.enabled);
    }

    // =========================================================================
    // ChartTooltip constructors
    // =========================================================================

    #[test]
    fn test_tooltip_enabled() {
        let tooltip = ChartTooltip::enabled();
        assert!(tooltip.enabled);
    }

    // =========================================================================
    // ChartTooltip builder methods
    // =========================================================================

    #[test]
    fn test_tooltip_enable_true() {
        let tooltip = ChartTooltip::new().enable(true);
        assert!(tooltip.enabled);
    }

    #[test]
    fn test_tooltip_enable_false() {
        let tooltip = ChartTooltip::enabled().enable(false);
        assert!(!tooltip.enabled);
    }

    #[test]
    fn test_tooltip_format() {
        let tooltip =
            ChartTooltip::new().format(ChartTooltipFormat::Custom("{x}: {y}".to_string()));
        assert!(matches!(tooltip.format, ChartTooltipFormat::Custom(_)));
    }

    #[test]
    fn test_tooltip_custom_format() {
        let tooltip = ChartTooltip::new().custom_format("Value: {}");
        assert!(matches!(tooltip.format, ChartTooltipFormat::Custom(_)));
    }

    #[test]
    fn test_tooltip_custom_format_string() {
        let tooltip = ChartTooltip::new().custom_format(String::from("Label: {l}"));
        assert!(matches!(tooltip.format, ChartTooltipFormat::Custom(_)));
    }

    #[test]
    fn test_tooltip_position() {
        let tooltip = ChartTooltip::new().position(ChartTooltipPosition::Fixed);
        assert_eq!(tooltip.position, ChartTooltipPosition::Fixed);
    }

    #[test]
    fn test_tooltip_builder_chain() {
        let tooltip = ChartTooltip::new()
            .enable(true)
            .custom_format("({x}, {y})")
            .position(ChartTooltipPosition::Fixed);

        assert!(tooltip.enabled);
        assert!(matches!(tooltip.format, ChartTooltipFormat::Custom(_)));
        assert_eq!(tooltip.position, ChartTooltipPosition::Fixed);
    }

    // =========================================================================
    // ChartTooltipPosition enum tests
    // =========================================================================

    #[test]
    fn test_tooltip_position_default() {
        assert_eq!(
            ChartTooltipPosition::default(),
            ChartTooltipPosition::Follow
        );
    }

    #[test]
    fn test_tooltip_position_clone() {
        let pos1 = ChartTooltipPosition::Fixed;
        let pos2 = pos1.clone();
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_tooltip_position_copy() {
        let pos1 = ChartTooltipPosition::Follow;
        let pos2 = pos1;
        assert_eq!(pos2, ChartTooltipPosition::Follow);
    }

    #[test]
    fn test_tooltip_position_partial_eq() {
        assert_eq!(ChartTooltipPosition::Follow, ChartTooltipPosition::Follow);
        assert_eq!(ChartTooltipPosition::Fixed, ChartTooltipPosition::Fixed);
        assert_ne!(ChartTooltipPosition::Follow, ChartTooltipPosition::Fixed);
    }

    // =========================================================================
    // ChartTooltip clone tests
    // =========================================================================

    #[test]
    fn test_tooltip_clone() {
        let tooltip1 = ChartTooltip::enabled()
            .custom_format("test")
            .position(ChartTooltipPosition::Fixed);
        let tooltip2 = tooltip1.clone();
        assert_eq!(tooltip1.enabled, tooltip2.enabled);
        assert_eq!(tooltip1.position, tooltip2.position);
    }

    // =========================================================================
    // ChartTooltipFormat tests
    // =========================================================================

    #[test]
    fn test_tooltip_format_default() {
        let format = ChartTooltipFormat::default();
        assert!(matches!(format, ChartTooltipFormat::Auto));
    }

    #[test]
    fn test_tooltip_format_clone() {
        let format1 = ChartTooltipFormat::Custom("test".to_string());
        let format2 = format1.clone();
        assert!(matches!(format1, ChartTooltipFormat::Custom(_)));
        assert!(matches!(format2, ChartTooltipFormat::Custom(_)));
    }
}
