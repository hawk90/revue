use crate::style::Color;

/// Grid configuration
#[derive(Clone, Debug, Default)]
pub struct ChartGrid {
    /// Show vertical grid lines (X axis)
    pub x: bool,
    /// Show horizontal grid lines (Y axis)
    pub y: bool,
    /// Grid line color
    pub color: Option<Color>,
    /// Grid line style
    pub style: GridStyle,
}

impl ChartGrid {
    /// Create a new grid (hidden by default)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a grid with both X and Y lines
    pub fn both() -> Self {
        Self {
            x: true,
            y: true,
            ..Default::default()
        }
    }

    /// Create a grid with only X lines
    pub fn x_only() -> Self {
        Self {
            x: true,
            y: false,
            ..Default::default()
        }
    }

    /// Create a grid with only Y lines
    pub fn y_only() -> Self {
        Self {
            x: false,
            y: true,
            ..Default::default()
        }
    }

    /// Enable/disable X grid lines
    pub fn x(mut self, show: bool) -> Self {
        self.x = show;
        self
    }

    /// Enable/disable Y grid lines
    pub fn y(mut self, show: bool) -> Self {
        self.y = show;
        self
    }

    /// Set grid line color
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set grid line style
    pub fn style(mut self, style: GridStyle) -> Self {
        self.style = style;
        self
    }

    /// Get the grid character for rendering
    pub fn char(&self) -> char {
        match self.style {
            GridStyle::Solid => '─',
            GridStyle::Dashed => '╌',
            GridStyle::Dotted => '·',
        }
    }

    /// Get the effective color (default if not set)
    pub fn effective_color(&self) -> Color {
        self.color.unwrap_or(Color::rgb(60, 60, 60))
    }
}

/// Grid line style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GridStyle {
    /// Solid lines
    #[default]
    Solid,
    /// Dashed lines
    Dashed,
    /// Dotted lines
    Dotted,
}
