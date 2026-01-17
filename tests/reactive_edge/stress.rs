//! Stress tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn test_many_signals() {
    // Create many signals and ensure they all work independently
    let signals: Vec<_> = (0..100).map(|i| signal(i as i32)).collect();

    // Verify initial values
    for (i, sig) in signals.iter().enumerate() {
        assert_eq!(sig.get(), i as i32);
    }

    // Update all signals
    for (i, sig) in signals.iter().enumerate() {
        sig.set((i * 2) as i32);
    }

    // Verify updated values
    for (i, sig) in signals.iter().enumerate() {
        assert_eq!(sig.get(), (i * 2) as i32);
    }
}

#[test]
fn test_many_effects_on_one_signal() {
    let signal = signal(0);
    let counts: Vec<_> = (0..50).map(|_| Arc::new(AtomicUsize::new(0))).collect();

    let _effects: Vec<_> = counts
        .iter()
        .map(|count| {
            let c = count.clone();
            let s = signal.clone();
            effect(move || {
                let _ = s.get();
                c.fetch_add(1, Ordering::SeqCst);
            })
        })
        .collect();

    // All effects run initially
    for count in &counts {
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    // Update signal
    signal.set(1);

    // All effects should have run again
    for count in &counts {
        assert_eq!(count.load(Ordering::SeqCst), 2);
    }

    signal.set(2);

    // All effects should have run again
    for count in &counts {
        assert_eq!(count.load(Ordering::SeqCst), 3);
    }
}

#[test]
fn test_signal_id_uniqueness() {
    // Ensure all signals get unique IDs
    let signals: Vec<_> = (0..1000).map(|i| signal(i)).collect();

    let mut ids = std::collections::HashSet::new();
    for sig in &signals {
        let id = sig.id();
        assert!(ids.insert(id), "Duplicate signal ID found: {:?}", id);
    }

    assert_eq!(ids.len(), 1000);
}
