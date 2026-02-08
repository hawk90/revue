use crate::style::Color;

/// Axis configuration for charts
#[derive(Clone, Debug)]
pub struct Axis {
    /// Axis title
    pub title: Option<String>,
    /// Minimum value (auto if None)
    pub min: Option<f64>,
    /// Maximum value (auto if None)
    pub max: Option<f64>,
    /// Number of tick marks
    pub ticks: usize,
    /// Show grid lines
    pub grid: bool,
    /// Axis color
    pub color: Color,
    /// Label formatter
    pub format: AxisFormat,
}

/// Axis label format
#[derive(Clone, Debug, Default)]
pub enum AxisFormat {
    /// Auto format based on value range
    #[default]
    Auto,
    /// Integer format (no decimals)
    Integer,
    /// Fixed decimal places
    Fixed(usize),
    /// Percentage format (value * 100%)
    Percent,
    /// Custom format string
    Custom(String),
}

impl Default for Axis {
    fn default() -> Self {
        Self {
            title: None,
            min: None,
            max: None,
            ticks: 5,
            grid: true,
            color: Color::rgb(100, 100, 100),
            format: AxisFormat::Auto,
        }
    }
}

impl Axis {
    /// Create a new axis with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set axis title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set min and max bounds
    pub fn bounds(mut self, min: f64, max: f64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    /// Set minimum bound
    pub fn min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// Set maximum bound
    pub fn max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Set number of tick marks
    pub fn ticks(mut self, ticks: usize) -> Self {
        self.ticks = ticks;
        self
    }

    /// Enable/disable grid lines
    pub fn grid(mut self, show: bool) -> Self {
        self.grid = show;
        self
    }

    /// Set axis color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set label format
    pub fn format(mut self, format: AxisFormat) -> Self {
        self.format = format;
        self
    }

    /// Format a value using the axis format
    pub fn format_value(&self, value: f64) -> String {
        match &self.format {
            AxisFormat::Auto => {
                if value.abs() >= 1000.0 {
                    format!("{:.0}", value)
                } else if value.abs() >= 1.0 {
                    format!("{:.1}", value)
                } else {
                    format!("{:.2}", value)
                }
            }
            AxisFormat::Integer => format!("{:.0}", value),
            AxisFormat::Fixed(n) => format!("{:.1$}", value, *n),
            AxisFormat::Percent => format!("{:.0}%", value * 100.0),
            AxisFormat::Custom(fmt) => fmt.replace("{}", &value.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // AxisFormat tests
    // =========================================================================

    #[test]
    fn test_axis_format_default() {
        let format = AxisFormat::default();
        // Just verify it doesn't panic - Auto format is default
        let _ = format;
    }

    #[test]
    fn test_axis_format_clone() {
        let format1 = AxisFormat::Fixed(2);
        let format2 = format1.clone();
        // Just verify it clones - can't assert equality without PartialEq
        let _ = format2;
    }

    // =========================================================================
    // Axis::new tests
    // =========================================================================

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

    // =========================================================================
    // Axis builder tests
    // =========================================================================

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
    fn test_axis_format() {
        let axis = Axis::new().format(AxisFormat::Integer);
        // Just verify it sets - can't assert equality without PartialEq
        let _ = axis.format;
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

    // =========================================================================
    // format_value tests
    // =========================================================================

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

    // =========================================================================
    // Axis clone tests
    // =========================================================================

    #[test]
    fn test_axis_clone() {
        let axis1 = Axis::new().title("Test").bounds(0.0, 100.0);
        let axis2 = axis1.clone();
        assert_eq!(axis1.title, axis2.title);
        assert_eq!(axis1.min, axis2.min);
        assert_eq!(axis1.max, axis2.max);
    }
}
