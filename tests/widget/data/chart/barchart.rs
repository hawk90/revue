//! Bar Chart widget public API tests extracted from barchart.rs

use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::traits::RenderContext;
use crate::style::Color;

pub use crate::widget::data::chart::barchart::{BarChart, Bar, BarOrientation};

#[test]
fn test_barchart_new() {
    let chart = BarChart::new();
    assert!(chart.bars.is_empty());
    assert_eq!(chart.orientation, BarOrientation::Horizontal);
}

#[test]
fn test_barchart_bar() {
    let chart = BarChart::new().bar("A", 10.0).bar("B", 20.0).bar("C", 30.0);

    assert_eq!(chart.bars.len(), 3);
    assert_eq!(chart.bars[0].label, "A");
    assert_eq!(chart.bars[0].value, 10.0);
}

#[test]
fn test_barchart_data() {
    let data = vec![("Sales", 100.0), ("Revenue", 200.0)];

    let chart = BarChart::new().data(data);
    assert_eq!(chart.bars.len(), 2);
}

#[test]
fn test_barchart_orientation() {
    let h = BarChart::new().horizontal();
    assert_eq!(h.orientation, BarOrientation::Horizontal);

    let v = BarChart::new().vertical();
    assert_eq!(v.orientation, BarOrientation::Vertical);
}

#[test]
fn test_barchart_styling() {
    let chart = BarChart::new()
        .max(100.0)
        .bar_width(2)
        .gap(1)
        .fg(Color::GREEN)
        .show_values(true);

    assert_eq!(chart.max, Some(100.0));
    assert_eq!(chart.bar_width, 2);
    assert_eq!(chart.gap, 1);
    assert_eq!(chart.fg, Color::GREEN);
    assert!(chart.show_values);
}

#[test]
fn test_barchart_render_horizontal() {
    let chart = BarChart::new()
        .bar("A", 50.0)
        .bar("B", 100.0)
        .max(100.0)
        .bar_width(1);

    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(0, 0, 40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Bars should be rendered
}

#[test]
fn test_barchart_render_vertical() {
    let chart = BarChart::new()
        .bar("A", 50.0)
        .bar("B", 100.0)
        .vertical()
        .max(100.0)
        .bar_width(3);

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Vertical bars should be rendered
}

#[test]
fn test_barchart_colored() {
    let chart = BarChart::new()
        .bar_colored("Red", 50.0, Color::RED)
        .bar_colored("Green", 75.0, Color::GREEN)
        .bar_colored("Blue", 100.0, Color::BLUE);

    assert_eq!(chart.bars.len(), 3);
    assert_eq!(chart.bars[0].color, Some(Color::RED));
}

#[test]
fn test_barchart_helper() {
    let chart = barchart().bar("Test", 42.0);

    assert_eq!(chart.bars.len(), 1);
}

#[test]
fn test_barchart_calculate_max() {
    let chart = BarChart::new().bar("A", 10.0).bar("B", 50.0).bar("C", 30.0);

    assert_eq!(chart.calculate_max(), 50.0);

    let chart_with_max = chart.max(100.0);
    assert_eq!(chart_with_max.calculate_max(), 100.0);
}

#[test]
fn test_barchart_empty() {
    let chart = BarChart::new();

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should not panic on empty data
}

#[test]
fn test_bar_struct() {
    let bar = Bar::new("Test", 42.0).color(Color::YELLOW);
    assert_eq!(bar.label, "Test");
    assert_eq!(bar.value, 42.0);
    assert_eq!(bar.color, Some(Color::YELLOW));
}

// =========================================================================
// BarOrientation enum tests (derived traits)
// =========================================================================

#[test]
fn test_bar_orientation_default() {
    assert_eq!(BarOrientation::default(), BarOrientation::Horizontal);
}

#[test]
fn test_bar_orientation_clone() {
    let orientation1 = BarOrientation::Vertical;
    let orientation2 = orientation1.clone();
    assert_eq!(orientation1, orientation2);
}

#[test]
fn test_bar_orientation_copy() {
    let orientation1 = BarOrientation::Horizontal;
    let orientation2 = orientation1;
    assert_eq!(orientation2, BarOrientation::Horizontal);
}

#[test]
fn test_bar_orientation_partial_eq() {
    assert_eq!(BarOrientation::Horizontal, BarOrientation::Horizontal);
    assert_eq!(BarOrientation::Vertical, BarOrientation::Vertical);
    assert_ne!(BarOrientation::Horizontal, BarOrientation::Vertical);
}

// =========================================================================
// BarChart::label_width tests
// =========================================================================

#[test]
fn test_barchart_label_width() {
    let chart = BarChart::new().label_width(10);
    assert_eq!(chart.label_width, Some(10));
}

// =========================================================================
// BarChart::calculate_max edge cases
// =========================================================================

#[test]
fn test_barchart_calculate_max_all_zero() {
    let chart = BarChart::new().bar("A", 0.0).bar("B", 0.0).bar("C", 0.0);
    assert_eq!(chart.calculate_max(), 1.0); // Should be at least 1.0
}

#[test]
fn test_barchart_calculate_max_with_negative() {
    let chart = BarChart::new().bar("A", -50.0).bar("B", 100.0).bar("C", -25.0);
    assert_eq!(chart.calculate_max(), 100.0);
}

// =========================================================================
// BarChart::render horizontal edge cases
// =========================================================================

#[test]
fn test_barchart_render_horizontal_zero_width() {
    let chart = BarChart::new().bar("A", 50.0).bar_width(0);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should handle zero width without panic
}

#[test]
fn test_barchart_render_horizontal_long_labels() {
    let chart = BarChart::new().bar("Very Long Label", 50.0).max(100.0).bar_width(1);

    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should truncate labels
}

// =========================================================================
// BarChart::render vertical edge cases
// =========================================================================

#[test]
fn test_barchart_render_vertical_zero_height() {
    let chart = BarChart::new().bar("A", 50.0).vertical().bar_width(1);

    let mut buffer = Buffer::new(20, 0);
    let area = Rect::new(0, 0, 20, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should handle zero height without panic
}

#[test]
fn test_barchart_render_vertical_long_labels() {
    let chart = BarChart::new()
        .bar("VeryLongLabel", 50.0)
        .vertical()
        .bar_width(5)
        .max(100.0);

    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should truncate labels
}

// =========================================================================
// BarChart::show_values tests
// =========================================================================

#[test]
fn test_barchart_show_values_false() {
    let chart = BarChart::new().show_values(false);
    assert!(!chart.show_values);
}

#[test]
fn test_barchart_show_values_toggle() {
    let chart1 = BarChart::new().show_values(true);
    assert!(chart1.show_values);

    let chart2 = chart1.show_values(false);
    assert!(!chart2.show_values);
}

// =========================================================================
// BarChart::gap tests
// =========================================================================

#[test]
fn test_barchart_gap() {
    let chart = BarChart::new().gap(5);
    assert_eq!(chart.gap, 5);
}

#[test]
fn test_barchart_gap_zero() {
    let chart = BarChart::new().gap(0);
    assert_eq!(chart.gap, 0);
}