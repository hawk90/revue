//! Notification Center for managing toasts and alerts
//!
//! Provides a centralized system for displaying notifications,
//! alerts, and status messages with queuing and auto-dismiss.

mod core;
mod types;

pub use core::{notification_center, NotificationCenter};
pub use types::*;
