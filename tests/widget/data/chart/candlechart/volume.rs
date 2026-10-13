//! CandleChart volume tests: separator and bars, missing and zero volume

use super::super::render_rows;
use super::sample_data_with_volume;
use revue::widget::{Candle, CandleChart};

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
