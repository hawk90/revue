//! Tests for Timeout
//!
//! Extracted from src/widget/traits/timeout.rs

use revue::widget::traits::Timeout;
use std::thread;
use std::time::Duration;

#[test]
fn test_timeout_new() {
    let timeout: Timeout<String> = Timeout::new(Duration::from_secs(5));
    assert!(!timeout.is_set());
    assert!(timeout.get().is_none());
}

#[test]
fn test_timeout_secs() {
    let timeout: Timeout<String> = Timeout::secs(5);
    assert!(!timeout.is_set());
}

#[test]
fn test_timeout_millis() {
    let timeout: Timeout<String> = Timeout::millis(500);
    assert!(!timeout.is_set());
}

#[test]
fn test_timeout_default() {
    let timeout: Timeout<String> = Timeout::default();
    assert!(!timeout.is_set());
}

#[test]
fn test_timeout_set_and_get() {
    let mut timeout = Timeout::secs(5);
    timeout.set("hello".to_string());
    assert!(timeout.is_set());
    assert_eq!(timeout.get(), Some(&"hello".to_string()));
}

#[test]
fn test_timeout_get_mut() {
    let mut timeout = Timeout::secs(5);
    timeout.set("hello".to_string());
    if let Some(val) = timeout.get_mut() {
        *val = "world".to_string();
    }
    assert_eq!(timeout.get(), Some(&"world".to_string()));
}

#[test]
fn test_timeout_clear() {
    let mut timeout = Timeout::secs(5);
    timeout.set("hello".to_string());
    assert!(timeout.is_set());
    timeout.clear();
    assert!(!timeout.is_set());
    assert!(timeout.get().is_none());
}

#[test]
fn test_timeout_take() {
    let mut timeout = Timeout::secs(5);
    timeout.set("hello".to_string());
    let value = timeout.take();
    assert_eq!(value, Some("hello".to_string()));
    assert!(!timeout.is_set());
}

#[test]
fn test_timeout_update() {
    let mut timeout = Timeout::secs(5);
    timeout.set("hello".to_string());
    timeout.update("world".to_string());
    assert_eq!(timeout.get(), Some(&"world".to_string()));
}

#[test]
fn test_timeout_is_expired_not_set() {
    let timeout: Timeout<String> = Timeout::millis(10);
    assert!(!timeout.is_expired());
}

#[test]
fn test_timeout_is_expired_not_yet() {
    let mut timeout = Timeout::secs(10);
    timeout.set("hello".to_string());
    assert!(!timeout.is_expired());
}

#[test]
fn test_timeout_is_expired_after_duration() {
    let mut timeout = Timeout::millis(10);
    timeout.set("hello".to_string());
    thread::sleep(Duration::from_millis(20));
    assert!(timeout.is_expired());
}

#[test]
fn test_timeout_remaining_not_set() {
    let timeout: Timeout<String> = Timeout::secs(5);
    assert!(timeout.remaining().is_none());
}

#[test]
fn test_timeout_remaining_some() {
    let mut timeout = Timeout::secs(10);
    timeout.set("hello".to_string());
    let remaining = timeout.remaining();
    assert!(remaining.is_some());
    assert!(remaining.unwrap() <= Duration::from_secs(10));
}

#[test]
fn test_timeout_remaining_zero_after_expiry() {
    let mut timeout = Timeout::millis(10);
    timeout.set("hello".to_string());
    thread::sleep(Duration::from_millis(20));
    let remaining = timeout.remaining();
    assert_eq!(remaining, Some(Duration::ZERO));
}

#[test]
fn test_timeout_reset_timer() {
    let mut timeout = Timeout::millis(50);
    timeout.set("hello".to_string());
    thread::sleep(Duration::from_millis(30));
    timeout.reset_timer();
    // After reset, should not be expired yet
    assert!(!timeout.is_expired());
}

#[test]
fn test_timeout_reset_timer_no_value() {
    let mut timeout: Timeout<String> = Timeout::secs(5);
    timeout.reset_timer(); // Should do nothing when no value
    assert!(!timeout.is_set());
}

#[test]
fn test_timeout_auto_clear_not_expired() {
    let mut timeout = Timeout::secs(10);
    timeout.set("hello".to_string());
    let value = timeout.auto_clear();
    assert_eq!(value, Some(&"hello".to_string()));
    assert!(timeout.is_set());
}

#[test]
fn test_timeout_auto_clear_expired() {
    let mut timeout = Timeout::millis(10);
    timeout.set("hello".to_string());
    thread::sleep(Duration::from_millis(20));
    let value = timeout.auto_clear();
    assert!(value.is_none());
    assert!(!timeout.is_set());
}
