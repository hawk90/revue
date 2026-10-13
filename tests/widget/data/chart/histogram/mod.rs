//! Histogram widget tests
//!
//! Layout used below: a 40x20 histogram without title has its plot area at
//! x=6..39, y=0..18, Y labels in columns 0..6 and X labels on row 19.

mod axes;
mod bars;
mod builders;
mod scale;
mod stats;
mod view;

use super::render;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::{BinConfig, Histogram};

/// Bins [0,10) [10,20) [20,30] holding 3, 1 and 2 values.
pub(super) fn three_bins() -> Histogram {
    Histogram::new(&[1.0, 2.0, 3.0, 11.0, 21.0, 22.0])
        .bins(BinConfig::Edges(vec![0.0, 10.0, 20.0, 30.0]))
}

pub(super) fn sym(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

/// Row on which the bar starting at column `x` has its top cell.
pub(super) fn bar_top(buffer: &Buffer, x: u16) -> Option<u16> {
    (0..18).find(|&y| sym(buffer, x, y) == '▀')
}

/// Number of bars, counted by their border cells on the bottom plot row.
pub(super) fn bar_count(hist: Histogram) -> usize {
    let buffer = render(&hist.bar_border(Color::WHITE), 40, 20);
    let bottom: String = (6..39).map(|x| sym(&buffer, x, 17)).collect();
    bottom.matches('│').count() / 2
}
