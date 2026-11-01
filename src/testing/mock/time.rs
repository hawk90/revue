//! Mock time controller for testing time-dependent code

use crate::utils::clock::{self, MockClockGuard};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Mock time controller for testing time-dependent code
///
/// [`install`](Self::install) makes it this thread's clock, so animations,
/// timers, toasts and debouncing move only when it advances:
///
/// ```rust
/// use revue::testing::MockTime;
/// use revue::utils::clock;
/// use std::time::Duration;
///
/// let time = MockTime::new();
/// let _clock = time.install();
/// let start = clock::now();
/// time.advance(Duration::from_secs(5));
/// assert_eq!(clock::since(start), Duration::from_secs(5));
/// ```
#[derive(Debug, Clone)]
pub struct MockTime {
    elapsed_ms: Arc<AtomicU64>,
}

impl MockTime {
    /// Create a new mock time starting at 0
    pub fn new() -> Self {
        Self {
            elapsed_ms: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Make this this thread's clock until the guard is dropped
    pub fn install(&self) -> MockClockGuard {
        clock::install_mock(Arc::clone(&self.elapsed_ms))
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        Duration::from_millis(self.elapsed_ms.load(Ordering::Relaxed))
    }

    /// Get elapsed milliseconds
    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed_ms.load(Ordering::Relaxed)
    }

    /// Advance time by duration
    pub fn advance(&self, duration: Duration) {
        self.elapsed_ms
            .fetch_add(duration.as_millis() as u64, Ordering::Relaxed);
    }

    /// Advance time by milliseconds
    pub fn advance_ms(&self, ms: u64) {
        self.elapsed_ms.fetch_add(ms, Ordering::Relaxed);
    }

    /// Advance time by seconds
    pub fn advance_secs(&self, secs: u64) {
        self.advance_ms(secs * 1000);
    }

    /// Reset time to 0
    pub fn reset(&self) {
        self.elapsed_ms.store(0, Ordering::Relaxed);
    }

    /// Set time to specific value
    pub fn set(&self, duration: Duration) {
        self.elapsed_ms
            .store(duration.as_millis() as u64, Ordering::Relaxed);
    }
}

impl Default for MockTime {
    fn default() -> Self {
        Self::new()
    }
}
