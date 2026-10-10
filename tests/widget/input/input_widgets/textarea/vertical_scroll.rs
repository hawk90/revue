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
fn page_down_and_page_up_scroll_the_view_by_a_page() {
    let mut ta = numbered();
    // Learn the viewport height, which is also the page size.
    rows(&ta, 5, 4);

    press(&mut ta, Key::PageDown, 2);
    assert_eq!(ta.cursor_position(), (8, 0));
    assert_eq!(rows(&ta, 5, 4), ["l08", "l09", "l10", "l11"]);

    press(&mut ta, Key::PageUp, 1);
    assert_eq!(ta.cursor_position(), (4, 0));
    assert_eq!(rows(&ta, 5, 4), ["l04", "l05", "l06", "l07"]);
}

#[test]
fn page_down_from_the_top_row_scrolls_a_full_page_and_keeps_the_cursor_row() {
    let mut ta = numbered();
    assert_eq!(rows(&ta, 5, 4), ["l00", "l01", "l02", "l03"]);

    // Cursor on screen row 0.
    press(&mut ta, Key::PageDown, 1);
    assert_eq!(ta.cursor_position(), (4, 0));
    assert_eq!(rows(&ta, 5, 4), ["l04", "l05", "l06", "l07"]);

    // Cursor on screen row 2 stays on row 2.
    let mut ta = numbered();
    ta.set_cursor(2, 0);
    rows(&ta, 5, 4);
    press(&mut ta, Key::PageDown, 1);
    assert_eq!(ta.cursor_position(), (6, 0));
    assert_eq!(rows(&ta, 5, 4), ["l04", "l05", "l06", "l07"]);
}

#[test]
fn repeated_page_down_reaches_the_end_then_lands_on_the_last_line() {
    let mut ta = numbered();
    ta.set_cursor(1, 0);
    rows(&ta, 5, 4);

    for (line, top) in [(5, 4), (9, 8), (13, 12), (17, 16)] {
        press(&mut ta, Key::PageDown, 1);
        assert_eq!(ta.cursor_position(), (line, 0));
        assert_eq!(rows(&ta, 5, 4)[0], format!("l{top:02}"));
    }

    // The view is at the end (the last line on the bottom row, never past
    // it), so the cursor goes to the last line.
    press(&mut ta, Key::PageDown, 1);
    assert_eq!(ta.cursor_position(), (19, 0));
    assert_eq!(rows(&ta, 5, 4), ["l16", "l17", "l18", "l19"]);
    press(&mut ta, Key::PageDown, 1);
    assert_eq!(ta.cursor_position(), (19, 0));
}

#[test]
fn page_down_scrolls_only_as_far_as_the_end() {
    // Ten lines in a 4-row view: the second page can only scroll 2 rows.
    let text: Vec<String> = (0..10).map(|i| format!("l{i:02}")).collect();
    let mut ta = TextArea::new()
        .content(text.join("\n"))
        .line_numbers(false)
        .wrap(false)
        .focused(true);
    ta.set_cursor(1, 0);
    rows(&ta, 5, 4);

    press(&mut ta, Key::PageDown, 1);
    assert_eq!(ta.cursor_position(), (5, 0));
    press(&mut ta, Key::PageDown, 1);
    assert_eq!(ta.cursor_position(), (7, 0));
    assert_eq!(rows(&ta, 5, 4), ["l06", "l07", "l08", "l09"]);
    press(&mut ta, Key::PageDown, 1);
    assert_eq!(ta.cursor_position(), (9, 0));
}

#[test]
fn repeated_page_up_reaches_the_top_then_lands_on_the_first_line() {
    let mut ta = numbered();
    ta.set_cursor(18, 0);
    assert_eq!(rows(&ta, 5, 4), ["l15", "l16", "l17", "l18"]);

    for (line, top) in [(14, 11), (10, 7), (6, 3), (3, 0)] {
        press(&mut ta, Key::PageUp, 1);
        assert_eq!(ta.cursor_position(), (line, 0));
        assert_eq!(rows(&ta, 5, 4)[0], format!("l{top:02}"));
    }

    press(&mut ta, Key::PageUp, 1);
    assert_eq!(ta.cursor_position(), (0, 0));
    assert_eq!(rows(&ta, 5, 4), ["l00", "l01", "l02", "l03"]);
}

#[test]
fn page_keys_keep_the_screen_column() {
    // Line 4 starts with wide glyphs: column 2 is the second one.
    let mut text: Vec<String> = (0..12).map(|i| format!("l{i:02}x")).collect();
    text[4] = "日本語".to_string();
    let mut ta = TextArea::new()
        .content(text.join("\n"))
        .line_numbers(false)
        .wrap(false)
        .focused(true);
    ta.set_cursor(0, 2);
    rows(&ta, 8, 4);

    press(&mut ta, Key::PageDown, 1);
    assert_eq!(ta.cursor_position(), (4, 1));
    press(&mut ta, Key::PageDown, 1);
    assert_eq!(ta.cursor_position(), (8, 2));
    press(&mut ta, Key::PageUp, 1);
    assert_eq!(ta.cursor_position(), (4, 1));
}

#[test]
fn page_keys_use_a_default_page_before_the_first_render() {
    let text: Vec<String> = (0..30).map(|i| format!("l{i:02}")).collect();
    let mut ta = TextArea::new()
        .content(text.join("\n"))
        .line_numbers(false)
        .wrap(false)
        .focused(true);
    press(&mut ta, Key::PageDown, 1);
    assert_eq!(ta.cursor_position(), (10, 0));
    press(&mut ta, Key::PageUp, 1);
    assert_eq!(ta.cursor_position(), (0, 0));
}

/// Wrapped at width 5, a 4-row view shows these rows:
/// `aaaaa aaaaa b c | ddddd ddddd e f | g h i j`.
fn wrapped() -> TextArea {
    TextArea::new()
        .content("aaaaaaaaaa\nb\nc\ndddddddddd\ne\nf\ng\nh\ni\nj")
        .line_numbers(false)
        .wrap(true)
        .focused(true)
}

#[test]
fn wrapped_lines_page_by_screen_rows() {
    let mut ta = wrapped();
    // Cursor on screen row 1: the second row of line 0, two columns in.
    ta.set_cursor(0, 7);
    assert_eq!(rows(&ta, 5, 4), ["aaaaa", "aaaaa", "b", "c"]);

    press(&mut ta, Key::PageDown, 1);
    assert_eq!(rows(&ta, 5, 4), ["ddddd", "ddddd", "e", "f"]);
    assert_eq!(ta.cursor_position(), (3, 7));

    // The last page: the view scrolls 4 rows and the cursor stays on row 1,
    // its column clamped to the one-char line.
    press(&mut ta, Key::PageDown, 1);
    assert_eq!(rows(&ta, 5, 4), ["g", "h", "i", "j"]);
    assert_eq!(ta.cursor_position(), (7, 1));

    press(&mut ta, Key::PageDown, 1);
    assert_eq!(ta.cursor_position(), (9, 1));

    // PageUp mirrors it: the cursor on row 3 stays on row 3.
    press(&mut ta, Key::PageUp, 1);
    assert_eq!(rows(&ta, 5, 4), ["ddddd", "ddddd", "e", "f"]);
    assert_eq!(ta.cursor_position(), (5, 1));

    press(&mut ta, Key::PageUp, 1);
    assert_eq!(rows(&ta, 5, 4), ["aaaaa", "aaaaa", "b", "c"]);
    assert_eq!(ta.cursor_position(), (2, 1));

    press(&mut ta, Key::PageUp, 1);
    assert_eq!(ta.cursor_position(), (0, 1));
}

#[test]
fn a_wrapped_line_across_the_bottom_edge_is_not_skipped() {
    // Line 2 takes rows 3 and 4, so only its first row shows; PageDown
    // brings it to the top instead of skipping its second row.
    let mut ta = TextArea::new()
        .content("aaaaaaaaaa\nb\ncccccddddd\ne\nf\ng\nh")
        .line_numbers(false)
        .wrap(true)
        .focused(true);
    assert_eq!(rows(&ta, 5, 4), ["aaaaa", "aaaaa", "b", "ccccc"]);

    press(&mut ta, Key::PageDown, 1);
    assert_eq!(rows(&ta, 5, 4), ["ccccc", "ddddd", "e", "f"]);
    assert_eq!(ta.cursor_position(), (2, 0));

    press(&mut ta, Key::PageUp, 1);
    assert_eq!(rows(&ta, 5, 4), ["aaaaa", "aaaaa", "b", "ccccc"]);
    assert_eq!(ta.cursor_position(), (0, 0));
}

#[test]
fn a_wrapped_line_taller_than_the_view_still_pages() {
    // Line 1 takes six rows of a four-row view.
    let mut ta = TextArea::new()
        .content("a\nbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\nc\nd")
        .line_numbers(false)
        .wrap(true)
        .focused(true);
    rows(&ta, 5, 4);
    for _ in 0..4 {
        press(&mut ta, Key::PageDown, 1);
        let (line, _) = ta.cursor_position();
        let shown = rows(&ta, 5, 4);
        let text = ["a", "bbbbb", "c", "d"][line];
        assert!(shown.contains(&text.to_string()), "{line}: {shown:?}");
    }
    assert_eq!(ta.cursor_position().0, 3);
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
