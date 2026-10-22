//! A change made on one thread reaches the computed values and effects
//! tracked on another (#873)
//!
//! The dependency tracker is per thread, so a `Signal::set` on a background
//! thread finds no dependents there. Each thread that tracks dependents
//! registers a queue. A change on any thread posts its signal to the other
//! threads' queues, and a thread [drains](drain) its own: the app's event
//! loop does on every frame, [`flush`](super::flush) does, and so does
//! reading a computed value. Draining notifies the thread's dependents of
//! the signals posted to it, as a change on that thread would have.

use super::SignalId;
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread::{self, ThreadId};

/// The signals changed on other threads, per thread that tracks dependents
fn queues() -> &'static Mutex<HashMap<ThreadId, Vec<SignalId>>> {
    static QUEUES: OnceLock<Mutex<HashMap<ThreadId, Vec<SignalId>>>> = OnceLock::new();
    QUEUES.get_or_init(Default::default)
}

/// How many threads track dependents
static TRACKING_THREADS: AtomicUsize = AtomicUsize::new(0);

/// This thread's place in [`queues`], removed when the thread ends
struct Registration {
    registered: Cell<bool>,
}

impl Drop for Registration {
    fn drop(&mut self) {
        if self.registered.get() {
            crate::utils::lock::lock_or_recover(queues()).remove(&thread::current().id());
            TRACKING_THREADS.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

thread_local! {
    static REGISTRATION: Registration = const {
        Registration {
            registered: Cell::new(false),
        }
    };
    /// Set while this thread notifies what was posted to it, so those
    /// notifications are not posted back
    static DRAINING: Cell<bool> = const { Cell::new(false) };
}

/// This thread tracks dependents: changes on other threads are posted to it
pub(super) fn register_current_thread() {
    REGISTRATION.with(|r| {
        if !r.registered.get() {
            crate::utils::lock::lock_or_recover(queues())
                .insert(thread::current().id(), Vec::new());
            TRACKING_THREADS.fetch_add(1, Ordering::SeqCst);
            r.registered.set(true);
        }
    });
}

/// Post a change of `signal` to every other thread that tracks dependents
pub(super) fn post(signal: SignalId) {
    if DRAINING.with(Cell::get) {
        return;
    }
    let here_registered = REGISTRATION.with(|r| r.registered.get());
    let others = TRACKING_THREADS.load(Ordering::SeqCst) - usize::from(here_registered);
    if others == 0 {
        return;
    }
    let here = thread::current().id();
    let mut queues = crate::utils::lock::lock_or_recover(queues());
    for (thread, queue) in queues.iter_mut() {
        if *thread != here && !queue.contains(&signal) {
            queue.push(signal);
        }
    }
}

/// Notify this thread's dependents of the signals changed on other threads
/// since the last drain. The app's event loop calls this every frame;
/// [`flush`](super::flush) and reading a computed value call it too.
pub(crate) fn drain() {
    if !REGISTRATION.with(|r| r.registered.get()) {
        return;
    }
    let posted = {
        let mut queues = crate::utils::lock::lock_or_recover(queues());
        match queues.get_mut(&thread::current().id()) {
            Some(queue) if !queue.is_empty() => std::mem::take(queue),
            _ => return,
        }
    };
    DRAINING.with(|d| d.set(true));
    struct Done;
    impl Drop for Done {
        fn drop(&mut self) {
            DRAINING.with(|d| d.set(false));
        }
    }
    let _done = Done;
    for signal in posted {
        super::tracker::notify_local_dependents(signal);
    }
}
