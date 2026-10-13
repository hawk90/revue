//! DataGrid row and column selection tests

use super::{render_to, row_text};
use proptest::prelude::*;
use revue::event::Key;
use revue::widget::data::datagrid::{DataGrid, GridColors, GridColumn, GridRow};

#[test]
fn test_navigation() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .row(GridRow::new().cell("a", "1").cell("b", "2"))
        .row(GridRow::new().cell("a", "3").cell("b", "4"));

    assert_eq!(grid.selected_row, 0);

    grid.select_next();
    assert_eq!(grid.selected_row, 1);

    grid.select_prev();
    assert_eq!(grid.selected_row, 0);
}

#[test]
fn test_select_next_col() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"))
        .column(GridColumn::new("c", "C"));

    assert_eq!(grid.selected_col, 0);
    grid.select_next_col();
    assert_eq!(grid.selected_col, 1);
    grid.select_next_col();
    assert_eq!(grid.selected_col, 2);
    grid.select_next_col();
    assert_eq!(grid.selected_col, 2); // Can't go past last
}

#[test]
fn test_select_prev_col() {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .column(GridColumn::new("b", "B"));

    grid.selected_col = 1;
    grid.select_prev_col();
    assert_eq!(grid.selected_col, 0);
    grid.select_prev_col();
    assert_eq!(grid.selected_col, 0); // Can't go before first
}

#[test]
fn test_page_up() {
    let mut grid = DataGrid::new().column(GridColumn::new("a", "A"));

    let rows: Vec<_> = (0..50)
        .map(|i| GridRow::new().cell("a", i.to_string()))
        .collect();
    grid = grid.rows(rows);

    grid.selected_row = 25;
    grid.page_up(10);
    assert_eq!(grid.selected_row, 15);

    grid.page_up(20);
    assert_eq!(grid.selected_row, 0); // Clamped to 0
}

#[test]
fn test_ensure_visible_with_height() {
    let mut grid = DataGrid::new().column(GridColumn::new("a", "A"));

    let rows: Vec<_> = (0..100)
        .map(|i| GridRow::new().cell("a", i.to_string()))
        .collect();
    grid = grid.rows(rows);

    grid.selected_row = 50;
    grid.scroll_row = 0;
    grid.ensure_visible_with_height(10);

    // Scroll should adjust to show selected row
    assert!(grid.scroll_row > 0);
}

#[test]
fn test_set_viewport_height() {
    let mut grid = DataGrid::new().column(GridColumn::new("a", "A"));

    let rows: Vec<_> = (0..50)
        .map(|i| GridRow::new().cell("a", i.to_string()))
        .collect();
    grid = grid.rows(rows);

    grid.selected_row = 30;
    grid.scroll_row = 0;
    grid.set_viewport_height(10);

    assert!(grid.scroll_row > 0);
}

#[test]
fn test_scroll_info() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"));

    let (scroll, total, _viewport) = grid.scroll_info();
    assert_eq!(scroll, 0);
    assert_eq!(total, 2);
}

#[test]
fn test_visible_row_count() {
    let grid = DataGrid::new()
        .column(GridColumn::new("a", "A"))
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"));

    assert_eq!(grid.visible_row_count(), 2);
}

#[test]
fn test_toggle_selection() {
    let mut grid = DataGrid::new()
        .multi_select(true)
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"));

    assert!(!grid.rows[0].selected);
    grid.toggle_selection();
    assert!(grid.rows[0].selected);
    grid.toggle_selection();
    assert!(!grid.rows[0].selected);
}

#[test]
fn test_toggle_selection_without_multi_select() {
    let mut grid = DataGrid::new()
        .multi_select(false)
        .row(GridRow::new().cell("a", "1"));

    grid.toggle_selection();
    // Should not toggle when multi_select is disabled
    assert!(!grid.rows[0].selected);
}

#[test]
fn test_selected_rows() {
    let mut grid = DataGrid::new()
        .multi_select(true)
        .row(GridRow::new().cell("a", "1"))
        .row(GridRow::new().cell("a", "2"))
        .row(GridRow::new().cell("a", "3"));

    grid.rows[0].selected = true;
    grid.rows[2].selected = true;

    let selected = grid.selected_rows();
    assert_eq!(selected.len(), 2);
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// With a render between keys, the selected row is always drawn, with
    /// the selection background, in the viewport
    #[test]
    fn the_selected_row_stays_on_screen(
        rows in 1usize..60,
        height in 3u16..15,
        keys in prop::collection::vec(0usize..6, 0..40),
    ) {
        let moves = [Key::Up, Key::Down, Key::PageUp, Key::PageDown, Key::Home, Key::End];
        let mut grid = DataGrid::new()
            .column(GridColumn::new("a", "A"))
            .rows((0..rows).map(|i| GridRow::new().cell("a", format!("r{i}"))).collect());
        render_to(&grid, 20, height);
        for k in keys {
            grid.handle_key(&moves[k]);
            render_to(&grid, 20, height);
        }

        let buffer = render_to(&grid, 20, height);
        prop_assert!(grid.selected_row < rows);
        prop_assert!(grid.scroll_row <= grid.selected_row);
        let y = 1 + (grid.selected_row - grid.scroll_row) as u16;
        prop_assert!(y < height, "selected {} scroll {}", grid.selected_row, grid.scroll_row);
        let line = row_text(&buffer, y, 20);
        let expected = format!("r{} ", grid.selected_row);
        prop_assert!(line.starts_with(&expected), "{:?}", line);
        prop_assert_eq!(buffer.get(0, y).unwrap().bg, Some(GridColors::default().selected_bg));
    }
}
