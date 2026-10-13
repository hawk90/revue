//! CandleChart builder tests: size, the constructor functions, Clone and Debug

use super::super::render_rows;
use super::{glyph_columns, last_drawn_row, plot_rows, sample_data};
use revue::widget::{candle_chart, ohlc_chart, CandleChart, CandleStyle};

#[test]
fn test_chart_size_builder() {
    let rows = plot_rows(CandleChart::new(sample_data()).size(3, 6), 80, 30);
    assert_eq!(glyph_columns(&rows), 3);
    // The lowest low (98) in the padded range 97.3..112.7 of 6 rows
    assert_eq!(last_drawn_row(&rows), Some(4), "{rows:#?}");
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

#[test]
fn test_chart_debug() {
    let chart = CandleChart::new(sample_data()).title("Debug Test");
    let debug_str = format!("{:?}", chart);

    assert!(debug_str.contains("CandleChart"));
}
