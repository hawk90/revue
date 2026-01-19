//! Module integration tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_automatic_dependency_tracking() {
    let count = signal(0);
    let run_count = Arc::new(AtomicUsize::new(0));

    let run_count_clone = run_count.clone();
    let count_clone = count.clone();
    let _effect = effect(move || {
        let _ = count_clone.get();
        run_count_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(run_count.load(Ordering::SeqCst), 1);

    count.set(1);
    assert_eq!(run_count.load(Ordering::SeqCst), 2);

    count.set(2);
    assert_eq!(run_count.load(Ordering::SeqCst), 3);
}

#[test]
fn test_multiple_signals_dependency() {
    let a = signal(1);
    let b = signal(2);
    let sum = Arc::new(AtomicUsize::new(0));
    let run_count = Arc::new(AtomicUsize::new(0));

    let sum_clone = sum.clone();
    let run_count_clone = run_count.clone();
    let a_clone = a.clone();
    let b_clone = b.clone();
    let _effect = effect(move || {
        sum_clone.store((a_clone.get() + b_clone.get()) as usize, Ordering::SeqCst);
        run_count_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(sum.load(Ordering::SeqCst), 3);
    assert_eq!(run_count.load(Ordering::SeqCst), 1);

    a.set(10);
    assert_eq!(sum.load(Ordering::SeqCst), 12);
    assert_eq!(run_count.load(Ordering::SeqCst), 2);

    b.set(20);
    assert_eq!(sum.load(Ordering::SeqCst), 30);
    assert_eq!(run_count.load(Ordering::SeqCst), 3);
}

#[test]
fn test_effect_stop_removes_tracking() {
    let count = signal(0);
    let run_count = Arc::new(AtomicUsize::new(0));

    let run_count_clone = run_count.clone();
    let count_clone = count.clone();
    let effect_handle = effect(move || {
        let _ = count_clone.get();
        run_count_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(run_count.load(Ordering::SeqCst), 1);

    effect_handle.stop();

    count.set(1);
    assert_eq!(run_count.load(Ordering::SeqCst), 1);
}

#[test]
fn test_effect_with_borrow() {
    let items = signal(vec![1, 2, 3]);
    let sum = Arc::new(AtomicUsize::new(0));

    let sum_clone = sum.clone();
    let items_clone = items.clone();
    let _effect = effect(move || {
        let s: i32 = items_clone.borrow().iter().sum();
        sum_clone.store(s as usize, Ordering::SeqCst);
    });

    assert_eq!(sum.load(Ordering::SeqCst), 6);

    items.update(|v| v.push(4));
    assert_eq!(sum.load(Ordering::SeqCst), 10);
}

#[test]
fn test_effect_with_with() {
    let name = signal(String::from("World"));
    let greeting = Arc::new(std::sync::RwLock::new(String::new()));

    let greeting_clone = greeting.clone();
    let name_clone = name.clone();
    let _effect = effect(move || {
        let g = name_clone.with(|n| format!("Hello, {}!", n));
        *greeting_clone.write().unwrap() = g;
    });

    assert_eq!(*greeting.read().unwrap(), "Hello, World!");

    name.set(String::from("Revue"));
    assert_eq!(*greeting.read().unwrap(), "Hello, Revue!");
}

#[test]
fn test_effect_dropped_cleans_up() {
    let count = signal(0);
    let run_count = Arc::new(AtomicUsize::new(0));

    {
        let run_count_clone = run_count.clone();
        let count_clone = count.clone();
        let _effect = effect(move || {
            let _ = count_clone.get();
            run_count_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(run_count.load(Ordering::SeqCst), 1);
        count.set(1);
        assert_eq!(run_count.load(Ordering::SeqCst), 2);
    }

    count.set(2);
    assert_eq!(run_count.load(Ordering::SeqCst), 2);
}

#[test]
fn test_conditional_dependency() {
    let flag = signal(true);
    let a = signal(1);
    let b = signal(2);
    let result = Arc::new(AtomicUsize::new(0));
    let run_count = Arc::new(AtomicUsize::new(0));

    let result_clone = result.clone();
    let run_count_clone = run_count.clone();
    let flag_clone = flag.clone();
    let a_clone = a.clone();
    let b_clone = b.clone();

    let _effect = effect(move || {
        let value = if flag_clone.get() {
            a_clone.get()
        } else {
            b_clone.get()
        };
        result_clone.store(value as usize, Ordering::SeqCst);
        run_count_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(result.load(Ordering::SeqCst), 1);
    assert_eq!(run_count.load(Ordering::SeqCst), 1);

    a.set(10);
    assert_eq!(result.load(Ordering::SeqCst), 10);
    assert_eq!(run_count.load(Ordering::SeqCst), 2);
}
