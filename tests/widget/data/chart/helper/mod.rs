//! Chart (line/area/step/scatter) widget tests
//!
//! Layout used below: a 40x20 chart without border or title has its plot
//! area at x=8..40, y=0..18, Y labels in columns 0..8 and X labels on
//! row 18.

mod axes;
mod constructors;
mod legend;
mod marks;
mod render;
mod scale;

use revue::render::Buffer;
use revue::widget::data::chart::{Axis, Chart, Series};

/// A chart with no legend and no grid lines.
pub(super) fn plain() -> Chart {
    Chart::new()
        .no_legend()
        .x_axis(Axis::new().grid(false))
        .y_axis(Axis::new().grid(false))
}

pub(super) fn diagonal() -> Series {
    Series::new("Data").data(vec![(0.0, 0.0), (10.0, 10.0)])
}

pub(super) fn flat() -> Series {
    Series::new("Flat").data(vec![(1.0, 5.0), (2.0, 5.0), (3.0, 5.0)])
}

pub(super) fn sym(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

/// Line-drawing cells inside the plot area of a 40x20 chart.
pub(super) fn plot_line_cells(buffer: &Buffer) -> usize {
    let mut n = 0;
    for y in 0..18 {
        for x in 8..40 {
            if matches!(sym(buffer, x, y), '─' | '│' | '╱' | '╲') {
                n += 1;
            }
        }
    }
    n
}

pub(super) fn is_braille(c: char) -> bool {
    ('\u{2801}'..='\u{28FF}').contains(&c)
}
