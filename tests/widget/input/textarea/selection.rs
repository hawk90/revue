//! Tests for public selection APIs

use revue::widget::input::input_widgets::textarea::selection::Selection;

#[test]
fn test_selection_new() {
    let sel = Selection::new((1, 5), (3, 10));
    assert_eq!(sel.start, (1, 5));
    assert_eq!(sel.end, (3, 10));
}

#[test]
fn test_selection_normalized_already_normalized() {
    let sel = Selection::new((1, 5), (3, 10));
    let norm = sel.normalized();
    assert_eq!(norm.start, (1, 5));
    assert_eq!(norm.end, (3, 10));
}

#[test]
fn test_selection_normalized_reversed_lines() {
    let sel = Selection::new((3, 10), (1, 5));
    let norm = sel.normalized();
    assert_eq!(norm.start, (1, 5));
    assert_eq!(norm.end, (3, 10));
}

#[test]
fn test_selection_normalized_same_line_reversed() {
    let sel = Selection::new((2, 10), (2, 5));
    let norm = sel.normalized();
    assert_eq!(norm.start, (2, 5));
    assert_eq!(norm.end, (2, 10));
}

#[test]
fn test_selection_normalized_same_line_already_normalized() {
    let sel = Selection::new((2, 5), (2, 10));
    let norm = sel.normalized();
    assert_eq!(norm.start, (2, 5));
    assert_eq!(norm.end, (2, 10));
}

#[test]
fn test_selection_contains_single_line() {
    let sel = Selection::new((2, 5), (2, 10));
    // Within selection
    assert!(sel.contains(2, 5));
    assert!(sel.contains(2, 7));
    assert!(sel.contains(2, 9));
    // Outside selection (end is exclusive)
    assert!(!sel.contains(2, 10));
    assert!(!sel.contains(2, 4));
    assert!(!sel.contains(1, 7));
    assert!(!sel.contains(3, 7));
}

#[test]
fn test_selection_contains_multi_line() {
    let sel = Selection::new((1, 5), (3, 10));
    // First line - from column 5 onwards
    assert!(sel.contains(1, 5));
    assert!(sel.contains(1, 10));
    assert!(sel.contains(1, 100));
    assert!(!sel.contains(1, 4));
    // Middle line - all columns
    assert!(sel.contains(2, 0));
    assert!(sel.contains(2, 50));
    // Last line - up to column 10 (exclusive)
    assert!(sel.contains(3, 0));
    assert!(sel.contains(3, 9));
    assert!(!sel.contains(3, 10));
    assert!(!sel.contains(3, 15));
    // Outside lines
    assert!(!sel.contains(0, 5));
    assert!(!sel.contains(4, 5));
}

#[test]
fn test_selection_contains_reversed_selection() {
    // Selection specified backwards should still work
    let sel = Selection::new((3, 10), (1, 5));
    assert!(sel.contains(2, 0));
    assert!(sel.contains(1, 5));
    assert!(!sel.contains(1, 4));
}

#[test]
fn test_selection_contains_line_before_start() {
    let sel = Selection::new((5, 0), (10, 5));
    assert!(!sel.contains(4, 0));
    assert!(!sel.contains(4, 100));
}

#[test]
fn test_selection_contains_line_after_end() {
    let sel = Selection::new((5, 0), (10, 5));
    assert!(!sel.contains(11, 0));
    assert!(!sel.contains(100, 0));
}

#[test]
fn test_selection_empty() {
    let sel = Selection::new((1, 5), (1, 5));
    assert_eq!(sel.start, (1, 5));
    assert_eq!(sel.end, (1, 5));
    // Empty selection doesn't contain anything (end is exclusive)
    assert!(!sel.contains(1, 5));
}

#[test]
fn test_selection_zero_position() {
    let sel = Selection::new((0, 0), (0, 5));
    assert!(sel.contains(0, 0));
    assert!(sel.contains(0, 4));
    assert!(!sel.contains(0, 5));
}

#[test]
fn test_selection_large_values() {
    let sel = Selection::new((1000, 1000), (2000, 2000));
    assert!(sel.contains(1500, 1500));
    assert!(!sel.contains(0, 0));
    assert!(!sel.contains(3000, 3000));
}

#[test]
fn test_selection_contains_same_line_start_column() {
    let sel = Selection::new((2, 5), (2, 10));
    assert!(sel.contains(2, 5));
}

#[test]
fn test_selection_contains_same_line_end_column() {
    let sel = Selection::new((2, 5), (2, 10));
    // End column is exclusive
    assert!(!sel.contains(2, 10));
}

#[test]
fn test_selection_contains_middle_line_all_columns() {
    let sel = Selection::new((1, 5), (5, 10));
    // Line 3 is in the middle - should contain all columns
    assert!(sel.contains(3, 0));
    assert!(sel.contains(3, 999));
}

#[test]
fn test_selection_contains_first_line_boundary() {
    let sel = Selection::new((2, 5), (4, 10));
    // First line boundary
    assert!(sel.contains(2, 5));
    assert!(!sel.contains(2, 4));
}

#[test]
fn test_selection_contains_last_line_boundary() {
    let sel = Selection::new((2, 5), (4, 10));
    // Last line boundary
    assert!(sel.contains(4, 9));
    assert!(!sel.contains(4, 10));
}

#[test]
fn test_selection_normalized_idempotent() {
    let sel = Selection::new((3, 10), (1, 5));
    let norm1 = sel.normalized();
    let norm2 = norm1.normalized();
    assert_eq!(norm1, norm2);
}

#[test]
fn test_selection_normalized_zero() {
    let sel = Selection::new((0, 0), (0, 0));
    let norm = sel.normalized();
    assert_eq!(norm.start, (0, 0));
    assert_eq!(norm.end, (0, 0));
}

#[test]
fn test_selection_public_fields() {
    let sel = Selection {
        start: (5, 10),
        end: (15, 20),
    };
    assert_eq!(sel.start, (5, 10));
    assert_eq!(sel.end, (15, 20));
}

#[test]
fn test_selection_contains_on_single_column_selection() {
    let sel = Selection::new((2, 5), (2, 6));
    assert!(sel.contains(2, 5));
    assert!(!sel.contains(2, 6));
    assert!(!sel.contains(2, 4));
}

#[test]
fn test_selection_contains_two_lines_same_column() {
    let sel = Selection::new((2, 10), (3, 10));
    // Both lines have same column
    assert!(sel.contains(2, 10));
    assert!(sel.contains(3, 9));
    assert!(!sel.contains(3, 10));
}

#[test]
fn test_selection_contains_touching_selections() {
    // Test adjacent selections (should not overlap)
    let sel1 = Selection::new((0, 0), (0, 5));
    let sel2 = Selection::new((0, 5), (0, 10));

    // sel1 should not include column 5 (end is exclusive)
    assert!(!sel1.contains(0, 5));
    // sel2 should start at column 5
    assert!(sel2.contains(0, 5));
}

#[test]
fn test_selection_contains_range() {
    let sel = Selection::new((2, 3), (4, 8));

    // Test single character in the middle of the selection
    assert!(sel.contains(2, 4));  // In first line
    assert!(sel.contains(2, 7));  // Still in first line

    // Test second line (entire line is selected)
    assert!(sel.contains(3, 0));
    assert!(sel.contains(3, 999));

    // Test last line boundary
    assert!(sel.contains(4, 7));
    assert!(!sel.contains(4, 8));  // End is exclusive
}

#[test]
fn test_selection_equal_after_normalize() {
    // Two different selections that should be equal after normalization
    let sel1 = Selection::new((1, 5), (3, 10));
    let sel2 = Selection::new((3, 10), (1, 5));

    let norm1 = sel1.normalized();
    let norm2 = sel2.normalized();

    assert_eq!(norm1, norm2);
}

#[test]
fn test_selection_nonequal_after_normalize() {
    // Two selections that should remain different after normalization
    let sel1 = Selection::new((1, 5), (3, 10));
    let sel2 = Selection::new((1, 6), (3, 10));

    let norm1 = sel1.normalized();
    let norm2 = sel2.normalized();

    assert_ne!(norm1, norm2);
}

#[test]
fn test_selection_clone_struct() {
    let original = Selection::new((1, 2), (3, 4));
    let cloned = original.clone();

    // Should be equal
    assert_eq!(original, cloned);

    // Should be independent
    let mut mutated = cloned;
    mutated.start = (5, 6);
    assert_ne!(original, mutated);
}

#[test]
fn test_selection_debug_format() {
    let sel = Selection::new((1, 2), (3, 4));
    let debug_str = format!("{:?}", sel);

    // Should contain "Selection"
    assert!(debug_str.contains("Selection"));
    // Should contain the coordinates
    assert!(debug_str.contains("1"));
    assert!(debug_str.contains("2"));
    assert!(debug_str.contains("3"));
    assert!(debug_str.contains("4"));
}