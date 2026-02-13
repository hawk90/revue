//! Tests for streamline widget helper functions

use revue::widget::streamline::helpers::*;

#[test]
fn test_streamline_function() {
    let chart = streamline();
    let _ = chart;
}

#[test]
fn test_streamline_with_data_function() {
    let layers = vec![
        revue::widget::streamline::types::StreamLayer::new("Layer1").data(vec![1.0, 2.0, 3.0]),
        revue::widget::streamline::types::StreamLayer::new("Layer2").data(vec![2.0, 3.0, 4.0]),
    ];
    let chart = streamline_with_data(layers);
    let _ = chart;
}

#[test]
fn test_genre_stream_function() {
    let data = vec![("Rock", vec![1.0, 2.0, 3.0]), ("Pop", vec![2.0, 3.0, 4.0])];
    let chart = genre_stream(data);
    let _ = chart;
}

#[test]
fn test_traffic_stream_function() {
    let data = vec![
        ("Organic", vec![10.0, 20.0, 30.0]),
        ("Direct", vec![15.0, 25.0, 35.0]),
    ];
    let chart = traffic_stream(data);
    let _ = chart;
}

#[test]
fn test_resource_stream_function() {
    let chart = resource_stream(
        vec![10.0, 20.0],
        vec![30.0, 40.0],
        vec![50.0, 60.0],
        vec![70.0, 80.0],
    );
    let _ = chart;
}

#[test]
fn test_streamline_empty_data() {
    let layers: Vec<revue::widget::streamline::types::StreamLayer> = vec![];
    let chart = streamline_with_data(layers);
    let _ = chart;
}

#[test]
fn test_genre_stream_empty() {
    let data: Vec<(&str, Vec<f64>)> = vec![];
    let chart = genre_stream(data);
    let _ = chart;
}

#[test]
fn test_genre_stream_single_genre() {
    let data = vec![("Jazz", vec![5.0, 10.0, 15.0])];
    let chart = genre_stream(data);
    let _ = chart;
}

#[test]
fn test_genre_stream_many_genres() {
    let data = vec![
        ("Rock", vec![1.0, 2.0]),
        ("Pop", vec![2.0, 3.0]),
        ("Jazz", vec![3.0, 4.0]),
        ("Classical", vec![4.0, 5.0]),
        ("Electronic", vec![5.0, 6.0]),
        ("Hip-Hop", vec![6.0, 7.0]),
        ("Country", vec![7.0, 8.0]),
    ];
    let chart = genre_stream(data);
    let _ = chart;
}

#[test]
fn test_traffic_stream_empty() {
    let data: Vec<(&str, Vec<f64>)> = vec![];
    let chart = traffic_stream(data);
    let _ = chart;
}

#[test]
fn test_traffic_stream_single_source() {
    let data = vec![("Social", vec![100.0, 200.0])];
    let chart = traffic_stream(data);
    let _ = chart;
}

#[test]
fn test_resource_stream_empty_data() {
    let chart = resource_stream(vec![], vec![], vec![], vec![]);
    let _ = chart;
}

#[test]
fn test_resource_stream_single_resource() {
    let chart = resource_stream(vec![10.0], vec![], vec![], vec![]);
    let _ = chart;
}

#[test]
fn test_streamline_multiple() {
    let chart1 = streamline();
    let chart2 = streamline();
    let _ = chart1;
    let _ = chart2;
}

#[test]
fn test_helpers_do_not_panic() {
    // All helper functions should not panic with valid input
    let _ = streamline();
    let _ = streamline_with_data(vec![]);
    let _ = genre_stream(vec![]);
    let _ = traffic_stream(vec![]);
    let _ = resource_stream(vec![], vec![], vec![], vec![]);
}

#[test]
fn test_streamline_with_data_multiple_layers() {
    let layers = vec![
        revue::widget::streamline::types::StreamLayer::new("A").data(vec![1.0]),
        revue::widget::streamline::types::StreamLayer::new("B").data(vec![2.0]),
        revue::widget::streamline::types::StreamLayer::new("C").data(vec![3.0]),
        revue::widget::streamline::types::StreamLayer::new("D").data(vec![4.0]),
        revue::widget::streamline::types::StreamLayer::new("E").data(vec![5.0]),
    ];
    let chart = streamline_with_data(layers);
    let _ = chart;
}

#[test]
fn test_genre_stream_with_string_names() {
    let data = vec![
        (String::from("R&B"), vec![1.0, 2.0]),
        (String::from("Blues"), vec![2.0, 3.0]),
    ];
    let chart = genre_stream(data.iter().map(|(n, d)| (n.as_str(), d.clone())).collect());
    let _ = chart;
}