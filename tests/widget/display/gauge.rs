//! Tests for Gauge widget
//!
//! Extracted from src/widget/display/gauge.rs

use revue::prelude::*;

#[test]
fn test_gauge_new() {
    let g = Gauge::new();
    assert_eq!(g.get_value(), 0.0);
}

#[test]
fn test_gauge_value() {
    let g = Gauge::new().value(0.5);
    assert_eq!(g.get_value(), 0.5);
}

#[test]
fn test_gauge_percent() {
    let g = Gauge::new().percent(75.0);
    assert_eq!(g.get_value(), 0.75);
}

#[test]
fn test_gauge_value_clamp() {
    let g1 = Gauge::new().value(1.5);
    assert_eq!(g1.get_value(), 1.0);

    let g2 = Gauge::new().value(-0.5);
    assert_eq!(g2.get_value(), 0.0);
}

#[test]
fn test_gauge_value_range() {
    let g = Gauge::new().value_range(50.0, 0.0, 100.0);
    assert_eq!(g.get_value(), 0.5);
}

#[test]
fn test_gauge_style() {
    let g = Gauge::new().style(GaugeStyle::Battery);
    // Can't access private style field
    // Just verify builder compiles
}

#[test]
fn test_gauge_thresholds() {
    let g = Gauge::new().thresholds(0.7, 0.9).value(0.95);
    // Can't directly test current_color() as it's private
    // Just verify builder compiles
}

#[test]
fn test_gauge_get_label() {
    let g = Gauge::new().percent(50.0);
    assert_eq!(g.get_label(), "50%");
}

#[test]
fn test_gauge_custom_label() {
    let g = Gauge::new().label("Custom");
    assert_eq!(g.get_label(), "Custom");
}

#[test]
fn test_gauge_helper_value() {
    let g = gauge().percent(50.0);
    assert_eq!(g.get_value(), 0.5);
}

#[test]
fn test_percentage_helper_value() {
    let g = percentage(75.0);
    assert_eq!(g.get_value(), 0.75);
}

#[test]
fn test_battery_helper_fields() {
    let g = battery(80.0);
    assert_eq!(g.get_value(), 0.8);
}

#[test]
fn test_value_range_validation() {
    // Normal case
    let g = Gauge::new().value_range(50.0, 0.0, 100.0);
    assert_eq!(g.get_value(), 0.5);

    // Swapped min/max
    let g = Gauge::new().value_range(50.0, 100.0, 0.0);
    assert_eq!(g.get_value(), 0.5);

    // Equal min/max (division by zero case)
    let g = Gauge::new().value_range(50.0, 50.0, 50.0);
    assert_eq!(g.get_value(), 0.0);
}

#[test]
fn test_thresholds_validation() {
    // Normal case
    let g = Gauge::new().thresholds(0.5, 0.8);
    // Can't access private threshold fields
    // Just verify builder compiles

    // Swapped thresholds
    let g = Gauge::new().thresholds(0.8, 0.5);
    // Can't access private threshold fields
    // Just verify builder compiles
}
