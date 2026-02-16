//! Core types for easing functions

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
