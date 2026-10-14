//! Streamline widget tests

mod helpers;
mod view;

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::genre_stream;
use revue::widget::resource_stream;
use revue::widget::streamline;
use revue::widget::streamline_with_data;
use revue::widget::traffic_stream;
use revue::widget::traits::RenderContext;
use revue::widget::StreamBaseline;
use revue::widget::StreamLayer;
use revue::widget::StreamOrder;
use revue::widget::Streamline;
use revue::widget::View;

fn render_buffer(chart: &Streamline, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    chart.render(&mut ctx);
    buffer
}

fn render_rows(chart: &Streamline, w: u16, h: u16) -> Vec<String> {
    let buffer = render_buffer(chart, w, h);
    (0..h)
        .map(|y| (0..w).map(|x| buffer.get(x, y).unwrap().symbol).collect())
        .collect()
}

/// A (2, 6) and B (4, 2)
fn two_layers() -> Streamline {
    streamline()
        .layer(StreamLayer::new("A").data(vec![2.0, 6.0]))
        .layer(StreamLayer::new("B").data(vec![4.0, 2.0]))
}

/// A (4), B (3), C (2), D (1) on a zero baseline
fn four_layers(order: StreamOrder) -> Streamline {
    let mut chart = streamline().order(order).baseline(StreamBaseline::Zero);
    for (name, v) in [("A", 4.0), ("B", 3.0), ("C", 2.0), ("D", 1.0)] {
        chart = chart.layer(StreamLayer::new(name).data(vec![v]));
    }
    chart
}

/// Each layer's band start at the first point
fn starts(stacks: &[Vec<(f64, f64)>]) -> Vec<f64> {
    stacks.iter().map(|s| s[0].0).collect()
}

#[test]
fn test_stream_layer_new() {
    let layer = StreamLayer::new("Test Layer");
    assert_eq!(layer.name, "Test Layer");
    assert!(layer.values.is_empty());
    assert!(layer.color.is_none());
}

#[test]
fn test_stream_layer_data() {
    let layer = StreamLayer::new("Sales").data(vec![10.0, 20.0, 15.0, 25.0]);
    assert_eq!(layer.values, vec![10.0, 20.0, 15.0, 25.0]);
}

#[test]
fn test_stream_layer_color() {
    let layer = StreamLayer::new("Marketing").color(Color::CYAN);
    assert_eq!(layer.color, Some(Color::CYAN));
}

#[test]
fn test_stream_layer_builder_pattern() {
    let layer = StreamLayer::new("Traffic")
        .data(vec![5.0, 8.0, 12.0, 10.0])
        .color(Color::rgb(100, 150, 200));

    assert_eq!(layer.name, "Traffic");
    assert_eq!(layer.values.len(), 4);
    assert!(layer.color.is_some());
}

#[test]
fn test_streamline_new() {
    let chart = Streamline::new();
    assert!(chart.compute_stacks().is_empty());
    // No layers: nothing on the screen
    assert!(render_rows(&chart, 20, 6)
        .iter()
        .all(|r| r.trim().is_empty()));
}

#[test]
fn test_streamline_helper() {
    assert_eq!(
        format!("{:?}", streamline()),
        format!("{:?}", Streamline::new())
    );
}

#[test]
fn test_streamline_layer() {
    let chart = streamline().layer(StreamLayer::new("Sales").data(vec![10.0, 20.0]));
    let rows = render_rows(&chart, 30, 8);
    assert!(rows[0].starts_with(" █ Sales"), "{rows:?}");
    // Both points have a band
    assert!(rows[1..].iter().any(|r| r.starts_with('█')), "{rows:?}");
    assert!(rows[1..].iter().any(|r| r.ends_with('█')), "{rows:?}");
}

#[test]
fn test_streamline_multiple_layers() {
    let chart = streamline()
        .layer(StreamLayer::new("Sales").data(vec![10.0, 20.0, 15.0]))
        .layer(StreamLayer::new("Marketing").data(vec![5.0, 8.0, 12.0]))
        .layer(StreamLayer::new("Support").data(vec![3.0, 4.0, 6.0]));
    assert_eq!(chart.compute_stacks().len(), 3);
    // The legend lists them in order, each swatch in the layer's color
    let rows = render_rows(&chart, 60, 10);
    assert!(
        rows[0].starts_with(" █ Sales  █ Marketing  █ Support"),
        "{rows:?}"
    );
    let buffer = render_buffer(&chart, 60, 10);
    for (i, x) in [(0, 1), (1, 10), (2, 23)] {
        assert_eq!(buffer.get(x, 0).unwrap().fg, Some(chart.get_layer_color(i)));
    }
}

#[test]
fn test_streamline_baseline_zero() {
    let chart = two_layers().baseline(StreamBaseline::Zero);
    let stacks = chart.compute_stacks();
    assert_eq!(stacks[0], vec![(0.0, 2.0), (0.0, 6.0)]);
    assert_eq!(stacks[1], vec![(2.0, 6.0), (6.0, 8.0)]);
}

#[test]
fn test_streamline_baseline_symmetric() {
    let chart = two_layers().baseline(StreamBaseline::Symmetric);
    let stacks = chart.compute_stacks();
    // Totals 6 and 8: centered on zero
    assert_eq!(stacks[0], vec![(-3.0, -1.0), (-4.0, 2.0)]);
    assert_eq!(stacks[1], vec![(-1.0, 3.0), (2.0, 4.0)]);
}

#[test]
fn test_streamline_baseline_wiggle() {
    let chart = two_layers().baseline(StreamBaseline::Wiggle);
    let stacks = chart.compute_stacks();
    // Start at -(2*v0 + 1*v1) / 2 / 2: x=0 -(4+4)/4 = -2, x=1 -(12+2)/4 = -3.5
    assert_eq!(stacks[0], vec![(-2.0, 0.0), (-3.5, 2.5)]);
    assert_eq!(stacks[1], vec![(0.0, 4.0), (2.5, 4.5)]);
}

#[test]
fn test_streamline_baseline_expand() {
    let chart = two_layers().baseline(StreamBaseline::Expand);
    let stacks = chart.compute_stacks();
    assert_eq!(stacks[0][0], (0.0, 2.0 / 6.0));
    assert_eq!(stacks[1][1].1, 1.0);
    // A zero total is not scaled
    let zeros = streamline()
        .baseline(StreamBaseline::Expand)
        .layer(StreamLayer::new("Z").data(vec![0.0]));
    assert_eq!(zeros.compute_stacks()[0][0], (0.0, 0.0));
}

#[test]
fn test_streamline_order_none() {
    // Stacked as added: A (4) at the bottom
    let stacks = four_layers(StreamOrder::None).compute_stacks();
    assert_eq!(starts(&stacks), [0.0, 4.0, 7.0, 9.0]);
}

#[test]
fn test_streamline_order_ascending() {
    // D (1), C (2), B (3), A (4) from the bottom
    let stacks = four_layers(StreamOrder::Ascending).compute_stacks();
    assert_eq!(starts(&stacks), [6.0, 3.0, 1.0, 0.0]);
}

#[test]
fn test_streamline_order_descending() {
    // A (4), B (3), C (2), D (1) from the bottom
    let stacks = four_layers(StreamOrder::Descending).compute_stacks();
    assert_eq!(starts(&stacks), [0.0, 4.0, 7.0, 9.0]);
}

#[test]
fn test_streamline_order_inside_out() {
    // The largest in the middle: D, B, A, C from the bottom
    let stacks = four_layers(StreamOrder::InsideOut).compute_stacks();
    assert_eq!(starts(&stacks), [4.0, 1.0, 8.0, 0.0]);
}

#[test]
fn test_streamline_with_data() {
    let layers = vec![
        StreamLayer::new("Sales").data(vec![10.0, 20.0, 15.0, 25.0]),
        StreamLayer::new("Marketing").data(vec![5.0, 8.0, 12.0, 10.0]),
    ];
    let chart = streamline_with_data(layers.clone());
    let chained = streamline().layers(layers);
    assert_eq!(chart.compute_stacks(), chained.compute_stacks());
    assert_eq!(render_rows(&chart, 40, 10), render_rows(&chained, 40, 10));
}

#[test]
fn test_genre_stream() {
    let data = vec![
        ("Rock", vec![10.0, 15.0, 12.0]),
        ("Pop", vec![8.0, 12.0, 10.0]),
        ("Jazz", vec![5.0, 7.0, 6.0]),
    ];
    let chart = genre_stream(data);
    assert_eq!(chart.title.as_deref(), Some("Music Genre Trends"));
    assert_eq!(chart.baseline, StreamBaseline::Symmetric);
    assert_eq!(chart.order, StreamOrder::InsideOut);
    let names: Vec<&str> = chart.layers.iter().map(|l| l.name.as_str()).collect();
    assert_eq!(names, ["Rock", "Pop", "Jazz"]);
}

#[test]
fn test_resource_stream() {
    let cpu = vec![50.0, 60.0, 55.0];
    let memory = vec![40.0, 45.0, 42.0];
    let disk = vec![30.0, 35.0, 32.0];
    let network = vec![20.0, 25.0, 22.0];
    let chart = resource_stream(cpu, memory, disk, network);
    assert_eq!(chart.title.as_deref(), Some("Resource Usage"));
    assert_eq!(chart.baseline, StreamBaseline::Zero);
    assert_eq!(chart.order, StreamOrder::None);
    let names: Vec<&str> = chart.layers.iter().map(|l| l.name.as_str()).collect();
    assert_eq!(names, ["CPU", "Memory", "Disk", "Network"]);
}

#[test]
fn test_traffic_stream() {
    let data = vec![
        ("Direct", vec![100.0, 120.0, 110.0]),
        ("Organic", vec![80.0, 90.0, 85.0]),
        ("Referral", vec![40.0, 50.0, 45.0]),
    ];
    let chart = traffic_stream(data);
    assert_eq!(chart.title.as_deref(), Some("Traffic Sources"));
    assert_eq!(chart.baseline, StreamBaseline::Expand);
    assert_eq!(chart.order, StreamOrder::Descending);
    assert_eq!(chart.layers.len(), 3);
}

#[test]
fn test_streamline_builder_pattern() {
    let chart = streamline()
        .layer(
            StreamLayer::new("Sales")
                .data(vec![10.0, 20.0])
                .color(Color::CYAN),
        )
        .layer(
            StreamLayer::new("Marketing")
                .data(vec![5.0, 8.0])
                .color(Color::BLUE),
        )
        .baseline(StreamBaseline::Symmetric)
        .order(StreamOrder::Descending);

    // The layer colors reach the legend swatches: " █ Sales  █ Marketing"
    let buffer = render_buffer(&chart, 40, 10);
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(Color::CYAN));
    assert_eq!(buffer.get(10, 0).unwrap().fg, Some(Color::BLUE));
}

#[test]
fn test_stream_layer_empty_data() {
    let layer = StreamLayer::new("Empty").data(vec![]);
    assert!(layer.values.is_empty());
}

#[test]
fn test_stream_layer_single_value() {
    let layer = StreamLayer::new("Single").data(vec![42.0]);
    assert_eq!(layer.values, vec![42.0]);
}

#[test]
fn test_stream_baseline_default() {
    assert_eq!(StreamBaseline::default(), StreamBaseline::Symmetric);
}

#[test]
fn test_stream_order_default() {
    assert_eq!(StreamOrder::default(), StreamOrder::None);
}

#[test]
fn test_stream_layer_with_name_str() {
    let layer = StreamLayer::new(String::from("Dynamic"));
    assert_eq!(layer.name, "Dynamic");
}

#[test]
fn test_stream_layer_negative_values() {
    let layer = StreamLayer::new("Losses").data(vec![-5.0, -10.0, -3.0]);
    assert_eq!(layer.values, vec![-5.0, -10.0, -3.0]);
}

#[test]
fn test_stream_layer_mixed_values() {
    let layer = StreamLayer::new("Mixed").data(vec![10.0, -5.0, 15.0, -3.0]);
    assert_eq!(layer.values, vec![10.0, -5.0, 15.0, -3.0]);
}
