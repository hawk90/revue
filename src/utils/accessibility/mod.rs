//! Accessibility utilities for TUI applications
//!
//! Provides accessibility support including:
//! - ARIA-like roles and labels
//! - Screen reader announcements
//! - Focus tracking
//! - Keyboard navigation helpers
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::utils::accessibility::{AccessibleNode, Role, announce};
//!
//! let node = AccessibleNode::new(Role::Button)
//!     .label("Submit")
//!     .description("Click to submit the form")
//!     .shortcut("Enter");
//!
//! // Announce to screen readers
//! announce("Form submitted successfully");
//! ```

mod announcement;
mod manager;
mod node;
mod roles;
mod state;

pub mod aria;

// Re-export ARIA types
pub use aria::{aria_attributes, aria_pairs, AriaAttribute, AriaBuilder, LiveRegion};

// Re-export main types
pub use announcement::{Announcement, Priority};
pub use manager::{
    accessibility_manager, announce, announce_now, shared_accessibility, AccessibilityManager,
    SharedAccessibility,
};
pub use node::AccessibleNode;
pub use roles::Role;
pub use state::AccessibleState;

use std::sync::atomic::{AtomicUsize, Ordering};

// ID counter for unique IDs
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Generate unique ID (pub for testing)
pub(crate) fn generate_id() -> String {
    let id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("a11y-{}", id)
}

#[cfg(test)]
mod tests;
