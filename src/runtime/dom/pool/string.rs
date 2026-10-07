//! String interning pools

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::PoolStats;
use crate::utils::lock::lock_or_recover;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
