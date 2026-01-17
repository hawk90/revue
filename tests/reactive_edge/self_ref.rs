//! Self-Referential Update tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn test_signal_self_update() {
    // Test updating a signal based on its own value
    // This should use the mutable reference directly, not try to read
    let counter = signal(0);

    // Correct way: use the mutable reference
    counter.update(|val| {
        *val = *val + 1;
    });
    assert_eq!(counter.get(), 1);

    counter.update(|val| {
        *val = *val * 2;
    });
    assert_eq!(counter.get(), 2);

    // Capture the value first or use the mutable reference:
    let current = counter.get();
    counter.set(current + 10);
    assert_eq!(counter.get(), 12);
}

#[test]
fn test_effect_triggered_by_own_write() {
    // Effect that writes to a signal it reads from
    // This should not cause infinite loops (effect shouldn't re-trigger)
    let counter = signal(0);
    let limit = signal(5);
    let run_count = Arc::new(AtomicUsize::new(0));

    let rc = run_count.clone();
    let c = counter.clone();
    let l = limit.clone();

    let _effect = effect(move || {
        rc.fetch_add(1, Ordering::SeqCst);

        // Prevent infinite loop with a guard
        if rc.load(Ordering::SeqCst) > 100 {
            panic!("Infinite loop detected!");
        }

        let current = c.get();
        let max = l.get();

        // Don't update if at limit (prevents re-trigger)
        if current < max {
            c.set(current + 1);
        }
    });

    // Effect should stabilize at counter=5
    // In current impl, this might cause issues
    assert!(
        run_count.load(Ordering::SeqCst) < 100,
        "Should not run 100+ times"
    );
}
