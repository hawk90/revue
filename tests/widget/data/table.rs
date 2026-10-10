//! Public API tests for the Table widget
//!
//! Table keeps its columns, colors, border and scroll settings private, so
//! those are checked through what `render()` draws.

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::data::{column, table, Column, Table};
use revue::widget::traits::{RenderContext, View};

fn render(t: &Table, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
    buffer
}

fn row_text(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

fn numbered(n: usize) -> Vec<Vec<String>> {
    (0..n).map(|i| vec![format!("{}", i)]).collect()
}

// =========================================================================
// Constructors and helpers
// =========================================================================

#[test]
fn test_table_new() {
    let t = Table::new(vec![
        Column::new("Name").width(6),
        Column::new("Age").width(3),
    ]);
    assert_eq!(t.row_count(), 0);
    assert_eq!(t.selected_index(), 0);

    let buffer = render(&t, 20, 5);
    assert_eq!(row_text(&buffer, 0), "┌──────┬───┐");
    assert_eq!(row_text(&buffer, 1), "│Name  │Age│");
    assert_eq!(row_text(&buffer, 2), "├──────┼───┤");
    assert_eq!(row_text(&buffer, 3), "└──────┴───┘");
}

#[test]
fn test_table_with_rows() {
    let t = Table::new(vec![Column::new("A").width(1), Column::new("B").width(1)])
        .row(vec!["1", "2"])
        .row(vec!["3", "4"]);

    assert_eq!(t.row_count(), 2);
    let buffer = render(&t, 10, 7);
    assert_eq!(row_text(&buffer, 3), "│1│2│");
    assert_eq!(row_text(&buffer, 4), "│3│4│");
    assert_eq!(row_text(&buffer, 5), "└─┴─┘");
}

#[test]
fn test_table_helpers() {
    let t = table(vec![column("A").width(1), column("B").width(1)]).row(vec!["1", "2"]);
    assert_eq!(t.row_count(), 1);
    assert_eq!(row_text(&render(&t, 10, 5), 1), "│A│B│");
}

#[test]
fn test_table_helper_empty_renders_nothing() {
    let t = table(vec![]).row(vec!["ignored"]);
    let buffer = render(&t, 10, 5);
    for y in 0..5 {
        assert_eq!(row_text(&buffer, y), "", "row {y}");
    }
}

#[test]
fn test_table_with_many_columns() {
    let cols: Vec<Column> = (0..10)
        .map(|i| Column::new(format!("{}", i)).width(1))
        .collect();
    let t = Table::new(cols);
    assert_eq!(row_text(&render(&t, 30, 4), 1), "│0│1│2│3│4│5│6│7│8│9│");
}

#[test]
fn test_table_default() {
    let t = Table::default();
    assert_eq!(t.row_count(), 0);
    assert_eq!(t.selected_index(), 0);
    // No columns: nothing is drawn
    let buffer = render(&t, 10, 4);
    for y in 0..4 {
        assert_eq!(row_text(&buffer, y), "", "row {y}");
    }
}

#[test]
fn test_table_default_style() {
    // Border on, white bold header without background, white-on-blue selection
    let t = Table::new(vec![Column::new("X").width(1)])
        .row(vec!["a"])
        .row(vec!["b"]);
    let buffer = render(&t, 10, 6);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');

    let header = buffer.get(1, 1).unwrap();
    assert_eq!(header.symbol, 'X');
    assert_eq!(header.fg, Some(Color::WHITE));
    assert_eq!(header.bg, None);
    assert!(header.modifier.contains(Modifier::BOLD));

    let selected = buffer.get(1, 3).unwrap();
    assert_eq!(selected.symbol, 'a');
    assert_eq!(selected.fg, Some(Color::WHITE));
    assert_eq!(selected.bg, Some(Color::BLUE));

    let other = buffer.get(1, 4).unwrap();
    assert_eq!(other.symbol, 'b');
    assert_eq!(other.bg, None);
}

// =========================================================================
// Rows
// =========================================================================

#[test]
fn test_table_rows_builder() {
    let t = Table::new(vec![Column::new("A")]).rows(vec![
        vec!["1".into()],
        vec!["2".into()],
        vec!["3".into()],
    ]);
    assert_eq!(t.row_count(), 3);
}

#[test]
fn test_table_rows_empty() {
    let t = Table::new(vec![Column::new("A")]).rows(vec![]);
    assert_eq!(t.row_count(), 0);
}

#[test]
fn test_table_rows_replaces_existing_rows() {
    let t = Table::new(vec![Column::new("X")])
        .row(vec!["old"])
        .rows(vec![vec!["a".into()], vec!["b".into()]]);
    assert_eq!(t.row_count(), 2);
}

#[test]
fn test_table_rows_then_row() {
    let t = Table::new(vec![Column::new("X")])
        .rows(vec![vec!["a".into()], vec!["b".into()]])
        .row(vec!["c"]);
    assert_eq!(t.row_count(), 3);
}

#[test]
fn test_table_with_many_rows() {
    let t = Table::new(vec![Column::new("X")]).rows(numbered(100));
    assert_eq!(t.row_count(), 100);
}

#[test]
fn test_table_with_empty_and_missing_cells() {
    let t = Table::new(vec![Column::new("A").width(1), Column::new("B").width(1)])
        .row(vec!["", ""])
        .row(vec!["x"]);
    assert_eq!(t.row_count(), 2);
    let buffer = render(&t, 10, 7);
    assert_eq!(row_text(&buffer, 3), "│ │ │");
    assert_eq!(row_text(&buffer, 4), "│x│ │");
}

#[test]
fn test_table_unicode_content() {
    let t = Table::new(vec![
        Column::new("名前").width(6),
        Column::new("값").width(2),
    ])
    .row(vec!["テスト", "🎉"]);
    assert_eq!(t.row_count(), 1);

    let buffer = render(&t, 20, 5);
    assert_eq!(buffer.get(1, 1).unwrap().symbol, '名');
    assert_eq!(buffer.get(3, 1).unwrap().symbol, '前');
    assert_eq!(buffer.get(8, 1).unwrap().symbol, '값');
    assert_eq!(buffer.get(1, 3).unwrap().symbol, 'テ');
    assert_eq!(buffer.get(5, 3).unwrap().symbol, 'ト');
    assert_eq!(buffer.get(8, 3).unwrap().symbol, '🎉');
}

// =========================================================================
// Selection and navigation (no wrap)
// =========================================================================

#[test]
fn test_table_selection() {
    let mut t = Table::new(vec![Column::new("X")])
        .row(vec!["a"])
        .row(vec!["b"])
        .row(vec!["c"]);

    assert_eq!(t.selected_index(), 0);

    t.select_next();
    assert_eq!(t.selected_index(), 1);

    t.select_next();
    assert_eq!(t.selected_index(), 2);

    t.select_next(); // Should stay at last
    assert_eq!(t.selected_index(), 2);

    t.select_prev();
    assert_eq!(t.selected_index(), 1);

    t.select_first();
    assert_eq!(t.selected_index(), 0);

    t.select_last();
    assert_eq!(t.selected_index(), 2);
}

#[test]
fn test_table_no_wrap_navigation() {
    let mut t = Table::new(vec![Column::new("X")])
        .row(vec!["a"])
        .row(vec!["b"]);

    t.select_prev();
    assert_eq!(t.selected_index(), 0); // Stays at 0

    t.select_last();
    t.select_next();
    assert_eq!(t.selected_index(), 1); // Stays at 1
}

#[test]
fn test_table_single_row() {
    let mut t = Table::new(vec![Column::new("X")]).row(vec!["only"]);

    t.select_next();
    assert_eq!(t.selected_index(), 0);

    t.select_prev();
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_table_navigation_empty() {
    let mut t = Table::new(vec![Column::new("X")]);

    t.select_next();
    assert_eq!(t.selected_index(), 0);
    t.select_prev();
    assert_eq!(t.selected_index(), 0);
    t.select_first();
    assert_eq!(t.selected_index(), 0);
    t.select_last();
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_table_selection_builder() {
    let t = Table::new(vec![Column::new("X")])
        .row(vec!["a"])
        .row(vec!["b"])
        .row(vec!["c"])
        .selected(2);
    assert_eq!(t.selected_index(), 2);
}

#[test]
fn test_table_selection_bounds() {
    let t = Table::new(vec![Column::new("X")])
        .row(vec!["a"])
        .row(vec!["b"])
        .selected(10); // Out of bounds

    // Clamped to the last row
    assert_eq!(t.selected_index(), 1);
}

#[test]
fn test_table_selected_before_rows_is_clamped() {
    // selected() clamps against the rows that exist at that point
    let t = Table::new(vec![Column::new("X")])
        .selected(1)
        .row(vec!["a"])
        .row(vec!["b"]);
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_table_selection_moves_highlight() {
    let mut t = Table::new(vec![Column::new("X").width(1)])
        .row(vec!["a"])
        .row(vec!["b"])
        .selected_style(Color::BLACK, Color::YELLOW);
    t.select_next();

    let buffer = render(&t, 10, 6);
    assert_eq!(buffer.get(1, 3).unwrap().bg, None);
    assert_eq!(buffer.get(1, 4).unwrap().bg, Some(Color::YELLOW));
}

// =========================================================================
// Paging
// =========================================================================

#[test]
fn test_table_page_down() {
    let mut t = Table::new(vec![Column::new("X")]).rows(numbered(20));

    t.page_down(5);
    assert_eq!(t.selected_index(), 5);
    t.page_down(5);
    assert_eq!(t.selected_index(), 10);
}

#[test]
fn test_table_page_up() {
    let mut t = Table::new(vec![Column::new("X")])
        .rows(numbered(20))
        .selected(15);

    t.page_up(5);
    assert_eq!(t.selected_index(), 10);
    t.page_up(5);
    assert_eq!(t.selected_index(), 5);
}

#[test]
fn test_table_page_down_clamps() {
    let mut t = Table::new(vec![Column::new("X")])
        .rows(numbered(10))
        .selected(8);

    t.page_down(5);
    assert_eq!(t.selected_index(), 9);
}

#[test]
fn test_table_page_up_clamps() {
    let mut t = Table::new(vec![Column::new("X")])
        .rows(numbered(10))
        .selected(2);

    t.page_up(5);
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_table_jump_to() {
    let mut t = Table::new(vec![Column::new("X")]).rows(numbered(20));

    t.jump_to(10);
    assert_eq!(t.selected_index(), 10);

    t.jump_to(0);
    assert_eq!(t.selected_index(), 0);

    t.jump_to(100); // Out of range
    assert_eq!(t.selected_index(), 19);
}

// =========================================================================
// Styling
// =========================================================================

#[test]
fn test_table_selected_style() {
    let t = Table::new(vec![Column::new("X").width(1)])
        .row(vec!["a"])
        .selected_style(Color::RED, Color::GREEN);

    let cell = *render(&t, 10, 5).get(1, 3).unwrap();
    assert_eq!(cell.symbol, 'a');
    assert_eq!(cell.fg, Some(Color::RED));
    assert_eq!(cell.bg, Some(Color::GREEN));
}

#[test]
fn test_table_header_style() {
    let t =
        Table::new(vec![Column::new("X").width(1)]).header_style(Color::YELLOW, Some(Color::BLACK));

    let cell = *render(&t, 10, 4).get(1, 1).unwrap();
    assert_eq!(cell.fg, Some(Color::YELLOW));
    assert_eq!(cell.bg, Some(Color::BLACK));
}

#[test]
fn test_table_header_style_no_bg() {
    let t = Table::new(vec![Column::new("X").width(1)])
        .header_style(Color::BLACK, Some(Color::BLUE))
        .header_style(Color::CYAN, None);

    let cell = *render(&t, 10, 4).get(1, 1).unwrap();
    assert_eq!(cell.fg, Some(Color::CYAN));
    assert_eq!(cell.bg, None);
}

#[test]
fn test_table_border_toggle() {
    let t = Table::new(vec![Column::new("X").width(1)])
        .row(vec!["a"])
        .border(false);
    let buffer = render(&t, 10, 4);
    assert_eq!(row_text(&buffer, 0), "X");
    assert_eq!(row_text(&buffer, 1), "a");
    assert_eq!(row_text(&buffer, 2), "");

    let t2 = Table::new(vec![Column::new("X").width(1)])
        .row(vec!["a"])
        .border(true);
    let buffer = render(&t2, 10, 5);
    assert_eq!(row_text(&buffer, 0), "┌─┐");
    assert_eq!(row_text(&buffer, 1), "│X│");
    assert_eq!(row_text(&buffer, 3), "│a│");
}

#[test]
fn test_table_full_builder_chain() {
    let t = Table::new(vec![Column::new("A").width(1), Column::new("B").width(1)])
        .row(vec!["1", "2"])
        .selected(0)
        .header_style(Color::YELLOW, Some(Color::BLACK))
        .selected_style(Color::WHITE, Color::BLUE)
        .border(false);

    assert_eq!(t.row_count(), 1);
    assert_eq!(t.selected_index(), 0);

    let buffer = render(&t, 10, 3);
    assert_eq!(row_text(&buffer, 0), "AB");
    assert_eq!(row_text(&buffer, 1), "12");
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::YELLOW));
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLACK));
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::WHITE));
    assert_eq!(buffer.get(0, 1).unwrap().bg, Some(Color::BLUE));
}

// =========================================================================
// Column
// =========================================================================

#[test]
fn test_column_builder() {
    let col = Column::new("Test").width(15);
    assert_eq!(col.title, "Test");
    assert_eq!(col.width, 15);
}

#[test]
fn test_column_width_default_is_auto() {
    let col = Column::new("Test");
    assert_eq!(col.width, 0);
}

#[test]
fn test_auto_width_columns_share_remaining_space() {
    // 14 wide: 4 borders + fixed 4 leaves 6 for the two auto columns
    let t = Table::new(vec![
        Column::new("A"),
        Column::new("B").width(4),
        Column::new("C"),
    ]);
    let buffer = render(&t, 14, 4);
    assert_eq!(row_text(&buffer, 0), "┌───┬────┬───┐");
}

#[test]
fn test_column_builder_chain() {
    let col = Column::new("Title").width(20).width(30);
    assert_eq!(col.title, "Title");
    assert_eq!(col.width, 30); // Last width wins
}

#[test]
fn test_column_new_with_string() {
    let col = Column::new(String::from("Owned String"));
    assert_eq!(col.title, "Owned String");
}

#[test]
fn test_column_new_empty_title() {
    let col = Column::new("");
    assert_eq!(col.title, "");
    assert_eq!(col.width, 0);
}

#[test]
fn test_column_new_unicode_title() {
    let col = Column::new("🎉 Celebration");
    assert_eq!(col.title, "🎉 Celebration");
}

#[test]
fn test_column_clone_is_independent() {
    let col1 = Column::new("Test").width(15);
    let col2 = col1.clone();
    assert_eq!(col2.title, "Test");
    assert_eq!(col2.width, 15);

    let col3 = col2.width(25);
    assert_eq!(col1.width, 15);
    assert_eq!(col3.width, 25);
}

#[test]
fn test_column_helper() {
    let col = column("Test Column");
    assert_eq!(col.title, "Test Column");
    assert_eq!(col.width, 0);

    let col = column(String::from("Owned")).width(25);
    assert_eq!(col.title, "Owned");
    assert_eq!(col.width, 25);
}

// =========================================================================
// Virtual scroll
// =========================================================================

/// Rows 0..n in a borderless one-column table, `viewport` rows high
/// (plus the header line).
fn scroll_table(n: usize) -> Table {
    Table::new(vec![Column::new("N").width(4)])
        .rows(numbered(n))
        .border(false)
}

fn has_scrollbar(buffer: &Buffer) -> bool {
    let x = buffer.width() - 1;
    (1..buffer.height()).any(|y| matches!(buffer.get(x, y).unwrap().symbol, '█' | '░'))
}

#[test]
fn test_table_virtual_scroll_default_off() {
    // 50 rows is below the auto threshold, so no scrollbar
    let buffer = render(&scroll_table(50), 10, 6);
    assert!(!has_scrollbar(&buffer));
}

#[test]
fn test_table_virtual_scroll_builder() {
    let buffer = render(&scroll_table(50).virtual_scroll(true), 10, 6);
    assert!(has_scrollbar(&buffer));
}

#[test]
fn test_table_auto_threshold() {
    // At 100+ rows virtual scrolling switches itself on
    let buffer = render(&scroll_table(99), 10, 6);
    assert!(!has_scrollbar(&buffer));
    let buffer = render(&scroll_table(100), 10, 6);
    assert!(has_scrollbar(&buffer));

    let mut large = scroll_table(200);
    large.page_down(10);
    assert_eq!(large.selected_index(), 10);
}

#[test]
fn test_table_scrollbar_default_on_and_can_be_hidden() {
    let buffer = render(&scroll_table(200), 10, 6);
    assert!(has_scrollbar(&buffer));

    let buffer = render(&scroll_table(200).show_scrollbar(false), 10, 6);
    assert!(!has_scrollbar(&buffer));
}

#[test]
fn test_table_no_scrollbar_when_rows_fit() {
    let buffer = render(&scroll_table(3).virtual_scroll(true), 10, 6);
    assert!(!has_scrollbar(&buffer));
}

#[test]
fn test_table_virtual_scroll_shows_first_rows() {
    let buffer = render(&scroll_table(10_000).virtual_scroll(true), 10, 6);
    assert_eq!(row_text(&buffer, 0).trim_end_matches(['█', '░']), "N");
    for (y, expected) in (1..6).zip(0..) {
        let text = row_text(&buffer, y);
        assert_eq!(
            text.trim_end_matches(['█', '░']).trim_end(),
            expected.to_string()
        );
    }
}

#[test]
fn test_table_virtual_scroll_keeps_selection_visible() {
    {
        let t = scroll_table(500).selected(50);
        let buffer = render(&t, 10, 6);

        let rows: Vec<String> = (1..6)
            .map(|y| {
                row_text(&buffer, y)
                    .trim_end_matches(['█', '░'])
                    .trim_end()
                    .to_string()
            })
            .collect();
        // Five data rows fit; scrolling down to row 50 leaves it at the bottom
        assert_eq!(rows, ["46", "47", "48", "49", "50"]);
        assert_eq!(buffer.get(0, 5).unwrap().bg, Some(Color::BLUE));
    }
}

// =========================================================================
// Empty input, edge values and the option matrix
// =========================================================================

/// The text of row `y` with box-drawing and scrollbar characters removed.
fn cell_text(buffer: &Buffer, y: u16) -> String {
    row_text(buffer, y)
        .chars()
        .filter(|c| !"│├┤┼┌┐└┘┬┴─█░".contains(*c))
        .collect::<String>()
        .trim()
        .to_string()
}

#[test]
fn test_table_without_columns_draws_nothing() {
    let t = Table::new(vec![]).rows(numbered(5));
    let buffer = render(&t, 10, 6);
    for y in 0..6 {
        assert_eq!(row_text(&buffer, y), "", "row {y}");
    }
}

#[test]
fn test_table_tiny_areas_do_not_panic() {
    for border in [true, false] {
        for n in [0, 1, 150] {
            for width in 0..5 {
                for height in 0..5 {
                    let t = scroll_table(n).border(border).selected(n.saturating_sub(1));
                    render(&t, width, height);
                }
            }
        }
    }
}

#[test]
fn test_table_huge_fixed_widths_do_not_panic() {
    let t = Table::new(vec![
        Column::new("A").width(u16::MAX),
        Column::new("B").width(u16::MAX),
        Column::new("C"),
    ])
    .row(vec!["a", "b", "c"]);
    let buffer = render(&t, 20, 5);
    assert!(row_text(&buffer, 1).contains('A'));
}

#[test]
fn test_table_wide_char_at_column_edge_is_not_split() {
    // Column 0 is three cells wide: "가" fits, "나" would straddle the border
    let t = Table::new(vec![Column::new("K").width(3), Column::new("V").width(2)])
        .row(vec!["가나", "x"]);
    let buffer = render(&t, 10, 5);
    assert_eq!(buffer.get(1, 3).unwrap().symbol, '가');
    assert_eq!(buffer.get(3, 3).unwrap().symbol, ' ');
    assert_eq!(buffer.get(4, 3).unwrap().symbol, '│');
    assert_eq!(buffer.get(5, 3).unwrap().symbol, 'x');
}

/// #854: virtual scroll (on by itself at 100+ rows) drew the bottom border
/// over the first data row.
#[test]
fn test_table_bottom_border_is_below_the_rows_at_the_threshold() {
    for n in [99, 100, 101, 150] {
        let t = Table::new(vec![Column::new("N").width(4)]).rows(numbered(n));
        let buffer = render(&t, 20, 10);
        assert_eq!(cell_text(&buffer, 3), "0", "{n} rows: first data row");
        assert!(
            row_text(&buffer, 9).starts_with('└'),
            "{n} rows: bottom border"
        );
        for y in 0..9 {
            assert!(
                !row_text(&buffer, y).starts_with('└'),
                "{n} rows: border at {y}"
            );
        }
    }
}

/// #854: below the virtual threshold the table never scrolled, so moving the
/// selection past the viewport left it off screen.
#[test]
fn test_table_small_table_scrolls_to_the_selection() {
    let mut t = Table::new(vec![Column::new("N").width(4)]).rows(numbered(20));
    for _ in 0..12 {
        t.select_next();
    }
    let buffer = render(&t, 20, 10);
    let shown: Vec<String> = (3..9).map(|y| cell_text(&buffer, y)).collect();
    assert!(
        shown.contains(&"12".to_string()),
        "row 12 not shown: {shown:?}"
    );
}

#[test]
fn test_table_scrolls_back_up_and_after_rows_shrink() {
    let mut t = scroll_table(150).selected(140);
    render(&t, 10, 6);
    t.select_first();
    let buffer = render(&t, 10, 6);
    assert_eq!(cell_text(&buffer, 1), "0");

    let t = scroll_table(150).selected(140);
    render(&t, 10, 6);
    let t = t.rows(numbered(3));
    let buffer = render(&t, 10, 6);
    let shown: Vec<String> = (1..4).map(|y| cell_text(&buffer, y)).collect();
    assert_eq!(shown, ["0", "1", "2"]);
}

/// Every combination of border, virtual scroll, row count, selection and
/// height: the selected row is drawn highlighted, data rows are contiguous
/// and in order, and the bottom border sits below all of them.
#[test]
fn test_table_option_matrix() {
    for border in [true, false] {
        for virtual_scroll in [false, true] {
            for height in [2u16, 3, 4, 5, 6, 10] {
                let viewport = height.saturating_sub(if border { 4 } else { 1 }) as usize;
                for n in [0, 1, viewport, viewport + 1, 150] {
                    let mut selections = vec![0, n / 2, n.saturating_sub(1)];
                    selections.dedup();
                    for sel in selections {
                        let ctx = format!(
                            "border={border} virtual={virtual_scroll} height={height} rows={n} sel={sel}"
                        );
                        let t = Table::new(vec![Column::new("N").width(4)])
                            .rows(numbered(n))
                            .border(border)
                            .virtual_scroll(virtual_scroll)
                            .selected(sel);
                        let buffer = render(&t, 12, height);

                        let first_data_y = if border { 3 } else { 1 };
                        let data_end = if border {
                            height.saturating_sub(1)
                        } else {
                            height
                        };
                        let shown: Vec<usize> = (first_data_y..data_end.max(first_data_y))
                            .filter_map(|y| cell_text(&buffer, y).parse().ok())
                            .collect();

                        assert!(
                            shown.windows(2).all(|w| w[1] == w[0] + 1),
                            "{ctx}: rows not contiguous {shown:?}"
                        );
                        assert_eq!(shown.len(), n.min(viewport), "{ctx}: {shown:?}");
                        if n > 0 && viewport > 0 {
                            let y = (first_data_y..data_end)
                                .find(|&y| cell_text(&buffer, y) == sel.to_string())
                                .unwrap_or_else(|| panic!("{ctx}: selection not shown {shown:?}"));
                            assert_eq!(buffer.get(1, y).unwrap().bg, Some(Color::BLUE), "{ctx}");
                        }
                        if border && height >= 4 {
                            let bottom = (0..height)
                                .filter(|&y| row_text(&buffer, y).starts_with('└'))
                                .collect::<Vec<_>>();
                            let last_row_y = first_data_y + shown.len() as u16;
                            assert_eq!(bottom, [last_row_y], "{ctx}: bottom border");
                        }
                    }
                }
            }
        }
    }
}

// =========================================================================
// Combined with other modules
// =========================================================================

/// Drawn into an offset sub-area (as a layout parent does), a scrolled table
/// stays inside it and leaves the surrounding cells alone.
#[test]
fn test_table_in_an_offset_area_stays_inside_it() {
    for border in [true, false] {
        for n in [0usize, 3, 150] {
            let mut buffer = Buffer::new(20, 12);
            for y in 0..12 {
                for x in 0..20 {
                    buffer.set(x, y, revue::render::Cell::new('x'));
                }
            }
            let area = Rect::new(3, 2, 12, 7);
            let t = Table::new(vec![Column::new("A"), Column::new("B").width(3)])
                .rows((0..n).map(|i| vec![i.to_string(), "b".into()]).collect())
                .border(border)
                .selected(n.saturating_sub(1));
            let mut ctx = RenderContext::new(&mut buffer, area);
            t.render(&mut ctx);

            for y in 0..12 {
                for x in 0..20 {
                    if !(3..15).contains(&x) || !(2..9).contains(&y) {
                        assert_eq!(
                            buffer.get(x, y).unwrap().symbol,
                            'x',
                            "border={border} rows={n}: drew outside the area at ({x}, {y})"
                        );
                    }
                }
            }
            if n == 150 {
                let last = (2..9).any(|y| (3..15).any(|x| buffer.get(x, y).unwrap().symbol == '9'));
                assert!(last, "border={border}: row 149 not shown");
            }
        }
    }
}

/// Inside a vstack, through the stylesheet and DOM path: a large table keeps
/// its sibling, its border and the stylesheet color on its rows.
#[test]
fn test_table_in_a_stack_with_a_stylesheet() {
    use revue::prelude::*;
    use revue::testing::PipelineHarness;

    struct Screen;
    impl View for Screen {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .child(
                    Table::new(vec![Column::new("N").width(5)])
                        .rows(numbered(150))
                        .element_id("tbl"),
                )
                .child(Text::new("footer"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Screen"
        }
    }

    let red = Color::rgb(255, 0, 0);
    let mut h = PipelineHarness::with_css("#tbl { color: #ff0000; }", 30, 12).dom_from_render(true);
    h.draw(&Screen);
    let buffer = h.buffer();
    let rows: Vec<String> = (0..12).map(|y| row_text(buffer, y)).collect();

    let top = rows
        .iter()
        .position(|r| r.starts_with('┌'))
        .expect("top border");
    let bottom = rows
        .iter()
        .position(|r| r.starts_with('└'))
        .expect("bottom border");
    assert!(bottom > top + 3, "bottom border over the rows: {rows:#?}");
    assert_eq!(cell_text(buffer, top as u16 + 3), "0", "{rows:#?}");
    assert!(
        rows.iter().any(|r| r.contains("footer")),
        "sibling lost: {rows:#?}"
    );
    // Row 1 is unselected and takes the stylesheet color
    assert_eq!(buffer.get(1, top as u16 + 4).unwrap().fg, Some(red));
}

fn draw_styled(css: &str, t: Table) -> revue::testing::PipelineHarness {
    use revue::prelude::*;

    struct Screen(Table);
    impl View for Screen {
        fn render(&self, ctx: &mut RenderContext) {
            vstack().child(self.0.clone().element_id("tbl")).render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Screen"
        }
    }

    let mut h = revue::testing::PipelineHarness::with_css(css, 30, 8).dom_from_render(true);
    h.draw(&Screen(t));
    h
}

fn two_rows() -> Table {
    Table::new(vec![Column::new("Name").width(6)])
        .row(vec!["a"])
        .row(vec!["b"])
}

/// The header has a default color of its own, but no builder named it, so
/// the stylesheet's `color` and `background` reach it.
#[test]
fn test_table_header_reads_the_stylesheet() {
    let red = Color::rgb(255, 0, 0);
    let navy = Color::rgb(0, 0, 128);
    let h = draw_styled("#tbl { color: #ff0000; background: #000080; }", two_rows());
    let header = h.buffer().get(1, 1).unwrap();
    assert_eq!(header.symbol, 'N');
    assert_eq!(header.fg, Some(red));
    assert_eq!(header.bg, Some(navy));
}

/// A header color the builder named outranks the stylesheet.
#[test]
fn test_table_header_style_outranks_the_stylesheet() {
    let green = Color::rgb(0, 255, 0);
    let h = draw_styled(
        "#tbl { color: #ff0000; }",
        two_rows().header_style(green, None),
    );
    assert_eq!(h.buffer().get(1, 1).unwrap().fg, Some(green));
}

/// Without a stylesheet the header keeps its white default.
#[test]
fn test_table_header_default_without_a_stylesheet() {
    let h = draw_styled("", two_rows());
    assert_eq!(h.buffer().get(1, 1).unwrap().fg, Some(Color::WHITE));
}

/// The border lines take `border-color`, falling back to `color`.
#[test]
fn test_table_border_lines_read_the_stylesheet() {
    let red = Color::rgb(255, 0, 0);
    let blue = Color::rgb(0, 0, 255);
    for (css, want) in [
        ("#tbl { border-color: #0000ff; color: #ff0000; }", blue),
        ("#tbl { color: #ff0000; }", red),
    ] {
        let h = draw_styled(css, two_rows());
        let buffer = h.buffer();
        for (x, y) in [(0, 0), (3, 0), (0, 2), (3, 2), (0, 5), (3, 5)] {
            let cell = buffer.get(x, y).unwrap();
            assert!(
                "┌─├└".contains(cell.symbol),
                "({x}, {y}) is {:?}",
                cell.symbol
            );
            assert_eq!(cell.fg, Some(want), "{css}: ({x}, {y})");
        }
    }
}

mod snapshots {
    use revue::prelude::*;
    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_table_basic() {
        let view = Table::new(vec![
            Column::new("Name"),
            Column::new("Age"),
            Column::new("City"),
        ])
        .row(vec!["Alice", "30", "NYC"])
        .row(vec!["Bob", "25", "LA"])
        .row(vec!["Charlie", "35", "Chicago"]);

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("table_basic");
    }

    #[test]
    fn test_table_with_header() {
        let view = Table::new(vec![
            Column::new("ID"),
            Column::new("Product"),
            Column::new("Price"),
        ])
        .row(vec!["1", "Widget", "$9.99"])
        .row(vec!["2", "Gadget", "$19.99"]);

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("table_with_header");
    }
}
