//! BoxPlot widget tests (BoxPlot, BoxGroup, BoxStats, WhiskerStyle)

mod axes;
mod boxes;
mod builders;
mod group;
mod scale;
mod stats;
mod view;

use super::rows;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::BoxStats;

/// Stats spanning 0..100 with quartiles at 25/50/75 and no outliers.
pub(super) fn even_stats() -> BoxStats {
    BoxStats {
        min: 0.0,
        q1: 25.0,
        median: 50.0,
        q3: 75.0,
        max: 100.0,
        outliers: vec![],
        whisker_low: 0.0,
        whisker_high: 100.0,
    }
}

/// Row `y` of `buffer` as a vector of chars.
pub(super) fn row_chars(buffer: &Buffer, y: u16) -> Vec<char> {
    rows(buffer)[y as usize].chars().collect()
}

/// Number of cells with symbol `ch` and foreground `fg`.
pub(super) fn count_colored(buffer: &Buffer, ch: char, fg: Color) -> usize {
    let mut n = 0;
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            let cell = buffer.get(x, y).unwrap();
            if cell.symbol == ch && cell.fg == Some(fg) {
                n += 1;
            }
        }
    }
    n
}
