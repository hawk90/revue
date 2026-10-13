//! HeatMap widget tests
//!
//! Assertions go through normalize()/color_for()/render_cell() or the
//! rendered buffer, not private fields.
//!
//! HeatMap lays itself out with stacks and gives every line one row and every
//! piece of a line exactly its text's width, so the cells start at the top
//! left (after the title, column labels and row labels) in any area large
//! enough to hold them.

mod builders;
mod cell;
mod types;
mod view;

use super::rows;
use revue::render::Buffer;
use revue::style::Color;

pub(super) fn fg(buffer: &Buffer, x: u16, y: u16) -> Option<Color> {
    buffer.get(x, y).unwrap().fg
}

pub(super) fn blank(buffer: &Buffer) -> bool {
    rows(buffer).iter().all(|r| r.trim().is_empty())
}
