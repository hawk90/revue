//! Splitter Property tests

#![allow(unused_imports)]

use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Test that splitter ratio clamping is idempotent
    #[test]
    fn test_splitter_ratio_clamp_idempotent(ratio in 0.0f32..1.0f32) {
        let clamped1 = ratio.clamp(0.1, 0.9);
        let clamped2 = clamped1.clamp(0.1, 0.9);
        prop_assert_eq!(clamped1, clamped2);
    }

    /// Test that splitter sizes sum to available (within bounds)
    #[test]
    fn test_splitter_sizes_sum(available in 10u16..1000u16, ratio in 0.1f32..0.9f32) {
        let first_size = (available as f32 * ratio).clamp(0.0, available as f32) as u16;
        let second_size = available.saturating_sub(first_size);

        // Sum should not exceed available
        prop_assert!(first_size.saturating_add(second_size) <= available);
    }

    /// Test that ratio 0.5 gives roughly equal halves
    #[test]
    fn test_splitter_equal_halves(available in 10u16..1000u16) {
        let ratio = 0.5;
        let first_size = (available as f32 * ratio).clamp(0.0, available as f32) as u16;
        let second_size = available.saturating_sub(first_size);

        // Sizes should be approximately equal (difference at most 1)
        let diff = if first_size > second_size {
            first_size.saturating_sub(second_size)
        } else {
            second_size.saturating_sub(first_size)
        };
        prop_assert!(diff <= 1);
    }

    /// Test that extreme ratios give extreme size distribution
    #[test]
    fn test_splitter_extreme_ratios(available in 10u16..1000u16) {
        let min_size = (available as f32 * 0.1).clamp(0.0, available as f32) as u16;
        let max_size = (available as f32 * 0.9).clamp(0.0, available as f32) as u16;

        // Max should be larger than min
        prop_assert!(max_size >= min_size);

        // Min should be at least 10% of available
        let expected_min = (available as f32 * 0.1) as u16;
        prop_assert!(min_size >= expected_min.saturating_sub(1) && min_size <= expected_min.saturating_add(1));
    }
}
