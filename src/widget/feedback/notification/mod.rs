//! Notification Center for managing toasts and alerts
//!
//! Provides a centralized system for displaying notifications,
//! alerts, and status messages with queuing and auto-dismiss.
//!
//! The center's files are child modules of this one, so they see its
//! private fields: `builders` (settings), `queue` (adding, dismissing and
//! expiring notifications), `input` (selection and keys), `view` (where each
//! notification goes) and `draw` (one notification, row by row).

mod builders;
mod draw;
mod input;
mod queue;
mod types;
mod view;

pub use types::*;

use crate::utils::KeyedSelection;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Notification Center widget
#[derive(Clone)]
pub struct NotificationCenter {
    /// Active notifications
    notifications: Vec<Notification>,
    /// Maximum visible notifications
    max_visible: usize,
    /// Position on screen
    position: NotificationPosition,
    /// Notification width
    width: u16,
    /// Show icons
    show_icons: bool,
    /// Show progress timer
    show_timer: bool,
    /// Spacing between notifications
    spacing: u16,
    /// Current tick counter
    tick_counter: u64,
    /// Selected notification (for dismissal), by id, among the shown ones:
    /// the newest `max_visible`, newest first
    selected: KeyedSelection<u64>,
    /// Focused state
    focused: bool,
    /// The notifications the last render drew, newest first
    drawn: DrawnIds,
    /// Widget properties
    props: WidgetProps,
}

/// The ids of the boxes the last render drew, newest first; `None` before
/// the first render. Behind a lock so `render(&self)` can record them and
/// the center stays `Send + Sync`; a clone gets its own copy.
#[derive(Default)]
struct DrawnIds(std::sync::Mutex<Option<Vec<u64>>>);

impl DrawnIds {
    fn set(&self, ids: Vec<u64>) {
        *self.0.lock().unwrap_or_else(|e| e.into_inner()) = Some(ids);
    }

    fn get(&self) -> Option<Vec<u64>> {
        self.0.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }
}

impl Clone for DrawnIds {
    fn clone(&self) -> Self {
        Self(std::sync::Mutex::new(self.get()))
    }
}

impl NotificationCenter {
    /// Create a new notification center
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
            max_visible: 5,
            position: NotificationPosition::TopRight,
            width: 40,
            show_icons: true,
            show_timer: true,
            spacing: 1,
            tick_counter: 0,
            selected: KeyedSelection::new(),
            focused: false,
            drawn: DrawnIds::default(),
            props: WidgetProps::new(),
        }
    }
}

impl Default for NotificationCenter {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(NotificationCenter);
impl_props_builders!(NotificationCenter);

/// Helper to create a notification center
pub fn notification_center() -> NotificationCenter {
    NotificationCenter::new()
}
