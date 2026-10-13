//! DataGrid header mouse hit-testing tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::datagrid::{DataGrid, GridColumn, GridRow};
use revue::widget::traits::{RenderContext, View};

#[test]
fn test_header_click_hits_rendered_column_with_row_numbers() {
    use revue::event::{MouseButton, MouseEventKind};

    let mut grid = DataGrid::new()
        .row_numbers(true)
        .reorderable(true)
        .column(GridColumn::new("a", "A").width(6).resizable(false))
        .column(GridColumn::new("b", "B").width(6).resizable(false))
        .rows((0..3).map(|_| GridRow::new().cell("a", "v")).collect());
    let area = Rect::new(0, 0, 40, 5);

    let mut buffer = Buffer::new(40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);

    // Press on the drawn "B" header title: that should start dragging column 1
    let b_x = (0..40)
        .find(|&x| buffer.get(x, 0).unwrap().symbol == 'B')
        .expect("header B drawn");
    assert!(grid.handle_mouse(MouseEventKind::Down(MouseButton::Left), b_x, 0, area));
    assert_eq!(grid.dragging_col, Some(1));
}

#[test]
fn test_header_mouse_follows_frozen_and_scrolled_columns() {
    use revue::event::{MouseButton, MouseEventKind};

    // A is frozen left; scrolling one column right hides B, so C is drawn
    // straight after A and D after C.
    let mut grid = DataGrid::new()
        .row_numbers(true)
        .reorderable(true)
        .freeze_columns_left(1)
        .column(GridColumn::new("a", "A").width(6).resizable(false))
        .column(GridColumn::new("b", "B").width(6).resizable(false))
        .column(GridColumn::new("c", "C").width(6).resizable(true))
        .column(GridColumn::new("d", "D").width(6).resizable(false))
        .rows((0..3).map(|_| GridRow::new().cell("a", "v")).collect());
    grid.scroll_col_right();
    let area = Rect::new(0, 0, 40, 5);

    let mut buffer = Buffer::new(40, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    grid.render(&mut ctx);

    let find = |title: char| (0..40).find(|&x| buffer.get(x, 0).unwrap().symbol == title);
    assert_eq!(find('B'), None, "B is scrolled out of view");
    let c_x = find('C').expect("header C drawn");

    // Hovering C's right separator (C is 6 wide) is C's resize handle
    assert!(grid.handle_mouse(MouseEventKind::Move, c_x + 6, 0, area));
    assert_eq!(grid.hovered_resize, Some(2));

    // Pressing on the drawn C title drags column C, not B
    assert!(grid.handle_mouse(MouseEventKind::Down(MouseButton::Left), c_x, 0, area));
    assert_eq!(grid.dragging_col, Some(2));
}

#[test]
fn test_header_mouse_in_offset_area() {
    use revue::event::{MouseButton, MouseEventKind};

    // The grid drawn at x=10: the hit-test works in absolute coordinates
    let mut grid = DataGrid::new()
        .row_numbers(true)
        .reorderable(true)
        .column(GridColumn::new("a", "A").width(6).resizable(false))
        .column(GridColumn::new("b", "B").width(6).resizable(false))
        .rows((0..3).map(|_| GridRow::new().cell("a", "v")).collect());
    let area = Rect::new(10, 2, 40, 5);

    // 1-digit gutter is 3 wide: A at 13..19, separator 19, B from 20
    assert!(grid.handle_mouse(MouseEventKind::Down(MouseButton::Left), 20, 2, area));
    assert_eq!(grid.dragging_col, Some(1));
}
