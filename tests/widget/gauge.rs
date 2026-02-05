//! Gauge widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{battery, gauge, percentage, Gauge, GaugeStyle, LabelPosition, View};

// ==================== Constructor Tests ====================

#[test]
fn test_gauge_new() {
    let g = Gauge::new();
    assert_eq!(g.get_value(), 0.0);
}

#[test]
fn test_gauge_default() {
    let g = Gauge::default();
    assert_eq!(g.get_value(), 0.0);
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

// ==================== Value Tests ====================

#[test]
fn test_gauge_set_get_value() {
    let mut g = Gauge::new();
    g.set_value(0.8);
    assert_eq!(g.get_value(), 0.8);
}

#[test]
fn test_gauge_value() {
    let g = Gauge::new().value(0.5);
    assert_eq!(g.get_value(), 0.5);
}

#[test]
fn test_gauge_value_clamp() {
    // Values above 1.0 are clamped
    let g = Gauge::new().value(1.5);
    assert_eq!(g.get_value(), 1.0);

    // Values below 0.0 are clamped
    let g = Gauge::new().value(-0.5);
    assert_eq!(g.get_value(), 0.0);
}

#[test]
fn test_gauge_percent() {
    let g = Gauge::new().percent(50.0);
    assert_eq!(g.get_value(), 0.5);
}

#[test]
fn test_gauge_percent_clamp() {
    // Percent above 100 is clamped
    let g = Gauge::new().percent(150.0);
    assert_eq!(g.get_value(), 1.0);

    // Percent below 0 is clamped
    let g = Gauge::new().percent(-50.0);
    assert_eq!(g.get_value(), 0.0);
}

#[test]
fn test_gauge_value_range() {
    let g = Gauge::new().value_range(50.0, 0.0, 100.0);
    assert!((g.get_value() - 0.5).abs() < 0.001);
}

#[test]
fn test_gauge_value_range_bounds() {
    // At min
    let g = Gauge::new().value_range(0.0, 0.0, 100.0);
    assert_eq!(g.get_value(), 0.0);

    // At max
    let g = Gauge::new().value_range(100.0, 0.0, 100.0);
    assert_eq!(g.get_value(), 1.0);

    // Below min (clamped)
    let g = Gauge::new().value_range(-10.0, 0.0, 100.0);
    assert_eq!(g.get_value(), 0.0);

    // Above max (clamped)
    let g = Gauge::new().value_range(150.0, 0.0, 100.0);
    assert_eq!(g.get_value(), 1.0);
}

#[test]
fn test_gauge_value_range_swapped() {
    // If min > max, they should be swapped
    let g = Gauge::new().value_range(50.0, 100.0, 0.0);
    assert!((g.get_value() - 0.5).abs() < 0.001);
}

// ==================== Style Tests ====================

#[test]
fn test_gauge_style_default() {
    let style = GaugeStyle::default();
    assert_eq!(style, GaugeStyle::Bar);
}

#[test]
fn test_gauge_style_all_variants() {
    let _ = GaugeStyle::Bar;
    let _ = GaugeStyle::Battery;
    let _ = GaugeStyle::Thermometer;
    let _ = GaugeStyle::Arc;
    let _ = GaugeStyle::Circle;
    let _ = GaugeStyle::Vertical;
    let _ = GaugeStyle::Segments;
    let _ = GaugeStyle::Dots;
}

#[test]
fn test_gauge_style_builder() {
    let _g = Gauge::new().style(GaugeStyle::Battery);
    // Private field - just verify it compiles
}

// ==================== LabelPosition Tests ====================

#[test]
fn test_label_position_default() {
    let pos = LabelPosition::default();
    assert_eq!(pos, LabelPosition::Inside);
}

#[test]
fn test_label_position_all_variants() {
    let _ = LabelPosition::None;
    let _ = LabelPosition::Inside;
    let _ = LabelPosition::Left;
    let _ = LabelPosition::Right;
    let _ = LabelPosition::Above;
    let _ = LabelPosition::Below;
}

// ==================== Builder Tests ====================

#[test]
fn test_gauge_width() {
    let _g = Gauge::new().width(15);
    // Private field - just verify it compiles
}

#[test]
fn test_gauge_width_minimum() {
    let _g = Gauge::new().width(2);
    // Width should be clamped to min 4
}

#[test]
fn test_gauge_height() {
    let _g = Gauge::new().height(10);
    // Private field - just verify it compiles
}

#[test]
fn test_gauge_height_minimum() {
    let _g = Gauge::new().height(0);
    // Height should be clamped to min 1
}

#[test]
fn test_gauge_label() {
    let _g = Gauge::new().label("Custom Label");
    // Private field - just verify it compiles
}

#[test]
fn test_gauge_label_position() {
    let _g = Gauge::new().label_position(LabelPosition::Right);
    // Private field - just verify it compiles
}

#[test]
fn test_gauge_show_percent() {
    let _g = Gauge::new().show_percent(false);
    // Private field - just verify it compiles
}

#[test]
fn test_gauge_fill_color() {
    let _g = Gauge::new().fill_color(Color::CYAN);
    // Private field - just verify it compiles
}

#[test]
fn test_gauge_empty_color() {
    let _g = Gauge::new().empty_color(Color::rgb(80, 80, 80));
    // Private field - just verify it compiles
}

#[test]
fn test_gauge_border() {
    let _g = Gauge::new().border(Color::WHITE);
    // Private field - just verify it compiles
}

#[test]
fn test_gauge_thresholds() {
    let _g = Gauge::new().thresholds(0.7, 0.9);
    // Private fields - just verify it compiles
}

#[test]
fn test_gauge_warning_color() {
    let _g = Gauge::new().warning_color(Color::YELLOW);
    // Private field - just verify it compiles
}

#[test]
fn test_gauge_critical_color() {
    let _g = Gauge::new().critical_color(Color::RED);
    // Private field - just verify it compiles
}

#[test]
fn test_gauge_segments() {
    let _g = Gauge::new().segments(15);
    // Private field - just verify it compiles
}

#[test]
fn test_gauge_segments_minimum() {
    let _g = Gauge::new().segments(1);
    // Segments should be clamped to min 2
}

#[test]
fn test_gauge_title() {
    let _g = Gauge::new().title("CPU Usage");
    // Private field - just verify it compiles
}

#[test]
fn test_gauge_builder_chain() {
    let _g = Gauge::new()
        .value(0.75)
        .style(GaugeStyle::Battery)
        .width(20)
        .height(5)
        .label("75%")
        .label_position(LabelPosition::Right)
        .show_percent(false)
        .fill_color(Color::GREEN)
        .empty_color(Color::rgb(60, 60, 60))
        .border(Color::WHITE)
        .thresholds(0.5, 0.2)
        .warning_color(Color::YELLOW)
        .critical_color(Color::RED)
        .segments(12)
        .title("Battery");
    // Just verify it compiles
}

// ==================== Rendering Tests ====================

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
fn test_gauge_with_label() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let g = Gauge::new().label(format!("{}%", 75));
    g.render(&mut ctx);
    // Should render with custom label
}

// ==================== Edge Case Tests ====================

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
fn test_gauge_value_boundary() {
    // Test value() method at boundaries (0.0 - 1.0 range)
    let g = Gauge::new().value(0.0);
    assert_eq!(g.get_value(), 0.0);

    let g = Gauge::new().value(0.5);
    assert_eq!(g.get_value(), 0.5);

    let g = Gauge::new().value(1.0);
    assert_eq!(g.get_value(), 1.0);

    // Test clamping
    let g = Gauge::new().value(-0.5);
    assert_eq!(g.get_value(), 0.0);

    let g = Gauge::new().value(1.5);
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

#[test]
fn test_gauge_thresholds_swapped() {
    // If warning >= critical, they should be swapped
    let _g = Gauge::new().thresholds(0.9, 0.5);
    // Just verify it compiles - thresholds are swapped internally
}

// =============================================================================
// CSS Integration Tests
// =============================================================================

#[test]
fn test_gauge_element_id() {
    let g = Gauge::new().element_id("cpu-gauge");
    assert_eq!(View::id(&g), Some("cpu-gauge"));
}

#[test]
fn test_gauge_single_class() {
    let g = Gauge::new().class("metric");
    assert!(View::classes(&g).contains(&"metric".to_string()));
}

#[test]
fn test_gauge_multiple_classes() {
    let g = Gauge::new().class("primary").class("gauge").class("cpu");
    let classes = View::classes(&g);
    assert!(classes.contains(&"primary".to_string()));
    assert!(classes.contains(&"gauge".to_string()));
    assert!(classes.contains(&"cpu".to_string()));
}

#[test]
fn test_gauge_classes_vec() {
    let g = Gauge::new().classes(vec!["metric", "widget", "colored"]);
    let classes = View::classes(&g);
    assert!(classes.contains(&"metric".to_string()));
    assert!(classes.contains(&"widget".to_string()));
    assert!(classes.contains(&"colored".to_string()));
}

#[test]
fn test_gauge_meta() {
    let g = Gauge::new()
        .element_id("test-gauge")
        .class("primary")
        .title("Test");

    let meta = g.meta();
    assert_eq!(meta.id, Some("test-gauge".to_string()));
    assert!(meta.classes.contains("primary"));
    assert_eq!(meta.widget_type, "Gauge");
}

#[test]
fn test_gauge_view_children() {
    let g = Gauge::new();
    assert!(View::children(&g).is_empty());
}

#[test]
fn test_gauge_view_widget_type() {
    let g = Gauge::new();
    assert_eq!(g.widget_type(), "Gauge");
}

// =============================================================================
// Color Tests
// =============================================================================

#[test]
fn test_gauge_fill_color_variants() {
    let colors = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
        Color::CYAN,
        Color::MAGENTA,
        Color::WHITE,
        Color::BLACK,
    ];

    for color in colors {
        let g = Gauge::new().fill_color(color);
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(0, 0, 20, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);
        g.render(&mut ctx);
    }
}

#[test]
fn test_gauge_empty_color_variants() {
    let colors = [
        Color::rgb(50, 50, 50),
        Color::rgb(100, 100, 100),
        Color::rgb(150, 150, 150),
    ];

    for color in colors {
        let g = Gauge::new().empty_color(color);
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(0, 0, 20, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);
        g.render(&mut ctx);
    }
}

// =============================================================================
// Label Edge Cases
// =============================================================================

#[test]
fn test_gauge_label_emoji() {
    let g = Gauge::new().label("🔥 100%");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    g.render(&mut ctx);
}

#[test]
fn test_gauge_label_unicode() {
    let g = Gauge::new().label("進行中 50%");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    g.render(&mut ctx);
}

#[test]
fn test_gauge_label_rtl() {
    let g = Gauge::new().label("مرحبا"); // Arabic
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    g.render(&mut ctx);
}

#[test]
fn test_gauge_label_special_chars() {
    let g = Gauge::new().label("Test@#$%^&*()");
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    g.render(&mut ctx);
}

#[test]
fn test_gauge_label_with_newline() {
    let g = Gauge::new().label("Line1\nLine2");
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    g.render(&mut ctx);
}

// =============================================================================
// GaugeStyle Enum Tests
// =============================================================================

#[test]
fn test_gauge_style_eq() {
    assert_eq!(GaugeStyle::Bar, GaugeStyle::Bar);
    assert_eq!(GaugeStyle::Battery, GaugeStyle::Battery);
}

#[test]
fn test_gauge_style_ne() {
    assert_ne!(GaugeStyle::Bar, GaugeStyle::Battery);
    assert_ne!(GaugeStyle::Circle, GaugeStyle::Vertical);
}

#[test]
fn test_gauge_all_styles_unique() {
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

    for i in 0..styles.len() {
        for j in (i + 1)..styles.len() {
            assert_ne!(styles[i], styles[j]);
        }
    }
}

// =============================================================================
// LabelPosition Enum Tests
// =============================================================================

#[test]
fn test_label_position_eq() {
    assert_eq!(LabelPosition::None, LabelPosition::None);
    assert_eq!(LabelPosition::Inside, LabelPosition::Inside);
}

#[test]
fn test_label_position_ne() {
    assert_ne!(LabelPosition::None, LabelPosition::Inside);
    assert_ne!(LabelPosition::Left, LabelPosition::Right);
}

#[test]
fn test_all_label_positions_unique() {
    let positions = [
        LabelPosition::None,
        LabelPosition::Inside,
        LabelPosition::Left,
        LabelPosition::Right,
        LabelPosition::Above,
        LabelPosition::Below,
    ];

    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            assert_ne!(positions[i], positions[j]);
        }
    }
}

// =============================================================================
// Render with Offset Tests
// =============================================================================

#[test]
fn test_gauge_render_with_offset() {
    let g = Gauge::new().percent(50.0);
    let mut buffer = Buffer::new(50, 10);
    let area = Rect::new(10, 5, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    g.render(&mut ctx);
    // Should render at offset
}

#[test]
fn test_gauge_render_multiple_positions() {
    let g = Gauge::new().percent(75.0);
    let mut buffer = Buffer::new(50, 10);

    let positions = [
        Rect::new(0, 0, 20, 3),
        Rect::new(5, 3, 20, 3),
        Rect::new(15, 6, 20, 3),
    ];

    for area in positions {
        buffer.clear();
        let mut ctx = RenderContext::new(&mut buffer, area);
        g.render(&mut ctx);
    }
}

// =============================================================================
// Multiple Render Calls
// =============================================================================

#[test]
fn test_gauge_multiple_renders() {
    let g = Gauge::new().percent(50.0);
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);

    for _ in 0..5 {
        buffer.clear();
        let mut ctx = RenderContext::new(&mut buffer, area);
        g.render(&mut ctx);
    }
}

#[test]
fn test_gauge_render_after_value_change() {
    let mut g = Gauge::new();
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);

    // Render at 0%
    {
        let mut ctx = RenderContext::new(&mut buffer, area);
        g.render(&mut ctx);
    }

    g.set_value(0.5);
    buffer.clear();

    // Render at 50%
    {
        let mut ctx = RenderContext::new(&mut buffer, area);
        g.render(&mut ctx);
    }

    g.set_value(1.0);
    buffer.clear();

    // Render at 100%
    {
        let mut ctx = RenderContext::new(&mut buffer, area);
        g.render(&mut ctx);
    }
}

// =============================================================================
// Special Value Tests
// =============================================================================

#[test]
fn test_gauge_nan_value() {
    let g = Gauge::new().value(f64::NAN);
    // Gauge stores NaN as-is
    let value = g.get_value();
    assert!(value.is_nan());
}

#[test]
fn test_gauge_infinity_value() {
    let g1 = Gauge::new().value(f64::INFINITY);
    // Infinity should be clamped to 1.0
    assert_eq!(g1.get_value(), 1.0);

    let g2 = Gauge::new().value(f64::NEG_INFINITY);
    // Negative infinity should be clamped to 0.0
    assert_eq!(g2.get_value(), 0.0);
}

#[test]
fn test_gauge_very_small_positive() {
    let g = Gauge::new().value(0.0001);
    assert!((g.get_value() - 0.0001).abs() < 0.00001);
}

#[test]
fn test_gauge_very_close_to_one() {
    let g = Gauge::new().value(0.9999);
    assert!((g.get_value() - 0.9999).abs() < 0.00001);
}

// =============================================================================
// Combination Tests
// =============================================================================

#[test]
fn test_gauge_all_colors() {
    let g = Gauge::new()
        .fill_color(Color::GREEN)
        .empty_color(Color::rgb(50, 50, 50))
        .border(Color::YELLOW);

    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 20, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    g.render(&mut ctx);
}

#[test]
fn test_gauge_with_all_options() {
    let g = Gauge::new()
        .value(0.67)
        .style(GaugeStyle::Segments)
        .width(20)
        .height(3)
        .label("67%")
        .label_position(LabelPosition::Right)
        .show_percent(false)
        .fill_color(Color::GREEN)
        .empty_color(Color::rgb(60, 60, 60))
        .border(Color::WHITE)
        .thresholds(0.5, 0.2)
        .warning_color(Color::YELLOW)
        .critical_color(Color::RED)
        .segments(10)
        .title("Progress")
        .element_id("test-gauge")
        .class("primary");

    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    g.render(&mut ctx);

    assert_eq!(View::id(&g), Some("test-gauge"));
    assert!(View::classes(&g).contains(&"primary".to_string()));
}

#[test]
fn test_gauge_battery_style_all_levels() {
    let levels = [0.0, 0.25, 0.5, 0.75, 1.0];

    for level in levels {
        let g = Gauge::new().style(GaugeStyle::Battery).value(level);
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(0, 0, 20, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);
        g.render(&mut ctx);
    }
}

// =============================================================================
// Width and Height Edge Cases
// =============================================================================

#[test]
fn test_gauge_very_large_width() {
    let g = Gauge::new().width(1000);
    let mut buffer = Buffer::new(100, 3);
    let area = Rect::new(0, 0, 100, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    g.render(&mut ctx);
}

#[test]
fn test_gauge_very_large_height() {
    let g = Gauge::new().height(100);
    let mut buffer = Buffer::new(20, 100);
    let area = Rect::new(0, 0, 20, 100);
    let mut ctx = RenderContext::new(&mut buffer, area);
    g.render(&mut ctx);
}

#[test]
fn test_gauge_single_pixel_width() {
    let g = Gauge::new().width(1);
    let mut buffer = Buffer::new(5, 3);
    let area = Rect::new(0, 0, 5, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    g.render(&mut ctx);
}

#[test]
fn test_gauge_single_pixel_height() {
    let g = Gauge::new().height(1);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    g.render(&mut ctx);
}

// =============================================================================
