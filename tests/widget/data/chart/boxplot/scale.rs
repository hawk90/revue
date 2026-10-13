//! BoxPlot value scale tests: axis bound overrides

use super::super::{render, rows};
use super::even_stats;
use revue::widget::data::chart::{Axis, BoxPlot};

#[test]
fn test_boxplot_value_axis_bounds_override() {
    let bp = BoxPlot::new()
        .group_stats("Data", even_stats())
        .value_axis(Axis::new().bounds(0.0, 1000.0));
    let rows = rows(&render(&bp, 40, 20));
    // Padding still applies to the overridden range (-100..1100)
    assert!(rows[1].starts_with("1100"));
    assert!(rows[18].starts_with("-100."));
}
