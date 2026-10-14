//! Streamline on the screen: placement, guards, legend, labels and
//! highlighting, plus model proptests of the stacking and the plot

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};
use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{StreamBaseline, StreamLayer, StreamOrder, Streamline, View};

fn render(chart: &Streamline, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    chart.render(&mut ctx);
    buffer
}

fn rows(buffer: &Buffer, w: u16, h: u16) -> Vec<String> {
    (0..h)
        .map(|y| (0..w).map(|x| buffer.get(x, y).unwrap().symbol).collect())
        .collect()
}

fn three() -> Streamline {
    Streamline::new()
        .layer(StreamLayer::new("Sales").data(vec![10.0, 20.0, 15.0, 25.0]))
        .layer(StreamLayer::new("Ads").data(vec![5.0, 8.0, 12.0, 10.0]))
        .layer(StreamLayer::new("Help").data(vec![3.0, 4.0, 6.0, 2.0]))
}

fn charts() -> Vec<Streamline> {
    vec![
        three(),
        three().title("Mix").bg(Color::BLUE),
        three()
            .x_labels(vec!["Q1".into(), "Q2".into(), "Q3".into(), "Q4".into()])
            .highlight(1),
        three().baseline(StreamBaseline::Zero).show_legend(false),
    ]
}

#[test]
fn zero_areas_draw_nothing() {
    for chart in charts() {
        assert_zero_areas_draw_nothing(&chart);
    }
}

#[test]
fn same_when_offset() {
    for chart in charts() {
        assert_same_when_offset(chart, 30, 10);
    }
}

#[test]
fn too_small_areas_draw_nothing() {
    for (w, h) in [(4, 10), (30, 2)] {
        let rows = rows(&render(&three(), w, h), w, h);
        assert!(
            rows.iter().all(|r| r.trim().is_empty()),
            "{w}x{h}: {rows:?}"
        );
    }
    // height(2) overrides a tall area
    let short = three().height(2);
    assert!(rows(&render(&short, 30, 10), 30, 10)
        .iter()
        .all(|r| r.trim().is_empty()));
}

#[test]
fn empty_layers_draw_the_legend_only() {
    let chart = Streamline::new()
        .layer(StreamLayer::new("E1"))
        .layer(StreamLayer::new("E2"));
    let rows = rows(&render(&chart, 30, 8), 30, 8);
    assert!(rows[0].starts_with(" █ E1  █ E2"), "{rows:?}");
    assert!(rows[1..].iter().all(|r| r.trim().is_empty()), "{rows:?}");
}

#[test]
fn title_is_centered_above_the_legend() {
    let rows = rows(&render(&three().title("Mix"), 31, 10), 31, 10);
    assert_eq!(rows[0].trim(), "Mix");
    assert_eq!(rows[0].find("Mix"), Some(14));
    assert!(rows[1].starts_with(" █ Sales"), "{rows:?}");
}

#[test]
fn legend_stops_ten_columns_before_the_edge() {
    // " █ Sales  █ Ads  █ Help": the third entry would start at column 17,
    // past 25 - 10, so it is left out
    let rows = rows(&render(&three(), 25, 8), 25, 8);
    assert!(rows[0].starts_with(" █ Sales  █ Ads"), "{rows:?}");
    assert!(!rows[0].contains("Help"), "{rows:?}");
}

#[test]
fn every_point_has_a_band_in_the_plot() {
    // 4 points across 31 columns: columns 0, 10, 20 and 30
    let chart = three().show_legend(false).show_labels(false);
    let buffer = render(&chart, 31, 10);
    for x in [0, 10, 20, 30] {
        assert!(
            (0..10).any(|y| buffer.get(x, y).unwrap().symbol == '█'),
            "col {x}"
        );
    }
    // The lowest layer reaches the bottom row and the highest the top row
    assert_eq!(buffer.get(30, 9).unwrap().symbol, '█');
    assert!((0..31).any(|x| buffer.get(x, 0).unwrap().symbol == '█'));
}

#[test]
fn bands_use_the_layer_colors_and_highlight_dims_the_others() {
    let chart = Streamline::new()
        .show_legend(false)
        .show_labels(false)
        .baseline(StreamBaseline::Zero)
        .layer(
            StreamLayer::new("A")
                .data(vec![1.0, 1.0])
                .color(Color::rgb(90, 60, 30)),
        )
        .layer(
            StreamLayer::new("B")
                .data(vec![1.0, 1.0])
                .color(Color::rgb(30, 60, 90)),
        );
    // Zero baseline over 0..2 in 9 rows: A covers the bottom rows, B the top
    let buffer = render(&chart, 10, 9);
    assert_eq!(buffer.get(0, 8).unwrap().fg, Some(Color::rgb(90, 60, 30)));
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::rgb(30, 60, 90)));

    let buffer = render(&chart.clone().highlight(1), 10, 9);
    assert_eq!(buffer.get(0, 8).unwrap().fg, Some(Color::rgb(30, 20, 10)));
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::rgb(30, 60, 90)));
}

#[test]
fn stream_label_sits_on_the_widest_point() {
    let chart = Streamline::new()
        .show_legend(false)
        .baseline(StreamBaseline::Zero)
        .layer(
            StreamLayer::new("Peak")
                .data(vec![1.0, 1.0, 9.0, 1.0, 1.0])
                .color(Color::GREEN),
        );
    let buffer = render(&chart, 41, 11);
    let rows = rows(&buffer, 41, 11);
    // Point 2 is at column 20; its band 0..9 is centered on 4.5, row 5
    assert_eq!(&rows[5][18..22], "Peak", "{rows:?}");
    let cell = buffer.get(18, 5).unwrap();
    assert_eq!((cell.fg, cell.bg), (Some(Color::WHITE), Some(Color::GREEN)));

    let hidden = chart.show_labels(false);
    assert!(!rows_text(&render(&hidden, 41, 11), 41, 11).contains("Peak"));
}

fn rows_text(buffer: &Buffer, w: u16, h: u16) -> String {
    rows(buffer, w, h).concat()
}

#[test]
fn x_labels_take_the_last_row() {
    let labels: Vec<String> = ["Q1", "Q2", "Q3", "Q4"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let chart = three().x_labels(labels).bg(Color::BLUE);
    let buffer = render(&chart, 33, 10);
    let rows = rows(&buffer, 33, 10);
    // Where the later labels go is #990's (they are spread over the width,
    // not placed at the points: draft-pk-c990); the first is at column 0
    assert!(rows[9].starts_with("Q1"), "{rows:?}");
    assert!(!rows[8].contains('Q'), "{rows:?}");
    assert_eq!(buffer.get(0, 9).unwrap().bg, Some(Color::BLUE));
    assert_eq!(buffer.get(0, 9).unwrap().fg, Some(Color::WHITE));
    // Without labels the plot takes the last row
    let plain = render(&three().show_legend(false), 33, 10);
    assert!((0..33).any(|x| plain.get(x, 9).unwrap().symbol == '█'));
}

#[test]
fn background_fills_the_chart_rows() {
    let chart = three().bg(Color::BLUE).height(6).show_labels(false);
    let buffer = render(&chart, 20, 10);
    // Row 0 is the legend: its gaps and names keep the background
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLUE));
    assert_eq!(buffer.get(3, 0).unwrap().bg, Some(Color::BLUE));
    for y in 6..10 {
        assert_eq!(buffer.get(5, y).unwrap().bg, None, "row {y}");
    }
}

#[test]
fn an_empty_palette_falls_back_to_the_defaults() {
    let chart = three().palette(vec![]);
    assert_eq!(
        chart.get_layer_color(0),
        Streamline::new().get_layer_color(0)
    );
    assert_eq!(
        chart.get_layer_color(12),
        Streamline::new().get_layer_color(12)
    );
    let own = three().palette(vec![Color::RED]);
    assert_eq!(render(&own, 30, 8).get(1, 0).unwrap().fg, Some(Color::RED));
}

/// Layer values from the generator: positive, so each band has a height
fn layers_strategy() -> impl Strategy<Value = Vec<Vec<f64>>> {
    proptest::collection::vec(proptest::collection::vec(0.5f64..50.0, 0..7), 1..6)
}

fn baseline_strategy() -> impl Strategy<Value = StreamBaseline> {
    prop_oneof![
        Just(StreamBaseline::Zero),
        Just(StreamBaseline::Symmetric),
        Just(StreamBaseline::Wiggle),
        Just(StreamBaseline::Expand),
    ]
}

fn order_strategy() -> impl Strategy<Value = StreamOrder> {
    prop_oneof![
        Just(StreamOrder::None),
        Just(StreamOrder::Ascending),
        Just(StreamOrder::Descending),
        Just(StreamOrder::InsideOut),
    ]
}

/// Plain model of the stacking order: layers by their sum
fn model_order(values: &[Vec<f64>], order: StreamOrder) -> Vec<usize> {
    let sum = |i: usize| values[i].iter().sum::<f64>();
    let mut idx: Vec<usize> = (0..values.len()).collect();
    match order {
        StreamOrder::None => idx,
        StreamOrder::Ascending => {
            idx.sort_by(|&a, &b| sum(a).total_cmp(&sum(b)));
            idx
        }
        StreamOrder::Descending | StreamOrder::InsideOut => {
            idx.sort_by(|&a, &b| sum(b).total_cmp(&sum(a)));
            if order == StreamOrder::Descending {
                return idx;
            }
            // Largest in the middle, the rest alternating below and above
            let mut out = std::collections::VecDeque::new();
            for (i, v) in idx.into_iter().enumerate() {
                if i % 2 == 0 {
                    out.push_back(v);
                } else {
                    out.push_front(v);
                }
            }
            out.into()
        }
    }
}

fn chart_of(values: &[Vec<f64>], baseline: StreamBaseline, order: StreamOrder) -> Streamline {
    let mut chart = Streamline::new().baseline(baseline).order(order);
    for (i, v) in values.iter().enumerate() {
        chart = chart.layer(StreamLayer::new(format!("L{i}")).data(v.clone()));
    }
    chart
}

proptest! {
    /// compute_stacks against a model: at each point the layers are stacked
    /// in the model's order, each band as tall as its value (scaled for
    /// Expand), each starting where the one below ends, from the baseline.
    #[test]
    #[allow(clippy::needless_range_loop)] // x indexes the layers' values and stacks alike
    fn stacks_match_the_model(
        values in layers_strategy(),
        baseline in baseline_strategy(),
        order in order_strategy(),
    ) {
        let stacks = chart_of(&values, baseline, order).compute_stacks();
        let points = values.iter().map(Vec::len).max().unwrap();
        if points == 0 {
            prop_assert!(stacks.is_empty());
            return Ok(());
        }
        prop_assert_eq!(stacks.len(), values.len());
        let ordered = model_order(&values, order);
        let n = values.len() as f64;
        for x in 0..points {
            let v: Vec<f64> = ordered.iter().map(|&i| values[i].get(x).copied().unwrap_or(0.0)).collect();
            let total: f64 = v.iter().sum();
            let (start, scale) = match baseline {
                StreamBaseline::Zero => (0.0, 1.0),
                StreamBaseline::Symmetric => (-total / 2.0, 1.0),
                StreamBaseline::Wiggle => {
                    let weighted: f64 = v.iter().enumerate().map(|(i, &vi)| (n - i as f64) * vi).sum();
                    (-weighted / n / 2.0, 1.0)
                }
                StreamBaseline::Expand => (0.0, 1.0 / total),
            };
            let mut y = start;
            for (k, &layer) in ordered.iter().enumerate() {
                let (y0, y1) = stacks[layer][x];
                prop_assert!((y0 - y).abs() < 1e-9, "x={} layer {}: {} vs {}", x, layer, y0, y);
                prop_assert!((y1 - y0 - v[k] * scale).abs() < 1e-9);
                y = y1;
            }
            if baseline == StreamBaseline::Expand {
                prop_assert!((y - 1.0).abs() < 1e-9);
            }
        }
    }

    /// Any positive data draws inside its plot rows, with a band at every
    /// data point's column (negative values panic: draft-pk-c990).
    #[test]
    fn plot_draws_a_band_at_every_point(
        values in layers_strategy(),
        baseline in baseline_strategy(),
        order in order_strategy(),
        w in 5u16..60,
        h in 3u16..16,
        legend in any::<bool>(),
    ) {
        let chart = chart_of(&values, baseline, order).show_legend(legend).show_labels(false);
        let buffer = render(&chart, w, h);
        let points = values.iter().map(Vec::len).max().unwrap();
        let top = u16::from(legend);
        let plot_height = h - top;
        if points == 0 || plot_height < 2 {
            for y in top..h {
                prop_assert!((0..w).all(|x| buffer.get(x, y).unwrap().symbol == ' '));
            }
            return Ok(());
        }
        for i in 0..points {
            let x = (i as f64 / (points - 1).max(1) as f64 * (w - 1) as f64) as u16;
            prop_assert!(
                (top..h).any(|y| buffer.get(x, y).unwrap().symbol == '█'),
                "no band at point {} (col {})", i, x
            );
        }
    }
}
