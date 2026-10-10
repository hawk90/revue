//! Virtuallist types tests

use revue::widget::data::virtuallist::types::{
    HeightCalculator, ItemRenderer, ScrollAlignment, ScrollMode,
};

// =========================================================================
// ScrollMode enum tests
// =========================================================================

#[test]
fn test_scroll_mode_default() {
    assert_eq!(ScrollMode::default(), ScrollMode::Item);
}

#[test]
#[allow(clippy::clone_on_copy)] // exercises the Clone impl itself
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
#[allow(clippy::clone_on_copy)] // exercises the Clone impl itself
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
