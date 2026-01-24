#![allow(unused_imports)]
//! Tests for easing functions

use crate::utils::easing::{
    functions::{
        ease_in_back, ease_in_bounce, ease_in_circ, ease_in_cubic, ease_in_elastic, ease_in_expo,
        ease_in_out_back, ease_in_out_bounce, ease_in_out_circ, ease_in_out_cubic,
        ease_in_out_elastic, ease_in_out_expo, ease_in_out_quad, ease_in_out_quart,
        ease_in_out_quint, ease_in_out_sine, ease_in_quad, ease_in_quart, ease_in_quint,
        ease_in_sine, ease_out_back, ease_out_bounce, ease_out_circ, ease_out_cubic,
        ease_out_elastic, ease_out_expo, ease_out_quad, ease_out_quart, ease_out_quint,
        ease_out_sine, linear,
    },
    helpers::{lerp, lerp_fn, Interpolator},
    types::Easing,
};

#[test]
fn test_linear() {
    assert_eq!(linear(0.0), 0.0);
    assert_eq!(linear(0.5), 0.5);
    assert_eq!(linear(1.0), 1.0);
}

#[test]
fn test_ease_out_quad() {
    assert_eq!(ease_out_quad(0.0), 0.0);
    assert_eq!(ease_out_quad(1.0), 1.0);
    // Ease out should be faster at start
    assert!(ease_out_quad(0.5) > 0.5);
}

#[test]
fn test_ease_in_quad() {
    assert_eq!(ease_in_quad(0.0), 0.0);
    assert_eq!(ease_in_quad(1.0), 1.0);
    // Ease in should be slower at start
    assert!(ease_in_quad(0.5) < 0.5);
}

#[test]
fn test_ease_in_out_quad() {
    assert_eq!(ease_in_out_quad(0.0), 0.0);
    assert_eq!(ease_in_out_quad(1.0), 1.0);
    assert!((ease_in_out_quad(0.5) - 0.5).abs() < 0.001);
}

#[test]
fn test_easing_enum() {
    let easing = Easing::OutQuad;
    assert_eq!(easing.ease(0.0), 0.0);
    assert_eq!(easing.ease(1.0), 1.0);
}

#[test]
fn test_lerp() {
    let value = lerp(0.0, 100.0, 0.5, Easing::Linear);
    assert!((value - 50.0).abs() < 0.001);

    let value = lerp(0.0, 100.0, 0.5, Easing::OutQuad);
    assert!(value > 50.0); // Ease out is faster
}

#[test]
fn test_interpolator() {
    let interp = Interpolator::new(0.0, 100.0).easing(Easing::OutQuad);

    assert_eq!(interp.at(0.0), 0.0);
    assert_eq!(interp.at(1.0), 100.0);
}

#[test]
fn test_all_easings_boundary() {
    let easings = [
        Easing::Linear,
        Easing::InQuad,
        Easing::OutQuad,
        Easing::InOutQuad,
        Easing::InCubic,
        Easing::OutCubic,
        Easing::InOutCubic,
        Easing::InQuart,
        Easing::OutQuart,
        Easing::InOutQuart,
        Easing::InQuint,
        Easing::OutQuint,
        Easing::InOutQuint,
        Easing::InSine,
        Easing::OutSine,
        Easing::InOutSine,
        Easing::InExpo,
        Easing::OutExpo,
        Easing::InOutExpo,
        Easing::InCirc,
        Easing::OutCirc,
        Easing::InOutCirc,
        Easing::InBack,
        Easing::OutBack,
        Easing::InOutBack,
        Easing::InElastic,
        Easing::OutElastic,
        Easing::InOutElastic,
        Easing::InBounce,
        Easing::OutBounce,
        Easing::InOutBounce,
    ];

    for easing in easings {
        let start = easing.ease(0.0);
        let end = easing.ease(1.0);

        // All easings should start at ~0 and end at ~1
        // (Back and Elastic may slightly overshoot)
        assert!(start.abs() < 0.01, "{:?} start: {}", easing, start);
        assert!((end - 1.0).abs() < 0.01, "{:?} end: {}", easing, end);
    }
}

// =========================================================================
// Easing enum tests
// =========================================================================

#[test]
fn test_easing_default() {
    let easing = Easing::default();
    assert_eq!(easing, Easing::Linear);
}

#[test]
fn test_easing_clone() {
    let easing = Easing::OutQuad;
    let cloned = easing;
    assert_eq!(easing, cloned);
}

#[test]
fn test_easing_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Easing::Linear);
    set.insert(Easing::OutQuad);
    set.insert(Easing::Linear); // Duplicate
    assert_eq!(set.len(), 2);
}

#[test]
fn test_easing_function() {
    let easing = Easing::OutQuad;
    let func = easing.function();
    assert_eq!(func(0.0), ease_out_quad(0.0));
    assert_eq!(func(1.0), ease_out_quad(1.0));
}

// =========================================================================
// Individual easing function tests
// =========================================================================

#[test]
fn test_linear_clamps() {
    assert_eq!(linear(-0.5), 0.0);
    assert_eq!(linear(1.5), 1.0);
}

#[test]
fn test_ease_in_cubic() {
    assert_eq!(ease_in_cubic(0.0), 0.0);
    assert_eq!(ease_in_cubic(1.0), 1.0);
    assert!(ease_in_cubic(0.5) < 0.5);
}

#[test]
fn test_ease_out_cubic() {
    assert_eq!(ease_out_cubic(0.0), 0.0);
    assert_eq!(ease_out_cubic(1.0), 1.0);
    assert!(ease_out_cubic(0.5) > 0.5);
}

#[test]
fn test_ease_in_out_cubic() {
    assert_eq!(ease_in_out_cubic(0.0), 0.0);
    assert_eq!(ease_in_out_cubic(1.0), 1.0);
    assert!((ease_in_out_cubic(0.5) - 0.5).abs() < 0.001);
}

#[test]
fn test_ease_in_quart() {
    assert_eq!(ease_in_quart(0.0), 0.0);
    assert_eq!(ease_in_quart(1.0), 1.0);
}

#[test]
fn test_ease_out_quart() {
    assert_eq!(ease_out_quart(0.0), 0.0);
    assert_eq!(ease_out_quart(1.0), 1.0);
}

#[test]
fn test_ease_in_out_quart() {
    assert_eq!(ease_in_out_quart(0.0), 0.0);
    assert_eq!(ease_in_out_quart(1.0), 1.0);
}

#[test]
fn test_ease_in_quint() {
    assert_eq!(ease_in_quint(0.0), 0.0);
    assert_eq!(ease_in_quint(1.0), 1.0);
}

#[test]
fn test_ease_out_quint() {
    assert_eq!(ease_out_quint(0.0), 0.0);
    assert_eq!(ease_out_quint(1.0), 1.0);
}

#[test]
fn test_ease_in_out_quint() {
    assert_eq!(ease_in_out_quint(0.0), 0.0);
    assert_eq!(ease_in_out_quint(1.0), 1.0);
}

#[test]
fn test_ease_in_sine() {
    assert!((ease_in_sine(0.0) - 0.0).abs() < 0.001);
    assert!((ease_in_sine(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn test_ease_out_sine() {
    assert!((ease_out_sine(0.0) - 0.0).abs() < 0.001);
    assert!((ease_out_sine(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn test_ease_in_out_sine() {
    assert!((ease_in_out_sine(0.0) - 0.0).abs() < 0.001);
    assert!((ease_in_out_sine(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn test_ease_in_expo() {
    assert_eq!(ease_in_expo(0.0), 0.0);
    assert!((ease_in_expo(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn test_ease_out_expo() {
    assert!((ease_out_expo(0.0) - 0.0).abs() < 0.001);
    assert_eq!(ease_out_expo(1.0), 1.0);
}

#[test]
fn test_ease_in_out_expo() {
    assert_eq!(ease_in_out_expo(0.0), 0.0);
    assert_eq!(ease_in_out_expo(1.0), 1.0);
    assert!((ease_in_out_expo(0.5) - 0.5).abs() < 0.001);
}

#[test]
fn test_ease_in_circ() {
    assert!((ease_in_circ(0.0) - 0.0).abs() < 0.001);
    assert!((ease_in_circ(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn test_ease_out_circ() {
    assert!((ease_out_circ(0.0) - 0.0).abs() < 0.001);
    assert!((ease_out_circ(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn test_ease_in_out_circ() {
    assert!((ease_in_out_circ(0.0) - 0.0).abs() < 0.001);
    assert!((ease_in_out_circ(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn test_ease_in_back() {
    assert!((ease_in_back(0.0) - 0.0).abs() < 0.001);
    assert!((ease_in_back(1.0) - 1.0).abs() < 0.001);
    // Back overshoots (goes negative at start)
    assert!(ease_in_back(0.3) < 0.0);
}

#[test]
fn test_ease_out_back() {
    assert!((ease_out_back(0.0) - 0.0).abs() < 0.001);
    assert!((ease_out_back(1.0) - 1.0).abs() < 0.001);
    // Back overshoots (goes above 1 near end)
    assert!(ease_out_back(0.7) > 1.0);
}

#[test]
fn test_ease_in_out_back() {
    assert!((ease_in_out_back(0.0) - 0.0).abs() < 0.001);
    assert!((ease_in_out_back(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn test_ease_in_elastic() {
    assert_eq!(ease_in_elastic(0.0), 0.0);
    assert_eq!(ease_in_elastic(1.0), 1.0);
}

#[test]
fn test_ease_out_elastic() {
    assert_eq!(ease_out_elastic(0.0), 0.0);
    assert_eq!(ease_out_elastic(1.0), 1.0);
}

#[test]
fn test_ease_in_out_elastic() {
    assert_eq!(ease_in_out_elastic(0.0), 0.0);
    assert_eq!(ease_in_out_elastic(1.0), 1.0);
}

#[test]
fn test_ease_in_bounce() {
    assert!((ease_in_bounce(0.0) - 0.0).abs() < 0.001);
    assert!((ease_in_bounce(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn test_ease_out_bounce() {
    assert!((ease_out_bounce(0.0) - 0.0).abs() < 0.001);
    assert!((ease_out_bounce(1.0) - 1.0).abs() < 0.001);
}

#[test]
fn test_ease_in_out_bounce() {
    assert!((ease_in_out_bounce(0.0) - 0.0).abs() < 0.001);
    assert!((ease_in_out_bounce(1.0) - 1.0).abs() < 0.001);
}

// =========================================================================
// Lerp tests
// =========================================================================

#[test]
fn test_lerp_boundaries() {
    let value_start = lerp(10.0, 100.0, 0.0, Easing::Linear);
    let value_end = lerp(10.0, 100.0, 1.0, Easing::Linear);
    assert!((value_start - 10.0).abs() < 0.001);
    assert!((value_end - 100.0).abs() < 0.001);
}

#[test]
fn test_lerp_negative_range() {
    let value = lerp(-100.0, 100.0, 0.5, Easing::Linear);
    assert!((value - 0.0).abs() < 0.001);
}

#[test]
fn test_lerp_fn() {
    let value = lerp_fn(0.0, 100.0, 0.5, linear);
    assert!((value - 50.0).abs() < 0.001);
}

#[test]
fn test_lerp_fn_custom() {
    // Custom easing: always return 1.0
    let always_one: crate::utils::easing::EasingFn = |_| 1.0;
    let value = lerp_fn(0.0, 100.0, 0.5, always_one);
    assert!((value - 100.0).abs() < 0.001);
}

// =========================================================================
// Interpolator tests
// =========================================================================

#[test]
fn test_interpolator_new() {
    let interp = Interpolator::new(0.0, 100.0);
    assert_eq!(interp.start, 0.0);
    assert_eq!(interp.end, 100.0);
    assert_eq!(interp.easing, Easing::Linear);
}

#[test]
fn test_interpolator_easing_builder() {
    let interp = Interpolator::new(0.0, 100.0).easing(Easing::InOutQuad);
    assert_eq!(interp.easing, Easing::InOutQuad);
}

#[test]
fn test_interpolator_at_boundaries() {
    let interp = Interpolator::new(50.0, 150.0);
    assert_eq!(interp.at(0.0), 50.0);
    assert_eq!(interp.at(1.0), 150.0);
}

#[test]
fn test_interpolator_at_middle() {
    let interp = Interpolator::new(0.0, 100.0);
    assert!((interp.at(0.5) - 50.0).abs() < 0.001);
}

#[test]
fn test_interpolator_clone() {
    let interp = Interpolator::new(0.0, 100.0).easing(Easing::OutQuad);
    let cloned = interp.clone();
    assert_eq!(cloned.start, 0.0);
    assert_eq!(cloned.end, 100.0);
    assert_eq!(cloned.easing, Easing::OutQuad);
}

// =========================================================================
// Easing behavior tests
// =========================================================================

#[test]
fn test_ease_in_slower_at_start() {
    // All ease-in functions should be slower at the start
    let ease_ins = [
        Easing::InQuad,
        Easing::InCubic,
        Easing::InQuart,
        Easing::InQuint,
    ];
    for easing in ease_ins {
        let value = easing.ease(0.25);
        assert!(value < 0.25, "{:?} at 0.25 = {}", easing, value);
    }
}

#[test]
fn test_ease_out_faster_at_start() {
    // All ease-out functions should be faster at the start
    let ease_outs = [
        Easing::OutQuad,
        Easing::OutCubic,
        Easing::OutQuart,
        Easing::OutQuint,
    ];
    for easing in ease_outs {
        let value = easing.ease(0.25);
        assert!(value > 0.25, "{:?} at 0.25 = {}", easing, value);
    }
}

#[test]
fn test_ease_in_out_symmetric() {
    // In-out functions should pass through (0.5, 0.5) approximately
    let in_outs = [
        Easing::InOutQuad,
        Easing::InOutCubic,
        Easing::InOutQuart,
        Easing::InOutQuint,
        Easing::InOutSine,
    ];
    for easing in in_outs {
        let value = easing.ease(0.5);
        assert!(
            (value - 0.5).abs() < 0.01,
            "{:?} at 0.5 = {}",
            easing,
            value
        );
    }
}
