//! Batched signal updates
//!
//! Batch multiple signal updates into a single render cycle for better performance.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::reactive::{signal, batch};
//!
//! let count = signal(0);
//! let name = signal(String::new());
//!
//! // Without batching: triggers 2 re-renders
//! count.set(1);
//! name.set("Alice".to_string());
//!
//! // With batching: triggers only 1 re-render
//! batch(|| {
//!     count.set(1);
//!     name.set("Alice".to_string());
//! });
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

/// Execute a function with batched updates
///
/// All signal updates within the closure are deferred until the batch completes.
/// This prevents intermediate re-renders and improves performance.
///
/// Batches can be nested - updates are only flushed when the outermost batch completes.
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{signal, batch};
///
/// let x = signal(0);
/// let y = signal(0);
/// let z = signal(0);
///
/// batch(|| {
///     x.set(1);
///     y.set(2);
///     z.set(3);
///     // No re-renders happen yet
/// });
/// // Single re-render happens here
/// ```
pub fn batch<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    start_batch();
    let result = f();
    end_batch();
    result
}

/// Start a batch manually
///
/// This is useful when you need more control over batch boundaries.
/// Must be paired with `end_batch()`.
///
/// # Example
///
/// ```rust,ignore
/// start_batch();
/// // ... do updates ...
/// end_batch();
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
    BATCH_DEPTH.with(|depth| {
        let mut d = depth.borrow_mut();
        *d = d.saturating_sub(1);

        if *d == 0 {
            flush_updates();
        }
    });
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

/// Force flush all pending updates immediately
///
/// Use this when you need to ensure updates are applied synchronously.
///
/// # Example
///
/// ```rust,ignore
/// batch(|| {
///     count.set(1);
///     flush(); // Force update now
///     // count is now definitely 1
///     count.set(2);
/// });
/// ```
pub fn flush() {
    flush_updates();
}

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
    PENDING_UPDATES.with(|updates| {
        let pending: Vec<_> = updates.borrow_mut().drain(..).collect();
        for update in pending {
            update();
        }
    });
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
/// ```rust,ignore
/// use revue::reactive::{signal, Transaction};
///
/// let balance = signal(100);
/// let error = signal(None);
///
/// let mut tx = Transaction::new();
/// tx.update(|| balance.update(|b| *b -= 50));
/// tx.update(|| {
///     if balance.get() < 0 {
///         error.set(Some("Insufficient funds".to_string()));
///         return Err(());
///     }
///     Ok(())
/// });
///
/// if some_condition {
///     tx.commit(); // Apply all updates
/// } else {
///     tx.rollback(); // Discard all updates
/// }
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
/// Automatically starts a batch when created and ends it when dropped.
///
/// # Example
///
/// ```rust,ignore
/// {
///     let _guard = BatchGuard::new();
///     signal1.set(1);
///     signal2.set(2);
/// } // Batch ends here, updates are flushed
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
        end_batch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
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
