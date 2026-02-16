//! Mermaid-style diagram rendering in ASCII
//!
//! Renders flowcharts, sequence diagrams, and other diagrams
//! using ASCII/Unicode art.

mod core;
mod helpers;
mod render;
mod types;

pub use core::Diagram;
pub use helpers::{diagram, edge, flowchart, node};
pub use types::{ArrowStyle, DiagramEdge, DiagramNode, DiagramType, NodeShape};

crate::impl_styled_view!(Diagram);
crate::impl_props_builders!(Diagram);
