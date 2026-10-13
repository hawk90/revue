//! Candle and ChartStyle tests: fields, metrics, bullish and doji candles

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::chart::candlechart::ChartStyle;
use revue::widget::traits::RenderContext;
use revue::widget::{Candle, CandleChart, CandleStyle, View};

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
