//! Feedback widgets - User feedback and notification components
//!
//! Widgets for providing feedback to users through various modal and notification mechanisms.

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
