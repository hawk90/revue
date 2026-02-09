//! Core types for easing functions

/// Easing function type
///
/// All easing functions take a progress value (0.0 to 1.0) and return
/// the eased value (typically 0.0 to 1.0, but may exceed for elastic/back).
pub type EasingFn = fn(f64) -> f64;

/// Standard easing types
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Easing {
    /// No easing, constant speed
    #[default]
    Linear,

    // Quadratic
    /// Quadratic ease in (accelerate)
    InQuad,
    /// Quadratic ease out (decelerate)
    OutQuad,
    /// Quadratic ease in-out
    InOutQuad,

    // Cubic
    /// Cubic ease in
    InCubic,
    /// Cubic ease out
    OutCubic,
    /// Cubic ease in-out
    InOutCubic,

    // Quartic
    /// Quartic ease in
    InQuart,
    /// Quartic ease out
    OutQuart,
    /// Quartic ease in-out
    InOutQuart,

    // Quintic
    /// Quintic ease in
    InQuint,
    /// Quintic ease out
    OutQuint,
    /// Quintic ease in-out
    InOutQuint,

    // Sine
    /// Sine ease in
    InSine,
    /// Sine ease out
    OutSine,
    /// Sine ease in-out
    InOutSine,

    // Exponential
    /// Exponential ease in
    InExpo,
    /// Exponential ease out
    OutExpo,
    /// Exponential ease in-out
    InOutExpo,

    // Circular
    /// Circular ease in
    InCirc,
    /// Circular ease out
    OutCirc,
    /// Circular ease in-out
    InOutCirc,

    // Back (overshoots)
    /// Back ease in
    InBack,
    /// Back ease out
    OutBack,
    /// Back ease in-out
    InOutBack,

    // Elastic (spring-like)
    /// Elastic ease in
    InElastic,
    /// Elastic ease out
    OutElastic,
    /// Elastic ease in-out
    InOutElastic,

    // Bounce
    /// Bounce ease in
    InBounce,
    /// Bounce ease out
    OutBounce,
    /// Bounce ease in-out
    InOutBounce,
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Easing type alias tests
    // =========================================================================

    #[test]
    fn test_easing_fn_type() {
        // Just verify EasingFn can be defined and called
        let linear: EasingFn = |t| t;
        assert_eq!(linear(0.5), 0.5);
    }

    // =========================================================================
    // Easing enum variants tests
    // =========================================================================

    #[test]
    fn test_easing_linear() {
        let _ = Easing::Linear;
    }

    #[test]
    fn test_easing_quadratic_variants() {
        let _ = Easing::InQuad;
        let _ = Easing::OutQuad;
        let _ = Easing::InOutQuad;
    }

    #[test]
    fn test_easing_cubic_variants() {
        let _ = Easing::InCubic;
        let _ = Easing::OutCubic;
        let _ = Easing::InOutCubic;
    }

    #[test]
    fn test_easing_quartic_variants() {
        let _ = Easing::InQuart;
        let _ = Easing::OutQuart;
        let _ = Easing::InOutQuart;
    }

    #[test]
    fn test_easing_quintic_variants() {
        let _ = Easing::InQuint;
        let _ = Easing::OutQuint;
        let _ = Easing::InOutQuint;
    }

    #[test]
    fn test_easing_sine_variants() {
        let _ = Easing::InSine;
        let _ = Easing::OutSine;
        let _ = Easing::InOutSine;
    }

    #[test]
    fn test_easing_exponential_variants() {
        let _ = Easing::InExpo;
        let _ = Easing::OutExpo;
        let _ = Easing::InOutExpo;
    }

    #[test]
    fn test_easing_circular_variants() {
        let _ = Easing::InCirc;
        let _ = Easing::OutCirc;
        let _ = Easing::InOutCirc;
    }

    #[test]
    fn test_easing_back_variants() {
        let _ = Easing::InBack;
        let _ = Easing::OutBack;
        let _ = Easing::InOutBack;
    }

    #[test]
    fn test_easing_elastic_variants() {
        let _ = Easing::InElastic;
        let _ = Easing::OutElastic;
        let _ = Easing::InOutElastic;
    }

    #[test]
    fn test_easing_bounce_variants() {
        let _ = Easing::InBounce;
        let _ = Easing::OutBounce;
        let _ = Easing::InOutBounce;
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
}
