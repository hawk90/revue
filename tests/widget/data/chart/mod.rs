//! Chart widget tests (src/widget/data/chart)

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{hstack, vstack, Text, View};

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

/// Assert that `view` draws nothing into a zero-width or zero-height area
/// inside a 10x10 buffer.
pub fn assert_zero_areas_draw_nothing(view: &impl View) {
    for (w, h) in [(0, 0), (0, 10), (10, 0)] {
        let mut buffer = Buffer::new(10, 10);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
        view.render(&mut ctx);
        assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()), "{w}x{h}");
    }
}

/// Assert that `view`, laid out by stacks below a 3-row header and right of
/// a `width`-column spacer, draws the same cells as `view` drawn alone, moved
/// by that offset, and nothing outside its own area.
pub fn assert_same_when_offset<V: View + Clone + 'static>(view: V, width: u16, height: u16) {
    let alone = render(&view, width, height);
    let row = hstack()
        .child_sized(Text::new(""), width)
        .child_sized(view, width);
    let page = vstack()
        .child_sized(Text::new(""), 3)
        .child_sized(row, height);
    let placed = render(&page, width * 2, height + 3);
    for y in 0..height + 3 {
        for x in 0..width * 2 {
            let cell = placed.get(x, y).unwrap();
            if x < width || y < 3 {
                assert_eq!(cell.symbol, ' ', "drawn outside its area at ({x}, {y})");
            } else {
                assert_eq!(cell, alone.get(x - width, y - 3).unwrap(), "({x}, {y})");
            }
        }
    }
}
