//! Concurrent Update tests

#![allow(unused_imports)]

use revue::reactive::*;
use serial_test::serial;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
#[serial]
fn test_multiple_signal_updates_batch() {
    // Test that multiple updates in sequence work correctly
    let a = signal(0);
    let b = signal(0);
    let c = signal(0);
    let sum = Arc::new(AtomicUsize::new(0));
    let run_count = Arc::new(AtomicUsize::new(0));

    let s = sum.clone();
    let rc = run_count.clone();
    let a_c = a.clone();
    let b_c = b.clone();
    let c_c = c.clone();

    let _effect = effect(move || {
        s.store(
            (a_c.get() + b_c.get() + c_c.get()) as usize,
            Ordering::SeqCst,
        );
        rc.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(sum.load(Ordering::SeqCst), 0);
    assert_eq!(run_count.load(Ordering::SeqCst), 1);

    // Update all three signals
    a.set(10);
    b.set(20);
    c.set(30);

    // Final sum should be correct
    assert_eq!(sum.load(Ordering::SeqCst), 60);

    // Effect runs once per signal update (no batching in current impl)
    assert!(run_count.load(Ordering::SeqCst) >= 4); // Initial + 3 updates
}

#[test]
#[serial]
fn test_rapid_signal_updates() {
    let signal = signal(0);
    let last_value = Arc::new(AtomicUsize::new(0));
    let run_count = Arc::new(AtomicUsize::new(0));

    let lv = last_value.clone();
    let rc = run_count.clone();
    let s = signal.clone();

    let _effect = effect(move || {
        lv.store(s.get() as usize, Ordering::SeqCst);
        rc.fetch_add(1, Ordering::SeqCst);
    });

    // Rapidly update signal
    for i in 1..=100 {
        signal.set(i);
    }

    // Should see final value
    assert_eq!(last_value.load(Ordering::SeqCst), 100);

    // Should have run 101 times (initial + 100 updates)
    assert_eq!(run_count.load(Ordering::SeqCst), 101);
}
