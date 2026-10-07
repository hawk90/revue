//! Status Indicator widget for displaying online/offline/busy states
//!
//! Provides visual feedback for connection status, user availability, or system health.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{StatusIndicator, Status, status_indicator};
//!
//! // Basic online indicator
//! StatusIndicator::online();
//!
//! // With label
//! StatusIndicator::busy().label("Do not disturb");
//!
//! // Custom status
//! status_indicator(Status::Away)
//!     .size(StatusSize::Large)
//!     .pulsing(true);
//! ```
//!
//! # Pulsing needs the app to drive it
//!
//! `pulsing(true)` alone does not animate. The widget owns no clock: it
//! blinks on an 8-frame cycle (dot shown for 6 frames, hidden for 2), and
//! the frame only moves when the app moves it.
//!
//! - An indicator the app keeps between renders can call [`StatusIndicator::tick`].
//! - An indicator built fresh inside `render` (the usual case) starts at
//!   frame 0 every time, so `tick()` on it never shows. Keep the frame in
//!   your app state, advance it when `Event::Tick` arrives, and pass it in
//!   with [`StatusIndicator::frame`].
//!
//! Either way the app must also redraw on `Event::Tick`: return `true` from
//! the `App::run` handler for it. `App::run_with_handler` only sees key
//! events, so it never redraws on a tick.
//!
//! ```rust,ignore
//! // Advance about every 100ms for a ~0.8s blink.
//! Event::Tick if last_step.elapsed() >= Duration::from_millis(100) => {
//!     last_step = Instant::now();
//!     pulse_frame = pulse_frame.wrapping_add(1);
//!     true
//! }
//! // ... in render:
//! StatusIndicator::busy().pulsing(true).frame(pulse_frame)
//! ```

mod render;
mod types;

pub use types::{Status, StatusSize, StatusStyle};

use crate::style::Color;
use crate::widget::traits::{WidgetProps, WidgetState};
use crate::{impl_styled_view, impl_widget_builders};

/// A status indicator widget for displaying availability/connection states
///
/// Shows online/offline/busy states with consistent visual styling.
#[derive(Clone)]
pub struct StatusIndicator {
    /// Current status
    status: Status,
    /// Size variant
    size: StatusSize,
    /// Display style
    style: StatusStyle,
    /// Custom label (overrides default)
    custom_label: Option<String>,
    /// Enable pulsing animation
    pulsing: bool,
    /// Animation frame counter
    frame: usize,
    /// Widget state
    state: WidgetState,
    /// Widget properties
    props: WidgetProps,
}

impl StatusIndicator {
    /// Create a new status indicator with the given status
    pub fn new(status: Status) -> Self {
        Self {
            status,
            size: StatusSize::default(),
            style: StatusStyle::default(),
            custom_label: None,
            pulsing: false,
            frame: 0,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Create an online status indicator
    pub fn online() -> Self {
        Self::new(Status::Online)
    }

    /// Create an offline status indicator
    pub fn offline() -> Self {
        Self::new(Status::Offline)
    }

    /// Create a busy status indicator
    pub fn busy() -> Self {
        Self::new(Status::Busy)
    }

    /// Create an away status indicator
    pub fn away() -> Self {
        Self::new(Status::Away)
    }

    /// Create an unknown status indicator
    pub fn unknown() -> Self {
        Self::new(Status::Unknown)
    }

    /// Create an error status indicator
    pub fn error() -> Self {
        Self::new(Status::Error)
    }

    /// Create a custom status indicator with a specific color
    pub fn custom(color: Color) -> Self {
        Self::new(Status::Custom(color))
    }

    /// Set the status
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    /// Set the size
    pub fn size(mut self, size: StatusSize) -> Self {
        self.size = size;
        self
    }

    /// Set the display style
    pub fn indicator_style(mut self, style: StatusStyle) -> Self {
        self.style = style;
        self
    }

    /// Set a custom label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.custom_label = Some(label.into());
        self
    }

    /// Enable/disable pulsing animation
    ///
    /// The pulse only advances when the app advances it, through
    /// [`tick`](Self::tick) or [`frame`](Self::frame). See the module docs.
    pub fn pulsing(mut self, pulsing: bool) -> Self {
        self.pulsing = pulsing;
        self
    }

    /// Set the animation frame
    ///
    /// For an indicator rebuilt on every render: keep the frame in app state,
    /// advance it on `Event::Tick`, and hand it in here. Frames `n % 8` of 6
    /// and 7 hide the dot when [`pulsing`](Self::pulsing) is on.
    pub fn frame(mut self, frame: usize) -> Self {
        self.frame = frame;
        self
    }

    /// Advance the animation frame by one
    ///
    /// Nothing calls this for you. Call it from the app (typically on
    /// `Event::Tick`) on an indicator that lives across renders, and redraw.
    pub fn tick(&mut self) {
        self.frame = self.frame.wrapping_add(1);
    }

    /// Get current status
    pub fn get_status(&self) -> Status {
        self.status
    }

    /// Set status mutably
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    /// Get the label to display
    fn get_label(&self) -> &str {
        self.custom_label
            .as_deref()
            .unwrap_or_else(|| self.status.label())
    }

    /// Calculate total width needed
    pub fn width(&self) -> u16 {
        match self.style {
            StatusStyle::Dot => self.size.width(),
            StatusStyle::DotWithLabel => {
                let label_len = crate::utils::display_width(self.get_label()) as u16;
                self.size.width() + 1 + label_len // dot + space + label
            }
            StatusStyle::LabelOnly => crate::utils::display_width(self.get_label()) as u16,
            StatusStyle::Badge => {
                let label_len = crate::utils::display_width(self.get_label()) as u16;
                label_len + 4 // padding + dot + space + label + padding
            }
        }
    }
}

impl Default for StatusIndicator {
    fn default() -> Self {
        Self::new(Status::Online)
    }
}

impl_styled_view!(StatusIndicator);
impl_widget_builders!(StatusIndicator);

/// Helper function to create a StatusIndicator
pub fn status_indicator(status: Status) -> StatusIndicator {
    StatusIndicator::new(status)
}

/// Helper function to create an online indicator
pub fn online() -> StatusIndicator {
    StatusIndicator::online()
}

/// Helper function to create an offline indicator
pub fn offline() -> StatusIndicator {
    StatusIndicator::offline()
}

/// Helper function to create a busy indicator
pub fn busy_indicator() -> StatusIndicator {
    StatusIndicator::busy()
}

/// Helper function to create an away indicator
pub fn away_indicator() -> StatusIndicator {
    StatusIndicator::away()
}
