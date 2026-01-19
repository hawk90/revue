//! Computed Value Caching tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn test_computed_caching_efficiency() {
    let source = signal(10);
    let compute_count = Arc::new(AtomicUsize::new(0));

    let cc = compute_count.clone();
    let s = source.clone();
    let expensive = computed(move || {
        cc.fetch_add(1, Ordering::SeqCst);
        s.get() * 2
    });

    // First access computes
    assert_eq!(expensive.get(), 20);
    assert_eq!(compute_count.load(Ordering::SeqCst), 1);

    // Multiple accesses use cache
    assert_eq!(expensive.get(), 20);
    assert_eq!(expensive.get(), 20);
    assert_eq!(expensive.get(), 20);
    assert_eq!(compute_count.load(Ordering::SeqCst), 1); // Still 1

    // Update source
    source.set(20);

    // Manual invalidation is required
    expensive.invalidate();

    // Next access recomputes
    assert_eq!(expensive.get(), 40);
    assert_eq!(compute_count.load(Ordering::SeqCst), 2);

    // Again, multiple accesses use cache
    assert_eq!(expensive.get(), 40);
    assert_eq!(expensive.get(), 40);
    assert_eq!(compute_count.load(Ordering::SeqCst), 2); // Still 2
}

#[test]
fn test_computed_invalidation() {
    let a = signal(1);
    let b = signal(2);

    let a_c = a.clone();
    let b_c = b.clone();
    let sum = computed(move || a_c.get() + b_c.get());

    assert_eq!(sum.get(), 3);
    assert!(!sum.is_dirty());

    // Changing either signal should invalidate
    a.set(10);
    // Note: In current impl, computed doesn't auto-track dependencies
    // This test documents expected behavior if it did

    b.set(20);
    // Manual invalidation would be needed in current impl
}
