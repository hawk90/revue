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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionPhase {
    /// Entering (appearing)
    Entering,
    /// Visible (animation complete or no animation)
    Visible,
    /// Leaving (disappearing)
    Leaving,
}
