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

mod builders;
mod helper;
mod input;
mod layout;
mod types;
mod view;

use crate::style::Color;
use crate::widget::traits::{WidgetProps, WidgetState};

// Re-exports
pub use types::{PopoverArrow, PopoverPosition, PopoverStyle, PopoverTrigger};

pub use helper::popover;

/// A popover widget for interactive overlays
///
/// Popovers are positioned relative to an anchor point and can contain
/// interactive content. They support various triggers and can be configured
/// to close on escape or click outside.
#[derive(Clone)]
pub struct Popover {
    /// Content text (for simple popovers)
    content: String,
    /// Anchor point (x, y)
    anchor: (u16, u16),
    /// Position relative to anchor
    position: PopoverPosition,
    /// Trigger type
    trigger: PopoverTrigger,
    /// Visual style
    popover_style: PopoverStyle,
    /// Arrow style
    arrow: PopoverArrow,
    /// Whether popover is open
    open: bool,
    /// Close on escape key
    close_on_escape: bool,
    /// Close when clicking outside
    close_on_click_outside: bool,
    /// Title (optional)
    title: Option<String>,
    /// Max width (0 = auto)
    max_width: u16,
    /// Custom border color
    border_color: Option<Color>,
    /// Widget state
    state: WidgetState,
    /// Widget properties
    props: WidgetProps,
}

impl Popover {
    /// Create a new popover with content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            anchor: (0, 0),
            position: PopoverPosition::Bottom,
            trigger: PopoverTrigger::Click,
            popover_style: PopoverStyle::Default,
            arrow: PopoverArrow::None,
            open: false,
            close_on_escape: true,
            close_on_click_outside: true,
            title: None,
            max_width: 40,
            border_color: None,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }
}

impl Default for Popover {
    fn default() -> Self {
        Self::new("")
    }
}

crate::impl_styled_view!(Popover);
crate::impl_widget_builders!(Popover);
