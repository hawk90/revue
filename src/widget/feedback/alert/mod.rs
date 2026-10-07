//! Alert widget for persistent in-place feedback messages
//!
//! Unlike Toast (ephemeral notifications), Alert stays visible until dismissed.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Alert, AlertLevel, alert};
//!
//! // Basic alert
//! Alert::new("Operation completed successfully")
//!     .level(AlertLevel::Success);
//!
//! // With title and dismiss button
//! alert("Connection failed")
//!     .title("Network Error")
//!     .level(AlertLevel::Error)
//!     .dismissible(true);
//!
//! // Info alert with custom styling
//! Alert::info("Press Ctrl+S to save your work")
//!     .title("Tip");
//! ```

mod render;
mod types;

pub use types::{AlertLevel, AlertVariant};

use crate::event::Key;
use crate::widget::traits::{WidgetProps, WidgetState};
use crate::{impl_styled_view, impl_widget_builders};

/// A persistent alert/notification widget
///
/// Displays important messages that require user attention.
/// Unlike Toast, Alert stays visible until explicitly dismissed.
#[derive(Clone)]
pub struct Alert {
    /// Alert message
    message: String,
    /// Optional title
    title: Option<String>,
    /// Severity level
    level: AlertLevel,
    /// Visual variant
    variant: AlertVariant,
    /// Show icon
    show_icon: bool,
    /// Allow dismissing
    dismissible: bool,
    /// Whether alert is dismissed
    dismissed: bool,
    /// Custom icon override
    custom_icon: Option<char>,
    /// Widget state
    state: WidgetState,
    /// Widget properties
    props: WidgetProps,
}

impl Alert {
    /// Create a new alert with a message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            title: None,
            level: AlertLevel::default(),
            variant: AlertVariant::default(),
            show_icon: true,
            dismissible: false,
            dismissed: false,
            custom_icon: None,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Create an info alert
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(message).level(AlertLevel::Info)
    }

    /// Create a success alert
    pub fn success(message: impl Into<String>) -> Self {
        Self::new(message).level(AlertLevel::Success)
    }

    /// Create a warning alert
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(message).level(AlertLevel::Warning)
    }

    /// Create an error alert
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message).level(AlertLevel::Error)
    }

    /// Set the alert title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the severity level
    pub fn level(mut self, level: AlertLevel) -> Self {
        self.level = level;
        self
    }

    /// Set the visual variant
    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Show/hide the icon
    pub fn icon(mut self, show: bool) -> Self {
        self.show_icon = show;
        self
    }

    /// Set a custom icon
    pub fn custom_icon(mut self, icon: char) -> Self {
        self.custom_icon = Some(icon);
        self.show_icon = true;
        self
    }

    /// Make the alert dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Check if alert is dismissed
    pub fn is_dismissed(&self) -> bool {
        self.dismissed
    }

    /// Dismiss the alert
    pub fn dismiss(&mut self) {
        self.dismissed = true;
    }

    /// Reset dismissed state (show again)
    pub fn reset(&mut self) {
        self.dismissed = false;
    }

    /// Calculate the height needed for this alert
    pub fn height(&self) -> u16 {
        if self.dismissed {
            return 0;
        }
        // Title and message each take a row.
        let text_rows = if self.title.is_some() { 2 } else { 1 };
        match self.variant {
            // Rounded border above and below the text.
            AlertVariant::Filled => text_rows + 2,
            // Outlined is only the accent bar down the left edge, and Minimal
            // has no border at all: neither adds a row.
            AlertVariant::Outlined | AlertVariant::Minimal => text_rows,
        }
    }

    /// Handle keyboard input
    ///
    /// Returns `true` if the key was handled.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.dismissed || !self.dismissible {
            return false;
        }

        match key {
            Key::Char('x') | Key::Char('X') | Key::Escape => {
                self.dismiss();
                true
            }
            _ => false,
        }
    }
}

impl Default for Alert {
    fn default() -> Self {
        Self::new("Alert")
    }
}

impl_styled_view!(Alert);
impl_widget_builders!(Alert);

/// Helper function to create an Alert
pub fn alert(message: impl Into<String>) -> Alert {
    Alert::new(message)
}

/// Helper function to create an info Alert
pub fn info_alert(message: impl Into<String>) -> Alert {
    Alert::info(message)
}

/// Helper function to create a success Alert
pub fn success_alert(message: impl Into<String>) -> Alert {
    Alert::success(message)
}

/// Helper function to create a warning Alert
pub fn warning_alert(message: impl Into<String>) -> Alert {
    Alert::warning(message)
}

/// Helper function to create an error Alert
pub fn error_alert(message: impl Into<String>) -> Alert {
    Alert::error(message)
}
