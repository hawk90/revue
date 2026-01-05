//! Lock utilities for consistent poison handling
//!
//! These helpers provide consistent recovery from poisoned locks across the codebase.
//! Instead of panicking on `PoisonError`, they recover by unwrapping the inner value.
//!
//! # Example
//!
//! ```ignore
//! use std::sync::Mutex;
//! use revue::utils::lock::lock_or_recover;
//!
//! let mutex = Mutex::new(42);
//! let guard = lock_or_recover(&mutex);
//! assert_eq!(*guard, 42);
//! ```

use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Acquires a mutex lock, recovering from poison if necessary.
///
/// If the lock is poisoned (a thread panicked while holding it),
/// this function recovers the inner value instead of panicking.
#[inline]
pub fn lock_or_recover<T>(lock: &Mutex<T>) -> MutexGuard<'_, T> {
    lock.lock().unwrap_or_else(|poisoned| {
        log_warn!("Mutex was poisoned, recovering");
        poisoned.into_inner()
    })
}

/// Acquires a read lock on an RwLock, recovering from poison if necessary.
#[inline]
pub fn read_or_recover<T>(lock: &RwLock<T>) -> RwLockReadGuard<'_, T> {
    lock.read().unwrap_or_else(|poisoned| {
        log_warn!("RwLock was poisoned (read), recovering");
        poisoned.into_inner()
    })
}

/// Acquires a write lock on an RwLock, recovering from poison if necessary.
#[inline]
pub fn write_or_recover<T>(lock: &RwLock<T>) -> RwLockWriteGuard<'_, T> {
    lock.write().unwrap_or_else(|poisoned| {
        log_warn!("RwLock was poisoned (write), recovering");
        poisoned.into_inner()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_lock_or_recover_normal() {
        let mutex = Mutex::new(42);
        let guard = lock_or_recover(&mutex);
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_lock_or_recover_poisoned() {
        let mutex = Arc::new(Mutex::new(42));
        let mutex_clone = Arc::clone(&mutex);

        // Poison the mutex by panicking while holding the lock
        let handle = thread::spawn(move || {
            let _guard = mutex_clone.lock().unwrap();
            panic!("intentional panic to poison mutex");
        });

        // Wait for the thread to panic
        let _ = handle.join();

        // Should recover instead of panicking
        let guard = lock_or_recover(&mutex);
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_read_or_recover_normal() {
        let rwlock = RwLock::new(42);
        let guard = read_or_recover(&rwlock);
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_write_or_recover_normal() {
        let rwlock = RwLock::new(42);
        let mut guard = write_or_recover(&rwlock);
        *guard = 100;
        drop(guard);
        assert_eq!(*read_or_recover(&rwlock), 100);
    }

    #[test]
    fn test_rwlock_poisoned_by_write() {
        let rwlock = Arc::new(RwLock::new(42));
        let rwlock_clone = Arc::clone(&rwlock);

        // Poison by panicking during write
        let handle = thread::spawn(move || {
            let _guard = rwlock_clone.write().unwrap();
            panic!("intentional panic to poison rwlock");
        });

        let _ = handle.join();

        // Both read and write should recover
        let guard = read_or_recover(&rwlock);
        assert_eq!(*guard, 42);
        drop(guard);

        let guard = write_or_recover(&rwlock);
        assert_eq!(*guard, 42);
    }
}
