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

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_timeout_new() {
        let timeout: Timeout<String> = Timeout::new(Duration::from_secs(5));
        assert!(!timeout.is_set());
        assert!(timeout.get().is_none());
    }

    #[test]
    fn test_timeout_secs() {
        let timeout: Timeout<String> = Timeout::secs(5);
        assert!(!timeout.is_set());
    }

    #[test]
    fn test_timeout_millis() {
        let timeout: Timeout<String> = Timeout::millis(500);
        assert!(!timeout.is_set());
    }

    #[test]
    fn test_timeout_default() {
        let timeout: Timeout<String> = Timeout::default();
        assert!(!timeout.is_set());
    }

    #[test]
    fn test_timeout_set_and_get() {
        let mut timeout = Timeout::secs(5);
        timeout.set("hello".to_string());
        assert!(timeout.is_set());
        assert_eq!(timeout.get(), Some(&"hello".to_string()));
    }

    #[test]
    fn test_timeout_get_mut() {
        let mut timeout = Timeout::secs(5);
        timeout.set("hello".to_string());
        if let Some(val) = timeout.get_mut() {
            *val = "world".to_string();
        }
        assert_eq!(timeout.get(), Some(&"world".to_string()));
    }

    #[test]
    fn test_timeout_clear() {
        let mut timeout = Timeout::secs(5);
        timeout.set("hello".to_string());
        assert!(timeout.is_set());
        timeout.clear();
        assert!(!timeout.is_set());
        assert!(timeout.get().is_none());
    }

    #[test]
    fn test_timeout_take() {
        let mut timeout = Timeout::secs(5);
        timeout.set("hello".to_string());
        let value = timeout.take();
        assert_eq!(value, Some("hello".to_string()));
        assert!(!timeout.is_set());
    }

    #[test]
    fn test_timeout_update() {
        let mut timeout = Timeout::secs(5);
        timeout.set("hello".to_string());
        timeout.update("world".to_string());
        assert_eq!(timeout.get(), Some(&"world".to_string()));
    }

    #[test]
    fn test_timeout_is_expired_not_set() {
        let timeout: Timeout<String> = Timeout::millis(10);
        assert!(!timeout.is_expired());
    }

    #[test]
    fn test_timeout_is_expired_not_yet() {
        let mut timeout = Timeout::secs(10);
        timeout.set("hello".to_string());
        assert!(!timeout.is_expired());
    }

    #[test]
    fn test_timeout_is_expired_after_duration() {
        let mut timeout = Timeout::millis(10);
        timeout.set("hello".to_string());
        thread::sleep(Duration::from_millis(20));
        assert!(timeout.is_expired());
    }

    #[test]
    fn test_timeout_remaining_not_set() {
        let timeout: Timeout<String> = Timeout::secs(5);
        assert!(timeout.remaining().is_none());
    }

    #[test]
    fn test_timeout_remaining_some() {
        let mut timeout = Timeout::secs(10);
        timeout.set("hello".to_string());
        let remaining = timeout.remaining();
        assert!(remaining.is_some());
        assert!(remaining.unwrap() <= Duration::from_secs(10));
    }

    #[test]
    fn test_timeout_remaining_zero_after_expiry() {
        let mut timeout = Timeout::millis(10);
        timeout.set("hello".to_string());
        thread::sleep(Duration::from_millis(20));
        let remaining = timeout.remaining();
        assert_eq!(remaining, Some(Duration::ZERO));
    }

    #[test]
    fn test_timeout_reset_timer() {
        let mut timeout = Timeout::millis(50);
        timeout.set("hello".to_string());
        thread::sleep(Duration::from_millis(30));
        timeout.reset_timer();
        // After reset, should not be expired yet
        assert!(!timeout.is_expired());
    }

    #[test]
    fn test_timeout_reset_timer_no_value() {
        let mut timeout: Timeout<String> = Timeout::secs(5);
        timeout.reset_timer(); // Should do nothing when no value
        assert!(!timeout.is_set());
    }

    #[test]
    fn test_timeout_auto_clear_not_expired() {
        let mut timeout = Timeout::secs(10);
        timeout.set("hello".to_string());
        let value = timeout.auto_clear();
        assert_eq!(value, Some(&"hello".to_string()));
        assert!(timeout.is_set());
    }

    #[test]
    fn test_timeout_auto_clear_expired() {
        let mut timeout = Timeout::millis(10);
        timeout.set("hello".to_string());
        thread::sleep(Duration::from_millis(20));
        let value = timeout.auto_clear();
        assert!(value.is_none());
        assert!(!timeout.is_set());
    }
}
