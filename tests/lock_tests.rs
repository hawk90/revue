//! Integration tests for lock utilities
//! Extracted from src/utils/lock.rs

use revue::utils::lock::*;
use std::sync::{Arc, Mutex, RwLock};
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
