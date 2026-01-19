//! Computed tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_computed_basic() {
    let computed = Computed::new(|| 42);
    assert_eq!(computed.get(), 42);
}

#[test]
fn test_computed_with_closure() {
    let multiplier = 3;
    let computed = Computed::new(move || 10 * multiplier);
    assert_eq!(computed.get(), 30);
}

#[test]
fn test_computed_caching() {
    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = call_count.clone();

    let computed = Computed::new(move || {
        call_count_clone.fetch_add(1, Ordering::SeqCst);
        42
    });

    assert_eq!(computed.get(), 42);
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    assert_eq!(computed.get(), 42);
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    computed.invalidate();
    assert_eq!(computed.get(), 42);
    assert_eq!(call_count.load(Ordering::SeqCst), 2);
}

#[test]
fn test_computed_dirty_flag() {
    let computed = Computed::new(|| 1);

    assert!(computed.is_dirty());

    computed.get();
    assert!(!computed.is_dirty());

    computed.invalidate();
    assert!(computed.is_dirty());
}

#[test]
fn test_computed_auto_invalidation() {
    let source = signal(10);
    let compute_count = Arc::new(AtomicUsize::new(0));

    let cc = compute_count.clone();
    let s = source.clone();
    let computed = Computed::new(move || {
        cc.fetch_add(1, Ordering::SeqCst);
        s.get() * 2
    });

    assert_eq!(computed.get(), 20);
    assert_eq!(compute_count.load(Ordering::SeqCst), 1);

    assert_eq!(computed.get(), 20);
    assert_eq!(computed.get(), 20);
    assert_eq!(compute_count.load(Ordering::SeqCst), 1);

    source.set(20);

    assert!(computed.is_dirty());

    assert_eq!(computed.get(), 40);
    assert_eq!(compute_count.load(Ordering::SeqCst), 2);

    assert_eq!(computed.get(), 40);
    assert_eq!(compute_count.load(Ordering::SeqCst), 2);
}

#[test]
fn test_computed_dynamic_dependencies() {
    let flag = signal(true);
    let a = signal(1);
    let b = signal(2);

    let f = flag.clone();
    let a_c = a.clone();
    let b_c = b.clone();

    let computed = Computed::new(move || if f.get() { a_c.get() } else { b_c.get() });

    assert_eq!(computed.get(), 1);

    a.set(10);
    assert_eq!(computed.get(), 10);

    flag.set(false);
    assert_eq!(computed.get(), 2);

    b.set(20);
    assert_eq!(computed.get(), 20);

    a.set(100);
    assert_eq!(computed.get(), 20);
}

#[test]
fn test_computed_thread_safety() {
    let computed = Arc::new(Computed::new(|| 42));

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let c = computed.clone();
            std::thread::spawn(move || {
                for _ in 0..100 {
                    assert_eq!(c.get(), 42);
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn test_computed_no_data_race_on_concurrent_recompute() {
    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = call_count.clone();

    let computed = Arc::new(Computed::new(move || {
        call_count_clone.fetch_add(1, Ordering::SeqCst);
        std::thread::sleep(Duration::from_micros(100));
        42
    }));

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let c = computed.clone();
            std::thread::spawn(move || c.get())
        })
        .collect();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    assert!(results.iter().all(|&v| v == 42));
    assert_eq!(
        call_count.load(Ordering::SeqCst),
        1,
        "Computation should run exactly once, but ran {} times",
        call_count.load(Ordering::SeqCst)
    );
}

#[test]
fn test_computed_recomputes_after_invalidation_with_contention() {
    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = call_count.clone();

    let computed = Arc::new(Computed::new(move || {
        call_count_clone.fetch_add(1, Ordering::SeqCst)
    }));

    let v1 = computed.get();
    assert_eq!(v1, 0);
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    for _ in 0..10 {
        assert_eq!(computed.get(), 0);
    }
    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    computed.invalidate();

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let c = computed.clone();
            std::thread::spawn(move || c.get())
        })
        .collect();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    assert!(results.iter().all(|&v| v == 1));
    assert_eq!(call_count.load(Ordering::SeqCst), 2);
}
