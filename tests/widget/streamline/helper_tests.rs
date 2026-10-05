//! Tests for the streamline preset constructors (`streamline_with_data`,
//! `genre_stream`, `traffic_stream`, `resource_stream`)

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{
    genre_stream, resource_stream, streamline, streamline_with_data, traffic_stream, StreamLayer,
    Streamline,
};

fn render_rows(chart: &Streamline, w: u16, h: u16) -> Vec<String> {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    chart.render(&mut ctx);
    (0..h)
        .map(|y| {
            (0..w)
                .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
                .collect()
        })
        .collect()
}

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn test_streamline_has_no_stacks() {
    assert!(streamline().compute_stacks().is_empty());
}

#[test]
fn test_streamline_with_data_keeps_every_layer() {
    let chart = streamline_with_data(vec![
        StreamLayer::new("A").data(vec![1.0, 2.0, 3.0]),
        StreamLayer::new("B").data(vec![2.0, 3.0, 4.0]),
        StreamLayer::new("C").data(vec![3.0]),
    ]);
    let stacks = chart.compute_stacks();
    assert_eq!(stacks.len(), 3);
    // Every layer gets one band per x position, padded to the longest layer
    assert!(stacks.iter().all(|s| s.len() == 3));
}

#[test]
fn test_streamline_with_data_empty() {
    assert!(streamline_with_data(vec![]).compute_stacks().is_empty());
}

#[test]
fn test_genre_stream_is_symmetric_around_zero() {
    let chart = genre_stream(vec![("Rock", vec![1.0, 2.0]), ("Pop", vec![3.0, 4.0])]);
    let stacks = chart.compute_stacks();
    assert_eq!(stacks.len(), 2);
    for x in 0..2 {
        let low = stacks.iter().map(|s| s[x].0).fold(f64::INFINITY, f64::min);
        let high = stacks
            .iter()
            .map(|s| s[x].1)
            .fold(f64::NEG_INFINITY, f64::max);
        assert!(approx(low, -high), "x={x}: {low} vs {high}");
    }
    // x = 0: total 4 -> spans -2..2
    let low0 = stacks.iter().map(|s| s[0].0).fold(f64::INFINITY, f64::min);
    assert!(approx(low0, -2.0));
}

#[test]
fn test_genre_stream_colors_cycle_after_six() {
    let names = ["a", "b", "c", "d", "e", "f", "g"];
    let chart = genre_stream(names.iter().map(|n| (*n, vec![1.0])).collect());
    assert_eq!(chart.get_layer_color(0), Color::rgb(231, 76, 60));
    assert_eq!(chart.get_layer_color(1), Color::rgb(52, 152, 219));
    assert_eq!(chart.get_layer_color(6), chart.get_layer_color(0));
    assert_ne!(chart.get_layer_color(5), chart.get_layer_color(0));
}

#[test]
fn test_genre_stream_empty() {
    assert!(genre_stream(vec![]).compute_stacks().is_empty());
}

#[test]
fn test_genre_stream_renders_title() {
    let chart = genre_stream(vec![("Jazz", vec![5.0, 10.0, 15.0])]);
    let rows = render_rows(&chart, 40, 10);
    assert!(rows[0].contains("Music Genre Trends"), "{rows:?}");
    assert!(rows[1].contains("Jazz"), "{rows:?}");
}

#[test]
fn test_traffic_stream_expands_to_unit_height() {
    let chart = traffic_stream(vec![
        ("Organic", vec![10.0, 20.0, 30.0]),
        ("Direct", vec![15.0, 25.0, 35.0]),
    ]);
    let stacks = chart.compute_stacks();
    for x in 0..3 {
        let low = stacks.iter().map(|s| s[x].0).fold(f64::INFINITY, f64::min);
        let high = stacks
            .iter()
            .map(|s| s[x].1)
            .fold(f64::NEG_INFINITY, f64::max);
        assert!(approx(low, 0.0));
        assert!(approx(high, 1.0));
    }
}

#[test]
fn test_traffic_stream_puts_largest_source_at_bottom() {
    let chart = traffic_stream(vec![("Small", vec![1.0, 1.0]), ("Large", vec![9.0, 9.0])]);
    let stacks = chart.compute_stacks();
    // Descending order: "Large" (index 1) is stacked first, from 0
    assert!(approx(stacks[1][0].0, 0.0));
    assert!(approx(stacks[1][0].1, 0.9));
    assert!(approx(stacks[0][0].0, 0.9));
}

#[test]
fn test_traffic_stream_empty() {
    assert!(traffic_stream(vec![]).compute_stacks().is_empty());
}

#[test]
fn test_resource_stream_stacks_from_zero_in_fixed_order() {
    let chart = resource_stream(
        vec![10.0, 20.0],
        vec![30.0, 40.0],
        vec![50.0, 60.0],
        vec![70.0, 80.0],
    );
    let stacks = chart.compute_stacks();
    assert_eq!(stacks.len(), 4);
    assert_eq!(stacks[0][0], (0.0, 10.0));
    assert_eq!(stacks[1][0], (10.0, 40.0));
    assert_eq!(stacks[2][0], (40.0, 90.0));
    assert_eq!(stacks[3][0], (90.0, 160.0));
}

#[test]
fn test_resource_stream_layer_colors() {
    let chart = resource_stream(vec![1.0], vec![1.0], vec![1.0], vec![1.0]);
    assert_eq!(chart.get_layer_color(0), Color::rgb(52, 152, 219));
    assert_eq!(chart.get_layer_color(1), Color::rgb(155, 89, 182));
    assert_eq!(chart.get_layer_color(2), Color::rgb(46, 204, 113));
    assert_eq!(chart.get_layer_color(3), Color::rgb(241, 196, 15));
}

#[test]
fn test_resource_stream_single_resource_pads_others_with_zero() {
    let stacks = resource_stream(vec![10.0], vec![], vec![], vec![]).compute_stacks();
    assert_eq!(stacks.len(), 4);
    assert_eq!(stacks[0][0], (0.0, 10.0));
    assert_eq!(stacks[3][0], (10.0, 10.0));
}

#[test]
fn test_resource_stream_empty_data() {
    assert!(resource_stream(vec![], vec![], vec![], vec![])
        .compute_stacks()
        .is_empty());
}

#[test]
fn test_resource_stream_renders_legend() {
    let chart = resource_stream(vec![1.0, 2.0], vec![1.0, 2.0], vec![1.0], vec![1.0]);
    let rows = render_rows(&chart, 60, 10);
    assert!(rows[0].contains("Resource Usage"), "{rows:?}");
    for name in ["CPU", "Memory", "Disk", "Network"] {
        assert!(rows[1].contains(name), "{name} missing: {rows:?}");
    }
}
