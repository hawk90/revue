//! Async state tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// Poll for a condition with a timeout, returning when the condition becomes true
/// or the timeout elapses. Returns true if condition was met, false on timeout.
fn poll_until<F>(mut condition: F, timeout_ms: u64) -> bool
where
    F: FnMut() -> bool,
{
    let start = Instant::now();
    let timeout = Duration::from_millis(timeout_ms);
    let poll_interval = Duration::from_millis(2);

    while start.elapsed() < timeout {
        if condition() {
            return true;
        }
        thread::sleep(poll_interval);
    }
    false
}

#[test]
fn test_async_state_variants() {
    let idle: AsyncState<i32> = AsyncState::Idle;
    assert!(idle.is_idle());
    assert!(!idle.is_loading());
    assert!(!idle.is_ready());
    assert!(!idle.is_error());

    let loading: AsyncState<i32> = AsyncState::Loading;
    assert!(loading.is_loading());

    let ready: AsyncState<i32> = AsyncState::Ready(42);
    assert!(ready.is_ready());
    assert_eq!(ready.value(), Some(&42));

    let error: AsyncState<i32> = AsyncState::Error("failed".to_string());
    assert!(error.is_error());
    assert_eq!(error.error(), Some("failed"));
}

#[test]
fn test_async_state_map() {
    let ready: AsyncState<i32> = AsyncState::Ready(10);
    let mapped = ready.map(|v| v * 2);
    assert_eq!(mapped, AsyncState::Ready(20));

    let loading: AsyncState<i32> = AsyncState::Loading;
    let mapped_loading = loading.map(|v| v * 2);
    assert!(mapped_loading.is_loading());
}

#[test]
fn test_async_state_unwrap_or() {
    let ready: AsyncState<i32> = AsyncState::Ready(42);
    assert_eq!(ready.unwrap_or(0), 42);

    let loading: AsyncState<i32> = AsyncState::Loading;
    assert_eq!(loading.unwrap_or(0), 0);
}

#[test]
fn test_use_async() {
    let (state, trigger) = use_async(|| {
        thread::sleep(Duration::from_millis(10));
        Ok(42)
    });

    assert!(state.get().is_idle());

    trigger();

    // Wait for async operation to complete (up to 1 second)
    poll_until(|| state.get().is_ready(), 1000);

    assert_eq!(state.get(), AsyncState::Ready(42));
}

#[test]
fn test_use_async_error() {
    let (state, trigger) = use_async::<i32, _>(|| Err("Something went wrong".to_string()));

    trigger();

    // Wait for async operation to complete with error (up to 1 second)
    poll_until(|| state.get().is_error(), 1000);

    assert!(state.get().is_error());
    assert_eq!(state.get().error(), Some("Something went wrong"));
}

#[test]
fn test_use_async_poll() {
    let (state, start, poll) = use_async_poll(|| {
        thread::sleep(Duration::from_millis(10));
        Ok("done".to_string())
    });

    assert!(state.get().is_idle());

    start();
    assert!(state.get().is_loading());

    // Wait for async operation to complete (up to 500ms)
    poll_until(|| poll(), 500);

    assert_eq!(state.get(), AsyncState::Ready("done".to_string()));
}

#[test]
fn test_use_async_immediate() {
    let state = use_async_immediate(|| {
        thread::sleep(Duration::from_millis(5));
        Ok(100)
    });

    assert!(state.get().is_loading());

    // Wait for async operation to complete (up to 1 second)
    poll_until(|| state.get() == AsyncState::Ready(100), 1000);

    assert_eq!(state.get(), AsyncState::Ready(100));
}

#[test]
fn test_async_state_display() {
    let idle: AsyncState<i32> = AsyncState::Idle;
    assert_eq!(format!("{}", idle), "Idle");

    let loading: AsyncState<i32> = AsyncState::Loading;
    assert_eq!(format!("{}", loading), "Loading");

    let ready: AsyncState<i32> = AsyncState::Ready(42);
    assert_eq!(format!("{}", ready), "Ready(42)");

    let error: AsyncState<i32> = AsyncState::Error("fail".to_string());
    assert_eq!(format!("{}", error), "Error(fail)");
}

#[test]
fn test_use_async_multiple_triggers() {
    let (state, trigger) = use_async(|| {
        thread::sleep(Duration::from_millis(5));
        Ok(42)
    });

    trigger();
    assert!(state.get().is_loading());

    trigger();
    assert!(state.get().is_loading());

    // Wait for async operation to complete (up to 1 second)
    poll_until(|| state.get() == AsyncState::Ready(42), 1000);

    assert_eq!(state.get(), AsyncState::Ready(42));
}

#[test]
fn test_use_async_poll_cross_thread() {
    let (state, start, poll) = use_async_poll(|| {
        thread::sleep(Duration::from_millis(10));
        Ok(42)
    });

    let completed = Arc::new(AtomicBool::new(false));
    let completed_clone = completed.clone();

    start();
    assert!(state.get().is_loading());

    let poll_thread = thread::spawn(move || {
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(500) {
            if poll() {
                completed_clone.store(true, Ordering::SeqCst);
                return true;
            }
            thread::sleep(Duration::from_millis(2));
        }
        false
    });

    let result = poll_thread.join().expect("poll thread should not panic");
    assert!(result, "polling should complete successfully");
    assert!(completed.load(Ordering::SeqCst));
    assert_eq!(state.get(), AsyncState::Ready(42));
}

#[test]
fn test_use_async_poll_start_from_different_thread() {
    let (state, start, poll) = use_async_poll(|| {
        thread::sleep(Duration::from_millis(10));
        Ok("cross-thread".to_string())
    });

    let start_thread = thread::spawn(move || {
        start();
    });
    start_thread.join().expect("start thread should not panic");

    thread::sleep(Duration::from_millis(5));
    assert!(state.get().is_loading());

    // Wait for async operation to complete (up to 500ms)
    poll_until(|| poll(), 500);

    assert_eq!(state.get(), AsyncState::Ready("cross-thread".to_string()));
}
