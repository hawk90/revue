//! Style inspector for CSS debugging

mod computed_property;
mod core;
mod helper;
mod impls;
mod style_inspector;
mod tests;
mod types;
mod view;

pub use core::StyleInspector;
pub use types::{ComputedProperty, PropertySource, StyleCategory};
