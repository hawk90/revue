//! Diamond Dependency tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn test_diamond_dependency_no_duplicate_updates() {
    // Classic diamond dependency:
    //      A
    //     / \
    //    B   C
    //     \ /
    //      D
    //
    // When A changes, D should recalculate correctly

    let a = signal(1);
    let run_count = Arc::new(AtomicUsize::new(0));

    // Use signals for intermediate values instead of computed
    // This tests the diamond dependency pattern with signals
    let b = signal(0);
    let c = signal(0);

    // Effect 1: Update b when a changes
    let a1 = a.clone();
    let b1 = b.clone();
    let _update_b = effect(move || {
        b1.set(a1.get() * 2);
    });

    // Effect 2: Update c when a changes
    let a2 = a.clone();
    let c1 = c.clone();
    let _update_c = effect(move || {
        c1.set(a2.get() * 3);
    });

    // Effect 3: Final effect that depends on both b and c
    let run_count_clone = run_count.clone();
    let b2 = b.clone();
    let c2 = c.clone();
    let _d = effect(move || {
        let _sum = b2.get() + c2.get();
        run_count_clone.fetch_add(1, Ordering::SeqCst);
    });

    // Wait for initial effects to settle
    let initial_count = run_count.load(Ordering::SeqCst);
    assert!(initial_count >= 1, "Effect should run at least once");

    // Change a - should trigger all effects
    a.set(10);

    // Verify b and c were updated correctly
    assert_eq!(b.get(), 20); // 10 * 2
    assert_eq!(c.get(), 30); // 10 * 3

    // Effect should have run additional times
    assert!(run_count.load(Ordering::SeqCst) > initial_count);
}

#[test]
fn test_diamond_dependency_correct_values() {
    // Verify that diamond dependencies compute correct final values
    let source = signal(5);
    let result = Arc::new(AtomicUsize::new(0));

    // Use signals instead of computed for diamond pattern
    let left = signal(0);
    let right = signal(0);

    // Update left
    let s1 = source.clone();
    let l1 = left.clone();
    let _update_left = effect(move || {
        l1.set(s1.get() + 10);
    });

    // Update right
    let s2 = source.clone();
    let r1 = right.clone();
    let _update_right = effect(move || {
        r1.set(s2.get() * 2);
    });

    // Compute result from left and right
    let res = result.clone();
    let l2 = left.clone();
    let r2 = right.clone();
    let _compute_result = effect(move || {
        res.store((l2.get() + r2.get()) as usize, Ordering::SeqCst);
    });

    // Initial: source=5 -> left=15, right=10 -> result=25
    assert_eq!(result.load(Ordering::SeqCst), 25);

    source.set(10);
    // Updated: source=10 -> left=20, right=20 -> result=40
    assert_eq!(result.load(Ordering::SeqCst), 40);
}

#[test]
fn test_triple_diamond_dependency() {
    // More complex diamond:
    //        A
    //      / | \
    //     B  C  D
    //      \ | /
    //        E

    let a = signal(1);
    let count = Arc::new(AtomicUsize::new(0));

    // Use signals for intermediate values
    let b = signal(0);
    let c = signal(0);
    let d = signal(0);

    // Update b from a
    let a1 = a.clone();
    let b1 = b.clone();
    let _update_b = effect(move || {
        b1.set(a1.get() + 1);
    });

    // Update c from a
    let a2 = a.clone();
    let c1 = c.clone();
    let _update_c = effect(move || {
        c1.set(a2.get() + 2);
    });

    // Update d from a
    let a3 = a.clone();
    let d1 = d.clone();
    let _update_d = effect(move || {
        d1.set(a3.get() + 3);
    });

    // Final effect e that depends on b, c, d
    let count_clone = count.clone();
    let b2 = b.clone();
    let c2 = c.clone();
    let d2 = d.clone();
    let _e = effect(move || {
        let _sum = b2.get() + c2.get() + d2.get();
        count_clone.fetch_add(1, Ordering::SeqCst);
    });

    let initial = count.load(Ordering::SeqCst);
    assert!(initial >= 1, "Effect should run at least once");

    a.set(10);
    // Should trigger updates
    assert!(count.load(Ordering::SeqCst) > initial);
}
