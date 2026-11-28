//! Chart (line/area/step/scatter) widget tests
//!
//! Layout used below: a 40x20 chart without border or title has its plot
//! area at y=0..18 and X labels on row 18. The Y label column is as wide as
//! the widest Y label (#1606), with one gap column after it, so the plot
//! starts at x=6 for the `diagonal` and empty charts (labels such as
//! "-0.50" and "-0.05", 5 columns) and at x=5 for `flat` ("4.50", 4).

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

/// Line-drawing cells in the plot rows of a 40x20 chart (the labels left
/// of the plot are never line-drawing cells).
pub(super) fn plot_line_cells(buffer: &Buffer) -> usize {
    let mut n = 0;
    for y in 0..18 {
        for x in 0..40 {
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
