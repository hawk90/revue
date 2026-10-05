//! Tests for Gauge widget
//!
//! Extracted from src/widget/display/gauge.rs. The value and helper cases
//! live in tests/widget/gauge.rs; these cover the degenerate range and the
//! threshold colors.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::Gauge;

/// Foreground color of the first (filled) bar cell.
fn fill_color(g: &Gauge) -> Option<Color> {
    let mut buffer = Buffer::new(20, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 1));
    g.render(&mut ctx);
    buffer.get(0, 0).unwrap().fg
}

fn gauge_at(value: f64) -> Gauge {
    Gauge::new()
        .fill_color(Color::GREEN)
        .warning_color(Color::YELLOW)
        .critical_color(Color::RED)
        .value(value)
}

#[test]
fn test_value_range_equal_min_max() {
    // An empty range has no position in it: no division by zero
    let g = Gauge::new().value_range(50.0, 50.0, 50.0);
    assert_eq!(g.get_value(), 0.0);
}

#[test]
fn test_gauge_thresholds() {
    assert_eq!(
        fill_color(&gauge_at(0.3).thresholds(0.5, 0.8)),
        Some(Color::GREEN)
    );
    assert_eq!(
        fill_color(&gauge_at(0.6).thresholds(0.5, 0.8)),
        Some(Color::YELLOW)
    );
    assert_eq!(
        fill_color(&gauge_at(0.95).thresholds(0.5, 0.8)),
        Some(Color::RED)
    );
}

#[test]
fn test_gauge_thresholds_inclusive() {
    assert_eq!(
        fill_color(&gauge_at(0.5).thresholds(0.5, 0.8)),
        Some(Color::YELLOW)
    );
    assert_eq!(
        fill_color(&gauge_at(0.8).thresholds(0.5, 0.8)),
        Some(Color::RED)
    );
}

#[test]
fn test_thresholds_validation() {
    // Swapped thresholds are put back in order
    for v in [0.3, 0.6, 0.95] {
        assert_eq!(
            fill_color(&gauge_at(v).thresholds(0.8, 0.5)),
            fill_color(&gauge_at(v).thresholds(0.5, 0.8)),
        );
    }
}

#[test]
fn test_gauge_without_thresholds_uses_fill_color() {
    assert_eq!(fill_color(&gauge_at(0.99)), Some(Color::GREEN));
}
