//! Tests for bar chart public API
//!
//! Extracted from src/widget/data/chart/barchart.rs

use revue::widget::data::chart::{BarChart, Bar, BarOrientation, barchart};
use revue::style::Color;

#[test]
fn test_bar_new() {
    let bar = Bar::new("Test", 10.0);
    assert_eq!(bar.label, "Test");
    assert_eq!(bar.value, 10.0);
    assert!(bar.color.is_none());
}

#[test]
fn test_bar_with_color() {
    let bar = Bar::new("Test", 10.0).color(Color::RED);
    assert_eq!(bar.color, Some(Color::RED));
}

#[test]
fn test_bar_chart_new() {
    let chart = BarChart::new();
    assert!(chart.bars.is_empty());
    assert_eq!(chart.orientation, BarOrientation::default());
    assert_eq!(chart.max, None);
    assert_eq!(chart.bar_width, 1);
    assert_eq!(chart.gap, 1);
    assert!(chart.show_values);
    assert_eq!(chart.fg, Color::CYAN);
}

#[test]
fn test_bar_chart_bar() {
    let chart = BarChart::new()
        .bar("Sales", 150.0)
        .bar("Revenue", 200.0);

    assert_eq!(chart.bars.len(), 2);
    assert_eq!(chart.bars[0].label, "Sales");
    assert_eq!(chart.bars[0].value, 150.0);
    assert_eq!(chart.bars[1].label, "Revenue");
    assert_eq!(chart.bars[1].value, 200.0);
}

#[test]
fn test_bar_chart_bar_colored() {
    let chart = BarChart::new()
        .bar_colored("Sales", 150.0, Color::GREEN)
        .bar_colored("Revenue", 200.0, Color::BLUE);

    assert_eq!(chart.bars.len(), 2);
    assert_eq!(chart.bars[0].color, Some(Color::GREEN));
    assert_eq!(chart.bars[1].color, Some(Color::BLUE));
}

#[test]
fn test_bar_chart_data() {
    let data = vec![
        ("Sales", 150.0),
        ("Revenue", 200.0),
        ("Profit", 75.0),
    ];
    let chart = BarChart::new().data(data);

    assert_eq!(chart.bars.len(), 3);
    assert_eq!(chart.bars[0].label, "Sales");
    assert_eq!(chart.bars[1].label, "Revenue");
    assert_eq!(chart.bars[2].label, "Profit");
}

#[test]
fn test_bar_chart_orientation() {
    let chart = BarChart::new().orientation(BarOrientation::Vertical);
    assert_eq!(chart.orientation, BarOrientation::Vertical);

    let chart = BarChart::new().vertical();
    assert_eq!(chart.orientation, BarOrientation::Vertical);

    let chart = BarChart::new().horizontal();
    assert_eq!(chart.orientation, BarOrientation::Horizontal);
}

#[test]
fn test_bar_chart_max() {
    let chart = BarChart::new().max(250.0);
    assert_eq!(chart.max, Some(250.0));
}

#[test]
fn test_bar_chart_bar_width() {
    let chart = BarChart::new().bar_width(3);
    assert_eq!(chart.bar_width, 3);

    // Test minimum width
    let chart = BarChart::new().bar_width(0);
    assert_eq!(chart.bar_width, 1);
}

#[test]
fn test_bar_chart_gap() {
    let chart = BarChart::new().gap(2);
    assert_eq!(chart.gap, 2);
}

#[test]
fn test_bar_chart_show_values() {
    let chart = BarChart::new().show_values(false);
    assert!(!chart.show_values);

    let chart = BarChart::new().show_values(true);
    assert!(chart.show_values);
}

#[test]
fn test_bar_chart_fg() {
    let chart = BarChart::new().fg(Color::YELLOW);
    assert_eq!(chart.fg, Color::YELLOW);
}

#[test]
fn test_bar_chart_label_width() {
    let chart = BarChart::new().label_width(10);
    assert_eq!(chart.label_width, Some(10));
}

#[test]
fn test_bar_chart_calculate_max() {
    let chart = BarChart::new()
        .bar("A", 10.0)
        .bar("B", 20.0)
        .bar("C", 30.0);

    assert_eq!(chart.calculate_max(), 30.0);

    let chart = chart.max(50.0);
    assert_eq!(chart.calculate_max(), 50.0);

    let chart = BarChart::new();
    assert_eq!(chart.calculate_max(), 1.0); // Default minimum
}

#[test]
fn test_bar_helper() {
    let chart = barchart();
    assert!(chart.bars.is_empty());
}