//! Integration tests for once utilities
//! Extracted from src/utils/once.rs

use revue::utils::once::*;

#[test]
fn test_once_new() {
    let once = Once::new();
    assert!(!once.is_called());
}

#[test]
fn test_once_single_call() {
    let mut once = Once::new();
    assert!(once.call());
    assert!(once.is_called());
}

#[test]
fn test_once_multiple_calls() {
    let mut once = Once::new();
    assert!(once.call()); // First
    assert!(!once.call()); // Second
    assert!(!once.call()); // Third
    assert!(!once.call()); // Fourth
    assert!(once.is_called());
}

#[test]
fn test_once_reset() {
    let mut once = Once::new();
    assert!(once.call());
    assert!(!once.call());

    once.reset();
    assert!(!once.is_called());
    assert!(once.call());
    assert!(!once.call());
}

#[test]
fn test_once_from_true() {
    let mut once = Once::from(true);
    assert!(once.is_called());
    assert!(!once.call());
}

#[test]
fn test_once_from_false() {
    let mut once = Once::from(false);
    assert!(!once.is_called());
    assert!(once.call());
}

#[test]
fn test_once_helper() {
    let mut one_shot = once();
    assert!(one_shot.call());
    assert!(!one_shot.call());
}

#[test]
fn test_once_default() {
    let once = Once::default();
    assert!(!once.is_called());
}

#[test]
fn test_once_clone() {
    let mut once1 = Once::new();
    once1.call();

    let mut once2 = once1.clone();
    assert!(once2.is_called());
    assert!(!once2.call());
}

#[test]
fn test_once_clone_uncalled() {
    let once1 = Once::new();
    let mut once2 = once1.clone();

    assert!(once2.call());
    // Cloned Once has its own independent state
    assert!(!once1.is_called());
}

#[test]
fn test_once_in_loop() {
    let mut once = Once::new();
    let mut count = 0;

    for _ in 0..100 {
        if once.call() {
            count += 1;
        }
    }

    assert_eq!(count, 1);
}

#[test]
fn test_once_with_callback() {
    let mut once = Once::new();
    let mut executed = false;

    for _ in 0..10 {
        if once.call() {
            executed = true;
        }
    }

    assert!(executed);
}
