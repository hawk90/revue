//! Markdown laid out at the width it is drawn at (#951): prose wraps at word
//! boundaries beside its markers, code wraps inside its box; tables fit the
//! width in table.rs.

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

#[test]
fn a_paragraph_wraps_at_word_boundaries() {
    assert_eq!(
        shown("one two three four five", 9, 4),
        ["one two", "three", "four five", ""]
    );
}

/// A word that ends on the last column stays on its row
#[test]
fn a_row_fills_to_the_last_column() {
    assert_eq!(shown("abcd efgh ij", 9, 2), ["abcd efgh", "ij"]);
}

/// The same source at a width it fits is drawn as before: one row per line
#[test]
fn text_that_fits_is_not_rewrapped() {
    let md = Markdown::new("short\n\n- item");
    assert_eq!(text_of(&md, 20), ["short", "• item"]);
    assert_eq!(md.line_count_at(20), md.line_count());
}

#[test]
fn a_word_wider_than_the_area_is_broken() {
    assert_eq!(shown("abcdefghij", 4, 3), ["abcd", "efgh", "ij"]);
}

#[test]
fn a_list_item_goes_on_after_its_bullet() {
    assert_eq!(
        shown("- alpha beta gamma", 12, 2),
        ["• alpha beta", "  gamma"]
    );
}

#[test]
fn a_numbered_item_goes_on_after_its_number() {
    let source = "10. alpha beta gamma";
    assert_eq!(shown(source, 14, 2), ["10. alpha beta", "    gamma"]);
}

#[test]
fn a_nested_item_keeps_its_indent_and_bullet_width() {
    let source = "- top\n  - inner words here";
    assert_eq!(
        shown(source, 12, 4),
        ["• top", "  • inner", "    words", "    here"]
    );
}

#[test]
fn a_task_item_goes_on_after_its_box() {
    assert_eq!(
        shown("- [x] done with this", 14, 2),
        ["[x] done with", "    this"]
    );
}

/// Every row of a quote starts with the bar, in the quote's style
#[test]
fn every_row_of_a_quote_starts_with_the_bar() {
    let md = Markdown::new("> quoted words go on");
    let buffer = draw(&md, 10, 3);
    assert_eq!(rows(&buffer), ["│ quoted", "│ words go", "│ on"]);
    let bar = |y: u16| *buffer.get(0, y).unwrap();
    assert_eq!(bar(1).symbol, '│');
    assert_eq!(bar(1).fg, bar(0).fg);
    assert_eq!(bar(1).modifier, bar(0).modifier);
}

#[test]
fn a_heading_goes_on_after_its_marker_in_bold() {
    let md = Markdown::new("## long heading text");
    let buffer = draw(&md, 10, 3);
    assert_eq!(rows(&buffer), ["## long", "   heading", "   text"]);
    let cell = buffer.get(3, 1).unwrap();
    assert_eq!(cell.symbol, 'h');
    assert!(cell.modifier.contains(Modifier::BOLD));
}

/// Emphasis moved to the next row keeps its style
#[test]
fn styles_go_with_their_words_onto_the_next_row() {
    let md = Markdown::new("plain **bold words**");
    let buffer = draw(&md, 10, 2);
    assert_eq!(rows(&buffer), ["plain bold", "words"]);
    assert!(!buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
    assert!(buffer.get(6, 0).unwrap().modifier.contains(Modifier::BOLD));
    assert!(buffer.get(0, 1).unwrap().modifier.contains(Modifier::BOLD));
}

/// Two columns a character, broken between characters, never inside one
#[test]
fn cjk_wraps_by_width() {
    assert_eq!(shown("가나다라마", 5, 3), ["가나", "다라", "마"]);
}

#[test]
fn a_code_line_goes_on_inside_the_box() {
    let source = "```\nlet value = compute(1, 2);\n```";
    let rows = shown(source, 14, 6);
    assert_eq!(
        rows,
        [
            "┌────────────┐",
            "│ let value  │",
            "│ = compute( │",
            "│ 1, 2);     │",
            "└────────────┘",
            "",
        ]
    );
    // Nothing of the code is dropped, spaces included
    let code: String = rows[1..4].iter().map(|r| &r[4..r.len() - 4]).collect();
    assert_eq!(code.trim_end(), "let value = compute(1, 2);");
}

#[test]
fn a_wrapped_code_line_leaves_the_number_column_blank() {
    let md = Markdown::new("```\nabcdefghij\n```").code_line_numbers(true);
    assert_eq!(
        rows(&draw(&md, 14, 5)),
        [
            "┌────────────┐",
            "│   1 │ abcd │",
            "│     │ efgh │",
            "│     │ ij   │",
            "└────────────┘",
        ]
    );
}

#[test]
fn a_code_block_without_a_border_wraps_to_the_area() {
    let md = Markdown::new("```\nabcdefgh\n```").code_border(false);
    assert_eq!(text_of(&md, 3), ["abc", "def", "gh"]);
}

#[test]
fn a_footnote_goes_on_after_its_number() {
    let md = Markdown::new("see[^n]\n\n[^n]: a long note here");
    let text = text_of(&md, 12);
    let start = text.iter().position(|row| row.starts_with("[1]")).unwrap();
    assert_eq!(text[start..], ["[1] a long", "    note", "    here"]);
}

#[test]
fn a_table_of_contents_entry_goes_on_after_its_dash() {
    let md = Markdown::new("# First heading words").show_toc(true);
    let text = text_of(&md, 10);
    // The title wraps too
    assert_eq!(text[..2], ["Table of", "Contents"]);
    assert_eq!(text[2..5], ["- First", "  heading", "  words"]);
}

/// Narrower than a list's markers: the text is cut, nothing panics
#[test]
fn an_area_narrower_than_the_markers_cuts() {
    assert_eq!(shown("- abc", 1, 2), ["•", ""]);
    assert_eq!(shown("> words here", 2, 1), ["│"]);
    assert_eq!(shown("```\nabc\n```", 1, 1), ["┌"]);
}

#[test]
fn the_wrapped_row_count_is_what_is_drawn() {
    let md = Markdown::new("one two three four five");
    assert_eq!(md.line_count(), 1);
    assert_eq!(md.line_count_at(9), 3);
    assert_eq!(md.line_count_at(0), 1, "no width: no wrapping");
}
