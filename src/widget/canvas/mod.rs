//! Canvas widget for custom drawing
//!
//! Provides a drawing surface for rendering custom graphics like
//! Gantt charts, diagrams, graphs, etc.
//!
//! Supports two rendering modes:
//! - **Character mode**: Standard character-based drawing
//! - **Braille mode**: High-resolution drawing using braille patterns (2x4 dots per cell)

mod braille;
mod clip;
mod draw;
mod grid;
mod layer;
#[cfg(test)]
mod tests;
mod transform;
mod widget;

pub use braille::{
    Arc, BrailleGrid, Circle, FilledCircle, FilledPolygon, FilledRectangle, Line, Points, Polygon,
    Rectangle, Shape,
};
pub use clip::ClipRegion;
pub use draw::DrawContext;
pub use layer::Layer;
pub use transform::Transform;
pub use widget::{braille_canvas, canvas, BrailleCanvas, Canvas};

// Re-export braille context at the top level
pub use braille::BrailleContext;
