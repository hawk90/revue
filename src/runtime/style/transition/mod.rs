//! CSS Transition animations
//!
//! Supports smooth transitions between style property values.
//!
//! # Example
//!
//! ```css
//! .button {
//!     background: #333;
//!     transition: background 0.3s ease-in-out;
//! }
//!
//! .button:hover {
//!     background: #555;
//! }
//! ```

mod definition;
mod manager;

pub(crate) use definition::parse_duration;
pub use definition::{Easing, Transition, Transitions};
pub use manager::{effective_duration, should_skip_animation, ActiveTransition, TransitionManager};

/// Interpolate between two u8 values
pub fn lerp_u8(from: u8, to: u8, t: f32) -> u8 {
    let from = from as f32;
    let to = to as f32;
    (from + (to - from) * t).round() as u8
}

/// Interpolate between two f32 values
pub fn lerp_f32(from: f32, to: f32, t: f32) -> f32 {
    from + (to - from) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp_f32() {
        assert_eq!(lerp_f32(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp_f32(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp_f32(0.0, 10.0, 1.0), 10.0);
    }

    #[test]
    fn test_lerp_u8() {
        assert_eq!(lerp_u8(0, 255, 0.5), 128);
        assert_eq!(lerp_u8(0, 255, 0.0), 0);
        assert_eq!(lerp_u8(0, 255, 1.0), 255);
    }
}
