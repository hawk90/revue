//! Tests for easing helpers module
//!
//! Extracted from src/utils/easing/helpers.rs

use revue::utils::easing::{lerp, lerp_fn, Easing, Interpolator};

// =========================================================================
// lerp() tests
// =========================================================================

#[test]
fn test_lerp_linear_start() {
    let result = lerp(0.0, 100.0, 0.0, Easing::Linear);
    assert_eq!(result, 0.0);
}

#[test]
fn test_lerp_linear_end() {
    let result = lerp(0.0, 100.0, 1.0, Easing::Linear);
    assert_eq!(result, 100.0);
}

#[test]
fn test_lerp_linear_mid() {
    let result = lerp(0.0, 100.0, 0.5, Easing::Linear);
    assert_eq!(result, 50.0);
}

#[test]
fn test_lerp_with_easing() {
    let result = lerp(0.0, 100.0, 0.5, Easing::OutQuad);
    // OutQuad at 0.5: 1 - (1-0.5)^2 = 1 - 0.25 = 0.75
    assert!((result - 75.0).abs() < 0.001);
}

#[test]
fn test_lerp_reverse_range() {
    let result = lerp(100.0, 0.0, 0.5, Easing::Linear);
    assert_eq!(result, 50.0);
}

#[test]
fn test_lerp_negative_start() {
    let result = lerp(-50.0, 50.0, 0.5, Easing::Linear);
    assert_eq!(result, 0.0);
}

#[test]
fn test_lerp_clamp_low() {
    // t < 0 is clamped to 0 by Linear easing
    let result = lerp(0.0, 100.0, -0.5, Easing::Linear);
    assert_eq!(result, 0.0);
}

#[test]
fn test_lerp_clamp_high() {
    // t > 1 is clamped to 1 by Linear easing
    let result = lerp(0.0, 100.0, 1.5, Easing::Linear);
    assert_eq!(result, 100.0);
}

// =========================================================================
// lerp_fn() tests
// =========================================================================

#[test]
fn test_lerp_fn_linear() {
    let result = lerp_fn(0.0, 100.0, 0.5, |t| t.clamp(0.0, 1.0));
    assert_eq!(result, 50.0);
}

#[test]
fn test_lerp_fn_custom_easing() {
    let result = lerp_fn(0.0, 100.0, 0.5, |t| t * t);
    assert_eq!(result, 25.0);
}

#[test]
fn test_lerp_fn_ease_in() {
    let result = lerp_fn(0.0, 100.0, 0.5, |t| t * t);
    assert_eq!(result, 25.0);
}

#[test]
fn test_lerp_fn_ease_out() {
    let result = lerp_fn(0.0, 100.0, 0.5, |t| 1.0 - (1.0 - t) * (1.0 - t));
    assert!((result - 75.0).abs() < 0.001);
}

// =========================================================================
// Interpolator::new() tests
// =========================================================================

#[test]
fn test_interpolator_new() {
    let interp = Interpolator::new(0.0, 100.0);
    assert_eq!(interp.start, 0.0);
    assert_eq!(interp.end, 100.0);
    assert_eq!(interp.easing, Easing::Linear);
}

#[test]
fn test_interpolator_new_negative() {
    let interp = Interpolator::new(-50.0, 50.0);
    assert_eq!(interp.start, -50.0);
    assert_eq!(interp.end, 50.0);
}

// =========================================================================
// Interpolator::easing() tests
// =========================================================================

#[test]
fn test_interpolator_easing_setter() {
    let interp = Interpolator::new(0.0, 100.0).easing(Easing::InQuad);
    assert_eq!(interp.easing, Easing::InQuad);
}

#[test]
fn test_interpolator_easing_chainable() {
    let interp = Interpolator::new(0.0, 100.0)
        .easing(Easing::OutCubic)
        .easing(Easing::InElastic); // Last setter wins
    assert_eq!(interp.easing, Easing::InElastic);
}

// =========================================================================
// Interpolator::at() tests
// =========================================================================

#[test]
fn test_interpolator_at_zero() {
    let interp = Interpolator::new(0.0, 100.0);
    assert_eq!(interp.at(0.0), 0.0);
}

#[test]
fn test_interpolator_at_one() {
    let interp = Interpolator::new(0.0, 100.0);
    assert_eq!(interp.at(1.0), 100.0);
}

#[test]
fn test_interpolator_at_half() {
    let interp = Interpolator::new(0.0, 100.0);
    assert_eq!(interp.at(0.5), 50.0);
}

#[test]
fn test_interpolator_at_with_easing() {
    let interp = Interpolator::new(0.0, 100.0).easing(Easing::OutQuad);
    assert!((interp.at(0.5) - 75.0).abs() < 0.001);
}

#[test]
fn test_interpolator_at_clamp_below() {
    // Linear easing clamps t to [0, 1]
    let interp = Interpolator::new(0.0, 100.0);
    assert_eq!(interp.at(-0.5), 0.0);
}

#[test]
fn test_interpolator_at_clamp_above() {
    // Linear easing clamps t to [0, 1]
    let interp = Interpolator::new(0.0, 100.0);
    assert_eq!(interp.at(1.5), 100.0);
}

#[test]
fn test_interpolator_at_reverse_range() {
    let interp = Interpolator::new(100.0, 0.0);
    assert_eq!(interp.at(0.5), 50.0);
}

#[test]
fn test_interpolator_at_in_ease() {
    let interp = Interpolator::new(0.0, 100.0).easing(Easing::InQuad);
    // InQuad: t^2, at t=0.5: 0.25
    assert!((interp.at(0.5) - 25.0).abs() < 0.001);
}

#[test]
fn test_interpolator_at_out_ease() {
    let interp = Interpolator::new(0.0, 100.0).easing(Easing::OutQuad);
    // OutQuad: 1-(1-t)^2, at t=0.5: 0.75
    assert!((interp.at(0.5) - 75.0).abs() < 0.001);
}

#[test]
fn test_interpolator_at_in_out_ease() {
    let interp = Interpolator::new(0.0, 100.0).easing(Easing::InOutQuad);
    // InOutQuad: <0.5: 2t^2, >=0.5: 1-2(1-t)^2
    // At t=0.5: exactly 0.5
    assert!((interp.at(0.5) - 50.0).abs() < 0.001);
}

// =========================================================================
// Interpolator clone tests
// =========================================================================

#[test]
fn test_interpolator_clone() {
    let interp1 = Interpolator::new(0.0, 100.0).easing(Easing::OutBounce);
    let interp2 = interp1.clone();
    assert_eq!(interp1.start, interp2.start);
    assert_eq!(interp1.end, interp2.end);
    assert_eq!(interp1.easing, interp2.easing);
}

// =========================================================================
// Interpolator builder chain tests
// =========================================================================

#[test]
fn test_interpolator_builder_chain() {
    let interp = Interpolator::new(10.0, 90.0).easing(Easing::InOutCubic);
    assert_eq!(interp.start, 10.0);
    assert_eq!(interp.end, 90.0);
    assert_eq!(interp.easing, Easing::InOutCubic);
}
