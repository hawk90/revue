//! Toast queue manager for centralized toast notifications
//!
//! Manages a queue of toasts with deduplication, positioning, and stacking control.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{ToastQueue, ToastEntry, ToastLevel, QueuePosition};
//!
//! // Create a toast queue
//! let mut queue = ToastQueue::new()
//!     .position(QueuePosition::TopRight)
//!     .max_visible(3)
//!     .stack_direction(StackDirection::Down);
//!
//! // Add toasts
//! queue.push("File saved", ToastLevel::Success);
//! queue.push_with_id("error-1", "Connection failed", ToastLevel::Error);
//!
//! // In tick handler
//! queue.tick();
//! ```

mod queue;
mod render;
mod types;

pub use types::{StackDirection, ToastEntry, ToastPriority};

use super::toast::ToastPosition;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};
use std::time::Duration;

/// Centralized toast queue manager
#[derive(Clone)]
pub struct ToastQueue {
    /// Queue of pending toasts
    queue: Vec<ToastEntry>,
    /// Currently visible toasts
    visible: Vec<ToastEntry>,
    /// Queue position
    position: ToastPosition,
    /// Stack direction
    stack_direction: StackDirection,
    /// Maximum visible toasts
    max_visible: usize,
    /// Default duration for toasts
    default_duration: Duration,
    /// Gap between toasts
    gap: u16,
    /// Toast width
    toast_width: u16,
    /// Enable deduplication
    deduplicate: bool,
    /// Pause on hover
    pause_on_hover: bool,
    /// Whether toast timers are currently paused
    paused: bool,
    /// Widget properties
    props: WidgetProps,
}

impl ToastQueue {
    /// Create a new toast queue
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            visible: Vec::new(),
            position: ToastPosition::TopRight,
            stack_direction: StackDirection::Down,
            max_visible: 5,
            default_duration: Duration::from_secs(4),
            gap: 1,
            toast_width: 40,
            deduplicate: true,
            pause_on_hover: false,
            paused: false,
            props: WidgetProps::new(),
        }
    }

    /// Set the position
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    /// Set stack direction
    pub fn stack_direction(mut self, direction: StackDirection) -> Self {
        self.stack_direction = direction;
        self
    }

    /// Set maximum visible toasts
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }

    /// Set default duration
    pub fn default_duration(mut self, duration: Duration) -> Self {
        self.default_duration = duration;
        self
    }

    /// Set gap between toasts
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Set toast width
    pub fn toast_width(mut self, width: u16) -> Self {
        self.toast_width = width;
        self
    }

    /// Enable/disable deduplication
    pub fn deduplicate(mut self, deduplicate: bool) -> Self {
        self.deduplicate = deduplicate;
        self
    }

    /// Enable/disable pause on hover
    pub fn pause_on_hover(mut self, pause: bool) -> Self {
        self.pause_on_hover = pause;
        self
    }
}

impl Default for ToastQueue {
    fn default() -> Self {
        Self::new()
    }
}

// Getters for testing
impl ToastQueue {
    #[doc(hidden)]
    pub fn get_queue(&self) -> &[ToastEntry] {
        &self.queue
    }

    #[doc(hidden)]
    pub fn get_visible(&self) -> &[ToastEntry] {
        &self.visible
    }

    #[doc(hidden)]
    pub fn get_position(&self) -> ToastPosition {
        self.position
    }

    #[doc(hidden)]
    pub fn get_stack_direction(&self) -> StackDirection {
        self.stack_direction
    }

    #[doc(hidden)]
    pub fn get_max_visible(&self) -> usize {
        self.max_visible
    }

    #[doc(hidden)]
    pub fn get_default_duration(&self) -> Duration {
        self.default_duration
    }

    #[doc(hidden)]
    pub fn get_gap(&self) -> u16 {
        self.gap
    }

    #[doc(hidden)]
    pub fn get_toast_width(&self) -> u16 {
        self.toast_width
    }

    #[doc(hidden)]
    pub fn get_deduplicate(&self) -> bool {
        self.deduplicate
    }
}

impl_styled_view!(ToastQueue);
impl_props_builders!(ToastQueue);

/// Create a new toast queue
pub fn toast_queue() -> ToastQueue {
    ToastQueue::new()
}
