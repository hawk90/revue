//! Card widget for grouping related content with visual boundaries
//!
//! Cards provide a structured container with optional header, body, and footer sections.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Card, card};
//!
//! // Basic card with title
//! Card::new()
//!     .title("User Profile")
//!     .body(user_info_widget);
//!
//! // Card with header, body, and footer
//! card()
//!     .title("Settings")
//!     .subtitle("Configure your preferences")
//!     .body(settings_form)
//!     .footer(action_buttons);
//!
//! // Collapsible card
//! Card::new()
//!     .title("Details")
//!     .collapsible(true)
//!     .body(details_content);
//! ```

mod builders;
mod helper;
mod input;
mod style;
mod types;
mod view;

use super::border::BorderType;
use crate::style::Color;
use crate::widget::traits::{View, WidgetProps, WidgetState};
use crate::{impl_styled_view, impl_widget_builders};

// Re-exports
pub use helper::card;
pub use types::CardVariant;

/// A card widget for grouping content with structure
pub struct Card {
    /// Card title
    title: Option<String>,
    /// Card subtitle
    subtitle: Option<String>,
    /// Header content (rendered below title)
    header: Option<Box<dyn View>>,
    /// Main body content
    body: Option<Box<dyn View>>,
    /// Footer content
    footer: Option<Box<dyn View>>,
    /// Visual variant
    variant: CardVariant,
    /// Border style
    border: BorderType,
    /// Background color
    bg_color: Option<Color>,
    /// Border/accent color
    border_color: Option<Color>,
    /// Title color
    title_color: Option<Color>,
    /// Whether the card is collapsible
    collapsible: bool,
    /// Whether the card is expanded (when collapsible)
    expanded: bool,
    /// Whether the card is clickable
    clickable: bool,
    /// Padding inside the card
    padding: u16,
    /// Widget state
    state: WidgetState,
    /// Minimum width constraint (0 = no constraint)
    min_width: u16,
    /// Minimum height constraint (0 = no constraint)
    min_height: u16,
    /// Maximum width constraint (0 = no constraint)
    max_width: u16,
    /// Maximum height constraint (0 = no constraint)
    max_height: u16,
    /// Widget properties
    props: WidgetProps,
}

impl Card {
    /// Create a new card
    pub fn new() -> Self {
        Self {
            title: None,
            subtitle: None,
            header: None,
            body: None,
            footer: None,
            variant: CardVariant::default(),
            border: BorderType::default(),
            bg_color: None,
            border_color: None,
            title_color: None,
            collapsible: false,
            expanded: true,
            clickable: false,
            padding: 1,
            state: WidgetState::new(),
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,
            props: WidgetProps::new(),
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(Card);
impl_widget_builders!(Card);
