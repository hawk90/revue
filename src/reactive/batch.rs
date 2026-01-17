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
