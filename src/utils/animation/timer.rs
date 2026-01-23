//! Timer implementation for animation timing

use crate::utils::easing::Easing;
use std::time::{Duration, Instant};

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
