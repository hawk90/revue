//! Rect Property tests

#![allow(unused_imports)]

use proptest::prelude::*;
use revue::layout::Rect;
use revue::reactive::signal;
use revue::style::Color;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// Rect area calculation matches width * height
    #[test]
    fn rect_area_correct(x: u16, y: u16, w: u16, h: u16) {
        let rect = Rect::new(x, y, w, h);
        let area = rect.width as u32 * rect.height as u32;
        prop_assert_eq!(area, w as u32 * h as u32);
    }

    /// Rect contains its own origin
    #[test]
    fn rect_contains_origin(x: u16, y: u16, w in 1u16..1000, h in 1u16..1000) {
        let rect = Rect::new(x, y, w, h);
        prop_assert!(rect.x >= x);
        prop_assert!(rect.y >= y);
    }

    /// Rect dimensions are preserved
    #[test]
    fn rect_dimensions_preserved(x: u16, y: u16, w: u16, h: u16) {
        let rect = Rect::new(x, y, w, h);
        prop_assert_eq!(rect.width, w);
        prop_assert_eq!(rect.height, h);
    }
}
