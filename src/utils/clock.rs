//! The time animations, timers and ticks read, and the time of day
//!
//! [`now`] is the real time unless a test installed a
//! [`MockTime`](crate::testing::MockTime) on this thread, so a test advances
//! time instead of sleeping. A custom widget that animates or expires reads
//! [`now`] and [`since`] the same way, and its tests can do the same.
//!
//! [`wall_now`] is the time of day: what a date, a clock face or a time axis
//! shows. It is the real one unless the installed mock was given a wall time
//! with [`MockTime::set_wall_clock`](crate::testing::MockTime::set_wall_clock);
//! that also fixes the time zone, so `Date::today` is the same day on every
//! machine. Advancing the mock moves both clocks by the same amount.
//!
//! The clock belongs to the thread: the UI runs on one thread, and tests that
//! run side by side do not see each other's time. A worker thread measures real
//! work and keeps the real time.

use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

/// The wall time a [`MockTime`](crate::testing::MockTime) was set to
#[derive(Clone, Copy, Debug)]
pub(crate) struct MockWall {
    /// The wall time when it was set
    pub(crate) at: SystemTime,
    /// The mock's counter when it was set
    pub(crate) at_elapsed_ms: u64,
    /// The time zone: a fixed offset from UTC
    pub(crate) offset: jiff::tz::Offset,
}

impl MockWall {
    /// The wall time when the counter reads `elapsed_ms`
    fn time(&self, elapsed_ms: u64) -> SystemTime {
        // The counter can also go back (`set`, `reset`), and the wall time
        // goes back with it.
        let moved = if elapsed_ms >= self.at_elapsed_ms {
            self.at
                .checked_add(Duration::from_millis(elapsed_ms - self.at_elapsed_ms))
        } else {
            self.at
                .checked_sub(Duration::from_millis(self.at_elapsed_ms - elapsed_ms))
        };
        moved.unwrap_or(self.at)
    }
}

/// A mock clock: the real time when it was installed, plus its counter
#[derive(Clone)]
struct Mock {
    base: Instant,
    elapsed_ms: Arc<AtomicU64>,
    wall: Arc<Mutex<Option<MockWall>>>,
}

impl Mock {
    /// The mocked wall time and time zone, if the mock was given one
    fn wall(&self) -> Option<(SystemTime, jiff::tz::Offset)> {
        let wall = (*self.wall.lock().unwrap_or_else(|e| e.into_inner()))?;
        Some((
            wall.time(self.elapsed_ms.load(Ordering::Relaxed)),
            wall.offset,
        ))
    }
}

thread_local! {
    static MOCK: RefCell<Option<Mock>> = const { RefCell::new(None) };
}

/// The current time on this thread
pub fn now() -> Instant {
    MOCK.with_borrow(|mock| match mock {
        Some(mock) => mock.base + Duration::from_millis(mock.elapsed_ms.load(Ordering::Relaxed)),
        None => Instant::now(),
    })
}

/// The time since `earlier`, zero if `earlier` is later than [`now`]
pub fn since(earlier: Instant) -> Duration {
    now().saturating_duration_since(earlier)
}

/// The time of day on this thread
///
/// The real one, unless the installed [`MockTime`](crate::testing::MockTime)
/// was given a wall time; a mock without one keeps the real time of day, so it
/// can drive animations alone.
pub fn wall_now() -> SystemTime {
    MOCK.with_borrow(|mock| mock.as_ref().and_then(Mock::wall))
        .map_or_else(SystemTime::now, |(time, _)| time)
}

/// The local date and time on this thread: [`wall_now`] in the local time
/// zone, or in the mock's time zone when the mock fixes one
pub(crate) fn local_now() -> jiff::ZonedDateTime {
    match MOCK.with_borrow(|mock| mock.as_ref().and_then(Mock::wall)) {
        Some((time, offset)) => {
            // Past jiff's years -9999..=9999 it is the nearest end.
            let timestamp =
                jiff::Timestamp::try_from(time).unwrap_or(if time < SystemTime::UNIX_EPOCH {
                    jiff::Timestamp::MIN
                } else {
                    jiff::Timestamp::MAX
                });
            timestamp.to_zoned_datetime(offset.to_time_zone())
        }
        None => jiff::ZonedDateTime::now(),
    }
}

/// Keeps a [`MockTime`](crate::testing::MockTime) installed on this thread
/// until it is dropped; then the clock installed before it is back
#[must_use = "the mock clock is removed as soon as this is dropped"]
pub struct MockClockGuard {
    previous: Option<Mock>,
    // The clock it restores is this thread's.
    _not_send: PhantomData<*const ()>,
}

impl Drop for MockClockGuard {
    fn drop(&mut self) {
        let previous = self.previous.take();
        MOCK.with_borrow_mut(|mock| *mock = previous);
    }
}

/// Install a mock counter as this thread's clock
pub(crate) fn install_mock(
    elapsed_ms: Arc<AtomicU64>,
    wall: Arc<Mutex<Option<MockWall>>>,
) -> MockClockGuard {
    // The clock reads what it read before, so time keeps moving forward for
    // anything that already holds an Instant, whatever the counter says now.
    let reading = now();
    let counted = Duration::from_millis(elapsed_ms.load(Ordering::Relaxed));
    let mock = Mock {
        base: reading.checked_sub(counted).unwrap_or(reading),
        elapsed_ms,
        wall,
    };
    let previous = MOCK.with_borrow_mut(|current| current.replace(mock));
    MockClockGuard {
        previous,
        _not_send: PhantomData,
    }
}
