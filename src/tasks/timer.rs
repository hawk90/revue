//! Timer system for delayed and periodic events
//!
//! Integrates with App's tick loop for non-blocking timers.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Unique timer identifier
pub type TimerId = &'static str;

/// Timer entry with expiration info
#[derive(Debug, Clone)]
pub struct TimerEntry {
    /// When the timer expires
    pub expires_at: Instant,
    /// Optional repeat interval
    pub repeat: Option<Duration>,
    /// Whether timer is active
    pub active: bool,
}

/// Timer manager for delayed events
///
/// # Example
///
/// ```ignore
/// let mut timer = Timer::new();
///
/// // One-shot timer
/// timer.set("message_clear", Duration::from_secs(3));
///
/// // Repeating timer
/// timer.set_repeating("auto_refresh", Duration::from_secs(30));
///
/// // In tick handler
/// while let Some(id) = timer.poll_expired() {
///     match id {
///         "message_clear" => state.message = None,
///         "auto_refresh" => state.refresh(),
///         _ => {}
///     }
/// }
/// ```
#[derive(Debug, Default)]
pub struct Timer {
    timers: HashMap<TimerId, TimerEntry>,
    expired: Vec<TimerId>,
}

impl Timer {
    /// Create a new timer manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a one-shot timer
    pub fn set(&mut self, id: TimerId, duration: Duration) {
        self.timers.insert(
            id,
            TimerEntry {
                expires_at: Instant::now() + duration,
                repeat: None,
                active: true,
            },
        );
    }

    /// Set a repeating timer
    pub fn set_repeating(&mut self, id: TimerId, interval: Duration) {
        self.timers.insert(
            id,
            TimerEntry {
                expires_at: Instant::now() + interval,
                repeat: Some(interval),
                active: true,
            },
        );
    }

    /// Cancel a timer
    pub fn cancel(&mut self, id: TimerId) {
        self.timers.remove(id);
    }

    /// Pause a timer (keeps remaining time)
    pub fn pause(&mut self, id: TimerId) {
        if let Some(entry) = self.timers.get_mut(id) {
            entry.active = false;
        }
    }

    /// Resume a paused timer
    pub fn resume(&mut self, id: TimerId) {
        if let Some(entry) = self.timers.get_mut(id) {
            entry.active = true;
        }
    }

    /// Check if timer exists and is active
    pub fn is_active(&self, id: TimerId) -> bool {
        self.timers.get(id).map(|e| e.active).unwrap_or(false)
    }

    /// Get remaining time for a timer
    pub fn remaining(&self, id: TimerId) -> Option<Duration> {
        self.timers.get(id).and_then(|entry| {
            let now = Instant::now();
            if entry.expires_at > now {
                Some(entry.expires_at - now)
            } else {
                None
            }
        })
    }

    /// Poll for expired timers. Call this in your tick handler.
    /// Returns expired timer IDs one at a time.
    pub fn poll_expired(&mut self) -> Option<TimerId> {
        // First, collect expired timers
        if self.expired.is_empty() {
            let now = Instant::now();
            let mut to_reschedule = Vec::new();

            for (&id, entry) in self.timers.iter_mut() {
                if entry.active && now >= entry.expires_at {
                    self.expired.push(id);
                    if let Some(interval) = entry.repeat {
                        to_reschedule.push((id, interval));
                    }
                }
            }

            // Reschedule repeating timers
            for (id, interval) in to_reschedule {
                if let Some(entry) = self.timers.get_mut(id) {
                    entry.expires_at = now + interval;
                }
            }

            // Remove one-shot timers that expired
            self.timers
                .retain(|_, entry| entry.repeat.is_some() || entry.expires_at > now);
        }

        // Return one expired timer at a time
        self.expired.pop()
    }

    /// Check if any timers are pending
    pub fn has_pending(&self) -> bool {
        !self.timers.is_empty()
    }

    /// Get count of active timers
    pub fn count(&self) -> usize {
        self.timers.len()
    }

    /// Clear all timers
    pub fn clear(&mut self) {
        self.timers.clear();
        self.expired.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_set_and_poll() {
        let mut timer = Timer::new();
        timer.set("test", Duration::from_millis(10));

        assert!(timer.is_active("test"));
        assert!(timer.remaining("test").is_some());

        // Wait for expiration
        std::thread::sleep(Duration::from_millis(20));

        let expired = timer.poll_expired();
        assert_eq!(expired, Some("test"));

        // Should be removed after polling
        assert!(!timer.is_active("test"));
    }

    #[test]
    fn test_repeating_timer() {
        let mut timer = Timer::new();
        timer.set_repeating("repeat", Duration::from_millis(10));

        std::thread::sleep(Duration::from_millis(15));

        let expired = timer.poll_expired();
        assert_eq!(expired, Some("repeat"));

        // Should still be active (repeating)
        assert!(timer.is_active("repeat"));
    }

    #[test]
    fn test_timer_cancel() {
        let mut timer = Timer::new();
        timer.set("cancel_me", Duration::from_secs(10));

        assert!(timer.is_active("cancel_me"));
        timer.cancel("cancel_me");
        assert!(!timer.is_active("cancel_me"));
    }
}
