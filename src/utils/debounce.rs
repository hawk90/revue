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
