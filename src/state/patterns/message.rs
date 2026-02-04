//! Message display with automatic timeout
//!
//! Provides a simple message state that automatically clears after a timeout period.
//! Commonly used for showing temporary status messages, errors, or confirmations.
//!
//! # Example
//!
//! ```ignore
//! use revue::patterns::MessageState;
//!
//! struct App {
//!     message: MessageState,
//! }
//!
//! impl App {
//!     fn save(&mut self) {
//!         // ... save logic ...
//!         self.message.set("Saved successfully!".to_string());
//!     }
//!
//!     fn poll(&mut self) -> bool {
//!         // Check timeout in animation loop
//!         self.message.check_timeout()
//!     }
//!
//!     fn render_footer(&self, ctx: &mut RenderContext) {
//!         if let Some(msg) = self.message.get() {
//!             ctx.draw_text(0, 0, msg, YELLOW);
//!             return;
//!         }
//!         // ... render normal footer ...
//!     }
//! }
//! ```

use crate::constants::MESSAGE_DEFAULT_DURATION;
use std::time::{Duration, Instant};

/// Default message display duration (3 seconds)
///
/// This is a re-export of [`crate::constants::MESSAGE_DEFAULT_DURATION`] for backwards compatibility.
pub const DEFAULT_MESSAGE_DURATION: Duration = MESSAGE_DEFAULT_DURATION;

/// Message state with automatic timeout
///
/// Messages are automatically cleared after `MESSAGE_DURATION` seconds.
/// Returns `true` from `check_timeout()` when a message is cleared,
/// indicating that a redraw is needed.
#[derive(Clone, Debug)]
pub struct MessageState {
    /// Current message text (None if no message)
    message: Option<String>,
    /// Time when message was set
    message_time: Option<Instant>,
    /// Duration before auto-clear
    duration: Duration,
}

impl Default for MessageState {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageState {
    /// Create a new message state with default 3-second timeout
    pub fn new() -> Self {
        Self {
            message: None,
            message_time: None,
            duration: DEFAULT_MESSAGE_DURATION,
        }
    }

    /// Create with custom timeout duration
    ///
    /// # Example
    ///
    /// ```ignore
    /// // 5-second timeout
    /// let msg = MessageState::with_duration(Duration::from_secs(5));
    /// ```
    pub fn with_duration(duration: Duration) -> Self {
        Self {
            message: None,
            message_time: None,
            duration,
        }
    }

    /// Set a new message (clears any existing message)
    ///
    /// # Example
    ///
    /// ```ignore
    /// app.message.set("File saved!".to_string());
    /// ```
    pub fn set(&mut self, message: String) {
        self.message = Some(message);
        self.message_time = Some(Instant::now());
    }

    /// Set message with custom duration (one-time override)
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Show error for 10 seconds
    /// app.message.set_with_duration(
    ///     "Critical error!".to_string(),
    ///     Duration::from_secs(10)
    /// );
    /// ```
    pub fn set_with_duration(&mut self, message: String, duration: Duration) {
        self.message = Some(message);
        self.message_time = Some(Instant::now());
        self.duration = duration;
    }

    /// Get current message (if any)
    pub fn get(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Check if there's an active message
    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    /// Clear message immediately
    pub fn clear(&mut self) {
        self.message = None;
        self.message_time = None;
    }

    /// Check if message timeout has elapsed and clear if so
    ///
    /// Returns `true` if message was cleared (needs redraw), `false` otherwise.
    ///
    /// Call this in your animation/poll loop:
    ///
    /// # Example
    ///
    /// ```ignore
    /// fn poll(&mut self) -> bool {
    ///     let mut needs_redraw = false;
    ///     needs_redraw |= self.message.check_timeout();
    ///     needs_redraw
    /// }
    /// ```
    pub fn check_timeout(&mut self) -> bool {
        if let Some(time) = self.message_time {
            if time.elapsed() >= self.duration {
                self.clear();
                return true; // Needs redraw
            }
        }
        false
    }

    /// Get remaining time before timeout
    ///
    /// Returns `None` if no message is active.
    pub fn remaining(&self) -> Option<Duration> {
        self.message_time.map(|time| {
            let elapsed = time.elapsed();
            self.duration.saturating_sub(elapsed)
        })
    }

    /// Check if message is about to expire (< 1 second remaining)
    pub fn is_expiring(&self) -> bool {
        self.remaining()
            .map(|r| r < Duration::from_secs(1))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_set_and_get() {
        let mut msg = MessageState::new();
        assert_eq!(msg.get(), None);

        msg.set("Hello".to_string());
        assert_eq!(msg.get(), Some("Hello"));
    }

    #[test]
    fn test_clear() {
        let mut msg = MessageState::new();
        msg.set("Test".to_string());
        assert!(msg.has_message());

        msg.clear();
        assert!(!msg.has_message());
        assert_eq!(msg.get(), None);
    }

    #[test]
    fn test_timeout() {
        let mut msg = MessageState::with_duration(Duration::from_millis(100));
        msg.set("Test".to_string());

        // Should not timeout immediately
        assert!(!msg.check_timeout());
        assert!(msg.has_message());

        // Wait for timeout
        thread::sleep(Duration::from_millis(150));

        // Should timeout and clear
        assert!(msg.check_timeout());
        assert!(!msg.has_message());
    }

    #[test]
    fn test_remaining() {
        let mut msg = MessageState::with_duration(Duration::from_secs(5));
        msg.set("Test".to_string());

        let remaining = msg.remaining().unwrap();
        assert!(remaining <= Duration::from_secs(5));
        assert!(remaining > Duration::from_secs(4));
    }

    #[test]
    fn test_is_expiring() {
        // Use 2 seconds duration so initially is_expiring() is false (remaining > 1s)
        let mut msg = MessageState::with_duration(Duration::from_millis(2000));
        msg.set("Test".to_string());

        // Initially not expiring (remaining ~2s > 1s)
        assert!(!msg.is_expiring());

        // After 1.2s, remaining ~0.8s < 1s, should be expiring
        thread::sleep(Duration::from_millis(1200));
        assert!(msg.is_expiring());
    }
}
