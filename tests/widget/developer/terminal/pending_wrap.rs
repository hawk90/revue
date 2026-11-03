//! Filling the last column leaves the wrap pending, as in xterm (#1462):
//! the next printable character wraps; a newline or return before it acts
//! on the same line.

use revue::widget::Terminal;

use super::{row, screen};

fn rows(term: &Terminal, width: u16, height: u16) -> Vec<String> {
    let buffer = screen(term, width, height);
    (0..height).map(|y| row(&buffer, y)).collect()
}

/// The issue's case: a full line then a newline leaves no blank line
#[test]
fn a_newline_after_a_full_line_goes_to_the_next_line() {
    let mut t = Terminal::new(3, 3);
    t.write("abc\nde");
    assert_eq!(rows(&t, 3, 3), ["abc", "de ", "   "]);
}

/// A return after a full line goes back to its own start, so a rewrite
/// lands on it
#[test]
fn a_return_after_a_full_line_stays_on_it() {
    let mut t = Terminal::new(4, 2);
    t.write("1234\rab");
    assert_eq!(rows(&t, 4, 2), ["ab34", "    "]);
}

/// The next printable character wraps: it starts the next line
#[test]
fn the_next_character_wraps() {
    let mut t = Terminal::new(3, 2);
    t.write("abcd");
    assert_eq!(rows(&t, 3, 2), ["abc", "d  "]);
    // and exactly two full lines take two rows, not three
    let mut t = Terminal::new(3, 3);
    t.write("abcdef\nx");
    assert_eq!(rows(&t, 3, 3), ["abc", "def", "x  "]);
}

/// A wide character that fills the last two columns leaves the wrap
/// pending too
#[test]
fn a_wide_character_on_the_last_two_columns_leaves_the_wrap_pending() {
    let mut t = Terminal::new(4, 3);
    t.write("ab世\ncd");
    let buffer = screen(&t, 4, 3);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '世');
    assert_eq!(row(&buffer, 1), "cd  ");
    assert_eq!(row(&buffer, 2), "    ");
}

/// While the wrap is pending the cursor is drawn on the last column
#[test]
fn the_cursor_shows_on_the_last_column_while_the_wrap_is_pending() {
    let mut t = Terminal::new(3, 2);
    t.focus();
    t.write("ab");
    t.write("c");
    let buffer = screen(&t, 3, 2);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '█');
    assert_eq!(row(&buffer, 1), "   ");
}
