//! Skeleton widget for loading placeholders

mod builders;
mod view;

use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Skeleton shape variant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkeletonShape {
    /// Rectangle/line (default)
    #[default]
    Rectangle,
    /// Circle/avatar placeholder
    Circle,
    /// Multiple lines (paragraph)
    Paragraph,
}

/// A skeleton widget for loading placeholders
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// if loading {
///     skeleton().width(20).height(3)
/// } else {
///     text("Content loaded!")
/// }
/// ```
#[derive(Clone)]
pub struct Skeleton {
    /// Width (0 = fill)
    width: u16,
    /// Height
    height: u16,
    /// Shape
    shape: SkeletonShape,
    /// Number of lines (for paragraph)
    lines: u16,
    /// Animation frame (for pulse effect)
    frame: u8,
    /// Animate
    animate: bool,
    /// Color
    /// The color the builder named, if it named one.
    ///
    /// `None` is not the same as `Some(SEPARATOR_COLOR)`. While this was a plain
    /// `Color`, naming the default explicitly was indistinguishable from
    /// saying nothing, so a stylesheet outranked a builder that had spoken.
    color: Option<Color>,
    /// Wave character
    wave_char: char,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Skeleton {
    /// Create a new skeleton
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 1,
            shape: SkeletonShape::Rectangle,
            lines: 3,
            frame: 0,
            animate: true,
            color: None,
            wave_char: '░',
            props: WidgetProps::new(),
        }
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(Skeleton);
impl_props_builders!(Skeleton);

/// Create a new skeleton
pub fn skeleton() -> Skeleton {
    Skeleton::new()
}

/// Create a text line skeleton
pub fn skeleton_text() -> Skeleton {
    Skeleton::new().height(1)
}

/// Create an avatar skeleton
pub fn skeleton_avatar() -> Skeleton {
    Skeleton::new().circle().height(3)
}

/// Create a paragraph skeleton
pub fn skeleton_paragraph() -> Skeleton {
    Skeleton::new().paragraph().lines(3)
}
