//! Rect tests

mod css_layout;
mod engine;
mod flex;
mod responsive;
mod tree;

use revue::layout::{merge_rects, Rect};

#[test]
fn test_rect_new() {
    let rect = Rect::new(10, 20, 30, 40);
    assert_eq!(rect.x, 10);
    assert_eq!(rect.y, 20);
    assert_eq!(rect.width, 30);
    assert_eq!(rect.height, 40);
}

#[test]
fn test_rect_contains() {
    let rect = Rect::new(10, 10, 20, 20);

    assert!(rect.contains(10, 10)); // Top-left
    assert!(rect.contains(15, 15)); // Center
    assert!(rect.contains(29, 29)); // Just inside
    assert!(!rect.contains(30, 30)); // Just outside
    assert!(!rect.contains(5, 15)); // Left of rect
}

#[test]
fn test_rect_edges() {
    let rect = Rect::new(10, 20, 30, 40);
    assert_eq!(rect.right(), 40);
    assert_eq!(rect.bottom(), 60);
}

#[test]
fn test_rect_intersects() {
    let r1 = Rect::new(0, 0, 20, 20);
    let r2 = Rect::new(10, 10, 20, 20);
    let r3 = Rect::new(100, 100, 10, 10);

    assert!(r1.intersects(&r2));
    assert!(r2.intersects(&r1));
    assert!(!r1.intersects(&r3));
}

#[test]
fn test_rect_intersection() {
    let r1 = Rect::new(0, 0, 20, 20);
    let r2 = Rect::new(10, 10, 20, 20);

    let intersection = r1.intersection(&r2).unwrap();
    assert_eq!(intersection, Rect::new(10, 10, 10, 10));

    let r3 = Rect::new(100, 100, 10, 10);
    assert!(r1.intersection(&r3).is_none());
}

#[test]
fn test_rect_union() {
    let r1 = Rect::new(0, 0, 20, 20);
    let r2 = Rect::new(10, 10, 30, 30);

    let union = r1.union(&r2);
    assert_eq!(union, Rect::new(0, 0, 40, 40));
}

#[test]
fn test_merge_rects_empty() {
    {
        let rects: Vec<Rect> = vec![];
        let merged = merge_rects(&rects);
        assert!(merged.is_empty());
    }
    {
        let rects = vec![];
        let result = merge_rects(&rects);
        assert!(result.is_empty());
    }
}

#[test]
fn test_merge_rects_single() {
    {
        let rects = vec![Rect::new(0, 0, 10, 10)];
        let merged = merge_rects(&rects);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0], Rect::new(0, 0, 10, 10));
    }
    {
        let rects = vec![Rect::new(0, 0, 10, 10)];
        let result = merge_rects(&rects);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], Rect::new(0, 0, 10, 10));
    }
}

#[test]
fn test_merge_rects_overlapping() {
    let rects = vec![Rect::new(0, 0, 20, 20), Rect::new(10, 10, 20, 20)];
    let merged = merge_rects(&rects);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0], Rect::new(0, 0, 30, 30));
}

#[test]
fn test_merge_rects_non_overlapping() {
    {
        let rects = vec![Rect::new(0, 0, 10, 10), Rect::new(50, 50, 10, 10)];
        let merged = merge_rects(&rects);
        assert_eq!(merged.len(), 2);
        assert!(merged.contains(&Rect::new(0, 0, 10, 10)));
        assert!(merged.contains(&Rect::new(50, 50, 10, 10)));
    }
    {
        let rects = vec![Rect::new(0, 0, 10, 10), Rect::new(20, 20, 10, 10)];
        let result = merge_rects(&rects);
        // Non-overlapping rects should remain separate
        assert_eq!(result.len(), 2);
    }
}

#[test]
fn test_merge_rects_multiple_overlapping() {
    let rects = vec![
        Rect::new(0, 0, 10, 10),
        Rect::new(5, 5, 10, 10),
        Rect::new(10, 10, 10, 10),
    ];
    let merged = merge_rects(&rects);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0], Rect::new(0, 0, 20, 20));
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
