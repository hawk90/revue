//! Animation system with easing and tweening

pub mod easing;

mod choreographer;
mod group;
mod keyframe;
mod presets;
mod stagger;
mod tween;

#[cfg(test)]
mod tests;

// Re-exports
pub use choreographer::Choreographer;
pub use group::{AnimationGroup, GroupMode};
pub use keyframe::{AnimationDirection, AnimationFillMode, CssKeyframe, KeyframeAnimation};
pub use presets::widget_animations;
pub use stagger::Stagger;
pub use tween::{Animation, Animations, Tween};

// Public types
/// Easing function type
pub type EasingFn = fn(f32) -> f32;

/// Animation state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimationState {
    /// Not started
    Pending,
    /// Currently running
    Running,
    /// Paused
    Paused,
    /// Finished
    Completed,
}
