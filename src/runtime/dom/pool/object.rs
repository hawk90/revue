//! Generic object pools and the RAII guard

use std::cell::RefCell;
use std::sync::Mutex;

use super::PoolStats;
use crate::utils::lock::lock_or_recover;

/// A generic object pool for reusing allocations
///
/// Objects are created with a factory function and returned to the pool
/// when no longer needed.
pub struct ObjectPool<T> {
    /// Factory for creating new objects
    factory: Box<dyn Fn() -> T>,
    /// Pooled objects ready for reuse
    pool: RefCell<Vec<T>>,
    /// Maximum pool size
    max_size: usize,
    /// Statistics
    stats: RefCell<PoolStats>,
}

impl<T> ObjectPool<T> {
    /// Create a new object pool
    pub fn new<F>(factory: F) -> Self
    where
        F: Fn() -> T + 'static,
    {
        Self::with_capacity(factory, 16)
    }

    /// Create a pool with specific capacity
    pub fn with_capacity<F>(factory: F, max_size: usize) -> Self
    where
        F: Fn() -> T + 'static,
    {
        Self {
            factory: Box::new(factory),
            pool: RefCell::new(Vec::with_capacity(max_size)),
            max_size,
            stats: RefCell::new(PoolStats::default()),
        }
    }

    /// Acquire an object from the pool
    pub fn acquire(&self) -> T {
        let mut stats = self.stats.borrow_mut();
        stats.acquires += 1;

        if let Some(obj) = self.pool.borrow_mut().pop() {
            stats.hits += 1;
            obj
        } else {
            stats.misses += 1;
            (self.factory)()
        }
    }

    /// Release an object back to the pool
    pub fn release(&self, obj: T) {
        let mut stats = self.stats.borrow_mut();
        stats.releases += 1;

        let mut pool = self.pool.borrow_mut();
        if pool.len() < self.max_size {
            pool.push(obj);
        } else {
            stats.discards += 1;
            // Object is dropped
        }
    }

    /// Get current pool size
    pub fn size(&self) -> usize {
        self.pool.borrow().len()
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        self.stats.borrow().clone()
    }

    /// Clear the pool
    pub fn clear(&self) {
        self.pool.borrow_mut().clear();
    }

    /// Pre-warm the pool with objects
    pub fn prewarm(&self, count: usize) {
        let target = count.min(self.max_size);
        let mut pool = self.pool.borrow_mut();
        while pool.len() < target {
            pool.push((self.factory)());
        }
    }
}

/// Thread-safe object pool
pub struct SyncObjectPool<T> {
    /// Factory for creating new objects
    factory: Box<dyn Fn() -> T + Send + Sync>,
    /// Pooled objects
    pool: Mutex<Vec<T>>,
    /// Maximum pool size
    max_size: usize,
    /// Statistics
    stats: Mutex<PoolStats>,
}

impl<T: Send> SyncObjectPool<T> {
    /// Create a new thread-safe pool
    pub fn new<F>(factory: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self::with_capacity(factory, 16)
    }

    /// Create with specific capacity
    pub fn with_capacity<F>(factory: F, max_size: usize) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            factory: Box::new(factory),
            pool: Mutex::new(Vec::with_capacity(max_size)),
            max_size,
            stats: Mutex::new(PoolStats::default()),
        }
    }

    /// Acquire an object
    pub fn acquire(&self) -> T {
        let mut stats = lock_or_recover(&self.stats);
        stats.acquires += 1;

        if let Some(obj) = lock_or_recover(&self.pool).pop() {
            stats.hits += 1;
            obj
        } else {
            stats.misses += 1;
            (self.factory)()
        }
    }

    /// Release an object
    pub fn release(&self, obj: T) {
        let mut stats = lock_or_recover(&self.stats);
        stats.releases += 1;

        let mut pool = lock_or_recover(&self.pool);
        if pool.len() < self.max_size {
            pool.push(obj);
        } else {
            stats.discards += 1;
        }
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        lock_or_recover(&self.stats).clone()
    }
}

/// RAII guard for pooled objects
///
/// Automatically returns the object to the pool when dropped.
pub struct Pooled<'a, T> {
    /// The pooled object
    value: Option<T>,
    /// Reference to the pool
    pool: &'a ObjectPool<T>,
}

impl<'a, T> Pooled<'a, T> {
    /// Create a new pooled guard
    pub fn new(pool: &'a ObjectPool<T>) -> Self {
        Self {
            value: Some(pool.acquire()),
            pool,
        }
    }

    /// Take the value, preventing automatic release
    pub fn take(mut self) -> T {
        self.value.take().unwrap_or_else(|| {
            panic!("Pooled value already taken - this is a bug in Pooled implementation")
        })
    }
}

impl<T> std::ops::Deref for Pooled<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // value is always Some until take() is called, which consumes self
        self.value.as_ref().unwrap_or_else(|| {
            panic!("Pooled value is None - deref called after take(), this is a bug")
        })
    }
}

impl<T> std::ops::DerefMut for Pooled<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // value is always Some until take() is called, which consumes self
        self.value.as_mut().unwrap_or_else(|| {
            panic!("Pooled value is None - deref_mut called after take(), this is a bug")
        })
    }
}

impl<T> Drop for Pooled<'_, T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            self.pool.release(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_object_pool_basic() {
        let pool: ObjectPool<Vec<u8>> = ObjectPool::new(|| Vec::with_capacity(64));

        // First acquire creates new
        let mut v1 = pool.acquire();
        assert!(v1.capacity() >= 64);
        v1.push(1);
        v1.push(2);

        // Release back to pool
        pool.release(v1);
        assert_eq!(pool.size(), 1);

        // Second acquire reuses
        let v2 = pool.acquire();
        assert!(v2.capacity() >= 64);
        // Note: contents are NOT cleared by default - caller should clear if needed
        assert_eq!(pool.size(), 0);

        let stats = pool.stats();
        assert_eq!(stats.acquires, 2);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hits, 1);
    }

    #[test]
    fn test_object_pool_max_size() {
        let pool: ObjectPool<u32> = ObjectPool::with_capacity(|| 0, 2);

        pool.release(1);
        pool.release(2);
        pool.release(3); // Should be discarded

        assert_eq!(pool.size(), 2);
        assert_eq!(pool.stats().discards, 1);
    }

    #[test]
    fn test_object_pool_prewarm() {
        let pool: ObjectPool<String> = ObjectPool::with_capacity(|| String::with_capacity(32), 10);
        pool.prewarm(5);

        assert_eq!(pool.size(), 5);

        // Acquiring should use pre-warmed objects
        let _ = pool.acquire();
        assert_eq!(pool.size(), 4);
    }

    #[test]
    fn test_pooled_guard() {
        let pool: ObjectPool<String> = ObjectPool::new(|| String::with_capacity(32));

        {
            let mut s = Pooled::new(&pool);
            s.push_str("hello");
            assert_eq!(&*s, "hello");
        } // Dropped, returned to pool

        assert_eq!(pool.size(), 1);
    }

    #[test]
    fn test_pooled_take() {
        let pool: ObjectPool<String> = ObjectPool::new(|| String::with_capacity(32));

        let s = {
            let mut pooled = Pooled::new(&pool);
            pooled.push_str("hello");
            pooled.take() // Take ownership
        };

        assert_eq!(s, "hello");
        assert_eq!(pool.size(), 0); // Not returned to pool
    }

    #[test]
    fn test_sync_object_pool() {
        use std::thread;

        let pool: Arc<SyncObjectPool<Vec<u8>>> =
            Arc::new(SyncObjectPool::new(|| Vec::with_capacity(64)));

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let pool = pool.clone();
                thread::spawn(move || {
                    for _ in 0..10 {
                        let v = pool.acquire();
                        pool.release(v);
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        let stats = pool.stats();
        assert_eq!(stats.acquires, 40);
        assert_eq!(stats.releases, 40);
    }
}
