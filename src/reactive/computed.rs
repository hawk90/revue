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
            return self.get_cached();
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
            self.get_cached()
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
    /// Returns the cached value if present, or panics if cache is empty.
    /// This should only be called when `needs_recompute()` returns false.
    fn get_cached(&self) -> T {
        self.cached
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .as_ref()
            .expect("get_cached called but cache is empty; this is a bug in Computed")
            .clone()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn test_computed_basic() {
        let computed = Computed::new(|| 42);
        assert_eq!(computed.get(), 42);
    }

    #[test]
    fn test_computed_with_closure() {
        let multiplier = 3;
        let computed = Computed::new(move || 10 * multiplier);
        assert_eq!(computed.get(), 30);
    }

    #[test]
    fn test_computed_caching() {
        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = call_count.clone();

        let computed = Computed::new(move || {
            call_count_clone.fetch_add(1, Ordering::SeqCst);
            42
        });

        // First call computes
        assert_eq!(computed.get(), 42);
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        // Second call uses cache
        assert_eq!(computed.get(), 42);
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        // After invalidation, recomputes
        computed.invalidate();
        assert_eq!(computed.get(), 42);
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_computed_dirty_flag() {
        let computed = Computed::new(|| 1);

        // Initially dirty
        assert!(computed.is_dirty());

        // After get, not dirty
        computed.get();
        assert!(!computed.is_dirty());

        // After invalidate, dirty again
        computed.invalidate();
        assert!(computed.is_dirty());
    }

    #[test]
    fn test_computed_auto_invalidation() {
        use crate::reactive::signal;

        let source = signal(10);
        let compute_count = Arc::new(AtomicUsize::new(0));

        let cc = compute_count.clone();
        let s = source.clone();
        let computed = Computed::new(move || {
            cc.fetch_add(1, Ordering::SeqCst);
            s.get() * 2
        });

        // First access computes
        assert_eq!(computed.get(), 20);
        assert_eq!(compute_count.load(Ordering::SeqCst), 1);

        // Multiple accesses use cache
        assert_eq!(computed.get(), 20);
        assert_eq!(computed.get(), 20);
        assert_eq!(compute_count.load(Ordering::SeqCst), 1);

        // Update source - should AUTO-INVALIDATE computed!
        source.set(20);

        // Computed should be dirty now
        assert!(computed.is_dirty());

        // Next access recomputes automatically
        assert_eq!(computed.get(), 40);
        assert_eq!(compute_count.load(Ordering::SeqCst), 2);

        // Cached again
        assert_eq!(computed.get(), 40);
        assert_eq!(compute_count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_computed_dynamic_dependencies() {
        use crate::reactive::signal;

        let flag = signal(true);
        let a = signal(1);
        let b = signal(2);

        let f = flag.clone();
        let a_c = a.clone();
        let b_c = b.clone();

        let computed = Computed::new(move || if f.get() { a_c.get() } else { b_c.get() });

        // Initially: flag=true, depends on a
        assert_eq!(computed.get(), 1);

        // Change a - should invalidate
        a.set(10);
        assert_eq!(computed.get(), 10);

        // Change flag to false - now depends on b
        flag.set(false);
        assert_eq!(computed.get(), 2);

        // Change b - should invalidate
        b.set(20);
        assert_eq!(computed.get(), 20);

        // Change a - should NOT affect (we don't depend on it anymore)
        // Note: Due to re-tracking on every get(), this will work correctly
        a.set(100);
        assert_eq!(computed.get(), 20); // Still 20, not affected by a
    }

    #[test]
    fn test_computed_thread_safety() {
        use std::thread;

        let computed = Arc::new(Computed::new(|| 42));

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let c = computed.clone();
                thread::spawn(move || {
                    for _ in 0..100 {
                        assert_eq!(c.get(), 42);
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }
    }

    #[test]
    fn test_computed_no_data_race_on_concurrent_recompute() {
        use std::thread;
        use std::time::Duration;

        // This test verifies that concurrent calls to get() on a dirty computed
        // do not cause a data race (double recomputation with inconsistent results).
        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = call_count.clone();

        // Computation that takes some time and increments counter
        let computed = Arc::new(Computed::new(move || {
            call_count_clone.fetch_add(1, Ordering::SeqCst);
            // Small delay to increase chance of race condition
            thread::sleep(Duration::from_micros(100));
            42
        }));

        // Spawn multiple threads that all try to get() simultaneously
        // when the computed is dirty
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let c = computed.clone();
                thread::spawn(move || c.get())
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All threads should get the same value
        assert!(results.iter().all(|&v| v == 42));

        // The computation should only have run ONCE despite 8 concurrent callers
        // (the recompute_lock ensures only one thread recomputes)
        assert_eq!(
            call_count.load(Ordering::SeqCst),
            1,
            "Computation should run exactly once, but ran {} times",
            call_count.load(Ordering::SeqCst)
        );
    }

    #[test]
    fn test_computed_recomputes_after_invalidation_with_contention() {
        use std::thread;

        let call_count = Arc::new(AtomicUsize::new(0));
        let call_count_clone = call_count.clone();

        let computed = Arc::new(Computed::new(move || {
            call_count_clone.fetch_add(1, Ordering::SeqCst)
        }));

        // First get: computes once
        let v1 = computed.get();
        assert_eq!(v1, 0);
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        // Multiple gets without invalidation: uses cache
        for _ in 0..10 {
            assert_eq!(computed.get(), 0);
        }
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        // Invalidate and get from multiple threads
        computed.invalidate();

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let c = computed.clone();
                thread::spawn(move || c.get())
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All should get the new value (1, since call_count was incremented)
        assert!(results.iter().all(|&v| v == 1));

        // Should have computed exactly twice total (initial + after invalidation)
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }
}
