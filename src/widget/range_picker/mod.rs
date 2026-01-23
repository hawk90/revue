//! Date/time range picker widget
//!
//! Provides a widget for selecting date ranges with:
//! - Start and end date selection
//! - Common preset ranges (Today, Last 7 Days, etc.)
//! - Optional time selection
//! - Validation to ensure end >= start

mod core;
mod impls;
mod navigation;
mod tests;
mod types;
mod view;

pub use core::RangePicker;
pub use types::{PresetRange, RangeFocus};

pub use impls::analytics_range_picker;
pub use impls::date_range_picker;
pub use impls::range_picker;
