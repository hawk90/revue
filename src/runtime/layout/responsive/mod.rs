#![allow(clippy::should_implement_trait, clippy::module_inception)]
//! Responsive layout breakpoints for terminal applications
//!
//! Provides a CSS-like breakpoint system for adapting layouts to different
//! terminal sizes.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::layout::{Breakpoints, Breakpoint, ResponsiveValue};
//!
//! // Create custom breakpoints
//! let bp = Breakpoints::new()
//!     .add(Breakpoint::new("sm", 40))
//!     .add(Breakpoint::new("md", 80))
//!     .add(Breakpoint::new("lg", 120));
//!
//! // Get current breakpoint for terminal width
//! let current = bp.current(100);
//! assert_eq!(current.name, "md");
//!
//! // Responsive values
//! let columns = ResponsiveValue::new(1)
//!     .at("sm", 2)
//!     .at("md", 3)
//!     .at("lg", 4);
//!
//! let cols = columns.resolve(&bp, 100);
//! assert_eq!(cols, 3);
//! ```

mod breakpoint;
mod container;
mod viewport;

pub use breakpoint::{breakpoints, responsive, Breakpoint, Breakpoints, ResponsiveValue};
pub use container::{
    container_max_width, container_min_width, ContainerQuery, ResponsiveContainer,
};
pub use viewport::{
    max_width, min_width, responsive, responsive_layout, MediaQuery, ResponsiveLayout,
};
