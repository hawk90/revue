//! Saturating Arithmetic Property tests

#![allow(unused_imports)]

use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// Test that saturating add never exceeds u16::MAX
    #[test]
    fn test_saturating_add_bound(a in 0u16.., b in 0u16..) {
        let result = a.saturating_add(b);
        prop_assert!(result <= u16::MAX);
    }

    /// Test that saturating add with zero is idempotent
    #[test]
    fn test_saturating_add_zero(value in 0u16..) {
        prop_assert_eq!(value.saturating_add(0), value);
    }

    /// Test that saturating sub never underflows below zero
    #[test]
    fn test_saturating_sub_bound(a in 0u16.., b in 0u16..) {
        let result = a.saturating_sub(b);
        prop_assert!(result <= a);
    }

    /// Test that saturating sub of same value gives zero
    #[test]
    fn test_saturating_sub_self(value in 0u16..) {
        prop_assert_eq!(value.saturating_sub(value), 0);
    }

    /// Test that saturating operations are reversible within bounds
    #[test]
    fn test_saturating_reversible(value in 0u16..40000u16, add in 0u16..10000u16) {
        let added = value.saturating_add(add);
        let back = added.saturating_sub(add);
        prop_assert!(back >= value.saturating_sub(1) && back <= value.saturating_add(1));
    }
}
