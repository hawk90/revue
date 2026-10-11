//! Zero, maximum and overflowing sizes, ratios and coordinates.

use revue::widget::{HSplit, VSplit};

#[test]
fn test_split_with_max_ratio() {
    let split = HSplit::new(0.9);
    assert!(split.ratio <= 0.9);
}

#[test]
fn test_splitter_ratio_zero() {
    let split = HSplit::new(0.0);
    // Should clamp to minimum
    assert!(split.ratio >= 0.1);
}

#[test]
fn test_splitter_ratio_one() {
    let split = HSplit::new(1.0);
    // Should clamp to maximum
    assert!(split.ratio <= 0.9);
}

#[test]
fn test_vsplit_ratio_boundaries() {
    let split_min = VSplit::new(0.0);
    let split_max = VSplit::new(1.0);

    assert!(split_min.ratio >= 0.1);
    assert!(split_max.ratio <= 0.9);
}
