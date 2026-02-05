//! Float to Integer Conversion Property tests

#![allow(unused_imports)]

use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// Test that small positive values are preserved
    #[test]
    fn test_clamped_small_positive(value in 0.0f32..100.0f32) {
        let result = value.clamp(0.0, 100.0) as u16;
        let expected = value as u16;
        prop_assert!(result >= expected.saturating_sub(1) && result <= expected.saturating_add(1));
    }

    /// Test that values above max clamp to max
    #[test]
    fn test_clamped_overflow_to_max(max in 1u16..1000u16) {
        let overflow = (max as f32) + 1000.0;
        let result = overflow.clamp(0.0, max as f32) as u16;
        prop_assert!(result >= max.saturating_sub(1) && result <= max);
    }

    /// Test that rounding is within 0.5 of original
    #[test]
    fn test_rounding_accuracy(value in 0.0f32..100.0f32) {
        let rounded = value.round() as u16;
        let diff = (value - rounded as f32).abs();
        prop_assert!(diff <= 0.5 || diff < 0.5001); // Account for floating point precision
    }
}

/// Test that zero input gives zero output
#[test]
fn test_clamped_zero_input() {
    let result = 0.0f32.clamp(0.0, 100.0) as u16;
    assert_eq!(result, 0);
}

/// Test that clamped float to u16 conversion is within bounds
#[test]
fn test_clamped_conversion() {
    for max in 1u16..1000u16 {
        for value in [0.0_f32, 100.0, 1000.0, 10000.0] {
            let clamped = value.clamp(0.0, max as f32);
            let converted = clamped as u16;
            assert!(converted <= max);
        }
    }
}
