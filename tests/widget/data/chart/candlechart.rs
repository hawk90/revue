//! CandleChart widget tests

use super::{render, render_rows};
use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::candlechart::ChartStyle;
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

    // When high < max(open, close), upper_shadow is negative (implementation behavior)
    let no_shadow = Candle::new(100.0, 105.0, 90.0, 110.0);
    assert_eq!(no_shadow.upper_shadow(), -5.0);
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
fn test_chart_show_grid_builder() {
    let chart = CandleChart::new(sample_data()).show_grid(true);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

// ============================================================================
// Helper Function Tests
// ============================================================================

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

#[test]
fn test_candlechart_header_shows_title_price_and_change() {
    let chart = CandleChart::new(vec![
        Candle::new(100.0, 105.0, 98.0, 100.0),
        Candle::new(100.0, 112.0, 99.0, 110.0),
    ])
    .title("AAPL");
    let rows = render_rows(&chart, 60, 20);
    assert!(rows[0].starts_with("AAPL"), "{:?}", rows[0]);
    assert!(rows[0].contains("110.00"), "{:?}", rows[0]);
    assert!(rows[0].contains("+10.00 (+10.00%)"), "{:?}", rows[0]);
}

#[test]
fn test_candlechart_empty_shows_placeholder() {
    let rows = render_rows(&CandleChart::new(vec![]).title("AAPL"), 40, 5);
    assert!(rows[0].starts_with("AAPL"));
    // The placeholder goes below the header (the stack decides the exact row)
    assert!(
        rows[1..].iter().any(|r| r.starts_with("No data")),
        "{rows:#?}"
    );
    assert!(!rows.iter().any(|r| r.contains('│')));
}

// ============================================================================
// Moved from src/widget/data/chart/candlechart/types.rs
// ============================================================================

#[test]
fn test_candle_body_size_bearish() {
    let candle = Candle::new(100.0, 110.0, 90.0, 95.0);
    assert_eq!(candle.body_size(), 5.0);
}

#[test]
fn test_candle_upper_shadow_zero() {
    let candle = Candle::new(100.0, 105.0, 95.0, 105.0);
    // high - max(open, close) = 105 - max(100, 105) = 105 - 105 = 0
    assert_eq!(candle.upper_shadow(), 0.0);
}

#[test]
fn test_candle_upper_shadow_bearish() {
    let candle = Candle::new(100.0, 120.0, 90.0, 95.0);
    // high - max(open, close) = 120 - 100 = 20
    assert_eq!(candle.upper_shadow(), 20.0);
}

#[test]
fn test_chart_style_debug() {
    let debug_str = format!("{:?}", ChartStyle::Candle);
    assert!(debug_str.contains("Candle"));
}

// ============================================================================
// Moved from src/widget/data/chart/candlechart/render.rs
// ============================================================================

#[test]
fn heikin_ashi_axis_covers_the_heikin_ashi_candles() {
    use revue::layout::Rect;
    use revue::render::Buffer;

    // A gap down: the second Heikin-Ashi candle opens at 195 (from the
    // first), far above the raw second candle's 100..110.
    let data = vec![
        Candle::new(195.0, 200.0, 190.0, 195.0),
        Candle::new(105.0, 110.0, 100.0, 105.0),
    ];
    let chart = CandleChart::new(data)
        .style(ChartStyle::HeikinAshi)
        .width(1)
        .scroll(1)
        .height(10)
        .show_volume(false);
    let mut buffer = Buffer::new(20, 12);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 12));
    chart.render(&mut ctx);
    let top: String = (0..9).map(|x| buffer.get(x, 0).unwrap().symbol).collect();
    let top: f64 = top.trim().parse().unwrap();
    assert!(
        top >= 195.0,
        "axis tops out at {top}, below the candle's 195"
    );
}

// ============================================================================
// Builders, checked on the screen
// ============================================================================

/// Glyphs a candle column is drawn with, in any style
const CANDLE_GLYPHS: [char; 7] = ['█', '│', '├', '┤', '┌', '└', '─'];

/// Rows of `chart` drawn without its price axis, so column `x` is candle `x`
fn plot_rows(chart: CandleChart, width: u16, height: u16) -> Vec<String> {
    render_rows(&chart.show_axis(false), width, height)
}

/// Columns holding any candle glyph
fn glyph_columns(rows: &[String]) -> usize {
    let width = rows.iter().map(|r| r.chars().count()).max().unwrap_or(0);
    (0..width)
        .filter(|&x| {
            rows.iter()
                .any(|r| r.chars().nth(x).is_some_and(|c| CANDLE_GLYPHS.contains(&c)))
        })
        .count()
}

/// Index of the last row holding anything
fn last_drawn_row(rows: &[String]) -> Option<usize> {
    rows.iter().rposition(|r| !r.trim().is_empty())
}

/// The first word of a row, such as its price label
fn first_word(row: &str) -> &str {
    row.split_whitespace().next().unwrap_or("")
}

#[test]
fn test_chart_new_default() {
    let data = sample_data();
    let chart = CandleChart::new(data);

    // 15 chart rows; the axis labels the top, middle and bottom row with
    // the price range padded by 5% (98..118 → 97..119)
    let rows = render_rows(&chart, 80, 20);
    assert_eq!(first_word(&rows[0]), "119.00");
    assert_eq!(first_word(&rows[7]), "108.00");
    assert_eq!(first_word(&rows[14]), "97.00");
    assert!(rows[15..].iter().all(|r| r.trim().is_empty()), "{rows:#?}");

    // One column per candle
    assert_eq!(glyph_columns(&plot_rows(chart, 80, 20)), 5);
}

#[test]
fn test_chart_size_builder() {
    let rows = plot_rows(CandleChart::new(sample_data()).size(3, 6), 80, 30);
    assert_eq!(glyph_columns(&rows), 3);
    // The lowest low (98) in the padded range 97.3..112.7 of 6 rows
    assert_eq!(last_drawn_row(&rows), Some(4), "{rows:#?}");
}

#[test]
fn test_chart_height_builder() {
    let rows = plot_rows(CandleChart::new(sample_data()).height(25), 80, 30);
    // The lowest low (98) in the padded range 97..119 of 25 rows
    assert_eq!(last_drawn_row(&rows), Some(22), "{rows:#?}");
}

#[test]
fn test_chart_width_builder() {
    // Wider than the data: every candle
    let rows = plot_rows(CandleChart::new(sample_data()).width(80), 100, 20);
    assert_eq!(glyph_columns(&rows), 5);

    let rows = plot_rows(CandleChart::new(sample_data()).width(2), 100, 20);
    assert_eq!(glyph_columns(&rows), 2);
}

#[test]
fn test_render_with_custom_colors() {
    let default = CandleChart::new(mixed_data()).show_axis(false);
    let custom = CandleChart::new(mixed_data())
        .bullish_color(Color::CYAN)
        .bearish_color(Color::MAGENTA)
        .show_axis(false);
    for (chart, up, down) in [
        (default, Color::GREEN, Color::RED),
        (custom, Color::CYAN, Color::MAGENTA),
    ] {
        let buffer = render(&chart, 80, 30);
        // mixed_data alternates bullish and bearish candles
        for x in 0..5u16 {
            let want = if x % 2 == 0 { up } else { down };
            let bodies: Vec<_> = (0..15)
                .filter_map(|y| buffer.get(x, y))
                .filter(|c| c.symbol == '█')
                .collect();
            assert!(!bodies.is_empty(), "column {x}");
            assert!(bodies.iter().all(|c| c.fg == Some(want)), "column {x}");
        }
    }
}

#[test]
fn test_chart_show_volume_builder() {
    // Off by default: nothing below the 15 chart rows
    let rows = render_rows(&CandleChart::new(sample_data_with_volume()), 80, 30);
    assert!(rows[15..].iter().all(|r| r.trim().is_empty()), "{rows:#?}");

    // A separator, then 4 rows of bars; the largest volume fills all 4
    let chart = CandleChart::new(sample_data_with_volume()).show_volume(true);
    let rows = render_rows(&chart, 80, 30);
    assert_eq!(rows[15].trim(), "─────");
    assert!(rows[16..20].iter().all(|r| r.contains('█')), "{rows:#?}");
    assert!(rows[20..].iter().all(|r| r.trim().is_empty()), "{rows:#?}");
}

#[test]
fn test_chart_show_axis_builder() {
    let chart = CandleChart::new(sample_data()).show_axis(false);
    let rows = render_rows(&chart, 80, 20);
    assert!(
        rows.iter()
            .all(|r| !r.contains(|c: char| c.is_ascii_digit())),
        "{rows:#?}"
    );
    // The first candle starts at the left edge
    assert!(
        rows.iter().any(|r| r.starts_with(CANDLE_GLYPHS)),
        "{rows:#?}"
    );
}

#[test]
fn test_chart_title_builder() {
    let chart = CandleChart::new(sample_data()).title("AAPL");
    let rows = render_rows(&chart, 80, 20);
    // Last close 116, up 4 from 112
    assert_eq!(rows[0].trim_end(), "AAPL  116.00  +4.00 (+3.57%)");

    let chart = CandleChart::new(sample_data()).title(String::from("GOOGL"));
    assert!(render_rows(&chart, 80, 20)[0].starts_with("GOOGL  116.00"));
}

#[test]
fn test_chart_crosshair_builder() {
    let chart = CandleChart::new(sample_data()).crosshair(2);
    let rows = render_rows(&chart, 80, 20);
    // Below the 15 chart rows
    assert_eq!(rows[15].trim_end(), "O:107.00 H:112.00 L:106.00 C:110.00");

    let chart = CandleChart::new(sample_data_with_volume()).crosshair(2);
    let rows = render_rows(&chart, 80, 20);
    assert_eq!(
        rows[15].trim_end(),
        "O:107.00 H:112.00 L:106.00 C:110.00 V:800"
    );
}

#[test]
fn test_invalid_crosshair_index() {
    let chart = CandleChart::new(sample_data()).crosshair(100);
    let rows = render_rows(&chart, 80, 20);
    assert!(!rows.iter().any(|r| r.contains("O:")), "{rows:#?}");
}

#[test]
fn test_chart_precision_builder() {
    let chart = CandleChart::new(sample_data()).precision(4);
    let rows = render_rows(&chart, 80, 20);
    assert_eq!(first_word(&rows[0]), "119.0000");
}

#[test]
fn test_high_precision_prices() {
    let data = vec![
        Candle::new(100.12345678, 110.98765432, 95.11111111, 105.22222222),
        Candle::new(105.22222222, 115.33333333, 100.44444444, 110.55555555),
    ];
    let chart = CandleChart::new(data).precision(8).title("T");
    let rows = render_rows(&chart, 80, 20);
    assert!(
        rows[0].starts_with("T  110.55555555  +5.33333333"),
        "{:?}",
        rows[0]
    );
    let decimals = first_word(&rows[1]).split('.').nth(1).unwrap_or("");
    assert_eq!(decimals.len(), 8, "{:?}", rows[1]);
}

#[test]
fn test_chart_price_range_builder() {
    let chart = CandleChart::new(sample_data()).price_range(90.0, 130.0);
    let rows = render_rows(&chart, 80, 20);
    // The given range, padded by 5% of it
    assert_eq!(first_word(&rows[0]), "132.00");
    assert_eq!(first_word(&rows[7]), "110.00");
    assert_eq!(first_word(&rows[14]), "88.00");
}

#[test]
fn test_chart_scroll_builder() {
    // The first candle drawn (crosshair 0) is the second of the data
    let chart = CandleChart::new(sample_data()).scroll(1).crosshair(0);
    let rows = render_rows(&chart, 80, 20);
    assert_eq!(rows[15].trim_end(), "O:103.00 H:108.00 L:102.00 C:107.00");
    assert_eq!(glyph_columns(&plot_rows(chart, 80, 20)), 4);

    let rows = plot_rows(CandleChart::new(sample_data()).width(2).scroll(1), 80, 20);
    assert_eq!(glyph_columns(&rows), 2);
}

#[test]
fn test_scroll_beyond_data() {
    for style in [CandleStyle::Candle, CandleStyle::HeikinAshi] {
        for offset in [5, 10, usize::MAX] {
            let chart = CandleChart::new(sample_data()).style(style).scroll(offset);
            let rows = render_rows(&chart, 80, 20);
            assert!(rows[0].starts_with("No data"), "{style:?} {offset}");
        }
    }
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

    let rows = render_rows(&chart, 80, 40);
    assert!(
        rows[0].starts_with("BTC/USD  116.000  +4.000 (+3.57%)"),
        "{:?}",
        rows[0]
    );
    let plot = &rows[1..21];
    assert_eq!(glyph_columns(plot), 4);
    assert!(plot.iter().any(|r| r.contains('├') && r.contains('┤')));
    assert!(!plot.iter().any(|r| r.contains('█')));
    // No volume in the data, so no volume bars: the crosshair line follows
    assert_eq!(
        rows[21].trim_end(),
        "O:112.000 H:118.000 L:110.000 C:116.000"
    );
}

#[test]
fn test_candle_chart_helper() {
    assert_eq!(
        render_rows(&candle_chart(sample_data()), 80, 20),
        render_rows(&CandleChart::new(sample_data()), 80, 20)
    );
}

#[test]
fn test_ohlc_chart_helper() {
    assert_eq!(
        render_rows(&ohlc_chart(sample_data()), 80, 20),
        render_rows(
            &CandleChart::new(sample_data()).style(CandleStyle::Ohlc),
            80,
            20
        )
    );
}

#[test]
fn test_chart_clone() {
    let original = CandleChart::new(sample_data())
        .style(CandleStyle::Ohlc)
        .title("Test")
        .crosshair(1);

    let cloned = original.clone();
    assert_eq!(render_rows(&cloned, 80, 20), render_rows(&original, 80, 20));
}

// ============================================================================
// Styles
// ============================================================================

#[test]
fn test_render_ohlc_style() {
    let rows = plot_rows(
        CandleChart::new(mixed_data()).style(CandleStyle::Ohlc),
        80,
        30,
    );
    // Open tick on the left, close tick on the right, no bodies
    assert!(rows.iter().any(|r| r.contains('├')), "{rows:#?}");
    assert!(rows.iter().any(|r| r.contains('┤')), "{rows:#?}");
    assert!(!rows.iter().any(|r| r.contains('█')), "{rows:#?}");
    assert_eq!(glyph_columns(&rows), 5);
}

#[test]
fn test_render_hollow_style() {
    // Bullish bodies are outlines, bearish ones are filled
    let rows = plot_rows(
        CandleChart::new(sample_data()).style(CandleStyle::Hollow),
        80,
        30,
    );
    assert!(rows.iter().any(|r| r.contains('┌')), "{rows:#?}");
    assert!(rows.iter().any(|r| r.contains('└')), "{rows:#?}");
    assert!(!rows.iter().any(|r| r.contains('█')), "{rows:#?}");

    let rows = plot_rows(
        CandleChart::new(mixed_data()).style(CandleStyle::Hollow),
        80,
        30,
    );
    assert!(rows.iter().any(|r| r.contains('█')), "{rows:#?}");
}

#[test]
fn test_render_heikin_ashi_style() {
    let data = vec![
        Candle::new(100.0, 110.0, 95.0, 105.0),
        Candle::new(105.0, 115.0, 100.0, 110.0),
    ];
    // The first Heikin-Ashi candle closes at the average of the first
    // candle, and the next opens at the midpoint of the first's body.
    for (index, info) in [
        (0, "O:102.50 H:110.00 L:95.00 C:102.50"),
        (1, "O:102.50 H:115.00 L:100.00 C:107.50"),
    ] {
        let chart = CandleChart::new(data.clone())
            .style(CandleStyle::HeikinAshi)
            .crosshair(index);
        assert_eq!(render_rows(&chart, 80, 20)[15].trim_end(), info);
    }
}

// ============================================================================
// Edges
// ============================================================================

#[test]
fn test_render_zero_height() {
    let chart = CandleChart::new(sample_data()).height(0);
    assert!(render_rows(&chart, 40, 20)
        .iter()
        .all(|r| r.trim().is_empty()));
}

#[test]
fn test_large_price_range() {
    let data = vec![
        Candle::new(1.0, 2.0, 0.5, 1.5),
        Candle::new(1.5, 10000.0, 1.0, 9999.0),
    ];
    let chart = CandleChart::new(data);

    // The small candle still takes a cell at the bottom
    let rows = plot_rows(chart, 80, 20);
    assert_eq!(glyph_columns(&rows), 2);
}

#[test]
fn test_zero_prices() {
    let data = vec![Candle::new(0.0, 0.0, 0.0, 0.0)];
    let chart = CandleChart::new(data);

    // A flat range draws the candle in the middle row
    let rows = render_rows(&chart, 80, 20);
    assert_eq!(super::count_char(&render(&chart, 80, 20), '█'), 1);
    assert!(rows[7].contains('█'), "{rows:#?}");
}

#[test]
fn test_very_small_height() {
    let rows = plot_rows(CandleChart::new(sample_data()).height(3), 80, 20);
    assert_eq!(glyph_columns(&rows), 5);
    assert!(rows[3..].iter().all(|r| r.trim().is_empty()), "{rows:#?}");
}

#[test]
fn test_very_small_width() {
    let rows = plot_rows(CandleChart::new(sample_data()).width(1), 80, 20);
    assert_eq!(glyph_columns(&rows), 1);
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

    let rows = plot_rows(chart, 100, 50);
    assert_eq!(glyph_columns(&rows), 80);
    assert!(rows[40..].iter().all(|r| r.trim().is_empty()), "{rows:#?}");
}

#[test]
fn test_volume_without_data() {
    let data = vec![
        Candle::with_volume(100.0, 110.0, 95.0, 105.0, 1000.0),
        Candle::new(103.0, 108.0, 102.0, 107.0), // no volume
        Candle::with_volume(107.0, 112.0, 106.0, 110.0, 2000.0),
    ];
    let chart = CandleChart::new(data).show_volume(true);

    // Volume bars sit right of the 10-column axis: the candle without
    // volume has none, the others are half and full height
    let rows = render_rows(&chart, 80, 30);
    let bars: Vec<String> = rows[16..20]
        .iter()
        .map(|r| r.chars().skip(10).take(3).collect())
        .collect();
    assert_eq!(bars, ["  █", "  █", "█ █", "█ █"]);
}

#[test]
fn test_zero_volume() {
    let data = vec![
        Candle::with_volume(100.0, 110.0, 95.0, 105.0, 0.0),
        Candle::with_volume(103.0, 108.0, 102.0, 107.0, 0.0),
    ];
    let chart = CandleChart::new(data).show_volume(true);

    // No volume to draw: no separator and no bars
    let rows = render_rows(&chart, 80, 30);
    assert!(rows[15..].iter().all(|r| r.trim().is_empty()), "{rows:#?}");
}

// ============================================================================
// Areas and placement
// ============================================================================

#[test]
fn test_candlechart_zero_area_draws_nothing() {
    let chart = CandleChart::new(sample_data_with_volume())
        .title("AAPL")
        .show_volume(true)
        .crosshair(1);
    super::assert_zero_areas_draw_nothing(&chart);
}

#[test]
fn test_candlechart_draws_the_same_when_offset() {
    let full = || {
        CandleChart::new(sample_data_with_volume())
            .title("AAPL")
            .show_volume(true)
            .crosshair(1)
            .height(8)
    };
    super::assert_same_when_offset(full(), 30, 16);
    super::assert_same_when_offset(full().style(CandleStyle::Ohlc).show_axis(false), 30, 16);
    super::assert_same_when_offset(full().style(CandleStyle::HeikinAshi), 30, 16);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Without the axis, any candles take one column each (up to the
    /// chart width) and stay inside the chart height.
    #[test]
    fn test_candles_take_one_column_each(
        prices in prop::collection::vec((1.0f64..1e6, 0.0f64..1.0, 0.0f64..1.0, 0.0f64..1.0), 1..30),
        width in 1usize..40,
        height in 2u16..20,
        style in 0usize..4,
    ) {
        // low <= open, close <= high
        let data: Vec<Candle> = prices
            .iter()
            .map(|&(low, a, b, span)| {
                let high = low * (1.0 + span);
                let at = |t: f64| low + (high - low) * t;
                Candle::new(at(a), high, low, at(b))
            })
            .collect();
        let style = [CandleStyle::Candle, CandleStyle::Ohlc, CandleStyle::Hollow, CandleStyle::HeikinAshi][style];
        let chart = CandleChart::new(data.clone()).style(style).size(width, height);
        let rows = plot_rows(chart, 50, 25);
        prop_assert_eq!(glyph_columns(&rows), data.len().min(width));
        let last = last_drawn_row(&rows).unwrap_or(0);
        prop_assert!(last < height as usize, "row {} of {}", last, height);
    }
}
