#![allow(clippy::needless_range_loop)]
//! Streamline Chart Widget (Stream Graph / ThemeRiver)
//!
//! A stacked area chart with smooth, flowing layers that shows how multiple
//! categories contribute to a total over time. Also known as stream graph
//! or ThemeRiver visualization.
//!
//! # Features
//!
//! - Multiple stacked layers with smooth transitions
//! - Various baseline modes (zero, symmetric, weighted)
//! - Automatic color assignment for layers
//! - Labels for each stream
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{streamline, StreamLayer};
//!
//! let chart = streamline()
//!     .layer(StreamLayer::new("Sales").data(vec![10.0, 20.0, 15.0, 25.0]))
//!     .layer(StreamLayer::new("Marketing").data(vec![5.0, 8.0, 12.0, 10.0]))
//!     .baseline(StreamBaseline::Symmetric);
//! ```

mod core;
mod helpers;
mod types;
mod view;

pub use core::Streamline;
pub use helpers::{
    genre_stream, resource_stream, streamline, streamline_with_data, traffic_stream,
};
pub use types::{StreamBaseline, StreamLayer, StreamOrder};

// KEEP HERE - Private implementation tests (all tests access private fields: layers, baseline, order, show_legend, title, palette, x_labels, bg_color, height, highlighted, and private methods: get_layer_color, compute_stacks)
#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::{RenderContext, View};

    #[test]
    fn test_streamline_new() {
        let s = Streamline::new();
        assert!(s.layers.is_empty());
        assert_eq!(s.baseline, StreamBaseline::Symmetric);
        assert_eq!(s.order, StreamOrder::None);
        assert!(s.show_legend);
        assert!(s.show_labels);
        assert!(s.title.is_none());
        assert!(s.bg_color.is_none());
        assert!(s.height.is_none());
        assert!(s.highlighted.is_none());
        assert!(!s.palette.is_empty());
    }

    #[test]
    fn test_streamline_default() {
        let s = Streamline::default();
        assert!(s.layers.is_empty());
        assert_eq!(s.baseline, StreamBaseline::Symmetric);
    }

    #[test]
    fn test_streamline_title() {
        let s = Streamline::new().title("Test Chart");
        assert_eq!(s.title, Some("Test Chart".to_string()));
    }

    #[test]
    fn test_streamline_layer() {
        let s = Streamline::new().layer(StreamLayer::new("Layer1").data(vec![1.0, 2.0, 3.0]));
        assert_eq!(s.layers.len(), 1);
        assert_eq!(s.layers[0].name, "Layer1");
    }

    #[test]
    fn test_streamline_layers() {
        let layers = vec![
            StreamLayer::new("A").data(vec![1.0]),
            StreamLayer::new("B").data(vec![2.0]),
        ];
        let s = Streamline::new().layers(layers);
        assert_eq!(s.layers.len(), 2);
    }

    #[test]
    fn test_streamline_baseline() {
        let s = Streamline::new().baseline(StreamBaseline::Zero);
        assert_eq!(s.baseline, StreamBaseline::Zero);
    }

    #[test]
    fn test_streamline_order() {
        let s = Streamline::new().order(StreamOrder::Ascending);
        assert_eq!(s.order, StreamOrder::Ascending);
    }

    #[test]
    fn test_streamline_show_legend() {
        let s = Streamline::new().show_legend(false);
        assert!(!s.show_legend);
    }

    #[test]
    fn test_streamline_show_labels() {
        let s = Streamline::new().show_labels(false);
        assert!(!s.show_labels);
    }

    #[test]
    fn test_streamline_x_labels() {
        let labels = vec!["Jan".to_string(), "Feb".to_string()];
        let s = Streamline::new().x_labels(labels);
        assert_eq!(s.x_labels.len(), 2);
    }

    #[test]
    fn test_streamline_bg_color() {
        use crate::style::Color;
        let s = Streamline::new().bg(Color::BLACK);
        assert_eq!(s.bg_color, Some(Color::BLACK));
    }

    #[test]
    fn test_streamline_height() {
        let s = Streamline::new().height(20);
        assert_eq!(s.height, Some(20));
    }

    #[test]
    fn test_streamline_palette() {
        use crate::style::Color;
        let palette = vec![Color::RED, Color::GREEN, Color::BLUE];
        let s = Streamline::new().palette(palette);
        assert_eq!(s.palette.len(), 3);
    }

    #[test]
    fn test_streamline_highlight() {
        let s = Streamline::new().highlight(2);
        assert_eq!(s.highlighted, Some(2));
    }

    #[test]
    fn test_streamline_get_layer_color() {
        use crate::style::Color;
        let s = Streamline::new()
            .layer(StreamLayer::new("L1").data(vec![1.0]))
            .layer(StreamLayer::new("L2").data(vec![2.0]).color(Color::RED));

        // First layer uses palette
        let c1 = s.get_layer_color(0);
        assert_ne!(c1, Color::RED);

        // Second layer has custom color
        let c2 = s.get_layer_color(1);
        assert_eq!(c2, Color::RED);
    }

    #[test]
    fn test_streamline_get_layer_color_wraps() {
        use crate::style::Color;

        // Test that palette wraps around
        let s = Streamline::new().palette(vec![Color::rgb(1, 1, 1), Color::rgb(2, 2, 2)]);

        let c0 = s.get_layer_color(0);
        let c1 = s.get_layer_color(1);
        let c2 = s.get_layer_color(2); // Should wrap to index 0
        let c3 = s.get_layer_color(3); // Should wrap to index 1

        assert_eq!(c0, s.palette[0]);
        assert_eq!(c1, s.palette[1]);
        assert_eq!(c2, s.palette[0]);
        assert_eq!(c3, s.palette[1]);
    }

    #[test]
    fn test_streamline_compute_stacks_empty() {
        let s = Streamline::new();
        let stacks = s.compute_stacks();
        assert!(stacks.is_empty());
    }

    #[test]
    fn test_streamline_compute_stacks_no_points() {
        let s = Streamline::new().layer(StreamLayer::new("L1").data(vec![]));
        let stacks = s.compute_stacks();
        assert!(stacks.is_empty());
    }

    #[test]
    fn test_streamline_compute_stacks_single_layer() {
        let s = Streamline::new().layer(StreamLayer::new("L1").data(vec![10.0, 20.0, 30.0]));
        let stacks = s.compute_stacks();
        assert_eq!(stacks.len(), 1);
        assert_eq!(stacks[0].len(), 3);
        // With Symmetric baseline, should be centered at 0
        assert_eq!(stacks[0][0].0, -5.0); // -10/2
        assert_eq!(stacks[0][0].1, 5.0); // -10/2 + 10
    }

    #[test]
    fn test_streamline_compute_stacks_two_layers() {
        let s = Streamline::new()
            .layer(StreamLayer::new("L1").data(vec![10.0, 20.0]))
            .layer(StreamLayer::new("L2").data(vec![5.0, 10.0]));
        let stacks = s.compute_stacks();
        assert_eq!(stacks.len(), 2);

        // Check first point
        let total = 15.0; // 10 + 5
        assert_eq!(stacks[0][0].0, -total / 2.0); // Start of L1
        assert_eq!(stacks[0][0].1, -total / 2.0 + 10.0); // End of L1
        assert_eq!(stacks[1][0].0, -total / 2.0 + 10.0); // Start of L2
        assert_eq!(stacks[1][0].1, -total / 2.0 + 15.0); // End of L2 (top)
    }

    #[test]
    fn test_streamline_compute_stacks_zero_baseline() {
        let s = Streamline::new()
            .baseline(StreamBaseline::Zero)
            .layer(StreamLayer::new("L1").data(vec![10.0]));
        let stacks = s.compute_stacks();
        assert_eq!(stacks[0][0].0, 0.0);
        assert_eq!(stacks[0][0].1, 10.0);
    }

    #[test]
    fn test_streamline_compute_stacks_expand_baseline() {
        let s = Streamline::new()
            .baseline(StreamBaseline::Expand)
            .layer(StreamLayer::new("L1").data(vec![10.0]))
            .layer(StreamLayer::new("L2").data(vec![20.0]));
        let stacks = s.compute_stacks();
        // Total is 30, scale is 1/30
        // L1: 0 to 10/30 = 0.333...
        // L2: 0.333... to 1.0
        assert!((stacks[0][0].0 - 0.0).abs() < 0.01);
        assert!((stacks[0][0].1 - 10.0 / 30.0).abs() < 0.01);
    }

    #[test]
    fn test_streamline_order_none() {
        let s = Streamline::new()
            .order(StreamOrder::None)
            .layer(StreamLayer::new("L1").data(vec![30.0]))
            .layer(StreamLayer::new("L2").data(vec![10.0]))
            .layer(StreamLayer::new("L3").data(vec![20.0]));

        let stacks = s.compute_stacks();
        // Order should be L1, L2, L3 (as added)
        // Total: 30 + 10 + 20 = 60
        // L1 starts at -30, ends at 0
        assert_eq!(stacks[0][0].0, -30.0);
        assert_eq!(stacks[0][0].1, 0.0);
    }

    #[test]
    fn test_streamline_order_ascending() {
        let s = Streamline::new()
            .order(StreamOrder::Ascending)
            .layer(StreamLayer::new("L1").data(vec![30.0]))
            .layer(StreamLayer::new("L2").data(vec![10.0]))
            .layer(StreamLayer::new("L3").data(vec![20.0]));

        let stacks = s.compute_stacks();
        // Order should be L2 (10), L3 (20), L1 (30)
        // With Symmetric baseline, total = 60, y0 = -60/2 = -30
        assert_eq!(stacks[1][0].0, -30.0); // L2 starts at -30
        assert_eq!(stacks[1][0].1, -20.0); // L2 ends at -30 + 10
    }

    #[test]
    fn test_streamline_order_descending() {
        let s = Streamline::new()
            .order(StreamOrder::Descending)
            .layer(StreamLayer::new("L1").data(vec![30.0]))
            .layer(StreamLayer::new("L2").data(vec![10.0]))
            .layer(StreamLayer::new("L3").data(vec![20.0]));

        let stacks = s.compute_stacks();
        // Order should be L1 (30), L3 (20), L2 (10)
        // Largest (L1) is first in stack
        assert_eq!(stacks[0][0].0, -30.0); // L1 starts at -60/2
        assert_eq!(stacks[0][0].1, 0.0); // L1 ends at -30 + 30 = 0
    }

    #[test]
    fn test_streamline_render() {
        let s = Streamline::new()
            .layer(StreamLayer::new("L1").data(vec![10.0, 20.0]))
            .height(10);

        let mut buffer = Buffer::new(50, 15);
        let area = Rect::new(0, 0, 50, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not panic
        s.render(&mut ctx);
    }

    #[test]
    fn test_streamline_render_empty() {
        let s = Streamline::new();

        let mut buffer = Buffer::new(50, 15);
        let area = Rect::new(0, 0, 50, 15);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not panic
        s.render(&mut ctx);
    }

    // =========================================================================
    // StreamLayer Tests
    // =========================================================================

    #[test]
    fn test_stream_layer_new() {
        let layer = StreamLayer::new("Test");
        assert_eq!(layer.name, "Test");
        assert!(layer.values.is_empty());
        assert!(layer.color.is_none());
    }

    #[test]
    fn test_stream_layer_data() {
        let layer = StreamLayer::new("Test").data(vec![1.0, 2.0, 3.0]);
        assert_eq!(layer.values.len(), 3);
        assert_eq!(layer.values[0], 1.0);
    }

    #[test]
    fn test_stream_layer_color() {
        use crate::style::Color;
        let layer = StreamLayer::new("Test").color(Color::BLUE);
        assert_eq!(layer.color, Some(Color::BLUE));
    }

    #[test]
    fn test_stream_layer_chain() {
        use crate::style::Color;
        let layer = StreamLayer::new("Test")
            .data(vec![1.0, 2.0])
            .color(Color::RED);
        assert_eq!(layer.name, "Test");
        assert_eq!(layer.values.len(), 2);
        assert_eq!(layer.color, Some(Color::RED));
    }

    // =========================================================================
    // Enum Tests
    // =========================================================================

    #[test]
    fn test_stream_baseline_default() {
        assert_eq!(StreamBaseline::default(), StreamBaseline::Symmetric);
    }

    #[test]
    fn test_stream_baseline_variants() {
        let _ = StreamBaseline::Zero;
        let _ = StreamBaseline::Symmetric;
        let _ = StreamBaseline::Wiggle;
        let _ = StreamBaseline::Expand;
    }

    #[test]
    fn test_stream_baseline_traits() {
        // Test Clone, Copy, PartialEq, Eq, Debug
        let b1 = StreamBaseline::Zero;
        let b2 = b1;
        assert_eq!(b1, StreamBaseline::Zero);
        assert_eq!(b1, b2);
        assert_eq!(b2, StreamBaseline::Zero);

        let b3 = StreamBaseline::Symmetric;
        assert_ne!(b1, b3);
    }

    #[test]
    fn test_stream_order_default() {
        assert_eq!(StreamOrder::default(), StreamOrder::None);
    }

    #[test]
    fn test_stream_order_variants() {
        let _ = StreamOrder::None;
        let _ = StreamOrder::Ascending;
        let _ = StreamOrder::Descending;
        let _ = StreamOrder::InsideOut;
    }

    #[test]
    fn test_stream_order_traits() {
        // Test Clone, Copy, PartialEq, Eq, Debug
        let o1 = StreamOrder::Ascending;
        let o2 = o1;
        assert_eq!(o1, StreamOrder::Ascending);
        assert_eq!(o1, o2);

        let o3 = StreamOrder::Descending;
        assert_ne!(o1, o3);
    }

    // =========================================================================
    // Helper Function Tests
    // =========================================================================

    #[test]
    fn test_streamline_helper() {
        let s = streamline();
        assert!(s.layers.is_empty());
    }

    #[test]
    fn test_streamline_clone() {
        let s1 = Streamline::new()
            .title("Test")
            .layer(StreamLayer::new("L1").data(vec![1.0]));
        let s2 = s1.clone();
        assert_eq!(s1.title, s2.title);
        assert_eq!(s1.layers.len(), s2.layers.len());
    }

    #[test]
    fn test_streamline_debug() {
        let s = Streamline::new();
        let _ = format!("{:?}", s);
    }

    #[test]
    fn test_stream_layer_debug() {
        let layer = StreamLayer::new("Test").data(vec![1.0, 2.0]);
        let _ = format!("{:?}", layer);
    }

    #[test]
    fn test_streamline_varying_length_layers() {
        let s = Streamline::new()
            .layer(StreamLayer::new("L1").data(vec![1.0, 2.0, 3.0]))
            .layer(StreamLayer::new("L2").data(vec![4.0, 5.0]));

        let stacks = s.compute_stacks();
        // Should handle varying lengths
        assert_eq!(stacks.len(), 2);
        assert_eq!(stacks[0].len(), 3); // Max length
        assert_eq!(stacks[1].len(), 3);
    }

    #[test]
    fn test_streamline_compute_stacks_negative_values() {
        let s = Streamline::new().layer(StreamLayer::new("L1").data(vec![-10.0, -5.0]));

        let stacks = s.compute_stacks();
        // Should handle negative values
        assert_eq!(stacks[0][0].0, 5.0); // -(-10)/2 = 5
        assert_eq!(stacks[0][0].1, -5.0); // 5 + (-10) = -5
    }

    #[test]
    fn test_streamline_compute_stacks_mixed_values() {
        let s = Streamline::new().layer(StreamLayer::new("L1").data(vec![-10.0, 5.0, 15.0]));

        let stacks = s.compute_stacks();
        // First point: -10
        assert!((stacks[0][0].0 - 5.0).abs() < 0.01);
        // Second point: 5
        assert!((stacks[0][1].0 - (-2.5)).abs() < 0.01);
        assert!((stacks[0][1].1 - 2.5).abs() < 0.01);
    }

    #[test]
    fn test_streamline_zero_values() {
        let s = Streamline::new().layer(StreamLayer::new("L1").data(vec![0.0, 0.0, 0.0]));

        let stacks = s.compute_stacks();
        assert_eq!(stacks[0][0].0, 0.0);
        assert_eq!(stacks[0][0].1, 0.0);
    }

    #[test]
    fn test_streamline_layer_name_types() {
        // Test with &str
        let l1 = StreamLayer::new("String slice");
        assert_eq!(l1.name, "String slice");

        // Test with String
        let name = String::from("Owned string");
        let l2 = StreamLayer::new(name.clone());
        assert_eq!(l2.name, name);

        // Test with &String
        let name = String::from("Ref to owned");
        let l3 = StreamLayer::new(&name);
        assert_eq!(l3.name, "Ref to owned");
    }

    #[test]
    fn test_streamline_title_types() {
        // &str
        let s1 = Streamline::new().title("String slice");
        assert_eq!(s1.title, Some("String slice".to_string()));

        // String
        let title = String::from("Owned");
        let s2 = Streamline::new().title(title.clone());
        assert_eq!(s2.title, Some(title));

        // &String
        let title = String::from("Ref");
        let s3 = Streamline::new().title(&title);
        assert_eq!(s3.title, Some("Ref".to_string()));
    }

    #[test]
    fn test_streamline_empty_layer_data() {
        let s = Streamline::new().layer(StreamLayer::new("Empty").data(vec![]));

        let stacks = s.compute_stacks();
        // Empty data should return empty stacks
        assert!(stacks.is_empty());
    }

    #[test]
    fn test_streamline_all_empty_layers() {
        let s = Streamline::new()
            .layer(StreamLayer::new("E1").data(vec![]))
            .layer(StreamLayer::new("E2").data(vec![]));

        let stacks = s.compute_stacks();
        assert!(stacks.is_empty());
    }

    #[test]
    fn test_streamline_single_point() {
        let s = Streamline::new().layer(StreamLayer::new("L1").data(vec![42.0]));

        let stacks = s.compute_stacks();
        assert_eq!(stacks.len(), 1);
        assert_eq!(stacks[0].len(), 1);
        assert_eq!(stacks[0][0].0, -21.0);
        assert_eq!(stacks[0][0].1, 21.0);
    }

    #[test]
    fn test_streamline_many_layers() {
        let mut s = Streamline::new();
        for i in 0..20 {
            s = s.layer(StreamLayer::new(format!("L{}", i)).data(vec![1.0]));
        }

        assert_eq!(s.layers.len(), 20);
        let stacks = s.compute_stacks();
        assert_eq!(stacks.len(), 20);
    }
}

crate::impl_styled_view!(Streamline);
crate::impl_props_builders!(Streamline);
