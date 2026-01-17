//! Effect tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_effect_runs_immediately() {
    let called = Arc::new(AtomicBool::new(false));
    let called_clone = called.clone();

    let _effect = Effect::new(move || {
        called_clone.store(true, Ordering::SeqCst);
    });

    assert!(called.load(Ordering::SeqCst));
}

#[test]
fn test_effect_lazy_does_not_run() {
    let called = Arc::new(AtomicBool::new(false));
    let called_clone = called.clone();

    let effect = Effect::lazy(move || {
        called_clone.store(true, Ordering::SeqCst);
    });

    assert!(!called.load(Ordering::SeqCst));

    effect.run();
    assert!(called.load(Ordering::SeqCst));
}

#[test]
fn test_effect_stop_and_resume() {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();

    let effect = Effect::lazy(move || {
        count_clone.fetch_add(1, Ordering::SeqCst);
    });

    effect.run();
    assert_eq!(count.load(Ordering::SeqCst), 1);

    effect.stop();
    effect.run();
    assert_eq!(count.load(Ordering::SeqCst), 1);

    effect.resume();
    effect.run();
    assert_eq!(count.load(Ordering::SeqCst), 2);
}

#[test]
fn test_effect_is_active() {
    let effect = Effect::lazy(|| {});

    assert!(effect.is_active());

    effect.stop();
    assert!(!effect.is_active());

    effect.resume();
    assert!(effect.is_active());
}

#[test]
fn test_effect_dynamic_dependency_retracking() {
    let flag = signal(true);
    let a = signal(1);
    let b = signal(2);
    let result = Arc::new(AtomicUsize::new(0));

    let res = result.clone();
    let f = flag.clone();
    let a_c = a.clone();
    let b_c = b.clone();

    let _effect = Effect::new(move || {
        let val = if f.get() { a_c.get() } else { b_c.get() };
        res.store(val as usize, Ordering::SeqCst);
    });

    assert_eq!(result.load(Ordering::SeqCst), 1);

    a.set(10);
    assert_eq!(result.load(Ordering::SeqCst), 10);

    flag.set(false);
    assert_eq!(result.load(Ordering::SeqCst), 2);

    b.set(20);
    assert_eq!(result.load(Ordering::SeqCst), 20);

    a.set(100);
    assert_eq!(result.load(Ordering::SeqCst), 20);
}
