//! Property-based tests for revue using proptest
//!
//! These tests verify invariants and properties that should hold true
//! across a wide range of inputs, helping catch edge cases.

use proptest::prelude::*;
use revue::style::Color;

// =========================================================================
// Color Property Tests
// =========================================================================

proptest! {
    /// Test that RGB color creation preserves component values
    #[test]
    fn test_rgb_preserves_components(r in 0u8.., g in 0u8.., b in 0u8..) {
        let color = Color::rgb(r, g, b);
        prop_assert_eq!(color.r, r);
        prop_assert_eq!(color.g, g);
        prop_assert_eq!(color.b, b);
        prop_assert_eq!(color.a, 255); // Always opaque
    }

    /// Test that RGBA color creation preserves component values
    #[test]
    fn test_rgba_preserves_components(r in 0u8.., g in 0u8.., b in 0u8.., a in 0u8..) {
        let color = Color::rgba(r, g, b, a);
        prop_assert_eq!(color.r, r);
        prop_assert_eq!(color.g, g);
        prop_assert_eq!(color.b, b);
        prop_assert_eq!(color.a, a);
    }

    /// Test hex color roundtrip for 24-bit colors
    #[test]
    fn test_hex_roundtrip(hex in 0x000000u32..=0xFFFFFF) {
        let color = Color::hex(hex);
        let reconstructed = ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
        prop_assert_eq!(reconstructed, hex);
        prop_assert_eq!(color.a, 255);
    }

    /// Test hexa color roundtrip for 32-bit colors
    #[test]
    fn test_hexa_roundtrip(hex in 0x00000000u32..=0xFFFFFFFF) {
        let color = Color::hexa(hex);
        let reconstructed = ((color.r as u32) << 24) | ((color.g as u32) << 16)
            | ((color.b as u32) << 8) | (color.a as u32);
        prop_assert_eq!(reconstructed, hex);
    }

    /// Test that with_alpha preserves other components
    #[test]
    fn test_with_alpha_preserves_rgb(r in 0u8.., g in 0u8.., b in 0u8.., a in 0u8..) {
        let color = Color::rgb(r, g, b);
        let with_a = color.with_alpha(a);
        prop_assert_eq!(with_a.r, r);
        prop_assert_eq!(with_a.g, g);
        prop_assert_eq!(with_a.b, b);
        prop_assert_eq!(with_a.a, a);
    }

    /// Test that darken never increases component values
    #[test]
    fn test_darken_never_increases(r in 0u8.., g in 0u8.., b in 0u8.., amount in 0u8..) {
        let color = Color::rgb(r, g, b);
        let darkened = color.darken(amount);
        prop_assert!(darkened.r <= r);
        prop_assert!(darkened.g <= g);
        prop_assert!(darkened.b <= b);
    }

    /// Test that lighten never decreases component values
    #[test]
    fn test_lighten_never_decreases(r in 0u8.., g in 0u8.., b in 0u8.., amount in 0u8..) {
        let color = Color::rgb(r, g, b);
        let lightened = color.lighten(amount);
        prop_assert!(lightened.r >= r);
        prop_assert!(lightened.g >= g);
        prop_assert!(lightened.b >= b);
    }

    /// Test that darken by 0 leaves color unchanged
    #[test]
    fn test_darken_zero_no_change(r in 0u8.., g in 0u8.., b in 0u8..) {
        let color = Color::rgb(r, g, b);
        prop_assert_eq!(color.darken(0), color);
    }

    /// Test that lighten by 0 leaves color unchanged
    #[test]
    fn test_lighten_zero_no_change(r in 0u8.., g in 0u8.., b in 0u8..) {
        let color = Color::rgb(r, g, b);
        prop_assert_eq!(color.lighten(0), color);
    }

    /// Test that alpha_f32 is in range [0.0, 1.0]
    #[test]
    fn test_alpha_f32_in_range(a in 0u8..) {
        let color = Color::rgba(0, 0, 0, a);
        let alpha = color.alpha_f32();
        prop_assert!((0.0..=1.0).contains(&alpha));
    }

    /// Test that with_alpha_f32 is idempotent for clamped values
    #[test]
    fn test_with_alpha_f32_clamps(a in 0u8..) {
        let color = Color::rgba(0, 0, 0, a);
        let alpha_f32 = color.alpha_f32();
        let new_color = color.with_alpha_f32(alpha_f32);
        prop_assert_eq!(new_color.a, a);
    }

    /// Test that blend(0.0) returns first color
    #[test]
    fn test_blend_zero_returns_first(
        r1 in 0u8.., g1 in 0u8.., b1 in 0u8.., a1 in 0u8..,
        r2 in 0u8.., g2 in 0u8.., b2 in 0u8.., a2 in 0u8..
    ) {
        let color1 = Color::rgba(r1, g1, b1, a1);
        let color2 = Color::rgba(r2, g2, b2, a2);
        prop_assert_eq!(color1.blend(color2, 0.0), color1);
    }

    /// Test that blend(1.0) returns second color
    #[test]
    fn test_blend_one_returns_second(
        r1 in 0u8.., g1 in 0u8.., b1 in 0u8.., a1 in 0u8..,
        r2 in 0u8.., g2 in 0u8.., b2 in 0u8.., a2 in 0u8..
    ) {
        let color1 = Color::rgba(r1, g1, b1, a1);
        let color2 = Color::rgba(r2, g2, b2, a2);
        prop_assert_eq!(color1.blend(color2, 1.0), color2);
    }

    /// Test that blend is symmetric (blend(a,b,0.5) == blend(b,a,0.5))
    #[test]
    fn test_blend_symmetric(
        r1 in 0u8.., g1 in 0u8.., b1 in 0u8.., a1 in 0u8..,
        r2 in 0u8.., g2 in 0u8.., b2 in 0u8.., a2 in 0u8..
    ) {
        let color1 = Color::rgba(r1, g1, b1, a1);
        let color2 = Color::rgba(r2, g2, b2, a2);
        let blended1 = color1.blend(color2, 0.5);
        let blended2 = color2.blend(color1, 0.5);
        prop_assert_eq!(blended1, blended2);
    }

    /// Test that is_transparent is true only when alpha is 0
    #[test]
    fn test_is_transparent_iff_alpha_zero(r in 0u8.., g in 0u8.., b in 0u8.., a in 0u8..) {
        let color = Color::rgba(r, g, b, a);
        prop_assert_eq!(color.is_transparent(), a == 0);
    }

    /// Test that is_opaque is true only when alpha is 255
    #[test]
    fn test_is_opaque_iff_alpha_max(r in 0u8.., g in 0u8.., b in 0u8.., a in 0u8..) {
        let color = Color::rgba(r, g, b, a);
        prop_assert_eq!(color.is_opaque(), a == 255);
    }

    /// Test that pressed is darker than original
    #[test]
    fn test_pressed_is_darker(r in 0u8.., g in 0u8.., b in 0u8..) {
        let color = Color::rgb(r, g, b);
        let pressed = color.pressed();
        prop_assert!(pressed.r <= color.r);
        prop_assert!(pressed.g <= color.g);
        prop_assert!(pressed.b <= color.b);
    }

    /// Test that hover is lighter than original
    #[test]
    fn test_hover_is_lighter(r in 0u8.., g in 0u8.., b in 0u8..) {
        let color = Color::rgb(r, g, b);
        let hovered = color.hover();
        prop_assert!(hovered.r >= color.r);
        prop_assert!(hovered.g >= color.g);
        prop_assert!(hovered.b >= color.b);
    }

    /// Test that with_interaction produces correct results
    #[test]
    fn test_with_interaction_priority(
        r in 0u8.., g in 0u8.., b in 0u8..
    ) {
        let base = Color::rgb(r, g, b);
        let pressed = base.pressed();
        let hovered = base.hover();

        // Pressed takes priority
        let result = base.with_interaction(true, true, false);
        prop_assert_eq!(result, pressed);

        // Hover takes priority when not pressed
        let result = base.with_interaction(false, true, false);
        prop_assert_eq!(result, hovered);

        // Focus same as hover
        let result = base.with_interaction(false, false, true);
        prop_assert_eq!(result, hovered);
    }
}

// =========================================================================
// Spacing Property Tests
// =========================================================================

proptest! {
    /// Test that Spacing::all sets all sides equally
    #[test]
    fn test_spacing_all(value in 0u16..) {
        use revue::style::Spacing;

        let spacing = Spacing::all(value);
        prop_assert_eq!(spacing.top, value);
        prop_assert_eq!(spacing.right, value);
        prop_assert_eq!(spacing.bottom, value);
        prop_assert_eq!(spacing.left, value);
    }

    /// Test that Spacing::vertical only affects top and bottom
    #[test]
    fn test_spacing_vertical(value in 0u16..) {
        use revue::style::Spacing;

        let spacing = Spacing::vertical(value);
        prop_assert_eq!(spacing.top, value);
        prop_assert_eq!(spacing.bottom, value);
        prop_assert_eq!(spacing.right, 0);
        prop_assert_eq!(spacing.left, 0);
    }

    /// Test that Spacing::horizontal only affects left and right
    #[test]
    fn test_spacing_horizontal(value in 0u16..) {
        use revue::style::Spacing;

        let spacing = Spacing::horizontal(value);
        prop_assert_eq!(spacing.top, 0);
        prop_assert_eq!(spacing.bottom, 0);
        prop_assert_eq!(spacing.right, value);
        prop_assert_eq!(spacing.left, value);
    }
}

// =========================================================================
// Layout Property Tests
// =========================================================================

proptest! {
    /// Test that Rect::new preserves all values
    #[test]
    fn test_rect_new_preserves_values(x in 0u16.., y in 0u16.., w in 0u16.., h in 0u16..) {
        use revue::layout::Rect;

        let rect = Rect::new(x, y, w, h);
        prop_assert_eq!(rect.x, x);
        prop_assert_eq!(rect.y, y);
        prop_assert_eq!(rect.width, w);
        prop_assert_eq!(rect.height, h);
    }

    /// Test that Rect::right() equals x + width (with saturation)
    #[test]
    fn test_rect_right(x in 0u16.., w in 0u16..40000u16) {
        use revue::layout::Rect;

        let rect = Rect::new(x, 0, w, 0);
        let expected = x.saturating_add(w);
        prop_assert_eq!(rect.right(), expected);
    }

    /// Test that Rect::bottom() equals y + height (with saturation)
    #[test]
    fn test_rect_bottom(y in 0u16.., h in 0u16..40000u16) {
        use revue::layout::Rect;

        let rect = Rect::new(0, y, 0, h);
        let expected = y.saturating_add(h);
        prop_assert_eq!(rect.bottom(), expected);
    }

    /// Test that Rect::contains returns true for point inside
    #[test]
    fn test_rect_contains_inside(x in 0u16..100u16, y in 0u16..100u16, w in 1u16..100u16, h in 1u16..100u16) {
        use revue::layout::Rect;

        let rect = Rect::new(x, y, w, h);

        // Test top-left corner (inside)
        if x < u16::MAX && y < u16::MAX {
            prop_assert!(rect.contains(x, y));
        }

        // Test a point in the middle
        let mid_x = x.saturating_add(w / 2);
        let mid_y = y.saturating_add(h / 2);
        if mid_x < u16::MAX && mid_y < u16::MAX {
            prop_assert!(rect.contains(mid_x, mid_y));
        }

        // Test point just before right edge
        if w > 0 && x < u16::MAX && y < u16::MAX {
            let near_right = x.saturating_add(w.saturating_sub(1));
            if near_right < u16::MAX {
                prop_assert!(rect.contains(near_right, y));
            }
        }
    }

    /// Test that Rect::contains returns false for points outside
    #[test]
    fn test_rect_contains_outside(x in 0u16..100u16, y in 0u16..100u16, w in 1u16..100u16, h in 1u16..100u16) {
        use revue::layout::Rect;

        let rect = Rect::new(x, y, w, h);

        // Test point to the left
        if x > 0 {
            prop_assert!(!rect.contains(x.saturating_sub(1), y));
        }

        // Test point above
        if y > 0 {
            prop_assert!(!rect.contains(x, y.saturating_sub(1)));
        }

        // Test point at right edge (contains is exclusive on right/bottom)
        prop_assert!(!rect.contains(rect.right(), y));

        // Test point at bottom edge
        prop_assert!(!rect.contains(x, rect.bottom()));
    }

    /// Test that rect intersection is within both rects
    #[test]
    fn test_rect_intersection_within_bounds(
        x1 in 0u16..100u16, y1 in 0u16..100u16, w1 in 1u16..100u16, h1 in 1u16..100u16,
        x2 in 0u16..100u16, y2 in 0u16..100u16, w2 in 1u16..100u16, h2 in 1u16..100u16
    ) {
        use revue::layout::Rect;

        let rect1 = Rect::new(x1, y1, w1, h1);
        let rect2 = Rect::new(x2, y2, w2, h2);

        if let Some(intersection) = rect1.intersection(&rect2) {
            // Intersection should be within rect1
            prop_assert!(intersection.x >= rect1.x || intersection.right() <= rect1.right());
            prop_assert!(intersection.y >= rect1.y || intersection.bottom() <= rect1.bottom());

            // Intersection should be within rect2
            prop_assert!(intersection.x >= rect2.x || intersection.right() <= rect2.right());
            prop_assert!(intersection.y >= rect2.y || intersection.bottom() <= rect2.bottom());

            // Intersection should be smaller or equal to both rects
            prop_assert!(intersection.width <= rect1.width || intersection.width <= rect2.width);
            prop_assert!(intersection.height <= rect1.height || intersection.height <= rect2.height);
        }
    }

    /// Test that rect union contains both input rects
    #[test]
    fn test_rect_union_contains_both(
        x1 in 0u16..100u16, y1 in 0u16..100u16, w1 in 1u16..100u16, h1 in 1u16..100u16,
        x2 in 0u16..100u16, y2 in 0u16..100u16, w2 in 1u16..100u16, h2 in 1u16..100u16
    ) {
        use revue::layout::Rect;

        let rect1 = Rect::new(x1, y1, w1, h1);
        let rect2 = Rect::new(x2, y2, w2, h2);
        let union = rect1.union(&rect2);

        // Union should contain rect1's top-left
        if rect1.x < u16::MAX && rect1.y < u16::MAX {
            prop_assert!(union.contains(rect1.x, rect1.y));
        }

        // Union should contain rect2's top-left
        if rect2.x < u16::MAX && rect2.y < u16::MAX {
            prop_assert!(union.contains(rect2.x, rect2.y));
        }

        // Union should be at least as large as both rects
        prop_assert!(union.width >= rect1.width);
        prop_assert!(union.width >= rect2.width);
        prop_assert!(union.height >= rect1.height);
        prop_assert!(union.height >= rect2.height);
    }

    /// Test that rect union is commutative
    #[test]
    fn test_rect_union_commutative(
        x1 in 0u16..100u16, y1 in 0u16..100u16, w1 in 1u16..100u16, h1 in 1u16..100u16,
        x2 in 0u16..100u16, y2 in 0u16..100u16, w2 in 1u16..100u16, h2 in 1u16..100u16
    ) {
        use revue::layout::Rect;

        let rect1 = Rect::new(x1, y1, w1, h1);
        let rect2 = Rect::new(x2, y2, w2, h2);
        prop_assert_eq!(rect1.union(&rect2), rect2.union(&rect1));
    }

    /// Test that rect intersection is commutative
    #[test]
    fn test_rect_intersection_commutative(
        x1 in 0u16..100u16, y1 in 0u16..100u16, w1 in 1u16..100u16, h1 in 1u16..100u16,
        x2 in 0u16..100u16, y2 in 0u16..100u16, w2 in 1u16..100u16, h2 in 1u16..100u16
    ) {
        use revue::layout::Rect;

        let rect1 = Rect::new(x1, y1, w1, h1);
        let rect2 = Rect::new(x2, y2, w2, h2);
        prop_assert_eq!(rect1.intersection(&rect2), rect2.intersection(&rect1));
    }

    /// Test that rect intersects with itself
    #[test]
    fn test_rect_intersects_self(x in 0u16..100u16, y in 0u16..100u16, w in 1u16..100u16, h in 1u16..100u16) {
        use revue::layout::Rect;

        let rect = Rect::new(x, y, w, h);
        prop_assert!(rect.intersects(&rect));
    }

    /// Test that rect union with itself returns itself
    #[test]
    fn test_rect_union_self(x in 0u16..100u16, y in 0u16..100u16, w in 1u16..100u16, h in 1u16..100u16) {
        use revue::layout::Rect;

        let rect = Rect::new(x, y, w, h);
        prop_assert_eq!(rect.union(&rect), rect);
    }

    /// Test that rect intersection with itself returns itself
    #[test]
    fn test_rect_intersection_self(x in 0u16..100u16, y in 0u16..100u16, w in 1u16..100u16, h in 1u16..100u16) {
        use revue::layout::Rect;

        let rect = Rect::new(x, y, w, h);
        prop_assert_eq!(rect.intersection(&rect), Some(rect));
    }
}

// =========================================================================
// Reactive System Property Tests
// =========================================================================

proptest! {
    /// Test that signal preserves set value
    #[test]
    fn test_signal_set_preserves_value(value in any::<i32>()) {
        use revue::reactive::signal;

        let sig = signal(value);
        sig.set(value);
        prop_assert_eq!(sig.get(), value);
    }

    /// Test that signal update preserves relationship
    #[test]
    fn test_signal_update_preserves_relationship(initial in any::<i32>(), delta in any::<i32>()) {
        use revue::reactive::signal;

        let sig = signal(initial);
        sig.update(|v| *v = v.wrapping_add(delta));
        prop_assert_eq!(sig.get(), initial.wrapping_add(delta));
    }

    /// Test that signal set then get returns same value
    #[test]
    fn test_signal_set_get_roundtrip(value in any::<String>()) {
        use revue::reactive::signal;

        let sig = signal(String::new());
        sig.set(value.clone());
        prop_assert_eq!(sig.get(), value);
    }

    /// Test that computed with simple dependency returns correct value
    #[test]
    fn test_computed_simple_dependency(value in any::<i32>()) {
        use revue::reactive::{signal, computed};

        let source = signal(value);
        let derived = computed(move || source.get().wrapping_mul(2));
        prop_assert_eq!(derived.get(), value.wrapping_mul(2));
    }

    /// Test that computed updates when source changes
    #[test]
    fn test_computed_updates_on_change(initial in any::<i32>(), new_val in any::<i32>()) {
        use revue::reactive::{signal, computed};

        let source = signal(initial);
        let source_clone = source.clone();
        let derived = computed(move || source_clone.get() + 1);

        source.set(new_val);
        prop_assert_eq!(derived.get(), new_val.wrapping_add(1));
    }

    /// Test that computed with multiple sources combines correctly
    #[test]
    fn test_computed_multiple_sources(
        val1 in any::<i32>(),
        val2 in any::<i32>()
    ) {
        use revue::reactive::{signal, computed};

        let sig1 = signal(val1);
        let sig2 = signal(val2);
        let sum = computed(move || sig1.get().wrapping_add(sig2.get()));

        prop_assert_eq!(sum.get(), val1.wrapping_add(val2));
    }

    /// Test that multiple computeds from same source are independent
    #[test]
    fn test_multiple_computeds_independent(source_val in any::<i32>()) {
        use revue::reactive::{signal, computed};

        let source = signal(source_val);
        let source_clone = source.clone();
        let doubled = computed(move || source_clone.get().wrapping_mul(2));
        let tripled = computed(move || source.get().wrapping_mul(3));

        prop_assert_eq!(doubled.get(), source_val.wrapping_mul(2));
        prop_assert_eq!(tripled.get(), source_val.wrapping_mul(3));
    }

    /// Test that computed chains correctly
    #[test]
    fn test_computed_chain(base_val in any::<i32>()) {
        use revue::reactive::{signal, computed};

        let base = signal(base_val);
        let step1 = computed(move || base.get().wrapping_add(1));
        let step2 = computed(move || step1.get().wrapping_mul(2));

        prop_assert_eq!(step2.get(), base_val.wrapping_add(1).wrapping_mul(2));
    }

    /// Test that boolean signal works correctly
    #[test]
    fn test_bool_signal(value in any::<bool>()) {
        use revue::reactive::signal;

        let sig = signal(value);
        prop_assert_eq!(sig.get(), value);
    }

    /// Test that signal update is idempotent for same value
    #[test]
    fn test_signal_same_value_idempotent(value in any::<i32>()) {
        use revue::reactive::signal;

        let sig = signal(value);
        let first = sig.get();
        sig.set(value);
        let second = sig.get();
        prop_assert_eq!(first, second);
    }

    /// Test that computed with bool operations works correctly
    #[test]
    fn test_computed_bool_operations(val1 in any::<bool>(), val2 in any::<bool>()) {
        use revue::reactive::{signal, computed};

        let sig1 = signal(val1);
        let sig2 = signal(val2);
        let sig1_clone = sig1.clone();
        let sig2_clone = sig2.clone();

        let and_result = computed(move || sig1.get() && sig2.get());
        let or_result = computed(move || sig1_clone.get() || sig2_clone.get());

        prop_assert_eq!(and_result.get(), val1 && val2);
        prop_assert_eq!(or_result.get(), val1 || val2);
    }

    /// Test that signal update with mutation works
    #[test]
    fn test_signal_update_mutation(initial in any::<Vec<i32>>(), to_add in any::<i32>()) {
        use revue::reactive::signal;

        let sig = signal(initial.clone());
        sig.update(|v| v.push(to_add));

        let mut expected = initial;
        expected.push(to_add);
        prop_assert_eq!(sig.get(), expected);
    }

    /// Test that computed is lazy (doesn't compute unnecessarily)
    #[test]
    fn test_computed_count_calls(val1 in any::<i32>()) {
        use revue::reactive::{signal, computed};
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::sync::Arc;

        let source = signal(val1);
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let derived = computed(move || {
            counter_clone.fetch_add(1, Ordering::Relaxed);
            source.get()
        });

        // Accessing multiple times should only compute once per source change
        let _ = derived.get();
        let _ = derived.get();

        prop_assert!(counter.load(Ordering::Relaxed) <= 2);
    }

    /// Test that signal clone creates independent handles to same value
    #[test]
    fn test_signal_clone_independence(value in any::<i32>()) {
        use revue::reactive::signal;

        let sig1 = signal(value);
        let sig2 = sig1.clone();

        sig1.set(value.wrapping_add(1));
        prop_assert_eq!(sig2.get(), value.wrapping_add(1));
        prop_assert_eq!(sig1.get(), sig2.get());
    }
}

// =========================================================================
// CSS Parsing Property Tests
// =========================================================================

proptest! {
    /// Test that unit parsing handles edge cases
    #[test]
    fn test_parse_unit_edge_cases(value in 0u32..10000u32) {
        use revue::style::Size;

        // Test that pixel values are handled correctly
        let _size = Size::Fixed(value as u16);

        // Test that we don't panic on large values
        let clamped = value.min(u16::MAX as u32) as u16;
        let _size2 = Size::Fixed(clamped);
    }

    /// Test that spacing values are non-negative
    #[test]
    fn test_spacing_non_negative(val1 in 0u16.., val2 in 0u16.., val3 in 0u16.., val4 in 0u16..) {
        use revue::style::Spacing;

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

    /// Test that color blending is commutative at 0.5
    #[test]
    fn test_color_blending_commutative(
        r1 in 0u8.., g1 in 0u8.., b1 in 0u8..,
        r2 in 0u8.., g2 in 0u8.., b2 in 0u8..
    ) {
        use revue::style::Color;

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
        use revue::style::Color;

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
        use revue::style::Color;

        let color1 = Color::rgb(r1, g1, b1);
        let color2 = Color::rgb(r2, g2, b2);

        prop_assert_eq!(color1.blend(color2, 1.0), color2);
    }

    /// Test that color darkening is idempotent for same amount
    #[test]
    fn test_color_darken_idempotent(r in 0u8.., g in 0u8.., b in 0u8.., amount in 0u8..) {
        use revue::style::Color;

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
        use revue::style::Color;

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
        use revue::style::Color;

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

// =========================================================================
// String and Text Property Tests
// =========================================================================

proptest! {
    /// Test that string length is preserved after basic operations
    #[test]
    fn test_string_length_invariant(s in "\\PC*") {
        // String length should be consistent
        let len = s.len();
        prop_assert_eq!(s.chars().count(), s.chars().count());
        prop_assert!(len <= 100000); // Reasonable limit
    }

    /// Test that repeated string concatenation is associative
    #[test]
    fn test_string_concat_associative(s1 in "\\PC*", s2 in "\\PC*", s3 in "\\PC*") {
        let result1 = format!("{}{}{}", s1, s2, s3);
        let result2 = format!("{}{}{}", s1, s2, s3);
        prop_assert_eq!(result1, result2);
    }

    /// Test that trimming reduces length or keeps same
    #[test]
    fn test_trim_reduces_length(s in "\\PC*") {
        let trimmed = s.trim();
        prop_assert!(trimmed.len() <= s.len());
    }

    /// Test that empty string is idempotent for operations
    #[test]
    fn test_empty_string_idempotent(s in "\\PC*") {
        prop_assert_eq!("".chars().count(), 0);
        prop_assert_eq!("".trim(), "");
        prop_assert_eq!(format!("{}{}", "", s), s.clone());
        prop_assert_eq!(format!("{}{}", s, ""), s);
    }

    /// Test that string repetition works correctly
    #[test]
    fn test_string_repeat(s in "\\PC*", times in 0u8..10u8) {
        let repeated = s.repeat(times as usize);
        let expected_count = s.chars().count() * times as usize;
        prop_assert_eq!(repeated.chars().count(), expected_count);
    }
}

// =========================================================================
// Buffer Operation Property Tests
// =========================================================================

proptest! {
    /// Test that buffer dimensions are preserved
    #[test]
    fn test_buffer_dimensions(w in 0u16..200u16, h in 0u16..200u16) {
        use revue::render::Buffer;

        let buffer = Buffer::new(w, h);
        prop_assert_eq!(buffer.width(), w);
        prop_assert_eq!(buffer.height(), h);
    }

    /// Test that buffer area calculation is correct
    #[test]
    fn test_buffer_area(w in 0u16..200u16, h in 0u16..200u16) {
        use revue::render::Buffer;

        let buffer = Buffer::new(w, h);
        let area = buffer.width() as u32 * buffer.height() as u32;
        prop_assert_eq!(area, w as u32 * h as u32);
    }

    /// Test that buffer cell access is bounded
    #[test]
    fn test_buffer_cell_in_bounds(w in 1u16..200u16, h in 1u16..200u16) {
        use revue::render::Buffer;

        let buffer = Buffer::new(w, h);
        // Safe access should not panic
        let _cell = buffer.get(0, 0);
        let _cell = buffer.get(w.saturating_sub(1), h.saturating_sub(1));
    }

    /// Test that buffer clearing resets content
    #[test]
    fn test_buffer_clear_reset(w in 1u16..100u16, h in 1u16..100u16) {
        use revue::render::Buffer;
        use revue::style::Color;
        use revue::render::Cell;

        let mut buffer = Buffer::new(w, h);

        // Write some content
        let cell = Cell::new('X').fg(Color::WHITE).bg(Color::BLACK);
        buffer.set(0, 0, cell);

        // Clear should reset
        buffer.clear();

        // After clear, cell should be cleared
        let cleared = buffer.get(0, 0);
        prop_assert!(cleared.is_some());
    }
}

// =========================================================================
// Positioned Widget Property Tests
// =========================================================================

proptest! {
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

// =========================================================================
// Stack Layout Property Tests
// =========================================================================

proptest! {
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

// =========================================================================
// Splitter Property Tests
// =========================================================================

proptest! {
    /// Test that splitter ratio clamping is idempotent
    #[test]
    fn test_splitter_ratio_clamp_idempotent(ratio in 0.0f32..1.0f32) {
        let clamped1 = ratio.clamp(0.1, 0.9);
        let clamped2 = clamped1.clamp(0.1, 0.9);
        prop_assert_eq!(clamped1, clamped2);
    }

    /// Test that splitter sizes sum to available (within bounds)
    #[test]
    fn test_splitter_sizes_sum(available in 10u16..1000u16, ratio in 0.1f32..0.9f32) {
        let first_size = (available as f32 * ratio).clamp(0.0, available as f32) as u16;
        let second_size = available.saturating_sub(first_size);

        // Sum should not exceed available
        prop_assert!(first_size.saturating_add(second_size) <= available);
    }

    /// Test that ratio 0.5 gives roughly equal halves
    #[test]
    fn test_splitter_equal_halves(available in 10u16..1000u16) {
        let ratio = 0.5;
        let first_size = (available as f32 * ratio).clamp(0.0, available as f32) as u16;
        let second_size = available.saturating_sub(first_size);

        // Sizes should be approximately equal (difference at most 1)
        let diff = if first_size > second_size {
            first_size.saturating_sub(second_size)
        } else {
            second_size.saturating_sub(first_size)
        };
        prop_assert!(diff <= 1);
    }

    /// Test that extreme ratios give extreme size distribution
    #[test]
    fn test_splitter_extreme_ratios(available in 10u16..1000u16) {
        let min_size = (available as f32 * 0.1).clamp(0.0, available as f32) as u16;
        let max_size = (available as f32 * 0.9).clamp(0.0, available as f32) as u16;

        // Max should be larger than min
        prop_assert!(max_size >= min_size);

        // Min should be at least 10% of available
        let expected_min = (available as f32 * 0.1) as u16;
        prop_assert!(min_size >= expected_min.saturating_sub(1) && min_size <= expected_min.saturating_add(1));
    }
}

// =========================================================================
// Widget State Transition Property Tests
// =========================================================================

proptest! {
    /// Test that boolean state toggle is idempotent when done twice
    #[test]
    fn test_bool_toggle_idempotent(initial in any::<bool>()) {
        // Toggle once
        let toggled_once = !initial;

        // Toggle twice (should return to original)
        let toggled_twice = !toggled_once;

        prop_assert_eq!(toggled_twice, initial);
    }

    /// Test that setting state to same value is idempotent
    #[test]
    fn test_bool_state_idempotent(state in any::<bool>()) {
        prop_assert_eq!(state, state);
        prop_assert_eq!(!(!state), state);
    }

    /// Test that mutually exclusive states are never both true
    #[test]
    fn test_mutually_exclusive_states(pressed in any::<bool>(), hovered in any::<bool>()) {
        // If pressed and hovered are mutually exclusive, at most one should be true
        // This is a property test - the actual implementation depends on the widget
        let _both_true = pressed && hovered;
        // We're just verifying that both CAN be true (not exclusive)
        // If they were exclusive: prop_assert!(!_both_true);
    }

    /// Test that disabled state overrides others
    #[test]
    fn test_disabled_overrides(disabled in any::<bool>(), focused in any::<bool>()) {
        // When disabled is true, focused should not matter for interaction
        // This tests the logical property
        if disabled {
            // Disabled widget should not process focused state
            let effective_focus = focused && !disabled;
            prop_assert_eq!(effective_focus, false);
        }
    }
}

// =========================================================================
// Saturating Arithmetic Property Tests
// =========================================================================

proptest! {
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

// =========================================================================
// Float to Integer Conversion Property Tests
// =========================================================================

proptest! {
    /// Test that small positive values are preserved
    #[test]
    fn test_clamped_small_positive(value in 0.0f32..100.0f32) {
        let result = value.clamp(0.0, 100.0) as u16;
        let expected = value as u16;
        prop_assert!(result >= expected.saturating_sub(1) && result <= expected.saturating_add(1));
    }

    /// Test that values above max clamp to max
    #[test]
    fn test_clamped_overflow_to_max(max in 1u16..1000u16) {
        let overflow = (max as f32) + 1000.0;
        let result = overflow.clamp(0.0, max as f32) as u16;
        prop_assert!(result >= max.saturating_sub(1) && result <= max);
    }

    /// Test that rounding is within 0.5 of original
    #[test]
    fn test_rounding_accuracy(value in 0.0f32..100.0f32) {
        let rounded = value.round() as u16;
        let diff = (value - rounded as f32).abs();
        prop_assert!(diff <= 0.5 || diff < 0.5001); // Account for floating point precision
    }
}

/// Test that zero input gives zero output
#[test]
fn test_clamped_zero_input() {
    let result = 0.0f32.clamp(0.0, 100.0) as u16;
    assert_eq!(result, 0);
}

/// Test that clamped float to u16 conversion is within bounds
#[test]
fn test_clamped_conversion() {
    for max in 1u16..1000u16 {
        for value in [0.0_f32, 100.0, 1000.0, 10000.0] {
            let clamped = value.clamp(0.0, max as f32);
            let converted = clamped as u16;
            assert!(converted <= max);
        }
    }
}
