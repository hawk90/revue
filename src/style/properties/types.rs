//! CSS property type definitions
//!
//! This module contains all enums, structs, and type aliases for CSS properties.

/// Display mode for layout
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Display {
    /// Flexbox layout (default)
    #[default]
    Flex,
    /// Block layout
    Block,
    /// CSS Grid layout
    Grid,
    /// Hidden - element takes no space
    None,
}

/// Position mode for layout
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Position {
    /// Static positioning (default, normal flow)
    #[default]
    Static,
    /// Relative to normal position
    Relative,
    /// Positioned relative to nearest positioned ancestor
    Absolute,
    /// Positioned relative to viewport
    Fixed,
}

/// Grid track sizing
#[derive(Debug, Clone, Default, PartialEq)]
pub enum GridTrack {
    /// Fixed size in cells
    Fixed(u16),
    /// Fractional unit (fr)
    Fr(f32),
    /// Automatic sizing
    #[default]
    Auto,
    /// Minimum content
    MinContent,
    /// Maximum content
    MaxContent,
}

/// Grid template (columns or rows)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GridTemplate {
    /// Track definitions
    pub tracks: Vec<GridTrack>,
}

impl GridTemplate {
    /// Create a new grid template
    pub fn new(tracks: Vec<GridTrack>) -> Self {
        Self { tracks }
    }

    /// Create a template with repeated tracks
    pub fn repeat(count: usize, track: GridTrack) -> Self {
        Self {
            tracks: vec![track; count],
        }
    }

    /// Create from fr values
    pub fn fr(values: &[f32]) -> Self {
        Self {
            tracks: values.iter().map(|&v| GridTrack::Fr(v)).collect(),
        }
    }

    /// Create from fixed values
    pub fn fixed(values: &[u16]) -> Self {
        Self {
            tracks: values.iter().map(|&v| GridTrack::Fixed(v)).collect(),
        }
    }
}

/// Grid placement for a single axis
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GridPlacement {
    /// Start line (1-based, 0 = auto)
    pub start: i16,
    /// End line (1-based, 0 = auto)
    pub end: i16,
}

impl GridPlacement {
    /// Create auto placement
    pub fn auto() -> Self {
        Self { start: 0, end: 0 }
    }

    /// Place at a specific line
    pub fn line(line: i16) -> Self {
        Self {
            start: line,
            end: 0,
        }
    }

    /// Span a number of tracks
    pub fn span(count: i16) -> Self {
        Self {
            start: 0,
            end: -count,
        } // Negative end means span
    }

    /// Place from start to end line
    pub fn from_to(start: i16, end: i16) -> Self {
        Self { start, end }
    }
}

/// Flex direction for flexbox layout
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FlexDirection {
    /// Horizontal layout (default)
    #[default]
    Row,
    /// Vertical layout
    Column,
}

/// Main axis alignment for flexbox
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum JustifyContent {
    /// Align to start (default)
    #[default]
    Start,
    /// Center alignment
    Center,
    /// Align to end
    End,
    /// Space between items
    SpaceBetween,
    /// Space around items
    SpaceAround,
}

/// Cross axis alignment for flexbox
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AlignItems {
    /// Align to start (default)
    #[default]
    Start,
    /// Center alignment
    Center,
    /// Align to end
    End,
    /// Stretch to fill
    Stretch,
}

/// Spacing values for padding and margin
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Spacing {
    /// Top spacing
    pub top: u16,
    /// Right spacing
    pub right: u16,
    /// Bottom spacing
    pub bottom: u16,
    /// Left spacing
    pub left: u16,
}

impl Spacing {
    /// Create spacing with same value on all sides
    pub fn all(value: u16) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }

    /// Create spacing with vertical (top/bottom) values only
    pub fn vertical(value: u16) -> Self {
        Self {
            top: value,
            right: 0,
            bottom: value,
            left: 0,
        }
    }

    /// Create spacing with horizontal (left/right) values only
    pub fn horizontal(value: u16) -> Self {
        Self {
            top: 0,
            right: value,
            bottom: 0,
            left: value,
        }
    }

    /// Create spacing with individual values
    pub fn new(top: u16, right: u16, bottom: u16, left: u16) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

/// Size constraint for width/height
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Size {
    /// Automatic sizing (default)
    #[default]
    Auto,
    /// Fixed size in cells
    Fixed(u16),
    /// Percentage of parent
    Percent(f32),
}

/// Border style options
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BorderStyle {
    /// No border (default)
    #[default]
    None,
    /// Solid line border
    Solid,
    /// Dashed line border
    Dashed,
    /// Double line border
    Double,
    /// Rounded corner border
    Rounded,
}

/// RGBA color value with alpha channel
///
/// Alpha channel: 0 = fully transparent, 255 = fully opaque.
/// Default color has alpha=0 to indicate "unset" state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha component (0=transparent, 255=opaque)
    pub a: u8,
}

impl Default for Color {
    /// Default color is transparent black (unset state)
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

/// CSS property enum for dynamic property handling
#[derive(Debug, Clone)]
pub enum Property {
    /// Display property value
    Display(Display),
    /// Flex direction property value
    FlexDirection(FlexDirection),
    /// Padding property value
    Padding(Spacing),
    /// Margin property value
    Margin(Spacing),
    /// Color property value
    Color(Color),
    /// Background property value
    Background(Color),
    /// Border style property value
    BorderStyle(BorderStyle),
    /// Border color property value
    BorderColor(Color),
}
