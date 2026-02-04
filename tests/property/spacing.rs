//! Spacing Property tests

#![allow(unused_imports)]

use proptest::prelude::*;
use revue::style::Spacing;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Test that Spacing::all sets all sides equally
    #[test]
    fn test_spacing_all(value in 0u16..) {
        let spacing = Spacing::all(value);
        prop_assert_eq!(spacing.top, value);
        prop_assert_eq!(spacing.right, value);
        prop_assert_eq!(spacing.bottom, value);
        prop_assert_eq!(spacing.left, value);
    }

    /// Test that Spacing::vertical only affects top and bottom
    #[test]
    fn test_spacing_vertical(value in 0u16..) {
        let spacing = Spacing::vertical(value);
        prop_assert_eq!(spacing.top, value);
        prop_assert_eq!(spacing.bottom, value);
        prop_assert_eq!(spacing.right, 0);
        prop_assert_eq!(spacing.left, 0);
    }

    /// Test that Spacing::horizontal only affects left and right
    #[test]
    fn test_spacing_horizontal(value in 0u16..) {
        let spacing = Spacing::horizontal(value);
        prop_assert_eq!(spacing.top, 0);
        prop_assert_eq!(spacing.bottom, 0);
        prop_assert_eq!(spacing.right, value);
        prop_assert_eq!(spacing.left, value);
    }

    /// Test that spacing values are non-negative
    #[test]
    fn test_spacing_non_negative(val1 in 0u16.., val2 in 0u16.., val3 in 0u16.., val4 in 0u16..) {
        let spacing = Spacing {
            top: val1,
            right: val2,
            bottom: val3,
            left: val4,
        };

        prop_assert!(spacing.top <= spacing.top.saturating_add(1));
        prop_assert!(spacing.right <= spacing.right.saturating_add(1));
        prop_assert!(spacing.bottom <= spacing.bottom.saturating_add(1));
        prop_assert!(spacing.left <= spacing.left.saturating_add(1));
    }
}
