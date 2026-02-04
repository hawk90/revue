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
