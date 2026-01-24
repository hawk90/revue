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

crate::impl_styled_view!(Streamline);
crate::impl_props_builders!(Streamline);
