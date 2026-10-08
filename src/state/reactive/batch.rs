//! Batched signal updates
//!
//! Inside a batch, signal values still change at once - reading one, or a
//! computed value derived from it, gives the new value - but what *reacts*
//! to a change waits: effects and [`Signal::subscribe`](super::Signal::subscribe)
//! callbacks are collected and run once each when the outermost batch ends.
//! An effect over two signals set in one batch runs once, and never sees one
//! updated and the other not.
//!
//! `SignalVec` diff subscribers are not deferred: each diff describes its
//! own change, so they are delivered as they happen.
//!
//! # Example
//!
//! ```
//! use revue::reactive::{batch, effect, signal};
//!
//! let first = signal("Ada".to_string());
//! let last = signal("Lovelace".to_string());
//! let shown = signal(Vec::new());
//! let _greeting = effect({
//!     let (first, last, shown) = (first.clone(), last.clone(), shown.clone());
//!     move || shown.update(|v| v.push(format!("{} {}", first.get(), last.get())))
//! });
//!
//! // Without a batch the effect runs after each set, and sees the mix
//! first.set("Grace".to_string());
//! last.set("Hopper".to_string());
//! assert_eq!(shown.get()[1], "Grace Lovelace");
//!
//! // In a batch it runs once, with both names
//! batch(|| {
//!     first.set("Alan".to_string());
//!     last.set("Turing".to_string());
//! });
//! assert_eq!(shown.get().last().unwrap(), "Alan Turing");
//! assert_eq!(shown.get().len(), 4);
//! ```

use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};

// =============================================================================
// Batch State
// =============================================================================

thread_local! {
    /// Batch depth counter for nested batches (per-thread)
    static BATCH_DEPTH: RefCell<usize> = const { RefCell::new(0) };

    /// Pending updates to flush (per-thread)
    static PENDING_UPDATES: RefCell<Vec<Box<dyn FnOnce()>>> = const { RefCell::new(Vec::new()) };
}

/// Counter for tracking batch operations (for debugging)
static BATCH_COUNTER: AtomicUsize = AtomicUsize::new(0);

// =============================================================================
// Batch API
// =============================================================================

/// Run `f` as a batch: effects and subscriptions it triggers run once,
/// after it returns
///
/// Signal values change immediately inside `f`; what reacts to them waits
/// until the outermost batch ends, runs the updates queued with
/// [`queue_update`], and then runs each deferred effect and subscription
/// once. Batches nest; only the outermost one flushes. `SignalVec` diff
/// subscribers are not deferred: each diff describes its own change.
///
/// If `f` panics the batch still ends. An outermost batch then discards
/// what it queued and deferred - nothing runs during the unwind - so an
/// effect may show the old state until its signals change again.
///
/// # Example
///
/// ```
/// use revue::reactive::{batch, effect, signal};
///
/// let (x, y) = (signal(0), signal(0));
/// let runs = signal(0);
/// let _sum = effect({
///     let (x, y, runs) = (x.clone(), y.clone(), runs.clone());
///     move || {
///         let _ = x.get() + y.get();
///         runs.update(|n| *n += 1);
///     }
/// });
///
/// let value = batch(|| {
///     x.set(1);
///     y.set(2);
///     assert_eq!(runs.get(), 1); // not yet
///     x.get() + y.get() // values are current
/// });
/// assert_eq!(value, 3);
/// assert_eq!(runs.get(), 2); // once for both changes
/// ```
pub fn batch<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // The guard ends the batch also when `f` panics.
    let _batch = BatchGuard::new();
    f()
}

/// Start a batch manually
///
/// This is useful when you need more control over batch boundaries.
/// Must be paired with `end_batch()`.
///
/// # Example
///
/// ```
/// use revue::reactive::{end_batch, is_batching, signal, start_batch};
///
/// let count = signal(0);
/// start_batch();
/// count.set(1); // effects on `count` wait for end_batch
/// assert!(is_batching());
/// end_batch();
/// assert!(!is_batching());
/// ```
pub fn start_batch() {
    BATCH_DEPTH.with(|depth| {
        *depth.borrow_mut() += 1;
    });
    BATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
}

/// End a batch manually
///
/// Flushes pending updates if this is the outermost batch.
pub fn end_batch() {
    let depth = batch_depth();
    if depth == 0 {
        return;
    }
    if depth > 1 {
        BATCH_DEPTH.with(|d| *d.borrow_mut() -= 1);
        return;
    }
    // The outermost batch. Its queued updates run while it is still open,
    // so what they change is deferred along with the rest; then the batch
    // closes and every deferred effect and subscription runs once. (The
    // depth borrow is released throughout: an update may queue another or
    // start a batch, which reads it.)
    {
        // If a queued update panics, close the batch anyway and drop what it
        // deferred, as a batch whose closure panics does.
        struct CloseOnUnwind;
        impl Drop for CloseOnUnwind {
            fn drop(&mut self) {
                if std::thread::panicking() {
                    BATCH_DEPTH.with(|d| *d.borrow_mut() = 0);
                    PENDING_UPDATES.with(|updates| updates.borrow_mut().clear());
                    super::tracker::discard_deferred();
                }
            }
        }
        let _close = CloseOnUnwind;
        flush_updates();
    }
    BATCH_DEPTH.with(|d| *d.borrow_mut() = 0);
    super::tracker::run_deferred();
}

/// Check if currently in a batch
///
/// This checks the thread-local batch depth, making batching a per-thread concept.
/// Each thread maintains its own independent batch state.
pub fn is_batching() -> bool {
    batch_depth() > 0
}

/// Get current batch depth (for debugging)
pub fn batch_depth() -> usize {
    BATCH_DEPTH.with(|depth| *depth.borrow())
}

/// Get total batch count (for debugging)
pub fn batch_count() -> usize {
    BATCH_COUNTER.load(Ordering::Relaxed)
}

/// Run what the current batch has held back so far, now
///
/// Runs the updates queued with [`queue_update`] and the effects and
/// subscriptions deferred so far, and keeps going until they stop
/// triggering each other. The batch stays open. Outside a batch nothing is
/// held back, so this does nothing.
///
/// # Panics
///
/// Panics if effects keep changing what they depend on for 100 rounds.
///
/// # Example
///
/// ```
/// use revue::reactive::{batch, effect, flush, signal};
///
/// let count = signal(0);
/// let seen = signal(0);
/// let _mirror = effect({
///     let (count, seen) = (count.clone(), seen.clone());
///     move || seen.set(count.get())
/// });
///
/// batch(|| {
///     count.set(1);
///     assert_eq!(seen.get(), 0); // deferred
///     flush();
///     assert_eq!(seen.get(), 1); // ran now, batch still open
///     count.set(2);
/// });
/// assert_eq!(seen.get(), 2);
/// ```
pub fn flush() {
    flush_updates();
    // Inside a batch, run what it deferred so far - and what those runs
    // defer in turn - until it settles.
    let mut rounds = 0;
    while super::tracker::has_deferred() {
        rounds += 1;
        assert!(
            rounds <= MAX_FLUSH_ROUNDS,
            "flush did not settle: effects kept changing what they depend on"
        );
        super::tracker::run_deferred();
        flush_updates();
    }
}

/// How many rounds of deferred effects `flush` runs before deciding the
/// effects feed each other in a loop.
const MAX_FLUSH_ROUNDS: usize = 100;

/// Queue an update to be executed when batch completes
///
/// If not in a batch, executes immediately.
pub fn queue_update<F: FnOnce() + 'static>(f: F) {
    if is_batching() {
        PENDING_UPDATES.with(|updates| {
            updates.borrow_mut().push(Box::new(f));
        });
    } else {
        f();
    }
}

/// Get number of pending updates
pub fn pending_count() -> usize {
    PENDING_UPDATES.with(|updates| updates.borrow().len())
}

// =============================================================================
// Internal
// =============================================================================

fn flush_updates() {
    // An update may queue another while the batch is still open: drain until
    // nothing is left.
    loop {
        let pending: Vec<_> =
            PENDING_UPDATES.with(|updates| updates.borrow_mut().drain(..).collect());
        if pending.is_empty() {
            return;
        }
        for update in pending {
            update();
        }
    }
}

// =============================================================================
// Transaction API
// =============================================================================

/// A transaction that can be committed or rolled back
///
/// Provides all-or-nothing semantics for signal updates.
///
/// # Example
///
/// ```
/// use revue::reactive::{signal, Transaction};
///
/// let balance = signal(100);
///
/// // Updates are recorded, not applied
/// let mut tx = Transaction::new();
/// let b = balance.clone();
/// tx.update(move || b.update(|v| *v -= 50));
/// assert_eq!(balance.get(), 100);
///
/// tx.commit(); // apply all updates, as one batch
/// assert_eq!(balance.get(), 50);
///
/// let mut tx = Transaction::new();
/// let b = balance.clone();
/// tx.update(move || b.set(0));
/// tx.rollback(); // discard all updates
/// assert_eq!(balance.get(), 50);
/// ```
pub struct Transaction {
    updates: Vec<Box<dyn FnOnce()>>,
    committed: bool,
}

impl Transaction {
    /// Create a new transaction
    pub fn new() -> Self {
        Self {
            updates: Vec::new(),
            committed: false,
        }
    }

    /// Add an update to the transaction
    pub fn update<F: FnOnce() + 'static>(&mut self, f: F) {
        self.updates.push(Box::new(f));
    }

    /// Commit the transaction (apply all updates in a batch)
    pub fn commit(mut self) {
        self.committed = true;
        batch(|| {
            for update in self.updates.drain(..) {
                update();
            }
        });
    }

    /// Rollback the transaction (discard all updates)
    pub fn rollback(mut self) {
        self.updates.clear();
    }

    /// Check if transaction has pending updates
    pub fn is_empty(&self) -> bool {
        self.updates.is_empty()
    }

    /// Get number of pending updates
    pub fn len(&self) -> usize {
        self.updates.len()
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        // If not committed, updates are discarded
        if !self.committed && !self.updates.is_empty() {
            // Log warning in debug mode
            #[cfg(debug_assertions)]
            eprintln!(
                "Warning: Transaction dropped without commit ({} updates discarded)",
                self.updates.len()
            );
        }
    }
}

// =============================================================================
// Batch Guard
// =============================================================================

/// RAII guard for batch scope
///
/// Automatically starts a batch when created and ends it when dropped, like
/// [`batch`]. When it is dropped by a panic unwinding, the batch ends without
/// flushing, and an outermost batch discards the updates it queued and the
/// effects it deferred.
///
/// # Example
///
/// ```
/// use revue::reactive::{effect, signal, BatchGuard};
///
/// let (a, b) = (signal(0), signal(0));
/// let runs = signal(0);
/// let _e = effect({
///     let (a, b, runs) = (a.clone(), b.clone(), runs.clone());
///     move || {
///         let _ = (a.get(), b.get());
///         runs.update(|n| *n += 1);
///     }
/// });
/// {
///     let _guard = BatchGuard::new();
///     a.set(1);
///     b.set(2);
/// } // Batch ends here: the effect runs once
/// assert_eq!(runs.get(), 2);
/// ```
pub struct BatchGuard {
    _private: (),
}

impl BatchGuard {
    /// Create a new batch guard (starts batch)
    pub fn new() -> Self {
        start_batch();
        Self { _private: () }
    }
}

impl Default for BatchGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for BatchGuard {
    fn drop(&mut self) {
        if std::thread::panicking() {
            // Unwinding: leave the batch, but do not run its updates now -
            // one that panicked too would abort the process. An outermost
            // batch that did not complete drops what it queued.
            let outermost = BATCH_DEPTH.with(|depth| {
                let mut d = depth.borrow_mut();
                *d = d.saturating_sub(1);
                *d == 0
            });
            if outermost {
                PENDING_UPDATES.with(|updates| updates.borrow_mut().clear());
                super::tracker::discard_deferred();
            }
        } else {
            end_batch();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_panicking_batch_still_ends() {
        let r = std::panic::catch_unwind(|| {
            batch(|| {
                queue_update(|| panic!("must not run during the unwind"));
                panic!("batch boom");
            })
        });
        assert!(r.is_err());
        assert!(!is_batching(), "the thread stayed in the batch");
        assert_eq!(pending_count(), 0);

        // Updates run immediately again.
        let ran = std::rc::Rc::new(std::cell::Cell::new(false));
        let r2 = ran.clone();
        queue_update(move || r2.set(true));
        assert!(ran.get());
    }

    #[test]
    fn a_flushed_update_can_queue_another() {
        use std::cell::Cell;
        use std::rc::Rc;
        let ran = Rc::new(Cell::new(0));
        let r = ran.clone();
        batch(|| {
            queue_update(move || {
                let r2 = r.clone();
                queue_update(move || r2.set(r2.get() + 1));
                batch(|| ());
            });
        });
        assert_eq!(ran.get(), 1);
        assert!(!is_batching());
    }
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    // batch function tests
    #[test]
    fn test_batch_basic() {
        let mut executed = false;
        batch(|| {
            executed = true;
        });
        assert!(executed);
    }

    #[test]
    fn test_batch_return_value() {
        let result = batch(|| 42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_batch_nested() {
        let mut depth = 0;
        batch(|| {
            depth = batch_depth();
            assert!(depth >= 1);

            batch(|| {
                let inner_depth = batch_depth();
                assert!(inner_depth > depth);
            });
        });
    }

    // start_batch/end_batch tests
    #[test]
    fn test_start_end_batch() {
        assert_eq!(batch_depth(), 0);
        assert!(!is_batching());

        start_batch();
        assert_eq!(batch_depth(), 1);
        assert!(is_batching());

        end_batch();
        assert_eq!(batch_depth(), 0);
        assert!(!is_batching());
    }

    #[test]
    fn test_nested_start_end_batch() {
        start_batch();
        assert_eq!(batch_depth(), 1);

        start_batch();
        assert_eq!(batch_depth(), 2);

        end_batch();
        assert_eq!(batch_depth(), 1);

        end_batch();
        assert_eq!(batch_depth(), 0);
    }

    // batch_depth tests
    #[test]
    fn test_batch_depth_initial() {
        assert_eq!(batch_depth(), 0);
    }

    #[test]
    fn test_batch_depth_single_batch() {
        batch(|| {
            assert_eq!(batch_depth(), 1);
        });
        assert_eq!(batch_depth(), 0);
    }

    #[test]
    fn test_batch_depth_nested_batches() {
        batch(|| {
            assert_eq!(batch_depth(), 1);
            batch(|| {
                assert_eq!(batch_depth(), 2);
                batch(|| {
                    assert_eq!(batch_depth(), 3);
                });
                assert_eq!(batch_depth(), 2);
            });
            assert_eq!(batch_depth(), 1);
        });
        assert_eq!(batch_depth(), 0);
    }

    // is_batching tests
    #[test]
    fn test_is_batching_false_initially() {
        assert!(!is_batching());
    }

    #[test]
    fn test_is_batching_true_in_batch() {
        batch(|| {
            assert!(is_batching());
        });
        assert!(!is_batching());
    }

    // batch_count tests
    #[test]
    fn test_batch_count_increments() {
        let count_before = batch_count();
        batch(|| {});
        assert!(batch_count() > count_before);
    }

    // flush tests
    #[test]
    fn test_flush_does_not_panic() {
        flush();
        start_batch();
        flush();
        end_batch();
    }

    // queue_update tests
    #[test]
    fn test_queue_update_outside_batch() {
        let executed = Arc::new(AtomicBool::new(false));
        let executed_clone = executed.clone();
        queue_update(move || {
            executed_clone.store(true, Ordering::SeqCst);
        });
        assert!(
            executed.load(Ordering::SeqCst),
            "Should execute immediately when not batching"
        );
    }

    #[test]
    fn test_queue_update_inside_batch() {
        let executed = Arc::new(AtomicBool::new(false));
        let executed_clone = executed.clone();
        batch(|| {
            queue_update(move || {
                executed_clone.store(true, Ordering::SeqCst);
            });
            // Should not execute yet
            assert!(!executed.load(Ordering::SeqCst));
        });
        // Should execute after batch ends
        assert!(executed.load(Ordering::SeqCst));
    }

    // pending_count tests
    #[test]
    fn test_pending_count_outside_batch() {
        assert_eq!(pending_count(), 0);
    }

    #[test]
    fn test_pending_count_inside_batch() {
        batch(|| {
            queue_update(|| {});
            assert_eq!(pending_count(), 1);

            queue_update(|| {});
            assert_eq!(pending_count(), 2);

            flush();
            assert_eq!(pending_count(), 0);
        });
    }

    // Transaction tests
    #[test]
    fn test_transaction_new() {
        let tx = Transaction::new();
        assert!(tx.is_empty());
        assert_eq!(tx.len(), 0);
    }

    #[test]
    fn test_transaction_default() {
        let tx = Transaction::default();
        assert!(tx.is_empty());
    }

    #[test]
    fn test_transaction_update() {
        let mut tx = Transaction::new();
        assert_eq!(tx.len(), 0);

        tx.update(|| {});
        assert_eq!(tx.len(), 1);

        tx.update(|| {});
        assert_eq!(tx.len(), 2);

        assert!(!tx.is_empty());
    }

    #[test]
    fn test_transaction_commit() {
        let executed = Arc::new(AtomicBool::new(false));
        let executed_clone = executed.clone();
        let mut tx = Transaction::new();
        tx.update(move || {
            executed_clone.store(true, Ordering::SeqCst);
        });
        assert!(!executed.load(Ordering::SeqCst));

        tx.commit();
        assert!(executed.load(Ordering::SeqCst));
    }

    #[test]
    fn test_transaction_rollback() {
        let executed = Arc::new(AtomicBool::new(false));
        let executed_clone = executed.clone();
        let mut tx = Transaction::new();
        tx.update(move || {
            executed_clone.store(true, Ordering::SeqCst);
        });

        tx.rollback();
        assert!(
            !executed.load(Ordering::SeqCst),
            "Updates should be discarded"
        );
        // After rollback, tx is moved and cannot be accessed
    }

    #[test]
    fn test_transaction_len() {
        let mut tx = Transaction::new();
        assert_eq!(tx.len(), 0);

        tx.update(|| {});
        tx.update(|| {});
        tx.update(|| {});

        assert_eq!(tx.len(), 3);
    }

    #[test]
    fn test_transaction_is_empty() {
        let mut tx = Transaction::new();
        assert!(tx.is_empty());

        tx.update(|| {});
        assert!(!tx.is_empty());
    }

    #[test]
    fn test_transaction_commit_empties() {
        let mut tx = Transaction::new();
        tx.update(|| {});
        tx.update(|| {});

        assert_eq!(tx.len(), 2);
        tx.commit();
        // After commit, tx is moved and cannot be accessed
        // The commit method calls batch() which drains updates
    }

    // BatchGuard tests
    #[test]
    fn test_batch_guard_new() {
        assert_eq!(batch_depth(), 0);

        {
            let _guard = BatchGuard::new();
            assert_eq!(batch_depth(), 1);
            assert!(is_batching());
        }

        assert_eq!(batch_depth(), 0);
    }

    #[test]
    fn test_batch_guard_default() {
        assert_eq!(batch_depth(), 0);

        {
            let _guard = BatchGuard::default();
            assert_eq!(batch_depth(), 1);
        }

        assert_eq!(batch_depth(), 0);
    }

    #[test]
    fn test_batch_guard_nested() {
        assert_eq!(batch_depth(), 0);

        {
            let _guard1 = BatchGuard::new();
            assert_eq!(batch_depth(), 1);

            {
                let _guard2 = BatchGuard::new();
                assert_eq!(batch_depth(), 2);
            }

            assert_eq!(batch_depth(), 1);
        }

        assert_eq!(batch_depth(), 0);
    }

    // Integration tests
    #[test]
    fn test_batch_with_queue_update() {
        use std::sync::Mutex;
        let results = Arc::new(Mutex::new(Vec::new()));
        batch(|| {
            let r1 = results.clone();
            queue_update(move || {
                r1.lock().unwrap().push(1);
            });
            let r2 = results.clone();
            queue_update(move || {
                r2.lock().unwrap().push(2);
            });
            let r3 = results.clone();
            queue_update(move || {
                r3.lock().unwrap().push(3);
            });
        });

        // Updates should execute
        let results_vec = results.lock().unwrap();
        assert_eq!(results_vec.len(), 3);
    }

    #[test]
    fn test_flush_inside_batch() {
        let executed = Arc::new(AtomicBool::new(false));
        let executed_clone = executed.clone();
        batch(|| {
            queue_update(move || {
                executed_clone.store(true, Ordering::SeqCst);
            });
            assert!(!executed.load(Ordering::SeqCst));

            flush();
            assert!(executed.load(Ordering::SeqCst));
        });
    }

    #[test]
    fn test_transaction_commit_in_batch() {
        let executed = Arc::new(AtomicBool::new(false));
        let executed_clone = executed.clone();
        batch(|| {
            let mut tx = Transaction::new();
            tx.update(move || {
                executed_clone.store(true, Ordering::SeqCst);
            });
            tx.commit();
        });
        assert!(executed.load(Ordering::SeqCst));
    }
}
