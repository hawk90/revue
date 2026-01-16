//! Animation system with easing and tweening

pub mod easing;

use std::time::{Duration, Instant};

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

/// A tween animation between two values
#[derive(Clone)]
pub struct Tween {
    from: f32,
    to: f32,
    duration: Duration,
    easing: EasingFn,
    start_time: Option<Instant>,
    pause_time: Option<Instant>,
    state: AnimationState,
    delay: Duration,
    repeat: u32,
    repeat_count: u32,
    reverse: bool,
    current_direction: bool, // false = forward, true = backward
}

impl Tween {
    /// Create a new tween
    pub fn new(from: f32, to: f32, duration: Duration) -> Self {
        Self {
            from,
            to,
            duration,
            easing: easing::linear,
            start_time: None,
            pause_time: None,
            state: AnimationState::Pending,
            delay: Duration::ZERO,
            repeat: 0,
            repeat_count: 0,
            reverse: false,
            current_direction: false,
        }
    }

    /// Set easing function
    pub fn easing(mut self, easing: EasingFn) -> Self {
        self.easing = easing;
        self
    }

    /// Set delay before animation starts
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Set number of times to repeat (0 = no repeat)
    pub fn repeat(mut self, count: u32) -> Self {
        self.repeat = count;
        self
    }

    /// Enable reverse (ping-pong) mode
    pub fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    /// Start the animation
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.state = AnimationState::Running;
        self.repeat_count = 0;
        self.current_direction = false;
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        if self.state == AnimationState::Running {
            self.pause_time = Some(Instant::now());
            self.state = AnimationState::Paused;
        }
    }

    /// Resume the animation
    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused {
            if let (Some(start), Some(pause)) = (self.start_time, self.pause_time) {
                // Calculate how long the animation ran before pause
                let elapsed_before_pause = pause.duration_since(start);
                // Set new start time so elapsed time matches what it was at pause
                self.start_time = Some(Instant::now() - elapsed_before_pause);
            }
            self.pause_time = None;
            self.state = AnimationState::Running;
        }
    }

    /// Reset the animation
    pub fn reset(&mut self) {
        self.start_time = None;
        self.pause_time = None;
        self.state = AnimationState::Pending;
        self.repeat_count = 0;
        self.current_direction = false;
    }

    /// Get current animation state
    pub fn state(&self) -> AnimationState {
        self.state
    }

    /// Check if animation is running
    pub fn is_running(&self) -> bool {
        self.state == AnimationState::Running
    }

    /// Check if animation is completed
    pub fn is_completed(&self) -> bool {
        self.state == AnimationState::Completed
    }

    /// Update and get current value
    pub fn value(&mut self) -> f32 {
        // Pending state should return initial value, not final
        if self.state == AnimationState::Pending {
            return self.from;
        }

        // Paused or Completed returns value based on direction
        if self.state != AnimationState::Running {
            return if self.current_direction {
                self.from
            } else {
                self.to
            };
        }

        let Some(start) = self.start_time else {
            return self.from;
        };

        let elapsed = start.elapsed();

        // Handle delay
        if elapsed < self.delay {
            return self.from;
        }

        let elapsed_after_delay = elapsed - self.delay;
        let progress = elapsed_after_delay.as_secs_f32() / self.duration.as_secs_f32();

        if progress >= 1.0 {
            // Check for repeat
            if self.repeat > 0 && self.repeat_count < self.repeat {
                self.repeat_count += 1;
                self.start_time = Some(Instant::now());

                if self.reverse {
                    self.current_direction = !self.current_direction;
                }

                return if self.current_direction {
                    self.to
                } else {
                    self.from
                };
            }

            self.state = AnimationState::Completed;
            return if self.current_direction {
                self.from
            } else {
                self.to
            };
        }

        let eased = (self.easing)(progress);

        if self.current_direction {
            self.to + (self.from - self.to) * eased
        } else {
            self.from + (self.to - self.from) * eased
        }
    }

    /// Get progress (0.0 - 1.0)
    pub fn progress(&self) -> f32 {
        let Some(start) = self.start_time else {
            return 0.0;
        };

        let elapsed = start.elapsed();
        if elapsed < self.delay {
            return 0.0;
        }

        let elapsed_after_delay = elapsed - self.delay;
        (elapsed_after_delay.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0)
    }
}

impl Default for Tween {
    fn default() -> Self {
        Self::new(0.0, 1.0, Duration::from_millis(300))
    }
}

/// Animation preset
#[derive(Clone)]
pub struct Animation {
    /// Animation name
    pub name: String,
    /// Keyframes
    pub tweens: Vec<Tween>,
}

impl Animation {
    /// Create a new animation
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tweens: Vec::new(),
        }
    }

    /// Add a tween
    pub fn tween(mut self, tween: Tween) -> Self {
        self.tweens.push(tween);
        self
    }

    /// Start all tweens
    pub fn start(&mut self) {
        for tween in &mut self.tweens {
            tween.start();
        }
    }

    /// Pause all tweens
    pub fn pause(&mut self) {
        for tween in &mut self.tweens {
            tween.pause();
        }
    }

    /// Resume all tweens
    pub fn resume(&mut self) {
        for tween in &mut self.tweens {
            tween.resume();
        }
    }

    /// Reset all tweens
    pub fn reset(&mut self) {
        for tween in &mut self.tweens {
            tween.reset();
        }
    }

    /// Check if all tweens are completed
    pub fn is_completed(&self) -> bool {
        self.tweens.iter().all(|t| t.is_completed())
    }
}

/// Common animation presets
pub struct Animations;

impl Animations {
    /// Fade in animation
    pub fn fade_in(duration: Duration) -> Tween {
        Tween::new(0.0, 1.0, duration).easing(easing::ease_out)
    }

    /// Fade out animation
    pub fn fade_out(duration: Duration) -> Tween {
        Tween::new(1.0, 0.0, duration).easing(easing::ease_in)
    }

    /// Slide in from left
    pub fn slide_in_left(distance: f32, duration: Duration) -> Tween {
        Tween::new(-distance, 0.0, duration).easing(easing::ease_out_cubic)
    }

    /// Slide in from right
    pub fn slide_in_right(distance: f32, duration: Duration) -> Tween {
        Tween::new(distance, 0.0, duration).easing(easing::ease_out_cubic)
    }

    /// Scale up
    pub fn scale_up(duration: Duration) -> Tween {
        Tween::new(0.0, 1.0, duration).easing(easing::back_out)
    }

    /// Bounce
    pub fn bounce(duration: Duration) -> Tween {
        Tween::new(0.0, 1.0, duration).easing(easing::bounce_out)
    }

    /// Pulse (repeating scale)
    pub fn pulse(duration: Duration) -> Tween {
        Tween::new(1.0, 1.2, duration)
            .easing(easing::ease_in_out)
            .reverse(true)
            .repeat(u32::MAX)
    }
}

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

// =============================================================================
// CSS @keyframes Animation
// =============================================================================

/// A single keyframe in a CSS-style animation
#[derive(Clone, Debug, Default)]
pub struct CssKeyframe {
    /// Percentage (0-100)
    pub percent: u8,
    /// Property values at this keyframe
    pub properties: HashMap<String, f32>,
}

impl CssKeyframe {
    /// Create a new keyframe at the given percentage
    pub fn new(percent: u8) -> Self {
        Self {
            percent: percent.min(100),
            properties: HashMap::new(),
        }
    }

    /// Set a property value
    pub fn set(mut self, property: &str, value: f32) -> Self {
        self.properties.insert(property.to_string(), value);
        self
    }

    /// Get a property value
    pub fn get(&self, property: &str) -> Option<f32> {
        self.properties.get(property).copied()
    }
}

use std::collections::HashMap;

/// CSS @keyframes style animation
///
/// Supports percentage-based keyframes with property interpolation.
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::animation::KeyframeAnimation;
/// use std::time::Duration;
///
/// let mut anim = KeyframeAnimation::new("fade-slide")
///     .keyframe(0, |kf| kf.set("opacity", 0.0).set("x", -20.0))
///     .keyframe(50, |kf| kf.set("opacity", 1.0).set("x", 10.0))
///     .keyframe(100, |kf| kf.set("opacity", 1.0).set("x", 0.0))
///     .duration(Duration::from_millis(500))
///     .easing(easing::ease_out);
///
/// anim.start();
/// // In render loop:
/// let opacity = anim.get("opacity");
/// let x = anim.get("x");
/// ```
#[derive(Clone)]
pub struct KeyframeAnimation {
    /// Animation name
    name: String,
    /// Keyframes sorted by percentage
    keyframes: Vec<CssKeyframe>,
    /// Total duration
    duration: Duration,
    /// Delay before starting
    delay: Duration,
    /// Easing function
    easing: EasingFn,
    /// Start time
    start_time: Option<Instant>,
    /// Current state
    state: AnimationState,
    /// Number of iterations (0 = infinite)
    iterations: u32,
    /// Current iteration
    current_iteration: u32,
    /// Direction (normal, reverse, alternate)
    direction: AnimationDirection,
    /// Fill mode (forwards, backwards, both, none)
    fill_mode: AnimationFillMode,
}

/// Animation direction
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AnimationDirection {
    /// Normal direction (0% to 100%)
    #[default]
    Normal,
    /// Reverse direction (100% to 0%)
    Reverse,
    /// Alternate direction each iteration
    Alternate,
    /// Alternate, starting in reverse
    AlternateReverse,
}

/// Animation fill mode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AnimationFillMode {
    /// No fill - returns to initial state
    #[default]
    None,
    /// Keep final values after animation ends
    Forwards,
    /// Apply initial values before animation starts (during delay)
    Backwards,
    /// Both forwards and backwards
    Both,
}

impl KeyframeAnimation {
    /// Create a new keyframe animation
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            keyframes: Vec::new(),
            duration: Duration::from_millis(300),
            delay: Duration::ZERO,
            easing: easing::linear,
            start_time: None,
            state: AnimationState::Pending,
            iterations: 1,
            current_iteration: 0,
            direction: AnimationDirection::Normal,
            fill_mode: AnimationFillMode::None,
        }
    }

    /// Add a keyframe at the given percentage
    pub fn keyframe(mut self, percent: u8, f: impl FnOnce(CssKeyframe) -> CssKeyframe) -> Self {
        let kf = f(CssKeyframe::new(percent));
        self.keyframes.push(kf);
        self.keyframes.sort_by_key(|k| k.percent);
        self
    }

    /// Set duration
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Set delay
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Set easing function
    pub fn easing(mut self, easing: EasingFn) -> Self {
        self.easing = easing;
        self
    }

    /// Set number of iterations (0 = infinite)
    pub fn iterations(mut self, n: u32) -> Self {
        self.iterations = n;
        self
    }

    /// Set to loop infinitely
    pub fn infinite(mut self) -> Self {
        self.iterations = 0;
        self
    }

    /// Set direction
    pub fn direction(mut self, direction: AnimationDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set fill mode
    pub fn fill_mode(mut self, fill_mode: AnimationFillMode) -> Self {
        self.fill_mode = fill_mode;
        self
    }

    /// Get animation name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Start the animation
    pub fn start(&mut self) {
        // Check reduced motion preference
        if crate::utils::prefers_reduced_motion() {
            // Skip to end state immediately
            self.state = AnimationState::Completed;
            self.current_iteration = self.iterations.max(1);
            return;
        }

        self.start_time = Some(Instant::now());
        self.state = AnimationState::Running;
        self.current_iteration = 0;
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        if self.state == AnimationState::Running {
            self.state = AnimationState::Paused;
        }
    }

    /// Resume the animation
    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused {
            self.state = AnimationState::Running;
        }
    }

    /// Reset the animation
    pub fn reset(&mut self) {
        self.start_time = None;
        self.state = AnimationState::Pending;
        self.current_iteration = 0;
    }

    /// Check if animation is running
    pub fn is_running(&self) -> bool {
        self.state == AnimationState::Running
    }

    /// Check if animation is completed
    pub fn is_completed(&self) -> bool {
        self.state == AnimationState::Completed
    }

    /// Get current state
    pub fn state(&self) -> AnimationState {
        self.state
    }

    /// Get current progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        let Some(start) = self.start_time else {
            return 0.0;
        };

        if self.state != AnimationState::Running {
            return if self.is_completed() { 1.0 } else { 0.0 };
        }

        let elapsed = start.elapsed();
        if elapsed < self.delay {
            return 0.0;
        }

        let elapsed_after_delay = elapsed - self.delay;
        (elapsed_after_delay.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0)
    }

    /// Get current value for a property
    pub fn get(&mut self, property: &str) -> f32 {
        self.update();

        if self.keyframes.is_empty() {
            return 0.0;
        }

        let progress = self.progress();

        // Handle fill modes
        if self.state == AnimationState::Pending
            && matches!(
                self.fill_mode,
                AnimationFillMode::Backwards | AnimationFillMode::Both
            )
        {
            // Return initial value during delay
            return self
                .keyframes
                .first()
                .and_then(|kf| kf.get(property))
                .unwrap_or(0.0);
        }

        if self.state == AnimationState::Completed
            && matches!(
                self.fill_mode,
                AnimationFillMode::Forwards | AnimationFillMode::Both
            )
        {
            // Return final value after completion
            return self
                .keyframes
                .last()
                .and_then(|kf| kf.get(property))
                .unwrap_or(0.0);
        }

        // Calculate direction for this iteration
        let is_reverse = match self.direction {
            AnimationDirection::Normal => false,
            AnimationDirection::Reverse => true,
            AnimationDirection::Alternate => self.current_iteration % 2 == 1,
            AnimationDirection::AlternateReverse => self.current_iteration.is_multiple_of(2),
        };

        // Apply easing and direction
        let t = (self.easing)(progress);
        let percent = if is_reverse { 1.0 - t } else { t } * 100.0;

        // Find surrounding keyframes
        let mut prev_kf = &self.keyframes[0];
        let mut next_kf = &self.keyframes[self.keyframes.len() - 1];

        for kf in &self.keyframes {
            if (kf.percent as f32) <= percent {
                prev_kf = kf;
            }
            if (kf.percent as f32) >= percent {
                next_kf = kf;
                break;
            }
        }

        // Get values from keyframes
        let prev_val = prev_kf.get(property).unwrap_or(0.0);
        let next_val = next_kf.get(property).unwrap_or(prev_val);

        // Interpolate
        if prev_kf.percent == next_kf.percent {
            return prev_val;
        }

        let local_t =
            (percent - prev_kf.percent as f32) / (next_kf.percent as f32 - prev_kf.percent as f32);
        prev_val + (next_val - prev_val) * local_t
    }

    /// Update animation state
    fn update(&mut self) {
        if self.state != AnimationState::Running {
            return;
        }

        let Some(start) = self.start_time else {
            return;
        };

        let elapsed = start.elapsed();
        if elapsed < self.delay {
            return;
        }

        let elapsed_after_delay = elapsed - self.delay;
        let iteration_progress = elapsed_after_delay.as_secs_f32() / self.duration.as_secs_f32();

        if iteration_progress >= 1.0 {
            self.current_iteration += 1;

            if self.iterations > 0 && self.current_iteration >= self.iterations {
                self.state = AnimationState::Completed;
            } else {
                // Start next iteration
                self.start_time = Some(Instant::now());
            }
        }
    }
}

// =============================================================================
// Stagger Animation
// =============================================================================

/// Staggered animation helper for animating multiple elements with delays
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::animation::Stagger;
/// use std::time::Duration;
///
/// let stagger = Stagger::new(5, Duration::from_millis(50));
///
/// for i in 0..5 {
///     let delay = stagger.delay_for(i);
///     // Create animation with this delay
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Stagger {
    /// Number of items
    count: usize,
    /// Delay between each item
    delay: Duration,
    /// Starting delay
    start_delay: Duration,
    /// Easing for delay distribution
    easing: Option<EasingFn>,
}

impl Stagger {
    /// Create a new stagger configuration
    pub fn new(count: usize, delay: Duration) -> Self {
        Self {
            count,
            delay,
            start_delay: Duration::ZERO,
            easing: None,
        }
    }

    /// Set starting delay
    pub fn start_delay(mut self, delay: Duration) -> Self {
        self.start_delay = delay;
        self
    }

    /// Set easing for delay distribution
    ///
    /// With easing, earlier items have shorter delays and later items have longer ones
    /// (or vice versa depending on the easing function).
    pub fn easing(mut self, easing: EasingFn) -> Self {
        self.easing = Some(easing);
        self
    }

    /// Get delay for item at index
    pub fn delay_for(&self, index: usize) -> Duration {
        if self.count == 0 {
            return self.start_delay;
        }

        let base_delay = self.delay.as_secs_f64() * index as f64;

        let adjusted = match self.easing {
            Some(ease) => {
                let t = index as f32 / (self.count - 1).max(1) as f32;
                let eased = ease(t);
                self.delay.as_secs_f64() * (self.count - 1) as f64 * eased as f64
            }
            None => base_delay,
        };

        self.start_delay + Duration::from_secs_f64(adjusted)
    }

    /// Get total duration for all items
    pub fn total_duration(&self, item_duration: Duration) -> Duration {
        if self.count == 0 {
            return Duration::ZERO;
        }

        self.delay_for(self.count - 1) + item_duration
    }

    /// Create animations for each item
    pub fn apply<F>(&self, mut create_animation: F) -> Vec<KeyframeAnimation>
    where
        F: FnMut(usize) -> KeyframeAnimation,
    {
        (0..self.count)
            .map(|i| {
                let mut anim = create_animation(i);
                anim.delay = self.delay_for(i);
                anim
            })
            .collect()
    }
}

// =============================================================================
// Animation Group
// =============================================================================

/// Mode for animation group execution
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GroupMode {
    /// Run all animations simultaneously
    #[default]
    Parallel,
    /// Run animations one after another
    Sequential,
}

/// Group of animations that can run in parallel or sequence
#[derive(Clone)]
pub struct AnimationGroup {
    animations: Vec<KeyframeAnimation>,
    mode: GroupMode,
    state: AnimationState,
    start_time: Option<Instant>,
}

impl AnimationGroup {
    /// Create a parallel animation group
    pub fn parallel() -> Self {
        Self {
            animations: Vec::new(),
            mode: GroupMode::Parallel,
            state: AnimationState::Pending,
            start_time: None,
        }
    }

    /// Create a sequential animation group
    pub fn sequential() -> Self {
        Self {
            animations: Vec::new(),
            mode: GroupMode::Sequential,
            state: AnimationState::Pending,
            start_time: None,
        }
    }

    /// Add an animation to the group
    pub fn with_animation(mut self, animation: KeyframeAnimation) -> Self {
        self.animations.push(animation);
        self
    }

    /// Get total duration of the group
    pub fn total_duration(&self) -> Duration {
        match self.mode {
            GroupMode::Parallel => self
                .animations
                .iter()
                .map(|a| a.delay + a.duration)
                .max()
                .unwrap_or(Duration::ZERO),
            GroupMode::Sequential => self.animations.iter().map(|a| a.delay + a.duration).sum(),
        }
    }

    /// Start all animations
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.state = AnimationState::Running;

        match self.mode {
            GroupMode::Parallel => {
                for anim in &mut self.animations {
                    anim.start();
                }
            }
            GroupMode::Sequential => {
                // Start only the first animation; others will be started as they complete
                if let Some(first) = self.animations.first_mut() {
                    first.start();
                }
            }
        }
    }

    /// Update all animations
    pub fn update(&mut self) {
        if self.state != AnimationState::Running {
            return;
        }

        match self.mode {
            GroupMode::Parallel => {
                // Check if all are completed
                if self.animations.iter().all(|a| a.is_completed()) {
                    self.state = AnimationState::Completed;
                }
            }
            GroupMode::Sequential => {
                // Find current running animation
                let mut should_start_next = false;
                let mut next_idx = 0;

                for (i, anim) in self.animations.iter().enumerate() {
                    if anim.is_running() {
                        break;
                    }
                    if anim.is_completed()
                        && i + 1 < self.animations.len()
                        && !self.animations[i + 1].is_running()
                        && !self.animations[i + 1].is_completed()
                    {
                        should_start_next = true;
                        next_idx = i + 1;
                    }
                }

                if should_start_next {
                    self.animations[next_idx].start();
                }

                // Check if all are completed
                if self.animations.iter().all(|a| a.is_completed()) {
                    self.state = AnimationState::Completed;
                }
            }
        }
    }

    /// Check if all animations are completed
    pub fn is_completed(&self) -> bool {
        self.state == AnimationState::Completed
    }

    /// Get mutable reference to animations
    pub fn animations_mut(&mut self) -> &mut [KeyframeAnimation] {
        &mut self.animations
    }
}

// =============================================================================
// Animation Choreographer
// =============================================================================

/// Manages multiple animation groups and coordinates complex animation sequences
///
/// # Example
///
/// ```rust,ignore
/// use revue::style::animation::{Choreographer, KeyframeAnimation, AnimationGroup};
/// use std::time::Duration;
///
/// let mut choreo = Choreographer::new();
///
/// // Add a staggered entrance
/// choreo.add_staggered(
///     "list-items",
///     5,
///     Duration::from_millis(50),
///     |i| KeyframeAnimation::new(format!("item-{}", i))
///         .keyframe(0, |kf| kf.set("opacity", 0.0))
///         .keyframe(100, |kf| kf.set("opacity", 1.0))
///         .duration(Duration::from_millis(200))
/// );
///
/// choreo.start("list-items");
/// ```
pub struct Choreographer {
    groups: HashMap<String, AnimationGroup>,
    staggered: HashMap<String, Vec<KeyframeAnimation>>,
}

impl Default for Choreographer {
    fn default() -> Self {
        Self::new()
    }
}

impl Choreographer {
    /// Create a new choreographer
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            staggered: HashMap::new(),
        }
    }

    /// Add an animation group
    pub fn add_group(&mut self, name: impl Into<String>, group: AnimationGroup) {
        self.groups.insert(name.into(), group);
    }

    /// Add a staggered animation set
    pub fn add_staggered<F>(
        &mut self,
        name: impl Into<String>,
        count: usize,
        delay: Duration,
        create: F,
    ) where
        F: FnMut(usize) -> KeyframeAnimation,
    {
        let stagger = Stagger::new(count, delay);
        let animations = stagger.apply(create);
        self.staggered.insert(name.into(), animations);
    }

    /// Start a named animation group or staggered set
    pub fn start(&mut self, name: &str) {
        if let Some(group) = self.groups.get_mut(name) {
            group.start();
        }
        if let Some(anims) = self.staggered.get_mut(name) {
            for anim in anims {
                anim.start();
            }
        }
    }

    /// Update all animations
    pub fn update(&mut self) {
        for group in self.groups.values_mut() {
            group.update();
        }
    }

    /// Get a value from a staggered animation
    pub fn get_staggered(&mut self, name: &str, index: usize, property: &str) -> f32 {
        self.staggered
            .get_mut(name)
            .and_then(|anims| anims.get_mut(index))
            .map(|anim| anim.get(property))
            .unwrap_or(0.0)
    }

    /// Check if a named animation is completed
    pub fn is_completed(&self, name: &str) -> bool {
        if let Some(group) = self.groups.get(name) {
            return group.is_completed();
        }
        if let Some(anims) = self.staggered.get(name) {
            return anims.iter().all(|a| a.is_completed());
        }
        true
    }
}

// =============================================================================
// Animation Presets for Widgets
// =============================================================================

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
