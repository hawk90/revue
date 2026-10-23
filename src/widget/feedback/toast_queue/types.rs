//! Stack direction, toast priority and queue entries

use crate::widget::feedback::toast::ToastLevel;
use std::time::{Duration, Instant};

/// Stack direction for toasts
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackDirection {
    /// New toasts appear below existing ones
    #[default]
    Down,
    /// New toasts appear above existing ones
    Up,
}

/// Priority level for toasts (higher = more important)
///
/// Priority only orders the pending queue: a toast waits behind the
/// pending ones of its priority or higher and goes ahead of the lower
/// ones, then shows when `max_visible` leaves a slot. It does not push out
/// a visible toast, and it does not change what can be dismissed: that is
/// [`ToastEntry::dismissible`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ToastPriority {
    /// Low priority: waits behind every other pending toast
    Low = 0,
    /// Normal priority
    #[default]
    Normal = 1,
    /// High priority: waits only behind pending High and Critical toasts
    High = 2,
    /// Critical priority: waits only behind pending Critical toasts
    Critical = 3,
}

/// A toast entry in the queue
#[derive(Clone, Debug)]
pub struct ToastEntry {
    /// Unique ID for deduplication
    pub id: Option<String>,
    /// Toast message
    pub message: String,
    /// Toast level
    pub level: ToastLevel,
    /// Priority
    pub priority: ToastPriority,
    /// Duration to show (None = use default)
    pub duration: Option<Duration>,
    /// Time when toast was created
    pub created_at: Instant,
    /// Time when toast was shown
    pub shown_at: Option<Instant>,
    /// Whether toast is dismissible
    pub dismissible: bool,
}

impl ToastEntry {
    /// Create a new toast entry
    pub fn new(message: impl Into<String>, level: ToastLevel) -> Self {
        Self {
            id: None,
            message: message.into(),
            level,
            priority: ToastPriority::Normal,
            duration: None,
            created_at: Instant::now(),
            shown_at: None,
            dismissible: true,
        }
    }

    /// Set an ID for deduplication
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the priority
    pub fn with_priority(mut self, priority: ToastPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Set custom duration
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set whether dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }
}
