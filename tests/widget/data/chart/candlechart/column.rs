//! CandleChart column tests: candle, OHLC and hollow glyphs and colors

use super::super::render;
use super::{glyph_columns, mixed_data, plot_rows, sample_data};
use revue::style::Color;
use revue::widget::{CandleChart, CandleStyle};

#[test]
fn test_render_with_custom_colors() {
    let default = CandleChart::new(mixed_data()).show_axis(false);
    let custom = CandleChart::new(mixed_data())
        .bullish_color(Color::CYAN)
        .bearish_color(Color::MAGENTA)
        .show_axis(false);
    for (chart, up, down) in [
        (default, Color::GREEN, Color::RED),
        (custom, Color::CYAN, Color::MAGENTA),
    ] {
        let buffer = render(&chart, 80, 30);
        // mixed_data alternates bullish and bearish candles
        for x in 0..5u16 {
            let want = if x % 2 == 0 { up } else { down };
            let bodies: Vec<_> = (0..15)
                .filter_map(|y| buffer.get(x, y))
                .filter(|c| c.symbol == '█')
                .collect();
            assert!(!bodies.is_empty(), "column {x}");
            assert!(bodies.iter().all(|c| c.fg == Some(want)), "column {x}");
        }
    }
}

#[test]
fn test_render_ohlc_style() {
    let rows = plot_rows(
        CandleChart::new(mixed_data()).style(CandleStyle::Ohlc),
        80,
        30,
    );
    // Open tick on the left, close tick on the right, no bodies
    assert!(rows.iter().any(|r| r.contains('├')), "{rows:#?}");
    assert!(rows.iter().any(|r| r.contains('┤')), "{rows:#?}");
    assert!(!rows.iter().any(|r| r.contains('█')), "{rows:#?}");
    assert_eq!(glyph_columns(&rows), 5);
}

#[test]
fn test_render_hollow_style() {
    // Bullish bodies are outlines, bearish ones are filled
    let rows = plot_rows(
        CandleChart::new(sample_data()).style(CandleStyle::Hollow),
        80,
        30,
    );
    assert!(rows.iter().any(|r| r.contains('┌')), "{rows:#?}");
    assert!(rows.iter().any(|r| r.contains('└')), "{rows:#?}");
    assert!(!rows.iter().any(|r| r.contains('█')), "{rows:#?}");

    let rows = plot_rows(
        CandleChart::new(mixed_data()).style(CandleStyle::Hollow),
        80,
        30,
    );
    assert!(rows.iter().any(|r| r.contains('█')), "{rows:#?}");
}
