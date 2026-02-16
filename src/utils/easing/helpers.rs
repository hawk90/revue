//! Utility functions and interpolator for easing

use crate::utils::easing::{types::Easing, EasingFn};

/// Interpolate between two values using an easing function
///
/// # Example
///
/// ```rust,ignore
/// use revue::utils::easing::{lerp, Easing};
///
/// let value = lerp(0.0, 100.0, 0.5, Easing::OutQuad);
/// assert!((value - 75.0).abs() < 0.001);
/// ```
pub fn lerp(start: f64, end: f64, t: f64, easing: Easing) -> f64 {
    start + (end - start) * easing.ease(t)
}

/// Interpolate using a custom easing function
pub fn lerp_fn(start: f64, end: f64, t: f64, f: EasingFn) -> f64 {
    start + (end - start) * f(t)
}

/// Create an animation interpolator
#[derive(Clone, Debug)]
pub struct Interpolator {
    /// Start value
    pub start: f64,
    /// End value
    pub end: f64,
    /// Easing function
    pub easing: Easing,
}

impl Interpolator {
    /// Create a new interpolator
    pub fn new(start: f64, end: f64) -> Self {
        Self {
            start,
            end,
            easing: Easing::Linear,
        }
    }

    /// Set easing function
    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Get interpolated value at progress t (0.0 to 1.0)
    pub fn at(&self, t: f64) -> f64 {
        lerp(self.start, self.end, t, self.easing)
    }
}
