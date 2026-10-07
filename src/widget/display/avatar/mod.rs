//! Avatar widget for user/entity representation

mod render;
mod types;

pub use types::{AvatarShape, AvatarSize};

use crate::style::Color;
use crate::widget::theme::DISABLED_FG;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// An avatar widget for user representation
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// hstack()
///     .child(avatar("John Doe").circle())
///     .child(text("John Doe"))
/// ```
#[derive(Clone)]
pub struct Avatar {
    /// Name to derive initials from
    name: String,
    /// Custom initials (overrides name-derived)
    initials: Option<String>,
    /// Size
    size: AvatarSize,
    /// Shape
    shape: AvatarShape,
    /// Background color
    bg_color: Option<Color>,
    /// Foreground color
    fg_color: Option<Color>,
    /// Status indicator color (online/offline dot)
    status: Option<Color>,
    /// Icon character (instead of initials)
    icon: Option<char>,
    /// Widget props for CSS integration
    props: WidgetProps,
}

impl Avatar {
    /// Create a new avatar from a name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            initials: None,
            size: AvatarSize::Medium,
            shape: AvatarShape::Circle,
            bg_color: None,
            fg_color: None,
            status: None,
            icon: None,
            props: WidgetProps::new(),
        }
    }

    /// Create an avatar with custom initials
    pub fn from_initials(initials: impl Into<String>) -> Self {
        Self {
            name: String::new(),
            initials: Some(initials.into()),
            size: AvatarSize::Medium,
            shape: AvatarShape::Circle,
            bg_color: None,
            fg_color: None,
            status: None,
            icon: None,
            props: WidgetProps::new(),
        }
    }

    /// Create an avatar with an icon
    pub fn from_icon(icon: char) -> Self {
        Self {
            name: String::new(),
            initials: None,
            size: AvatarSize::Medium,
            shape: AvatarShape::Circle,
            bg_color: None,
            fg_color: None,
            status: None,
            icon: Some(icon),
            props: WidgetProps::new(),
        }
    }

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

impl Default for Avatar {
    fn default() -> Self {
        Self::new("")
    }
}

impl_styled_view!(Avatar);
impl_props_builders!(Avatar);

/// Create a new avatar from a name
pub fn avatar(name: impl Into<String>) -> Avatar {
    Avatar::new(name)
}

/// Create an avatar with an icon
pub fn avatar_icon(icon: char) -> Avatar {
    Avatar::from_icon(icon)
}
