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

#[cfg(test)]
mod tests {
    use super::*;

    // Computed::new tests
    #[test]
    fn test_computed_new() {
        let c = Computed::new(|| 42);
        assert_eq!(c.get(), 42);
    }

    #[test]
    fn test_computed_new_string() {
        let c = Computed::new(|| "hello".to_string());
        assert_eq!(c.get(), "hello");
    }

    #[test]
    fn test_computed_new_vec() {
        let c = Computed::new(|| vec![1, 2, 3]);
        assert_eq!(c.get(), vec![1, 2, 3]);
    }

    #[test]
    fn test_computed_new_closure_with_captures() {
        let x = 10;
        let c = Computed::new(move || x * 2);
        assert_eq!(c.get(), 20);
    }

    // Computed::get tests
    #[test]
    fn test_computed_get_basic() {
        let c = Computed::new(|| 42);
        let result = c.get();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_computed_get_multiple_calls() {
        let c = Computed::new(|| 42);
        assert_eq!(c.get(), 42);
        assert_eq!(c.get(), 42);
        assert_eq!(c.get(), 42);
    }

    #[test]
    fn test_computed_get_returns_cloned_value() {
        let c = Computed::new(|| vec![1, 2, 3]);
        let v1 = c.get();
        let v2 = c.get();
        // Each get should return a clone of the cached value
        assert_eq!(v1, vec![1, 2, 3]);
        assert_eq!(v2, vec![1, 2, 3]);
    }

    // Computed::invalidate tests
    #[test]
    fn test_computed_invalidate() {
        let c = Computed::new(|| 42);
        // First get should compute
        assert_eq!(c.get(), 42);

        // Invalidate the cache
        c.invalidate();
        assert!(c.is_dirty());

        // Next get should recompute
        assert_eq!(c.get(), 42);
    }

    #[test]
    fn test_computed_invalidate_multiple() {
        let c = Computed::new(|| 42);
        c.invalidate();
        c.invalidate();
        c.invalidate();
        assert!(c.is_dirty());
    }

    // Computed::is_dirty tests
    #[test]
    fn test_computed_is_dirty_initially() {
        let c = Computed::new(|| 42);
        assert!(c.is_dirty(), "Should be dirty initially (cache empty)");
    }

    #[test]
    fn test_computed_is_dirty_after_get() {
        let c = Computed::new(|| 42);
        c.get();
        assert!(!c.is_dirty(), "Should be clean after first get");
    }

    #[test]
    fn test_computed_is_dirty_after_invalidate() {
        let c = Computed::new(|| 42);
        c.get();
        assert!(!c.is_dirty());

        c.invalidate();
        assert!(c.is_dirty());
    }

    // Computed::clone tests
    #[test]
    fn test_computed_clone_shares_cache() {
        let c1 = Computed::new(|| 42);
        // Get from first instance
        assert_eq!(c1.get(), 42);

        // Clone should share the same cache
        let c2 = c1.clone();
        assert_eq!(c2.get(), 42);
    }

    #[test]
    fn test_computed_clone_invalidate_affects_both() {
        let c1 = Computed::new(|| 42);
        c1.get();

        let c2 = c1.clone();
        // Invalidate c1 should affect c2 as well (same cache)
        c1.invalidate();
        assert!(c2.is_dirty());
    }

    // Computed with different types
    #[test]
    fn test_computed_with_i32() {
        let c = Computed::new(|| 123_i32);
        assert_eq!(c.get(), 123);
    }

    #[test]
    fn test_computed_with_u64() {
        let c = Computed::new(|| 999_u64);
        assert_eq!(c.get(), 999);
    }

    #[test]
    fn test_computed_with_f64() {
        let c = Computed::new(|| 3.14_f64);
        assert!((c.get() - 3.14).abs() < 0.001);
    }

    #[test]
    fn test_computed_with_bool() {
        let c = Computed::new(|| true);
        assert!(c.get());
    }

    #[test]
    fn test_computed_with_option() {
        let c = Computed::new(|| Some(42));
        assert_eq!(c.get(), Some(42));
    }

    #[test]
    fn test_computed_with_result() {
        let c = Computed::new(|| Ok::<i32, &str>(42));
        assert_eq!(c.get(), Ok(42));
    }

    // Computed with complex computation
    #[test]
    fn test_computed_with_complex_calculation() {
        let c = Computed::new(|| {
            let mut sum = 0;
            for i in 1..=100 {
                sum += i;
            }
            sum
        });
        assert_eq!(c.get(), 5050);
    }

    #[test]
    fn test_computed_with_string_concat() {
        let c = Computed::new(|| {
            let mut s = String::new();
            for i in 1..=5 {
                s.push_str(&i.to_string());
            }
            s
        });
        assert_eq!(c.get(), "12345");
    }

    // Thread-safety tests (basic)
    #[test]
    fn test_computed_send_sync() {
        // This test just verifies that Computed implements Send and Sync
        fn is_send_sync<T: Send + Sync>() {}
        is_send_sync::<Computed<i32>>();
    }
}
