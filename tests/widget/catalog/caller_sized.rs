//! Sizes the caller passes to a builder, as large as they go, draw and measure
//! without a panic at any area (#978, REV-LAYOUT-006). A row that runs out of
//! columns stops drawing; it never wraps around.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::data::chart::chart_common::{Axis, Marker};
use revue::widget::traits::{RenderContext, View};
use revue::widget::*;

type Build = Box<dyn Fn(u16) -> Box<dyn View>>;

fn b<V: View + 'static>(f: impl Fn(u16) -> V + 'static) -> Build {
    Box::new(move |n| Box::new(f(n)))
}

fn tree_nodes() -> TreeNode {
    TreeNode::new("a")
        .expanded(true)
        .child(TreeNode::new("b").expanded(true).child(TreeNode::new("c")))
}

fn cases() -> Vec<(&'static str, Build)> {
    let bars = || BarChart::new().bar("a", 1.0).bar("b", 2.0);
    let items = || vec!["a".to_string(), "b".to_string()];
    let file = || {
        vec![FileEntry::directory("d", "/d")
            .expanded(true)
            .children(vec![FileEntry::directory("e", "/d/e")
                .expanded(true)
                .children(vec![FileEntry::file("f", "/d/e/f")])])]
    };
    vec![
        (
            "Tree indent",
            b(|n| Tree::new().indent(n).node(tree_nodes())),
        ),
        (
            "JsonViewer indent_size",
            b(|n| JsonViewer::from_content(r#"{"a":{"b":{"c":1}}}"#).indent_size(n)),
        ),
        (
            "FileTree indent",
            b(move |n| FileTree::new().root(file()).indent(n)),
        ),
        (
            "FileTree height",
            b(move |n| FileTree::new().root(file()).height(n)),
        ),
        ("BarChart gap", b(move |n| bars().gap(n))),
        (
            "BarChart gap vertical",
            b(move |n| bars().vertical().gap(n)),
        ),
        ("BarChart bar_width", b(move |n| bars().bar_width(n))),
        (
            "BarChart bar_width vertical",
            b(move |n| bars().vertical().bar_width(n)),
        ),
        ("BarChart label_width", b(move |n| bars().label_width(n))),
        (
            "BarChart label_width vertical",
            b(move |n| bars().vertical().label_width(n)),
        ),
        (
            "DataGrid row_height",
            b(|n| {
                DataGrid::new()
                    .column(GridColumn::new("a", "A"))
                    .row_height(n)
            }),
        ),
        (
            "GridColumn width",
            b(|n| {
                DataGrid::new()
                    .column(GridColumn::new("a", "A").width(n))
                    .column(GridColumn::new("b", "B").width(n))
            }),
        ),
        (
            "Table width",
            b(|n| {
                Table::new(vec![Column::new("A").width(n), Column::new("B").width(n)])
                    .rows(vec![vec!["1".to_string(), "2".to_string()]])
            }),
        ),
        (
            "VirtualList item_height",
            b(move |n| VirtualList::new(items()).item_height(n).selected(1)),
        ),
        ("Divider margin", b(|n| Divider::new().margin(n))),
        ("Divider length", b(|n| Divider::new().length(n))),
        ("Gauge width", b(|n| Gauge::new().percent(50.0).width(n))),
        ("Gauge height", b(|n| Gauge::new().percent(50.0).height(n))),
        (
            "Gauge segments",
            b(|n| Gauge::new().percent(50.0).segments(n)),
        ),
        ("Skeleton width", b(|n| Skeleton::new().width(n))),
        ("Skeleton height", b(|n| Skeleton::new().height(n))),
        ("Skeleton lines", b(|n| Skeleton::new().lines(n))),
        ("Modal width", b(|n| Modal::new().title("T").width(n))),
        ("Modal height", b(|n| Modal::new().title("T").height(n))),
        ("Slider length", b(|n| Slider::new().length(n))),
        ("Slider ticks", b(|n| Slider::new().ticks(n))),
        ("Radio gap", b(move |n| RadioGroup::new(items()).gap(n))),
        ("Switch width", b(|n| Switch::new().width(n))),
        ("Button width", b(|n| Button::new("B").width(n))),
        (
            "Select width",
            b(move |n| Select::new().options(items()).width(n)),
        ),
        (
            "Combobox width",
            b(move |n| Combobox::new().options(items()).width(n)),
        ),
        ("NumberInput width", b(|n| NumberInput::new().width(n))),
        ("SearchBar width", b(|n| SearchBar::new().width(n))),
        ("MaskedInput width", b(|n| MaskedInput::password().width(n))),
        (
            "Pagination max_visible",
            b(|n| Pagination::new(100).max_visible(n)),
        ),
        ("Pagination current", b(|n| Pagination::new(100).current(n))),
        ("Pagination pages", b(|n| Pagination::new(n).current(n))),
        (
            "Candle height",
            b(|n| CandleChart::new(vec![Candle::new(1.0, 2.0, 0.5, 1.5)]).height(n)),
        ),
        ("TimeSeries height", b(|n| TimeSeries::new().height(n))),
        (
            "Waveline height",
            b(|n| Waveline::new(vec![1.0, 2.0]).height(n)),
        ),
        ("Streamline height", b(|n| Streamline::new().height(n))),
        ("StatusBar height", b(|n| StatusBar::new().height(n))),
        ("Toast width", b(|n| Toast::new("t").width(n))),
        (
            "ZenMode padding",
            b(|n| {
                let mut z = ZenMode::new(Text::new("x")).padding(n);
                z.enable();
                z
            }),
        ),
        (
            "Border max",
            b(|n| Border::single().min_size(n, n).child(Text::new("x"))),
        ),
        (
            "Card padding",
            b(|n| Card::new().padding(n).title("T").body(Text::new("x"))),
        ),
        (
            "Card min",
            b(|n| Card::new().min_size(n, n).title("T").body(Text::new("x"))),
        ),
        (
            "Stack gap",
            b(|n| vstack().gap(n).child(Text::new("a")).child(Text::new("b"))),
        ),
        (
            "Stack sized",
            b(|n| {
                hstack()
                    .child_sized(Text::new("a"), n)
                    .child_sized(Text::new("b"), n)
            }),
        ),
        ("Grid min", b(|n| Grid::new().min_size(n, n))),
    ]
}

/// Data outside the range the caller set, or a mark scaled past the area.
fn out_of_range() -> Vec<(&'static str, Box<dyn View>)> {
    fn v(view: impl View + 'static) -> Box<dyn View> {
        Box::new(view)
    }
    vec![
        (
            "BarChart value above max",
            v(BarChart::new().bar("a", 1e6).max(1.0)),
        ),
        (
            "BarChart vertical value above max",
            v(BarChart::new().vertical().max(1.0).bar("a", 10.0)),
        ),
        (
            "BarChart infinite",
            v(BarChart::new().bar("a", f64::INFINITY).bar("b", f64::NAN)),
        ),
        (
            "TimeSeries value above range",
            v(TimeSeries::new().y_range(0.0, 100.0).series(
                TimeSeriesData::new("s")
                    .point(1_700_000_000, 150.0)
                    .point(1_700_000_060, -50.0),
            )),
        ),
        (
            "ScatterChart point above bounds",
            v(ScatterChart::new()
                .y_axis(Axis::new().bounds(0.0, 10.0))
                .x_axis(Axis::new().bounds(0.0, 1.0))
                .series(ScatterSeries::new("s").points(&[(5.0, 100.0), (-5.0, -100.0)]))),
        ),
        (
            "ScatterChart many ticks",
            v(ScatterChart::new()
                .x_axis(Axis::new().ticks(200))
                .series(ScatterSeries::new("s").points(&[(1.0, 1.0), (2.0, 2.0)]))),
        ),
    ]
}

#[test]
fn data_outside_the_range_stays_inside_the_chart() {
    let mut failures = Vec::new();
    for (name, view) in out_of_range() {
        for (w, h) in [
            (0, 0),
            (1, 1),
            (10, 3),
            (10, 10),
            (40, 5),
            (40, 20),
            (400, 20),
        ] {
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut buffer = Buffer::new(w, h);
                let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
                view.render(&mut ctx);
            }));
            if outcome.is_err() {
                failures.push(format!("{name} in {w}x{h}"));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn a_huge_builder_size_draws_and_measures_without_a_panic() {
    let mut failures = Vec::new();
    for (name, build) in cases() {
        for n in [0, 1, 40000, u16::MAX] {
            let view = build(n);
            for (w, h) in [(0, 0), (1, 1), (10, 3), (40, 10), (80, 24)] {
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut buffer = Buffer::new(w, h);
                    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
                    view.render(&mut ctx);
                    view.measure(w, h);
                    view.measure(u16::MAX, u16::MAX);
                }));
                if outcome.is_err() {
                    failures.push(format!("{name} {n} in {w}x{h}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn rows(view: &dyn View, w: u16, h: u16) -> Vec<String> {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    view.render(&mut ctx);
    (0..h)
        .map(|y| (0..w).map(|x| buffer.get(x, y).unwrap().symbol).collect())
        .collect()
}

#[test]
fn a_row_that_runs_out_of_columns_stops_drawing() {
    // Each deeper row starts past the edge, and nothing wraps back in.
    let drawn = rows(&Tree::new().indent(u16::MAX).node(tree_nodes()), 6, 3);
    assert_eq!(drawn, ["▼a    ", "      ", "      "]);

    let drawn = rows(
        &JsonViewer::from_content(r#"{"a":1}"#)
            .indent_size(u16::MAX)
            .show_line_numbers(false),
        8,
        2,
    );
    assert!(drawn[0].contains('{'), "{drawn:?}");
    assert_eq!(drawn[1], " ".repeat(8), "{drawn:?}");
}

#[test]
fn a_value_above_the_chart_maximum_fills_the_bar_and_stops() {
    let horizontal = BarChart::new().bar("a", 1e6).max(1.0).show_values(false);
    let drawn = rows(&horizontal, 10, 1);
    assert_eq!(
        drawn[0].chars().filter(|c| *c == '█').count(),
        8,
        "{drawn:?}"
    );

    let vertical = BarChart::new()
        .vertical()
        .max(1.0)
        .bar("a", 10.0)
        .show_values(false);
    let drawn = rows(&vertical, 3, 5);
    let bar = drawn.iter().filter(|r| r.contains('█')).count();
    assert_eq!(bar, 4, "the bar fills the rows above its label: {drawn:?}");
}

#[test]
fn a_point_outside_the_bounds_goes_on_the_edge() {
    let series = ScatterSeries::new("s")
        .points(&[(0.5, 100.0)])
        .marker(Marker::Star);
    let chart = ScatterChart::new()
        .x_axis(Axis::new().bounds(0.0, 1.0))
        .y_axis(Axis::new().bounds(0.0, 10.0))
        .series(series);
    let drawn = rows(&chart, 40, 20);
    let marks: Vec<usize> = drawn
        .iter()
        .enumerate()
        .filter(|(_, r)| r.contains(Marker::Star.char()))
        .map(|(y, _)| y)
        .collect();
    assert_eq!(marks.len(), 1, "{drawn:#?}");
    assert!(marks[0] < 3, "the top of the chart: {drawn:#?}");
}
