//! A table wider than its room fits it (#1472): the widest columns shrink
//! first and a cell's text wraps inside its column, so nothing is dropped.

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::widget::markdown::Markdown;
use revue::widget::traits::{RenderContext, View};

fn draw(md: &Markdown, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    md.render(&mut ctx);
    buffer
}

/// Each row's text with trailing blanks dropped; a wide character's second
/// cell is skipped.
fn rows(buffer: &Buffer) -> Vec<String> {
    (0..buffer.height())
        .map(|y| {
            let row: String = (0..buffer.width())
                .filter_map(|x| buffer.get(x, y))
                .filter(|cell| !cell.is_continuation())
                .map(|cell| cell.symbol)
                .collect();
            row.trim_end().to_string()
        })
        .collect()
}

fn shown(source: &str, w: u16, h: u16) -> Vec<String> {
    rows(&draw(&Markdown::new(source), w, h))
}

fn text_of(md: &Markdown, width: u16) -> Vec<String> {
    md.lines_at(width)
        .iter()
        .map(|line| line.segments.iter().map(|s| s.text.as_str()).collect())
        .collect()
}

const TWO_COLUMNS: &str = "| name | value |\n|---|---|\n| a | longer words here |";

/// The wide column shrinks, the narrow one keeps its width, and the right
/// border is on the last column
#[test]
fn a_wide_table_fits_the_width() {
    assert_eq!(
        shown(TWO_COLUMNS, 20, 8),
        [
            "┌──────┬───────────┐",
            "│ name │ value     │",
            "├──────┼───────────┤",
            "│ a    │ longer    │",
            "│      │ words     │",
            "│      │ here      │",
            "└──────┴───────────┘",
            "",
        ]
    );
}

/// At a width it fits, the table is drawn as wide as its cells
#[test]
fn a_table_that_fits_keeps_its_widths() {
    let md = Markdown::new(TWO_COLUMNS);
    assert_eq!(text_of(&md, 28), text_of(&md, 80));
    assert_eq!(text_of(&md, 28)[0], "┌──────┬───────────────────┐");
    assert_ne!(text_of(&md, 27), text_of(&md, 80));
}

/// The lines of a wrapped cell are its words, none lost
#[test]
fn a_wrapped_cell_keeps_every_word() {
    let rows = shown(TWO_COLUMNS, 20, 8);
    let words: Vec<&str> = rows[3..6]
        .iter()
        .flat_map(|row| row.split('│').nth(2).unwrap().split_whitespace())
        .collect();
    assert_eq!(words.join(" "), "longer words here");
}

/// Each line of a wrapped cell takes the column's alignment
#[test]
fn every_line_of_a_cell_is_aligned() {
    assert_eq!(
        shown("| a |\n|--:|\n| one two |", 9, 6),
        [
            "┌───────┐",
            "│     a │",
            "├───────┤",
            "│   one │",
            "│   two │",
            "└───────┘",
        ]
    );
}

/// Every line of a wrapped header cell is in the header style
#[test]
fn a_wrapped_header_stays_bold() {
    let md = Markdown::new("| long head | b |\n|---|---|\n| x | y |");
    let buffer = draw(&md, 12, 7);
    assert_eq!(
        rows(&buffer)[..3],
        ["┌──────┬───┐", "│ long │ b │", "│ head │   │"]
    );
    for y in 1..3 {
        let cell = buffer.get(2, y).unwrap();
        assert!(cell.modifier.contains(Modifier::BOLD), "row {y}");
    }
    assert!(!buffer.get(2, 4).unwrap().modifier.contains(Modifier::BOLD));
}

/// CJK breaks between characters and never inside one
#[test]
fn a_wide_character_cell_wraps_by_width() {
    assert_eq!(
        shown("| 가나다라 | b |\n|---|---|\n| x | y |", 12, 6),
        [
            "┌──────┬───┐",
            "│ 가나 │ b │",
            "│ 다라 │   │",
            "├──────┼───┤",
            "│ x    │ y │",
            "└──────┴───┘",
        ]
    );
}

/// A table in a list item fits the room left after the item's indent
#[test]
fn a_table_in_an_item_fits_the_room_after_the_indent() {
    assert_eq!(
        shown("- a\n\n  | x | y |\n  |---|---|\n  | one two | b |", 14, 7),
        [
            "• a",
            "  ┌──────┬───┐",
            "  │ x    │ y │",
            "  ├──────┼───┤",
            "  │ one  │ b │",
            "  │ two  │   │",
            "  └──────┴───┘",
        ]
    );
}

/// Too narrow for a column per character: drawn as wide as its cells and cut
#[test]
fn a_table_too_narrow_for_its_columns_is_cut() {
    let md = Markdown::new(TWO_COLUMNS);
    assert_eq!(text_of(&md, 8), text_of(&md, 80));
    assert_eq!(rows(&draw(&md, 8, 1)), ["┌──────┬"]);
}

/// The rows a wrapped table takes are counted at the width
#[test]
fn the_row_count_follows_the_wrapped_cells() {
    let md = Markdown::new(TWO_COLUMNS);
    assert_eq!(md.line_count(), 5);
    assert_eq!(md.line_count_at(20), 7);
}

/// A column of wide characters keeps two columns when the table is cut to
/// the bone, so its border stays in line
#[test]
fn a_wide_character_column_keeps_two_columns() {
    assert_eq!(
        shown("| abc | 가나 |\n|---|---|", 10, 5),
        [
            "┌───┬────┐",
            "│ a │ 가 │",
            "│ b │ 나 │",
            "│ c │    │",
            "└───┴────┘",
        ]
    );
}
