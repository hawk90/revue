//! CandleChart price scale tests: padding, a fixed range, flat and wide ranges, chart height

use super::super::{render, render_rows};
use super::{first_word, glyph_columns, last_drawn_row, plot_rows, sample_data};
use revue::widget::data::chart::candlechart::ChartStyle;
use revue::widget::traits::RenderContext;
use revue::widget::{Candle, CandleChart, View};

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
    assert_eq!(super::super::count_char(&render(&chart, 80, 20), '█'), 1);
    assert!(rows[7].contains('█'), "{rows:#?}");
}

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

#[test]
fn test_chart_height_builder() {
    let rows = plot_rows(CandleChart::new(sample_data()).height(25), 80, 30);
    // The lowest low (98) in the padded range 97..119 of 25 rows
    assert_eq!(last_drawn_row(&rows), Some(22), "{rows:#?}");
}

#[test]
fn test_very_small_height() {
    let rows = plot_rows(CandleChart::new(sample_data()).height(3), 80, 20);
    assert_eq!(glyph_columns(&rows), 5);
    assert!(rows[3..].iter().all(|r| r.trim().is_empty()), "{rows:#?}");
}

#[test]
fn test_render_zero_height() {
    let chart = CandleChart::new(sample_data()).height(0);
    assert!(render_rows(&chart, 40, 20)
        .iter()
        .all(|r| r.trim().is_empty()));
}
