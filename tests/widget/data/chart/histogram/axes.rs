//! Histogram axis label tests, both orientations

use super::super::render_rows;
use super::three_bins;

#[test]
fn test_histogram_axis_labels() {
    let rows = render_rows(&three_bins(), 40, 20);
    // Y: count from max (3) down to 0
    assert!(rows[1].starts_with("3 "), "{:?}", rows[1]);
    assert!(rows[18].starts_with("0 "), "{:?}", rows[18]);
    // X: bin range 0..30 in five ticks
    assert!(
        rows[19].starts_with("      0.00    7.5      15.0    22.5"),
        "{:?}",
        rows[19]
    );
}

#[test]
fn test_histogram_horizontal_axis_labels() {
    let rows = render_rows(&three_bins().horizontal(), 40, 20);
    // Bin values down the left side, lowest first
    assert!(rows[0].starts_with("0.00 "), "{:?}", rows[0]);
    assert!(rows[4].starts_with("7.5 "), "{:?}", rows[4]);
    assert!(rows[17].starts_with("30.0 "), "{:?}", rows[17]);
    // Counts along the bottom, from 0 on the left to the maximum on the right
    assert!(rows[19].starts_with("      0 "), "{:?}", rows[19]);
    assert!(rows[19].trim_end().ends_with('3'), "{:?}", rows[19]);
}
