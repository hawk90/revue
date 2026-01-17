//! Easing functions for animations
//!
//! Provides standard easing functions for smooth animations and transitions.
//! Based on Robert Penner's easing equations.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::easing::{ease_out_quad, Easing};
//!
//! // Using function directly
//! let progress = 0.5;  // 50% through animation
//! let eased = ease_out_quad(progress);
//!
//! // Using Easing enum
//! let eased = Easing::OutQuad.ease(progress);
//!
//! // Interpolate between values
//! let start = 0.0;
//! let end = 100.0;
//! let value = start + (end - start) * eased;
//! ```

use std::f64::consts::PI;

/// Easing function type
///
/// All easing functions take a progress value (0.0 to 1.0) and return
/// the eased value (typically 0.0 to 1.0, but may exceed for elastic/back).
pub type EasingFn = fn(f64) -> f64;

/// Standard easing types
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Easing {
    /// No easing, constant speed
    #[default]
    Linear,

    // Quadratic
    /// Quadratic ease in (accelerate)
    InQuad,
    /// Quadratic ease out (decelerate)
    OutQuad,
    /// Quadratic ease in-out
    InOutQuad,

    // Cubic
    /// Cubic ease in
    InCubic,
    /// Cubic ease out
    OutCubic,
    /// Cubic ease in-out
    InOutCubic,

    // Quartic
    /// Quartic ease in
    InQuart,
    /// Quartic ease out
    OutQuart,
    /// Quartic ease in-out
    InOutQuart,

    // Quintic
    /// Quintic ease in
    InQuint,
    /// Quintic ease out
    OutQuint,
    /// Quintic ease in-out
    InOutQuint,

    // Sine
    /// Sine ease in
    InSine,
    /// Sine ease out
    OutSine,
    /// Sine ease in-out
    InOutSine,

    // Exponential
    /// Exponential ease in
    InExpo,
    /// Exponential ease out
    OutExpo,
    /// Exponential ease in-out
    InOutExpo,

    // Circular
    /// Circular ease in
    InCirc,
    /// Circular ease out
    OutCirc,
    /// Circular ease in-out
    InOutCirc,

    // Back (overshoots)
    /// Back ease in
    InBack,
    /// Back ease out
    OutBack,
    /// Back ease in-out
    InOutBack,

    // Elastic (spring-like)
    /// Elastic ease in
    InElastic,
    /// Elastic ease out
    OutElastic,
    /// Elastic ease in-out
    InOutElastic,

    // Bounce
    /// Bounce ease in
    InBounce,
    /// Bounce ease out
    OutBounce,
    /// Bounce ease in-out
    InOutBounce,
}

impl Easing {
    /// Apply the easing function to a progress value
    ///
    /// # Arguments
    ///
    /// * `t` - Progress value from 0.0 to 1.0
    ///
    /// # Returns
    ///
    /// The eased value
    pub fn ease(&self, t: f64) -> f64 {
        match self {
            Easing::Linear => linear(t),

            Easing::InQuad => ease_in_quad(t),
            Easing::OutQuad => ease_out_quad(t),
            Easing::InOutQuad => ease_in_out_quad(t),

            Easing::InCubic => ease_in_cubic(t),
            Easing::OutCubic => ease_out_cubic(t),
            Easing::InOutCubic => ease_in_out_cubic(t),

            Easing::InQuart => ease_in_quart(t),
            Easing::OutQuart => ease_out_quart(t),
            Easing::InOutQuart => ease_in_out_quart(t),

            Easing::InQuint => ease_in_quint(t),
            Easing::OutQuint => ease_out_quint(t),
            Easing::InOutQuint => ease_in_out_quint(t),

            Easing::InSine => ease_in_sine(t),
            Easing::OutSine => ease_out_sine(t),
            Easing::InOutSine => ease_in_out_sine(t),

            Easing::InExpo => ease_in_expo(t),
            Easing::OutExpo => ease_out_expo(t),
            Easing::InOutExpo => ease_in_out_expo(t),

            Easing::InCirc => ease_in_circ(t),
            Easing::OutCirc => ease_out_circ(t),
            Easing::InOutCirc => ease_in_out_circ(t),

            Easing::InBack => ease_in_back(t),
            Easing::OutBack => ease_out_back(t),
            Easing::InOutBack => ease_in_out_back(t),

            Easing::InElastic => ease_in_elastic(t),
            Easing::OutElastic => ease_out_elastic(t),
            Easing::InOutElastic => ease_in_out_elastic(t),

            Easing::InBounce => ease_in_bounce(t),
            Easing::OutBounce => ease_out_bounce(t),
            Easing::InOutBounce => ease_in_out_bounce(t),
        }
    }

    /// Get the easing function pointer
    pub fn function(&self) -> EasingFn {
        match self {
            Easing::Linear => linear,

            Easing::InQuad => ease_in_quad,
            Easing::OutQuad => ease_out_quad,
            Easing::InOutQuad => ease_in_out_quad,

            Easing::InCubic => ease_in_cubic,
            Easing::OutCubic => ease_out_cubic,
            Easing::InOutCubic => ease_in_out_cubic,

            Easing::InQuart => ease_in_quart,
            Easing::OutQuart => ease_out_quart,
            Easing::InOutQuart => ease_in_out_quart,

            Easing::InQuint => ease_in_quint,
            Easing::OutQuint => ease_out_quint,
            Easing::InOutQuint => ease_in_out_quint,

            Easing::InSine => ease_in_sine,
            Easing::OutSine => ease_out_sine,
            Easing::InOutSine => ease_in_out_sine,

            Easing::InExpo => ease_in_expo,
            Easing::OutExpo => ease_out_expo,
            Easing::InOutExpo => ease_in_out_expo,

            Easing::InCirc => ease_in_circ,
            Easing::OutCirc => ease_out_circ,
            Easing::InOutCirc => ease_in_out_circ,

            Easing::InBack => ease_in_back,
            Easing::OutBack => ease_out_back,
            Easing::InOutBack => ease_in_out_back,

            Easing::InElastic => ease_in_elastic,
            Easing::OutElastic => ease_out_elastic,
            Easing::InOutElastic => ease_in_out_elastic,

            Easing::InBounce => ease_in_bounce,
            Easing::OutBounce => ease_out_bounce,
            Easing::InOutBounce => ease_in_out_bounce,
        }
    }
}

// ============================================================================
// Linear
// ============================================================================

/// Linear interpolation (no easing)
pub fn linear(t: f64) -> f64 {
    t.clamp(0.0, 1.0)
}

// ============================================================================
// Quadratic
// ============================================================================

/// Quadratic ease in
pub fn ease_in_quad(t: f64) -> f64 {
    t * t
}

/// Quadratic ease out
pub fn ease_out_quad(t: f64) -> f64 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Quadratic ease in-out
pub fn ease_in_out_quad(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

// ============================================================================
// Cubic
// ============================================================================

/// Cubic ease in
pub fn ease_in_cubic(t: f64) -> f64 {
    t * t * t
}

/// Cubic ease out
pub fn ease_out_cubic(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

/// Cubic ease in-out
pub fn ease_in_out_cubic(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

// ============================================================================
// Quartic
// ============================================================================

/// Quartic ease in
pub fn ease_in_quart(t: f64) -> f64 {
    t * t * t * t
}

/// Quartic ease out
pub fn ease_out_quart(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(4)
}

/// Quartic ease in-out
pub fn ease_in_out_quart(t: f64) -> f64 {
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(4) / 2.0
    }
}

// ============================================================================
// Quintic
// ============================================================================

/// Quintic ease in
pub fn ease_in_quint(t: f64) -> f64 {
    t * t * t * t * t
}

/// Quintic ease out
pub fn ease_out_quint(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(5)
}

/// Quintic ease in-out
pub fn ease_in_out_quint(t: f64) -> f64 {
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(5) / 2.0
    }
}

// ============================================================================
// Sine
// ============================================================================

/// Sine ease in
pub fn ease_in_sine(t: f64) -> f64 {
    1.0 - (t * PI / 2.0).cos()
}

/// Sine ease out
pub fn ease_out_sine(t: f64) -> f64 {
    (t * PI / 2.0).sin()
}

/// Sine ease in-out
pub fn ease_in_out_sine(t: f64) -> f64 {
    -(((t * PI).cos() - 1.0) / 2.0)
}

// ============================================================================
// Exponential
// ============================================================================

/// Exponential ease in
pub fn ease_in_expo(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else {
        2.0_f64.powf(10.0 * t - 10.0)
    }
}

/// Exponential ease out
pub fn ease_out_expo(t: f64) -> f64 {
    if t == 1.0 {
        1.0
    } else {
        1.0 - 2.0_f64.powf(-10.0 * t)
    }
}

/// Exponential ease in-out
pub fn ease_in_out_expo(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else if t < 0.5 {
        2.0_f64.powf(20.0 * t - 10.0) / 2.0
    } else {
        (2.0 - 2.0_f64.powf(-20.0 * t + 10.0)) / 2.0
    }
}

// ============================================================================
// Circular
// ============================================================================

/// Circular ease in
pub fn ease_in_circ(t: f64) -> f64 {
    1.0 - (1.0 - t * t).sqrt()
}

/// Circular ease out
pub fn ease_out_circ(t: f64) -> f64 {
    (1.0 - (t - 1.0).powi(2)).sqrt()
}

/// Circular ease in-out
pub fn ease_in_out_circ(t: f64) -> f64 {
    if t < 0.5 {
        (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
}

// ============================================================================
// Back (overshoots)
// ============================================================================

const C1: f64 = 1.70158;
const C2: f64 = C1 * 1.525;
const C3: f64 = C1 + 1.0;

/// Back ease in (overshoots at start)
pub fn ease_in_back(t: f64) -> f64 {
    C3 * t * t * t - C1 * t * t
}

/// Back ease out (overshoots at end)
pub fn ease_out_back(t: f64) -> f64 {
    1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
}

/// Back ease in-out
pub fn ease_in_out_back(t: f64) -> f64 {
    if t < 0.5 {
        ((2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
    } else {
        ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
    }
}

// ============================================================================
// Elastic
// ============================================================================

const C4: f64 = (2.0 * PI) / 3.0;
const C5: f64 = (2.0 * PI) / 4.5;

/// Elastic ease in
pub fn ease_in_elastic(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        -2.0_f64.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * C4).sin()
    }
}

/// Elastic ease out
pub fn ease_out_elastic(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0
    }
}

/// Elastic ease in-out
pub fn ease_in_out_elastic(t: f64) -> f64 {
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else if t < 0.5 {
        -(2.0_f64.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0
    } else {
        (2.0_f64.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * C5).sin()) / 2.0 + 1.0
    }
}

// ============================================================================
// Bounce
// ============================================================================

const N1: f64 = 7.5625;
const D1: f64 = 2.75;

/// Bounce ease out (base function)
pub fn ease_out_bounce(t: f64) -> f64 {
    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t = t - 1.5 / D1;
        N1 * t * t + 0.75
    } else if t < 2.5 / D1 {
        let t = t - 2.25 / D1;
        N1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / D1;
        N1 * t * t + 0.984375
    }
}

/// Bounce ease in
pub fn ease_in_bounce(t: f64) -> f64 {
    1.0 - ease_out_bounce(1.0 - t)
}

/// Bounce ease in-out
pub fn ease_in_out_bounce(t: f64) -> f64 {
    if t < 0.5 {
        (1.0 - ease_out_bounce(1.0 - 2.0 * t)) / 2.0
    } else {
        (1.0 + ease_out_bounce(2.0 * t - 1.0)) / 2.0
    }
}

// ============================================================================
// Utility functions
// ============================================================================

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        assert_eq!(linear(0.0), 0.0);
        assert_eq!(linear(0.5), 0.5);
        assert_eq!(linear(1.0), 1.0);
    }

    #[test]
    fn test_ease_out_quad() {
        assert_eq!(ease_out_quad(0.0), 0.0);
        assert_eq!(ease_out_quad(1.0), 1.0);
        // Ease out should be faster at start
        assert!(ease_out_quad(0.5) > 0.5);
    }

    #[test]
    fn test_ease_in_quad() {
        assert_eq!(ease_in_quad(0.0), 0.0);
        assert_eq!(ease_in_quad(1.0), 1.0);
        // Ease in should be slower at start
        assert!(ease_in_quad(0.5) < 0.5);
    }

    #[test]
    fn test_ease_in_out_quad() {
        assert_eq!(ease_in_out_quad(0.0), 0.0);
        assert_eq!(ease_in_out_quad(1.0), 1.0);
        assert!((ease_in_out_quad(0.5) - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_easing_enum() {
        let easing = Easing::OutQuad;
        assert_eq!(easing.ease(0.0), 0.0);
        assert_eq!(easing.ease(1.0), 1.0);
    }

    #[test]
    fn test_lerp() {
        let value = lerp(0.0, 100.0, 0.5, Easing::Linear);
        assert!((value - 50.0).abs() < 0.001);

        let value = lerp(0.0, 100.0, 0.5, Easing::OutQuad);
        assert!(value > 50.0); // Ease out is faster
    }

    #[test]
    fn test_interpolator() {
        let interp = Interpolator::new(0.0, 100.0).easing(Easing::OutQuad);

        assert_eq!(interp.at(0.0), 0.0);
        assert_eq!(interp.at(1.0), 100.0);
    }

    #[test]
    fn test_all_easings_boundary() {
        let easings = [
            Easing::Linear,
            Easing::InQuad,
            Easing::OutQuad,
            Easing::InOutQuad,
            Easing::InCubic,
            Easing::OutCubic,
            Easing::InOutCubic,
            Easing::InQuart,
            Easing::OutQuart,
            Easing::InOutQuart,
            Easing::InQuint,
            Easing::OutQuint,
            Easing::InOutQuint,
            Easing::InSine,
            Easing::OutSine,
            Easing::InOutSine,
            Easing::InExpo,
            Easing::OutExpo,
            Easing::InOutExpo,
            Easing::InCirc,
            Easing::OutCirc,
            Easing::InOutCirc,
            Easing::InBack,
            Easing::OutBack,
            Easing::InOutBack,
            Easing::InElastic,
            Easing::OutElastic,
            Easing::InOutElastic,
            Easing::InBounce,
            Easing::OutBounce,
            Easing::InOutBounce,
        ];

        for easing in easings {
            let start = easing.ease(0.0);
            let end = easing.ease(1.0);

            // All easings should start at ~0 and end at ~1
            // (Back and Elastic may slightly overshoot)
            assert!(start.abs() < 0.01, "{:?} start: {}", easing, start);
            assert!((end - 1.0).abs() < 0.01, "{:?} end: {}", easing, end);
        }
    }

    // =========================================================================
    // Easing enum tests
    // =========================================================================

    #[test]
    fn test_easing_default() {
        let easing = Easing::default();
        assert_eq!(easing, Easing::Linear);
    }

    #[test]
    fn test_easing_clone() {
        let easing = Easing::OutQuad;
        let cloned = easing;
        assert_eq!(easing, cloned);
    }

    #[test]
    fn test_easing_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Easing::Linear);
        set.insert(Easing::OutQuad);
        set.insert(Easing::Linear); // Duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_easing_function() {
        let easing = Easing::OutQuad;
        let func = easing.function();
        assert_eq!(func(0.0), ease_out_quad(0.0));
        assert_eq!(func(1.0), ease_out_quad(1.0));
    }

    // =========================================================================
    // Individual easing function tests
    // =========================================================================

    #[test]
    fn test_linear_clamps() {
        assert_eq!(linear(-0.5), 0.0);
        assert_eq!(linear(1.5), 1.0);
    }

    #[test]
    fn test_ease_in_cubic() {
        assert_eq!(ease_in_cubic(0.0), 0.0);
        assert_eq!(ease_in_cubic(1.0), 1.0);
        assert!(ease_in_cubic(0.5) < 0.5);
    }

    #[test]
    fn test_ease_out_cubic() {
        assert_eq!(ease_out_cubic(0.0), 0.0);
        assert_eq!(ease_out_cubic(1.0), 1.0);
        assert!(ease_out_cubic(0.5) > 0.5);
    }

    #[test]
    fn test_ease_in_out_cubic() {
        assert_eq!(ease_in_out_cubic(0.0), 0.0);
        assert_eq!(ease_in_out_cubic(1.0), 1.0);
        assert!((ease_in_out_cubic(0.5) - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_ease_in_quart() {
        assert_eq!(ease_in_quart(0.0), 0.0);
        assert_eq!(ease_in_quart(1.0), 1.0);
    }

    #[test]
    fn test_ease_out_quart() {
        assert_eq!(ease_out_quart(0.0), 0.0);
        assert_eq!(ease_out_quart(1.0), 1.0);
    }

    #[test]
    fn test_ease_in_out_quart() {
        assert_eq!(ease_in_out_quart(0.0), 0.0);
        assert_eq!(ease_in_out_quart(1.0), 1.0);
    }

    #[test]
    fn test_ease_in_quint() {
        assert_eq!(ease_in_quint(0.0), 0.0);
        assert_eq!(ease_in_quint(1.0), 1.0);
    }

    #[test]
    fn test_ease_out_quint() {
        assert_eq!(ease_out_quint(0.0), 0.0);
        assert_eq!(ease_out_quint(1.0), 1.0);
    }

    #[test]
    fn test_ease_in_out_quint() {
        assert_eq!(ease_in_out_quint(0.0), 0.0);
        assert_eq!(ease_in_out_quint(1.0), 1.0);
    }

    #[test]
    fn test_ease_in_sine() {
        assert!((ease_in_sine(0.0) - 0.0).abs() < 0.001);
        assert!((ease_in_sine(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ease_out_sine() {
        assert!((ease_out_sine(0.0) - 0.0).abs() < 0.001);
        assert!((ease_out_sine(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ease_in_out_sine() {
        assert!((ease_in_out_sine(0.0) - 0.0).abs() < 0.001);
        assert!((ease_in_out_sine(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ease_in_expo() {
        assert_eq!(ease_in_expo(0.0), 0.0);
        assert!((ease_in_expo(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ease_out_expo() {
        assert!((ease_out_expo(0.0) - 0.0).abs() < 0.001);
        assert_eq!(ease_out_expo(1.0), 1.0);
    }

    #[test]
    fn test_ease_in_out_expo() {
        assert_eq!(ease_in_out_expo(0.0), 0.0);
        assert_eq!(ease_in_out_expo(1.0), 1.0);
        assert!((ease_in_out_expo(0.5) - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_ease_in_circ() {
        assert!((ease_in_circ(0.0) - 0.0).abs() < 0.001);
        assert!((ease_in_circ(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ease_out_circ() {
        assert!((ease_out_circ(0.0) - 0.0).abs() < 0.001);
        assert!((ease_out_circ(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ease_in_out_circ() {
        assert!((ease_in_out_circ(0.0) - 0.0).abs() < 0.001);
        assert!((ease_in_out_circ(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ease_in_back() {
        assert!((ease_in_back(0.0) - 0.0).abs() < 0.001);
        assert!((ease_in_back(1.0) - 1.0).abs() < 0.001);
        // Back overshoots (goes negative at start)
        assert!(ease_in_back(0.3) < 0.0);
    }

    #[test]
    fn test_ease_out_back() {
        assert!((ease_out_back(0.0) - 0.0).abs() < 0.001);
        assert!((ease_out_back(1.0) - 1.0).abs() < 0.001);
        // Back overshoots (goes above 1 near end)
        assert!(ease_out_back(0.7) > 1.0);
    }

    #[test]
    fn test_ease_in_out_back() {
        assert!((ease_in_out_back(0.0) - 0.0).abs() < 0.001);
        assert!((ease_in_out_back(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ease_in_elastic() {
        assert_eq!(ease_in_elastic(0.0), 0.0);
        assert_eq!(ease_in_elastic(1.0), 1.0);
    }

    #[test]
    fn test_ease_out_elastic() {
        assert_eq!(ease_out_elastic(0.0), 0.0);
        assert_eq!(ease_out_elastic(1.0), 1.0);
    }

    #[test]
    fn test_ease_in_out_elastic() {
        assert_eq!(ease_in_out_elastic(0.0), 0.0);
        assert_eq!(ease_in_out_elastic(1.0), 1.0);
    }

    #[test]
    fn test_ease_in_bounce() {
        assert!((ease_in_bounce(0.0) - 0.0).abs() < 0.001);
        assert!((ease_in_bounce(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ease_out_bounce() {
        assert!((ease_out_bounce(0.0) - 0.0).abs() < 0.001);
        assert!((ease_out_bounce(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ease_in_out_bounce() {
        assert!((ease_in_out_bounce(0.0) - 0.0).abs() < 0.001);
        assert!((ease_in_out_bounce(1.0) - 1.0).abs() < 0.001);
    }

    // =========================================================================
    // Lerp tests
    // =========================================================================

    #[test]
    fn test_lerp_boundaries() {
        let value_start = lerp(10.0, 100.0, 0.0, Easing::Linear);
        let value_end = lerp(10.0, 100.0, 1.0, Easing::Linear);
        assert!((value_start - 10.0).abs() < 0.001);
        assert!((value_end - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_lerp_negative_range() {
        let value = lerp(-100.0, 100.0, 0.5, Easing::Linear);
        assert!((value - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_lerp_fn() {
        let value = lerp_fn(0.0, 100.0, 0.5, linear);
        assert!((value - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_lerp_fn_custom() {
        // Custom easing: always return 1.0
        let always_one: EasingFn = |_| 1.0;
        let value = lerp_fn(0.0, 100.0, 0.5, always_one);
        assert!((value - 100.0).abs() < 0.001);
    }

    // =========================================================================
    // Interpolator tests
    // =========================================================================

    #[test]
    fn test_interpolator_new() {
        let interp = Interpolator::new(0.0, 100.0);
        assert_eq!(interp.start, 0.0);
        assert_eq!(interp.end, 100.0);
        assert_eq!(interp.easing, Easing::Linear);
    }

    #[test]
    fn test_interpolator_easing_builder() {
        let interp = Interpolator::new(0.0, 100.0).easing(Easing::InOutQuad);
        assert_eq!(interp.easing, Easing::InOutQuad);
    }

    #[test]
    fn test_interpolator_at_boundaries() {
        let interp = Interpolator::new(50.0, 150.0);
        assert_eq!(interp.at(0.0), 50.0);
        assert_eq!(interp.at(1.0), 150.0);
    }

    #[test]
    fn test_interpolator_at_middle() {
        let interp = Interpolator::new(0.0, 100.0);
        assert!((interp.at(0.5) - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_interpolator_clone() {
        let interp = Interpolator::new(0.0, 100.0).easing(Easing::OutQuad);
        let cloned = interp.clone();
        assert_eq!(cloned.start, 0.0);
        assert_eq!(cloned.end, 100.0);
        assert_eq!(cloned.easing, Easing::OutQuad);
    }

    // =========================================================================
    // Easing behavior tests
    // =========================================================================

    #[test]
    fn test_ease_in_slower_at_start() {
        // All ease-in functions should be slower at the start
        let ease_ins = [
            Easing::InQuad,
            Easing::InCubic,
            Easing::InQuart,
            Easing::InQuint,
        ];
        for easing in ease_ins {
            let value = easing.ease(0.25);
            assert!(value < 0.25, "{:?} at 0.25 = {}", easing, value);
        }
    }

    #[test]
    fn test_ease_out_faster_at_start() {
        // All ease-out functions should be faster at the start
        let ease_outs = [
            Easing::OutQuad,
            Easing::OutCubic,
            Easing::OutQuart,
            Easing::OutQuint,
        ];
        for easing in ease_outs {
            let value = easing.ease(0.25);
            assert!(value > 0.25, "{:?} at 0.25 = {}", easing, value);
        }
    }

    #[test]
    fn test_ease_in_out_symmetric() {
        // In-out functions should pass through (0.5, 0.5) approximately
        let in_outs = [
            Easing::InOutQuad,
            Easing::InOutCubic,
            Easing::InOutQuart,
            Easing::InOutQuint,
            Easing::InOutSine,
        ];
        for easing in in_outs {
            let value = easing.ease(0.5);
            assert!(
                (value - 0.5).abs() < 0.01,
                "{:?} at 0.5 = {}",
                easing,
                value
            );
        }
    }
}
