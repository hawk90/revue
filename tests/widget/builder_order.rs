//! #1011: the same builder calls in either order build the same widget. A
//! builder stores what it is given and the widget reconciles the fields
//! where they meet, not in whichever builder runs second.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::*;

fn abc() -> Vec<&'static str> {
    vec!["a", "b", "c"]
}

/// What `view` draws in `w` x `h`, cell by cell
fn screen(view: &impl View, w: u16, h: u16) -> Vec<String> {
    let mut buffer = Buffer::new(w, h);
    view.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, w, h)));
    (0..h)
        .map(|y| {
            (0..w)
                .map(|x| {
                    let c = buffer.get(x, y).unwrap();
                    format!("{}{:?}{:?}", c.symbol, c.fg, c.bg)
                })
                .collect()
        })
        .collect()
}

#[test]
fn select_selected_and_options() {
    assert_eq!(Select::new().selected(2).options(abc()).value(), Some("c"));
    assert_eq!(Select::new().options(abc()).selected(2).value(), Some("c"));
}

#[test]
fn tabs_selected_and_tabs() {
    assert_eq!(Tabs::new().selected(1).tabs(abc()).selected_index(), 1);
    assert_eq!(Tabs::new().tabs(abc()).selected(1).selected_index(), 1);
}

#[test]
fn table_selected_and_rows() {
    let rows = || (1..=3).map(|i| vec![i.to_string()]).collect::<Vec<_>>();
    let a = Table::new(vec![Column::new("n")]).selected(2).rows(rows());
    let b = Table::new(vec![Column::new("n")]).rows(rows()).selected(2);
    assert_eq!(a.selected_index(), 2);
    assert_eq!(b.selected_index(), 2);
}

#[test]
fn tree_selected_and_nodes() {
    let nodes = || abc().into_iter().map(TreeNode::new).collect::<Vec<_>>();
    assert_eq!(Tree::new().selected(2).nodes(nodes()).selected_index(), 2);
    assert_eq!(Tree::new().nodes(nodes()).selected(2).selected_index(), 2);
}

#[test]
fn stepper_current_and_steps() {
    let steps = || abc().into_iter().map(Step::new).collect::<Vec<_>>();
    let a = Stepper::new().current(2).steps(steps());
    let b = Stepper::new().steps(steps()).current(2);
    assert_eq!(a.current_step().map(|s| s.title.clone()), Some("c".into()));
    assert_eq!(b.current_step().map(|s| s.title.clone()), Some("c".into()));
}

#[test]
fn multi_select_selected_indices_and_options() {
    let a = MultiSelect::new()
        .selected_indices(vec![0, 1])
        .options(abc());
    let b = MultiSelect::new()
        .options(abc())
        .selected_indices(vec![0, 1]);
    assert_eq!(a.get_selected_indices(), &[0, 1]);
    assert_eq!(b.get_selected_indices(), &[0, 1]);
}

#[test]
fn slider_value_range_and_step() {
    let a = Slider::new().value(150.0).range(0.0, 200.0).get_value();
    let b = Slider::new().range(0.0, 200.0).value(150.0).get_value();
    assert_eq!((a, b), (150.0, 150.0));
    let a = Slider::new().value(37.0).step(10.0).get_value();
    let b = Slider::new().step(10.0).value(37.0).get_value();
    assert_eq!(a, b);
}

#[test]
fn rating_value_and_max_value() {
    let a = Rating::new().value(8.0).max_value(10).get_value();
    let b = Rating::new().max_value(10).value(8.0).get_value();
    assert_eq!((a, b), (8.0, 8.0));
    let a = Rating::new().value(1.0).max_value(0).get_value();
    let b = Rating::new().max_value(0).value(1.0).get_value();
    assert_eq!(a, b);
}

#[test]
fn range_picker_start_and_end() {
    let (early, late) = (Date::new(2025, 1, 10), Date::new(2025, 1, 20));
    let a = RangePicker::new()
        .start_date(late)
        .end_date(early)
        .get_range();
    let b = RangePicker::new()
        .end_date(early)
        .start_date(late)
        .get_range();
    assert_eq!(a, b);
    assert!(a.0 <= a.1, "start after end: {a:?}");
}

#[test]
fn datagrid_data_and_column() {
    let a = DataGrid::new()
        .data(vec![vec!["xyz".to_string()]])
        .column(GridColumn::new("k", "K"));
    let b = DataGrid::new()
        .column(GridColumn::new("k", "K"))
        .data(vec![vec!["xyz".to_string()]]);
    assert_eq!(screen(&a, 20, 4), screen(&b, 20, 4));
}

#[test]
fn grid_item_col_span_and_col() {
    let a = GridItem::new(Text::new("x")).col_span(3).col(2);
    let b = GridItem::new(Text::new("x")).col(2).col_span(3);
    assert_eq!(
        (a.placement.col_start, a.placement.col_end),
        (b.placement.col_start, b.placement.col_end)
    );
}

#[test]
fn heatmap_show_values_and_cell_width() {
    let data = || vec![vec![0.5, 1.0], vec![0.25, 0.75]];
    let a = HeatMap::new(data()).show_values(true).cell_width(2);
    let b = HeatMap::new(data()).cell_width(2).show_values(true);
    assert_eq!(screen(&a, 20, 4), screen(&b, 20, 4));
}

#[test]
fn piechart_donut_and_style() {
    let a = PieChart::new()
        .slice("a", 1.0)
        .slice("b", 2.0)
        .donut(0.0)
        .style(PieStyle::Donut);
    let b = PieChart::new()
        .slice("a", 1.0)
        .slice("b", 2.0)
        .style(PieStyle::Donut)
        .donut(0.0);
    assert_eq!(screen(&a, 20, 10), screen(&b, 20, 10));
}
