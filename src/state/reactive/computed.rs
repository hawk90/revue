//! Computed (derived) values
//!
//! Thread-safe computed values using Arc and atomic operations.

use super::tracker::{dispose_subscriber, start_tracking, stop_tracking, Subscriber, SubscriberId};
use crate::utils::lock::{lock_or_recover, read_or_recover, write_or_recover};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};

/// A derived value that automatically updates when dependencies change
///
/// Thread-safe: can be shared across threads.
pub struct Computed<T: Clone + Send + Sync + 'static> {
    /// Unique identifier for this computed value
    id: SubscriberId,
    /// The computation function
    compute: Arc<dyn Fn() -> T + Send + Sync>,
    /// Cached result
    cached: Arc<RwLock<Option<T>>>,
    /// Whether cache is invalid (shared via Arc for callbacks)
    dirty: Arc<AtomicBool>,
    /// Lock to prevent concurrent recomputation (avoids data race)
    recompute_lock: Arc<Mutex<()>>,
    /// Reference count for clones (for proper subscriber disposal)
    ref_count: Arc<AtomicUsize>,
}

// Computed<T> auto-derives Send when T: Send, Sync when T: Send + Sync.
// All fields are inherently thread-safe:
// - id: SubscriberId (Copy, u64)
// - compute: Arc<dyn Fn() -> T + Send + Sync> (Send+Sync)
// - cached: Arc<RwLock<Option<T>>> (Send when T: Send, Sync when T: Send+Sync)
// - dirty: Arc<AtomicBool> (Send+Sync)

impl<T: Clone + Send + Sync + 'static> Computed<T> {
    /// Create a new computed value
    pub fn new(f: impl Fn() -> T + Send + Sync + 'static) -> Self {
        let id = SubscriberId::new();
        let compute = Arc::new(f);

        Self {
            id,
            compute,
            cached: Arc::new(RwLock::new(None)),
            dirty: Arc::new(AtomicBool::new(true)),
            recompute_lock: Arc::new(Mutex::new(())),
            ref_count: Arc::new(AtomicUsize::new(1)),
        }
    }

    /// Get the computed value, using cache if available
    ///
    /// This automatically tracks dependencies during computation
    /// and invalidates when any dependency changes.
    ///
    /// Thread-safe: uses a lock to prevent concurrent recomputation.
    pub fn get(&self) -> T {
        // Fast path: check if we can use cached value without locking
        if !self.needs_recompute() {
            if let Some(value) = self.get_cached() {
                return value;
            }
            // Cache unexpectedly empty, fall through to recompute
        }

        // Slow path: acquire recompute lock to prevent data race
        // Multiple threads may reach here, but only one will recompute
        let _guard = lock_or_recover(&self.recompute_lock);

        // Double-check after acquiring lock: another thread may have computed
        if self.needs_recompute() {
            self.recompute_and_cache()
        } else {
            // Cache should exist here, but handle gracefully if not
            match self.get_cached() {
                Some(value) => value,
                None => self.recompute_and_cache(),
            }
        }
    }

    /// Check if recomputation is needed
    fn needs_recompute(&self) -> bool {
        let is_dirty = self.dirty.load(Ordering::SeqCst);
        let has_cache = read_or_recover(&self.cached).is_some();

        is_dirty || !has_cache
    }

    /// Recompute the value and cache it
    ///
    /// SAFETY: Caller must hold `recompute_lock` to prevent concurrent recomputation.
    fn recompute_and_cache(&self) -> T {
        // Create a subscriber that invalidates when dependencies change
        let dirty_flag = self.dirty.clone();
        let subscriber = Subscriber {
            id: self.id,
            callback: Arc::new(move || {
                dirty_flag.store(true, Ordering::SeqCst);
            }),
        };

        // Track dependencies during computation
        start_tracking(subscriber);
        let value = (self.compute)();
        stop_tracking();

        // Cache the result and mark as clean
        *write_or_recover(&self.cached) = Some(value.clone());
        self.dirty.store(false, Ordering::SeqCst);

        value
    }

    /// Get the cached value
    ///
    /// Returns the cached value if present, or None if cache is empty.
    /// This should only be called when `needs_recompute()` returns false,
    /// but handles the empty cache case gracefully.
    fn get_cached(&self) -> Option<T> {
        read_or_recover(&self.cached).as_ref().cloned()
    }

    /// Force recalculation on next get
    pub fn invalidate(&self) {
        self.dirty.store(true, Ordering::SeqCst);
    }

    /// Check if the value needs recalculation
    pub fn is_dirty(&self) -> bool {
        self.dirty.load(Ordering::SeqCst)
    }
}

impl<T: Clone + Send + Sync + 'static> Clone for Computed<T> {
    /// Clone creates a shared reference to the same computed value.
    /// Both clones share the same cache, dirty flag, and recompute lock.
    fn clone(&self) -> Self {
        // Increment reference count
        self.ref_count.fetch_add(1, Ordering::SeqCst);

        Self {
            id: self.id,
            compute: self.compute.clone(),
            cached: self.cached.clone(),
            dirty: self.dirty.clone(),
            recompute_lock: self.recompute_lock.clone(),
            ref_count: self.ref_count.clone(),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> Drop for Computed<T> {
    /// Dispose subscriber when the last clone is dropped.
    ///
    /// This prevents memory leaks in the dependency tracker by removing
    /// the subscriber callback when the Computed value is no longer used.
    fn drop(&mut self) {
        // Decrement reference count and check if this is the last clone
        if self.ref_count.fetch_sub(1, Ordering::SeqCst) == 1 {
            // This was the last reference, dispose the subscriber
            dispose_subscriber(self.id);
        }
    }
}
