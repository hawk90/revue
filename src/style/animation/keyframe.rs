//! CSS @keyframes style animation support

use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::{AnimationState, EasingFn};
use crate::style::animation::easing;

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
    pub duration: Duration,
    /// Delay before starting
    pub delay: Duration,
    /// Easing function
    easing: EasingFn,
    /// Start time
    start_time: Option<Instant>,
    /// Current state
    state: AnimationState,
    /// Number of iterations (0 = infinite)
    pub iterations: u32,
    /// Current iteration
    current_iteration: u32,
    /// Direction (normal, reverse, alternate)
    pub direction: AnimationDirection,
    /// Fill mode (forwards, backwards, both, none)
    pub fill_mode: AnimationFillMode,
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
