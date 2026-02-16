//! Tests for easing functions module
//!
//! Extracted from src/utils/easing/functions.rs

use revue::utils::easing::*;

// =========================================================================
// Linear tests
// =========================================================================

#[test]
fn test_linear() {
    assert_eq!(linear(0.0), 0.0);
    assert_eq!(linear(0.5), 0.5);
    assert_eq!(linear(1.0), 1.0);
}

#[test]
fn test_linear_clamp_below() {
    assert_eq!(linear(-0.5), 0.0);
}

#[test]
fn test_linear_clamp_above() {
    assert_eq!(linear(1.5), 1.0);
}

// =========================================================================
// Quadratic tests
// =========================================================================

#[test]
fn test_ease_in_quad() {
    assert_eq!(ease_in_quad(0.0), 0.0);
    assert_eq!(ease_in_quad(0.5), 0.25);
    assert_eq!(ease_in_quad(1.0), 1.0);
}

#[test]
fn test_ease_out_quad() {
    assert_eq!(ease_out_quad(0.0), 0.0);
    assert_eq!(ease_out_quad(0.5), 0.75);
    assert_eq!(ease_out_quad(1.0), 1.0);
}

#[test]
fn test_ease_in_out_quad() {
    assert_eq!(ease_in_out_quad(0.0), 0.0);
    assert_eq!(ease_in_out_quad(0.25), 0.125);
    assert_eq!(ease_in_out_quad(0.5), 0.5);
    assert_eq!(ease_in_out_quad(0.75), 0.875);
    assert_eq!(ease_in_out_quad(1.0), 1.0);
}

// =========================================================================
// Cubic tests
// =========================================================================

#[test]
fn test_ease_in_cubic() {
    assert_eq!(ease_in_cubic(0.0), 0.0);
    assert_eq!(ease_in_cubic(0.5), 0.125);
    assert_eq!(ease_in_cubic(1.0), 1.0);
}

#[test]
fn test_ease_out_cubic() {
    assert_eq!(ease_out_cubic(0.0), 0.0);
    assert_eq!(ease_out_cubic(0.5), 0.875);
    assert_eq!(ease_out_cubic(1.0), 1.0);
}

#[test]
fn test_ease_in_out_cubic() {
    assert_eq!(ease_in_out_cubic(0.0), 0.0);
    assert_eq!(ease_in_out_cubic(0.5), 0.5);
    assert_eq!(ease_in_out_cubic(1.0), 1.0);
}

// =========================================================================
// Quartic tests
// =========================================================================

#[test]
fn test_ease_in_quart() {
    assert_eq!(ease_in_quart(0.0), 0.0);
    assert_eq!(ease_in_quart(0.5), 0.0625);
    assert_eq!(ease_in_quart(1.0), 1.0);
}

#[test]
fn test_ease_out_quart() {
    assert_eq!(ease_out_quart(0.0), 0.0);
    assert_eq!(ease_out_quart(0.5), 0.9375);
    assert_eq!(ease_out_quart(1.0), 1.0);
}

#[test]
fn test_ease_in_out_quart() {
    assert_eq!(ease_in_out_quart(0.0), 0.0);
    assert_eq!(ease_in_out_quart(1.0), 1.0);
}

// =========================================================================
// Quintic tests
// =========================================================================

#[test]
fn test_ease_in_quint() {
    assert_eq!(ease_in_quint(0.0), 0.0);
    assert_eq!(ease_in_quint(0.5), 0.03125);
    assert_eq!(ease_in_quint(1.0), 1.0);
}

#[test]
fn test_ease_out_quint() {
    assert_eq!(ease_out_quint(0.0), 0.0);
    assert_eq!(ease_out_quint(0.5), 0.96875);
    assert_eq!(ease_out_quint(1.0), 1.0);
}

#[test]
fn test_ease_in_out_quint() {
    assert_eq!(ease_in_out_quint(0.0), 0.0);
    assert_eq!(ease_in_out_quint(1.0), 1.0);
}

// =========================================================================
// Sine tests
// =========================================================================

#[test]
fn test_ease_in_sine() {
    let result0 = ease_in_sine(0.0);
    let result1 = ease_in_sine(1.0);
    assert!((result0 - 0.0).abs() < 0.001);
    assert!((result1 - 1.0).abs() < 0.001);
}

#[test]
fn test_ease_out_sine() {
    let result0 = ease_out_sine(0.0);
    let result1 = ease_out_sine(1.0);
    assert!((result0 - 0.0).abs() < 0.001);
    assert!((result1 - 1.0).abs() < 0.001);
}

#[test]
fn test_ease_in_out_sine() {
    let result0 = ease_in_out_sine(0.0);
    let result05 = ease_in_out_sine(0.5);
    let result1 = ease_in_out_sine(1.0);
    assert!((result0 - 0.0).abs() < 0.001);
    assert!((result05 - 0.5).abs() < 0.001);
    assert!((result1 - 1.0).abs() < 0.001);
}

// =========================================================================
// Exponential tests
// =========================================================================

#[test]
fn test_ease_in_expo_zero() {
    assert_eq!(ease_in_expo(0.0), 0.0);
}

#[test]
fn test_ease_in_expo_mid() {
    let result = ease_in_expo(0.5);
    assert!(result > 0.0);
    assert!(result < 1.0);
}

#[test]
fn test_ease_out_expo_one() {
    assert_eq!(ease_out_expo(1.0), 1.0);
}

#[test]
fn test_ease_out_expo_zero() {
    let result = ease_out_expo(0.0);
    // ease_out_expo(0.0) should be very close to 0, but not quite
    assert!(result >= 0.0);
    assert!(result <= 1.0);
}

#[test]
fn test_ease_in_out_expo_edges() {
    assert_eq!(ease_in_out_expo(0.0), 0.0);
    assert_eq!(ease_in_out_expo(1.0), 1.0);
}

// =========================================================================
// Circular tests
// =========================================================================

#[test]
fn test_ease_in_circ() {
    assert_eq!(ease_in_circ(0.0), 0.0);
    assert_eq!(ease_in_circ(1.0), 1.0);
}

#[test]
fn test_ease_out_circ() {
    assert_eq!(ease_out_circ(0.0), 0.0);
    assert_eq!(ease_out_circ(1.0), 1.0);
}

#[test]
fn test_ease_in_out_circ() {
    assert_eq!(ease_in_out_circ(0.0), 0.0);
    assert_eq!(ease_in_out_circ(0.5), 0.5);
    assert_eq!(ease_in_out_circ(1.0), 1.0);
}

// =========================================================================
// Back tests
// =========================================================================

#[test]
fn test_ease_in_back() {
    let result = ease_in_back(0.0);
    assert_eq!(result, 0.0);
}

#[test]
fn test_ease_out_back() {
    let result = ease_out_back(1.0);
    assert_eq!(result, 1.0);
}

#[test]
fn test_ease_in_out_back() {
    // Just verify no panic at key points
    let _ = ease_in_out_back(0.0);
    let _ = ease_in_out_back(0.5);
    let _ = ease_in_out_back(1.0);
}

// =========================================================================
// Elastic tests
// =========================================================================

#[test]
fn test_ease_in_elastic_edges() {
    assert_eq!(ease_in_elastic(0.0), 0.0);
    assert_eq!(ease_in_elastic(1.0), 1.0);
}

#[test]
fn test_ease_out_elastic_edges() {
    assert_eq!(ease_out_elastic(0.0), 0.0);
    assert_eq!(ease_out_elastic(1.0), 1.0);
}

#[test]
fn test_ease_in_out_elastic_edges() {
    assert_eq!(ease_in_out_elastic(0.0), 0.0);
    assert_eq!(ease_in_out_elastic(1.0), 1.0);
}

// =========================================================================
// Bounce tests
// =========================================================================

#[test]
fn test_ease_out_bounce() {
    assert_eq!(ease_out_bounce(0.0), 0.0);
    assert_eq!(ease_out_bounce(1.0), 1.0);
}

#[test]
fn test_ease_in_bounce() {
    assert_eq!(ease_in_bounce(0.0), 0.0);
    assert_eq!(ease_in_bounce(1.0), 1.0);
}

#[test]
fn test_ease_in_out_bounce() {
    assert_eq!(ease_in_out_bounce(0.0), 0.0);
    assert_eq!(ease_in_out_bounce(1.0), 1.0);
}

#[test]
fn test_ease_in_out_bounce_mid() {
    let result = ease_in_out_bounce(0.5);
    // Bounce should be around 0.5 at midpoint
    assert!(result > 0.0);
    assert!(result < 1.0);
}
