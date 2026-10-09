//! `RenderBatch::optimize` changes how a batch is sent, never what it
//! draws: overlapping ops keep their paint order, and wide cells are not
//! merged into text that re-advances by char width (#867)

use revue::layout::Rect;
use revue::render::{Buffer, RenderBatch};
use revue::style::Color;

fn rows(buffer: &Buffer) -> Vec<String> {
    (0..buffer.height())
        .map(|y| {
            (0..buffer.width())
                .map(|x| {
                    let cell = buffer.get(x, y).unwrap();
                    format!("{}{:?}", cell.symbol, cell.fg)
                })
                .collect()
        })
        .collect()
}

/// The batch drawn as queued, and drawn after optimize
fn drawn_both_ways(build: impl Fn(&mut RenderBatch)) -> (Vec<String>, Vec<String>) {
    let draw = |optimize: bool| {
        let mut batch = RenderBatch::new();
        build(&mut batch);
        if optimize {
            batch.optimize();
        }
        let mut buffer = Buffer::new(6, 2);
        batch.apply_to_buffer(&mut buffer);
        rows(&buffer)
    };
    (draw(false), draw(true))
}

#[test]
fn a_fill_drawn_after_text_stays_on_top() {
    let (plain, optimized) = drawn_both_ways(|b| {
        b.text(1, 0, "ab", None, None);
        b.fill_rect(Rect::new(0, 0, 4, 1), '=', None, None);
    });
    assert_eq!(optimized, plain);
}

#[test]
fn a_cell_drawn_after_a_line_stays_on_top() {
    let (plain, optimized) = drawn_both_ways(|b| {
        b.set_cell(2, 0, 'x', None, None);
        b.hline(0, 0, 4, '-', None);
        b.set_cell(1, 1, 'y', None, None);
        b.vline(1, 0, 2, '|', None);
    });
    assert_eq!(optimized, plain);
}

#[test]
fn a_wide_cell_is_not_merged_into_text() {
    let (plain, optimized) = drawn_both_ways(|b| {
        b.set_cell(0, 0, '日', None, None);
        b.set_cell(1, 0, 'x', None, None);
        b.set_cell(2, 0, 'y', None, None);
    });
    assert_eq!(optimized, plain);
}

#[test]
fn cells_out_of_order_still_merge() {
    let mut batch = RenderBatch::new();
    for x in [3, 1, 2, 0] {
        batch.set_cell(x, 0, 'a', Some(Color::RED), None);
    }
    batch.optimize();
    assert_eq!(batch.len(), 1, "four adjacent cells become one text op");
}

#[test]
fn writes_to_the_same_cell_keep_their_order() {
    let (plain, optimized) = drawn_both_ways(|b| {
        b.set_cell(1, 0, 'a', None, None);
        b.set_cell(0, 0, 'z', None, None);
        b.set_cell(1, 0, 'b', None, None);
    });
    assert_eq!(optimized, plain);
}
