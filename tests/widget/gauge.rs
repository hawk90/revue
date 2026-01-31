//! Gauge widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{battery, gauge, percentage, Gauge, GaugeStyle, View};

#[test]
fn test_gauge_set_get_value() {
    let mut g = Gauge::new();
    g.set_value(0.8);
    assert_eq!(g.get_value(), 0.8);
}

#[test]
fn test_gauge_render_all_styles() {
    let styles = [
        GaugeStyle::Bar,
        GaugeStyle::Battery,
        GaugeStyle::Thermometer,
        GaugeStyle::Arc,
        GaugeStyle::Circle,
        GaugeStyle::Vertical,
        GaugeStyle::Segments,
        GaugeStyle::Dots,
    ];

    for style in styles {
        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let g = Gauge::new().style(style).percent(50.0);
        g.render(&mut ctx);
    }
}

#[test]
fn test_gauge_with_title() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let g = Gauge::new().title("CPU Usage").percent(75.0);
    g.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'C');
}

#[test]
fn test_gauge_helper() {
    let g = gauge().percent(50.0);
    assert_eq!(g.get_value(), 0.5);
}

#[test]
fn test_percentage_helper() {
    let g = percentage(75.0);
    assert_eq!(g.get_value(), 0.75);
}

#[test]
fn test_battery_helper() {
    let g = battery(80.0);
    assert_eq!(g.get_value(), 0.8);
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[test]
fn test_gauge_zero_percent() {
    let g = Gauge::new().percent(0.0);
    assert_eq!(g.get_value(), 0.0);

    // Should render without panicking
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    g.render(&mut ctx);
}

#[test]
fn test_gauge_hundred_percent() {
    let g = Gauge::new().percent(100.0);
    assert_eq!(g.get_value(), 1.0);

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    g.render(&mut ctx);
}

#[test]
fn test_gauge_negative_value_clamped() {
    // Negative values should be clamped to 0
    let g = Gauge::new().percent(-50.0);
    assert_eq!(g.get_value(), 0.0);
}

#[test]
fn test_gauge_over_100_clamped() {
    // Values over 100 should be clamped to 1.0
    let g = Gauge::new().percent(150.0);
    assert_eq!(g.get_value(), 1.0);
}

#[test]
fn test_gauge_fractional_percent() {
    // Test fractional percentages
    let g = Gauge::new().percent(37.5);
    assert!((g.get_value() - 0.375).abs() < 0.001);

    let g = Gauge::new().percent(0.1);
    assert!((g.get_value() - 0.001).abs() < 0.0001);

    let g = Gauge::new().percent(99.9);
    assert!((g.get_value() - 0.999).abs() < 0.001);
}

#[test]
fn test_gauge_very_small_area() {
    // Test rendering in minimal space
    let mut buffer = Buffer::new(3, 1);
    let area = Rect::new(0, 0, 3, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let g = Gauge::new().percent(50.0);
    g.render(&mut ctx); // Should not panic
}

#[test]
fn test_gauge_zero_width() {
    // Test handling of zero width area
    let mut buffer = Buffer::new(0, 5);
    let area = Rect::new(0, 0, 0, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let g = Gauge::new().percent(50.0);
    g.render(&mut ctx); // Should handle gracefully
}

#[test]
fn test_gauge_zero_height() {
    // Test handling of zero height area
    let mut buffer = Buffer::new(20, 0);
    let area = Rect::new(0, 0, 20, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let g = Gauge::new().percent(50.0);
    g.render(&mut ctx); // Should handle gracefully
}

#[test]
fn test_gauge_long_title_truncation() {
    // Very long title should be handled
    let long_title = "This is a very long gauge title that might get truncated";
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let g = Gauge::new().title(long_title).percent(50.0);
    g.render(&mut ctx); // Should handle without panic
}

#[test]
fn test_gauge_unicode_title() {
    // Test unicode characters in title
    let unicode_title = "📊 CPU 使用率";
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let g = Gauge::new().title(unicode_title).percent(50.0);
    g.render(&mut ctx);
}

#[test]
fn test_gauge_set_value_boundary() {
    let mut g = Gauge::new();

    // Test setting values at boundaries
    g.set_value(0.0);
    assert_eq!(g.get_value(), 0.0);

    g.set_value(0.5);
    assert_eq!(g.get_value(), 0.5);

    g.set_value(1.0);
    assert_eq!(g.get_value(), 1.0);

    // Test clamping
    g.set_value(-0.5);
    assert_eq!(g.get_value(), 0.0);

    g.set_value(1.5);
    assert_eq!(g.get_value(), 1.0);
}

#[test]
fn test_gauge_ratio_boundary() {
    // Test ratio() method at boundaries
    let g = Gauge::new().ratio(0.0);
    assert_eq!(g.get_value(), 0.0);

    let g = Gauge::new().ratio(0.5);
    assert_eq!(g.get_value(), 0.5);

    let g = Gauge::new().ratio(1.0);
    assert_eq!(g.get_value(), 1.0);

    // Test clamping
    let g = Gauge::new().ratio(-0.5);
    assert_eq!(g.get_value(), 0.0);

    let g = Gauge::new().ratio(1.5);
    assert_eq!(g.get_value(), 1.0);
}

#[test]
fn test_gauge_all_styles_at_boundaries() {
    let styles = [
        GaugeStyle::Bar,
        GaugeStyle::Battery,
        GaugeStyle::Thermometer,
        GaugeStyle::Arc,
        GaugeStyle::Circle,
        GaugeStyle::Vertical,
        GaugeStyle::Segments,
        GaugeStyle::Dots,
    ];

    for style in styles {
        // Test at 0%
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(0, 0, 20, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);
        Gauge::new().style(style).percent(0.0).render(&mut ctx);

        // Test at 100%
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(0, 0, 20, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);
        Gauge::new().style(style).percent(100.0).render(&mut ctx);

        // Test at 50%
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(0, 0, 20, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);
        Gauge::new().style(style).percent(50.0).render(&mut ctx);
    }
}

#[test]
fn test_gauge_with_label_at_boundaries() {
    // Test gauge with label at different values
    for value in [0.0, 25.0, 50.0, 75.0, 100.0] {
        let mut buffer = Buffer::new(30, 3);
        let area = Rect::new(0, 0, 30, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let g = Gauge::new().label(format!("{}%", value)).percent(value);
        g.render(&mut ctx);
    }
}

#[test]
fn test_gauge_empty_title() {
    // Empty title should be handled
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let g = Gauge::new().title("").percent(50.0);
    g.render(&mut ctx);
}

#[test]
fn test_gauge_whitespace_title() {
    // Whitespace-only title should be handled
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let g = Gauge::new().title("   ").percent(50.0);
    g.render(&mut ctx);
}

// =============================================================================
