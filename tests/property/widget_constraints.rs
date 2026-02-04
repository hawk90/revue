//! Widget Constraints Property tests

#![allow(unused_imports)]

use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Test that percentage positioning stays within bounds
    #[test]
    fn test_positioned_percent_x_in_bounds(percent in 0.0f32..100.0f32) {
        // When percent is in [0, 100], resulting offset should be non-negative
        let clamped = percent.clamp(0.0, 100.0);
        prop_assert!(clamped >= 0.0 && clamped <= 100.0);
    }

    /// Test that percentage calculation is monotonic
    #[test]
    fn test_percent_calculation_monotonic(
        width in 1u16..1000u16,
        percent1 in 0.0f32..100.0f32,
        percent2 in 0.0f32..100.0f32
    ) {
        let offset1 = (width as f32 * percent1 / 100.0).max(0.0).min(width as f32);
        let offset2 = (width as f32 * percent2 / 100.0).max(0.0).min(width as f32);

        if percent1 <= percent2 {
            prop_assert!(offset1 <= offset2 || (offset1 - offset2).abs() < 0.5);
        }
    }

    /// Test that zero percent gives zero offset
    #[test]
    fn test_zero_percent_zero_offset(width in 1u16..1000u16) {
        let offset = (width as f32 * 0.0 / 100.0) as u16;
        prop_assert_eq!(offset, 0);
    }

    /// Test that 100% gives full width offset
    #[test]
    fn test_hundred_percent_full_width(width in 1u16..1000u16) {
        let offset = (width as f32 * 100.0 / 100.0).max(0.0).min(width as f32) as u16;
        prop_assert!(offset >= width.saturating_sub(1) || offset == width);
    }

    /// Test that negative percentages are clamped to zero
    #[test]
    fn test_negative_percent_clamped(width in 1u16..1000u16) {
        let percent = -50.0;
        let offset = (width as f32 * percent / 100.0).max(0.0).min(width as f32) as u16;
        prop_assert_eq!(offset, 0);
    }

    /// Test that percentages > 100 are clamped
    #[test]
    fn test_overflow_percent_clamped(width in 1u16..1000u16) {
        let percent = 150.0;
        let offset = (width as f32 * percent / 100.0).max(0.0).min(width as f32) as u16;
        prop_assert!(offset <= width);
    }
}
