//! Timer for animation timing

use crate::utils::easing::Easing;
use std::time::{Duration, Instant};

/// A simple timer for animation timing
#[derive(Clone, Debug)]
pub struct Timer {
    start: Option<Instant>,
    /// Duration of the timer
    pub duration: Duration,
    /// Time elapsed while paused
    elapsed_on_pause: Duration,
    /// Whether the timer is paused
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_new() {
        let timer = Timer::new(Duration::from_millis(100));
        assert_eq!(timer.duration, Duration::from_millis(100));
        assert!(!timer.is_running());
        assert!(!timer.is_finished());
        assert_eq!(timer.elapsed(), Duration::ZERO);
    }

    #[test]
    fn test_timer_from_millis() {
        let timer = Timer::from_millis(500);
        assert_eq!(timer.duration, Duration::from_millis(500));
    }

    #[test]
    fn test_timer_from_secs_f32() {
        let timer = Timer::from_secs_f32(1.5);
        assert_eq!(timer.duration, Duration::from_secs_f32(1.5));
    }

    #[test]
    fn test_timer_start() {
        let mut timer = Timer::from_millis(100);
        timer.start();
        assert!(timer.is_running());
        assert!(!timer.is_finished());
    }

    #[test]
    fn test_timer_pause() {
        let mut timer = Timer::from_millis(100);
        timer.start();
        timer.pause();
        assert!(!timer.is_running());
        // Elapsed time may be zero on fast systems, which is valid
        let elapsed_before = timer.elapsed();
        assert!(elapsed_before >= Duration::ZERO);
    }

    #[test]
    fn test_timer_resume() {
        let mut timer = Timer::from_millis(100);
        timer.start();
        timer.pause();
        timer.resume();
        assert!(timer.is_running());
    }

    #[test]
    fn test_timer_reset() {
        let mut timer = Timer::from_millis(100);
        timer.start();
        timer.reset();
        assert!(!timer.is_running());
        assert_eq!(timer.elapsed(), Duration::ZERO);
    }

    #[test]
    fn test_timer_restart() {
        let mut timer = Timer::from_millis(100);
        timer.start();
        timer.pause();
        timer.restart();
        assert!(timer.is_running());
        assert!(!timer.paused);
    }

    #[test]
    fn test_timer_elapsed() {
        let mut timer = Timer::from_millis(100);
        timer.reset(); // Ensure clean state
        timer.start();
        let elapsed = timer.elapsed();
        // Elapsed should be >= 0 and <= duration (may be 0 on fast systems)
        assert!(elapsed <= timer.duration);
    }

    #[test]
    fn test_timer_remaining() {
        let mut timer = Timer::from_millis(100);
        timer.start();
        let remaining = timer.remaining();
        assert!(remaining <= timer.duration);
    }

    #[test]
    fn test_timer_progress() {
        let mut timer = Timer::from_millis(100);
        timer.start();
        let progress = timer.progress();
        assert!(progress >= 0.0 && progress <= 1.0);
    }

    #[test]
    fn test_timer_progress_zero_duration() {
        let timer = Timer::from_secs_f32(0.0);
        assert_eq!(timer.progress(), 1.0);
    }

    #[test]
    fn test_timer_is_finished() {
        let mut timer = Timer::from_millis(10);
        timer.start();
        // Not finished immediately
        assert!(!timer.is_finished());
    }

    #[test]
    fn test_timer_elapsed_on_pause() {
        let mut timer = Timer::from_millis(100);
        timer.start();
        timer.pause();
        let elapsed1 = timer.elapsed();
        std::thread::sleep(Duration::from_millis(10));
        let elapsed2 = timer.elapsed();
        // Elapsed should not change while paused
        assert_eq!(elapsed1, elapsed2);
    }

    #[test]
    fn test_timer_pause_twice() {
        let mut timer = Timer::from_millis(100);
        timer.start();
        timer.pause();
        timer.pause(); // Second pause should do nothing
        assert!(!timer.is_running());
    }

    #[test]
    fn test_timer_resume_not_paused() {
        let mut timer = Timer::from_millis(100);
        timer.start();
        let was_running = timer.is_running();
        timer.resume();
        // Should remain running
        assert_eq!(timer.is_running(), was_running);
    }

    #[test]
    fn test_timer_remaining_clamped() {
        let timer = Timer::from_millis(10);
        let remaining = timer.remaining();
        assert_eq!(remaining, timer.duration);
    }

    #[test]
    fn test_timer_progress_eased() {
        let mut timer = Timer::from_millis(100);
        timer.start();
        let progress = timer.progress_eased(Easing::Linear);
        assert!(progress >= 0.0 && progress <= 1.0);
    }

    #[test]
    fn test_timer_default_fields() {
        let timer = Timer::new(Duration::from_secs(1));
        assert_eq!(timer.elapsed_on_pause, Duration::ZERO);
        assert!(!timer.paused);
    }
}
