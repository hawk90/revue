//! Slider orientation and visual styles

/// Slider orientation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SliderOrientation {
    /// Horizontal slider
    #[default]
    Horizontal,
    /// Vertical slider
    Vertical,
}

/// Slider style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SliderStyle {
    /// Default block style
    #[default]
    Block,
    /// Line style with knob
    Line,
    /// Thin line
    Thin,
    /// Gradient fill
    Gradient,
    /// Dots
    Dots,
}
