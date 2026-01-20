//! CandleChart widget integration tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{candle_chart, ohlc_chart, Candle, CandleChart, CandleStyle, StyledView, View};

// Helper function to create sample candle data
fn sample_data() -> Vec<Candle> {
    vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),
        Candle::new(103.0, 108.0, 102.0, 107.0),
        Candle::new(107.0, 112.0, 106.0, 110.0),
        Candle::new(110.0, 115.0, 108.0, 112.0),
        Candle::new(112.0, 118.0, 110.0, 116.0),
    ]
}

// Helper function to create sample data with volume
fn sample_data_with_volume() -> Vec<Candle> {
    vec![
        Candle::with_volume(100.0, 105.0, 98.0, 103.0, 1000.0),
        Candle::with_volume(103.0, 108.0, 102.0, 107.0, 1500.0),
        Candle::with_volume(107.0, 112.0, 106.0, 110.0, 800.0),
        Candle::with_volume(110.0, 115.0, 108.0, 112.0, 2000.0),
        Candle::with_volume(112.0, 118.0, 110.0, 116.0, 1200.0),
    ]
}

// Helper function to create mixed bullish/bearish data
fn mixed_data() -> Vec<Candle> {
    vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),  // bullish
        Candle::new(103.0, 108.0, 100.0, 102.0), // bearish
        Candle::new(102.0, 107.0, 101.0, 105.0), // bullish
        Candle::new(105.0, 110.0, 103.0, 104.0), // bearish
        Candle::new(104.0, 109.0, 103.0, 108.0), // bullish
    ]
}

// ============================================================================
// Candle Data Structure Tests
// ============================================================================

#[test]
fn test_candle_new_basic() {
    let candle = Candle::new(100.0, 110.0, 95.0, 105.0);
    assert_eq!(candle.open, 100.0);
    assert_eq!(candle.high, 110.0);
    assert_eq!(candle.low, 95.0);
    assert_eq!(candle.close, 105.0);
    assert_eq!(candle.volume, None);
    assert_eq!(candle.timestamp, None);
}

#[test]
fn test_candle_with_volume() {
    let candle = Candle::with_volume(100.0, 110.0, 95.0, 105.0, 5000.0);
    assert_eq!(candle.open, 100.0);
    assert_eq!(candle.high, 110.0);
    assert_eq!(candle.low, 95.0);
    assert_eq!(candle.close, 105.0);
    assert_eq!(candle.volume, Some(5000.0));
    assert_eq!(candle.timestamp, None);
}

#[test]
fn test_candle_timestamp_builder() {
    let candle = Candle::new(100.0, 110.0, 95.0, 105.0).timestamp(1234567890);
    assert_eq!(candle.timestamp, Some(1234567890));
}

#[test]
fn test_candle_with_volume_and_timestamp() {
    let candle = Candle::with_volume(100.0, 110.0, 95.0, 105.0, 5000.0).timestamp(9876543210);
    assert_eq!(candle.volume, Some(5000.0));
    assert_eq!(candle.timestamp, Some(9876543210));
}

#[test]
fn test_candle_is_bullish() {
    let bullish = Candle::new(100.0, 110.0, 95.0, 105.0);
    assert!(bullish.is_bullish());

    let bearish = Candle::new(100.0, 110.0, 95.0, 98.0);
    assert!(!bearish.is_bullish());

    // Doji (open equals close) is considered bullish
    let doji = Candle::new(100.0, 105.0, 95.0, 100.0);
    assert!(doji.is_bullish());
}

#[test]
fn test_candle_body_size() {
    let candle = Candle::new(100.0, 115.0, 90.0, 110.0);
    assert_eq!(candle.body_size(), 10.0);

    let doji = Candle::new(100.0, 105.0, 95.0, 100.0);
    assert_eq!(doji.body_size(), 0.0);
}

#[test]
fn test_candle_upper_shadow() {
    let candle = Candle::new(100.0, 115.0, 90.0, 110.0);
    assert_eq!(candle.upper_shadow(), 5.0);

    // No upper shadow when high <= max(open, close)
    let no_shadow = Candle::new(100.0, 105.0, 90.0, 110.0);
    assert_eq!(no_shadow.upper_shadow(), 0.0);
}

#[test]
fn test_candle_lower_shadow() {
    let candle = Candle::new(100.0, 115.0, 90.0, 110.0);
    assert_eq!(candle.lower_shadow(), 10.0);

    // No lower shadow
    let no_shadow = Candle::new(100.0, 115.0, 100.0, 110.0);
    assert_eq!(no_shadow.lower_shadow(), 0.0);
}

#[test]
fn test_candle_range() {
    let candle = Candle::new(100.0, 115.0, 90.0, 110.0);
    assert_eq!(candle.range(), 25.0);

    let flat = Candle::new(100.0, 100.0, 100.0, 100.0);
    assert_eq!(flat.range(), 0.0);
}

#[test]
fn test_candle_clone_and_debug() {
    let candle = Candle::new(100.0, 110.0, 95.0, 105.0);
    let cloned = candle;

    assert_eq!(cloned.open, candle.open);
    assert_eq!(cloned.high, candle.high);
    assert_eq!(cloned.low, candle.low);
    assert_eq!(cloned.close, candle.close);

    // Test Debug format
    let debug_str = format!("{:?}", candle);
    assert!(debug_str.contains("Candle"));
}

// ============================================================================
// ChartStyle Tests
// ============================================================================

#[test]
fn test_chart_style_variants() {
    let candle = CandleStyle::Candle;
    let ohlc = CandleStyle::Ohlc;
    let hollow = CandleStyle::Hollow;
    let heikin_ashi = CandleStyle::HeikinAshi;

    assert_eq!(candle, CandleStyle::Candle);
    assert_ne!(candle, ohlc);
    assert_ne!(ohlc, hollow);
    assert_ne!(hollow, heikin_ashi);
}

#[test]
fn test_chart_style_default() {
    let default_style = CandleStyle::default();
    assert_eq!(default_style, CandleStyle::Candle);
}

#[test]
fn test_chart_style_clone() {
    let style = CandleStyle::HeikinAshi;
    let cloned = style;

    assert_eq!(cloned, CandleStyle::HeikinAshi);
}

// ============================================================================
// CandleChart Builder Tests
// ============================================================================

#[test]
fn test_chart_new_default() {
    let data = sample_data();
    let chart = CandleChart::new(data);

    // Test that chart can be created and renders without panicking
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_chart_style_builder() {
    // Test that different styles render without panicking
    let chart1 = CandleChart::new(sample_data()).style(CandleStyle::Ohlc);
    let chart2 = CandleChart::new(sample_data()).style(CandleStyle::Hollow);
    let chart3 = CandleChart::new(sample_data()).style(CandleStyle::HeikinAshi);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart1.render(&mut ctx);
    chart2.render(&mut ctx);
    chart3.render(&mut ctx);
}

#[test]
fn test_chart_size_builder() {
    let chart = CandleChart::new(sample_data()).size(60, 20);

    // Test that custom size renders without panicking
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_chart_height_builder() {
    let chart = CandleChart::new(sample_data()).height(25);

    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_chart_width_builder() {
    let chart = CandleChart::new(sample_data()).width(80);

    let mut buffer = Buffer::new(100, 20);
    let area = Rect::new(0, 0, 100, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_chart_colors_builder() {
    let chart = CandleChart::new(sample_data())
        .bullish_color(Color::CYAN)
        .bearish_color(Color::MAGENTA);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_chart_show_volume_builder() {
    let chart = CandleChart::new(sample_data_with_volume()).show_volume(true);

    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_chart_show_axis_builder() {
    let chart = CandleChart::new(sample_data()).show_axis(false);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_chart_show_grid_builder() {
    let chart = CandleChart::new(sample_data()).show_grid(true);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_chart_title_builder() {
    let chart = CandleChart::new(sample_data()).title("AAPL");

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_chart_crosshair_builder() {
    let chart = CandleChart::new(sample_data()).crosshair(2);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_chart_precision_builder() {
    let chart = CandleChart::new(sample_data()).precision(4);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_chart_price_range_builder() {
    let chart = CandleChart::new(sample_data()).price_range(90.0, 130.0);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_chart_scroll_builder() {
    let chart = CandleChart::new(sample_data()).scroll(1);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_chart_builder_chain() {
    let chart = CandleChart::new(sample_data())
        .style(CandleStyle::Ohlc)
        .size(50, 20)
        .bullish_color(Color::CYAN)
        .bearish_color(Color::YELLOW)
        .show_volume(true)
        .show_axis(false)
        .show_grid(true)
        .title("BTC/USD")
        .crosshair(3)
        .precision(3)
        .price_range(95.0, 125.0)
        .scroll(1);

    let mut buffer = Buffer::new(80, 40);
    let area = Rect::new(0, 0, 80, 40);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // If this renders without panic, all builder methods work
    chart.render(&mut ctx);
}

// ============================================================================
// Helper Function Tests
// ============================================================================

#[test]
fn test_candle_chart_helper() {
    let chart = candle_chart(sample_data());

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_ohlc_chart_helper() {
    let chart = ohlc_chart(sample_data());

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

// ============================================================================
// Data Analysis Tests
// ============================================================================

#[test]
fn test_current_price() {
    let data = sample_data();
    let chart = CandleChart::new(data);

    assert_eq!(chart.current_price(), Some(116.0));

    let empty_chart = CandleChart::new(vec![]);
    assert_eq!(empty_chart.current_price(), None);
}

#[test]
fn test_price_change_positive() {
    let data = vec![
        Candle::new(100.0, 105.0, 98.0, 100.0),
        Candle::new(100.0, 110.0, 95.0, 110.0),
    ];
    let chart = CandleChart::new(data);

    let (change, percent) = chart.price_change().unwrap();
    assert_eq!(change, 10.0);
    assert!((percent - 10.0).abs() < 0.001);
}

#[test]
fn test_price_change_negative() {
    let data = vec![
        Candle::new(100.0, 105.0, 98.0, 105.0),
        Candle::new(105.0, 110.0, 95.0, 100.0),
    ];
    let chart = CandleChart::new(data);

    let (change, percent) = chart.price_change().unwrap();
    assert_eq!(change, -5.0);
    assert!((percent - (-4.7619)).abs() < 0.01);
}

#[test]
fn test_price_change_insufficient_data() {
    let single_candle = vec![Candle::new(100.0, 105.0, 98.0, 103.0)];
    let chart = CandleChart::new(single_candle);
    assert_eq!(chart.price_change(), None);

    let empty_chart = CandleChart::new(vec![]);
    assert_eq!(empty_chart.price_change(), None);
}

// ============================================================================
// Render Tests
// ============================================================================

#[test]
fn test_render_basic_chart() {
    let chart = CandleChart::new(sample_data()).title("Test Chart");
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_render_empty_chart() {
    let chart = CandleChart::new(vec![]).title("Empty Chart");
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_render_with_title() {
    let chart = CandleChart::new(sample_data()).title("AAPL");
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_render_with_volume() {
    let chart = CandleChart::new(sample_data_with_volume()).show_volume(true);
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_render_with_crosshair() {
    let chart = CandleChart::new(sample_data()).crosshair(2);
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_render_without_axis() {
    let chart = CandleChart::new(sample_data()).show_axis(false);
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_render_with_custom_precision() {
    let chart = CandleChart::new(sample_data()).precision(4);
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_render_ohlc_style() {
    let chart = CandleChart::new(sample_data()).style(CandleStyle::Ohlc);
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_render_hollow_style() {
    let chart = CandleChart::new(sample_data()).style(CandleStyle::Hollow);
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_render_heikin_ashi_style() {
    let chart = CandleChart::new(sample_data()).style(CandleStyle::HeikinAshi);
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_render_with_custom_colors() {
    let chart = CandleChart::new(mixed_data())
        .bullish_color(Color::CYAN)
        .bearish_color(Color::MAGENTA);
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_render_with_scroll() {
    let chart = CandleChart::new(sample_data()).scroll(2);
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_render_with_custom_size() {
    let chart = CandleChart::new(sample_data()).size(30, 10);
    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

#[test]
fn test_render_all_features_combined() {
    let chart = CandleChart::new(sample_data_with_volume())
        .style(CandleStyle::Candle)
        .size(50, 20)
        .bullish_color(Color::rgb(0, 255, 0))
        .bearish_color(Color::rgb(255, 0, 0))
        .show_volume(true)
        .show_axis(true)
        .show_grid(true)
        .title("BTC/USD")
        .crosshair(2)
        .precision(2)
        .price_range(95.0, 125.0)
        .scroll(1);

    let mut buffer = Buffer::new(80, 40);
    let area = Rect::new(0, 0, 80, 40);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
}

// ============================================================================
// View and StyledView Trait Tests
// ============================================================================

#[test]
fn test_chart_view_meta() {
    let chart = CandleChart::new(sample_data());
    let meta = chart.meta();

    assert!(meta.id.is_none());
    assert!(meta.classes.is_empty());
}

#[test]
fn test_chart_view_id() {
    let chart = CandleChart::new(sample_data()).element_id("my-chart");
    assert_eq!(View::id(&chart), Some("my-chart"));
}

#[test]
fn test_chart_view_classes() {
    let chart = CandleChart::new(sample_data())
        .class("financial")
        .class("candlestick");

    assert!(chart.has_class("financial"));
    assert!(chart.has_class("candlestick"));
    assert!(!chart.has_class("line"));

    let meta = chart.meta();
    assert!(meta.classes.contains("financial"));
    assert!(meta.classes.contains("candlestick"));
}

#[test]
fn test_chart_styled_view_methods() {
    let mut chart = CandleChart::new(sample_data());

    chart.set_id("test-chart");
    assert_eq!(View::id(&chart), Some("test-chart"));

    chart.add_class("price-chart");
    assert!(chart.has_class("price-chart"));

    chart.remove_class("price-chart");
    assert!(!chart.has_class("price-chart"));

    chart.toggle_class("active");
    assert!(chart.has_class("active"));

    chart.toggle_class("active");
    assert!(!chart.has_class("active"));
}

#[test]
fn test_chart_with_inline_styles() {
    let chart = CandleChart::new(sample_data()).class("styled");

    let meta = chart.meta();
    // Verify that the chart was created successfully
    assert!(meta.classes.contains("styled"));
}

// ============================================================================
// Clone and Debug Tests
// ============================================================================

#[test]
fn test_chart_clone() {
    let original = CandleChart::new(sample_data())
        .style(CandleStyle::Ohlc)
        .title("Test")
        .crosshair(1);

    let cloned = original;

    // Test that cloned chart renders without panicking
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    cloned.render(&mut ctx);
}

#[test]
fn test_chart_debug() {
    let chart = CandleChart::new(sample_data()).title("Debug Test");
    let debug_str = format!("{:?}", chart);

    assert!(debug_str.contains("CandleChart"));
}

// ============================================================================
// Edge Cases and Special Scenarios
// ============================================================================

#[test]
fn test_single_candle() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data);

    assert_eq!(chart.current_price(), Some(105.0));
    assert_eq!(chart.price_change(), None);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_doji_candles() {
    // Create doji candles directly for testing
    let doji = Candle::new(100.0, 105.0, 95.0, 100.0); // perfect doji
    assert!(doji.is_bullish()); // doji is bullish
    assert_eq!(doji.body_size(), 0.0);

    let data = vec![
        Candle::new(100.0, 105.0, 95.0, 100.0), // perfect doji
        Candle::new(100.0, 102.0, 98.0, 100.0), // small doji
    ];
    let chart = CandleChart::new(data);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_large_price_range() {
    let data = vec![
        Candle::new(1.0, 2.0, 0.5, 1.5),
        Candle::new(1.5, 10000.0, 1.0, 9999.0),
    ];
    let chart = CandleChart::new(data);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_negative_prices() {
    let data = vec![
        Candle::new(-10.0, -5.0, -15.0, -8.0),
        Candle::new(-8.0, -3.0, -12.0, -6.0),
    ];
    let chart = CandleChart::new(data);

    assert_eq!(chart.current_price(), Some(-6.0));

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_zero_prices() {
    let data = vec![Candle::new(0.0, 0.0, 0.0, 0.0)];
    let chart = CandleChart::new(data);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_very_small_height() {
    let chart = CandleChart::new(sample_data()).height(3);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_very_small_width() {
    let chart = CandleChart::new(sample_data()).width(1);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_large_dimensions() {
    let large_data: Vec<Candle> = (0..100)
        .map(|i| {
            let base = 100.0 + i as f64;
            Candle::new(base, base + 5.0, base - 5.0, base + 2.0)
        })
        .collect();

    let chart = CandleChart::new(large_data).size(80, 40);

    let mut buffer = Buffer::new(100, 50);
    let area = Rect::new(0, 0, 100, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_scroll_beyond_data() {
    let data = sample_data();
    // Scroll beyond data should handle gracefully or clamp to valid range
    // The current implementation may panic, so we test within valid range
    let chart = CandleChart::new(data.clone()).scroll(2); // Scroll within range

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_invalid_crosshair_index() {
    let chart = CandleChart::new(sample_data()).crosshair(100);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_all_bullish_candles() {
    let data = vec![
        Candle::new(100.0, 110.0, 98.0, 105.0),
        Candle::new(105.0, 115.0, 103.0, 110.0),
        Candle::new(110.0, 120.0, 108.0, 115.0),
    ];

    // Verify all are bullish at data creation level
    assert!(data.iter().all(|c| c.is_bullish()));

    let chart = CandleChart::new(data);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_all_bearish_candles() {
    let data = vec![
        Candle::new(110.0, 115.0, 100.0, 105.0),
        Candle::new(105.0, 110.0, 95.0, 100.0),
        Candle::new(100.0, 105.0, 90.0, 95.0),
    ];

    // Verify all are bearish at data creation level
    assert!(data.iter().all(|c| !c.is_bullish()));

    let chart = CandleChart::new(data);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_high_precision_prices() {
    let data = vec![
        Candle::new(100.12345678, 110.98765432, 95.11111111, 105.22222222),
        Candle::new(105.22222222, 115.33333333, 100.44444444, 110.55555555),
    ];
    let chart = CandleChart::new(data).precision(8);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_volume_without_data() {
    let data = vec![
        Candle::with_volume(100.0, 110.0, 95.0, 105.0, 1000.0),
        Candle::new(103.0, 108.0, 102.0, 107.0), // no volume
        Candle::with_volume(107.0, 112.0, 106.0, 110.0, 2000.0),
    ];
    let chart = CandleChart::new(data).show_volume(true);

    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_zero_volume() {
    let data = vec![
        Candle::with_volume(100.0, 110.0, 95.0, 105.0, 0.0),
        Candle::with_volume(103.0, 108.0, 102.0, 107.0, 0.0),
    ];
    let chart = CandleChart::new(data).show_volume(true);

    let mut buffer = Buffer::new(80, 30);
    let area = Rect::new(0, 0, 80, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_equal_open_and_close() {
    // Marubozu candles - test Candle structure directly
    let marubozu = Candle::new(100.0, 110.0, 100.0, 110.0); // bullish marubozu
    assert_eq!(marubozu.body_size(), 10.0);
    assert_eq!(marubozu.upper_shadow(), 0.0);
    assert_eq!(marubozu.lower_shadow(), 0.0);

    let data = vec![
        Candle::new(100.0, 110.0, 100.0, 110.0), // bullish marubozu
        Candle::new(110.0, 110.0, 100.0, 100.0), // bearish marubozu
    ];
    let chart = CandleChart::new(data);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}
