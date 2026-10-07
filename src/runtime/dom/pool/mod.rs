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

mod buffer;
mod object;
mod string;
mod vec;

pub use buffer::BufferPool;
pub use object::{ObjectPool, Pooled, SyncObjectPool};
pub use string::{StringPool, SyncStringPool};
pub use vec::VecPool;

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
