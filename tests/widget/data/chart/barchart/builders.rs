//! BarChart builder tests: construction, the constructor function, data and orientation

use super::super::{render, render_rows, rows};
use revue::widget::data::chart::{barchart, BarChart, BarOrientation};

#[test]
fn test_barchart_new_is_empty() {
    let buffer = render(&BarChart::new(), 20, 10);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_barchart_helper_matches_new() {
    let a = render_rows(&barchart().bar("Test", 42.0), 30, 3);
    let b = render_rows(&BarChart::new().bar("Test", 42.0), 30, 3);
    assert_eq!(a, b);
    assert!(a[0].starts_with("Test █"));
}

#[test]
fn test_barchart_default_matches_new() {
    let a = render_rows(&BarChart::default().bar("A", 1.0), 30, 3);
    let b = render_rows(&BarChart::new().bar("A", 1.0), 30, 3);
    assert_eq!(a, b);
}

#[test]
fn test_barchart_bar_and_data_agree() {
    let by_bar = BarChart::new()
        .bar("Sales", 150.0)
        .bar("Revenue", 200.0)
        .bar("Profit", 75.0);
    let by_data =
        BarChart::new().data(vec![("Sales", 150.0), ("Revenue", 200.0), ("Profit", 75.0)]);
    let rows = render_rows(&by_bar, 60, 6);
    assert_eq!(rows, render_rows(&by_data, 60, 6));

    // Labels are right-aligned to the widest label, in insertion order
    assert!(rows[0].starts_with("  Sales █"));
    assert!(rows[2].starts_with("Revenue █"));
    assert!(rows[4].starts_with(" Profit █"));
}

#[test]
fn test_barchart_orientation_builders() {
    let data = [("A", 1.0)];
    let v1 = render_rows(&BarChart::new().data(data).vertical(), 10, 5);
    let v2 = render_rows(
        &BarChart::new()
            .data(data)
            .orientation(BarOrientation::Vertical),
        10,
        5,
    );
    let h1 = render_rows(&BarChart::new().data(data).vertical().horizontal(), 10, 5);
    let h2 = render_rows(&BarChart::new().data(data), 10, 5);
    assert_eq!(v1, v2);
    assert_eq!(h1, h2);
    assert_ne!(v1, h1);
    // Vertical: label sits on the bottom row
    assert!(v1[4].starts_with('A'));
}
