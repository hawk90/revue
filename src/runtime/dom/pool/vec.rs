//! Pool for reusing vectors

use std::cell::RefCell;

use super::PoolStats;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
