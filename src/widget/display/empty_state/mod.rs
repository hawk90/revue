//! Empty State widget for displaying no-data scenarios gracefully
//!
//! A dedicated widget for consistent, helpful empty state displays.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{EmptyState, EmptyStateType, empty_state};
//!
//! // Basic empty state
//! EmptyState::new("No items yet")
//!     .description("Create your first item to get started");
//!
//! // Search with no results
//! empty_state("No results found")
//!     .state_type(EmptyStateType::NoResults)
//!     .description("Try adjusting your search terms")
//!     .action("Clear search");
//!
//! // Error state
//! EmptyState::error("Failed to load data")
//!     .description("Check your connection and try again")
//!     .action("Retry");
//! ```

mod render;
mod types;

pub use types::{EmptyStateType, EmptyStateVariant};

use crate::widget::traits::{WidgetProps, WidgetState};
use crate::{impl_styled_view, impl_widget_builders};

/// An empty state widget for no-data scenarios
///
/// Displays a consistent, helpful message when there's no content to show.
#[derive(Clone)]
pub struct EmptyState {
    /// Primary message/title
    title: String,
    /// Optional description text
    description: Option<String>,
    /// State type
    state_type: EmptyStateType,
    /// Visual variant
    variant: EmptyStateVariant,
    /// Show icon
    show_icon: bool,
    /// Custom icon override
    custom_icon: Option<char>,
    /// Optional action button text
    action: Option<String>,
    /// Widget state
    state: WidgetState,
    /// Widget properties
    props: WidgetProps,
}

impl EmptyState {
    /// Create a new empty state with a title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            state_type: EmptyStateType::default(),
            variant: EmptyStateVariant::default(),
            show_icon: true,
            custom_icon: None,
            action: None,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Create an empty state for no results
    pub fn no_results(title: impl Into<String>) -> Self {
        Self::new(title).state_type(EmptyStateType::NoResults)
    }

    /// Create an error empty state
    pub fn error(title: impl Into<String>) -> Self {
        Self::new(title).state_type(EmptyStateType::Error)
    }

    /// Create a no permission empty state
    pub fn no_permission(title: impl Into<String>) -> Self {
        Self::new(title).state_type(EmptyStateType::NoPermission)
    }

    /// Create an offline empty state
    pub fn offline(title: impl Into<String>) -> Self {
        Self::new(title).state_type(EmptyStateType::Offline)
    }

    /// Create a first-use empty state
    pub fn first_use(title: impl Into<String>) -> Self {
        Self::new(title).state_type(EmptyStateType::FirstUse)
    }

    /// Set the description text
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the state type
    pub fn state_type(mut self, state_type: EmptyStateType) -> Self {
        self.state_type = state_type;
        self
    }

    /// Set the visual variant
    pub fn variant(mut self, variant: EmptyStateVariant) -> Self {
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

    /// Set an action button text
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Calculate the height needed for this empty state
    pub fn height(&self) -> u16 {
        let has_desc = self.description.is_some();
        let has_action = self.action.is_some();
        match self.variant {
            EmptyStateVariant::Full => {
                // Mirrors `render_full`: icon and a blank row, the title, the
                // description and a blank row before the action, the action.
                let mut h = 1; // title
                if self.show_icon {
                    h += 2;
                }
                if has_desc {
                    h += 1;
                }
                if has_action {
                    h += if has_desc { 2 } else { 1 };
                }
                h
            }
            EmptyStateVariant::Compact => {
                // Icon and title share the first row; then description, action.
                1 + u16::from(has_desc) + u16::from(has_action)
            }
            EmptyStateVariant::Minimal => 1,
        }
    }
}

impl Default for EmptyState {
    fn default() -> Self {
        Self::new("No items")
    }
}

impl_styled_view!(EmptyState);
impl_widget_builders!(EmptyState);

/// Helper function to create an EmptyState
pub fn empty_state(title: impl Into<String>) -> EmptyState {
    EmptyState::new(title)
}

/// Helper function to create a no-results EmptyState
pub fn no_results(title: impl Into<String>) -> EmptyState {
    EmptyState::no_results(title)
}

/// Helper function to create an error EmptyState
pub fn empty_error(title: impl Into<String>) -> EmptyState {
    EmptyState::error(title)
}

/// Helper function to create a first-use EmptyState
pub fn first_use(title: impl Into<String>) -> EmptyState {
    EmptyState::first_use(title)
}
