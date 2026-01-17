//! Signal tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_signal_get_set() {
    let count = Signal::new(0);
    assert_eq!(count.get(), 0);

    count.set(5);
    assert_eq!(count.get(), 5);
}

#[test]
fn test_signal_update() {
    let count = Signal::new(10);
    count.update(|n| *n += 5);
    assert_eq!(count.get(), 15);

    count.update(|n| *n *= 2);
    assert_eq!(count.get(), 30);
}

#[test]
fn test_signal_subscribe() {
    let count = Signal::new(0);
    let called = Arc::new(AtomicUsize::new(0));

    let called_clone = called.clone();
    let _sub = count.subscribe(move || {
        called_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(called.load(Ordering::SeqCst), 0);

    count.set(1);
    assert_eq!(called.load(Ordering::SeqCst), 1);

    count.set(2);
    assert_eq!(called.load(Ordering::SeqCst), 2);
}

#[test]
fn test_signal_unsubscribe_on_drop() {
    let count = Signal::new(0);
    let called = Arc::new(AtomicUsize::new(0));

    let called_clone = called.clone();
    let sub = count.subscribe(move || {
        called_clone.fetch_add(1, Ordering::SeqCst);
    });

    count.set(1);
    assert_eq!(called.load(Ordering::SeqCst), 1);

    drop(sub);

    count.set(2);
    assert_eq!(called.load(Ordering::SeqCst), 1);
}

#[test]
fn test_signal_multiple_subscriptions() {
    let count = Signal::new(0);
    let called1 = Arc::new(AtomicUsize::new(0));
    let called2 = Arc::new(AtomicUsize::new(0));

    let c1 = called1.clone();
    let sub1 = count.subscribe(move || {
        c1.fetch_add(1, Ordering::SeqCst);
    });

    let c2 = called2.clone();
    let _sub2 = count.subscribe(move || {
        c2.fetch_add(1, Ordering::SeqCst);
    });

    count.set(1);
    assert_eq!(called1.load(Ordering::SeqCst), 1);
    assert_eq!(called2.load(Ordering::SeqCst), 1);

    drop(sub1);

    count.set(2);
    assert_eq!(called1.load(Ordering::SeqCst), 1);
    assert_eq!(called2.load(Ordering::SeqCst), 2);
}

#[test]
fn test_signal_clone_shares_value() {
    let count = Signal::new(0);
    let count2 = count.clone();

    count.set(42);
    assert_eq!(count2.get(), 42);

    count2.set(100);
    assert_eq!(count.get(), 100);
}

#[test]
fn test_signal_with_string() {
    let name = Signal::new(String::from("hello"));
    assert_eq!(name.get(), "hello");

    name.set(String::from("world"));
    assert_eq!(name.get(), "world");

    name.update(|s| s.push_str("!"));
    assert_eq!(name.get(), "world!");
}

#[test]
fn test_signal_with_vec() {
    let items = Signal::new(vec![1, 2, 3]);
    assert_eq!(items.get(), vec![1, 2, 3]);

    items.update(|v| v.push(4));
    assert_eq!(items.get(), vec![1, 2, 3, 4]);
}

#[test]
fn test_signal_unique_ids() {
    let s1 = Signal::new(1);
    let s2 = Signal::new(2);
    assert_ne!(s1.id(), s2.id());
}

#[test]
fn test_signal_read_zero_copy() {
    let items = Signal::new(vec![1, 2, 3]);
    assert_eq!(items.read().len(), 3);
    assert_eq!(items.read()[0], 1);
}

#[test]
fn test_signal_borrow_zero_copy() {
    let items = Signal::new(vec![1, 2, 3]);
    assert_eq!(items.borrow().len(), 3);
    assert_eq!(items.borrow()[0], 1);
}

#[test]
fn test_signal_with_zero_copy() {
    let items = Signal::new(vec![1, 2, 3]);
    let len = items.with(|v| v.len());
    assert_eq!(len, 3);

    let sum: i32 = items.with(|v| v.iter().sum());
    assert_eq!(sum, 6);
}

#[test]
fn test_signal_thread_safety() {
    let count = Signal::new(0);
    let count_clone = count.clone();

    let handle = std::thread::spawn(move || {
        for _ in 0..100 {
            count_clone.update(|n| *n += 1);
        }
    });

    for _ in 0..100 {
        count.update(|n| *n += 1);
    }

    handle.join().unwrap();
    assert_eq!(count.get(), 200);
}

#[test]
fn test_signal_cross_thread_subscribe() {
    let count = Signal::new(0);
    let notified = Arc::new(AtomicUsize::new(0));

    let notified_clone = notified.clone();
    let _sub = count.subscribe(move || {
        notified_clone.fetch_add(1, Ordering::SeqCst);
    });

    let count_clone = count.clone();
    let handle = std::thread::spawn(move || {
        count_clone.set(42);
    });

    handle.join().unwrap();

    assert_eq!(count.get(), 42);
    assert_eq!(notified.load(Ordering::SeqCst), 1);
}

#[test]
fn test_signal_debug() {
    let sig = Signal::new(42);
    let debug_str = format!("{:?}", sig);
    assert!(debug_str.contains("Signal"));
    assert!(debug_str.contains("42"));
}
