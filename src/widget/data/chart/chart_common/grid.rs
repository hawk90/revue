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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ChartGrid::new tests
    // =========================================================================

    #[test]
    fn test_grid_new() {
        let grid = ChartGrid::new();
        assert!(!grid.x);
        assert!(!grid.y);
        assert!(grid.color.is_none());
        assert_eq!(grid.style, GridStyle::Solid);
    }

    #[test]
    fn test_grid_default() {
        let grid = ChartGrid::default();
        assert!(!grid.x);
        assert!(!grid.y);
        assert_eq!(grid.style, GridStyle::Solid);
    }

    // =========================================================================
    // ChartGrid constructors
    // =========================================================================

    #[test]
    fn test_grid_both() {
        let grid = ChartGrid::both();
        assert!(grid.x);
        assert!(grid.y);
    }

    #[test]
    fn test_grid_x_only() {
        let grid = ChartGrid::x_only();
        assert!(grid.x);
        assert!(!grid.y);
    }

    #[test]
    fn test_grid_y_only() {
        let grid = ChartGrid::y_only();
        assert!(!grid.x);
        assert!(grid.y);
    }

    // =========================================================================
    // ChartGrid builder methods
    // =========================================================================

    #[test]
    fn test_grid_x_true() {
        let grid = ChartGrid::new().x(true);
        assert!(grid.x);
    }

    #[test]
    fn test_grid_x_false() {
        let grid = ChartGrid::both().x(false);
        assert!(!grid.x);
        assert!(grid.y); // Y should still be true
    }

    #[test]
    fn test_grid_y_true() {
        let grid = ChartGrid::new().y(true);
        assert!(grid.y);
    }

    #[test]
    fn test_grid_y_false() {
        let grid = ChartGrid::both().y(false);
        assert!(!grid.y);
        assert!(grid.x); // X should still be true
    }

    #[test]
    fn test_grid_color() {
        let grid = ChartGrid::new().color(Color::RED);
        assert_eq!(grid.color, Some(Color::RED));
    }

    #[test]
    fn test_grid_style() {
        let grid = ChartGrid::new().style(GridStyle::Dashed);
        assert_eq!(grid.style, GridStyle::Dashed);
    }

    #[test]
    fn test_grid_builder_chain() {
        let grid = ChartGrid::new()
            .x(true)
            .y(true)
            .color(Color::BLUE)
            .style(GridStyle::Dotted);

        assert!(grid.x);
        assert!(grid.y);
        assert_eq!(grid.color, Some(Color::BLUE));
        assert_eq!(grid.style, GridStyle::Dotted);
    }

    // =========================================================================
    // char method tests
    // =========================================================================

    #[test]
    fn test_grid_char_solid() {
        let grid = ChartGrid::new().style(GridStyle::Solid);
        assert_eq!(grid.char(), '─');
    }

    #[test]
    fn test_grid_char_dashed() {
        let grid = ChartGrid::new().style(GridStyle::Dashed);
        assert_eq!(grid.char(), '╌');
    }

    #[test]
    fn test_grid_char_dotted() {
        let grid = ChartGrid::new().style(GridStyle::Dotted);
        assert_eq!(grid.char(), '·');
    }

    // =========================================================================
    // effective_color tests
    // =========================================================================

    #[test]
    fn test_effective_color_when_set() {
        let grid = ChartGrid::new().color(Color::GREEN);
        assert_eq!(grid.effective_color(), Color::GREEN);
    }

    #[test]
    fn test_effective_color_when_none() {
        let grid = ChartGrid::new();
        assert_eq!(grid.effective_color(), Color::rgb(60, 60, 60));
    }

    // =========================================================================
    // GridStyle enum tests
    // =========================================================================

    #[test]
    fn test_grid_style_default() {
        assert_eq!(GridStyle::default(), GridStyle::Solid);
    }

    #[test]
    fn test_grid_style_clone() {
        let style1 = GridStyle::Dashed;
        let style2 = style1.clone();
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_grid_style_copy() {
        let style1 = GridStyle::Dotted;
        let style2 = style1;
        assert_eq!(style2, GridStyle::Dotted);
    }

    #[test]
    fn test_grid_style_partial_eq() {
        assert_eq!(GridStyle::Solid, GridStyle::Solid);
        assert_eq!(GridStyle::Dashed, GridStyle::Dashed);
        assert_ne!(GridStyle::Solid, GridStyle::Dotted);
    }

    // =========================================================================
    // ChartGrid clone tests
    // =========================================================================

    #[test]
    fn test_grid_clone() {
        let grid1 = ChartGrid::both()
            .color(Color::YELLOW)
            .style(GridStyle::Dashed);
        let grid2 = grid1.clone();
        assert_eq!(grid1.x, grid2.x);
        assert_eq!(grid1.y, grid2.y);
        assert_eq!(grid1.color, grid2.color);
        assert_eq!(grid1.style, grid2.style);
    }
}
