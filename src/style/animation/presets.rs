//! Pre-built animations for common widget effects

use std::time::Duration;

use super::{AnimationFillMode, KeyframeAnimation};
use crate::style::animation::easing;

/// Pre-built animations for common widget effects
pub mod widget_animations {
    use super::*;

    /// Fade in animation
    pub fn fade_in(duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("fade-in")
            .keyframe(0, |kf| kf.set("opacity", 0.0))
            .keyframe(100, |kf| kf.set("opacity", 1.0))
            .duration(Duration::from_millis(duration_ms))
            .easing(easing::ease_out)
            .fill_mode(AnimationFillMode::Forwards)
    }

    /// Fade out animation
    pub fn fade_out(duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("fade-out")
            .keyframe(0, |kf| kf.set("opacity", 1.0))
            .keyframe(100, |kf| kf.set("opacity", 0.0))
            .duration(Duration::from_millis(duration_ms))
            .easing(easing::ease_in)
            .fill_mode(AnimationFillMode::Forwards)
    }

    /// Slide in from left
    pub fn slide_in_left(distance: f32, duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("slide-in-left")
            .keyframe(0, |kf| kf.set("x", -distance).set("opacity", 0.0))
            .keyframe(100, |kf| kf.set("x", 0.0).set("opacity", 1.0))
            .duration(Duration::from_millis(duration_ms))
            .easing(easing::ease_out_cubic)
            .fill_mode(AnimationFillMode::Forwards)
    }

    /// Slide in from right
    pub fn slide_in_right(distance: f32, duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("slide-in-right")
            .keyframe(0, |kf| kf.set("x", distance).set("opacity", 0.0))
            .keyframe(100, |kf| kf.set("x", 0.0).set("opacity", 1.0))
            .duration(Duration::from_millis(duration_ms))
            .easing(easing::ease_out_cubic)
            .fill_mode(AnimationFillMode::Forwards)
    }

    /// Slide in from top
    pub fn slide_in_top(distance: f32, duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("slide-in-top")
            .keyframe(0, |kf| kf.set("y", -distance).set("opacity", 0.0))
            .keyframe(100, |kf| kf.set("y", 0.0).set("opacity", 1.0))
            .duration(Duration::from_millis(duration_ms))
            .easing(easing::ease_out_cubic)
            .fill_mode(AnimationFillMode::Forwards)
    }

    /// Slide in from bottom
    pub fn slide_in_bottom(distance: f32, duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("slide-in-bottom")
            .keyframe(0, |kf| kf.set("y", distance).set("opacity", 0.0))
            .keyframe(100, |kf| kf.set("y", 0.0).set("opacity", 1.0))
            .duration(Duration::from_millis(duration_ms))
            .easing(easing::ease_out_cubic)
            .fill_mode(AnimationFillMode::Forwards)
    }

    /// Scale up (zoom in)
    pub fn scale_up(duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("scale-up")
            .keyframe(0, |kf| kf.set("scale", 0.0).set("opacity", 0.0))
            .keyframe(100, |kf| kf.set("scale", 1.0).set("opacity", 1.0))
            .duration(Duration::from_millis(duration_ms))
            .easing(easing::back_out)
            .fill_mode(AnimationFillMode::Forwards)
    }

    /// Scale down (zoom out)
    pub fn scale_down(duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("scale-down")
            .keyframe(0, |kf| kf.set("scale", 1.0).set("opacity", 1.0))
            .keyframe(100, |kf| kf.set("scale", 0.0).set("opacity", 0.0))
            .duration(Duration::from_millis(duration_ms))
            .easing(easing::ease_in)
            .fill_mode(AnimationFillMode::Forwards)
    }

    /// Bounce animation
    pub fn bounce(duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("bounce")
            .keyframe(0, |kf| kf.set("y", 0.0))
            .keyframe(50, |kf| kf.set("y", -10.0))
            .keyframe(100, |kf| kf.set("y", 0.0))
            .duration(Duration::from_millis(duration_ms))
            .easing(easing::bounce_out)
    }

    /// Shake animation (for errors)
    pub fn shake(duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("shake")
            .keyframe(0, |kf| kf.set("x", 0.0))
            .keyframe(25, |kf| kf.set("x", -5.0))
            .keyframe(50, |kf| kf.set("x", 5.0))
            .keyframe(75, |kf| kf.set("x", -5.0))
            .keyframe(100, |kf| kf.set("x", 0.0))
            .duration(Duration::from_millis(duration_ms))
            .fill_mode(AnimationFillMode::Forwards)
    }

    /// Pulse animation (repeating)
    pub fn pulse(duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("pulse")
            .keyframe(0, |kf| kf.set("scale", 1.0))
            .keyframe(50, |kf| kf.set("scale", 1.1))
            .keyframe(100, |kf| kf.set("scale", 1.0))
            .duration(Duration::from_millis(duration_ms))
            .infinite()
    }

    /// Blink animation (repeating)
    pub fn blink(duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("blink")
            .keyframe(0, |kf| kf.set("opacity", 1.0))
            .keyframe(50, |kf| kf.set("opacity", 0.0))
            .keyframe(100, |kf| kf.set("opacity", 1.0))
            .duration(Duration::from_millis(duration_ms))
            .infinite()
    }

    /// Spin animation (repeating)
    pub fn spin(duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("spin")
            .keyframe(0, |kf| kf.set("rotation", 0.0))
            .keyframe(100, |kf| kf.set("rotation", 360.0))
            .duration(Duration::from_millis(duration_ms))
            .infinite()
    }

    /// Typing cursor blink
    pub fn cursor_blink() -> KeyframeAnimation {
        KeyframeAnimation::new("cursor-blink")
            .keyframe(0, |kf| kf.set("opacity", 1.0))
            .keyframe(50, |kf| kf.set("opacity", 0.0))
            .keyframe(100, |kf| kf.set("opacity", 1.0))
            .duration(Duration::from_millis(1000))
            .infinite()
    }

    /// Toast notification entrance
    pub fn toast_enter() -> KeyframeAnimation {
        KeyframeAnimation::new("toast-enter")
            .keyframe(0, |kf| kf.set("y", 20.0).set("opacity", 0.0))
            .keyframe(100, |kf| kf.set("y", 0.0).set("opacity", 1.0))
            .duration(Duration::from_millis(200))
            .easing(easing::ease_out_cubic)
            .fill_mode(AnimationFillMode::Forwards)
    }

    /// Toast notification exit
    pub fn toast_exit() -> KeyframeAnimation {
        KeyframeAnimation::new("toast-exit")
            .keyframe(0, |kf| kf.set("y", 0.0).set("opacity", 1.0))
            .keyframe(100, |kf| kf.set("y", -20.0).set("opacity", 0.0))
            .duration(Duration::from_millis(200))
            .easing(easing::ease_in)
            .fill_mode(AnimationFillMode::Forwards)
    }

    /// Modal dialog entrance
    pub fn modal_enter() -> KeyframeAnimation {
        KeyframeAnimation::new("modal-enter")
            .keyframe(0, |kf| kf.set("scale", 0.9).set("opacity", 0.0))
            .keyframe(100, |kf| kf.set("scale", 1.0).set("opacity", 1.0))
            .duration(Duration::from_millis(200))
            .easing(easing::ease_out)
            .fill_mode(AnimationFillMode::Forwards)
    }

    /// Modal dialog exit
    pub fn modal_exit() -> KeyframeAnimation {
        KeyframeAnimation::new("modal-exit")
            .keyframe(0, |kf| kf.set("scale", 1.0).set("opacity", 1.0))
            .keyframe(100, |kf| kf.set("scale", 0.95).set("opacity", 0.0))
            .duration(Duration::from_millis(150))
            .easing(easing::ease_in)
            .fill_mode(AnimationFillMode::Forwards)
    }

    /// Progress bar shimmer effect
    pub fn shimmer(duration_ms: u64) -> KeyframeAnimation {
        KeyframeAnimation::new("shimmer")
            .keyframe(0, |kf| kf.set("x", -100.0))
            .keyframe(100, |kf| kf.set("x", 100.0))
            .duration(Duration::from_millis(duration_ms))
            .infinite()
    }
}
