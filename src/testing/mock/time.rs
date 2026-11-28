//! Mock time controller for testing time-dependent code

use crate::utils::clock::{self, MockClockGuard, MockWall};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

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
///
/// [`set_wall_clock`](Self::set_wall_clock) also fixes the time of day and the
/// time zone, so `Date::today`, `Time::now` and time axes do not depend on
/// when or where the test runs. Advancing moves the time of day too:
///
/// ```rust
/// use revue::testing::MockTime;
/// use revue::widget::Date;
/// use std::time::{Duration, SystemTime};
///
/// let time = MockTime::new();
/// // 2024-02-29 23:30 UTC, in a zone one hour east of UTC
/// let utc = SystemTime::UNIX_EPOCH + Duration::from_secs(1_709_249_400);
/// time.set_wall_clock(utc, 3600);
/// let _clock = time.install();
/// assert_eq!(Date::today(), Date::new(2024, 3, 1), "00:30 there");
/// time.advance_secs(24 * 3600);
/// assert_eq!(Date::today(), Date::new(2024, 3, 2));
/// ```
#[derive(Debug, Clone)]
pub struct MockTime {
    elapsed_ms: Arc<AtomicU64>,
    wall: Arc<Mutex<Option<MockWall>>>,
}

impl MockTime {
    /// Create a new mock time starting at 0
    pub fn new() -> Self {
        Self {
            elapsed_ms: Arc::new(AtomicU64::new(0)),
            wall: Arc::new(Mutex::new(None)),
        }
    }

    /// Make this this thread's clock until the guard is dropped
    pub fn install(&self) -> MockClockGuard {
        clock::install_mock(Arc::clone(&self.elapsed_ms), Arc::clone(&self.wall))
    }

    /// Fix the time of day at `time`, in a time zone `utc_offset_secs` seconds
    /// east of UTC (negative is west: New York in winter is `-5 * 3600`)
    ///
    /// From here on the time of day moves with this mock: advancing it by a
    /// minute makes the time of day a minute later. The zone is a fixed
    /// offset, with no daylight saving change. Without this call the time of
    /// day is the real one in the machine's own zone, even while the mock is
    /// installed. It can be called before or after [`install`](Self::install).
    ///
    /// # Panics
    ///
    /// If `utc_offset_secs` is beyond ±25:59:59 (±93,599 seconds).
    pub fn set_wall_clock(&self, time: SystemTime, utc_offset_secs: i32) {
        let offset = jiff::tz::Offset::from_seconds(utc_offset_secs).unwrap_or_else(|_| {
            panic!("a UTC offset is at most ±93599 seconds, not {utc_offset_secs}")
        });
        let wall = MockWall {
            at: time,
            at_elapsed_ms: self.elapsed_ms(),
            offset,
        };
        *self.wall.lock().unwrap_or_else(|e| e.into_inner()) = Some(wall);
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

    /// Reset time to 0; a fixed time of day goes back by as much
    pub fn reset(&self) {
        self.elapsed_ms.store(0, Ordering::Relaxed);
    }

    /// Set time to specific value; a fixed time of day moves by as much
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
