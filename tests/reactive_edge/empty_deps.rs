//! Edge Case: Empty Dependencies tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn test_effect_with_no_dependencies() {
    let run_count = Arc::new(AtomicUsize::new(0));
    let rc = run_count.clone();

    let _effect = effect(move || {
        rc.fetch_add(1, Ordering::SeqCst);
    });

    // Should run once on creation
    assert_eq!(run_count.load(Ordering::SeqCst), 1);

    // Should not run again (no dependencies to trigger it)
    // In practice, there's no way to trigger it without dependencies
}

#[test]
fn test_computed_with_no_dependencies() {
    let constant = computed(|| 42);

    assert_eq!(constant.get(), 42);
    assert_eq!(constant.get(), 42);

    // Should always return same value
    assert!(!constant.is_dirty());
}
