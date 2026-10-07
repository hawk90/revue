//! Waveline display style and interpolation method

/// Display style for the waveline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WaveStyle {
    /// Simple line
    #[default]
    Line,
    /// Filled area under the line
    Filled,
    /// Mirrored (centered, like audio visualization)
    Mirrored,
    /// Bars instead of smooth line
    Bars,
    /// Dots at data points
    Dots,
    /// Smooth bezier curve
    Smooth,
}

/// Interpolation method for smooth curves
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Interpolation {
    /// No interpolation (connect points directly)
    #[default]
    Linear,
    /// Smooth bezier curves
    Bezier,
    /// Catmull-Rom spline
    CatmullRom,
    /// Step function
    Step,
}
