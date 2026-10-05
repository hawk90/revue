//! ClipRegion tests

use revue::widget::ClipRegion;

#[test]
fn test_clip_region_intersect_contained() {
    let clip1 = ClipRegion::new(0.0, 0.0, 100.0, 100.0);
    let clip2 = ClipRegion::new(25.0, 25.0, 50.0, 50.0);

    let result = clip1.intersect(&clip2);
    assert!(result.is_some());

    let intersection = result.unwrap();
    // Should return the smaller region (clip2)
    assert_eq!(intersection.x_min, 25.0);
    assert_eq!(intersection.y_min, 25.0);
    assert_eq!(intersection.x_max, 75.0);
    assert_eq!(intersection.y_max, 75.0);
}

#[test]
fn test_clip_region_intersect_touching_edge() {
    let clip1 = ClipRegion::new(0.0, 0.0, 50.0, 50.0);
    let clip2 = ClipRegion::new(50.0, 0.0, 50.0, 50.0);

    // Bounds are inclusive, so regions sharing an edge intersect in a
    // zero-width strip along it
    let edge = clip1.intersect(&clip2).unwrap();
    assert_eq!(edge.x_min, 50.0);
    assert_eq!(edge.x_max, 50.0);
    assert_eq!(edge.y_min, 0.0);
    assert_eq!(edge.y_max, 50.0);
    assert!(edge.contains(50.0, 25.0));
    assert!(!edge.contains(49.9, 25.0));
}

#[test]
fn test_clip_region_intersect_same_region() {
    let clip1 = ClipRegion::new(10.0, 20.0, 100.0, 50.0);
    let clip2 = ClipRegion::new(10.0, 20.0, 100.0, 50.0);

    let result = clip1.intersect(&clip2);
    assert!(result.is_some());

    let intersection = result.unwrap();
    assert_eq!(intersection.x_min, 10.0);
    assert_eq!(intersection.y_min, 20.0);
    assert_eq!(intersection.x_max, 110.0);
    assert_eq!(intersection.y_max, 70.0);
}

#[test]
fn test_clip_region_intersect_negative_coords() {
    let clip1 = ClipRegion::from_bounds(-50.0, -50.0, 0.0, 0.0);
    let clip2 = ClipRegion::from_bounds(-25.0, -25.0, 25.0, 25.0);

    let result = clip1.intersect(&clip2);
    assert!(result.is_some());

    let intersection = result.unwrap();
    assert_eq!(intersection.x_min, -25.0);
    assert_eq!(intersection.y_min, -25.0);
    assert_eq!(intersection.x_max, 0.0);
    assert_eq!(intersection.y_max, 0.0);
}

#[test]
fn test_clip_region_copy() {
    let clip1 = ClipRegion::new(10.0, 20.0, 100.0, 50.0);
    let clip2 = clip1;

    assert_eq!(clip2.x_min, 10.0);
    assert_eq!(clip2.y_min, 20.0);
}

#[test]
fn test_clip_region_clone() {
    let clip1 = ClipRegion::new(10.0, 20.0, 100.0, 50.0);
    let clip2 = clip1.clone();

    assert_eq!(clip2.x_min, 10.0);
    assert_eq!(clip2.y_min, 20.0);
}

#[test]
fn test_clip_region_zero_size() {
    let clip = ClipRegion::new(50.0, 50.0, 0.0, 0.0);
    assert_eq!(clip.x_min, 50.0);
    assert_eq!(clip.x_max, 50.0);
    assert_eq!(clip.y_min, 50.0);
    assert_eq!(clip.y_max, 50.0);

    // Zero-sized region still contains its origin point
    assert!(clip.contains(50.0, 50.0));
}

#[test]
fn test_clip_region_from_bounds_inverted() {
    // Create a region with inverted coordinates (x_max < x_min)
    let clip = ClipRegion::from_bounds(100.0, 100.0, 50.0, 50.0);
    // This creates an invalid region, but it's allowed
    assert_eq!(clip.x_min, 100.0);
    assert_eq!(clip.x_max, 50.0);

    // contains() should still work with inverted coordinates
    assert!(!clip.contains(75.0, 75.0));
}

#[test]
fn test_clip_region_square() {
    let clip = ClipRegion::new(0.0, 0.0, 50.0, 50.0);
    assert!(clip.contains(25.0, 25.0));
    assert!(clip.contains(0.0, 50.0));
    assert!(clip.contains(50.0, 0.0));
}

#[test]
fn test_clip_region_wide_rectangle() {
    let clip = ClipRegion::new(0.0, 0.0, 200.0, 50.0);
    assert!(clip.contains(100.0, 25.0));
    assert!(!clip.contains(250.0, 25.0));
}

#[test]
fn test_clip_region_tall_rectangle() {
    let clip = ClipRegion::new(0.0, 0.0, 50.0, 200.0);
    assert!(clip.contains(25.0, 100.0));
    assert!(!clip.contains(25.0, 250.0));
}
