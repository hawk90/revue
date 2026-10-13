//! CandleChart price axis tests: labels, precision, hiding the axis

use super::super::render_rows;
use super::{first_word, glyph_columns, plot_rows, sample_data, CANDLE_GLYPHS};
use revue::widget::{Candle, CandleChart};

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
