//! Computed (derived) values
//!
//! Thread-safe computed values using Arc and atomic operations.

use super::tracker::{start_tracking, stop_tracking, Subscriber, SubscriberId};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};

/// A derived value that automatically updates when dependencies change
///
/// Thread-safe: can be shared across threads.
pub struct Computed<T> {
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
        let _guard = self
            .recompute_lock
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

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
        let has_cache = self
            .cached
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .is_some();

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
        *self
            .cached
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(value.clone());
        self.dirty.store(false, Ordering::SeqCst);

        value
    }

    /// Get the cached value
    ///
    /// Returns the cached value if present, or None if cache is empty.
    /// This should only be called when `needs_recompute()` returns false,
    /// but handles the empty cache case gracefully.
    fn get_cached(&self) -> Option<T> {
        self.cached
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .as_ref()
            .cloned()
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
        Self {
            id: self.id,
            compute: self.compute.clone(),
            cached: self.cached.clone(),
            dirty: self.dirty.clone(),
            recompute_lock: self.recompute_lock.clone(),
        }
    }
}
