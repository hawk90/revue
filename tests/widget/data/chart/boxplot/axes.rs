//! BoxPlot axis label tests: values and group names

use super::super::{render, rows};
use super::even_stats;
use revue::widget::data::chart::BoxPlot;

#[test]
fn test_boxplot_value_axis_labels() {
    let bp = BoxPlot::new().group_stats("Data", even_stats());
    let rows = rows(&render(&bp, 40, 20));
    // Five labels from the padded max (110) down to the padded min (-10)
    assert!(rows[1].starts_with("110.0"));
    assert!(rows[5].starts_with("80.0"));
    assert!(rows[9].starts_with("50.0"));
    assert!(rows[13].starts_with("20.0"));
    assert!(rows[18].starts_with("-10.0"));
}

#[test]
fn test_boxplot_category_labels() {
    let bp = BoxPlot::new()
        .group("A", &[1.0, 2.0, 3.0])
        .group("B", &[4.0, 5.0, 6.0]);
    let rows = rows(&render(&bp, 40, 20));
    // Groups split the 34 columns right of the value labels: centers 14 and 31
    let last: Vec<char> = rows[19].chars().collect();
    assert_eq!(last[14], 'A');
    assert_eq!(last[31], 'B');
}

#[test]
fn test_boxplot_long_category_label_clipped() {
    let bp = BoxPlot::new().group("VeryLongGroupName", &[1.0, 2.0, 3.0]);
    let rows = rows(&render(&bp, 20, 10));
    // Label is centered on column 13, so it starts at column 5; cells left
    // of the chart area (the value-label column, x < 6) are not drawn
    assert_eq!(rows[9].trim_end(), "      eryLongGroupNa");
}
