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

mod render;

use super::super::border::BorderType;
use crate::event::Key;
use crate::style::Color;
use crate::widget::traits::{View, WidgetProps, WidgetState};
use crate::{impl_styled_view, impl_widget_builders};

use super::types::CardVariant;

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

    /// Set the card title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the card subtitle
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Set custom header content
    pub fn header(mut self, header: impl View + 'static) -> Self {
        self.header = Some(Box::new(header));
        self
    }

    /// Set the body content
    pub fn body(mut self, body: impl View + 'static) -> Self {
        self.body = Some(Box::new(body));
        self
    }

    /// Set the footer content
    pub fn footer(mut self, footer: impl View + 'static) -> Self {
        self.footer = Some(Box::new(footer));
        self
    }

    /// Set the visual variant
    ///
    /// [`CardVariant::Flat`] has no border, so choosing it clears the border
    /// just as [`Card::flat`] does; a `border_style` set afterwards still wins.
    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        if variant == CardVariant::Flat {
            self.border = BorderType::None;
        }
        self
    }

    /// Set the border style
    pub fn border_style(mut self, border: BorderType) -> Self {
        self.border = border;
        self
    }

    /// Use outlined variant
    pub fn outlined(mut self) -> Self {
        self.variant = CardVariant::Outlined;
        self
    }

    /// Use filled variant
    pub fn filled(mut self) -> Self {
        self.variant = CardVariant::Filled;
        self
    }

    /// Use elevated variant
    pub fn elevated(mut self) -> Self {
        self.variant = CardVariant::Elevated;
        self
    }

    /// Use flat variant (no border)
    pub fn flat(mut self) -> Self {
        self.variant = CardVariant::Flat;
        self.border = BorderType::None;
        self
    }

    /// Use rounded border
    pub fn rounded(mut self) -> Self {
        self.border = BorderType::Rounded;
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set border/accent color
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Set title color
    pub fn title_color(mut self, color: Color) -> Self {
        self.title_color = Some(color);
        self
    }

    /// Make the card collapsible
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Set the expanded state (for collapsible cards)
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Make the card clickable
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// Set padding inside the card
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Set minimum width constraint
    pub fn min_width(mut self, width: u16) -> Self {
        self.min_width = width;
        self
    }

    /// Set minimum height constraint
    pub fn min_height(mut self, height: u16) -> Self {
        self.min_height = height;
        self
    }

    /// Set maximum width constraint (0 = no limit)
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Set maximum height constraint (0 = no limit)
    pub fn max_height(mut self, height: u16) -> Self {
        self.max_height = height;
        self
    }

    /// Set both min width and height
    pub fn min_size(self, width: u16, height: u16) -> Self {
        self.min_width(width).min_height(height)
    }

    /// Set both max width and height (0 = no limit)
    pub fn max_size(self, width: u16, height: u16) -> Self {
        self.max_width(width).max_height(height)
    }

    /// Set all size constraints at once
    pub fn constrain(self, min_w: u16, min_h: u16, max_w: u16, max_h: u16) -> Self {
        self.min_width(min_w)
            .min_height(min_h)
            .max_width(max_w)
            .max_height(max_h)
    }

    /// Toggle expanded state
    pub fn toggle(&mut self) {
        if self.collapsible {
            self.expanded = !self.expanded;
        }
    }

    /// Expand the card
    pub fn expand(&mut self) {
        self.expanded = true;
    }

    /// Collapse the card
    pub fn collapse(&mut self) {
        self.expanded = false;
    }

    /// Check if expanded
    pub fn is_expanded(&self) -> bool {
        self.expanded
    }

    /// Check if collapsible
    pub fn is_collapsible(&self) -> bool {
        self.collapsible
    }

    /// Handle keyboard input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if !self.collapsible || self.state.disabled {
            return false;
        }

        match key {
            Key::Enter | Key::Char(' ') => {
                self.toggle();
                true
            }
            Key::Right | Key::Char('l') => {
                self.expand();
                true
            }
            Key::Left | Key::Char('h') => {
                self.collapse();
                true
            }
            _ => false,
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
