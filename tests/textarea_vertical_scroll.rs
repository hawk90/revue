//! TextArea scrolls vertically to keep the cursor in view.
//!
//! The vertical offset only ever moved up (when the cursor went above it), so
//! moving the cursor below the last visible row, paging down or pressing
//! Enter on the last row left the cursor off screen. The offset is now
//! settled at render time, where the viewport height is known, the same way
//! the horizontal scroll is.

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::TextArea;

fn render(ta: &TextArea, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let area = Rect::new(0, 0, w, h);
    let mut ctx = RenderContext::new(&mut buffer, area);
    ta.render(&mut ctx);
    buffer
}

/// The visible rows, trailing blanks trimmed.
fn rows(ta: &TextArea, w: u16, h: u16) -> Vec<String> {
    let buffer = render(ta, w, h);
    (0..h)
        .map(|y| {
            (0..w)
                .filter_map(|x| buffer.get(x, y))
                .map(|cell| cell.symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}

/// Twenty lines `l00` to `l19`, unwrapped, with no gutter and a 3-row-high
/// view (TextArea's default minimum height).
fn numbered() -> TextArea {
    let text: Vec<String> = (0..20).map(|i| format!("l{i:02}")).collect();
    TextArea::new()
        .content(text.join("\n"))
        .line_numbers(false)
        .wrap(false)
        .focused(true)
}

fn press(ta: &mut TextArea, key: Key, times: usize) {
    for _ in 0..times {
        ta.handle_key(&key);
    }
}

#[test]
fn moving_down_past_the_bottom_scrolls_by_as_little_as_possible() {
    let mut ta = numbered();
    assert_eq!(rows(&ta, 5, 3), ["l00", "l01", "l02"]);

    press(&mut ta, Key::Down, 3);
    assert_eq!(ta.cursor_position(), (3, 0));
    assert_eq!(rows(&ta, 5, 3), ["l01", "l02", "l03"]);

    press(&mut ta, Key::Down, 4);
    assert_eq!(rows(&ta, 5, 3), ["l05", "l06", "l07"]);
}

#[test]
fn moving_back_up_inside_the_view_does_not_scroll() {
    let mut ta = numbered();
    press(&mut ta, Key::Down, 7);
    assert_eq!(rows(&ta, 5, 3), ["l05", "l06", "l07"]);

    press(&mut ta, Key::Up, 2);
    assert_eq!(rows(&ta, 5, 3), ["l05", "l06", "l07"]);
}

#[test]
fn moving_up_past_the_top_scrolls_up() {
    let mut ta = numbered();
    press(&mut ta, Key::Down, 10);
    assert_eq!(rows(&ta, 5, 3), ["l08", "l09", "l10"]);

    press(&mut ta, Key::Up, 4);
    assert_eq!(ta.cursor_position(), (6, 0));
    assert_eq!(rows(&ta, 5, 3), ["l06", "l07", "l08"]);
}

#[test]
fn page_down_and_page_up_keep_the_cursor_in_view() {
    let mut ta = numbered();
    // Learn the viewport height, which is also the page size.
    rows(&ta, 5, 4);

    press(&mut ta, Key::PageDown, 2);
    assert_eq!(ta.cursor_position(), (8, 0));
    assert_eq!(rows(&ta, 5, 4), ["l05", "l06", "l07", "l08"]);

    press(&mut ta, Key::PageUp, 1);
    assert_eq!(ta.cursor_position(), (4, 0));
    assert_eq!(rows(&ta, 5, 4), ["l04", "l05", "l06", "l07"]);
}

#[test]
fn enter_on_the_last_visible_row_scrolls_to_the_new_line() {
    let mut ta = TextArea::new()
        .content("a\nb\nc")
        .line_numbers(false)
        .wrap(false)
        .focused(true);
    ta.set_cursor(2, 1);
    assert_eq!(rows(&ta, 5, 3), ["a", "b", "c"]);

    ta.handle_key(&Key::Enter);
    ta.handle_key(&Key::Char('d'));
    assert_eq!(ta.cursor_position(), (3, 1));
    assert_eq!(rows(&ta, 5, 3), ["b", "c", "d"]);
}

#[test]
fn jumping_to_the_last_line_shows_it() {
    let mut ta = numbered();
    ta.set_cursor(19, 0);
    assert_eq!(rows(&ta, 5, 3), ["l17", "l18", "l19"]);
}

#[test]
fn wrapped_lines_count_every_visual_row() {
    // Line 0 takes two rows at width 5, so line 2 is the fourth row.
    let mut ta = TextArea::new()
        .content("aaaaaaaaaa\nb\nc")
        .line_numbers(false)
        .wrap(true)
        .focused(true);
    assert_eq!(rows(&ta, 5, 3), ["aaaaa", "aaaaa", "b"]);

    ta.set_cursor(2, 0);
    assert_eq!(rows(&ta, 5, 3), ["b", "c", ""]);
}

#[test]
fn a_cursor_on_a_later_wrapped_row_of_its_line_is_shown() {
    // Line 2 wraps onto two rows; the cursor sits on the second.
    let mut ta = TextArea::new()
        .content("a\nb\ncccccddddd")
        .line_numbers(false)
        .wrap(true)
        .focused(true);
    ta.set_cursor(2, 7);
    assert_eq!(rows(&ta, 5, 3), ["b", "ccccc", "ddddd"]);
}
