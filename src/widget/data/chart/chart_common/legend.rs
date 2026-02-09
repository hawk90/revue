/// Legend configuration
#[derive(Clone, Debug, Default)]
pub struct Legend {
    /// Position of the legend
    pub position: LegendPosition,
    /// Orientation of legend items
    pub orientation: LegendOrientation,
    /// Whether legend items are interactive (click to toggle)
    pub interactive: bool,
}

impl Legend {
    /// Create a new legend with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set legend position
    pub fn position(mut self, position: LegendPosition) -> Self {
        self.position = position;
        self
    }

    /// Set legend orientation
    pub fn orientation(mut self, orientation: LegendOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Enable interactive mode
    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    /// Create legend at top left
    pub fn top_left() -> Self {
        Self::new().position(LegendPosition::TopLeft)
    }

    /// Create legend at top center
    pub fn top_center() -> Self {
        Self::new().position(LegendPosition::TopCenter)
    }

    /// Create legend at top right
    pub fn top_right() -> Self {
        Self::new().position(LegendPosition::TopRight)
    }

    /// Create legend at bottom left
    pub fn bottom_left() -> Self {
        Self::new().position(LegendPosition::BottomLeft)
    }

    /// Create legend at bottom center
    pub fn bottom_center() -> Self {
        Self::new().position(LegendPosition::BottomCenter)
    }

    /// Create legend at bottom right
    pub fn bottom_right() -> Self {
        Self::new().position(LegendPosition::BottomRight)
    }

    /// Create legend on the left side
    pub fn left() -> Self {
        Self::new()
            .position(LegendPosition::Left)
            .orientation(LegendOrientation::Vertical)
    }

    /// Create legend on the right side
    pub fn right() -> Self {
        Self::new()
            .position(LegendPosition::Right)
            .orientation(LegendOrientation::Vertical)
    }

    /// Create a hidden legend
    pub fn none() -> Self {
        Self::new().position(LegendPosition::None)
    }

    /// Create a hidden legend (alias)
    pub fn hidden() -> Self {
        Self::none()
    }

    /// Check if legend is visible
    pub fn is_visible(&self) -> bool {
        self.position != LegendPosition::None
    }
}

/// Legend position
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LegendPosition {
    /// Top left corner
    TopLeft,
    /// Top center
    TopCenter,
    /// Top right corner
    #[default]
    TopRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom center
    BottomCenter,
    /// Bottom right corner
    BottomRight,
    /// Left side (vertical)
    Left,
    /// Right side (vertical)
    Right,
    /// Hidden
    None,
}

/// Legend item orientation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LegendOrientation {
    /// Items arranged horizontally
    #[default]
    Horizontal,
    /// Items arranged vertically
    Vertical,
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Legend::new tests
    // =========================================================================

    #[test]
    fn test_legend_new() {
        let legend = Legend::new();
        assert_eq!(legend.position, LegendPosition::TopRight);
        assert_eq!(legend.orientation, LegendOrientation::Horizontal);
        assert!(!legend.interactive);
    }

    #[test]
    fn test_legend_default() {
        let legend = Legend::default();
        assert_eq!(legend.position, LegendPosition::TopRight);
        assert_eq!(legend.orientation, LegendOrientation::Horizontal);
    }

    // =========================================================================
    // Legend builder methods
    // =========================================================================

    #[test]
    fn test_legend_position() {
        let legend = Legend::new().position(LegendPosition::BottomLeft);
        assert_eq!(legend.position, LegendPosition::BottomLeft);
    }

    #[test]
    fn test_legend_orientation() {
        let legend = Legend::new().orientation(LegendOrientation::Vertical);
        assert_eq!(legend.orientation, LegendOrientation::Vertical);
    }

    #[test]
    fn test_legend_interactive() {
        let legend = Legend::new().interactive(true);
        assert!(legend.interactive);
    }

    #[test]
    fn test_legend_builder_chain() {
        let legend = Legend::new()
            .position(LegendPosition::TopCenter)
            .orientation(LegendOrientation::Vertical)
            .interactive(true);

        assert_eq!(legend.position, LegendPosition::TopCenter);
        assert_eq!(legend.orientation, LegendOrientation::Vertical);
        assert!(legend.interactive);
    }

    // =========================================================================
    // Legend position constructors
    // =========================================================================

    #[test]
    fn test_legend_top_left() {
        let legend = Legend::top_left();
        assert_eq!(legend.position, LegendPosition::TopLeft);
    }

    #[test]
    fn test_legend_top_center() {
        let legend = Legend::top_center();
        assert_eq!(legend.position, LegendPosition::TopCenter);
    }

    #[test]
    fn test_legend_top_right() {
        let legend = Legend::top_right();
        assert_eq!(legend.position, LegendPosition::TopRight);
    }

    #[test]
    fn test_legend_bottom_left() {
        let legend = Legend::bottom_left();
        assert_eq!(legend.position, LegendPosition::BottomLeft);
    }

    #[test]
    fn test_legend_bottom_center() {
        let legend = Legend::bottom_center();
        assert_eq!(legend.position, LegendPosition::BottomCenter);
    }

    #[test]
    fn test_legend_bottom_right() {
        let legend = Legend::bottom_right();
        assert_eq!(legend.position, LegendPosition::BottomRight);
    }

    #[test]
    fn test_legend_left() {
        let legend = Legend::left();
        assert_eq!(legend.position, LegendPosition::Left);
        assert_eq!(legend.orientation, LegendOrientation::Vertical);
    }

    #[test]
    fn test_legend_right() {
        let legend = Legend::right();
        assert_eq!(legend.position, LegendPosition::Right);
        assert_eq!(legend.orientation, LegendOrientation::Vertical);
    }

    #[test]
    fn test_legend_none() {
        let legend = Legend::none();
        assert_eq!(legend.position, LegendPosition::None);
    }

    #[test]
    fn test_legend_hidden() {
        let legend = Legend::hidden();
        assert_eq!(legend.position, LegendPosition::None);
    }

    // =========================================================================
    // is_visible tests
    // =========================================================================

    #[test]
    fn test_is_visible_when_visible() {
        let legend = Legend::top_left();
        assert!(legend.is_visible());
    }

    #[test]
    fn test_is_visible_when_none() {
        let legend = Legend::none();
        assert!(!legend.is_visible());
    }

    #[test]
    fn test_is_visible_all_positions() {
        assert!(Legend::top_left().is_visible());
        assert!(Legend::top_center().is_visible());
        assert!(Legend::top_right().is_visible());
        assert!(Legend::bottom_left().is_visible());
        assert!(Legend::bottom_center().is_visible());
        assert!(Legend::bottom_right().is_visible());
        assert!(Legend::left().is_visible());
        assert!(Legend::right().is_visible());
        assert!(!Legend::none().is_visible());
    }

    // =========================================================================
    // LegendPosition enum tests
    // =========================================================================

    #[test]
    fn test_legend_position_default() {
        assert_eq!(LegendPosition::default(), LegendPosition::TopRight);
    }

    #[test]
    fn test_legend_position_clone() {
        let pos1 = LegendPosition::BottomCenter;
        let pos2 = pos1.clone();
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_legend_position_copy() {
        let pos1 = LegendPosition::Left;
        let pos2 = pos1;
        assert_eq!(pos2, LegendPosition::Left);
    }

    #[test]
    fn test_legend_position_partial_eq() {
        assert_eq!(LegendPosition::TopLeft, LegendPosition::TopLeft);
        assert_eq!(LegendPosition::Right, LegendPosition::Right);
        assert_ne!(LegendPosition::TopLeft, LegendPosition::BottomLeft);
    }

    #[test]
    fn test_legend_position_all_unique() {
        assert_ne!(LegendPosition::TopLeft, LegendPosition::TopCenter);
        assert_ne!(LegendPosition::TopCenter, LegendPosition::TopRight);
        assert_ne!(LegendPosition::TopLeft, LegendPosition::TopRight);
        assert_ne!(LegendPosition::BottomLeft, LegendPosition::None);
        assert_ne!(LegendPosition::Left, LegendPosition::Right);
    }

    // =========================================================================
    // LegendOrientation enum tests
    // =========================================================================

    #[test]
    fn test_legend_orientation_default() {
        assert_eq!(LegendOrientation::default(), LegendOrientation::Horizontal);
    }

    #[test]
    fn test_legend_orientation_clone() {
        let ori1 = LegendOrientation::Vertical;
        let ori2 = ori1.clone();
        assert_eq!(ori1, ori2);
    }

    #[test]
    fn test_legend_orientation_copy() {
        let ori1 = LegendOrientation::Horizontal;
        let ori2 = ori1;
        assert_eq!(ori2, LegendOrientation::Horizontal);
    }

    #[test]
    fn test_legend_orientation_partial_eq() {
        assert_eq!(LegendOrientation::Horizontal, LegendOrientation::Horizontal);
        assert_eq!(LegendOrientation::Vertical, LegendOrientation::Vertical);
        assert_ne!(LegendOrientation::Horizontal, LegendOrientation::Vertical);
    }

    // =========================================================================
    // Legend clone tests
    // =========================================================================

    #[test]
    fn test_legend_clone() {
        let legend1 = Legend::new()
            .position(LegendPosition::BottomRight)
            .orientation(LegendOrientation::Vertical)
            .interactive(true);
        let legend2 = legend1.clone();
        assert_eq!(legend1.position, legend2.position);
        assert_eq!(legend1.orientation, legend2.orientation);
        assert_eq!(legend1.interactive, legend2.interactive);
    }
}
