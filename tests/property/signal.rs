//! Signal Property tests

#![allow(unused_imports)]

use proptest::prelude::*;
use revue::layout::Rect;
use revue::reactive::signal;
use revue::style::Color;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Signal get returns the set value
    #[test]
    fn signal_get_returns_set(initial: i32, new_value: i32) {
        let sig = signal(initial);
        prop_assert_eq!(sig.get(), initial);
        sig.set(new_value);
        prop_assert_eq!(sig.get(), new_value);
    }

    /// Signal update modifies value correctly
    #[test]
    fn signal_update_works(initial: i32, delta: i32) {
        let sig = signal(initial);
        sig.update(|v| *v = v.wrapping_add(delta));
        prop_assert_eq!(sig.get(), initial.wrapping_add(delta));
    }

    /// Signal clone shares state
    #[test]
    fn signal_clone_shares_state(initial: i32, new_value: i32) {
        let sig1 = signal(initial);
        let sig2 = sig1.clone();
        sig1.set(new_value);
        prop_assert_eq!(sig2.get(), new_value);
    }
}
