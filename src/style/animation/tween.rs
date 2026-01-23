//! Tween animation support

use std::time::{Duration, Instant};

use super::{AnimationState, EasingFn};
use crate::style::animation::easing;

/// A tween animation between two values
#[derive(Clone)]
pub struct Tween {
    /// Starting value
    pub from: f32,
    /// Ending value
    pub to: f32,
    /// Animation duration
    pub duration: Duration,
    easing: EasingFn,
    start_time: Option<Instant>,
    pause_time: Option<Instant>,
    state: AnimationState,
    /// Delay before animation starts
    pub delay: Duration,
    /// Number of times to repeat (0 = no repeat)
    pub repeat: u32,
    repeat_count: u32,
    /// Enable reverse (ping-pong) mode
    pub reverse: bool,
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
