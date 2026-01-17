//! Deep Dependency Chain tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn test_deep_signal_chain() {
    // Test a long chain: a -> b -> c -> d -> e -> f
    let a = signal(1);
    let b = signal(0);
    let c = signal(0);
    let d = signal(0);
    let e = signal(0);
    let f = signal(0);

    // Chain effects
    let a1 = a.clone();
    let b1 = b.clone();
    let _eff_b = effect(move || b1.set(a1.get() * 2));

    let b2 = b.clone();
    let c1 = c.clone();
    let _eff_c = effect(move || c1.set(b2.get() + 1));

    let c2 = c.clone();
    let d1 = d.clone();
    let _eff_d = effect(move || d1.set(c2.get() * 3));

    let d2 = d.clone();
    let e1 = e.clone();
    let _eff_e = effect(move || e1.set(d2.get() - 5));

    let e2 = e.clone();
    let f1 = f.clone();
    let _eff_f = effect(move || f1.set(e2.get() / 2));

    // Verify initial computation
    // a=1 -> b=2 -> c=3 -> d=9 -> e=4 -> f=2
    assert_eq!(f.get(), 2);

    // Change source
    a.set(5);
    // a=5 -> b=10 -> c=11 -> d=33 -> e=28 -> f=14
    assert_eq!(f.get(), 14);
}

#[test]
fn test_deep_effect_chain() {
    // Test effects triggering in a chain
    let a = signal(0);
    let b = signal(0);
    let c = signal(0);

    let a_clone = a.clone();
    let b_clone = b.clone();
    let _effect1 = effect(move || {
        let val = a_clone.get();
        b_clone.set(val * 2);
    });

    let b_clone2 = b.clone();
    let c_clone = c.clone();
    let _effect2 = effect(move || {
        let val = b_clone2.get();
        c_clone.set(val + 10);
    });

    // Initial state
    assert_eq!(c.get(), 10); // b=0, c=0+10

    // Update a
    a.set(5);
    // a=5 -> b=10 -> c=20
    assert_eq!(b.get(), 10);
    assert_eq!(c.get(), 20);
}
