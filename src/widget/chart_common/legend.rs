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
