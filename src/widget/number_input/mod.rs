//! Number input widget with increment/decrement controls
//!
//! Provides a numeric input field with:
//! - Up/Down arrow key controls for increment/decrement
//! - Direct numeric entry
//! - Min/max value constraints
//! - Configurable step size and precision
//! - Optional prefix/suffix display (e.g., "$", "%")

mod core;
mod helper;

#[cfg(test)]
#[cfg(test)]
mod tests;

// Re-exports
pub use core::NumberInput;
pub use helper::{currency_input, integer_input, number_input, percentage_input};
