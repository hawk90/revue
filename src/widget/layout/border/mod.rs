//! Border/Frame widget for surrounding content

mod border_type;
mod draw;
mod view;

#[cfg(test)]
mod tests;

pub use border_type::BorderType;
pub use draw::draw_border;

use crate::style::Color;
use crate::widget::traits::{View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// A border widget that wraps content
pub struct Border {
    child: Option<Box<dyn View>>,
    border_type: BorderType,
    title: Option<String>,
    fg: Option<Color>,
    bg: Option<Color>,
    /// Minimum width constraint (0 = no constraint)
    min_width: u16,
    /// Minimum height constraint (0 = no constraint)
    min_height: u16,
    /// Maximum width constraint (0 = no constraint)
    max_width: u16,
    /// Maximum height constraint (0 = no constraint)
    max_height: u16,
    props: WidgetProps,
}

impl Border {
    /// Create a new border
    pub fn new() -> Self {
        Self {
            child: None,
            border_type: BorderType::Single,
            title: None,
            fg: None,
            bg: None,
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,
            props: WidgetProps::new(),
        }
    }

    /// Set the child widget
    pub fn child(mut self, child: impl View + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    /// Set border type
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    /// Set title (displayed in top border)
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set border color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
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

    // ─────────────────────────────────────────────────────────────────────────
    // Getter methods (for testing)
    // ─────────────────────────────────────────────────────────────────────────

    #[doc(hidden)]
    pub fn get_border_type(&self) -> BorderType {
        self.border_type
    }

    #[doc(hidden)]
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    #[doc(hidden)]
    pub fn get_fg(&self) -> Option<Color> {
        self.fg
    }

    #[doc(hidden)]
    pub fn get_bg(&self) -> Option<Color> {
        self.bg
    }

    #[doc(hidden)]
    pub fn has_child(&self) -> bool {
        self.child.is_some()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Preset builders
    // ─────────────────────────────────────────────────────────────────────────

    /// Create single border
    pub fn single() -> Self {
        Self::new().border_type(BorderType::Single)
    }

    /// Create double border
    pub fn double() -> Self {
        Self::new().border_type(BorderType::Double)
    }

    /// Create rounded border
    pub fn rounded() -> Self {
        Self::new().border_type(BorderType::Rounded)
    }

    /// Create thick border
    pub fn thick() -> Self {
        Self::new().border_type(BorderType::Thick)
    }

    /// Create ASCII border (for basic terminals)
    pub fn ascii() -> Self {
        Self::new().border_type(BorderType::Ascii)
    }

    /// Create a panel (double border with cyan color)
    pub fn panel() -> Self {
        Self::new().border_type(BorderType::Double).fg(Color::CYAN)
    }

    /// Create a card (rounded border with white color)
    pub fn card() -> Self {
        Self::new()
            .border_type(BorderType::Rounded)
            .fg(Color::WHITE)
    }

    /// Create an error box (single border with red color)
    pub fn error_box() -> Self {
        Self::new().border_type(BorderType::Single).fg(Color::RED)
    }

    /// Create a success box (single border with green color)
    pub fn success_box() -> Self {
        Self::new().border_type(BorderType::Single).fg(Color::GREEN)
    }
}

impl Default for Border {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a border
pub fn border() -> Border {
    Border::new()
}

impl_styled_view!(Border);
impl_props_builders!(Border);
