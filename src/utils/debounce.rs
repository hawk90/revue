//! Debounce and throttle utilities for event handling
//!
//! High-frequency events (mouse move, key repeat, scroll) can overwhelm handlers.
//! These utilities help manage event frequency.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::{Debouncer, Throttle};
//! use std::time::Duration;
//!
//! // Debounce search input - call after 300ms of no activity
//! let mut debouncer = Debouncer::new(Duration::from_millis(300));
//! if debouncer.call() {
//!     perform_search();
//! }
//!
//! // Throttle scroll handler - call at most once per 100ms
//! let mut throttle = Throttle::new(Duration::from_millis(100));
//! if throttle.call() {
//!     update_position();
//! }
//! ```

use std::time::{Duration, Instant};

/// Edge mode for debounce/throttle behavior
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Edge {
    /// Call on the trailing edge (after the delay)
    #[default]
    Trailing,
    /// Call on the leading edge (immediately, then wait)
    Leading,
    /// Call on both edges
    Both,
}

/// A debouncer delays execution until a period of inactivity
///
/// Useful for search inputs, resize handlers, etc.
#[derive(Clone, Debug)]
pub struct Debouncer {
    /// Delay duration
    delay: Duration,
    /// Last call time
    last_call: Option<Instant>,
    /// Edge mode
    edge: Edge,
    /// Whether leading edge was fired
    leading_fired: bool,
    /// Whether there's a pending call
    pending: bool,
}

impl Debouncer {
    /// Create a new debouncer with the given delay
    pub fn new(delay: Duration) -> Self {
        Self {
            delay,
            last_call: None,
            edge: Edge::Trailing,
            leading_fired: false,
            pending: false,
        }
    }

    /// Create a debouncer with leading edge (immediate first call)
    pub fn leading(delay: Duration) -> Self {
        Self {
            delay,
            last_call: None,
            edge: Edge::Leading,
            leading_fired: false,
            pending: false,
        }
    }

    /// Create a debouncer with both edges
    pub fn both_edges(delay: Duration) -> Self {
        Self {
            delay,
            last_call: None,
            edge: Edge::Both,
            leading_fired: false,
            pending: false,
        }
    }

    /// Set the edge mode
    pub fn with_edge(mut self, edge: Edge) -> Self {
        self.edge = edge;
        self
    }

    /// Record a call and check if the handler should execute
    ///
    /// Returns `true` if the handler should be called now.
    pub fn call(&mut self) -> bool {
        let now = Instant::now();

        match self.edge {
            Edge::Trailing => {
                self.pending = true;
                self.last_call = Some(now);
                false
            }
            Edge::Leading => {
                if !self.leading_fired {
                    self.leading_fired = true;
                    self.last_call = Some(now);
                    // Leading edge fires immediately, no pending
                    true
                } else {
                    self.last_call = Some(now);
                    // Subsequent calls just reset timer, no trailing
                    false
                }
            }
            Edge::Both => {
                self.pending = true;
                if !self.leading_fired {
                    self.leading_fired = true;
                    self.last_call = Some(now);
                    true
                } else {
                    self.last_call = Some(now);
                    false
                }
            }
        }
    }

    /// Check if a pending call is ready to execute (trailing edge)
    ///
    /// Call this periodically (e.g., on each tick) to check if the
    /// debounce period has elapsed.
    pub fn is_ready(&mut self) -> bool {
        if !self.pending {
            return false;
        }

        if let Some(last) = self.last_call {
            if last.elapsed() >= self.delay {
                self.pending = false;
                self.leading_fired = false;
                self.last_call = None;
                matches!(self.edge, Edge::Trailing | Edge::Both)
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Check if there's a pending call
    pub fn is_pending(&self) -> bool {
        self.pending
    }

    /// Cancel any pending call
    pub fn cancel(&mut self) {
        self.pending = false;
        self.last_call = None;
        self.leading_fired = false;
    }

    /// Reset the debouncer state
    pub fn reset(&mut self) {
        self.cancel();
    }

    /// Get the remaining time until the debouncer fires (if pending)
    pub fn remaining(&self) -> Option<Duration> {
        if !self.pending {
            return None;
        }
        self.last_call.map(|last| {
            let elapsed = last.elapsed();
            if elapsed >= self.delay {
                Duration::ZERO
            } else {
                self.delay - elapsed
            }
        })
    }

    /// Get the configured delay
    pub fn delay(&self) -> Duration {
        self.delay
    }

    /// Set a new delay
    pub fn set_delay(&mut self, delay: Duration) {
        self.delay = delay;
    }
}

impl Default for Debouncer {
    fn default() -> Self {
        Self::new(Duration::from_millis(300))
    }
}

/// A throttle limits execution rate to at most once per interval
///
/// Useful for scroll handlers, mouse move events, etc.
#[derive(Clone, Debug)]
pub struct Throttle {
    /// Minimum interval between calls
    interval: Duration,
    /// Last execution time
    last_exec: Option<Instant>,
    /// Edge mode
    edge: Edge,
    /// Whether there's a pending trailing call
    pending: bool,
}

impl Throttle {
    /// Create a new throttle with the given interval
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            last_exec: None,
            edge: Edge::Leading,
            pending: false,
        }
    }

    /// Create a throttle with trailing edge
    pub fn trailing(interval: Duration) -> Self {
        Self {
            interval,
            last_exec: None,
            edge: Edge::Trailing,
            pending: false,
        }
    }

    /// Create a throttle with both edges
    pub fn both_edges(interval: Duration) -> Self {
        Self {
            interval,
            last_exec: None,
            edge: Edge::Both,
            pending: false,
        }
    }

    /// Set the edge mode
    pub fn with_edge(mut self, edge: Edge) -> Self {
        self.edge = edge;
        self
    }

    /// Record a call and check if the handler should execute
    ///
    /// Returns `true` if the handler should be called now.
    pub fn call(&mut self) -> bool {
        let now = Instant::now();

        let can_execute = match self.last_exec {
            None => true,
            Some(last) => last.elapsed() >= self.interval,
        };

        match self.edge {
            Edge::Leading => {
                if can_execute {
                    self.last_exec = Some(now);
                    self.pending = false;
                    true
                } else {
                    false
                }
            }
            Edge::Trailing => {
                self.pending = true;
                false
            }
            Edge::Both => {
                if can_execute {
                    self.last_exec = Some(now);
                    self.pending = false;
                    true
                } else {
                    self.pending = true;
                    false
                }
            }
        }
    }

    /// Check if a pending trailing call is ready
    pub fn is_ready(&mut self) -> bool {
        if !self.pending {
            return false;
        }

        let can_execute = match self.last_exec {
            None => true,
            Some(last) => last.elapsed() >= self.interval,
        };

        if can_execute && matches!(self.edge, Edge::Trailing | Edge::Both) {
            self.last_exec = Some(Instant::now());
            self.pending = false;
            true
        } else {
            false
        }
    }

    /// Check if there's a pending trailing call
    pub fn is_pending(&self) -> bool {
        self.pending
    }

    /// Cancel any pending trailing call
    pub fn cancel(&mut self) {
        self.pending = false;
    }

    /// Reset the throttle state
    pub fn reset(&mut self) {
        self.last_exec = None;
        self.pending = false;
    }

    /// Get the remaining time until the next call is allowed
    pub fn remaining(&self) -> Duration {
        match self.last_exec {
            None => Duration::ZERO,
            Some(last) => {
                let elapsed = last.elapsed();
                if elapsed >= self.interval {
                    Duration::ZERO
                } else {
                    self.interval - elapsed
                }
            }
        }
    }

    /// Get the configured interval
    pub fn interval(&self) -> Duration {
        self.interval
    }

    /// Set a new interval
    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval;
    }
}

impl Default for Throttle {
    fn default() -> Self {
        Self::new(Duration::from_millis(100))
    }
}

/// Helper function to create a debouncer
pub fn debouncer(delay: Duration) -> Debouncer {
    Debouncer::new(delay)
}

/// Helper function to create a debouncer with milliseconds
pub fn debounce_ms(ms: u64) -> Debouncer {
    Debouncer::new(Duration::from_millis(ms))
}

/// Helper function to create a throttle
pub fn throttle(interval: Duration) -> Throttle {
    Throttle::new(interval)
}

/// Helper function to create a throttle with milliseconds
pub fn throttle_ms(ms: u64) -> Throttle {
    Throttle::new(Duration::from_millis(ms))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_debouncer_new() {
        let d = Debouncer::new(Duration::from_millis(100));
        assert_eq!(d.delay(), Duration::from_millis(100));
        assert!(!d.is_pending());
    }

    #[test]
    fn test_debouncer_trailing_edge() {
        let mut d = Debouncer::new(Duration::from_millis(50));

        // First call should not execute (trailing edge)
        assert!(!d.call());
        assert!(d.is_pending());

        // Not ready yet
        assert!(!d.is_ready());

        // Wait for debounce period
        sleep(Duration::from_millis(60));

        // Now should be ready
        assert!(d.is_ready());
        assert!(!d.is_pending());
    }

    #[test]
    fn test_debouncer_leading_edge() {
        let mut d = Debouncer::leading(Duration::from_millis(50));

        // First call should execute immediately
        assert!(d.call());
        assert!(!d.is_pending());

        // Second call should not execute
        assert!(!d.call());
        assert!(!d.is_pending()); // Leading doesn't have trailing

        // Wait and call again
        sleep(Duration::from_millis(60));
        // After is_ready resets, next call should execute
        d.reset();
        assert!(d.call());
    }

    #[test]
    fn test_debouncer_both_edges() {
        let mut d = Debouncer::both_edges(Duration::from_millis(50));

        // First call should execute (leading)
        assert!(d.call());

        // Second call should not execute immediately
        assert!(!d.call());
        assert!(d.is_pending());

        // Wait for trailing edge
        sleep(Duration::from_millis(60));

        // Trailing edge should fire
        assert!(d.is_ready());
    }

    #[test]
    fn test_debouncer_cancel() {
        let mut d = Debouncer::new(Duration::from_millis(100));

        d.call();
        assert!(d.is_pending());

        d.cancel();
        assert!(!d.is_pending());
        assert!(d.remaining().is_none());
    }

    #[test]
    fn test_debouncer_remaining() {
        let mut d = Debouncer::new(Duration::from_millis(100));

        assert!(d.remaining().is_none());

        d.call();
        let remaining = d.remaining().unwrap();
        assert!(remaining <= Duration::from_millis(100));
        assert!(remaining > Duration::ZERO);
    }

    #[test]
    fn test_throttle_new() {
        let t = Throttle::new(Duration::from_millis(100));
        assert_eq!(t.interval(), Duration::from_millis(100));
        assert!(!t.is_pending());
    }

    #[test]
    fn test_throttle_leading_edge() {
        let mut t = Throttle::new(Duration::from_millis(50));

        // First call should execute
        assert!(t.call());

        // Second call should not execute (within interval)
        assert!(!t.call());

        // Wait for interval
        sleep(Duration::from_millis(60));

        // Now should execute
        assert!(t.call());
    }

    #[test]
    fn test_throttle_trailing_edge() {
        let mut t = Throttle::trailing(Duration::from_millis(50));

        // First call should not execute (trailing)
        assert!(!t.call());
        assert!(t.is_pending());

        // Check is_ready immediately - should execute since no last_exec
        assert!(t.is_ready());
        assert!(!t.is_pending());
    }

    #[test]
    fn test_throttle_both_edges() {
        let mut t = Throttle::both_edges(Duration::from_millis(50));

        // First call should execute (leading)
        assert!(t.call());

        // Second call should mark pending
        assert!(!t.call());
        assert!(t.is_pending());

        // Wait for interval
        sleep(Duration::from_millis(60));

        // Trailing should fire
        assert!(t.is_ready());
    }

    #[test]
    fn test_throttle_remaining() {
        let mut t = Throttle::new(Duration::from_millis(100));

        assert_eq!(t.remaining(), Duration::ZERO);

        t.call();
        let remaining = t.remaining();
        assert!(remaining <= Duration::from_millis(100));
    }

    #[test]
    fn test_throttle_reset() {
        let mut t = Throttle::new(Duration::from_millis(100));

        t.call();
        assert!(!t.call()); // Throttled

        t.reset();
        assert!(t.call()); // Can call again after reset
    }

    #[test]
    fn test_helper_functions() {
        let d = debouncer(Duration::from_millis(100));
        assert_eq!(d.delay(), Duration::from_millis(100));

        let d = debounce_ms(200);
        assert_eq!(d.delay(), Duration::from_millis(200));

        let t = throttle(Duration::from_millis(100));
        assert_eq!(t.interval(), Duration::from_millis(100));

        let t = throttle_ms(200);
        assert_eq!(t.interval(), Duration::from_millis(200));
    }

    #[test]
    fn test_debouncer_default() {
        let d = Debouncer::default();
        assert_eq!(d.delay(), Duration::from_millis(300));
    }

    #[test]
    fn test_throttle_default() {
        let t = Throttle::default();
        assert_eq!(t.interval(), Duration::from_millis(100));
    }

    #[test]
    fn test_debouncer_set_delay() {
        let mut d = Debouncer::new(Duration::from_millis(100));
        d.set_delay(Duration::from_millis(200));
        assert_eq!(d.delay(), Duration::from_millis(200));
    }

    #[test]
    fn test_throttle_set_interval() {
        let mut t = Throttle::new(Duration::from_millis(100));
        t.set_interval(Duration::from_millis(200));
        assert_eq!(t.interval(), Duration::from_millis(200));
    }

    #[test]
    fn test_throttle_cancel() {
        let mut t = Throttle::trailing(Duration::from_millis(100));
        t.call();
        assert!(t.is_pending());

        t.cancel();
        assert!(!t.is_pending());
    }

    #[test]
    fn test_edge_enum() {
        assert_eq!(Edge::default(), Edge::Trailing);

        let d = Debouncer::new(Duration::from_millis(100)).with_edge(Edge::Leading);
        assert_eq!(d.edge, Edge::Leading);

        let t = Throttle::new(Duration::from_millis(100)).with_edge(Edge::Both);
        assert_eq!(t.edge, Edge::Both);
    }
}
