//! Animation system with easing and tweening

pub mod easing;

mod choreographer;
mod group;
mod keyframe;
mod presets;
mod stagger;
mod tween;

#[cfg(test)]
mod tests {
//! Animation system tests

use std::time::Duration;

use super::*;
use crate::style::animation::easing;

#[cfg(test)]
mod tests {
    use super::*;

    // Easing function tests
    #[test]
    fn test_easing_linear() {
        assert_eq!(easing::linear(0.0), 0.0);
        assert_eq!(easing::linear(0.5), 0.5);
        assert_eq!(easing::linear(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_in() {
        assert_eq!(easing::ease_in(0.0), 0.0);
        assert!(easing::ease_in(0.5) < 0.5);
        assert_eq!(easing::ease_in(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_out() {
        assert_eq!(easing::ease_out(0.0), 0.0);
        assert!(easing::ease_out(0.5) > 0.5);
        assert_eq!(easing::ease_out(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_in_out() {
        assert_eq!(easing::ease_in_out(0.0), 0.0);
        assert!((easing::ease_in_out(0.5) - 0.5).abs() < 0.01);
        assert_eq!(easing::ease_in_out(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_in_out_first_half() {
        // t < 0.5 branch
        let result = easing::ease_in_out(0.25);
        assert!(result < 0.25);
    }

    #[test]
    fn test_easing_ease_in_out_second_half() {
        // t >= 0.5 branch
        let result = easing::ease_in_out(0.75);
        assert!(result > 0.75);
    }

    #[test]
    fn test_easing_ease_in_cubic() {
        assert_eq!(easing::ease_in_cubic(0.0), 0.0);
        assert!(easing::ease_in_cubic(0.5) < 0.5);
        assert_eq!(easing::ease_in_cubic(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_out_cubic() {
        assert_eq!(easing::ease_out_cubic(0.0), 0.0);
        assert!(easing::ease_out_cubic(0.5) > 0.5);
        assert_eq!(easing::ease_out_cubic(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_in_out_cubic() {
        assert_eq!(easing::ease_in_out_cubic(0.0), 0.0);
        assert!((easing::ease_in_out_cubic(0.5) - 0.5).abs() < 0.01);
        assert_eq!(easing::ease_in_out_cubic(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_in_out_cubic_first_half() {
        let result = easing::ease_in_out_cubic(0.25);
        assert!(result < 0.25);
    }

    #[test]
    fn test_easing_ease_in_out_cubic_second_half() {
        let result = easing::ease_in_out_cubic(0.75);
        assert!(result > 0.75);
    }

    #[test]
    fn test_easing_bounce_out() {
        assert_eq!(easing::bounce_out(0.0), 0.0);
        assert!(easing::bounce_out(1.0) > 0.99);
    }

    #[test]
    fn test_easing_bounce_out_branches() {
        // First branch: t < 1/2.75
        let r1 = easing::bounce_out(0.2);
        assert!(r1 > 0.0 && r1 < 1.0);

        // Second branch: t < 2/2.75
        let r2 = easing::bounce_out(0.5);
        assert!(r2 > 0.0 && r2 < 1.0);

        // Third branch: t < 2.5/2.75
        let r3 = easing::bounce_out(0.85);
        assert!(r3 > 0.0 && r3 < 1.0);

        // Fourth branch: t >= 2.5/2.75
        let r4 = easing::bounce_out(0.95);
        assert!(r4 > 0.0 && r4 <= 1.0);
    }

    #[test]
    fn test_easing_elastic_out() {
        assert_eq!(easing::elastic_out(0.0), 0.0);
        assert_eq!(easing::elastic_out(1.0), 1.0);
    }

    #[test]
    fn test_easing_elastic_out_middle() {
        let result = easing::elastic_out(0.5);
        assert!(result > 0.9); // Elastic overshoots
    }

    #[test]
    fn test_easing_back_out() {
        assert!((easing::back_out(0.0) - 0.0).abs() < 0.001); // Starts at 0
        assert!(easing::back_out(0.5) > 1.0); // Overshoots past 1 mid-animation
        assert!((easing::back_out(1.0) - 1.0).abs() < 0.001); // Ends at 1
    }

    // AnimationState tests
    #[test]
    fn test_animation_state_eq() {
        assert_eq!(AnimationState::Pending, AnimationState::Pending);
        assert_eq!(AnimationState::Running, AnimationState::Running);
        assert_eq!(AnimationState::Paused, AnimationState::Paused);
        assert_eq!(AnimationState::Completed, AnimationState::Completed);
    }

    #[test]
    fn test_animation_state_clone() {
        let state = AnimationState::Running;
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    // Tween tests
    #[test]
    fn test_tween_new() {
        let tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
        assert_eq!(tween.state(), AnimationState::Pending);
    }

    #[test]
    fn test_tween_default() {
        let tween = Tween::default();
        assert_eq!(tween.from, 0.0);
        assert_eq!(tween.to, 1.0);
        assert_eq!(tween.duration, Duration::from_millis(300));
    }

    #[test]
    fn test_tween_builder_easing() {
        let tween = Tween::new(0.0, 1.0, Duration::from_millis(100)).easing(easing::ease_in);
        // Verify it doesn't crash and returns self
        assert_eq!(tween.state(), AnimationState::Pending);
    }

    #[test]
    fn test_tween_builder_delay() {
        let tween =
            Tween::new(0.0, 1.0, Duration::from_millis(100)).delay(Duration::from_millis(50));
        assert_eq!(tween.delay, Duration::from_millis(50));
    }

    #[test]
    fn test_tween_builder_repeat() {
        let tween = Tween::new(0.0, 1.0, Duration::from_millis(100)).repeat(3);
        assert_eq!(tween.repeat, 3);
    }

    #[test]
    fn test_tween_builder_reverse() {
        let tween = Tween::new(0.0, 1.0, Duration::from_millis(100)).reverse(true);
        assert!(tween.reverse);
    }

    #[test]
    fn test_tween_start() {
        let mut tween = Tween::new(0.0, 100.0, Duration::from_millis(100));
        tween.start();
        assert_eq!(tween.state(), AnimationState::Running);
        assert!(tween.is_running());
    }

    #[test]
    fn test_tween_pause() {
        let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
        tween.start();
        tween.pause();
        assert_eq!(tween.state(), AnimationState::Paused);
        assert!(!tween.is_running());
    }

    #[test]
    fn test_tween_pause_not_running() {
        let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
        tween.pause(); // Should do nothing
        assert_eq!(tween.state(), AnimationState::Pending);
    }

    #[test]
    fn test_tween_resume() {
        let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
        tween.start();
        tween.pause();
        tween.resume();
        assert_eq!(tween.state(), AnimationState::Running);
    }

    #[test]
    fn test_tween_resume_not_paused() {
        let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
        tween.start();
        tween.resume(); // Should do nothing
        assert_eq!(tween.state(), AnimationState::Running);
    }

    #[test]
    fn test_tween_reset() {
        let mut tween = Tween::new(0.0, 100.0, Duration::from_millis(100));
        tween.start();
        tween.reset();
        assert_eq!(tween.state(), AnimationState::Pending);
        assert!(!tween.is_running());
        assert!(!tween.is_completed());
    }

    #[test]
    fn test_tween_value_pending() {
        let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
        assert_eq!(tween.value(), 0.0); // Pending returns initial 'from' value
    }

    #[test]
    fn test_tween_progress_pending() {
        let tween = Tween::new(0.0, 100.0, Duration::from_secs(1));
        assert_eq!(tween.progress(), 0.0);
    }

    #[test]
    fn test_tween_is_completed() {
        let tween = Tween::new(0.0, 100.0, Duration::from_millis(1));
        assert!(!tween.is_completed());
    }

    // Animation tests
    #[test]
    fn test_animation_new() {
        let anim = Animation::new("test");
        assert_eq!(anim.name, "test");
        assert!(anim.tweens.is_empty());
    }

    #[test]
    fn test_animation_tween() {
        let anim = Animation::new("test").tween(Tween::new(0.0, 1.0, Duration::from_secs(1)));
        assert_eq!(anim.tweens.len(), 1);
    }

    #[test]
    fn test_animation_multiple_tweens() {
        let anim = Animation::new("test")
            .tween(Tween::new(0.0, 1.0, Duration::from_secs(1)))
            .tween(Tween::new(1.0, 0.0, Duration::from_secs(1)));
        assert_eq!(anim.tweens.len(), 2);
    }

    #[test]
    fn test_animation_start() {
        let mut anim = Animation::new("test").tween(Tween::new(0.0, 1.0, Duration::from_secs(1)));
        anim.start();
        assert!(anim.tweens[0].is_running());
    }

    #[test]
    fn test_animation_pause() {
        let mut anim = Animation::new("test").tween(Tween::new(0.0, 1.0, Duration::from_secs(1)));
        anim.start();
        anim.pause();
        assert_eq!(anim.tweens[0].state(), AnimationState::Paused);
    }

    #[test]
    fn test_animation_resume() {
        let mut anim = Animation::new("test").tween(Tween::new(0.0, 1.0, Duration::from_secs(1)));
        anim.start();
        anim.pause();
        anim.resume();
        assert!(anim.tweens[0].is_running());
    }

    #[test]
    fn test_animation_reset() {
        let mut anim = Animation::new("test").tween(Tween::new(0.0, 1.0, Duration::from_secs(1)));
        anim.start();
        anim.reset();
        assert_eq!(anim.tweens[0].state(), AnimationState::Pending);
    }

    #[test]
    fn test_animation_is_completed_empty() {
        let anim = Animation::new("test");
        assert!(anim.is_completed()); // Empty animation is complete
    }

    // Animations presets tests
    #[test]
    fn test_animations_fade_in() {
        let tween = Animations::fade_in(Duration::from_millis(300));
        assert_eq!(tween.from, 0.0);
        assert_eq!(tween.to, 1.0);
    }

    #[test]
    fn test_animations_fade_out() {
        let tween = Animations::fade_out(Duration::from_millis(300));
        assert_eq!(tween.from, 1.0);
        assert_eq!(tween.to, 0.0);
    }

    #[test]
    fn test_animations_slide_in_left() {
        let tween = Animations::slide_in_left(100.0, Duration::from_millis(300));
        assert_eq!(tween.from, -100.0);
        assert_eq!(tween.to, 0.0);
    }

    #[test]
    fn test_animations_slide_in_right() {
        let tween = Animations::slide_in_right(100.0, Duration::from_millis(300));
        assert_eq!(tween.from, 100.0);
        assert_eq!(tween.to, 0.0);
    }

    #[test]
    fn test_animations_scale_up() {
        let tween = Animations::scale_up(Duration::from_millis(300));
        assert_eq!(tween.from, 0.0);
        assert_eq!(tween.to, 1.0);
    }

    #[test]
    fn test_animations_bounce() {
        let tween = Animations::bounce(Duration::from_millis(300));
        assert_eq!(tween.from, 0.0);
        assert_eq!(tween.to, 1.0);
    }

    #[test]
    fn test_animations_pulse() {
        let tween = Animations::pulse(Duration::from_millis(300));
        assert_eq!(tween.from, 1.0);
        assert_eq!(tween.to, 1.2);
        assert!(tween.reverse);
        assert_eq!(tween.repeat, u32::MAX);
    }

    // CssKeyframe tests
    #[test]
    fn test_css_keyframe_new() {
        let kf = CssKeyframe::new(50);
        assert_eq!(kf.percent, 50);
        assert!(kf.properties.is_empty());
    }

    #[test]
    fn test_css_keyframe_new_clamped() {
        let kf = CssKeyframe::new(150); // Over 100
        assert_eq!(kf.percent, 100);
    }

    #[test]
    fn test_css_keyframe_set() {
        let kf = CssKeyframe::new(0).set("opacity", 0.5);
        assert_eq!(kf.get("opacity"), Some(0.5));
    }

    #[test]
    fn test_css_keyframe_get_missing() {
        let kf = CssKeyframe::new(0);
        assert_eq!(kf.get("opacity"), None);
    }

    #[test]
    fn test_css_keyframe_multiple_properties() {
        let kf = CssKeyframe::new(50)
            .set("opacity", 1.0)
            .set("scale", 1.5)
            .set("x", 100.0);
        assert_eq!(kf.get("opacity"), Some(1.0));
        assert_eq!(kf.get("scale"), Some(1.5));
        assert_eq!(kf.get("x"), Some(100.0));
    }

    // KeyframeAnimation tests
    #[test]
    fn test_keyframe_animation_new() {
        let anim = KeyframeAnimation::new("test");
        assert_eq!(anim.name(), "test");
        assert_eq!(anim.state(), AnimationState::Pending);
    }

    #[test]
    fn test_keyframe_animation_keyframe() {
        let anim = KeyframeAnimation::new("test")
            .keyframe(0, |kf| kf.set("opacity", 0.0))
            .keyframe(100, |kf| kf.set("opacity", 1.0));
        // Keyframes are stored
        assert!(!anim.is_running());
    }

    #[test]
    fn test_keyframe_animation_duration() {
        let anim = KeyframeAnimation::new("test").duration(Duration::from_millis(500));
        assert_eq!(anim.duration, Duration::from_millis(500));
    }

    #[test]
    fn test_keyframe_animation_delay() {
        let anim = KeyframeAnimation::new("test").delay(Duration::from_millis(100));
        assert_eq!(anim.delay, Duration::from_millis(100));
    }

    #[test]
    fn test_keyframe_animation_easing() {
        let anim = KeyframeAnimation::new("test").easing(easing::ease_out);
        assert_eq!(anim.state(), AnimationState::Pending);
    }

    #[test]
    fn test_keyframe_animation_iterations() {
        let anim = KeyframeAnimation::new("test").iterations(3);
        assert_eq!(anim.iterations, 3);
    }

    #[test]
    fn test_keyframe_animation_infinite() {
        let anim = KeyframeAnimation::new("test").infinite();
        assert_eq!(anim.iterations, 0);
    }

    #[test]
    fn test_keyframe_animation_direction() {
        let anim = KeyframeAnimation::new("test").direction(AnimationDirection::Reverse);
        assert_eq!(anim.direction, AnimationDirection::Reverse);
    }

    #[test]
    fn test_keyframe_animation_fill_mode() {
        let anim = KeyframeAnimation::new("test").fill_mode(AnimationFillMode::Forwards);
        assert_eq!(anim.fill_mode, AnimationFillMode::Forwards);
    }

    #[test]
    fn test_keyframe_animation_pause_resume() {
        let mut anim = KeyframeAnimation::new("test").duration(Duration::from_secs(1));
        anim.start();
        anim.pause();
        assert_eq!(anim.state(), AnimationState::Paused);
        anim.resume();
        assert!(anim.is_running());
    }

    #[test]
    fn test_keyframe_animation_reset() {
        let mut anim = KeyframeAnimation::new("test");
        anim.start();
        anim.reset();
        assert_eq!(anim.state(), AnimationState::Pending);
    }

    #[test]
    fn test_keyframe_animation_progress_pending() {
        let anim = KeyframeAnimation::new("test");
        assert_eq!(anim.progress(), 0.0);
    }

    // AnimationDirection tests
    #[test]
    fn test_animation_direction_default() {
        let dir = AnimationDirection::default();
        assert_eq!(dir, AnimationDirection::Normal);
    }

    #[test]
    fn test_animation_direction_variants() {
        assert_eq!(AnimationDirection::Normal, AnimationDirection::Normal);
        assert_eq!(AnimationDirection::Reverse, AnimationDirection::Reverse);
        assert_eq!(AnimationDirection::Alternate, AnimationDirection::Alternate);
        assert_eq!(
            AnimationDirection::AlternateReverse,
            AnimationDirection::AlternateReverse
        );
    }

    // AnimationFillMode tests
    #[test]
    fn test_animation_fill_mode_default() {
        let fill = AnimationFillMode::default();
        assert_eq!(fill, AnimationFillMode::None);
    }

    #[test]
    fn test_animation_fill_mode_variants() {
        assert_eq!(AnimationFillMode::None, AnimationFillMode::None);
        assert_eq!(AnimationFillMode::Forwards, AnimationFillMode::Forwards);
        assert_eq!(AnimationFillMode::Backwards, AnimationFillMode::Backwards);
        assert_eq!(AnimationFillMode::Both, AnimationFillMode::Both);
    }

    // Stagger tests
    #[test]
    fn test_stagger() {
        let stagger = Stagger::new(5, Duration::from_millis(50));
        assert_eq!(stagger.delay_for(0), Duration::ZERO);
        assert_eq!(stagger.delay_for(1), Duration::from_millis(50));
        assert_eq!(stagger.delay_for(4), Duration::from_millis(200));
    }

    // AnimationGroup tests
    #[test]
    fn test_animation_group_parallel() {
        let group = AnimationGroup::parallel()
            .with_animation(KeyframeAnimation::new("a").duration(Duration::from_millis(100)))
            .with_animation(KeyframeAnimation::new("b").duration(Duration::from_millis(200)));

        assert_eq!(group.total_duration(), Duration::from_millis(200));
    }

    #[test]
    fn test_animation_group_sequential() {
        let group = AnimationGroup::sequential()
            .with_animation(KeyframeAnimation::new("a").duration(Duration::from_millis(100)))
            .with_animation(KeyframeAnimation::new("b").duration(Duration::from_millis(200)));

        assert_eq!(group.total_duration(), Duration::from_millis(300));
    }
}

}

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
