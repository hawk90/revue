//! Split orientation, divider styles and panes

/// Split orientation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitOrientation {
    /// Split horizontally (panes side by side)
    #[default]
    Horizontal,
    /// Split vertically (panes stacked)
    Vertical,
}

/// Splitter style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitterStyle {
    /// Single line
    #[default]
    Line,
    /// Double line
    Double,
    /// Thick line
    Thick,
    /// No visible splitter
    Hidden,
}

/// A pane in the splitter
#[derive(Clone)]
pub struct Pane {
    /// Pane identifier
    pub id: String,
    /// Minimum size (percentage or absolute)
    pub min_size: u16,
    /// Maximum size (0 = unlimited)
    pub max_size: u16,
    /// Initial size ratio (0.0 - 1.0)
    pub ratio: f32,
    /// Whether pane is collapsible
    pub collapsible: bool,
    /// Whether pane is collapsed
    pub collapsed: bool,
}

impl Pane {
    /// Create a new pane
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            min_size: 5,
            max_size: 0,
            ratio: 0.5,
            collapsible: false,
            collapsed: false,
        }
    }

    /// Set minimum size
    pub fn min_size(mut self, size: u16) -> Self {
        self.min_size = size;
        self
    }

    /// Set maximum size
    pub fn max_size(mut self, size: u16) -> Self {
        self.max_size = size;
        self
    }

    /// Set initial ratio
    pub fn ratio(mut self, ratio: f32) -> Self {
        self.ratio = ratio.clamp(0.0, 1.0);
        self
    }

    /// Make collapsible
    pub fn collapsible(mut self) -> Self {
        self.collapsible = true;
        self
    }

    /// Toggle collapsed state
    pub fn toggle_collapse(&mut self) {
        if self.collapsible {
            self.collapsed = !self.collapsed;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pane_new() {
        let p = Pane::new("left");
        assert_eq!(p.id, "left");
        assert_eq!(p.ratio, 0.5);
        assert_eq!(p.min_size, 5);
        assert!(!p.collapsible);
        assert!(!p.collapsed);
    }

    #[test]
    fn test_pane_builder() {
        let p = Pane::new("main")
            .ratio(0.7)
            .min_size(10)
            .max_size(50)
            .collapsible();
        assert_eq!(p.ratio, 0.7);
        assert_eq!(p.min_size, 10);
        assert_eq!(p.max_size, 50);
        assert!(p.collapsible);
    }

    #[test]
    fn test_pane_ratio_clamped() {
        let p = Pane::new("x").ratio(1.5);
        assert_eq!(p.ratio, 1.0);

        let p = Pane::new("x").ratio(-0.5);
        assert_eq!(p.ratio, 0.0);
    }

    #[test]
    fn test_pane_toggle_collapse() {
        let mut p = Pane::new("x").collapsible();
        assert!(!p.collapsed);
        p.toggle_collapse();
        assert!(p.collapsed);
        p.toggle_collapse();
        assert!(!p.collapsed);
    }

    #[test]
    fn test_pane_toggle_collapse_not_collapsible() {
        let mut p = Pane::new("x");
        p.toggle_collapse(); // Should be no-op
        assert!(!p.collapsed);
    }
}
