//! Tests for candle chart public API
//!
//! Extracted from src/widget/data/chart/candlechart.rs

use revue::widget::data::chart::{
    CandleChart, Candle, ChartStyle, candle_chart, ohlc_chart,
};
use revue::style::Color;

#[test]
fn test_candle_new() {
    let candle = Candle::new(100.0, 110.0, 95.0, 105.0);
    assert_eq!(candle.open, 100.0);
    assert_eq!(candle.high, 110.0);
    assert_eq!(candle.low, 95.0);
    assert_eq!(candle.close, 105.0);
}

#[test]
fn test_candle_bullish() {
    let bullish = Candle::new(100.0, 110.0, 95.0, 108.0);
    assert!(bullish.is_bullish());

    let bearish = Candle::new(100.0, 110.0, 95.0, 92.0);
    assert!(!bearish.is_bullish());
}

#[test]
fn test_candle_metrics() {
    let candle = Candle::new(100.0, 115.0, 90.0, 110.0);
    assert_eq!(candle.body_size(), 10.0);
    assert_eq!(candle.upper_shadow(), 5.0);
    assert_eq!(candle.lower_shadow(), 10.0);
    assert_eq!(candle.range(), 25.0);
}

#[test]
fn test_chart_new() {
    let data = vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),
        Candle::new(103.0, 108.0, 102.0, 107.0),
    ];
    let chart = CandleChart::new(data);
    assert_eq!(chart.data.len(), 2);
}

#[test]
fn test_price_range() {
    let data = vec![
        Candle::new(100.0, 110.0, 95.0, 105.0),
        Candle::new(105.0, 120.0, 100.0, 115.0),
    ];
    let chart = CandleChart::new(data);
    let (min, max) = chart.get_price_range();

    // Should include padding
    assert!(min < 95.0);
    assert!(max > 120.0);
}

#[test]
fn test_price_change() {
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
fn test_heikin_ashi() {
    let data = vec![
        Candle::new(100.0, 110.0, 95.0, 105.0),
        Candle::new(105.0, 115.0, 100.0, 110.0),
    ];
    let chart = CandleChart::new(data).style(ChartStyle::HeikinAshi);
    let ha = chart.to_heikin_ashi();

    assert_eq!(ha.len(), 2);
    // First HA candle close should be average
    assert!((ha[0].close - 102.5).abs() < 0.001);
}

#[test]
fn test_helper_functions() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];

    let cc = candle_chart(data.clone());
    assert_eq!(cc.style, ChartStyle::Candle);

    let oc = ohlc_chart(data);
    assert_eq!(oc.style, ChartStyle::Ohlc);
}

#[test]
fn test_candle_with_volume() {
    let candle = Candle::with_volume(100.0, 110.0, 95.0, 105.0, 5000.0);
    assert_eq!(candle.volume, Some(5000.0));
    assert_eq!(candle.open, 100.0);
}

#[test]
fn test_candle_timestamp() {
    let candle = Candle::new(100.0, 110.0, 95.0, 105.0).timestamp(1234567890);
    assert_eq!(candle.timestamp, Some(1234567890));
}

#[test]
fn test_chart_style_ohlc() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).style(ChartStyle::Ohlc);
    assert_eq!(chart.style, ChartStyle::Ohlc);
}

#[test]
fn test_chart_size() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).size(30, 12);
    assert_eq!(chart.width, 30);
    assert_eq!(chart.height, 12);
}

#[test]
fn test_chart_bullish_color() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).bullish_color(Color::rgb(0, 255, 0));
    assert_eq!(chart.bullish_color, Color::rgb(0, 255, 0));
}

#[test]
fn test_chart_show_volume_true() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).show_volume(true);
    assert!(chart.show_volume);
}

#[test]
fn test_chart_builder_chain() {
    let data = vec![
        Candle::with_volume(100.0, 105.0, 98.0, 103.0, 1000.0),
        Candle::with_volume(103.0, 108.0, 102.0, 107.0, 1500.0),
    ];
    let chart = CandleChart::new(data)
        .title("Stock Chart")
        .style(ChartStyle::Ohlc)
        .size(40, 15)
        .bullish_color(Color::rgb(0, 200, 0))
        .bearish_color(Color::rgb(200, 0, 0))
        .show_volume(true)
        .show_axis(true)
        .show_grid(true)
        .crosshair(0)
        .precision(3)
        .price_range(95.0, 115.0)
        .scroll(0);

    assert_eq!(chart.title, Some("Stock Chart".to_string()));
    assert_eq!(chart.style, ChartStyle::Ohlc);
    assert_eq!(chart.width, 40);
    assert_eq!(chart.height, 15);
    assert!(chart.show_volume);
    assert!(chart.show_axis);
    assert!(chart.show_grid);
    assert_eq!(chart.crosshair, Some(0));
    assert_eq!(chart.precision, 3);
    assert_eq!(chart.min_price, Some(95.0));
    assert_eq!(chart.max_price, Some(115.0));
    assert_eq!(chart.offset, 0);
}