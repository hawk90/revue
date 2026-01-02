//! Animation utilities for terminal UI
//!
//! Provides frame-based animation primitives that work well with terminal
//! render loops. Includes spring physics, keyframes, sequences, and timers.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::animation::{Spring, Keyframes, Timer};
//!
//! // Spring animation for smooth motion
//! let mut spring = Spring::new(0.0, 100.0);
//! loop {
//!     let value = spring.update(dt);
//!     if spring.is_settled() { break; }
//! }
//!
//! // Keyframe animation
//! let anim = Keyframes::new()
//!     .add(0.0, 0.0)
//!     .add(0.5, 100.0)
//!     .add(1.0, 50.0);
//! let value = anim.at(0.25);  // Interpolated between keyframes
//! ```

use std::time::{Duration, Instant};
use crate::utils::easing::Easing;

// ─────────────────────────────────────────────────────────────────────────────
// Interpolatable Trait
// ─────────────────────────────────────────────────────────────────────────────

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

// ─────────────────────────────────────────────────────────────────────────────
// Timer
// ─────────────────────────────────────────────────────────────────────────────

/// A simple timer for animation timing
#[derive(Clone, Debug)]
pub struct Timer {
    start: Option<Instant>,
    duration: Duration,
    elapsed_on_pause: Duration,
    paused: bool,
}

impl Timer {
    /// Create a new timer with given duration
    pub fn new(duration: Duration) -> Self {
        Self {
            start: None,
            duration,
            elapsed_on_pause: Duration::ZERO,
            paused: false,
        }
    }

    /// Create a timer from milliseconds
    pub fn from_millis(ms: u64) -> Self {
        Self::new(Duration::from_millis(ms))
    }

    /// Create a timer from seconds
    pub fn from_secs_f32(secs: f32) -> Self {
        Self::new(Duration::from_secs_f32(secs))
    }

    /// Start the timer
    pub fn start(&mut self) {
        self.start = Some(Instant::now());
        self.elapsed_on_pause = Duration::ZERO;
        self.paused = false;
    }

    /// Pause the timer
    pub fn pause(&mut self) {
        if !self.paused && self.start.is_some() {
            self.elapsed_on_pause = self.elapsed();
            self.paused = true;
        }
    }

    /// Resume the timer
    pub fn resume(&mut self) {
        if self.paused {
            self.start = Some(Instant::now() - self.elapsed_on_pause);
            self.paused = false;
        }
    }

    /// Reset the timer
    pub fn reset(&mut self) {
        self.start = None;
        self.elapsed_on_pause = Duration::ZERO;
        self.paused = false;
    }

    /// Restart the timer (reset and start)
    pub fn restart(&mut self) {
        self.reset();
        self.start();
    }

    /// Check if timer is running
    pub fn is_running(&self) -> bool {
        self.start.is_some() && !self.paused && !self.is_finished()
    }

    /// Check if timer has finished
    pub fn is_finished(&self) -> bool {
        self.elapsed() >= self.duration
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        match self.start {
            Some(start) if !self.paused => Instant::now() - start,
            _ => self.elapsed_on_pause,
        }
    }

    /// Get remaining time
    pub fn remaining(&self) -> Duration {
        self.duration.saturating_sub(self.elapsed())
    }

    /// Get progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        let elapsed = self.elapsed().as_secs_f64();
        let duration = self.duration.as_secs_f64();
        if duration == 0.0 {
            1.0
        } else {
            (elapsed / duration).clamp(0.0, 1.0)
        }
    }

    /// Get eased progress
    pub fn progress_eased(&self, easing: Easing) -> f64 {
        easing.ease(self.progress())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Spring Physics
// ─────────────────────────────────────────────────────────────────────────────

/// Spring physics simulation for smooth, natural motion
///
/// Uses a critically damped spring model for smooth animations
/// without oscillation (unless configured to bounce).
#[derive(Clone, Debug)]
pub struct Spring {
    /// Current value
    value: f64,
    /// Target value
    target: f64,
    /// Velocity
    velocity: f64,
    /// Stiffness (spring constant)
    stiffness: f64,
    /// Damping ratio (1.0 = critical damping)
    damping: f64,
    /// Mass
    mass: f64,
    /// Threshold for settling
    threshold: f64,
}

impl Spring {
    /// Create a new spring at initial value
    pub fn new(initial: f64, target: f64) -> Self {
        Self {
            value: initial,
            target,
            velocity: 0.0,
            stiffness: 180.0,
            damping: 12.0,
            mass: 1.0,
            threshold: 0.01,
        }
    }

    /// Create a spring starting at target (no animation)
    pub fn at(value: f64) -> Self {
        Self::new(value, value)
    }

    /// Set stiffness (higher = faster, snappier)
    pub fn stiffness(mut self, stiffness: f64) -> Self {
        self.stiffness = stiffness.max(0.1);
        self
    }

    /// Set damping ratio (1.0 = critical, <1 = bouncy, >1 = sluggish)
    pub fn damping(mut self, damping: f64) -> Self {
        self.damping = damping.max(0.1);
        self
    }

    /// Set mass (higher = slower, more momentum)
    pub fn mass(mut self, mass: f64) -> Self {
        self.mass = mass.max(0.01);
        self
    }

    /// Set settling threshold
    pub fn threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold.max(0.0001);
        self
    }

    /// Preset: snappy animation
    pub fn snappy() -> Self {
        Self::at(0.0).stiffness(400.0).damping(30.0)
    }

    /// Preset: gentle animation
    pub fn gentle() -> Self {
        Self::at(0.0).stiffness(100.0).damping(15.0)
    }

    /// Preset: bouncy animation
    pub fn bouncy() -> Self {
        Self::at(0.0).stiffness(200.0).damping(8.0)
    }

    /// Preset: slow animation
    pub fn slow() -> Self {
        Self::at(0.0).stiffness(50.0).damping(10.0)
    }

    /// Set target value
    pub fn set_target(&mut self, target: f64) {
        self.target = target;
    }

    /// Set value immediately (no animation)
    pub fn set_value(&mut self, value: f64) {
        self.value = value;
        self.velocity = 0.0;
    }

    /// Get current value
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get target value
    pub fn target(&self) -> f64 {
        self.target
    }

    /// Get velocity
    pub fn velocity(&self) -> f64 {
        self.velocity
    }

    /// Check if spring has settled (close to target with low velocity)
    pub fn is_settled(&self) -> bool {
        (self.value - self.target).abs() < self.threshold
            && self.velocity.abs() < self.threshold
    }

    /// Update spring simulation
    ///
    /// Call this every frame with the time delta.
    /// Returns the current value.
    pub fn update(&mut self, dt: f64) -> f64 {
        if self.is_settled() {
            self.value = self.target;
            self.velocity = 0.0;
            return self.value;
        }

        // Spring force: F = -k * x
        let displacement = self.value - self.target;
        let spring_force = -self.stiffness * displacement;

        // Damping force: F = -c * v
        let damping_force = -self.damping * self.velocity;

        // Acceleration: a = F / m
        let acceleration = (spring_force + damping_force) / self.mass;

        // Update velocity and position
        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;

        self.value
    }

    /// Update with fixed 60fps timestep
    pub fn tick(&mut self) -> f64 {
        self.update(1.0 / 60.0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Keyframe
// ─────────────────────────────────────────────────────────────────────────────

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

/// Keyframe-based animation
#[derive(Clone, Debug)]
pub struct Keyframes<T: Interpolatable> {
    keyframes: Vec<Keyframe<T>>,
}

impl<T: Interpolatable> Default for Keyframes<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Interpolatable> Keyframes<T> {
    /// Create new empty keyframes
    pub fn new() -> Self {
        Self {
            keyframes: Vec::new(),
        }
    }

    /// Add a keyframe
    pub fn add(mut self, time: f64, value: T) -> Self {
        self.keyframes.push(Keyframe::new(time, value));
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        self
    }

    /// Add a keyframe with easing
    pub fn add_eased(mut self, time: f64, value: T, easing: Easing) -> Self {
        self.keyframes.push(Keyframe::new(time, value).easing(easing));
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        self
    }

    /// Get number of keyframes
    pub fn len(&self) -> usize {
        self.keyframes.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.keyframes.is_empty()
    }

    /// Get value at time (0.0 to 1.0)
    pub fn at(&self, t: f64) -> Option<T> {
        if self.keyframes.is_empty() {
            return None;
        }

        let t = t.clamp(0.0, 1.0);

        // Find surrounding keyframes
        let mut prev = &self.keyframes[0];
        let mut next = &self.keyframes[self.keyframes.len() - 1];

        for kf in &self.keyframes {
            if kf.time <= t {
                prev = kf;
            }
            if kf.time >= t {
                next = kf;
                break;
            }
        }

        if prev.time >= next.time {
            return Some(prev.value.clone());
        }

        // Calculate local progress between keyframes
        let local_t = (t - prev.time) / (next.time - prev.time);
        let eased_t = next.easing.ease(local_t);

        Some(prev.value.lerp(&next.value, eased_t))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Animated Value
// ─────────────────────────────────────────────────────────────────────────────

/// A value that animates smoothly to targets
#[derive(Clone, Debug)]
pub struct AnimatedValue<T: Interpolatable> {
    /// Current value
    current: T,
    /// Start value of animation
    from: T,
    /// Target value
    to: T,
    /// Timer for animation
    timer: Timer,
    /// Easing function
    easing: Easing,
}

impl<T: Interpolatable> AnimatedValue<T> {
    /// Create a new animated value
    pub fn new(initial: T, duration: Duration) -> Self {
        Self {
            current: initial.clone(),
            from: initial.clone(),
            to: initial,
            timer: Timer::new(duration),
            easing: Easing::OutQuad,
        }
    }

    /// Set easing function
    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Animate to a new target
    pub fn animate_to(&mut self, target: T) {
        self.from = self.current.clone();
        self.to = target;
        self.timer.restart();
    }

    /// Set value immediately (no animation)
    pub fn set(&mut self, value: T) {
        self.current = value.clone();
        self.from = value.clone();
        self.to = value;
        self.timer.reset();
    }

    /// Get current value (and update animation)
    pub fn value(&mut self) -> &T {
        if self.timer.is_running() || !self.timer.is_finished() {
            let t = self.timer.progress_eased(self.easing);
            self.current = self.from.lerp(&self.to, t);
        }
        &self.current
    }

    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        self.timer.is_finished()
    }

    /// Get target value
    pub fn target(&self) -> &T {
        &self.to
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Sequence
// ─────────────────────────────────────────────────────────────────────────────

/// Animation step in a sequence
#[derive(Clone)]
pub struct SequenceStep {
    /// Duration of this step
    pub duration: Duration,
    /// Easing for this step
    pub easing: Easing,
    /// Target value (normalized 0.0 to 1.0)
    pub target: f64,
}

impl SequenceStep {
    /// Create a new step
    pub fn new(duration: Duration, target: f64) -> Self {
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

/// Sequential animation with multiple steps
#[derive(Clone)]
pub struct Sequence {
    steps: Vec<SequenceStep>,
    current_step: usize,
    timer: Timer,
    value: f64,
    started: bool,
    repeat: bool,
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

impl Sequence {
    /// Create a new sequence
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            current_step: 0,
            timer: Timer::new(Duration::ZERO),
            value: 0.0,
            started: false,
            repeat: false,
        }
    }

    /// Add a step
    pub fn then(mut self, duration: Duration, target: f64) -> Self {
        self.steps.push(SequenceStep::new(duration, target));
        self
    }

    /// Add a step with easing
    pub fn then_eased(mut self, duration: Duration, target: f64, easing: Easing) -> Self {
        self.steps.push(SequenceStep::new(duration, target).easing(easing));
        self
    }

    /// Add a delay (step that holds current value)
    pub fn delay(self, duration: Duration) -> Self {
        let current = self.steps.last().map(|s| s.target).unwrap_or(0.0);
        self.then(duration, current)
    }

    /// Enable looping
    pub fn repeat(mut self, repeat: bool) -> Self {
        self.repeat = repeat;
        self
    }

    /// Start the sequence
    pub fn start(&mut self) {
        self.current_step = 0;
        self.value = 0.0;
        self.started = true;
        if let Some(step) = self.steps.first() {
            self.timer = Timer::new(step.duration);
            self.timer.start();
        }
    }

    /// Reset the sequence
    pub fn reset(&mut self) {
        self.current_step = 0;
        self.value = 0.0;
        self.started = false;
        self.timer.reset();
    }

    /// Check if sequence is running
    pub fn is_running(&self) -> bool {
        self.started && self.current_step < self.steps.len()
    }

    /// Check if sequence is complete
    pub fn is_complete(&self) -> bool {
        self.started && self.current_step >= self.steps.len()
    }

    /// Get current value (and advance if needed)
    pub fn value(&mut self) -> f64 {
        if !self.started || self.steps.is_empty() {
            return self.value;
        }

        // Check if current step is complete
        while self.current_step < self.steps.len() && self.timer.is_finished() {
            // Move to next step
            self.value = self.steps[self.current_step].target;
            self.current_step += 1;

            if self.current_step < self.steps.len() {
                self.timer = Timer::new(self.steps[self.current_step].duration);
                self.timer.start();
            } else if self.repeat {
                // Loop back to start
                self.current_step = 0;
                self.timer = Timer::new(self.steps[0].duration);
                self.timer.start();
            }
        }

        // Interpolate current step
        if self.current_step < self.steps.len() {
            let step = &self.steps[self.current_step];
            let prev_value = if self.current_step == 0 {
                0.0
            } else {
                self.steps[self.current_step - 1].target
            };
            let t = self.timer.progress_eased(step.easing);
            self.value = prev_value + (step.target - prev_value) * t;
        }

        self.value
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Ticker
// ─────────────────────────────────────────────────────────────────────────────

/// Frame rate tracker and delta time calculator
#[derive(Clone, Debug)]
pub struct Ticker {
    last_tick: Option<Instant>,
    frame_count: u64,
    fps_update_time: Instant,
    fps: f64,
    target_fps: Option<f64>,
}

impl Default for Ticker {
    fn default() -> Self {
        Self::new()
    }
}

impl Ticker {
    /// Create a new ticker
    pub fn new() -> Self {
        Self {
            last_tick: None,
            frame_count: 0,
            fps_update_time: Instant::now(),
            fps: 0.0,
            target_fps: None,
        }
    }

    /// Create a ticker with target FPS
    pub fn with_target_fps(fps: f64) -> Self {
        Self {
            target_fps: Some(fps),
            ..Self::new()
        }
    }

    /// Tick and get delta time in seconds
    pub fn tick(&mut self) -> f64 {
        let now = Instant::now();
        let dt = match self.last_tick {
            Some(last) => (now - last).as_secs_f64(),
            None => 1.0 / 60.0, // Default to 60fps on first tick
        };
        self.last_tick = Some(now);
        self.frame_count += 1;

        // Update FPS every second
        let elapsed = now - self.fps_update_time;
        if elapsed >= Duration::from_secs(1) {
            self.fps = self.frame_count as f64 / elapsed.as_secs_f64();
            self.frame_count = 0;
            self.fps_update_time = now;
        }

        // Clamp to reasonable range
        dt.clamp(0.001, 0.1)
    }

    /// Get current FPS
    pub fn fps(&self) -> f64 {
        self.fps
    }

    /// Get frame duration for target FPS
    pub fn frame_duration(&self) -> Duration {
        match self.target_fps {
            Some(fps) => Duration::from_secs_f64(1.0 / fps),
            None => Duration::from_secs_f64(1.0 / 60.0),
        }
    }

    /// Get time since last tick
    pub fn elapsed_since_tick(&self) -> Duration {
        match self.last_tick {
            Some(last) => Instant::now() - last,
            None => Duration::ZERO,
        }
    }

    /// Check if enough time has passed for next frame
    pub fn should_render(&self) -> bool {
        match self.target_fps {
            Some(fps) => {
                let frame_duration = Duration::from_secs_f64(1.0 / fps);
                self.elapsed_since_tick() >= frame_duration
            }
            None => true,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Presets
// ─────────────────────────────────────────────────────────────────────────────

/// Common animation presets
pub mod presets {
    use super::*;

    /// Fade in animation (0.0 to 1.0)
    pub fn fade_in(duration_ms: u64) -> Sequence {
        Sequence::new()
            .then_eased(Duration::from_millis(duration_ms), 1.0, Easing::OutQuad)
    }

    /// Fade out animation (1.0 to 0.0)
    pub fn fade_out(duration_ms: u64) -> Sequence {
        Sequence::new()
            .then_eased(Duration::from_millis(duration_ms), 0.0, Easing::InQuad)
    }

    /// Pulse animation (repeating scale)
    pub fn pulse(duration_ms: u64) -> Sequence {
        Sequence::new()
            .then_eased(Duration::from_millis(duration_ms / 2), 1.0, Easing::OutQuad)
            .then_eased(Duration::from_millis(duration_ms / 2), 0.0, Easing::InQuad)
            .repeat(true)
    }

    /// Blink animation (on/off)
    pub fn blink(duration_ms: u64) -> Sequence {
        Sequence::new()
            .then(Duration::from_millis(duration_ms / 2), 1.0)
            .then(Duration::from_millis(duration_ms / 2), 0.0)
            .repeat(true)
    }

    /// Slide in from left (-1.0 to 0.0)
    pub fn slide_in_left(duration_ms: u64) -> Sequence {
        Sequence::new()
            .then_eased(Duration::from_millis(duration_ms), 0.0, Easing::OutCubic)
    }

    /// Bounce animation
    pub fn bounce(duration_ms: u64) -> Sequence {
        Sequence::new()
            .then_eased(Duration::from_millis(duration_ms), 1.0, Easing::OutBounce)
    }

    /// Elastic animation
    pub fn elastic(duration_ms: u64) -> Sequence {
        Sequence::new()
            .then_eased(Duration::from_millis(duration_ms), 1.0, Easing::OutElastic)
    }

    /// Typewriter effect (linear reveal)
    pub fn typewriter(total_chars: usize, chars_per_second: f64) -> Sequence {
        let duration = Duration::from_secs_f64(total_chars as f64 / chars_per_second);
        Sequence::new()
            .then(duration, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Timer Tests
    // =========================================================================

    #[test]
    fn test_timer_new() {
        let timer = Timer::new(Duration::from_secs(1));
        assert!(!timer.is_running());
        assert!(!timer.is_finished());
        assert_eq!(timer.progress(), 0.0);
    }

    #[test]
    fn test_timer_from_millis() {
        let timer = Timer::from_millis(500);
        assert_eq!(timer.duration, Duration::from_millis(500));
    }

    #[test]
    fn test_timer_start() {
        let mut timer = Timer::from_millis(1000);
        timer.start();
        assert!(timer.is_running());
    }

    #[test]
    fn test_timer_pause_resume() {
        let mut timer = Timer::from_millis(1000);
        timer.start();

        std::thread::sleep(Duration::from_millis(10));
        timer.pause();
        let elapsed_paused = timer.elapsed();

        std::thread::sleep(Duration::from_millis(50));
        let elapsed_still_paused = timer.elapsed();

        // Elapsed should not change while paused
        assert_eq!(elapsed_paused, elapsed_still_paused);

        timer.resume();
        assert!(timer.is_running());
    }

    #[test]
    fn test_timer_progress() {
        let timer = Timer::from_millis(0);
        assert_eq!(timer.progress(), 1.0); // Zero duration = finished
    }

    // =========================================================================
    // Spring Tests
    // =========================================================================

    #[test]
    fn test_spring_new() {
        let spring = Spring::new(0.0, 100.0);
        assert_eq!(spring.value(), 0.0);
        assert_eq!(spring.target(), 100.0);
    }

    #[test]
    fn test_spring_at() {
        let spring = Spring::at(50.0);
        assert_eq!(spring.value(), 50.0);
        assert_eq!(spring.target(), 50.0);
        assert!(spring.is_settled());
    }

    #[test]
    fn test_spring_update() {
        let mut spring = Spring::new(0.0, 100.0);
        let dt = 1.0 / 60.0;

        // Spring should move towards target
        let v1 = spring.update(dt);
        assert!(v1 > 0.0);

        // Keep updating
        for _ in 0..200 {
            spring.update(dt);
        }

        // Should eventually settle near target
        assert!(spring.is_settled());
        assert!((spring.value() - 100.0).abs() < 1.0);
    }

    #[test]
    fn test_spring_presets() {
        let snappy = Spring::snappy();
        let gentle = Spring::gentle();
        let bouncy = Spring::bouncy();

        assert!(snappy.stiffness > gentle.stiffness);
        assert!(bouncy.damping < snappy.damping);
    }

    #[test]
    fn test_spring_set_target() {
        let mut spring = Spring::at(0.0);
        spring.set_target(100.0);
        assert_eq!(spring.target(), 100.0);
        assert!(!spring.is_settled());
    }

    // =========================================================================
    // Keyframes Tests
    // =========================================================================

    #[test]
    fn test_keyframes_new() {
        let kf: Keyframes<f64> = Keyframes::new();
        assert!(kf.is_empty());
    }

    #[test]
    fn test_keyframes_add() {
        let kf = Keyframes::new()
            .add(0.0, 0.0)
            .add(1.0, 100.0);

        assert_eq!(kf.len(), 2);
    }

    #[test]
    fn test_keyframes_at() {
        let kf = Keyframes::new()
            .add(0.0, 0.0)
            .add(1.0, 100.0);

        assert_eq!(kf.at(0.0), Some(0.0));
        assert_eq!(kf.at(1.0), Some(100.0));

        let mid: f64 = kf.at(0.5).unwrap();
        assert!((mid - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_keyframes_three_points() {
        let kf = Keyframes::new()
            .add(0.0, 0.0)
            .add(0.5, 100.0)
            .add(1.0, 50.0);

        assert_eq!(kf.at(0.0), Some(0.0_f64));
        assert_eq!(kf.at(0.5), Some(100.0_f64));
        assert_eq!(kf.at(1.0), Some(50.0_f64));

        let mid1: f64 = kf.at(0.25).unwrap();
        assert!((mid1 - 50.0).abs() < 0.01);

        let mid2: f64 = kf.at(0.75).unwrap();
        assert!((mid2 - 75.0).abs() < 0.01);
    }

    #[test]
    fn test_keyframes_eased() {
        let kf = Keyframes::new()
            .add(0.0, 0.0)
            .add_eased(1.0, 100.0, Easing::OutQuad);

        let mid: f64 = kf.at(0.5).unwrap();
        // OutQuad at 0.5 = 0.75, so value should be 75
        assert!((mid - 75.0).abs() < 0.01);
    }

    #[test]
    fn test_keyframes_empty() {
        let kf: Keyframes<f64> = Keyframes::new();
        assert!(kf.at(0.5).is_none());
    }

    // =========================================================================
    // AnimatedValue Tests
    // =========================================================================

    #[test]
    fn test_animated_value_new() {
        let mut av = AnimatedValue::new(0.0, Duration::from_millis(100));
        assert_eq!(*av.value(), 0.0);
    }

    #[test]
    fn test_animated_value_set() {
        let mut av = AnimatedValue::new(0.0, Duration::from_millis(100));
        av.set(50.0);
        assert_eq!(*av.value(), 50.0);
    }

    #[test]
    fn test_animated_value_target() {
        let mut av = AnimatedValue::new(0.0, Duration::from_millis(100));
        av.animate_to(100.0);
        assert_eq!(*av.target(), 100.0);
    }

    // =========================================================================
    // Sequence Tests
    // =========================================================================

    #[test]
    fn test_sequence_new() {
        let seq = Sequence::new();
        assert!(!seq.is_running());
        assert!(!seq.is_complete());
    }

    #[test]
    fn test_sequence_then() {
        let seq = Sequence::new()
            .then(Duration::from_millis(100), 0.5)
            .then(Duration::from_millis(100), 1.0);

        assert_eq!(seq.steps.len(), 2);
    }

    #[test]
    fn test_sequence_start() {
        let mut seq = Sequence::new()
            .then(Duration::from_millis(100), 1.0);

        seq.start();
        assert!(seq.is_running());
    }

    #[test]
    fn test_sequence_value() {
        let mut seq = Sequence::new()
            .then(Duration::from_millis(0), 1.0); // Instant completion

        seq.start();
        std::thread::sleep(Duration::from_millis(10));
        let _ = seq.value();

        assert!(seq.is_complete());
    }

    // =========================================================================
    // Ticker Tests
    // =========================================================================

    #[test]
    fn test_ticker_new() {
        let ticker = Ticker::new();
        assert_eq!(ticker.fps(), 0.0);
    }

    #[test]
    fn test_ticker_tick() {
        let mut ticker = Ticker::new();
        let dt = ticker.tick();

        // First tick should return default dt
        assert!(dt > 0.0);
        assert!(dt < 0.1);
    }

    #[test]
    fn test_ticker_with_target_fps() {
        let ticker = Ticker::with_target_fps(30.0);
        let frame_dur = ticker.frame_duration();

        assert!((frame_dur.as_secs_f64() - 1.0/30.0).abs() < 0.001);
    }

    // =========================================================================
    // Interpolatable Tests
    // =========================================================================

    #[test]
    fn test_interpolatable_f64() {
        let a: f64 = 0.0;
        let b: f64 = 100.0;
        assert_eq!(a.lerp(&b, 0.0), 0.0);
        assert_eq!(a.lerp(&b, 1.0), 100.0);
        assert_eq!(a.lerp(&b, 0.5), 50.0);
    }

    #[test]
    fn test_interpolatable_u8() {
        let a: u8 = 0;
        let b: u8 = 100;
        assert_eq!(a.lerp(&b, 0.5), 50);
    }

    #[test]
    fn test_interpolatable_tuple() {
        let a = (0.0, 0.0);
        let b = (100.0, 200.0);
        let mid = a.lerp(&b, 0.5);
        assert_eq!(mid, (50.0, 100.0));
    }

    // =========================================================================
    // Preset Tests
    // =========================================================================

    #[test]
    fn test_preset_fade_in() {
        let mut seq = presets::fade_in(100);
        seq.start();
        assert!(seq.is_running());
    }

    #[test]
    fn test_preset_pulse() {
        let seq = presets::pulse(200);
        assert!(seq.repeat);
    }

    #[test]
    fn test_preset_blink() {
        let seq = presets::blink(500);
        assert!(seq.repeat);
        assert_eq!(seq.steps.len(), 2);
    }
}
