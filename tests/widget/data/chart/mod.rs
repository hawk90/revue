//! Chart widget tests (src/widget/data/chart)

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::View;

mod barchart;
mod boxplot;
mod candlechart;
mod chart_common;
mod chart_render;
mod chart_stats;
mod heatmap;
mod helper;
mod histogram;
mod piechart;
mod scatterchart;
mod sparkline;
mod timeseries;
mod types;
mod waveline;

pub use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

/// Render `view` into a fresh `width` x `height` buffer.
pub fn render(view: &impl View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    view.render(&mut ctx);
    buffer
}

/// Each row of `buffer` as text.
pub fn rows(buffer: &Buffer) -> Vec<String> {
    (0..buffer.height())
        .map(|y| {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
                .collect()
        })
        .collect()
}

/// Render `view` and return each row as text.
pub fn render_rows(view: &impl View, width: u16, height: u16) -> Vec<String> {
    rows(&render(view, width, height))
}

/// Number of cells in `buffer` showing `ch`.
pub fn count_char(buffer: &Buffer, ch: char) -> usize {
    rows(buffer)
        .iter()
        .map(|r| r.chars().filter(|&c| c == ch).count())
        .sum()
}
