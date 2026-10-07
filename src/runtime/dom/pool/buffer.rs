//! Size-keyed pool for render buffers

use std::cell::RefCell;
use std::collections::HashMap;

use super::PoolStats;
use crate::render::Buffer;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
