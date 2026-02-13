//! Candle Chart widget public API tests extracted from candlechart.rs

use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::traits::RenderContext;
use crate::style::Color;

pub use crate::widget::data::chart::candlechart::{CandleChart, Candle, ChartStyle};

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
fn test_candle_chart_render_basic() {
    let data = vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),
        Candle::new(103.0, 108.0, 102.0, 107.0),
        Candle::new(107.0, 112.0, 106.0, 111.0),
    ];

    let chart = CandleChart::new(data);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_candlechart_render_with_title() {
    let data = vec![Candle::new(100.0, 105.0, 98.0, 103.0)];
    let chart = CandleChart::new(data).title("AAPL");

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);

    // Title should be rendered
    let mut title_found = false;
    for x in 0..40 {
        if let Some(cell) = buffer.get(x, 0) {
            if cell.symbol == 'A' {
                title_found = true;
                break;
            }
        }
    }
    assert!(title_found);
}

#[test]
fn test_candlechart_render_with_volume() {
    let data = vec![
        Candle::with_volume(100.0, 105.0, 98.0, 103.0, 1000.0),
        Candle::with_volume(103.0, 108.0, 102.0, 107.0, 1500.0),
    ];
    let chart = CandleChart::new(data).show_volume(true);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should render volume bars
}

#[test]
fn test_candlechart_render_empty() {
    let chart = CandleChart::new(vec![]);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should handle empty data gracefully
}

#[test]
fn test_candlechart_render_ohlc() {
    let data = vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),
        Candle::new(103.0, 108.0, 102.0, 107.0),
    ];
    let chart = CandleChart::new(data).style(ChartStyle::Ohlc);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should render OHLC bars
}

#[test]
fn test_candlechart_render_hollow() {
    let data = vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),
        Candle::new(103.0, 108.0, 102.0, 107.0),
    ];
    let chart = CandleChart::new(data).style(ChartStyle::Hollow);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should render hollow candles
}

#[test]
fn test_candlechart_render_heikin_ashi() {
    let data = vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),
        Candle::new(103.0, 108.0, 102.0, 107.0),
    ];
    let chart = CandleChart::new(data).style(ChartStyle::HeikinAshi);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    chart.render(&mut ctx);
    // Should render Heikin-Ashi candles
}

// =========================================================================
// ChartStyle enum tests (derived traits)
// =========================================================================

#[test]
fn test_chart_style_default() {
    assert_eq!(ChartStyle::default(), ChartStyle::Candle);
}

#[test]
fn test_chart_style_clone() {
    let style1 = ChartStyle::Ohlc;
    let style2 = style1.clone();
    assert_eq!(style1, style2);
}

#[test]
fn test_chart_style_copy() {
    let style1 = ChartStyle::Hollow;
    let style2 = style1;
    assert_eq!(style2, ChartStyle::Hollow);
}

#[test]
fn test_chart_style_partial_eq() {
    assert_eq!(ChartStyle::Candle, ChartStyle::Candle);
    assert_eq!(ChartStyle::Ohlc, ChartStyle::Ohlc);
    assert_ne!(ChartStyle::Candle, ChartStyle::HeikinAshi);
}

// =========================================================================
// CandleChart::style tests
// =========================================================================

#[test]
fn test_chart_style_ohlc() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).style(ChartStyle::Ohlc);
    assert_eq!(chart.style, ChartStyle::Ohlc);
}

#[test]
fn test_chart_style_hollow() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).style(ChartStyle::Hollow);
    assert_eq!(chart.style, ChartStyle::Hollow);
}

#[test]
fn test_chart_style_heikin_ashi() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).style(ChartStyle::HeikinAshi);
    assert_eq!(chart.style, ChartStyle::HeikinAshi);
}

// =========================================================================
// CandleChart::size tests
// =========================================================================

#[test]
fn test_chart_size() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).size(30, 12);
    assert_eq!(chart.width, 30);
    assert_eq!(chart.height, 12);
}

// =========================================================================
// CandleChart::height tests
// =========================================================================

#[test]
fn test_chart_height() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).height(20);
    assert_eq!(chart.height, 20);
}

// =========================================================================
// CandleChart::width tests
// =========================================================================

#[test]
fn test_chart_width() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).width(50);
    assert_eq!(chart.width, 50);
}

// =========================================================================
// CandleChart::bullish_color tests
// =========================================================================

#[test]
fn test_chart_bullish_color() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).bullish_color(Color::rgb(0, 255, 0));
    assert_eq!(chart.bullish_color, Color::rgb(0, 255, 0));
}

// =========================================================================
// CandleChart::bearish_color tests
// =========================================================================

#[test]
fn test_chart_bearish_color() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).bearish_color(Color::rgb(255, 0, 0));
    assert_eq!(chart.bearish_color, Color::rgb(255, 0, 0));
}

// =========================================================================
// CandleChart::show_volume tests
// =========================================================================

#[test]
fn test_chart_show_volume_true() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).show_volume(true);
    assert!(chart.show_volume);
}

#[test]
fn test_chart_show_volume_false() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).show_volume(false);
    assert!(!chart.show_volume);
}

// =========================================================================
// CandleChart::show_axis tests
// =========================================================================

#[test]
fn test_chart_show_axis_true() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).show_axis(true);
    assert!(chart.show_axis);
}

#[test]
fn test_chart_show_axis_false() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).show_axis(false);
    assert!(!chart.show_axis);
}

// =========================================================================
// CandleChart::title tests
// =========================================================================

#[test]
fn test_chart_title_str() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).title("AAPL");
    assert_eq!(chart.title, Some("AAPL".to_string()));
}

#[test]
fn test_chart_title_string() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).title(String::from("GOOGL"));
    assert_eq!(chart.title, Some("GOOGL".to_string()));
}

// =========================================================================
// CandleChart::crosshair tests
// =========================================================================

#[test]
fn test_chart_crosshair() {
    let data = vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),
        Candle::new(103.0, 108.0, 102.0, 107.0),
    ];
    let chart = CandleChart::new(data).crosshair(1);
    assert_eq!(chart.crosshair, Some(1));
}

// =========================================================================
// CandleChart::precision tests
// =========================================================================

#[test]
fn test_chart_precision() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).precision(4);
    assert_eq!(chart.precision, 4);
}

// =========================================================================
// CandleChart::price_range tests
// =========================================================================

#[test]
fn test_chart_price_range_method() {
    let data = vec![Candle::new(100.0, 110.0, 95.0, 105.0)];
    let chart = CandleChart::new(data).price_range(90.0, 120.0);
    assert_eq!(chart.min_price, Some(90.0));
    assert_eq!(chart.max_price, Some(120.0));
}

// =========================================================================
// CandleChart::scroll tests
// =========================================================================

#[test]
fn test_chart_scroll() {
    let data = vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),
        Candle::new(103.0, 108.0, 102.0, 107.0),
        Candle::new(107.0, 112.0, 106.0, 111.0),
    ];
    let chart = CandleChart::new(data).scroll(1);
    assert_eq!(chart.offset, 1);
}

// =========================================================================
// CandleChart::current_price tests
// =========================================================================

#[test]
fn test_current_price_some() {
    let data = vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),
        Candle::new(103.0, 108.0, 102.0, 107.0),
    ];
    let chart = CandleChart::new(data);
    assert_eq!(chart.current_price(), Some(107.0));
}

#[test]
fn test_current_price_none() {
    let chart = CandleChart::new(vec![]);
    assert_eq!(chart.current_price(), None);
}

// =========================================================================
// CandleChart::price_change tests
// =========================================================================

#[test]
fn test_price_change_single_candle() {
    let data = vec![Candle::new(100.0, 105.0, 98.0, 103.0)];
    let chart = CandleChart::new(data);
    assert_eq!(chart.price_change(), None);
}

#[test]
fn test_price_change_negative() {
    let data = vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),
        Candle::new(103.0, 108.0, 102.0, 100.0),
    ];
    let chart = CandleChart::new(data);
    let (change, _percent) = chart.price_change().unwrap();
    assert_eq!(change, -3.0);
}

// =========================================================================
// CandleChart::visible_candles tests
// =========================================================================

#[test]
fn test_visible_candles_partial() {
    let data = vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),
        Candle::new(103.0, 108.0, 102.0, 107.0),
        Candle::new(107.0, 112.0, 106.0, 111.0),
    ];
    let chart = CandleChart::new(data).width(2).scroll(1);
    let visible = chart.visible_candles();
    assert_eq!(visible.len(), 2);
}

#[test]
fn test_visible_candles_empty() {
    let chart = CandleChart::new(vec![]);
    let visible = chart.visible_candles();
    assert_eq!(visible.len(), 0);
}

// =========================================================================
// Chart builder chain test
// =========================================================================

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