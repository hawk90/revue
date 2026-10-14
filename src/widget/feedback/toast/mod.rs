//! Toast notification widget
//!
//! Displays temporary notification messages with different severity levels.
//!
//! The struct and its builders live here; `types` has the level and position
//! enums, and `view` sizes, places and draws the toast.

mod types;
mod view;

pub use types::{ToastLevel, ToastPosition};

use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// A toast notification widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let toast = Toast::new("File saved successfully!")
///     .level(ToastLevel::Success)
///     .position(ToastPosition::TopRight);
/// ```
#[derive(Clone)]
pub struct Toast {
    message: String,
    level: ToastLevel,
    position: ToastPosition,
    width: Option<u16>,
    show_icon: bool,
    show_border: bool,
    props: WidgetProps,
}

impl Toast {
    /// Create a new toast with a message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            level: ToastLevel::default(),
            position: ToastPosition::default(),
            width: None,
            show_icon: true,
            show_border: true,
            props: WidgetProps::new(),
        }
    }

    /// Create an info toast
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(message).level(ToastLevel::Info)
    }

    /// Create a success toast
    pub fn success(message: impl Into<String>) -> Self {
        Self::new(message).level(ToastLevel::Success)
    }

    /// Create a warning toast
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(message).level(ToastLevel::Warning)
    }

    /// Create an error toast
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message).level(ToastLevel::Error)
    }

    /// Set the toast level
    pub fn level(mut self, level: ToastLevel) -> Self {
        self.level = level;
        self
    }

    /// Set the toast position
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    /// Set fixed width
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Show or hide the icon
    pub fn show_icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Show or hide the border
    pub fn show_border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    // Getters for testing
    #[doc(hidden)]
    pub fn get_message(&self) -> &str {
        &self.message
    }

    #[doc(hidden)]
    pub fn get_level(&self) -> ToastLevel {
        self.level
    }

    #[doc(hidden)]
    pub fn get_position(&self) -> ToastPosition {
        self.position
    }

    #[doc(hidden)]
    pub fn get_width(&self) -> Option<u16> {
        self.width
    }

    #[doc(hidden)]
    pub fn get_show_icon(&self) -> bool {
        self.show_icon
    }

    #[doc(hidden)]
    pub fn get_show_border(&self) -> bool {
        self.show_border
    }
}

impl_styled_view!(Toast);
impl_props_builders!(Toast);

/// Create a toast notification
pub fn toast(message: impl Into<String>) -> Toast {
    Toast::new(message)
}
