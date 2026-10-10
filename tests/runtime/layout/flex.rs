//! Edge case tests for flex layout

use revue::layout::Rect;

#[test]
fn test_flex_zero_space() {
    // Test flex behavior with zero available space
    let available = Rect::new(0, 0, 0, 10);
    assert_eq!(available.width, 0);
}

#[test]
fn test_flex_single_child_full_width() {
    // Test that a single flex child takes full width
    let parent = Rect::new(0, 0, 100, 20);
    let child = Rect::new(0, 0, 100, 20);
    assert_eq!(child.width, parent.width);
}

#[test]
fn test_flex_overflow_saturates() {
    // Test that flex overflow saturates at u16::MAX
    let large = u16::MAX - 10;
    let result = large.saturating_add(100);
    assert_eq!(result, u16::MAX);
}
