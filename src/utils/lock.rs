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
