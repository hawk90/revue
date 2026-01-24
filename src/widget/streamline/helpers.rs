//! Helper functions for creating streamline charts

use super::{core::Streamline, types::StreamLayer};
use crate::style::Color;

/// Create a new streamline chart
pub fn streamline() -> Streamline {
    Streamline::new()
}

/// Create a streamline chart with layers
pub fn streamline_with_data(layers: Vec<StreamLayer>) -> Streamline {
    let mut chart = Streamline::new();
    for layer in layers {
        chart = chart.layer(layer);
    }
    chart
}

/// Create a music genre popularity stream graph
pub fn genre_stream(data: Vec<(&str, Vec<f64>)>) -> Streamline {
    let mut chart = Streamline::new()
        .title("Music Genre Trends")
        .baseline(super::types::StreamBaseline::Symmetric)
        .order(super::types::StreamOrder::InsideOut);

    let colors = [
        Color::rgb(231, 76, 60),
        Color::rgb(52, 152, 219),
        Color::rgb(46, 204, 113),
        Color::rgb(155, 89, 182),
        Color::rgb(241, 196, 15),
        Color::rgb(230, 126, 34),
    ];

    for (i, (name, values)) in data.into_iter().enumerate() {
        let layer = StreamLayer::new(name)
            .data(values)
            .color(colors[i % colors.len()]);
        chart = chart.layer(layer);
    }

    chart
}

/// Create a traffic source stream graph
pub fn traffic_stream(data: Vec<(&str, Vec<f64>)>) -> Streamline {
    let mut chart = Streamline::new()
        .title("Traffic Sources")
        .baseline(super::types::StreamBaseline::Expand)
        .order(super::types::StreamOrder::Descending);

    for (name, values) in data {
        chart = chart.layer(StreamLayer::new(name).data(values));
    }

    chart
}

/// Create a resource usage stream
pub fn resource_stream(
    cpu: Vec<f64>,
    memory: Vec<f64>,
    disk: Vec<f64>,
    network: Vec<f64>,
) -> Streamline {
    Streamline::new()
        .title("Resource Usage")
        .baseline(super::types::StreamBaseline::Zero)
        .layer(
            StreamLayer::new("CPU")
                .data(cpu)
                .color(Color::rgb(52, 152, 219)),
        )
        .layer(
            StreamLayer::new("Memory")
                .data(memory)
                .color(Color::rgb(155, 89, 182)),
        )
        .layer(
            StreamLayer::new("Disk")
                .data(disk)
                .color(Color::rgb(46, 204, 113)),
        )
        .layer(
            StreamLayer::new("Network")
                .data(network)
                .color(Color::rgb(241, 196, 15)),
        )
}
