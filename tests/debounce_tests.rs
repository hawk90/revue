//! Debounce and throttle utilities tests

use revue::utils::debounce::{
    debounce_ms, debouncer, throttle, throttle_ms, Debouncer, Edge, Throttle,
};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_debouncer_new() {
    let d = Debouncer::new(Duration::from_millis(100));
    assert_eq!(d.delay(), Duration::from_millis(100));
    assert!(!d.is_pending());
}

#[test]
fn test_debouncer_trailing_edge() {
    let mut d = Debouncer::new(Duration::from_millis(50));

    // First call should not execute (trailing edge)
    assert!(!d.call());
    assert!(d.is_pending());

    // Not ready yet
    assert!(!d.is_ready());

    // Wait for debounce period
    sleep(Duration::from_millis(60));

    // Now should be ready
    assert!(d.is_ready());
    assert!(!d.is_pending());
}

#[test]
fn test_debouncer_leading_edge() {
    let mut d = Debouncer::leading(Duration::from_millis(50));

    // First call should execute immediately
    assert!(d.call());
    assert!(!d.is_pending());

    // Second call should not execute
    assert!(!d.call());
    assert!(!d.is_pending()); // Leading doesn't have trailing

    // Wait and call again
    sleep(Duration::from_millis(60));
    // After is_ready resets, next call should execute
    d.reset();
    assert!(d.call());
}

#[test]
fn test_debouncer_both_edges() {
    let mut d = Debouncer::both_edges(Duration::from_millis(50));

    // First call should execute (leading)
    assert!(d.call());

    // Second call should not execute immediately
    assert!(!d.call());
    assert!(d.is_pending());

    // Wait for trailing edge
    sleep(Duration::from_millis(60));

    // Trailing edge should fire
    assert!(d.is_ready());
}

#[test]
fn test_debouncer_cancel() {
    let mut d = Debouncer::new(Duration::from_millis(100));

    d.call();
    assert!(d.is_pending());

    d.cancel();
    assert!(!d.is_pending());
    assert!(d.remaining().is_none());
}

#[test]
fn test_debouncer_remaining() {
    let mut d = Debouncer::new(Duration::from_millis(100));

    assert!(d.remaining().is_none());

    d.call();
    let remaining = d.remaining().unwrap();
    assert!(remaining <= Duration::from_millis(100));
    assert!(remaining > Duration::ZERO);
}

#[test]
fn test_throttle_new() {
    let t = Throttle::new(Duration::from_millis(100));
    assert_eq!(t.interval(), Duration::from_millis(100));
    assert!(!t.is_pending());
}

#[test]
fn test_throttle_leading_edge() {
    let mut t = Throttle::new(Duration::from_millis(50));

    // First call should execute
    assert!(t.call());

    // Second call should not execute (within interval)
    assert!(!t.call());

    // Wait for interval
    sleep(Duration::from_millis(60));

    // Now should execute
    assert!(t.call());
}

#[test]
fn test_throttle_trailing_edge() {
    let mut t = Throttle::trailing(Duration::from_millis(50));

    // First call should not execute (trailing)
    assert!(!t.call());
    assert!(t.is_pending());

    // Check is_ready immediately - should execute since no last_exec
    assert!(t.is_ready());
    assert!(!t.is_pending());
}

#[test]
fn test_throttle_both_edges() {
    let mut t = Throttle::both_edges(Duration::from_millis(50));

    // First call should execute (leading)
    assert!(t.call());

    // Second call should mark pending
    assert!(!t.call());
    assert!(t.is_pending());

    // Wait for interval
    sleep(Duration::from_millis(60));

    // Trailing should fire
    assert!(t.is_ready());
}

#[test]
fn test_throttle_remaining() {
    let mut t = Throttle::new(Duration::from_millis(100));

    assert_eq!(t.remaining(), Duration::ZERO);

    t.call();
    let remaining = t.remaining();
    assert!(remaining <= Duration::from_millis(100));
}

#[test]
fn test_throttle_reset() {
    let mut t = Throttle::new(Duration::from_millis(100));

    t.call();
    assert!(!t.call()); // Throttled

    t.reset();
    assert!(t.call()); // Can call again after reset
}

#[test]
fn test_helper_functions() {
    let d = debouncer(Duration::from_millis(100));
    assert_eq!(d.delay(), Duration::from_millis(100));

    let d = debounce_ms(200);
    assert_eq!(d.delay(), Duration::from_millis(200));

    let t = throttle(Duration::from_millis(100));
    assert_eq!(t.interval(), Duration::from_millis(100));

    let t = throttle_ms(200);
    assert_eq!(t.interval(), Duration::from_millis(200));
}

#[test]
fn test_debouncer_default() {
    let d = Debouncer::default();
    assert_eq!(d.delay(), Duration::from_millis(300));
}

#[test]
fn test_throttle_default() {
    let t = Throttle::default();
    assert_eq!(t.interval(), Duration::from_millis(100));
}

#[test]
fn test_debouncer_set_delay() {
    let mut d = Debouncer::new(Duration::from_millis(100));
    d.set_delay(Duration::from_millis(200));
    assert_eq!(d.delay(), Duration::from_millis(200));
}

#[test]
fn test_throttle_set_interval() {
    let mut t = Throttle::new(Duration::from_millis(100));
    t.set_interval(Duration::from_millis(200));
    assert_eq!(t.interval(), Duration::from_millis(200));
}

#[test]
fn test_throttle_cancel() {
    let mut t = Throttle::trailing(Duration::from_millis(100));
    t.call();
    assert!(t.is_pending());

    t.cancel();
    assert!(!t.is_pending());
}

#[test]
fn test_edge_enum() {
    assert_eq!(Edge::default(), Edge::Trailing);

    // Test that with_edge creates functional debouncers with different edge behaviors
    let mut d_leading = Debouncer::new(Duration::from_millis(100)).with_edge(Edge::Leading);
    // Leading edge should execute immediately on first call
    assert!(d_leading.call());

    let mut d_trailing = Debouncer::new(Duration::from_millis(100));
    // Trailing edge should not execute immediately
    assert!(!d_trailing.call());

    let mut d_both = Debouncer::new(Duration::from_millis(100)).with_edge(Edge::Both);
    // Both edges should execute on leading edge
    assert!(d_both.call());

    // Test throttle with different edges
    let mut t_leading = Throttle::new(Duration::from_millis(100)).with_edge(Edge::Leading);
    assert!(t_leading.call());

    let mut t_both = Throttle::new(Duration::from_millis(100)).with_edge(Edge::Both);
    // Both edges should execute on leading edge
    assert!(t_both.call());
}
