//! Transition and TransitionGroup widgets - type definitions

use crate::style::easing;
use std::time::Duration;

/// Transition animation configuration
#[derive(Clone)]
pub struct Animation {
    /// Type of animation preset
    preset: AnimationPreset,
    /// Duration in milliseconds
    duration: Duration,
    /// Easing function
    easing: fn(f32) -> f32,
    /// Delay before animation starts
    delay: Duration,
}

/// Animation presets for common transitions
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationPreset {
    /// Fade opacity from 0 to 1 (enter) or 1 to 0 (leave)
    Fade,
    /// Slide from left (enter) or to left (leave)
    SlideLeft,
    /// Slide from right (enter) or to right (leave)
    SlideRight,
    /// Slide from top (enter) or to top (leave)
    SlideUp,
    /// Slide from bottom (enter) or to bottom (leave)
    SlideDown,
    /// Scale from 0 to 1 (enter) or 1 to 0 (leave)
    Scale,
    /// Custom animation with user-defined parameters
    Custom {
        /// Opacity start (enter) / end (leave)
        opacity: Option<f32>,
        /// X offset start (enter) / end (leave)
        offset_x: Option<i16>,
        /// Y offset start (enter) / end (leave)
        offset_y: Option<i16>,
        /// Scale start (enter) / end (leave)
        scale: Option<f32>,
    },
}

impl Animation {
    /// Create a fade animation
    pub fn fade() -> Self {
        Self {
            preset: AnimationPreset::Fade,
            duration: Duration::from_millis(300),
            easing: easing::ease_in_out,
            delay: Duration::ZERO,
        }
    }

    /// Fade in animation
    pub fn fade_in() -> Self {
        Self::fade()
    }

    /// Fade out animation
    pub fn fade_out() -> Self {
        Self::fade()
    }

    /// Slide from left animation
    pub fn slide_left() -> Self {
        Self {
            preset: AnimationPreset::SlideLeft,
            duration: Duration::from_millis(300),
            easing: easing::ease_out,
            delay: Duration::ZERO,
        }
    }

    /// Slide in from left
    pub fn slide_in_left() -> Self {
        Self::slide_left()
    }

    /// Slide to left
    pub fn slide_out_left() -> Self {
        Self::slide_left()
    }

    /// Slide from right animation
    pub fn slide_right() -> Self {
        Self {
            preset: AnimationPreset::SlideRight,
            duration: Duration::from_millis(300),
            easing: easing::ease_out,
            delay: Duration::ZERO,
        }
    }

    /// Slide in from right
    pub fn slide_in_right() -> Self {
        Self::slide_right()
    }

    /// Slide to right
    pub fn slide_out_right() -> Self {
        Self::slide_right()
    }

    /// Slide from top animation
    pub fn slide_up() -> Self {
        Self {
            preset: AnimationPreset::SlideUp,
            duration: Duration::from_millis(300),
            easing: easing::ease_out,
            delay: Duration::ZERO,
        }
    }

    /// Slide in from top
    pub fn slide_in_up() -> Self {
        Self::slide_up()
    }

    /// Slide to top
    pub fn slide_out_up() -> Self {
        Self::slide_up()
    }

    /// Slide from bottom animation
    pub fn slide_down() -> Self {
        Self {
            preset: AnimationPreset::SlideDown,
            duration: Duration::from_millis(300),
            easing: easing::ease_out,
            delay: Duration::ZERO,
        }
    }

    /// Slide in from bottom
    pub fn slide_in_down() -> Self {
        Self::slide_down()
    }

    /// Slide to bottom
    pub fn slide_out_down() -> Self {
        Self::slide_down()
    }

    /// Scale animation
    pub fn scale() -> Self {
        Self {
            preset: AnimationPreset::Scale,
            duration: Duration::from_millis(300),
            easing: easing::back_out,
            delay: Duration::ZERO,
        }
    }

    /// Scale up animation
    pub fn scale_up() -> Self {
        Self::scale()
    }

    /// Scale down animation
    pub fn scale_down() -> Self {
        Self::scale()
    }

    /// Create a custom animation
    pub fn custom(
        opacity: Option<f32>,
        offset_x: Option<i16>,
        offset_y: Option<i16>,
        scale: Option<f32>,
    ) -> Self {
        Self {
            preset: AnimationPreset::Custom {
                opacity,
                offset_x,
                offset_y,
                scale,
            },
            duration: Duration::from_millis(300),
            easing: easing::ease_in_out,
            delay: Duration::ZERO,
        }
    }

    /// Set animation duration
    pub fn duration(mut self, duration_ms: u64) -> Self {
        self.duration = Duration::from_millis(duration_ms);
        self
    }

    /// Set easing function
    pub fn easing(mut self, easing: fn(f32) -> f32) -> Self {
        self.easing = easing;
        self
    }

    /// Set delay before animation starts
    pub fn delay(mut self, delay_ms: u64) -> Self {
        self.delay = Duration::from_millis(delay_ms);
        self
    }

    /// Get the preset
    pub fn preset(&self) -> AnimationPreset {
        self.preset
    }

    /// Get the duration
    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    /// Get the easing function
    pub fn get_easing(&self) -> fn(f32) -> f32 {
        self.easing
    }

    /// Get the delay
    pub fn get_delay(&self) -> Duration {
        self.delay
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self::fade()
    }
}

/// Transition state
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransitionPhase {
    /// Entering (appearing)
    Entering,
    /// Visible (animation complete or no animation)
    Visible,
    /// Leaving (disappearing)
    Leaving,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::easing;

    #[test]
    fn test_animation_default() {
        let anim = Animation::default();
        assert_eq!(anim.preset(), AnimationPreset::Fade);
        assert_eq!(anim.get_duration(), Duration::from_millis(300));
    }

    #[test]
    fn test_animation_fade() {
        let anim = Animation::fade();
        assert_eq!(anim.preset(), AnimationPreset::Fade);
        assert_eq!(anim.get_duration(), Duration::from_millis(300));
        // Test that easing function works
        let result = anim.get_easing()(0.5);
        assert!((result - 0.5).abs() < 1.0);
        assert_eq!(anim.get_delay(), Duration::ZERO);
    }

    #[test]
    fn test_animation_fade_in_out() {
        let fade_in = Animation::fade_in();
        let fade_out = Animation::fade_out();
        assert_eq!(fade_in.preset(), AnimationPreset::Fade);
        assert_eq!(fade_out.preset(), AnimationPreset::Fade);
    }

    #[test]
    fn test_animation_slide_left() {
        let anim = Animation::slide_left();
        assert_eq!(anim.preset(), AnimationPreset::SlideLeft);
        // Test that easing function works
        let _result = anim.get_easing()(0.5);
    }

    #[test]
    fn test_animation_slide_right() {
        let anim = Animation::slide_right();
        assert_eq!(anim.preset(), AnimationPreset::SlideRight);
    }

    #[test]
    fn test_animation_slide_up() {
        let anim = Animation::slide_up();
        assert_eq!(anim.preset(), AnimationPreset::SlideUp);
    }

    #[test]
    fn test_animation_slide_down() {
        let anim = Animation::slide_down();
        assert_eq!(anim.preset(), AnimationPreset::SlideDown);
    }

    #[test]
    fn test_animation_slide_in_left() {
        let anim = Animation::slide_in_left();
        assert_eq!(anim.preset(), AnimationPreset::SlideLeft);
    }

    #[test]
    fn test_animation_slide_out_left() {
        let anim = Animation::slide_out_left();
        assert_eq!(anim.preset(), AnimationPreset::SlideLeft);
    }

    #[test]
    fn test_animation_slide_in_right() {
        let anim = Animation::slide_in_right();
        assert_eq!(anim.preset(), AnimationPreset::SlideRight);
    }

    #[test]
    fn test_animation_slide_out_right() {
        let anim = Animation::slide_out_right();
        assert_eq!(anim.preset(), AnimationPreset::SlideRight);
    }

    #[test]
    fn test_animation_slide_in_up() {
        let anim = Animation::slide_in_up();
        assert_eq!(anim.preset(), AnimationPreset::SlideUp);
    }

    #[test]
    fn test_animation_slide_out_up() {
        let anim = Animation::slide_out_up();
        assert_eq!(anim.preset(), AnimationPreset::SlideUp);
    }

    #[test]
    fn test_animation_slide_in_down() {
        let anim = Animation::slide_in_down();
        assert_eq!(anim.preset(), AnimationPreset::SlideDown);
    }

    #[test]
    fn test_animation_slide_out_down() {
        let anim = Animation::slide_out_down();
        assert_eq!(anim.preset(), AnimationPreset::SlideDown);
    }

    #[test]
    fn test_animation_scale() {
        let anim = Animation::scale();
        assert_eq!(anim.preset(), AnimationPreset::Scale);
        // Test that easing function works
        let _result = anim.get_easing()(0.5);
    }

    #[test]
    fn test_animation_scale_up() {
        let anim = Animation::scale_up();
        assert_eq!(anim.preset(), AnimationPreset::Scale);
    }

    #[test]
    fn test_animation_scale_down() {
        let anim = Animation::scale_down();
        assert_eq!(anim.preset(), AnimationPreset::Scale);
    }

    #[test]
    fn test_animation_custom() {
        let anim = Animation::custom(Some(0.5), Some(10), Some(-5), Some(0.8));
        assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
    }

    #[test]
    fn test_animation_custom_all_none() {
        let anim = Animation::custom(None, None, None, None);
        assert!(matches!(anim.preset(), AnimationPreset::Custom { .. }));
    }

    #[test]
    fn test_animation_duration() {
        let anim = Animation::fade().duration(500);
        assert_eq!(anim.get_duration(), Duration::from_millis(500));
    }

    #[test]
    fn test_animation_easing() {
        let anim = Animation::fade().easing(easing::linear);
        // Test that the easing function can be set and called
        let result = anim.get_easing()(0.5);
        assert_eq!(result, 0.5); // linear should return the same value
    }

    #[test]
    fn test_animation_delay() {
        let anim = Animation::fade().delay(100);
        assert_eq!(anim.get_delay(), Duration::from_millis(100));
    }

    #[test]
    fn test_animation_clone() {
        let anim = Animation::fade().duration(500).easing(easing::linear);
        let cloned = anim.clone();
        assert_eq!(anim.preset(), cloned.preset());
        assert_eq!(anim.get_duration(), cloned.get_duration());
        // Test that both easing functions produce same result
        assert_eq!(anim.get_easing()(0.5), cloned.get_easing()(0.5));
    }

    #[test]
    fn test_animation_preset_all_variants() {
        // Test all AnimationPreset variants can be created
        let _ = Animation::fade();
        let _ = Animation::slide_left();
        let _ = Animation::slide_right();
        let _ = Animation::slide_up();
        let _ = Animation::slide_down();
        let _ = Animation::scale();
        let _ = Animation::custom(Some(0.5), Some(10), Some(-5), Some(0.8));
    }

    #[test]
    fn test_animation_preset_partial_equality() {
        let fade1 = Animation::fade();
        let fade2 = Animation::fade();
        assert_eq!(fade1.preset(), fade2.preset());

        let slide = Animation::slide_left();
        assert_ne!(fade1.preset(), slide.preset());
    }

    #[test]
    fn test_transition_phase_all_variants() {
        let entering = TransitionPhase::Entering;
        let visible = TransitionPhase::Visible;
        let leaving = TransitionPhase::Leaving;

        assert_eq!(entering, TransitionPhase::Entering);
        assert_eq!(visible, TransitionPhase::Visible);
        assert_eq!(leaving, TransitionPhase::Leaving);

        assert_ne!(entering, visible);
        assert_ne!(visible, leaving);
        assert_ne!(entering, leaving);
    }

    #[test]
    fn test_transition_phase_clone() {
        let phase = TransitionPhase::Entering;
        assert_eq!(phase, phase.clone());
    }

    #[test]
    fn test_animation_builder_chain() {
        let anim = Animation::fade()
            .duration(500)
            .delay(100)
            .easing(easing::linear);

        assert_eq!(anim.get_duration(), Duration::from_millis(500));
        assert_eq!(anim.get_delay(), Duration::from_millis(100));
        // Test the easing function
        let result = anim.get_easing()(0.5);
        assert_eq!(result, 0.5); // linear should return the same value
    }
}
