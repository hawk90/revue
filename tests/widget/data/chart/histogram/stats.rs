//! Histogram statistics tests: mean and median values and their overlay lines

use super::super::{count_char, render};
use super::{sym, three_bins};
use revue::widget::data::chart::Histogram;

#[test]
fn test_histogram_mean_median() {
    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    assert_eq!(hist.mean(), Some(3.0));
    assert_eq!(hist.median(), Some(3.0));
    let hist = Histogram::new(&[1.0, 2.0, 3.0, 4.0]);
    assert_eq!(hist.median(), Some(2.5));
    let empty = Histogram::new(&[]);
    assert_eq!(empty.mean(), None);
    assert_eq!(empty.median(), None);
}

#[test]
fn test_histogram_show_stats() {
    // mean 10 -> column 17, median 7 -> column 13
    let plain = render(&three_bins(), 40, 20);
    assert_eq!(count_char(&plain, 'μ'), 0);

    let buffer = render(&three_bins().show_stats(true), 40, 20);
    for y in 1..18 {
        assert_eq!(sym(&buffer, 17, y), '│', "mean y={y}");
        assert_eq!(sym(&buffer, 13, y), '┊', "median y={y}");
    }
    assert_eq!(sym(&buffer, 18, 0), 'μ');
    assert_eq!(sym(&buffer, 14, 0), 'M');
}

#[test]
fn test_histogram_horizontal_show_stats() {
    // mean 10 -> row 6, median 7 -> row 4, each across the whole plot
    let buffer = render(&three_bins().horizontal().show_stats(true), 40, 20);
    for x in 6..39 {
        assert_eq!(sym(&buffer, x, 6), '─', "mean x={x}");
        assert_eq!(sym(&buffer, x, 4), '┄', "median x={x}");
    }
    assert_eq!(sym(&buffer, 38, 7), 'μ');
    assert_eq!(sym(&buffer, 38, 5), 'M');
}
