//! Axis public API tests
mod tests {
    use revue::widget::data::chart::{Axis, AxisFormat};
    use revue::style::Color;

    #[test]
    fn test_axis_new() {
        let axis = Axis::new();
        assert!(axis.title.is_none());
        assert!(axis.min.is_none());
        assert!(axis.max.is_none());
        assert_eq!(axis.ticks, 5);
        assert!(axis.grid);
        assert!(matches!(axis.format, AxisFormat::Auto));
    }

    #[test]
    fn test_axis_default() {
        let axis = Axis::default();
        assert_eq!(axis.ticks, 5);
        assert!(axis.grid);
    }

    #[test]
    fn test_axis_title() {
        let axis = Axis::new().title("X Axis");
        assert_eq!(axis.title, Some("X Axis".to_string()));
    }

    #[test]
    fn test_axis_bounds() {
        let axis = Axis::new().bounds(0.0, 100.0);
        assert_eq!(axis.min, Some(0.0));
        assert_eq!(axis.max, Some(100.0));
    }

    #[test]
    fn test_axis_min() {
        let axis = Axis::new().min(10.0);
        assert_eq!(axis.min, Some(10.0));
    }

    #[test]
    fn test_axis_max() {
        let axis = Axis::new().max(100.0);
        assert_eq!(axis.max, Some(100.0));
    }

    #[test]
    fn test_axis_ticks() {
        let axis = Axis::new().ticks(10);
        assert_eq!(axis.ticks, 10);
    }

    #[test]
    fn test_axis_grid() {
        let axis = Axis::new().grid(false);
        assert!(!axis.grid);
    }

    #[test]
    fn test_axis_color() {
        let axis = Axis::new().color(Color::RED);
        assert_eq!(axis.color, Color::RED);
    }

    #[test]
    fn test_axis_builder_chain() {
        let axis = Axis::new()
            .title("Test Axis")
            .bounds(0.0, 100.0)
            .ticks(8)
            .grid(false)
            .color(Color::BLUE)
            .format(AxisFormat::Fixed(2));

        assert_eq!(axis.title, Some("Test Axis".to_string()));
        assert_eq!(axis.min, Some(0.0));
        assert_eq!(axis.max, Some(100.0));
        assert_eq!(axis.ticks, 8);
        assert!(!axis.grid);
        assert_eq!(axis.color, Color::BLUE);
        assert!(matches!(axis.format, AxisFormat::Fixed(2)));
    }

    #[test]
    fn test_format_value_auto_large() {
        let axis = Axis::new().format(AxisFormat::Auto);
        assert_eq!(axis.format_value(1234.56), "1235");
    }

    #[test]
    fn test_format_value_auto_medium() {
        let axis = Axis::new().format(AxisFormat::Auto);
        assert_eq!(axis.format_value(12.34), "12.3");
    }

    #[test]
    fn test_format_value_auto_small() {
        let axis = Axis::new().format(AxisFormat::Auto);
        assert_eq!(axis.format_value(0.123), "0.12");
    }

    #[test]
    fn test_format_value_auto_zero() {
        let axis = Axis::new().format(AxisFormat::Auto);
        assert_eq!(axis.format_value(0.0), "0.00");
    }

    #[test]
    fn test_format_value_auto_negative() {
        let axis = Axis::new().format(AxisFormat::Auto);
        // -100.0 has abs >= 1.0, so formatted with 1 decimal place
        assert_eq!(axis.format_value(-100.0), "-100.0");
    }

    #[test]
    fn test_format_value_integer() {
        let axis = Axis::new().format(AxisFormat::Integer);
        assert_eq!(axis.format_value(12.34), "12");
    }

    #[test]
    fn test_format_value_fixed() {
        let axis = Axis::new().format(AxisFormat::Fixed(2));
        assert_eq!(axis.format_value(12.345), "12.35");
    }

    #[test]
    fn test_format_value_fixed_zero() {
        let axis = Axis::new().format(AxisFormat::Fixed(1));
        assert_eq!(axis.format_value(0.0), "0.0");
    }

    #[test]
    fn test_format_value_percent() {
        let axis = Axis::new().format(AxisFormat::Percent);
        assert_eq!(axis.format_value(0.75), "75%");
    }

    #[test]
    fn test_format_value_percent_zero() {
        let axis = Axis::new().format(AxisFormat::Percent);
        assert_eq!(axis.format_value(0.0), "0%");
    }

    #[test]
    fn test_format_value_custom() {
        let axis = Axis::new().format(AxisFormat::Custom("Value: {}".to_string()));
        assert_eq!(axis.format_value(42.0), "Value: 42");
    }

    #[test]
    fn test_format_value_custom_with_string() {
        let axis = Axis::new().format(AxisFormat::Custom("[{}]".to_string()));
        assert_eq!(axis.format_value(100.0), "[100]");
    }

    #[test]
    fn test_axis_clone() {
        let axis1 = Axis::new().title("Test").bounds(0.0, 100.0);
        let axis2 = axis1.clone();
        assert_eq!(axis1.title, axis2.title);
        assert_eq!(axis1.min, axis2.min);
        assert_eq!(axis1.max, axis2.max);
    }
}