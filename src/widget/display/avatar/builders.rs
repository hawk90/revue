//! Builder methods: size, shape, colors, status and icon

use super::{Avatar, AvatarShape, AvatarSize};
use crate::style::Color;
use crate::widget::theme::DISABLED_FG;

impl Avatar {
    /// Set size
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    /// Small size shorthand
    pub fn small(mut self) -> Self {
        self.size = AvatarSize::Small;
        self
    }

    /// Medium size shorthand
    pub fn medium(mut self) -> Self {
        self.size = AvatarSize::Medium;
        self
    }

    /// Large size shorthand
    pub fn large(mut self) -> Self {
        self.size = AvatarSize::Large;
        self
    }

    /// Set shape
    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    /// Circle shape shorthand
    pub fn circle(mut self) -> Self {
        self.shape = AvatarShape::Circle;
        self
    }

    /// Square shape shorthand
    pub fn square(mut self) -> Self {
        self.shape = AvatarShape::Square;
        self
    }

    /// Rounded shape shorthand
    pub fn rounded(mut self) -> Self {
        self.shape = AvatarShape::Rounded;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }

    /// Set colors
    pub fn colors(mut self, bg: Color, fg: Color) -> Self {
        self.bg_color = Some(bg);
        self.fg_color = Some(fg);
        self
    }

    /// Set online status
    pub fn online(mut self) -> Self {
        self.status = Some(Color::rgb(40, 200, 80));
        self
    }

    /// Set offline status
    pub fn offline(mut self) -> Self {
        self.status = Some(DISABLED_FG);
        self
    }

    /// Set away status
    pub fn away(mut self) -> Self {
        self.status = Some(Color::rgb(200, 180, 40));
        self
    }

    /// Set busy status
    pub fn busy(mut self) -> Self {
        self.status = Some(Color::rgb(200, 60, 60));
        self
    }

    /// Set custom status color
    pub fn status(mut self, color: Color) -> Self {
        self.status = Some(color);
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }
}
