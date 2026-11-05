//! Blocks inside list items and quotes (#1475): every row drawn inside a
//! container starts with its prefix, after the prefixes of the containers
//! around it, and the block is laid out at the width less them.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::markdown::types::AdmonitionType;
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

/// The rows with no width limit
fn unwrapped(source: &str) -> Vec<String> {
    Markdown::new(source)
        .lines
        .iter()
        .map(|line| line.segments.iter().map(|s| s.text.as_str()).collect())
        .collect()
}

/// A tight item has no paragraph to end its text: the heading still starts
/// a row of its own, under the item's text (#1473)
#[test]
fn a_heading_in_a_tight_item_starts_its_own_row() {
    assert_eq!(unwrapped("- foo\n  # bar"), ["• foo", "  # bar"]);
    assert_eq!(
        unwrapped("1. foo\n   ## bar\n2. x"),
        ["1. foo", "   ## bar", "2. x"]
    );
    // It wraps beside its `#` and the item's indent
    assert_eq!(
        shown("- foo\n  # bar baz qux", 10, 4),
        ["• foo", "  # bar", "    baz", "    qux"]
    );
}

#[test]
fn text_after_a_heading_in_an_item_stays_in_the_item() {
    assert_eq!(
        unwrapped("- foo\n  # bar\n  baz"),
        ["• foo", "  # bar", "  baz"]
    );
}

#[test]
fn a_second_paragraph_lines_up_with_the_item_text() {
    assert_eq!(unwrapped("- foo\n\n  bar"), ["• foo", "  bar"]);
    assert_eq!(unwrapped("10. foo\n\n    bar"), ["10. foo", "    bar"]);
}

/// The loose nested item: no row of blanks before it, and its second
/// paragraph under its own text
#[test]
fn a_loose_nested_item_keeps_its_indent() {
    assert_eq!(unwrapped("- a\n  - b\n\n    c"), ["• a", "  • b", "    c"]);
}

/// A nested item lines up with its parent's text, whatever the parent's
/// marker is
#[test]
fn a_nested_item_lines_up_with_the_parent_text() {
    assert_eq!(
        unwrapped("1. a\n   - b\n   - c\n2. d"),
        ["1. a", "   • b", "   • c", "2. d"]
    );
    assert_eq!(unwrapped("- [x] a\n  - b"), ["[x] a", "    • b"]);
}

/// The box sits after the item's indent and fits the width left; its right
/// side is on the last column
#[test]
fn a_code_block_in_an_item_is_boxed_after_the_indent() {
    assert_eq!(
        shown("- foo\n  ```\n  abcdefgh\n  ```", 10, 5),
        [
            "• foo",
            "  ┌──────┐",
            "  │ abcd │",
            "  │ efgh │",
            "  └──────┘",
        ]
    );
}

#[test]
fn a_table_in_an_item_is_drawn_after_the_indent() {
    assert_eq!(
        unwrapped("- a\n\n  | x | y |\n  |---|---|\n  | 1 | 2 |"),
        [
            "• a",
            "  ┌───┬───┐",
            "  │ x │ y │",
            "  ├───┼───┤",
            "  │ 1 │ 2 │",
            "  └───┴───┘",
        ]
    );
}

#[test]
fn a_quote_in_an_item_is_drawn_after_the_indent() {
    assert_eq!(unwrapped("- foo\n  > q"), ["• foo", "  │ q"]);
    assert_eq!(
        unwrapped("- a\n\n  > q1\n  >\n  > q2"),
        ["• a", "  │ q1", "  │ q2"]
    );
}

/// The bar is the outer container, so it comes first; the item's text wraps
/// after both
#[test]
fn a_list_in_a_quote_draws_the_bar_first() {
    assert_eq!(unwrapped("> - a\n> - b"), ["│ • a", "│ • b"]);
    assert_eq!(
        shown("> - one two six", 8, 3),
        ["│ • one", "│   two", "│   six"]
    );
}

/// An item holding a quote: the bullet on the first row, blanks under it on
/// the rest, the bar on every row
#[test]
fn a_quote_in_an_item_wraps_after_both_prefixes() {
    assert_eq!(
        shown("- > words here now", 10, 3),
        ["• │ words", "  │ here", "  │ now"]
    );
}

#[test]
fn a_nested_quote_draws_a_bar_per_level() {
    assert_eq!(unwrapped("> > inner"), ["│ │ inner"]);
    assert_eq!(
        shown("> outer\n>\n> > in one two", 9, 4),
        ["│ outer", "│ │ in", "│ │ one", "│ │ two"]
    );
}

#[test]
fn a_heading_and_code_in_a_quote_keep_the_bar() {
    assert_eq!(unwrapped("> # h\n> text"), ["│ # h", "│ text"]);
    assert_eq!(
        unwrapped("> ```\n> code\n> ```"),
        ["│ ┌──────┐", "│ │ code │", "│ └──────┘"]
    );
}

/// A callout's title is inside its quote, and the bar takes the callout's
/// color on every row
#[test]
fn a_callout_bar_is_in_the_callout_color() {
    let md = Markdown::new("> [!NOTE]\n> body text here");
    let buffer = draw(&md, 12, 3);
    let rows = rows(&buffer);
    assert!(rows[0].starts_with("│ ℹ"), "{rows:?}");
    assert_eq!(rows[1..], ["│ body text", "│ here"]);
    for y in 0..3 {
        let bar = buffer.get(0, y).unwrap();
        assert_eq!(bar.symbol, '│');
        assert_eq!(bar.fg, Some(AdmonitionType::Note.color()), "row {y}");
    }
    let plain = draw(&Markdown::new("> plain"), 12, 1);
    assert_ne!(
        plain.get(0, 0).unwrap().fg,
        Some(AdmonitionType::Note.color())
    );
}

/// An item with nothing in it still shows its bullet
#[test]
fn an_empty_item_shows_its_bullet() {
    assert_eq!(unwrapped("-\n- a"), ["• ", "• a"]);
}

/// Prefixes wider than the area are cut like any row; nothing panics
#[test]
fn prefixes_wider_than_the_area_cut() {
    assert_eq!(shown("> > > a", 3, 1), ["│ │"]);
    assert_eq!(shown("- - - a", 1, 1), ["•"]);
}
