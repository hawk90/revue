//! Feedback widgets - User feedback and notification components
//!
//! This module provides widgets for providing feedback to users through
//! modals, notifications, alerts, tooltips, and other feedback mechanisms.
//!
//! # Widget Categories
//!
//! ## Modal & Dialog
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Modal`] | Modal dialog overlay | [`modal()`] |
//!
//! ## Notifications
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Toast`] | Temporary notification popup | [`toast()`] |
//! | [`ToastQueue`] | Multiple toasts with queue | [`toast_queue()`] |
//! | [`NotificationCenter`] | Notification center | [`notification_center()`] |
//!
//! ## Alerts & Messages
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Alert`] | Alert box with levels | [`alert()`], [`error_alert()`] |
//!
//! ## Contextual Help
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Tooltip`] | Hover tooltip | [`tooltip()`] |
//! | [`Popover`] | Anchor-positioned overlay | [`popover()`] |
//! | [`ContextMenu`] | Right-click menu | [`context_menu()`] |
//!
//! ## Status
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`StatusBar`] | Application status bar | [`statusbar()`], [`header()`], [`footer()`] |
//! | [`KeyHint`] | Keyboard shortcut hint | [`key_hint()`] |
//!
//! # Quick Start
//!
//! ## Toast Notification
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! toast()
//!     .message("Operation completed successfully!")
//!     .level(ToastLevel::Success)
//!     .duration(Duration::from_secs(3));
//! ```
//!
//! ## Modal Dialog
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! modal()
//!     .title("Confirm Action")
//!     .content("Are you sure you want to proceed?")
//!     .button("Yes", || println!("Confirmed"))
//!     .button("No", || println!("Cancelled"));
//! ```
//!
//! ## Alert
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! alert()
//!     .level(AlertLevel::Warning)
//!     .title("Warning")
//!     .content("This action cannot be undone");
//! ```
//!
//! ## Tooltip
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! tooltip()
//!     .content("Hover for more information")
//!     .position(TooltipPosition::Top);
//! ```
//!
//! ## Status Bar
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! statusbar()
//!     .section("Mode", "Normal")
//!     .section("File", "main.rs")
//!     .key_hint("^Q", "Quit");
//! ```

pub mod alert;
pub mod menu;
pub mod modal;
pub mod notification;
pub mod popover;
pub mod statusbar;
pub mod toast;
pub mod toast_queue;
pub mod tooltip;

// Re-exports for convenience
pub use alert::{
    alert, error_alert, info_alert, success_alert, warning_alert, Alert, AlertLevel, AlertVariant,
};
pub use menu::{context_menu, menu, menu_bar, menu_item, ContextMenu, Menu, MenuBar, MenuItem};
pub use modal::{modal, Modal, ModalButton, ModalButtonStyle};
pub use notification::{
    notification_center, Notification, NotificationCenter, NotificationLevel, NotificationPosition,
};
pub use popover::{popover, Popover, PopoverArrow, PopoverPosition, PopoverStyle, PopoverTrigger};
pub use statusbar::{
    footer, header, key_hint, section as status_section, statusbar, KeyHint, SectionAlign,
    StatusBar, StatusSection,
};
pub use toast::{toast, Toast, ToastLevel, ToastPosition};
pub use toast_queue::{toast_queue, StackDirection, ToastEntry, ToastPriority, ToastQueue};
pub use tooltip::{tooltip, Tooltip, TooltipArrow, TooltipPosition, TooltipStyle};
