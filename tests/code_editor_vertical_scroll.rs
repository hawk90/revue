//! CodeEditor scrolls vertically to keep the cursor in view.
//!
//! The vertical offset only ever moved up (when the cursor went above it), so
//! moving the cursor below the last visible row, paging down, pressing Enter
//! on the last row or jumping to a line left the cursor off screen. The
//! offset is now settled at render time, where the viewport height is known,
//! the same way the horizontal scroll is.

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::CodeEditor;

/// The visible rows, trailing blanks trimmed.
fn rows(ed: &CodeEditor, w: u16, h: u16) -> Vec<String> {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    ed.render(&mut ctx);
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

fn editor(content: &str) -> CodeEditor {
    CodeEditor::new()
        .content(content)
        .line_numbers(false)
        .highlight_current_line(false)
        .bracket_matching(false)
}

/// Forty lines `l00` to `l39`.
fn numbered() -> CodeEditor {
    let text: Vec<String> = (0..40).map(|i| format!("l{i:02}")).collect();
    editor(&text.join("\n"))
}

fn press(ed: &mut CodeEditor, key: Key, times: usize) {
    for _ in 0..times {
        ed.handle_key(&key);
    }
}

#[test]
fn moving_down_past_the_bottom_scrolls_by_as_little_as_possible() {
    let mut ed = numbered();
    assert_eq!(rows(&ed, 5, 3), ["l00", "l01", "l02"]);

    press(&mut ed, Key::Down, 3);
    assert_eq!(ed.cursor_position(), (3, 0));
    assert_eq!(rows(&ed, 5, 3), ["l01", "l02", "l03"]);

    press(&mut ed, Key::Down, 4);
    assert_eq!(rows(&ed, 5, 3), ["l05", "l06", "l07"]);
}

#[test]
fn moving_back_up_inside_the_view_does_not_scroll() {
    let mut ed = numbered();
    press(&mut ed, Key::Down, 7);
    assert_eq!(rows(&ed, 5, 3), ["l05", "l06", "l07"]);

    press(&mut ed, Key::Up, 2);
    assert_eq!(rows(&ed, 5, 3), ["l05", "l06", "l07"]);
}

#[test]
fn moving_up_past_the_top_scrolls_up() {
    let mut ed = numbered();
    press(&mut ed, Key::Down, 10);
    assert_eq!(rows(&ed, 5, 3), ["l08", "l09", "l10"]);

    press(&mut ed, Key::Up, 4);
    assert_eq!(ed.cursor_position(), (6, 0));
    assert_eq!(rows(&ed, 5, 3), ["l06", "l07", "l08"]);
}

#[test]
fn page_down_and_page_up_scroll_the_view_by_a_page() {
    let mut ed = numbered();
    rows(&ed, 5, 4);

    press(&mut ed, Key::PageDown, 1);
    assert_eq!(ed.cursor_position(), (4, 0));
    assert_eq!(rows(&ed, 5, 4), ["l04", "l05", "l06", "l07"]);

    press(&mut ed, Key::PageUp, 1);
    assert_eq!(ed.cursor_position(), (0, 0));
    assert_eq!(rows(&ed, 5, 4), ["l00", "l01", "l02", "l03"]);
}

#[test]
fn page_down_from_the_top_row_scrolls_a_full_page_and_keeps_the_cursor_row() {
    let mut ed = numbered();
    ed.set_cursor(2, 0);
    assert_eq!(rows(&ed, 5, 4), ["l00", "l01", "l02", "l03"]);

    press(&mut ed, Key::PageDown, 1);
    assert_eq!(ed.cursor_position(), (6, 0));
    assert_eq!(rows(&ed, 5, 4), ["l04", "l05", "l06", "l07"]);
}

#[test]
fn repeated_page_down_reaches_the_end_then_lands_on_the_last_line() {
    let mut ed = numbered();
    ed.set_cursor(3, 0);
    rows(&ed, 5, 8);

    for (line, top) in [(11, 8), (19, 16), (27, 24), (35, 32)] {
        press(&mut ed, Key::PageDown, 1);
        assert_eq!(ed.cursor_position(), (line, 0));
        assert_eq!(rows(&ed, 5, 8)[0], format!("l{top:02}"));
    }

    // The last line is on the bottom row (the view never scrolls past it),
    // so the cursor goes to the last line.
    press(&mut ed, Key::PageDown, 1);
    assert_eq!(ed.cursor_position(), (39, 0));
    assert_eq!(rows(&ed, 5, 8)[0], "l32");
    press(&mut ed, Key::PageDown, 1);
    assert_eq!(ed.cursor_position(), (39, 0));
}

#[test]
fn repeated_page_up_reaches_the_top_then_lands_on_the_first_line() {
    let mut ed = numbered();
    ed.set_cursor(36, 0);
    assert_eq!(rows(&ed, 5, 8)[0], "l29");

    for (line, top) in [(28, 21), (20, 13), (12, 5), (7, 0)] {
        press(&mut ed, Key::PageUp, 1);
        assert_eq!(ed.cursor_position(), (line, 0));
        assert_eq!(rows(&ed, 5, 8)[0], format!("l{top:02}"));
    }

    press(&mut ed, Key::PageUp, 1);
    assert_eq!(ed.cursor_position(), (0, 0));
    assert_eq!(rows(&ed, 5, 8)[0], "l00");
}

#[test]
fn page_keys_keep_the_screen_column() {
    // Line 4 starts with wide glyphs: column 2 is the second one.
    let mut text: Vec<String> = (0..12).map(|i| format!("l{i:02}x")).collect();
    text[4] = "日本語".to_string();
    let mut ed = editor(&text.join("\n"));
    ed.set_cursor(0, 2);
    rows(&ed, 8, 4);

    press(&mut ed, Key::PageDown, 1);
    assert_eq!(ed.cursor_position(), (4, 1));
    press(&mut ed, Key::PageDown, 1);
    assert_eq!(ed.cursor_position(), (8, 2));
    press(&mut ed, Key::PageUp, 1);
    assert_eq!(ed.cursor_position(), (4, 1));
}

#[test]
fn page_keys_extend_a_selection_in_selection_mode() {
    let mut ed = numbered();
    rows(&ed, 5, 4);
    ed.start_selection();
    press(&mut ed, Key::PageDown, 1);
    assert_eq!(ed.get_selection().as_deref(), Some("l00\nl01\nl02\nl03\n"));
    press(&mut ed, Key::PageUp, 1);
    assert_eq!(ed.cursor_position(), (0, 0));
}

#[test]
fn paging_never_hides_the_cursor_under_the_find_box() {
    let mut ed = long();
    // Learn the full 4-row page, then page so the cursor is on the bottom row.
    ed.set_cursor(3, 0);
    rows(&ed, 30, 4);
    press(&mut ed, Key::PageDown, 1);
    assert_eq!(ed.cursor_position(), (7, 0));

    // The box takes row 0 and the view shrinks to 3 rows.
    ed.open_find();
    let shown = rows(&ed, 30, 4);
    assert!(shown[0].starts_with("Find:"), "{shown:?}");
    assert!(shown[1..].contains(&"l07".to_string()), "{shown:?}");

    // Paging while the box is open, by the full or the shrunk page.
    for page in [4, 3, 4, 3] {
        ed.page_down(page);
        let line = format!("l{:02}", ed.cursor_position().0);
        let shown = rows(&ed, 30, 4);
        assert!(shown[1..].contains(&line), "{line}: {shown:?}");
    }
    for page in [4, 3, 4, 3] {
        ed.page_up(page);
        let line = format!("l{:02}", ed.cursor_position().0);
        let shown = rows(&ed, 30, 4);
        assert!(shown[1..].contains(&line), "{line}: {shown:?}");
    }

    // Closed again, the page learned with the box open (3 rows) still
    // keeps the cursor in view.
    ed.handle_key(&Key::Escape);
    for _ in 0..3 {
        press(&mut ed, Key::PageDown, 1);
        let line = format!("l{:02}", ed.cursor_position().0);
        assert!(rows(&ed, 30, 4).contains(&line), "{line}");
    }
}

#[test]
fn enter_on_the_last_visible_row_scrolls_to_the_new_line() {
    let mut ed = editor("a\nb\nc");
    ed.set_cursor(2, 1);
    assert_eq!(rows(&ed, 5, 3), ["a", "b", "c"]);

    ed.handle_key(&Key::Enter);
    ed.handle_key(&Key::Char('d'));
    assert_eq!(ed.cursor_position(), (3, 1));
    assert_eq!(rows(&ed, 5, 3), ["b", "c", "d"]);
}

#[test]
fn goto_line_shows_the_target_line() {
    let mut ed = numbered();
    // Lines are 1-based in the go-to-line dialog.
    ed.goto_line(31);
    assert_eq!(ed.cursor_position(), (30, 0));
    assert_eq!(rows(&ed, 5, 3), ["l28", "l29", "l30"]);
}

/// A hundred lines `l00` to `l99`.
fn long() -> CodeEditor {
    let text: Vec<String> = (0..100).map(|i| format!("l{i:02}")).collect();
    editor(&text.join("\n"))
}

#[test]
fn page_keys_move_by_the_visible_height() {
    for h in [3u16, 5, 8, 13] {
        let mut ed = long();
        rows(&ed, 5, h);
        let page = h as usize;

        press(&mut ed, Key::PageDown, 1);
        assert_eq!(ed.cursor_position().0, page, "height {h}");
        rows(&ed, 5, h);
        press(&mut ed, Key::PageDown, 1);
        assert_eq!(ed.cursor_position().0, 2 * page, "height {h}");
        let shown = rows(&ed, 5, h);
        assert!(shown.contains(&format!("l{:02}", 2 * page)), "height {h}");

        press(&mut ed, Key::PageUp, 1);
        assert_eq!(ed.cursor_position().0, page, "height {h}");
        let shown = rows(&ed, 5, h);
        assert!(shown.contains(&format!("l{page:02}")), "height {h}");
    }
}

#[test]
fn page_keys_follow_a_resize() {
    let mut ed = long();
    rows(&ed, 5, 10);
    press(&mut ed, Key::PageDown, 1);
    assert_eq!(ed.cursor_position().0, 10);

    rows(&ed, 5, 4);
    press(&mut ed, Key::PageDown, 1);
    assert_eq!(ed.cursor_position().0, 14);
}

#[test]
fn page_keys_clamp_at_the_buffer_edges() {
    let mut ed = long();
    rows(&ed, 5, 8);
    ed.set_cursor(95, 0);
    rows(&ed, 5, 8);
    press(&mut ed, Key::PageDown, 1);
    assert_eq!(ed.cursor_position().0, 99);
    assert_eq!(rows(&ed, 5, 8).last().map(String::as_str), Some("l99"));

    ed.set_cursor(3, 0);
    rows(&ed, 5, 8);
    press(&mut ed, Key::PageUp, 1);
    assert_eq!(ed.cursor_position().0, 0);
    assert_eq!(rows(&ed, 5, 8)[0], "l00");
}

// The find (30 columns) and go-to-line (20 columns) boxes are drawn on row 0.
// The views below are exactly as wide as the box, so it covers the row.

#[test]
fn a_find_match_on_the_top_row_is_not_hidden_under_the_find_box() {
    let mut ed = long();
    ed.set_cursor(20, 0);
    assert_eq!(rows(&ed, 30, 4), ["l17", "l18", "l19", "l20"]);

    ed.open_find();
    ed.set_find_query("l17");
    ed.handle_key(&Key::Enter);
    assert_eq!(ed.cursor_position(), (17, 0));
    let shown = rows(&ed, 30, 4);
    assert!(shown[0].starts_with("Find:"), "{shown:?}");
    assert!(shown[1..].contains(&"l17".to_string()), "{shown:?}");
}

#[test]
fn the_first_line_shows_below_the_find_box() {
    let mut ed = long();
    ed.open_find();
    let shown = rows(&ed, 30, 4);
    assert!(shown[0].starts_with("Find:"), "{shown:?}");
    assert_eq!(shown[1..], ["l00", "l01", "l02"]);

    ed.handle_key(&Key::Escape);
    assert_eq!(rows(&ed, 30, 4), ["l00", "l01", "l02", "l03"]);
}

#[test]
fn the_cursor_line_shows_below_the_goto_line_box() {
    let mut ed = long();
    ed.set_cursor(20, 0);
    rows(&ed, 20, 4);
    ed.set_cursor(17, 0);
    assert_eq!(rows(&ed, 20, 4)[0], "l17");

    ed.open_goto_line();
    let shown = rows(&ed, 20, 4);
    assert!(shown[0].starts_with("Go to line:"), "{shown:?}");
    assert!(shown[1..].contains(&"l17".to_string()), "{shown:?}");
}

#[test]
fn page_keys_use_a_default_page_before_the_first_render() {
    let mut ed = long();
    press(&mut ed, Key::PageDown, 1);
    assert_eq!(ed.cursor_position().0, 20);
    press(&mut ed, Key::PageUp, 1);
    assert_eq!(ed.cursor_position().0, 0);
}
