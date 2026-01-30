//! Integration tests for Streamline widget

use revue::style::Color;
use revue::widget::{
    genre_stream, resource_stream, streamline, streamline_with_data, traffic_stream,
    StreamBaseline, StreamLayer, StreamOrder, Streamline,
};

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
    let _chart = Streamline::new();
}

#[test]
fn test_streamline_helper() {
    let _chart = streamline();
}

#[test]
fn test_streamline_layer() {
    let chart = streamline().layer(StreamLayer::new("Sales").data(vec![10.0, 20.0]));
    // Layer added successfully
}

#[test]
fn test_streamline_multiple_layers() {
    let chart = streamline()
        .layer(StreamLayer::new("Sales").data(vec![10.0, 20.0, 15.0]))
        .layer(StreamLayer::new("Marketing").data(vec![5.0, 8.0, 12.0]))
        .layer(StreamLayer::new("Support").data(vec![3.0, 4.0, 6.0]));
}

#[test]
fn test_streamline_baseline_zero() {
    let _chart = streamline().baseline(StreamBaseline::Zero);
}

#[test]
fn test_streamline_baseline_symmetric() {
    let _chart = streamline().baseline(StreamBaseline::Symmetric);
}

#[test]
fn test_streamline_baseline_wiggle() {
    let _chart = streamline().baseline(StreamBaseline::Wiggle);
}

#[test]
fn test_streamline_baseline_expand() {
    let _chart = streamline().baseline(StreamBaseline::Expand);
}

#[test]
fn test_streamline_order_none() {
    let _chart = streamline().order(StreamOrder::None);
}

#[test]
fn test_streamline_order_ascending() {
    let _chart = streamline().order(StreamOrder::Ascending);
}

#[test]
fn test_streamline_order_descending() {
    let _chart = streamline().order(StreamOrder::Descending);
}

#[test]
fn test_streamline_order_inside_out() {
    let _chart = streamline().order(StreamOrder::InsideOut);
}

#[test]
fn test_streamline_with_data() {
    let layers = vec![
        StreamLayer::new("Sales").data(vec![10.0, 20.0, 15.0, 25.0]),
        StreamLayer::new("Marketing").data(vec![5.0, 8.0, 12.0, 10.0]),
    ];
    let _chart = streamline_with_data(layers);
}

#[test]
fn test_genre_stream() {
    let data = vec![
        ("Rock", vec![10.0, 15.0, 12.0]),
        ("Pop", vec![8.0, 12.0, 10.0]),
        ("Jazz", vec![5.0, 7.0, 6.0]),
    ];
    let _chart = genre_stream(data);
}

#[test]
fn test_resource_stream() {
    let cpu = vec![50.0, 60.0, 55.0];
    let memory = vec![40.0, 45.0, 42.0];
    let disk = vec![30.0, 35.0, 32.0];
    let network = vec![20.0, 25.0, 22.0];
    let _chart = resource_stream(cpu, memory, disk, network);
}

#[test]
fn test_traffic_stream() {
    let data = vec![
        ("Direct", vec![100.0, 120.0, 110.0]),
        ("Organic", vec![80.0, 90.0, 85.0]),
        ("Referral", vec![40.0, 50.0, 45.0]),
    ];
    let _chart = traffic_stream(data);
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

    // Builder pattern works
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
