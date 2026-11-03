//! The controls command output uses on its own line (#1423): backspace,
//! erase in line, cursor moves and a column. Whole-screen commands are
//! dropped: the lines are a scrollback, not a screen grid (#1461).

use revue::style::Color;
use revue::widget::Terminal;

use super::{row, screen};

fn rows(term: &Terminal, width: u16, height: u16) -> Vec<String> {
    let buffer = screen(term, width, height);
    (0..height).map(|y| row(&buffer, y)).collect()
}

/// The issue's case: a backspace takes the column back, and a progress
/// line rewritten after `\r` erases what it does not cover
#[test]
fn backspace_and_erase_rewrite_a_line() {
    let mut t = Terminal::new(10, 2);
    t.write("abc\x08d\n50%\r\x1b[K9%");
    assert_eq!(rows(&t, 10, 2), ["abd       ", "9%        "]);
}

/// A backspace at the first column stays there; it never goes back to the
/// line above, as in xterm without reverse wrap
#[test]
fn backspace_stops_at_the_first_column() {
    let mut t = Terminal::new(6, 2);
    t.write("ab\n\x08\x08x");
    assert_eq!(rows(&t, 6, 2), ["ab    ", "x     "]);
}

/// A spinner: backspace and the next frame over the last one
#[test]
fn a_spinner_draws_each_frame_in_place() {
    let mut t = Terminal::new(8, 1);
    t.write("run -");
    for frame in ['\\', '|', '/'] {
        t.write(&format!("\x08{frame}"));
    }
    assert_eq!(rows(&t, 8, 1), ["run /   "]);
}

/// The three erase-in-line forms: to the end, to the cursor (inclusive),
/// the whole line; the cursor does not move
#[test]
fn each_erase_in_line_form_erases_its_part() {
    let cases = [
        ("\x1b[K", "ab      "),
        ("\x1b[0K", "ab      "),
        ("\x1b[1K", "   def  "),
        ("\x1b[2K", "        "),
    ];
    for (seq, want) in cases {
        let mut t = Terminal::new(8, 1);
        t.write("abcdef\r\x1b[2C");
        t.write(seq);
        assert_eq!(rows(&t, 8, 1)[0], want, "{seq:?}");
        // the cursor is still on column 2
        t.write("Z");
        assert_eq!(rows(&t, 8, 1)[0].chars().nth(2), Some('Z'), "{seq:?}");
    }
}

/// Erased cells take the background in effect, as in xterm: to the edge
/// for `K`, even past where the line's text ended
#[test]
fn erased_cells_take_the_background_in_effect() {
    let mut t = Terminal::new(6, 1);
    t.write("ab\r\x1b[44m\x1b[K");
    let buffer = screen(&t, 6, 1);
    for x in 0..6 {
        assert_eq!(buffer.get(x, 0).unwrap().bg, Some(Color::BLUE), "{x}");
        assert_eq!(buffer.get(x, 0).unwrap().symbol, ' ', "{x}");
    }
}

/// A multi-line progress display: up a line, erase it, write it again, and
/// back down to where it was
#[test]
fn up_erase_and_down_redraw_lines_above() {
    let mut t = Terminal::new(8, 3);
    t.write("a 10%\nb 20%\n");
    t.write("\x1b[2A\x1b[2K\ra 99%\x1b[1B\r\x1b[2Kb 98%\x1b[1B\r");
    t.write("done");
    assert_eq!(rows(&t, 8, 3), ["a 99%   ", "b 98%   ", "done    "]);
}

/// Up stops at the screen's top row, never in the scrollback above it, and
/// down at the last line
#[test]
fn up_and_down_stop_at_the_screen() {
    let mut t = Terminal::new(4, 2).max_scrollback(10);
    t.write("1\n2\n3\n4");
    // the screen is lines "3" and "4"; far up lands on "3"
    t.write("\x1b[99A\rX\x1b[99B\rY");
    assert_eq!(rows(&t, 4, 4), ["1   ", "2   ", "X   ", "Y   "]);
}

/// Right stops at the last column, left at the first; a column counts from
/// 1, and 0 or none is 1
#[test]
fn moves_and_columns_stay_within_the_width() {
    let mut t = Terminal::new(5, 1);
    t.write("\x1b[99CR\r\x1b[99DL\x1b[3GM\x1b[0GN\x1b[GO\x1b[99GP");
    // R at 4, then N at 0 over L, O at 0 again, P at 4 over R
    assert_eq!(rows(&t, 5, 1), ["O M P"]);
}

/// Whole-screen commands and private or intermediate forms change nothing
#[test]
fn whole_screen_and_private_forms_are_dropped() {
    let mut t = Terminal::new(6, 2);
    t.write("ab\ncd");
    t.write("\x1b[2J\x1b[1;1H\x1b[?1A\x1b[1 D\x1b[3K");
    t.write("e");
    assert_eq!(rows(&t, 6, 2), ["ab    ", "cde   "]);
}

/// After a full line (the wrap pending, #1462) a backspace goes to the
/// column before the last, and a move or an erase ends the wrap: the next
/// char goes on the cursor's column, not the next line
#[test]
fn controls_after_a_full_line_end_the_pending_wrap() {
    let cases = [
        ("\x08X", "abXd"),
        ("\x1b[DX", "abXd"),
        ("\x1b[CX", "abcX"),
        ("\x1b[KX", "abcX"),
        ("\x1b[1KX", "   X"),
        ("\x1b[2KX", "   X"),
        ("\x1b[2GX", "aXcd"),
    ];
    for (seq, want) in cases {
        let mut t = Terminal::new(4, 2);
        t.write("abcd");
        t.write(seq);
        assert_eq!(rows(&t, 4, 2), [want, "    "], "{seq:?}");
    }
}
