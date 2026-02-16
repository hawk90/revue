//! Virtuallist types tests

use revue::widget::data::virtuallist::types::{HeightCalculator, ItemRenderer, ScrollAlignment, ScrollMode};

// =========================================================================
// ScrollMode enum tests
// =========================================================================

#[test]
fn test_scroll_mode_default() {
    assert_eq!(ScrollMode::default(), ScrollMode::Item);
}

#[test]
fn test_scroll_mode_clone() {
    let mode = ScrollMode::Smooth;
    assert_eq!(mode, mode.clone());
}

#[test]
fn test_scroll_mode_copy() {
    let m1 = ScrollMode::Center;
    let m2 = m1;
    assert_eq!(m1, ScrollMode::Center);
    assert_eq!(m2, ScrollMode::Center);
}

#[test]
fn test_scroll_mode_partial_eq() {
    assert_eq!(ScrollMode::Item, ScrollMode::Item);
    assert_eq!(ScrollMode::Smooth, ScrollMode::Smooth);
    assert_eq!(ScrollMode::Center, ScrollMode::Center);
    assert_ne!(ScrollMode::Item, ScrollMode::Smooth);
}

#[test]
fn test_scroll_mode_debug() {
    let debug_str = format!("{:?}", ScrollMode::Smooth);
    assert!(debug_str.contains("Smooth"));
}

// =========================================================================
// ScrollAlignment enum tests
// =========================================================================

#[test]
fn test_scroll_alignment_default() {
    assert_eq!(ScrollAlignment::default(), ScrollAlignment::Start);
}

#[test]
fn test_scroll_alignment_clone() {
    let align = ScrollAlignment::Center;
    assert_eq!(align, align.clone());
}

#[test]
fn test_scroll_alignment_copy() {
    let a1 = ScrollAlignment::End;
    let a2 = a1;
    assert_eq!(a1, ScrollAlignment::End);
    assert_eq!(a2, ScrollAlignment::End);
}

#[test]
fn test_scroll_alignment_partial_eq() {
    assert_eq!(ScrollAlignment::Start, ScrollAlignment::Start);
    assert_eq!(ScrollAlignment::Center, ScrollAlignment::Center);
    assert_eq!(ScrollAlignment::End, ScrollAlignment::End);
    assert_eq!(ScrollAlignment::Nearest, ScrollAlignment::Nearest);
    assert_ne!(ScrollAlignment::Start, ScrollAlignment::End);
}

#[test]
fn test_scroll_alignment_debug() {
    let debug_str = format!("{:?}", ScrollAlignment::Nearest);
    assert!(debug_str.contains("Nearest"));
}

// =========================================================================
// Type alias tests
// =========================================================================

#[test]
fn test_item_renderer_type() {
    // Verify ItemRenderer is a boxed function
    let renderer: ItemRenderer<&str> = Box::new(|item, _idx, _sel| item.to_string());
    assert_eq!(renderer(&"test", 0, false), "test");
}

#[test]
fn test_height_calculator_type() {
    // Verify HeightCalculator is a boxed function
    let calculator: HeightCalculator<&str> = Box::new(|item, _idx| item.len() as u16);
    assert_eq!(calculator(&"hello", 0), 5);
}

// =========================================================================
// ScrollMode variant usage tests
// =========================================================================

#[test]
fn test_scroll_mode_item_variant() {
    let mode = ScrollMode::Item;
    assert!(matches!(mode, ScrollMode::Item));
}

#[test]
fn test_scroll_mode_smooth_variant() {
    let mode = ScrollMode::Smooth;
    assert!(matches!(mode, ScrollMode::Smooth));
}

#[test]
fn test_scroll_mode_center_variant() {
    let mode = ScrollMode::Center;
    assert!(matches!(mode, ScrollMode::Center));
}

// =========================================================================
// ScrollAlignment variant usage tests
// =========================================================================

#[test]
fn test_scroll_alignment_start_variant() {
    let align = ScrollAlignment::Start;
    assert!(matches!(align, ScrollAlignment::Start));
}

#[test]
fn test_scroll_alignment_center_variant() {
    let align = ScrollAlignment::Center;
    assert!(matches!(align, ScrollAlignment::Center));
}

#[test]
fn test_scroll_alignment_end_variant() {
    let align = ScrollAlignment::End;
    assert!(matches!(align, ScrollAlignment::End));
}

#[test]
fn test_scroll_alignment_nearest_variant() {
    let align = ScrollAlignment::Nearest;
    assert!(matches!(align, ScrollAlignment::Nearest));
}

// =========================================================================
// Derived trait implementations
// =========================================================================

#[test]
fn test_all_scroll_mode_values() {
    // Test that all variants can be created and compared
    let modes = [ScrollMode::Item, ScrollMode::Smooth, ScrollMode::Center];

    for (_i, mode) in modes.iter().enumerate() {
        // Verify each mode equals itself
        assert_eq!(*mode, *mode);
        // Verify debug formatting works
        let debug_str = format!("{:?}", mode);
        assert!(!debug_str.is_empty());
        // Verify we can clone it
        let cloned = *mode;
        assert_eq!(mode, &cloned);
    }
}

#[test]
fn test_all_scroll_alignment_values() {
    // Test that all variants can be created and compared
    let alignments = [
        ScrollAlignment::Start,
        ScrollAlignment::Center,
        ScrollAlignment::End,
        ScrollAlignment::Nearest,
    ];

    for (_i, align) in alignments.iter().enumerate() {
        // Verify each alignment equals itself
        assert_eq!(*align, *align);
        // Verify debug formatting works
        let debug_str = format!("{:?}", align);
        assert!(!debug_str.is_empty());
        // Verify we can clone it
        let cloned = *align;
        assert_eq!(align, &cloned);
    }
}

// =========================================================================
// Integration tests with closures
// =========================================================================

#[test]
fn test_item_renderer_closure() {
    let renderer: ItemRenderer<i32> = Box::new(|item, idx, selected| {
        let prefix = if selected { "> " } else { "  " };
        format!("{}[{}] {}", prefix, idx, item)
    });

    assert_eq!(renderer(&42, 0, true), "> [0] 42");
    assert_eq!(renderer(&42, 0, false), "  [0] 42");
}

#[test]
fn test_height_calculator_closure() {
    let calculator: HeightCalculator<String> = Box::new(|item, _idx| {
        (item.len() as u16 + 1) / 2 + 1 // Rough line count
    });

    assert_eq!(calculator(&"short".to_string(), 0), 4); // (5 + 1) / 2 + 1
    assert_eq!(calculator(&"much longer string".to_string(), 0), 10); // (18 + 1) / 2 + 1
}

#[test]
fn test_item_renderer_with_context() {
    let items = vec!["A", "B", "C"];
    let renderer: ItemRenderer<&str> = Box::new(|item, idx, selected| {
        let marker = if selected { "✓" } else { " " };
        format!("{} {}: {}", marker, idx + 1, item)
    });

    assert_eq!(renderer(&items[0], 0, false), "  1: A");
    assert_eq!(renderer(&items[1], 1, true), "✓ 2: B");
}

#[test]
fn test_height_calculator_with_multi_line() {
    let calculator: HeightCalculator<&str> = Box::new(|item, _idx| item.lines().count() as u16);

    assert_eq!(calculator(&"single line", 0), 1);
    assert_eq!(calculator(&"line 1\nline 2", 0), 2);
    assert_eq!(calculator(&"line 1\nline 2\nline 3", 0), 3);
}
