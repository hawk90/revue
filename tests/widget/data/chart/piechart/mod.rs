//! PieChart widget tests
//!
//! Geometry used below: a 40x20 chart without title is centered on
//! (20, 10). Slices are drawn clockwise from `start_angle` (-90 = top).

mod builders;
mod labels;
mod slices;
mod types;
mod view;

use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::PieChart;

/// Positions of '█' cells drawn in `color`.
pub(super) fn cells(buffer: &Buffer, color: Color) -> Vec<(u16, u16)> {
    let mut out = Vec::new();
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            let cell = buffer.get(x, y).unwrap();
            if cell.symbol == '█' && cell.fg == Some(color) {
                out.push((x, y));
            }
        }
    }
    out
}

pub(super) fn halves() -> PieChart {
    PieChart::new()
        .no_legend()
        .slice_colored("A", 50.0, Color::RED)
        .slice_colored("B", 50.0, Color::BLUE)
}

/// Positions of every '█' cell, whatever its color.
pub(super) fn filled(buffer: &Buffer) -> Vec<(u16, u16)> {
    let mut out = Vec::new();
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            if buffer.get(x, y).unwrap().symbol == '█' {
                out.push((x, y));
            }
        }
    }
    out
}
