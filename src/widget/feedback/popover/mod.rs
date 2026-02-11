//! Popover widget for anchor-positioned interactive overlays
//!
//! Unlike Tooltip (hover-only), Popover supports click triggers, focus trapping,
//! and interactive content. Essential for DatePicker, Combobox, etc.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Popover, PopoverPosition, popover};
//!
//! // Basic popover
//! Popover::new("Click me for details")
//!     .anchor(10, 5)
//!     .position(PopoverPosition::Bottom);
//!
//! // Interactive popover with trigger
//! popover("Menu content")
//!     .trigger(PopoverTrigger::Click)
//!     .close_on_escape(true)
//!     .close_on_click_outside(true);
//! ```

mod core;
mod impls;
mod types;

// Re-exports
pub use types::{PopoverArrow, PopoverPosition, PopoverStyle, PopoverTrigger};

pub use core::Popover;
pub use impls::popover;
