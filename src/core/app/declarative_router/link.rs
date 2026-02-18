//! Link widget for declarative router navigation

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// A link widget for router navigation
pub struct Link {
    /// Target path
    path: String,
    /// Display label
    label: String,
    /// Whether this link is currently active
    active: bool,
    /// Foreground color
    fg: Option<Color>,
    /// Active foreground color
    active_fg: Option<Color>,
    /// Active background color
    active_bg: Option<Color>,
    /// Underline text
    underline: bool,
    props: WidgetProps,
}

impl Link {
    /// Create a new link
    pub fn new(path: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            label: label.into(),
            active: false,
            fg: Some(Color::CYAN),
            active_fg: Some(Color::WHITE),
            active_bg: Some(Color::BLUE),
            underline: true,
            props: WidgetProps::new(),
        }
    }

    /// Set whether this link is active
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set active foreground color
    pub fn active_fg(mut self, color: Color) -> Self {
        self.active_fg = Some(color);
        self
    }

    /// Set active background color
    pub fn active_bg(mut self, color: Color) -> Self {
        self.active_bg = Some(color);
        self
    }

    /// Enable/disable underline
    pub fn underline(mut self, enabled: bool) -> Self {
        self.underline = enabled;
        self
    }

    /// Get the target path
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get the label
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Check if active
    pub fn is_active(&self) -> bool {
        self.active
    }
}

impl View for Link {
    crate::impl_view_meta!("Link");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let (fg, bg) = if self.active {
            (self.active_fg, self.active_bg)
        } else {
            (self.fg, None)
        };

        for (i, ch) in self.label.chars().take(area.width as usize).enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            if self.active {
                cell.modifier |= crate::render::Modifier::BOLD;
            }
            if self.underline && !self.active {
                cell.modifier |= crate::render::Modifier::UNDERLINE;
            }
            ctx.buffer.set(area.x + i as u16, area.y, cell);
        }
    }
}

impl_styled_view!(Link);
impl_props_builders!(Link);

/// Helper to create a link
pub fn link(path: impl Into<String>, label: impl Into<String>) -> Link {
    Link::new(path, label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_new() {
        let l = Link::new("/home", "Home");
        assert_eq!(l.path(), "/home");
        assert_eq!(l.label(), "Home");
        assert!(!l.is_active());
    }

    #[test]
    fn test_link_active() {
        let l = Link::new("/", "Home").active(true);
        assert!(l.is_active());
    }

    #[test]
    fn test_link_builder_chain() {
        let l = Link::new("/about", "About")
            .fg(Color::RED)
            .active_fg(Color::GREEN)
            .active_bg(Color::YELLOW)
            .underline(false)
            .active(true);

        assert!(l.is_active());
        assert_eq!(l.fg, Some(Color::RED));
        assert_eq!(l.active_fg, Some(Color::GREEN));
        assert_eq!(l.active_bg, Some(Color::YELLOW));
        assert!(!l.underline);
    }

    #[test]
    fn test_link_helper() {
        let l = link("/test", "Test");
        assert_eq!(l.path(), "/test");
        assert_eq!(l.label(), "Test");
    }
}
