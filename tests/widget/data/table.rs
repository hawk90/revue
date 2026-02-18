//! Public API tests for Table widget

use revue::style::Color;
use revue::widget::data::{table, Column};

// =========================================================================
// Constructor tests
// =========================================================================

#[test]
fn test_table_new() {
    let t = revue::widget::data::Table::new(vec![Column::new("Name"), Column::new("Age")]);
    assert_eq!(t.columns.len(), 2);
    assert_eq!(t.row_count(), 0);
}

#[test]
fn test_table_with_rows() {
    let t = revue::widget::data::Table::new(vec![Column::new("A"), Column::new("B")])
        .row(vec!["1", "2"])
        .row(vec!["3", "4"]);

    assert_eq!(t.row_count(), 2);
}

#[test]
fn test_table_selection() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")])
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
fn test_column_builder() {
    let col = Column::new("Test").width(15);
    assert_eq!(col.title, "Test");
    assert_eq!(col.width, 15);
}

#[test]
fn test_table_helpers() {
    let t = table(vec![Column::new("A"), Column::new("B")]).row(vec!["1", "2"]);

    assert_eq!(t.columns.len(), 2);
    assert_eq!(t.row_count(), 1);
}

#[test]
fn test_table_no_wrap_navigation() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")])
        .row(vec!["a"])
        .row(vec!["b"]);

    // At start, can't go up
    assert_eq!(t.selected_index(), 0);
    t.select_prev();
    assert_eq!(t.selected_index(), 0); // Stays at 0

    // At end, can't go down
    t.select_last();
    assert_eq!(t.selected_index(), 1);
    t.select_next();
    assert_eq!(t.selected_index(), 1); // Stays at 1
}

#[test]
fn test_table_navigation_comprehensive() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")])
        .row(vec!["a"])
        .row(vec!["b"])
        .row(vec!["c"]);

    // Start at first
    assert_eq!(t.selected_index(), 0);

    // Go to last
    t.select_last();
    assert_eq!(t.selected_index(), 2);

    // Go back to first
    t.select_first();
    assert_eq!(t.selected_index(), 0);

    // Navigate down twice
    t.select_next();
    t.select_next();
    assert_eq!(t.selected_index(), 2);

    // Navigate up once
    t.select_prev();
    assert_eq!(t.selected_index(), 1);
}

#[test]
fn test_table_selected_index_with_rows() {
    let t = revue::widget::data::Table::new(vec![Column::new("Name")])
        .row(vec!["Alice"])
        .row(vec!["Bob"])
        .selected(1);

    assert_eq!(t.selected_index(), 1);
    assert_eq!(t.row_count(), 2);
}

#[test]
fn test_table_empty() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")]);
    assert_eq!(t.row_count(), 0);
}

#[test]
fn test_table_single_row() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")]).row(vec!["only"]);

    assert_eq!(t.selected_index(), 0);

    t.select_next();
    assert_eq!(t.selected_index(), 0); // Can't go further

    t.select_prev();
    assert_eq!(t.selected_index(), 0); // Can't go back
}

#[test]
fn test_table_rows_builder() {
    let t = revue::widget::data::Table::new(vec![Column::new("A")]).rows(vec![
        vec!["1".into()],
        vec!["2".into()],
        vec!["3".into()],
    ]);

    assert_eq!(t.row_count(), 3);
}


#[test]
fn test_table_selection_builder() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")])
        .row(vec!["a"])
        .row(vec!["b"])
        .row(vec!["c"])
        .selected(2);

    assert_eq!(t.selected_index(), 2);
}

#[test]
fn test_table_selection_bounds() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")])
        .row(vec!["a"])
        .row(vec!["b"])
        .selected(10); // Out of bounds

    // Should be clamped to valid range
    assert!(t.selected_index() <= 1);
}

#[test]
fn test_table_selected_style() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")])
        .row(vec!["a"])
        .selected_style(Color::WHITE, Color::BLUE);

    assert_eq!(t.selected_fg, Some(Color::WHITE));
    assert_eq!(t.selected_bg, Some(Color::BLUE));
}

#[test]
fn test_table_header_style() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")]).header_style(Color::YELLOW, Some(Color::BLACK));

    assert_eq!(t.header_fg, Some(Color::YELLOW));
    assert_eq!(t.header_bg, Some(Color::BLACK));
}

#[test]
fn test_table_border_toggle() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")]).border(false);
    assert!(!t.border);

    let t2 = revue::widget::data::Table::new(vec![Column::new("X")]).border(true);
    assert!(t2.border);
}

// =========================================================================
// Column Clone trait tests
// =========================================================================

#[test]
fn test_column_clone_basic() {
    let col1 = Column::new("Test").width(10);
    let col2 = col1.clone();

    assert_eq!(col1.title, col2.title);
    assert_eq!(col1.width, col2.width);
}

#[test]
fn test_column_clone_with_title() {
    let col1 = Column::new("Original Title");
    let col2 = col1.clone();

    assert_eq!(col2.title, "Original Title");
    // Modifying clone shouldn't affect original
    let col3 = Column::new(col2.title.clone()).width(20);
    assert_eq!(col1.width, 0);
    assert_eq!(col3.width, 20);
}

#[test]
fn test_column_clone_with_width() {
    let col1 = Column::new("Test").width(15);
    let col2 = col1.clone();

    assert_eq!(col2.width, 15);

    let col3 = col2.width(25);
    assert_eq!(col1.width, 15);
    assert_eq!(col3.width, 25);
}

#[test]
fn test_column_clone_empty() {
    let col1 = Column::new("");
    let col2 = col1.clone();

    assert_eq!(col2.title, "");
    assert_eq!(col2.width, 0);
}

// =========================================================================
// Table Default trait tests
// =========================================================================

#[test]
fn test_table_default() {
    let t = revue::widget::data::Table::default();
    assert_eq!(t.columns.len(), 0);
    assert_eq!(t.row_count(), 0);
    assert_eq!(t.selected_index(), 0);
    assert!(t.border);
    assert_eq!(t.header_fg, Some(Color::WHITE));
    assert_eq!(t.selected_bg, Some(Color::BLUE));
}

#[test]
fn test_table_default_empty_columns() {
    let t = revue::widget::data::Table::default();
    assert!(t.columns.is_empty());
}

#[test]
fn test_table_default_has_border() {
    let t = revue::widget::data::Table::default();
    assert!(t.border);
}

#[test]
fn test_table_default_colors() {
    let t = revue::widget::data::Table::default();
    assert_eq!(t.header_fg, Some(Color::WHITE));
    assert_eq!(t.header_bg, None);
    assert_eq!(t.selected_fg, Some(Color::WHITE));
    assert_eq!(t.selected_bg, Some(Color::BLUE));
}

// =========================================================================
// Column public field tests
// =========================================================================

#[test]
fn test_column_public_fields_accessible() {
    let col = Column::new("Field Test").width(20);

    // Direct field access
    assert_eq!(col.title, "Field Test");
    assert_eq!(col.width, 20);
}

#[test]
fn test_column_title_field() {
    let col = Column::new("Custom Title");
    assert_eq!(col.title, "Custom Title");
}

#[test]
fn test_column_width_field_default() {
    let col = Column::new("Test");
    assert_eq!(col.width, 0);
}

#[test]
fn test_column_width_field_set() {
    let col = Column::new("Test").width(100);
    assert_eq!(col.width, 100);
}

// =========================================================================
// Table builder chain tests
// =========================================================================

#[test]
fn test_table_full_builder_chain() {
    let t = revue::widget::data::Table::new(vec![Column::new("A"), Column::new("B")])
        .row(vec!["1", "2"])
        .selected(0)
        .header_style(Color::YELLOW, Some(Color::BLACK))
        .selected_style(Color::WHITE, Color::BLUE)
        .border(false);

    assert_eq!(t.row_count(), 1);
    assert_eq!(t.selected_index(), 0);
    assert_eq!(t.header_fg, Some(Color::YELLOW));
    assert_eq!(t.header_bg, Some(Color::BLACK));
    assert_eq!(t.selected_fg, Some(Color::WHITE));
    assert_eq!(t.selected_bg, Some(Color::BLUE));
    assert!(!t.border);
}

#[test]
fn test_table_multiple_rows_builder() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")])
        .row(vec!["a"])
        .row(vec!["b"])
        .row(vec!["c"])
        .row(vec!["d"]);

    assert_eq!(t.row_count(), 4);
}

// =========================================================================
// Table rows method tests
// =========================================================================

#[test]
fn test_table_rows_empty() {
    let t = revue::widget::data::Table::new(vec![Column::new("A")]).rows(vec![]);
    assert_eq!(t.row_count(), 0);
}

#[test]
fn test_table_rows_multiple() {
    let t = revue::widget::data::Table::new(vec![Column::new("A"), Column::new("B")]).rows(vec![
        vec!["1".into(), "2".into()],
        vec!["3".into(), "4".into()],
        vec!["5".into(), "6".into()],
    ]);

    assert_eq!(t.row_count(), 3);
}

#[test]
fn test_table_rows_with_string() {
    let t = revue::widget::data::Table::new(vec![Column::new("Name")])
        .rows(vec![vec![String::from("Alice")], vec![String::from("Bob")]]);

    assert_eq!(t.row_count(), 2);
}

#[test]
fn test_table_rows_then_row() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")])
        .rows(vec![vec!["a".into()], vec!["b".into()]])
        .row(vec!["c"]);

    assert_eq!(t.row_count(), 3);
}

// =========================================================================
// Table navigation edge cases
// =========================================================================

#[test]
fn test_table_navigation_empty() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")]);

    t.select_next();
    t.select_prev();
    t.select_first();
    t.select_last();

    // Should not panic
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_table_select_first_resets_offset() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")])
        .row(vec!["a"])
        .row(vec!["b"])
        .row(vec!["c"])
        .selected(2);

    t.select_first();
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_table_select_last_from_start() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")])
        .row(vec!["a"])
        .row(vec!["b"])
        .row(vec!["c"]);

    t.select_last();
    assert_eq!(t.selected_index(), 2);
}

// =========================================================================
// Table render edge cases
// =========================================================================


// =========================================================================
// Column method tests
// =========================================================================

#[test]
fn test_column_new_with_string() {
    let col = Column::new(String::from("Owned String"));
    assert_eq!(col.title, "Owned String");
}

#[test]
fn test_column_new_empty_title() {
    let col = Column::new("");
    assert_eq!(col.title, "");
}

#[test]
fn test_column_new_unicode_title() {
    let col = Column::new("🎉 Celebration");
    assert_eq!(col.title, "🎉 Celebration");
}

#[test]
fn test_column_width_zero() {
    let col = Column::new("Test").width(0);
    assert_eq!(col.width, 0);
}

#[test]
fn test_column_width_large() {
    let col = Column::new("Test").width(1000);
    assert_eq!(col.width, 1000);
}

#[test]
fn test_column_builder_chain() {
    let col = Column::new("Title").width(20).width(30);
    assert_eq!(col.title, "Title");
    assert_eq!(col.width, 30); // Last width wins
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_table_helper_empty() {
    let t = table(vec![]);
    assert_eq!(t.columns.len(), 0);
}

#[test]
fn test_table_helper_with_columns() {
    let t = table(vec![Column::new("A").width(5), Column::new("B").width(10)]);

    assert_eq!(t.columns.len(), 2);
}

#[test]
fn test_column_helper() {
    let col = revue::widget::data::column("Test Column");
    assert_eq!(col.title, "Test Column");
    assert_eq!(col.width, 0);
}

#[test]
fn test_column_helper_with_string() {
    let col = revue::widget::data::column(String::from("Owned"));
    assert_eq!(col.title, "Owned");
}

#[test]
fn test_column_helper_chainable() {
    let col = revue::widget::data::column("Chained").width(25);
    assert_eq!(col.title, "Chained");
    assert_eq!(col.width, 25);
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_table_with_many_columns() {
    let cols: Vec<Column> = (0..10).map(|i| Column::new(format!("Col{}", i))).collect();
    let t = revue::widget::data::Table::new(cols);
    assert_eq!(t.columns.len(), 10);
}

#[test]
fn test_table_with_many_rows() {
    let rows: Vec<Vec<String>> = (0..100).map(|i| vec![format!("Row{}", i)]).collect();

    let t = revue::widget::data::Table::new(vec![Column::new("X")]).rows(rows);
    assert_eq!(t.row_count(), 100);
}

#[test]
fn test_table_with_empty_cells() {
    let t = revue::widget::data::Table::new(vec![Column::new("A"), Column::new("B")])
        .row(vec!["", ""])
        .row(vec!["x", ""]);

    assert_eq!(t.row_count(), 2);
}

#[test]
fn test_table_unicode_content() {
    let t = revue::widget::data::Table::new(vec![Column::new("名前"), Column::new("값")]).row(vec!["テスト", "🎉"]);

    assert_eq!(t.row_count(), 1);
}

#[test]
fn test_table_selected_valid_range() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")])
        .row(vec!["a"])
        .row(vec!["b"])
        .selected(1);

    assert_eq!(t.selected_index(), 1);
}

#[test]
fn test_table_select_first_no_rows() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")]);
    t.select_first();
    // Should not panic
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_table_select_last_no_rows() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")]);
    t.select_last();
    // Should not panic
    assert_eq!(t.selected_index(), 0);
}

#[test]
fn test_table_header_style_no_bg() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")]).header_style(Color::CYAN, None);

    assert_eq!(t.header_fg, Some(Color::CYAN));
    assert_eq!(t.header_bg, None);
}

// =========================================================================
// Virtual scroll tests
// =========================================================================

#[test]
fn test_table_virtual_scroll_default_off() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")]);
    assert!(!t.virtual_scroll);
}

#[test]
fn test_table_virtual_scroll_builder() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")]).virtual_scroll(true);
    assert!(t.virtual_scroll);
}

#[test]
fn test_table_overscan_builder() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")]).overscan(10);
    assert_eq!(t.overscan, 10);
}

#[test]
fn test_table_show_scrollbar_builder() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")]).show_scrollbar(false);
    assert!(!t.show_scrollbar);
}

#[test]
fn test_table_page_down() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")])
        .rows((0..20).map(|i| vec![format!("{}", i)]).collect());

    assert_eq!(t.selected_index(), 0);
    t.page_down(5);
    assert_eq!(t.selected_index(), 5);
    t.page_down(5);
    assert_eq!(t.selected_index(), 10);
}

#[test]
fn test_table_page_up() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")])
        .rows((0..20).map(|i| vec![format!("{}", i)]).collect())
        .selected(15);

    t.page_up(5);
    assert_eq!(t.selected_index(), 10);
    t.page_up(5);
    assert_eq!(t.selected_index(), 5);
}

#[test]
fn test_table_page_down_clamps() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")])
        .rows((0..10).map(|i| vec![format!("{}", i)]).collect())
        .selected(8);

    t.page_down(5);
    assert_eq!(t.selected_index(), 9); // Clamped to last
}

#[test]
fn test_table_page_up_clamps() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")])
        .rows((0..10).map(|i| vec![format!("{}", i)]).collect())
        .selected(2);

    t.page_up(5);
    assert_eq!(t.selected_index(), 0); // Clamped to first
}

#[test]
fn test_table_jump_to() {
    let mut t = revue::widget::data::Table::new(vec![Column::new("X")])
        .rows((0..20).map(|i| vec![format!("{}", i)]).collect());

    t.jump_to(10);
    assert_eq!(t.selected_index(), 10);

    t.jump_to(0);
    assert_eq!(t.selected_index(), 0);

    t.jump_to(100); // Out of range
    assert_eq!(t.selected_index(), 19); // Clamped
}

#[test]
fn test_table_10k_rows_smoke() {
    let rows: Vec<Vec<String>> = (0..10_000).map(|i| vec![format!("Row {}", i)]).collect();

    let t = revue::widget::data::Table::new(vec![Column::new("X")])
        .rows(rows)
        .virtual_scroll(true);

    assert_eq!(t.row_count(), 10_000);
    // Virtual scroll should be active
    assert!(t.virtual_scroll);
}

#[test]
fn test_table_auto_threshold() {
    // Below threshold: not virtual
    let t_small = revue::widget::data::Table::new(vec![Column::new("X")])
        .rows((0..50).map(|i| vec![format!("{}", i)]).collect());
    assert!(!t_small.virtual_scroll);

    // Above threshold: auto-activates (internal, but page_down still works)
    let mut t_large = revue::widget::data::Table::new(vec![Column::new("X")])
        .rows((0..200).map(|i| vec![format!("{}", i)]).collect());
    t_large.page_down(10);
    assert_eq!(t_large.selected_index(), 10);
}

#[test]
fn test_table_scrollbar_default_on() {
    let t = revue::widget::data::Table::new(vec![Column::new("X")]);
    assert!(t.show_scrollbar);
}

#[test]
fn test_table_virtual_scroll_full_builder_chain() {
    let t = revue::widget::data::Table::new(vec![Column::new("Data")])
        .rows((0..500).map(|i| vec![format!("Item {}", i)]).collect())
        .virtual_scroll(true)
        .overscan(3)
        .show_scrollbar(true)
        .selected(50);

    assert_eq!(t.row_count(), 500);
    assert!(t.virtual_scroll);
    assert_eq!(t.overscan, 3);
    assert!(t.show_scrollbar);
    assert_eq!(t.selected_index(), 50);
}
