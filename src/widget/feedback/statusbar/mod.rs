//! Status bar widget for header/footer displays
//!
//! Provides configurable status bars with sections for displaying
//! application state, key hints, and other information.

mod render;
mod types;

#[allow(deprecated)]
pub use types::SectionAlign;
pub use types::{KeyHint, StatusBarPosition, StatusSection};

use crate::style::Color;
use crate::widget::theme::{DARK_BG, SECONDARY_TEXT};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Status bar widget
#[derive(Clone)]
pub struct StatusBar {
    /// Left-aligned sections
    left: Vec<StatusSection>,
    /// Center-aligned sections
    center: Vec<StatusSection>,
    /// Right-aligned sections
    right: Vec<StatusSection>,
    /// Position
    position: StatusBarPosition,
    /// Background color
    bg: Color,
    /// Default foreground color
    fg: Option<Color>,
    /// Key hints
    key_hints: Vec<KeyHint>,
    /// Key hint foreground
    key_fg: Color,
    /// Key hint background
    key_bg: Color,
    /// Separator between sections
    separator: Option<char>,
    /// Height (usually 1)
    height: u16,
    /// Widget props for CSS integration
    props: WidgetProps,
}

impl StatusBar {
    /// Create a new status bar
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            center: Vec::new(),
            right: Vec::new(),
            position: StatusBarPosition::Bottom,
            bg: DARK_BG,
            fg: None,
            key_hints: Vec::new(),
            key_fg: Color::BLACK,
            key_bg: SECONDARY_TEXT,
            separator: None,
            height: 1,
            props: WidgetProps::new(),
        }
    }

    /// Set position
    pub fn position(mut self, position: StatusBarPosition) -> Self {
        self.position = position;
        self
    }

    /// Set as header (top position)
    pub fn header(mut self) -> Self {
        self.position = StatusBarPosition::Top;
        self
    }

    /// Set as footer (bottom position)
    pub fn footer(mut self) -> Self {
        self.position = StatusBarPosition::Bottom;
        self
    }

    /// Add left section
    pub fn left(mut self, section: StatusSection) -> Self {
        self.left.push(section);
        self
    }

    /// Add center section
    pub fn center(mut self, section: StatusSection) -> Self {
        self.center.push(section);
        self
    }

    /// Add right section
    pub fn right(mut self, section: StatusSection) -> Self {
        self.right.push(section);
        self
    }

    /// Add left text
    pub fn left_text(self, text: impl Into<String>) -> Self {
        self.left(StatusSection::new(text))
    }

    /// Add center text
    pub fn center_text(self, text: impl Into<String>) -> Self {
        self.center(StatusSection::new(text))
    }

    /// Add right text
    pub fn right_text(self, text: impl Into<String>) -> Self {
        self.right(StatusSection::new(text))
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Add key hint
    pub fn key(mut self, key: impl Into<String>, description: impl Into<String>) -> Self {
        self.key_hints.push(KeyHint::new(key, description));
        self
    }

    /// Add multiple key hints
    pub fn keys(mut self, hints: Vec<KeyHint>) -> Self {
        self.key_hints.extend(hints);
        self
    }

    /// Set separator character, drawn between neighboring sections of the
    /// left, center and right groups
    pub fn separator(mut self, sep: char) -> Self {
        self.separator = Some(sep);
        self
    }

    /// Set height
    pub fn height(mut self, height: u16) -> Self {
        self.height = height.max(1);
        self
    }

    /// Update a left section by index
    pub fn update_left(&mut self, index: usize, content: impl Into<String>) {
        if let Some(section) = self.left.get_mut(index) {
            section.content = content.into();
        }
    }

    /// Update a center section by index
    pub fn update_center(&mut self, index: usize, content: impl Into<String>) {
        if let Some(section) = self.center.get_mut(index) {
            section.content = content.into();
        }
    }

    /// Update a right section by index
    pub fn update_right(&mut self, index: usize, content: impl Into<String>) {
        if let Some(section) = self.right.get_mut(index) {
            section.content = content.into();
        }
    }

    /// Clear all sections
    pub fn clear(&mut self) {
        self.left.clear();
        self.center.clear();
        self.right.clear();
        self.key_hints.clear();
    }

    /// Get render Y position
    fn render_y(&self, area_height: u16) -> u16 {
        match self.position {
            StatusBarPosition::Top => 0,
            StatusBarPosition::Bottom => area_height.saturating_sub(self.height),
        }
    }

    // Getters for testing
    #[doc(hidden)]
    pub fn get_left(&self) -> &[StatusSection] {
        &self.left
    }

    #[doc(hidden)]
    pub fn get_center(&self) -> &[StatusSection] {
        &self.center
    }

    #[doc(hidden)]
    pub fn get_right(&self) -> &[StatusSection] {
        &self.right
    }

    #[doc(hidden)]
    pub fn get_position(&self) -> StatusBarPosition {
        self.position
    }

    #[doc(hidden)]
    pub fn get_bg(&self) -> Color {
        self.bg
    }

    #[doc(hidden)]
    pub fn get_fg(&self) -> Option<Color> {
        self.fg
    }

    #[doc(hidden)]
    pub fn get_key_hints(&self) -> &[KeyHint] {
        &self.key_hints
    }

    #[doc(hidden)]
    pub fn get_separator(&self) -> Option<char> {
        self.separator
    }

    #[doc(hidden)]
    pub fn get_height(&self) -> u16 {
        self.height
    }

    #[doc(hidden)]
    pub fn get_render_y(&self, area_height: u16) -> u16 {
        self.render_y(area_height)
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(StatusBar);
impl_props_builders!(StatusBar);

// Helper functions

/// Create a new status bar
pub fn statusbar() -> StatusBar {
    StatusBar::new()
}

/// Create a header status bar (positioned at top)
pub fn header() -> StatusBar {
    StatusBar::new().header()
}

/// Create a footer status bar (positioned at bottom)
pub fn footer() -> StatusBar {
    StatusBar::new().footer()
}

/// Create a status bar section with content
pub fn section(content: impl Into<String>) -> StatusSection {
    StatusSection::new(content)
}

/// Create a key hint with key and description
pub fn key_hint(key: impl Into<String>, description: impl Into<String>) -> KeyHint {
    KeyHint::new(key, description)
}
