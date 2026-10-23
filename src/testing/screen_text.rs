//! A buffer read back as text, the way the screen shows it: a wide glyph
//! is one character (its continuation cell is left out), and a position
//! found in the text is a column on the screen

use crate::render::Buffer;

/// Row `y` as text, trailing spaces trimmed; empty past the last row
pub(crate) fn row_text(buffer: &Buffer, y: u16) -> String {
    row_with_columns(buffer, y).0.trim_end().to_string()
}

/// Every row as text, joined by newlines, with trailing blank rows dropped
pub(crate) fn screen_text(buffer: &Buffer) -> String {
    let mut lines: Vec<String> = (0..buffer.height()).map(|y| row_text(buffer, y)).collect();
    while lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }
    lines.join("\n")
}

/// The column and row where `needle` first appears, top to bottom
pub(crate) fn find(buffer: &Buffer, needle: &str) -> Option<(u16, u16)> {
    (0..buffer.height()).find_map(|y| {
        let (text, columns) = row_with_columns(buffer, y);
        let at = text.find(needle)?;
        // The byte offset counts the characters before it; each has a column
        let index = text[..at].chars().count();
        Some((columns[index], y))
    })
}

/// Row `y` as text, and the column each of its characters starts at
fn row_with_columns(buffer: &Buffer, y: u16) -> (String, Vec<u16>) {
    let mut text = String::new();
    let mut columns = Vec::new();
    if y < buffer.height() {
        for x in 0..buffer.width() {
            match buffer.get(x, y) {
                Some(cell) if cell.is_continuation() => {}
                Some(cell) => {
                    text.push(cell.symbol);
                    columns.push(x);
                }
                None => {
                    text.push(' ');
                    columns.push(x);
                }
            }
        }
    }
    (text, columns)
}
