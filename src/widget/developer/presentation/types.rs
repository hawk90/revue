//! Slide transition and alignment settings

/// Slide transition effect
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Transition {
    /// No transition
    #[default]
    None,
    /// Fade in/out
    Fade,
    /// Slide from left
    SlideLeft,
    /// Slide from right
    SlideRight,
    /// Slide from bottom
    SlideUp,
    /// Zoom in
    ZoomIn,
}

/// Text alignment for slides
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SlideAlign {
    /// Left aligned
    Left,
    /// Center aligned
    #[default]
    Center,
    /// Right aligned
    Right,
}
