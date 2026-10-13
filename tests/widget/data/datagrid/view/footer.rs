//! DataGrid footer rows

use super::super::{render_to, row_text};
use revue::widget::data::datagrid::{DataGrid, GridColumn, GridRow};

fn numbered_grid(rows: usize) -> DataGrid {
    let mut grid = DataGrid::new()
        .column(GridColumn::new("name", "Name").width(10))
        .column(GridColumn::new("value", "Value").width(12));
    for i in 0..rows {
        grid = grid.row(
            GridRow::new()
                .cell("name", format!("row{i}"))
                .cell("value", (i + 1).to_string()),
        );
    }
    grid
}

#[test]
fn footer_row_is_painted_below_the_rows() {
    let grid = numbered_grid(3).add_sum("value");
    let buf = render_to(&grid, 40, 8);

    let footer = row_text(&buf, 7, 40);
    assert!(footer.contains("Total"), "footer label missing: {footer:?}");
    assert!(footer.contains("Sum: 6"), "footer sum missing: {footer:?}");
    // The data rows are still drawn above it.
    assert!(row_text(&buf, 1, 40).contains("row0"));
}

#[test]
fn hidden_footer_is_not_painted() {
    let grid = numbered_grid(3).add_sum("value").show_footer(false);
    let buf = render_to(&grid, 40, 8);
    for y in 0..8 {
        assert!(!row_text(&buf, y, 40).contains("Sum:"));
    }
}

#[test]
fn footer_takes_a_row_from_the_viewport_and_last_row_stays_reachable() {
    // Height 7 = header + 5 data rows + footer.
    let mut grid = numbered_grid(20).add_sum("value");
    render_to(&grid, 40, 7);
    grid.select_last();
    let buf = render_to(&grid, 40, 7);

    let last_data_line = row_text(&buf, 5, 40);
    assert!(
        last_data_line.contains("row19"),
        "last row not visible above the footer: {last_data_line:?}"
    );
    let footer = row_text(&buf, 6, 40);
    assert!(footer.contains("Sum: 210"), "footer missing: {footer:?}");
    assert!(
        !footer.contains("row"),
        "a data row drew over the footer: {footer:?}"
    );
}
