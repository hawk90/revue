//! Object pooling for memory optimization
//!
//! Provides reusable object pools to reduce allocation overhead in hot paths.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::dom::pool::{ObjectPool, BufferPool};
//!
//! // Generic object pool
//! let pool: ObjectPool<Vec<u8>> = ObjectPool::new(|| Vec::with_capacity(1024));
//! let mut buffer = pool.acquire();
//! buffer.extend_from_slice(b"Hello");
//! pool.release(buffer);
//!
//! // Buffer pool for render buffers
//! let mut buffer_pool = BufferPool::new();
//! let buf = buffer_pool.acquire(80, 24);
//! // ... use buffer ...
//! buffer_pool.release(buf);
//! ```

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::render::Buffer;
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

/// Statistics for pool usage
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    /// Total acquisitions
    pub acquires: usize,
    /// Cache hits (reused from pool)
    pub hits: usize,
    /// Cache misses (new allocation)
    pub misses: usize,
    /// Total releases
    pub releases: usize,
    /// Objects discarded (pool full)
    pub discards: usize,
}

impl PoolStats {
    /// Hit rate (0.0 - 1.0)
    pub fn hit_rate(&self) -> f32 {
        if self.acquires == 0 {
            0.0
        } else {
            self.hits as f32 / self.acquires as f32
        }
    }
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

/// Specialized pool for render buffers
///
/// Pools buffers by size to avoid reallocation when terminal resizes.
pub struct BufferPool {
    /// Pools organized by (width, height)
    pools: RefCell<HashMap<(u16, u16), Vec<Buffer>>>,
    /// Maximum buffers per size
    max_per_size: usize,
    /// Statistics
    stats: RefCell<PoolStats>,
}

impl Default for BufferPool {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferPool {
    /// Create a new buffer pool
    pub fn new() -> Self {
        Self::with_capacity(4)
    }

    /// Create with specific capacity per size
    pub fn with_capacity(max_per_size: usize) -> Self {
        Self {
            pools: RefCell::new(HashMap::new()),
            max_per_size,
            stats: RefCell::new(PoolStats::default()),
        }
    }

    /// Acquire a buffer of the given size
    pub fn acquire(&self, width: u16, height: u16) -> Buffer {
        let mut stats = self.stats.borrow_mut();
        stats.acquires += 1;

        let key = (width, height);
        let mut pools = self.pools.borrow_mut();

        if let Some(pool) = pools.get_mut(&key) {
            if let Some(mut buf) = pool.pop() {
                stats.hits += 1;
                buf.clear();
                return buf;
            }
        }

        // Try to find a larger buffer we can use
        for ((w, h), pool) in pools.iter_mut() {
            if *w >= width && *h >= height {
                if let Some(mut buf) = pool.pop() {
                    stats.hits += 1;
                    buf.resize(width, height);
                    return buf;
                }
            }
        }

        stats.misses += 1;
        Buffer::new(width, height)
    }

    /// Release a buffer back to the pool
    pub fn release(&self, buf: Buffer) {
        let mut stats = self.stats.borrow_mut();
        stats.releases += 1;

        let key = (buf.width(), buf.height());
        let mut pools = self.pools.borrow_mut();

        let pool = pools.entry(key).or_default();
        if pool.len() < self.max_per_size {
            pool.push(buf);
        } else {
            stats.discards += 1;
        }
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        self.stats.borrow().clone()
    }

    /// Clear all pooled buffers
    pub fn clear(&self) {
        self.pools.borrow_mut().clear();
    }

    /// Total number of pooled buffers
    pub fn total_buffered(&self) -> usize {
        self.pools.borrow().values().map(|v| v.len()).sum()
    }
}

/// String interning pool for frequently used strings
///
/// Useful for widget names, class names, and other repeated strings.
pub struct StringPool {
    /// Interned strings
    strings: RefCell<HashMap<String, Arc<str>>>,
    /// Statistics
    stats: RefCell<PoolStats>,
}

impl Default for StringPool {
    fn default() -> Self {
        Self::new()
    }
}

impl StringPool {
    /// Create a new string pool
    pub fn new() -> Self {
        Self {
            strings: RefCell::new(HashMap::new()),
            stats: RefCell::new(PoolStats::default()),
        }
    }

    /// Intern a string
    pub fn intern(&self, s: impl AsRef<str>) -> Arc<str> {
        let s = s.as_ref();
        let mut stats = self.stats.borrow_mut();
        stats.acquires += 1;

        let mut strings = self.strings.borrow_mut();
        if let Some(interned) = strings.get(s) {
            stats.hits += 1;
            interned.clone()
        } else {
            stats.misses += 1;
            let interned: Arc<str> = s.into();
            strings.insert(s.to_owned(), interned.clone());
            interned
        }
    }

    /// Check if a string is interned
    pub fn contains(&self, s: &str) -> bool {
        self.strings.borrow().contains_key(s)
    }

    /// Number of interned strings
    pub fn len(&self) -> usize {
        self.strings.borrow().len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.strings.borrow().is_empty()
    }

    /// Get statistics
    pub fn stats(&self) -> PoolStats {
        self.stats.borrow().clone()
    }

    /// Clear all interned strings
    pub fn clear(&self) {
        self.strings.borrow_mut().clear();
    }
}

/// Thread-safe string pool
pub struct SyncStringPool {
    /// Interned strings
    strings: Mutex<HashMap<String, Arc<str>>>,
    /// Statistics
    stats: Mutex<PoolStats>,
}

impl Default for SyncStringPool {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncStringPool {
    /// Create a new thread-safe string pool
    pub fn new() -> Self {
        Self {
            strings: Mutex::new(HashMap::new()),
            stats: Mutex::new(PoolStats::default()),
        }
    }

    /// Intern a string
    pub fn intern(&self, s: impl AsRef<str>) -> Arc<str> {
        let s = s.as_ref();
        let mut stats = lock_or_recover(&self.stats);
        stats.acquires += 1;

        let mut strings = lock_or_recover(&self.strings);
        if let Some(interned) = strings.get(s) {
            stats.hits += 1;
            interned.clone()
        } else {
            stats.misses += 1;
            let interned: Arc<str> = s.into();
            strings.insert(s.to_owned(), interned.clone());
            interned
        }
    }

    /// Get statistics
    pub fn stats(&self) -> PoolStats {
        lock_or_recover(&self.stats).clone()
    }
}

/// Pool for reusing vectors
pub struct VecPool<T> {
    /// Pool of empty vectors with reserved capacity
    pool: RefCell<Vec<Vec<T>>>,
    /// Default capacity for new vectors
    default_capacity: usize,
    /// Maximum pool size
    max_size: usize,
    /// Statistics
    stats: RefCell<PoolStats>,
}

impl<T> VecPool<T> {
    /// Create a new vector pool
    pub fn new(default_capacity: usize) -> Self {
        Self::with_max_size(default_capacity, 16)
    }

    /// Create with specific max size
    pub fn with_max_size(default_capacity: usize, max_size: usize) -> Self {
        Self {
            pool: RefCell::new(Vec::with_capacity(max_size)),
            default_capacity,
            max_size,
            stats: RefCell::new(PoolStats::default()),
        }
    }

    /// Acquire a vector
    pub fn acquire(&self) -> Vec<T> {
        let mut stats = self.stats.borrow_mut();
        stats.acquires += 1;

        if let Some(vec) = self.pool.borrow_mut().pop() {
            stats.hits += 1;
            vec
        } else {
            stats.misses += 1;
            Vec::with_capacity(self.default_capacity)
        }
    }

    /// Release a vector back to the pool
    pub fn release(&self, mut vec: Vec<T>) {
        let mut stats = self.stats.borrow_mut();
        stats.releases += 1;

        vec.clear();

        let mut pool = self.pool.borrow_mut();
        if pool.len() < self.max_size {
            pool.push(vec);
        } else {
            stats.discards += 1;
        }
    }

    /// Get statistics
    pub fn stats(&self) -> PoolStats {
        self.stats.borrow().clone()
    }

    /// Clear the pool
    pub fn clear(&self) {
        self.pool.borrow_mut().clear();
    }

    /// Pre-warm the pool
    pub fn prewarm(&self, count: usize) {
        let target = count.min(self.max_size);
        let mut pool = self.pool.borrow_mut();
        while pool.len() < target {
            pool.push(Vec::with_capacity(self.default_capacity));
        }
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
        self.value.take().unwrap()
    }
}

impl<T> std::ops::Deref for Pooled<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref().unwrap()
    }
}

impl<T> std::ops::DerefMut for Pooled<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_mut().unwrap()
    }
}

impl<T> Drop for Pooled<'_, T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            self.pool.release(value);
        }
    }
}

/// Create an object pool
pub fn object_pool<T, F>(factory: F) -> ObjectPool<T>
where
    F: Fn() -> T + 'static,
{
    ObjectPool::new(factory)
}

/// Create a buffer pool
pub fn buffer_pool() -> BufferPool {
    BufferPool::new()
}

/// Create a string pool
pub fn string_pool() -> StringPool {
    StringPool::new()
}

/// Create a vector pool
pub fn vec_pool<T>(capacity: usize) -> VecPool<T> {
    VecPool::new(capacity)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_buffer_pool_basic() {
        let pool = BufferPool::new();

        let buf = pool.acquire(80, 24);
        assert_eq!(buf.width(), 80);
        assert_eq!(buf.height(), 24);

        pool.release(buf);
        assert_eq!(pool.total_buffered(), 1);

        // Acquire same size
        let buf2 = pool.acquire(80, 24);
        assert_eq!(buf2.width(), 80);
        assert_eq!(pool.stats().hits, 1);
    }

    #[test]
    fn test_buffer_pool_resize() {
        let pool = BufferPool::new();

        // Release a large buffer
        let buf = Buffer::new(160, 48);
        pool.release(buf);

        // Acquire a smaller buffer - should resize the larger one
        let buf2 = pool.acquire(80, 24);
        assert_eq!(buf2.width(), 80);
        assert_eq!(buf2.height(), 24);
        assert_eq!(pool.stats().hits, 1);
    }

    #[test]
    fn test_string_pool_basic() {
        let pool = StringPool::new();

        let s1 = pool.intern("hello");
        let s2 = pool.intern("hello");
        let s3 = pool.intern("world");

        // Same string should return same Arc
        assert!(Arc::ptr_eq(&s1, &s2));
        assert!(!Arc::ptr_eq(&s1, &s3));

        assert_eq!(pool.len(), 2);
        assert_eq!(pool.stats().hits, 1);
        assert_eq!(pool.stats().misses, 2);
    }

    #[test]
    fn test_vec_pool_basic() {
        let pool: VecPool<i32> = VecPool::new(16);

        let mut v = pool.acquire();
        assert!(v.capacity() >= 16);
        v.push(1);
        v.push(2);

        pool.release(v);

        // Acquire should return cleared vector
        let v2 = pool.acquire();
        assert!(v2.is_empty());
        assert!(v2.capacity() >= 16);
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

    #[test]
    fn test_sync_string_pool() {
        use std::thread;

        let pool: Arc<SyncStringPool> = Arc::new(SyncStringPool::new());

        let handles: Vec<_> = (0..4)
            .map(|i| {
                let pool = pool.clone();
                thread::spawn(move || {
                    for j in 0..10 {
                        let _ = pool.intern(format!("string-{}-{}", i, j));
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        let stats = pool.stats();
        assert_eq!(stats.acquires, 40);
    }

    #[test]
    fn test_pool_stats_hit_rate() {
        let pool: ObjectPool<u32> = ObjectPool::new(|| 0);

        pool.release(1);
        let _ = pool.acquire();
        let _ = pool.acquire();

        let stats = pool.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert!((stats.hit_rate() - 0.5).abs() < 0.01);
    }
}
