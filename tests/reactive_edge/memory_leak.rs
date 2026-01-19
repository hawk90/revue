//! Memory Leak Prevention tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn test_effect_disposal_prevents_leaks() {
    let signal = signal(0);
    let call_count = Arc::new(AtomicUsize::new(0));

    let count_clone = call_count.clone();
    let signal_clone = signal.clone();

    {
        let _effect = effect(move || {
            let _ = signal_clone.get();
            count_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        signal.set(1);
        assert_eq!(call_count.load(Ordering::SeqCst), 2);

        // Effect dropped here
    }

    // After effect is dropped, signal updates should not trigger it
    signal.set(2);
    assert_eq!(call_count.load(Ordering::SeqCst), 2); // Still 2, not 3
}

#[test]
fn test_manual_effect_stop() {
    let signal = signal(0);
    let call_count = Arc::new(AtomicUsize::new(0));

    let count_clone = call_count.clone();
    let signal_clone = signal.clone();
    let effect = effect(move || {
        let _ = signal_clone.get();
        count_clone.fetch_add(1, Ordering::SeqCst);
    });

    assert_eq!(call_count.load(Ordering::SeqCst), 1);

    signal.set(1);
    assert_eq!(call_count.load(Ordering::SeqCst), 2);

    // Manually stop the effect
    effect.stop();

    // Updates should not trigger effect anymore
    signal.set(2);
    assert_eq!(call_count.load(Ordering::SeqCst), 2); // Still 2
}

#[test]
fn test_multiple_effects_cleanup() {
    let signal = signal(0);
    let count1 = Arc::new(AtomicUsize::new(0));
    let count2 = Arc::new(AtomicUsize::new(0));
    let count3 = Arc::new(AtomicUsize::new(0));

    let c1 = count1.clone();
    let s1 = signal.clone();
    let effect1 = effect(move || {
        let _ = s1.get();
        c1.fetch_add(1, Ordering::SeqCst);
    });

    let c2 = count2.clone();
    let s2 = signal.clone();
    let effect2 = effect(move || {
        let _ = s2.get();
        c2.fetch_add(1, Ordering::SeqCst);
    });

    let c3 = count3.clone();
    let s3 = signal.clone();
    let _effect3 = effect(move || {
        let _ = s3.get();
        c3.fetch_add(1, Ordering::SeqCst);
    });

    // All effects run initially
    assert_eq!(count1.load(Ordering::SeqCst), 1);
    assert_eq!(count2.load(Ordering::SeqCst), 1);
    assert_eq!(count3.load(Ordering::SeqCst), 1);

    signal.set(1);
    assert_eq!(count1.load(Ordering::SeqCst), 2);
    assert_eq!(count2.load(Ordering::SeqCst), 2);
    assert_eq!(count3.load(Ordering::SeqCst), 2);

    // Stop first two effects
    effect1.stop();
    effect2.stop();

    signal.set(2);
    assert_eq!(count1.load(Ordering::SeqCst), 2); // Stopped
    assert_eq!(count2.load(Ordering::SeqCst), 2); // Stopped
    assert_eq!(count3.load(Ordering::SeqCst), 3); // Still running
}
