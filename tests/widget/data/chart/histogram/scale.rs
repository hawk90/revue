//! Histogram value scale tests: counts, densities and cumulative frequencies

use super::super::{render, rows};
use super::{bar_top, three_bins};

#[test]
fn test_histogram_density() {
    // Same shape as counts, scaled to densities; labels get two decimals
    let buffer = render(&three_bins().density(true), 40, 20);
    assert_eq!(bar_top(&buffer, 6), Some(0));
    assert_eq!(bar_top(&buffer, 17), Some(12));
    assert_eq!(bar_top(&buffer, 28), Some(6));
    assert!(rows(&buffer)[1].starts_with("0.05"));
}

#[test]
fn test_histogram_cumulative() {
    // Cumulative frequency 0.5, 0.67, 1.0 of 18 rows: heights 9, 12, 18
    let buffer = render(&three_bins().cumulative(true), 40, 20);
    assert_eq!(bar_top(&buffer, 6), Some(9));
    assert_eq!(bar_top(&buffer, 17), Some(6));
    assert_eq!(bar_top(&buffer, 28), Some(0));
    assert!(rows(&buffer)[1].starts_with("1.00"));
}
