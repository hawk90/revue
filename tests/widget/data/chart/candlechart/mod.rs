//! CandleChart widget tests

mod axis;
mod builders;
mod candle;
mod column;
mod scale;
mod series;
mod view;
mod volume;

use super::render_rows;
use revue::widget::{Candle, CandleChart};

pub(super) fn sample_data() -> Vec<Candle> {
    vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),
        Candle::new(103.0, 108.0, 102.0, 107.0),
        Candle::new(107.0, 112.0, 106.0, 110.0),
        Candle::new(110.0, 115.0, 108.0, 112.0),
        Candle::new(112.0, 118.0, 110.0, 116.0),
    ]
}

pub(super) fn sample_data_with_volume() -> Vec<Candle> {
    vec![
        Candle::with_volume(100.0, 105.0, 98.0, 103.0, 1000.0),
        Candle::with_volume(103.0, 108.0, 102.0, 107.0, 1500.0),
        Candle::with_volume(107.0, 112.0, 106.0, 110.0, 800.0),
        Candle::with_volume(110.0, 115.0, 108.0, 112.0, 2000.0),
        Candle::with_volume(112.0, 118.0, 110.0, 116.0, 1200.0),
    ]
}

pub(super) fn mixed_data() -> Vec<Candle> {
    vec![
        Candle::new(100.0, 105.0, 98.0, 103.0),  // bullish
        Candle::new(103.0, 108.0, 100.0, 102.0), // bearish
        Candle::new(102.0, 107.0, 101.0, 105.0), // bullish
        Candle::new(105.0, 110.0, 103.0, 104.0), // bearish
        Candle::new(104.0, 109.0, 103.0, 108.0), // bullish
    ]
}

/// Rows of `chart` drawn without its price axis, so column `x` is candle `x`
pub(super) fn plot_rows(chart: CandleChart, width: u16, height: u16) -> Vec<String> {
    render_rows(&chart.show_axis(false), width, height)
}

/// Columns holding any candle glyph
pub(super) fn glyph_columns(rows: &[String]) -> usize {
    let width = rows.iter().map(|r| r.chars().count()).max().unwrap_or(0);
    (0..width)
        .filter(|&x| {
            rows.iter()
                .any(|r| r.chars().nth(x).is_some_and(|c| CANDLE_GLYPHS.contains(&c)))
        })
        .count()
}

/// Index of the last row holding anything
pub(super) fn last_drawn_row(rows: &[String]) -> Option<usize> {
    rows.iter().rposition(|r| !r.trim().is_empty())
}

/// The first word of a row, such as its price label
pub(super) fn first_word(row: &str) -> &str {
    row.split_whitespace().next().unwrap_or("")
}

/// Glyphs a candle column is drawn with, in any style
pub(super) const CANDLE_GLYPHS: [char; 7] = ['█', '│', '├', '┤', '┌', '└', '─'];
