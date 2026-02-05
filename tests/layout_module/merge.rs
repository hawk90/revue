//! Rect merging tests

use revue::layout::{merge_rects, Rect};

#[test]
fn test_merge_rects_empty() {
    let rects = vec![];
    let result = merge_rects(&rects);
    assert!(result.is_empty());
}

#[test]
fn test_merge_rects_single() {
    let rects = vec![Rect::new(0, 0, 10, 10)];
    let result = merge_rects(&rects);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Rect::new(0, 0, 10, 10));
}

#[test]
fn test_merge_rects_non_overlapping() {
    let rects = vec![Rect::new(0, 0, 10, 10), Rect::new(20, 20, 10, 10)];
    let result = merge_rects(&rects);
    // Non-overlapping rects should remain separate
    assert_eq!(result.len(), 2);
}

#[test]
fn test_merge_rects_overlapping_pair() {
    let rects = vec![Rect::new(0, 0, 20, 20), Rect::new(10, 10, 20, 20)];
    let result = merge_rects(&rects);
    // Overlapping rects should be merged
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].width, 30);
    assert_eq!(result[0].height, 30);
}

#[test]
fn test_merge_rects_chain() {
    let rects = vec![
        Rect::new(0, 0, 10, 10),
        Rect::new(10, 0, 10, 10),
        Rect::new(20, 0, 10, 10),
    ];
    let result = merge_rects(&rects);
    // Adjacent rects don't intersect (edge-to-edge), so they won't merge
    // Only overlapping rects are merged
    // First two: (0-10) and (10-20) don't overlap
    // Third: (20-30) doesn't overlap with second
    assert_eq!(result.len(), 3);
}
