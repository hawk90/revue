//! Tooltip public API tests
mod tests {
    use revue::widget::data::chart::{ChartTooltip, ChartTooltipFormat, ChartTooltipPosition};

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

    #[test]
    fn test_tooltip_enabled() {
        let tooltip = ChartTooltip::enabled();
        assert!(tooltip.enabled);
    }

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

    #[test]
    fn test_tooltip_clone() {
        let tooltip1 = ChartTooltip::enabled()
            .custom_format("test")
            .position(ChartTooltipPosition::Fixed);
        let tooltip2 = tooltip1.clone();
        assert_eq!(tooltip1.enabled, tooltip2.enabled);
        assert_eq!(tooltip1.position, tooltip2.position);
    }

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