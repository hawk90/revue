//! Runtime tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_new_runtime() {
    let runtime = ReactiveRuntime::new();
    assert!(!runtime.has_pending());
}

#[test]
fn test_default_runtime() {
    let runtime = ReactiveRuntime::default();
    assert!(!runtime.has_pending());
}

#[test]
fn test_schedule_effect() {
    let mut runtime = ReactiveRuntime::new();
    assert!(!runtime.has_pending());

    runtime.schedule_effect(Box::new(|| {}));
    assert!(runtime.has_pending());
}

#[test]
fn test_flush_clears_dirty() {
    let mut runtime = ReactiveRuntime::new();
    runtime.mark_dirty(SignalId::new());
    runtime.mark_dirty(SignalId::new());
    assert!(runtime.has_pending());

    runtime.flush();
    assert!(!runtime.has_pending());
}

#[test]
fn test_flush_runs_effects() {
    let mut runtime = ReactiveRuntime::new();
    let counter = Arc::new(AtomicUsize::new(0));

    let counter_clone = counter.clone();
    runtime.schedule_effect(Box::new(move || {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    }));

    let counter_clone = counter.clone();
    runtime.schedule_effect(Box::new(move || {
        counter_clone.fetch_add(10, Ordering::SeqCst);
    }));

    assert_eq!(counter.load(Ordering::SeqCst), 0);
    runtime.flush();
    assert_eq!(counter.load(Ordering::SeqCst), 11);
}

#[test]
fn test_has_pending_effects_only() {
    let mut runtime = ReactiveRuntime::new();
    runtime.schedule_effect(Box::new(|| {}));
    assert!(runtime.has_pending());
}

#[test]
fn test_has_pending_both() {
    let mut runtime = ReactiveRuntime::new();
    runtime.mark_dirty(SignalId::new());
    runtime.schedule_effect(Box::new(|| {}));
    assert!(runtime.has_pending());
}
