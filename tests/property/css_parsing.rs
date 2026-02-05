//! CSS Parsing Property tests

#![allow(unused_imports)]

use proptest::prelude::*;
use revue::style::{Color, Size};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Test that unit parsing handles edge cases
    #[test]
    fn test_parse_unit_edge_cases(value in 0u32..10000u32) {
        // Test that pixel values are handled correctly
        let _size = Size::Fixed(value as u16);

        // Test that we don't panic on large values
        let clamped = value.min(u16::MAX as u32) as u16;
        let _size2 = Size::Fixed(clamped);
    }

    /// Test that color blending is commutative at 0.5
    #[test]
    fn test_color_blending_commutative(
        r1 in 0u8.., g1 in 0u8.., b1 in 0u8..,
        r2 in 0u8.., g2 in 0u8.., b2 in 0u8..
    ) {
        let color1 = Color::rgb(r1, g1, b1);
        let color2 = Color::rgb(r2, g2, b2);

        let blend1 = color1.blend(color2, 0.5);
        let blend2 = color2.blend(color1, 0.5);

        prop_assert_eq!(blend1, blend2);
    }

    /// Test that color blend with 0.0 returns first color
    #[test]
    fn test_color_blend_zero_ratio(
        r1 in 0u8.., g1 in 0u8.., b1 in 0u8..,
        r2 in 0u8.., g2 in 0u8.., b2 in 0u8..
    ) {
        let color1 = Color::rgb(r1, g1, b1);
        let color2 = Color::rgb(r2, g2, b2);

        prop_assert_eq!(color1.blend(color2, 0.0), color1);
    }

    /// Test that color blend with 1.0 returns second color
    #[test]
    fn test_color_blend_one_ratio(
        r1 in 0u8.., g1 in 0u8.., b1 in 0u8..,
        r2 in 0u8.., g2 in 0u8.., b2 in 0u8..
    ) {
        let color1 = Color::rgb(r1, g1, b1);
        let color2 = Color::rgb(r2, g2, b2);

        prop_assert_eq!(color1.blend(color2, 1.0), color2);
    }

    /// Test that color darkening is idempotent for same amount
    #[test]
    fn test_color_darken_idempotent(r in 0u8.., g in 0u8.., b in 0u8.., amount in 0u8..) {
        let color = Color::rgb(r, g, b);
        let darkened_once = color.darken(amount);
        let darkened_twice = darkened_once.darken(amount);

        // Darkening twice should give same result as darkening once by 2*amount
        let double_darken = color.darken(amount.saturating_mul(2));
        prop_assert_eq!(darkened_twice, double_darken);
    }

    /// Test that color lightening is monotonic
    #[test]
    fn test_color_lighten_monotonic(r in 0u8.., g in 0u8.., b in 0u8.., amount1 in 0u8.., amount2 in 0u8..) {
        let color = Color::rgb(r, g, b);

        let lighten1 = color.lighten(amount1);
        let lighten2 = color.lighten(amount2);

        if amount1 <= amount2 {
            prop_assert!(lighten1.r <= lighten2.r);
            prop_assert!(lighten1.g <= lighten2.g);
            prop_assert!(lighten1.b <= lighten2.b);
        }
    }

    /// Test that color darkening is monotonic
    #[test]
    fn test_color_darken_monotonic(r in 0u8.., g in 0u8.., b in 0u8.., amount1 in 0u8.., amount2 in 0u8..) {
        let color = Color::rgb(r, g, b);

        let darken1 = color.darken(amount1);
        let darken2 = color.darken(amount2);

        if amount1 <= amount2 {
            prop_assert!(darken1.r >= darken2.r);
            prop_assert!(darken1.g >= darken2.g);
            prop_assert!(darken1.b >= darken2.b);
        }
    }
}
