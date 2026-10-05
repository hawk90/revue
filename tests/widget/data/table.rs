//! Public API tests for the Table widget
//!
//! Table keeps its columns, colours, border and scroll settings private, so
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
