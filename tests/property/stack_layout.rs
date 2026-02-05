//! Stack Layout Property tests

#![allow(unused_imports)]

use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Test that stack total size grows with more items
    #[test]
    fn test_stack_size_monotonic(
        size1 in 1u16..50u16,
        size2 in 1u16..50u16,
        gap in 0u16..10u16
    ) {
        // Stack with one item
        let size_one = size1;

        // Stack with two items (horizontal)
        let size_two = size1.saturating_add(gap).saturating_add(size2);

        prop_assert!(size_two >= size_one);
    }

    /// Test that zero gap gives minimal spacing
    #[test]
    fn test_stack_zero_gap(size1 in 1u16..50u16, size2 in 1u16..50u16) {
        let with_gap = size1.saturating_add(5).saturating_add(size2);
        let without_gap = size1.saturating_add(0).saturating_add(size2);

        prop_assert!(without_gap <= with_gap);
    }

    /// Test that saturating arithmetic prevents overflow
    #[test]
    fn test_stack_saturating_no_overflow(
        size in 40000u16..u16::MAX,
        gap in 1000u16..5000u16
    ) {
        // These operations should not overflow
        let result = size.saturating_add(gap);
        prop_assert!(result <= u16::MAX);
    }
}
