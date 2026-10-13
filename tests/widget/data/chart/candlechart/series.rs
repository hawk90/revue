//! CandleChart series tests: the drawn window (scroll, width, Heikin-Ashi) and the price figures

use super::super::render_rows;
use super::{glyph_columns, last_drawn_row, plot_rows, sample_data};
use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{Candle, CandleChart, CandleStyle, View};

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
fn test_chart_width_builder() {
    // Wider than the data: every candle
    let rows = plot_rows(CandleChart::new(sample_data()).width(80), 100, 20);
    assert_eq!(glyph_columns(&rows), 5);

    let rows = plot_rows(CandleChart::new(sample_data()).width(2), 100, 20);
    assert_eq!(glyph_columns(&rows), 2);
}

#[test]
fn test_very_small_width() {
    let rows = plot_rows(CandleChart::new(sample_data()).width(1), 80, 20);
    assert_eq!(glyph_columns(&rows), 1);
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
