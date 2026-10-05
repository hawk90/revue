//! CandleChart header tests not covered by tests/widget/candlechart.rs
//! (whose render tests only check for panics) or the in-source tests

use super::render_rows;
use revue::widget::data::chart::{Candle, CandleChart};

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
