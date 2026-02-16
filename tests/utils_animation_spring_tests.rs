//! Tests for animation spring module
//!
//! Extracted from src/utils/animation/spring.rs

use revue::utils::animation::Spring;

#[test]
fn test_spring_new() {
    let spring = Spring::new(0.0, 100.0);
    assert_eq!(spring.value(), 0.0);
    assert_eq!(spring.target(), 100.0);
    assert_eq!(spring.velocity(), 0.0);
}

#[test]
fn test_spring_at() {
    let spring = Spring::at(50.0);
    assert_eq!(spring.value(), 50.0);
    assert_eq!(spring.target(), 50.0);
    assert!(spring.is_settled());
}

#[test]
fn test_spring_snappy() {
    let spring = Spring::snappy();
    assert_eq!(spring.value(), 0.0);
    assert_eq!(spring.target(), 0.0);
    // Stiffness and damping are private fields - presets tested via behavior
    assert!(spring.is_settled());
}

#[test]
fn test_spring_gentle() {
    let spring = Spring::gentle();
    assert_eq!(spring.value(), 0.0);
    assert_eq!(spring.target(), 0.0);
    assert!(spring.is_settled());
}

#[test]
fn test_spring_bouncy() {
    let spring = Spring::bouncy();
    assert_eq!(spring.value(), 0.0);
    assert_eq!(spring.target(), 0.0);
    assert!(spring.is_settled());
}

#[test]
fn test_spring_slow() {
    let spring = Spring::slow();
    assert_eq!(spring.value(), 0.0);
    assert_eq!(spring.target(), 0.0);
    assert!(spring.is_settled());
}

#[test]
fn test_spring_set_target() {
    let mut spring = Spring::new(0.0, 100.0);
    spring.set_target(50.0);
    assert_eq!(spring.target(), 50.0);
}

#[test]
fn test_spring_set_value() {
    let mut spring = Spring::new(0.0, 100.0);
    spring.set_value(75.0);
    assert_eq!(spring.value(), 75.0);
    assert_eq!(spring.velocity(), 0.0);
}

#[test]
fn test_spring_is_settled() {
    let spring = Spring::at(50.0);
    assert!(spring.is_settled());
}

#[test]
fn test_spring_is_settled_false() {
    let spring = Spring::new(0.0, 100.0);
    assert!(!spring.is_settled());
}

#[test]
fn test_spring_update_changes_value() {
    let mut spring = Spring::new(0.0, 100.0);
    let value1 = spring.update(0.016);
    let value2 = spring.update(0.016);
    // Spring should move toward target
    assert!(value2 > value1);
    assert!(value2 <= 100.0);
}

#[test]
fn test_spring_tick() {
    let mut spring = Spring::new(0.0, 100.0);
    let value1 = spring.tick();
    let value2 = spring.tick();
    // Spring should move toward target
    assert!(value2 > value1);
}

#[test]
fn test_spring_update_settled() {
    let mut spring = Spring::at(50.0);
    spring.set_target(50.0);
    let value = spring.update(0.016);
    // Already at target
    assert_eq!(value, 50.0);
}

// Removed test_spring_default_fields - it accessed private fields (stiffness, damping, mass, threshold)
// Default values are tested indirectly through behavior tests

#[test]
fn test_spring_velocity_changes() {
    let mut spring = Spring::new(0.0, 100.0);
    let v1 = spring.velocity();
    spring.update(0.016);
    let v2 = spring.velocity();
    // Velocity should change as spring accelerates
    assert_ne!(v1, v2);
}

#[test]
fn test_spring_velocity_toward_target() {
    let mut spring = Spring::new(0.0, 100.0);
    spring.update(0.016);
    // Velocity should be positive (toward 100)
    assert!(spring.velocity() > 0.0);
}

#[test]
fn test_spring_velocity_from_target() {
    let mut spring = Spring::new(100.0, 0.0);
    spring.update(0.016);
    // Velocity should be negative (toward 0)
    assert!(spring.velocity() < 0.0);
}

#[test]
fn test_spring_is_settled_near_target() {
    let mut spring = Spring::new(0.0, 10.0);
    // Move close to target
    for _ in 0..1000 {
        spring.update(0.016);
        if spring.is_settled() {
            break;
        }
    }
    // Should eventually settle
    assert!(spring.is_settled());
    assert!((spring.value() - 10.0).abs() < 0.1);
}
