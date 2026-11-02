//! TextArea with tabs in the text (#1378): a tab reaches the next multiple
//! of `tab_width` from the start of its row, in drawing, the cursor,
//! wrapping, horizontal scroll and moving between lines alike.

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::TextArea;

fn render(ta: &TextArea, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    ta.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// Cells drawn on the cursor's white background, as `(x, y)`
fn cursor_cells(ta: &TextArea, w: u16, h: u16) -> Vec<(u16, u16)> {
    let buffer = render(ta, w, h);
    let mut cells = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if buffer.get(x, y).unwrap().bg == Some(Color::WHITE) {
                cells.push((x, y));
            }
        }
    }
    cells
}

fn area(content: &str) -> TextArea {
    TextArea::new().line_numbers(false).content(content)
}

#[test]
fn a_tab_reaches_the_next_stop() {
    let ta = area("\tb\nab\tc\nabcd\te");
    let buffer = render(&ta, 20, 3);
    assert_eq!(row(&buffer, 0), "    b");
    assert_eq!(row(&buffer, 1), "ab  c");
    assert_eq!(row(&buffer, 2), "abcd    e");
    // The stop is `tab_width`
    let buffer = render(&area("\tb\na\tb").tab_width(2), 10, 2);
    assert_eq!(row(&buffer, 0), "  b");
    assert_eq!(row(&buffer, 1), "a b");
}

#[test]
fn the_cursor_on_a_tab_covers_it() {
    let mut ta = area("\tx").focused(true);
    ta.set_cursor(0, 0);
    assert_eq!(
        cursor_cells(&ta, 10, 1),
        vec![(0, 0), (1, 0), (2, 0), (3, 0)]
    );
    ta.set_cursor(0, 1);
    assert_eq!(cursor_cells(&ta, 10, 1), vec![(4, 0)]);
    // Past the end of a line ending in a tab: after the stop
    let mut ta = area("a\t").focused(true);
    ta.set_cursor(0, 2);
    assert_eq!(cursor_cells(&ta, 10, 1), vec![(4, 0)]);
}

#[test]
fn wrapping_counts_tab_columns() {
    // Text width 6 (a 7-column view keeps its last column, #1008): `ab`,
    // a tab to column 4, `cd` fill the row, `ef` wraps
    let ta = area("ab\tcdef").wrap(true);
    let buffer = render(&ta, 7, 3);
    assert_eq!(row(&buffer, 0), "ab  cd");
    assert_eq!(row(&buffer, 1), "ef");
}

#[test]
fn a_wrapped_row_starts_its_own_tab_stops() {
    // The tab starts row 1, so it takes a whole stop from that row's start
    let ta = area("abcdef\tg").wrap(true);
    let buffer = render(&ta, 7, 3);
    assert_eq!(row(&buffer, 0), "abcdef");
    assert_eq!(row(&buffer, 1), "    g");
}

#[test]
fn a_tab_wider_than_the_row_still_moves_on() {
    // Text width 2: a tab reaches column 4 but has a row of its own, and
    // the rows after it go on
    let ta = area("\tab").wrap(true).tab_width(4);
    let buffer = render(&ta, 3, 3);
    assert_eq!(row(&buffer, 0), "");
    assert_eq!(row(&buffer, 1), "ab");
}

#[test]
fn horizontal_scroll_counts_tab_columns() {
    let mut ta = area("\t\tx").wrap(false).focused(true);
    ta.set_cursor(0, 2); // `x`, column 8
    let buffer = render(&ta, 6, 1);
    assert_eq!(row(&buffer, 0), "     x");
    assert_eq!(cursor_cells(&ta, 6, 1), vec![(5, 0)]);
    ta.set_cursor(0, 0);
    assert_eq!(
        cursor_cells(&ta, 6, 1),
        vec![(0, 0), (1, 0), (2, 0), (3, 0)]
    );
}

#[test]
fn up_and_down_keep_the_screen_column_across_tabs() {
    let mut ta = area("\tx\nabcdefgh").focused(true);
    ta.set_cursor(0, 1); // `x`, column 4
    ta.handle_key(&Key::Down);
    assert_eq!(ta.cursor_position(), (1, 4));
    ta.set_cursor(1, 2); // column 2, inside the tab above
    ta.handle_key(&Key::Up);
    assert_eq!(ta.cursor_position(), (0, 0));
}
