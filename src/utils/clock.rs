//! The time animations, timers and ticks read
//!
//! [`now`] is the real time unless a test installed a
//! [`MockTime`](crate::testing::MockTime) on this thread, so a test advances
//! time instead of sleeping. A custom widget that animates or expires reads
//! [`now`] and [`since`] the same way, and its tests can do the same.
//!
//! The clock belongs to the thread: the UI runs on one thread, and tests that
//! run side by side do not see each other's time. A worker thread measures real
//! work and keeps the real time.

use std::cell::RefCell;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// A mock clock: the real time when it was installed, plus its counter
#[derive(Clone)]
struct Mock {
    base: Instant,
    elapsed_ms: Arc<AtomicU64>,
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
pub(crate) fn install_mock(elapsed_ms: Arc<AtomicU64>) -> MockClockGuard {
    // The clock reads what it read before, so time keeps moving forward for
    // anything that already holds an Instant, whatever the counter says now.
    let reading = now();
    let counted = Duration::from_millis(elapsed_ms.load(Ordering::Relaxed));
    let mock = Mock {
        base: reading.checked_sub(counted).unwrap_or(reading),
        elapsed_ms,
    };
    let previous = MOCK.with_borrow_mut(|current| current.replace(mock));
    MockClockGuard {
        previous,
        _not_send: PhantomData,
    }
}
