//! Core types and enums for animation utilities

use crate::utils::easing::Easing;

/// Trait for types that can be interpolated
pub trait Interpolatable: Clone {
    /// Interpolate between two values
    fn lerp(&self, other: &Self, t: f64) -> Self;
}

impl Interpolatable for f32 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        *self + (*other - *self) * t as f32
    }
}

impl Interpolatable for f64 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        *self + (*other - *self) * t
    }
}

impl Interpolatable for i32 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        (*self as f64 + (*other - *self) as f64 * t).round() as i32
    }
}

impl Interpolatable for u8 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        (*self as f64 + (*other as f64 - *self as f64) * t).round() as u8
    }
}

impl Interpolatable for u16 {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        (*self as f64 + (*other as f64 - *self as f64) * t).round() as u16
    }
}

impl Interpolatable for (f64, f64) {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        (self.0.lerp(&other.0, t), self.1.lerp(&other.1, t))
    }
}

impl Interpolatable for (f64, f64, f64) {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        (
            self.0.lerp(&other.0, t),
            self.1.lerp(&other.1, t),
            self.2.lerp(&other.2, t),
        )
    }
}

/// A single keyframe in an animation
#[derive(Clone, Debug)]
pub struct Keyframe<T: Interpolatable> {
    /// Time position (0.0 to 1.0)
    pub time: f64,
    /// Value at this keyframe
    pub value: T,
    /// Easing function to use when transitioning TO this keyframe
    pub easing: Easing,
}

impl<T: Interpolatable> Keyframe<T> {
    /// Create a new keyframe
    pub fn new(time: f64, value: T) -> Self {
        Self {
            time: time.clamp(0.0, 1.0),
            value,
            easing: Easing::Linear,
        }
    }

    /// Set easing function
    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }
}

/// Animation step in a sequence
#[derive(Clone)]
pub struct SequenceStep {
    /// Duration of this step
    pub duration: std::time::Duration,
    /// Easing for this step
    pub easing: Easing,
    /// Target value (normalized 0.0 to 1.0)
    pub target: f64,
}

impl SequenceStep {
    /// Create a new step
    pub fn new(duration: std::time::Duration, target: f64) -> Self {
        Self {
            duration,
            easing: Easing::Linear,
            target: target.clamp(0.0, 1.0),
        }
    }

    /// Set easing
    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }
}
