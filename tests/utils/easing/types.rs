//! Tests for easing types module

use revue::utils::easing::Easing;

// =========================================================================
// Easing type alias tests
// =========================================================================

#[test]
fn test_easing_fn_type() {
    use revue::utils::easing::EasingFn;
    // Just verify EasingFn can be defined and called
    let linear: EasingFn = |t| t;
    assert_eq!(linear(0.5), 0.5);
}

// =========================================================================
// Easing trait tests
// =========================================================================

#[test]
fn test_easing_default() {
    let easing = Easing::default();
    assert_eq!(easing, Easing::Linear);
}

#[test]
fn test_easing_clone() {
    let easing = Easing::InOutCubic;
    let cloned = easing;
    // Both valid due to Copy
    assert_eq!(cloned, Easing::InOutCubic);
    assert_eq!(easing, Easing::InOutCubic);
}

#[test]
fn test_easing_copy() {
    let easing1 = Easing::OutBounce;
    let easing2 = easing1;
    // Both valid
    assert_eq!(easing1, Easing::OutBounce);
    assert_eq!(easing2, Easing::OutBounce);
}

#[test]
fn test_easing_equality() {
    assert_eq!(Easing::Linear, Easing::Linear);
    assert_eq!(Easing::InQuad, Easing::InQuad);
    assert_ne!(Easing::Linear, Easing::InQuad);
}

#[test]
fn test_easing_partial_eq_all_variants() {
    // All variants should be comparable
    let easings = vec![
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

    // Each variant should equal itself
    for easing in &easings {
        assert_eq!(*easing, *easing);
    }

    // Check adjacent variants are not equal
    for i in 0..easings.len() - 1 {
        assert_ne!(easings[i], easings[i + 1]);
    }
}

#[test]
fn test_easing_debug() {
    let easing = Easing::InOutBounce;
    let debug = format!("{:?}", easing);
    assert!(debug.contains("InOutBounce"));
}

#[test]
fn test_easing_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Easing::Linear);
    set.insert(Easing::InQuad);
    set.insert(Easing::Linear); // Duplicate, should not increase count

    assert_eq!(set.len(), 2);
}

#[test]
fn test_easing_all_distinct() {
    let easings = vec![
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

    use std::collections::HashSet;
    let set: HashSet<_> = easings.iter().collect();
    assert_eq!(set.len(), easings.len());
}
