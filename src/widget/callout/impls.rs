//! Callout implementations

use super::core::Callout;
use super::types::{CalloutType, CalloutVariant};
use crate::event::Key;
use crate::widget::traits::WidgetState;

impl Callout {
    /// Create a new callout with content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            title: None,
            callout_type: CalloutType::default(),
            variant: CalloutVariant::default(),
            show_icon: true,
            custom_icon: None,
            collapsible: false,
            expanded: true,
            collapsed_icon: '▶',
            expanded_icon: '▼',
            state: WidgetState::new(),
            props: crate::widget::traits::WidgetProps::new(),
        }
    }

    /// Create a note callout
    pub fn note(content: impl Into<String>) -> Self {
        Self::new(content).callout_type(CalloutType::Note)
    }

    /// Create a tip callout
    pub fn tip(content: impl Into<String>) -> Self {
        Self::new(content).callout_type(CalloutType::Tip)
    }

    /// Create an important callout
    pub fn important(content: impl Into<String>) -> Self {
        Self::new(content).callout_type(CalloutType::Important)
    }

    /// Create a warning callout
    pub fn warning(content: impl Into<String>) -> Self {
        Self::new(content).callout_type(CalloutType::Warning)
    }

    /// Create a danger callout
    pub fn danger(content: impl Into<String>) -> Self {
        Self::new(content).callout_type(CalloutType::Danger)
    }

    /// Create an info callout
    pub fn info(content: impl Into<String>) -> Self {
        Self::new(content).callout_type(CalloutType::Info)
    }

    /// Set the callout type
    pub fn callout_type(mut self, callout_type: CalloutType) -> Self {
        self.callout_type = callout_type;
        self
    }

    /// Set the title (overrides default type title)
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the visual variant
    pub fn variant(mut self, variant: CalloutVariant) -> Self {
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

    /// Make the callout collapsible
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Set the expanded state (for collapsible callouts)
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set custom collapse/expand icons
    pub fn collapse_icons(mut self, collapsed: char, expanded: char) -> Self {
        self.collapsed_icon = collapsed;
        self.expanded_icon = expanded;
        self
    }

    /// Toggle expanded state
    pub fn toggle(&mut self) {
        if self.collapsible {
            self.expanded = !self.expanded;
        }
    }

    /// Expand the callout
    pub fn expand(&mut self) {
        self.expanded = true;
    }

    /// Collapse the callout
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

    /// Set expanded state mutably
    pub fn set_expanded(&mut self, expanded: bool) {
        self.expanded = expanded;
    }

    /// Get the icon to display
    pub fn get_icon(&self) -> char {
        self.custom_icon.unwrap_or_else(|| self.callout_type.icon())
    }

    /// Get the collapse icon based on state
    pub fn collapse_icon(&self) -> char {
        if self.expanded {
            self.expanded_icon
        } else {
            self.collapsed_icon
        }
    }

    /// Get the title to display
    pub fn get_title(&self) -> &str {
        self.title
            .as_deref()
            .unwrap_or_else(|| self.callout_type.default_title())
    }

    /// Calculate the height needed for this callout
    pub fn height(&self) -> u16 {
        if self.collapsible && !self.expanded {
            return 1; // Just header
        }

        let content_lines = self.content.lines().count().max(1) as u16;

        match self.variant {
            CalloutVariant::Filled => {
                // top border + title + content + bottom border
                2 + content_lines + 1
            }
            CalloutVariant::LeftBorder => {
                // title + content
                1 + content_lines
            }
            CalloutVariant::Minimal => {
                // title + content (no borders)
                1 + content_lines
            }
        }
    }

    /// Handle keyboard input
    ///
    /// Returns `true` if the key was handled.
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

impl Default for Callout {
    fn default() -> Self {
        Self::new("Callout")
    }
}
