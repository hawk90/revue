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
fn page_down_and_page_up_keep_the_cursor_in_view() {
    let mut ed = numbered();
    rows(&ed, 5, 4);

    press(&mut ed, Key::PageDown, 1);
    let (line, _) = ed.cursor_position();
    assert!(line >= 4, "PageDown moved past the first page");
    let expected: Vec<String> = (line - 3..=line).map(|i| format!("l{i:02}")).collect();
    assert_eq!(rows(&ed, 5, 4), expected);

    press(&mut ed, Key::PageUp, 1);
    assert_eq!(ed.cursor_position(), (0, 0));
    assert_eq!(rows(&ed, 5, 4), ["l00", "l01", "l02", "l03"]);
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
