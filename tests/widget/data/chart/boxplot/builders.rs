//! BoxPlot builder tests: adding groups

use super::super::{count_char, render};
use super::even_stats;
use revue::widget::data::chart::{BoxGroup, BoxPlot};

#[test]
fn test_boxplot_add_group_and_group_stats() {
    let bp = BoxPlot::new()
        .add_group(BoxGroup::new("A", &[1.0, 2.0, 3.0]))
        .group_stats("B", even_stats());
    let buffer = render(&bp, 40, 20);
    // A's box is a single row on B's 0..100 scale, so count the median caps
    assert_eq!(count_char(&buffer, '├'), 2);
}
