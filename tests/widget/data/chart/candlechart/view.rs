//! CandleChart layout tests: header, crosshair line, placeholder, View and StyledView

use super::super::render_rows;
use super::{glyph_columns, plot_rows, sample_data, sample_data_with_volume};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{Candle, CandleChart, CandleStyle, StyledView, View};

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
fn test_chart_show_grid_builder() {
    let chart = CandleChart::new(sample_data()).show_grid(true);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    chart.render(&mut ctx);
}

#[test]
fn test_candlechart_zero_area_draws_nothing() {
    let chart = CandleChart::new(sample_data_with_volume())
        .title("AAPL")
        .show_volume(true)
        .crosshair(1);
    super::super::assert_zero_areas_draw_nothing(&chart);
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
    super::super::assert_same_when_offset(full(), 30, 16);
    super::super::assert_same_when_offset(full().style(CandleStyle::Ohlc).show_axis(false), 30, 16);
    super::super::assert_same_when_offset(full().style(CandleStyle::HeikinAshi), 30, 16);
}
