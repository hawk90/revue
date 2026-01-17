//! Color Property tests

#![allow(unused_imports)]

use proptest::prelude::*;
use revue::layout::Rect;
use revue::reactive::signal;
use revue::style::Color;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// RGB values are preserved in Color construction
    #[test]
    fn color_rgb_preserved(r: u8, g: u8, b: u8) {
        let color = Color::rgb(r, g, b);
        prop_assert_eq!(color.r, r);
        prop_assert_eq!(color.g, g);
        prop_assert_eq!(color.b, b);
    }

    /// Color equality is reflexive
    #[test]
    fn color_equality_reflexive(r: u8, g: u8, b: u8) {
        let color = Color::rgb(r, g, b);
        prop_assert_eq!(color, color);
    }

    /// Color from same values equals
    #[test]
    fn color_same_values_equal(r: u8, g: u8, b: u8) {
        let c1 = Color::rgb(r, g, b);
        let c2 = Color::rgb(r, g, b);
        prop_assert_eq!(c1, c2);
    }
}
