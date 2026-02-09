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

mod functions;
mod helpers;
mod types;

// Re-export public API
pub use helpers::{lerp, lerp_fn, Interpolator};
pub use types::{Easing, EasingFn};

// Re-export all easing functions
pub use functions::{
    ease_in_back, ease_in_bounce, ease_in_circ, ease_in_cubic, ease_in_elastic, ease_in_expo,
    ease_in_out_back, ease_in_out_bounce, ease_in_out_circ, ease_in_out_cubic, ease_in_out_elastic,
    ease_in_out_expo, ease_in_out_quad, ease_in_out_quart, ease_in_out_quint, ease_in_out_sine,
    ease_in_quad, ease_in_quart, ease_in_quint, ease_in_sine, ease_out_back, ease_out_bounce,
    ease_out_circ, ease_out_cubic, ease_out_elastic, ease_out_expo, ease_out_quad, ease_out_quart,
    ease_out_quint, ease_out_sine, linear,
};

// ============================================================================
// Easing impl
// ============================================================================

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
