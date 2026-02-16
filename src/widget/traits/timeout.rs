//! Timeout utility for auto-clearing values

use std::time::{Duration, Instant};

/// A simple timeout tracker for auto-clearing messages or timed events.
///
/// # Example
/// ```rust,ignore
/// use revue::widget::traits::Timeout;
/// use std::time::Duration;
///
/// let mut msg_timeout = Timeout::new(Duration::from_secs(3));
///
/// // Set a message
/// msg_timeout.set("Operation complete".to_string());
///
/// // In your tick handler
/// if msg_timeout.is_expired() {
///     msg_timeout.clear();
/// }
///
/// // Get current value
/// if let Some(msg) = msg_timeout.get() {
///     println!("{}", msg);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Timeout<T> {
    value: Option<T>,
    set_time: Option<Instant>,
    duration: Duration,
}

impl<T> Timeout<T> {
    /// Create a new timeout tracker with the specified duration.
    pub fn new(duration: Duration) -> Self {
        Self {
            value: None,
            set_time: None,
            duration,
        }
    }

    /// Create a timeout with seconds.
    pub fn secs(secs: u64) -> Self {
        Self::new(Duration::from_secs(secs))
    }

    /// Create a timeout with milliseconds.
    pub fn millis(millis: u64) -> Self {
        Self::new(Duration::from_millis(millis))
    }

    /// Set a value and start the timeout timer.
    pub fn set(&mut self, value: T) {
        self.value = Some(value);
        self.set_time = Some(Instant::now());
    }

    /// Clear the value and timer.
    pub fn clear(&mut self) {
        self.value = None;
        self.set_time = None;
    }

    /// Check if the timeout has expired.
    pub fn is_expired(&self) -> bool {
        self.set_time
            .map(|t| t.elapsed() > self.duration)
            .unwrap_or(false)
    }

    /// Get a reference to the current value if set.
    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Get a mutable reference to the current value if set.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut()
    }

    /// Take the value, clearing the timeout.
    pub fn take(&mut self) -> Option<T> {
        self.set_time = None;
        self.value.take()
    }

    /// Check if a value is set (not necessarily expired).
    pub fn is_set(&self) -> bool {
        self.value.is_some()
    }

    /// Get remaining time before expiration.
    pub fn remaining(&self) -> Option<Duration> {
        self.set_time.map(|t| {
            let elapsed = t.elapsed();
            if elapsed < self.duration {
                self.duration - elapsed
            } else {
                Duration::ZERO
            }
        })
    }

    /// Update the timeout value without resetting the timer.
    pub fn update(&mut self, value: T) {
        self.value = Some(value);
    }

    /// Reset the timer without changing the value.
    pub fn reset_timer(&mut self) {
        if self.value.is_some() {
            self.set_time = Some(Instant::now());
        }
    }

    /// Auto-clear if expired and return reference.
    pub fn auto_clear(&mut self) -> Option<&T> {
        if self.is_expired() {
            self.clear();
            None
        } else {
            self.value.as_ref()
        }
    }
}

impl<T> Default for Timeout<T> {
    fn default() -> Self {
        Self::new(Duration::from_secs(3))
    }
}
