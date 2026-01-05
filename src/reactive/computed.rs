//! Computed (derived) values
//!
//! Thread-safe computed values using Arc and atomic operations.

use super::tracker::{start_tracking, stop_tracking, Subscriber, SubscriberId};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

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
        }
    }

    /// Get the computed value, using cache if available
    ///
    /// This automatically tracks dependencies during computation
    /// and invalidates when any dependency changes.
    pub fn get(&self) -> T {
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

    /// Get the cached value (assumes cache exists)
    fn get_cached(&self) -> T {
        self.cached
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .as_ref()
            .unwrap()
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
    /// Both clones share the same cache and dirty flag.
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            compute: self.compute.clone(),
            cached: self.cached.clone(),
            dirty: self.dirty.clone(),
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
}
