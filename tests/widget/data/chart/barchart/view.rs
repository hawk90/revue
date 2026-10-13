//! BarChart value scale and View tests: the maximum, all-zero values and zero areas

use super::super::{count_char, render, render_rows, rows};
use super::bar;
use revue::style::Color;
use revue::widget::data::chart::BarChart;

#[test]
fn test_barchart_auto_max_is_largest_value() {
    // Without max(), the largest bar fills the whole bar area (29 cells)
    let rows = render_rows(&BarChart::new().bar("A", 10.0).bar("B", 50.0), 40, 3);
    assert!(rows[2].contains(&bar(29)));
    assert!(!rows[2].contains(&bar(30)));
    // 10 / 50 * 29 = 5.8
    assert!(rows[0].contains(&format!(" {} ", bar(5))));
    assert!(!rows[0].contains(&bar(6)));
}

#[test]
fn test_barchart_explicit_max() {
    let rows = render_rows(&BarChart::new().bar("A", 50.0).max(200.0), 40, 1);
    // 50 / 200 * 29 = 7.25
    assert_eq!(rows[0].trim_end(), format!("A {} 50.0", bar(7)));
}

#[test]
fn test_barchart_all_zero_values() {
    // Max falls back to 1.0, so zero bars draw nothing
    let chart = BarChart::new().bar("A", 0.0).bar("B", 0.0);
    let buffer = render(&chart, 40, 3);
    assert_eq!(count_char(&buffer, '█'), 0);
    assert_eq!(rows(&buffer)[0].trim_end(), "A  0.0");
}

#[test]
fn test_barchart_zero_area() {
    let chart = BarChart::new().bar("A", 50.0);
    let buffer = render(&chart, 20, 0);
    assert_eq!(buffer.height(), 0);
    let buffer = render(&chart.vertical(), 0, 5);
    assert_eq!(buffer.width(), 0);
}

#[test]
fn test_barchart_zero_area_draws_nothing() {
    let chart = BarChart::new().bar("A", 50.0).bar("B", 20.0);
    super::super::assert_zero_areas_draw_nothing(&chart);
    super::super::assert_zero_areas_draw_nothing(&chart.vertical());
}

#[test]
fn test_barchart_draws_the_same_when_offset() {
    let chart = BarChart::new()
        .bar("Mon", 10.0)
        .bar_colored("Tue", -20.0, Color::RED)
        .bar("수", 15.0)
        .bar_width(2);
    super::super::assert_same_when_offset(chart.clone(), 30, 10);
    super::super::assert_same_when_offset(chart.vertical(), 30, 10);
}
