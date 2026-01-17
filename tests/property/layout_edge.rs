//! Layout Edge Case Property tests

#![allow(unused_imports)]

use proptest::prelude::*;
use revue::layout::Rect;
use revue::reactive::signal;
use revue::style::Color;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// Rect intersection is commutative (with bounded values to avoid saturation edge cases)
    #[test]
    fn rect_intersection_commutative(
        x1 in 0u16..60000, y1 in 0u16..60000, w1 in 1u16..1000, h1 in 1u16..1000,
        x2 in 0u16..60000, y2 in 0u16..60000, w2 in 1u16..1000, h2 in 1u16..1000
    ) {
        let r1 = Rect::new(x1, y1, w1, h1);
        let r2 = Rect::new(x2, y2, w2, h2);

        let i1 = r1.intersection(&r2);
        let i2 = r2.intersection(&r1);

        prop_assert_eq!(i1, i2);
    }

    /// Rect union is commutative (with bounded values to avoid saturation edge cases)
    #[test]
    fn rect_union_commutative(
        x1 in 0u16..60000, y1 in 0u16..60000, w1 in 1u16..1000, h1 in 1u16..1000,
        x2 in 0u16..60000, y2 in 0u16..60000, w2 in 1u16..1000, h2 in 1u16..1000
    ) {
        let r1 = Rect::new(x1, y1, w1, h1);
        let r2 = Rect::new(x2, y2, w2, h2);

        let u1 = r1.union(&r2);
        let u2 = r2.union(&r1);

        prop_assert_eq!(u1, u2);
    }

    /// Rect contains itself (when non-zero dimensions)
    #[test]
    fn rect_contains_inner_points(
        x in 0u16..1000, y in 0u16..1000,
        w in 2u16..100, h in 2u16..100
    ) {
        let rect = Rect::new(x, y, w, h);

        // Top-left corner is inside
        prop_assert!(rect.contains(x, y));

        // Center point is inside (if dimensions allow)
        if w > 1 && h > 1 {
            prop_assert!(rect.contains(x + w / 2, y + h / 2));
        }
    }

    /// Rect does not contain points outside
    #[test]
    fn rect_excludes_outside_points(
        x in 10u16..1000, y in 10u16..1000,
        w in 1u16..100, h in 1u16..100
    ) {
        let rect = Rect::new(x, y, w, h);

        // Points just outside should not be contained
        prop_assert!(!rect.contains(x.saturating_sub(1), y));
        prop_assert!(!rect.contains(x, y.saturating_sub(1)));
    }

    /// Rect right edge is computed correctly
    #[test]
    fn rect_right_edge_correct(x: u16, y: u16, w: u16, h: u16) {
        let rect = Rect::new(x, y, w, h);

        // Avoid overflow
        if let Some(expected) = x.checked_add(w) {
            prop_assert_eq!(rect.right(), expected);
        }
    }

    /// Rect bottom edge is computed correctly
    #[test]
    fn rect_bottom_edge_correct(x: u16, y: u16, w: u16, h: u16) {
        let rect = Rect::new(x, y, w, h);

        // Avoid overflow
        if let Some(expected) = y.checked_add(h) {
            prop_assert_eq!(rect.bottom(), expected);
        }
    }

    /// Zero dimension rect never contains any point
    #[test]
    fn zero_dimension_rect_empty(x: u16, y: u16, test_x: u16, test_y: u16) {
        let zero_width = Rect::new(x, y, 0, 10);
        let zero_height = Rect::new(x, y, 10, 0);
        let zero_both = Rect::new(x, y, 0, 0);

        prop_assert!(!zero_width.contains(test_x, test_y));
        prop_assert!(!zero_height.contains(test_x, test_y));
        prop_assert!(!zero_both.contains(test_x, test_y));
    }

    /// Rect intersection with self equals self
    #[test]
    fn rect_self_intersection(x in 0u16..60000, y in 0u16..60000, w in 1u16..1000, h in 1u16..1000) {
        let rect = Rect::new(x, y, w, h);
        let intersection = rect.intersection(&rect);

        prop_assert_eq!(intersection, Some(rect));
    }

    /// Rect union with self equals self
    #[test]
    fn rect_self_union(x in 0u16..60000, y in 0u16..60000, w in 1u16..1000, h in 1u16..1000) {
        let rect = Rect::new(x, y, w, h);
        let union = rect.union(&rect);

        prop_assert_eq!(union, rect);
    }

    /// Intersection result is contained in both rects
    #[test]
    fn rect_intersection_subset(
        x1 in 0u16..500, y1 in 0u16..500, w1 in 1u16..100, h1 in 1u16..100,
        x2 in 0u16..500, y2 in 0u16..500, w2 in 1u16..100, h2 in 1u16..100
    ) {
        let r1 = Rect::new(x1, y1, w1, h1);
        let r2 = Rect::new(x2, y2, w2, h2);

        if let Some(intersection) = r1.intersection(&r2) {
            // Intersection should be within bounds of both rects
            prop_assert!(intersection.x >= r1.x);
            prop_assert!(intersection.y >= r1.y);
            prop_assert!(intersection.x >= r2.x);
            prop_assert!(intersection.y >= r2.y);

            // Intersection dimensions should not exceed either rect
            prop_assert!(intersection.width <= r1.width);
            prop_assert!(intersection.height <= r1.height);
            prop_assert!(intersection.width <= r2.width);
            prop_assert!(intersection.height <= r2.height);
        }
    }

    /// Union contains both original rects
    #[test]
    fn rect_union_superset(
        x1 in 0u16..500, y1 in 0u16..500, w1 in 1u16..100, h1 in 1u16..100,
        x2 in 0u16..500, y2 in 0u16..500, w2 in 1u16..100, h2 in 1u16..100
    ) {
        let r1 = Rect::new(x1, y1, w1, h1);
        let r2 = Rect::new(x2, y2, w2, h2);
        let union = r1.union(&r2);

        // Union should start at or before both rects
        prop_assert!(union.x <= r1.x);
        prop_assert!(union.y <= r1.y);
        prop_assert!(union.x <= r2.x);
        prop_assert!(union.y <= r2.y);

        // Union should extend to or past both rects' right/bottom edges
        if r1.right() <= u16::MAX && r2.right() <= u16::MAX {
            prop_assert!(union.right() >= r1.right());
            prop_assert!(union.right() >= r2.right());
        }
        if r1.bottom() <= u16::MAX && r2.bottom() <= u16::MAX {
            prop_assert!(union.bottom() >= r1.bottom());
            prop_assert!(union.bottom() >= r2.bottom());
        }
    }

    /// Intersects is symmetric
    #[test]
    fn rect_intersects_symmetric(
        x1 in 0u16..60000, y1 in 0u16..60000, w1 in 1u16..1000, h1 in 1u16..1000,
        x2 in 0u16..60000, y2 in 0u16..60000, w2 in 1u16..1000, h2 in 1u16..1000
    ) {
        let r1 = Rect::new(x1, y1, w1, h1);
        let r2 = Rect::new(x2, y2, w2, h2);

        prop_assert_eq!(r1.intersects(&r2), r2.intersects(&r1));
    }

    /// A rect always intersects itself
    #[test]
    fn rect_intersects_self(x in 0u16..60000, y in 0u16..60000, w in 1u16..1000, h in 1u16..1000) {
        let rect = Rect::new(x, y, w, h);
        prop_assert!(rect.intersects(&rect));
    }

    /// Test edge saturation behavior - rect.right() saturates at u16::MAX
    #[test]
    fn rect_right_saturates(x in 60000u16..=u16::MAX, w in 10000u16..=u16::MAX) {
        let rect = Rect::new(x, 0, w, 1);
        // Should not panic, should saturate
        let right = rect.right();
        prop_assert!(right >= x);
        prop_assert!(right <= u16::MAX);
    }

    /// Test edge saturation behavior - rect.bottom() saturates at u16::MAX
    #[test]
    fn rect_bottom_saturates(y in 60000u16..=u16::MAX, h in 10000u16..=u16::MAX) {
        let rect = Rect::new(0, y, 1, h);
        // Should not panic, should saturate
        let bottom = rect.bottom();
        prop_assert!(bottom >= y);
        prop_assert!(bottom <= u16::MAX);
    }
}
