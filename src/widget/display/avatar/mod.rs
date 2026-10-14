//! Avatar widget for user/entity representation

mod builders;
mod color;
mod initials;
mod types;
mod view;

pub use types::{AvatarShape, AvatarSize};

use crate::style::Color;
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
            initials: Some(initials.into()),
            ..Self::new(String::new())
        }
    }

    /// Create an avatar with an icon
    pub fn from_icon(icon: char) -> Self {
        Self {
            icon: Some(icon),
            ..Self::new(String::new())
        }
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
