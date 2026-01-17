//! String/Text Property tests

#![allow(unused_imports)]

use proptest::prelude::*;
use revue::layout::Rect;
use revue::reactive::signal;
use revue::style::Color;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    /// Unicode char count matches expected
    #[test]
    fn unicode_string_char_count(s in "\\PC{0,100}") {
        let char_count = s.chars().count();
        // Char count should be <= byte length
        prop_assert!(char_count <= s.len());
    }

    /// String bytes are valid UTF-8
    #[test]
    fn string_is_valid_utf8(s in "\\PC{0,100}") {
        // If we got here, the string is already valid UTF-8
        prop_assert!(std::str::from_utf8(s.as_bytes()).is_ok());
    }
}
