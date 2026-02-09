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

#[cfg(test)]
mod tests {
    use super::*;

    // Display tests
    #[test]
    fn test_display_default() {
        assert_eq!(Display::default(), Display::Flex);
    }

    #[test]
    fn test_display_variants() {
        assert_eq!(Display::Flex, Display::Flex);
        assert_eq!(Display::Block, Display::Block);
        assert_eq!(Display::Grid, Display::Grid);
        assert_eq!(Display::None, Display::None);
    }

    // Position tests
    #[test]
    fn test_position_default() {
        assert_eq!(Position::default(), Position::Static);
    }

    #[test]
    fn test_position_variants() {
        assert_eq!(Position::Static, Position::Static);
        assert_eq!(Position::Relative, Position::Relative);
        assert_eq!(Position::Absolute, Position::Absolute);
        assert_eq!(Position::Fixed, Position::Fixed);
    }

    // GridTrack tests
    #[test]
    fn test_grid_track_default() {
        assert_eq!(GridTrack::default(), GridTrack::Auto);
    }

    #[test]
    fn test_grid_track_fixed() {
        let track = GridTrack::Fixed(10);
        assert_eq!(track, GridTrack::Fixed(10));
    }

    #[test]
    fn test_grid_track_fr() {
        let track = GridTrack::Fr(1.5);
        assert_eq!(track, GridTrack::Fr(1.5));
    }

    #[test]
    fn test_grid_track_min_content() {
        let track = GridTrack::MinContent;
        assert_eq!(track, GridTrack::MinContent);
    }

    #[test]
    fn test_grid_track_max_content() {
        let track = GridTrack::MaxContent;
        assert_eq!(track, GridTrack::MaxContent);
    }

    // GridTemplate tests
    #[test]
    fn test_grid_template_default() {
        let template = GridTemplate::default();
        assert!(template.tracks.is_empty());
    }

    #[test]
    fn test_grid_template_new() {
        let template = GridTemplate::new(vec![GridTrack::Fixed(10), GridTrack::Fr(1.0)]);
        assert_eq!(template.tracks.len(), 2);
    }

    #[test]
    fn test_grid_template_repeat() {
        let template = GridTemplate::repeat(3, GridTrack::Fixed(10));
        assert_eq!(template.tracks.len(), 3);
        assert!(template.tracks.iter().all(|t| t == &GridTrack::Fixed(10)));
    }

    #[test]
    fn test_grid_template_fr() {
        let template = GridTemplate::fr(&[1.0, 2.0, 1.5]);
        assert_eq!(template.tracks.len(), 3);
        assert_eq!(template.tracks[0], GridTrack::Fr(1.0));
        assert_eq!(template.tracks[1], GridTrack::Fr(2.0));
        assert_eq!(template.tracks[2], GridTrack::Fr(1.5));
    }

    #[test]
    fn test_grid_template_fixed() {
        let template = GridTemplate::fixed(&[10, 20, 30]);
        assert_eq!(template.tracks.len(), 3);
        assert_eq!(template.tracks[0], GridTrack::Fixed(10));
        assert_eq!(template.tracks[1], GridTrack::Fixed(20));
        assert_eq!(template.tracks[2], GridTrack::Fixed(30));
    }

    // GridPlacement tests
    #[test]
    fn test_grid_placement_default() {
        let placement = GridPlacement::default();
        assert_eq!(placement.start, 0);
        assert_eq!(placement.end, 0);
    }

    #[test]
    fn test_grid_placement_auto() {
        let placement = GridPlacement::auto();
        assert_eq!(placement.start, 0);
        assert_eq!(placement.end, 0);
    }

    #[test]
    fn test_grid_placement_line() {
        let placement = GridPlacement::line(5);
        assert_eq!(placement.start, 5);
        assert_eq!(placement.end, 0);
    }

    #[test]
    fn test_grid_placement_span() {
        let placement = GridPlacement::span(3);
        assert_eq!(placement.start, 0);
        assert_eq!(placement.end, -3);
    }

    #[test]
    fn test_grid_placement_from_to() {
        let placement = GridPlacement::from_to(2, 5);
        assert_eq!(placement.start, 2);
        assert_eq!(placement.end, 5);
    }

    // FlexDirection tests
    #[test]
    fn test_flex_direction_default() {
        assert_eq!(FlexDirection::default(), FlexDirection::Row);
    }

    #[test]
    fn test_flex_direction_variants() {
        assert_eq!(FlexDirection::Row, FlexDirection::Row);
        assert_eq!(FlexDirection::Column, FlexDirection::Column);
    }

    // JustifyContent tests
    #[test]
    fn test_justify_content_default() {
        assert_eq!(JustifyContent::default(), JustifyContent::Start);
    }

    #[test]
    fn test_justify_content_variants() {
        assert_eq!(JustifyContent::Start, JustifyContent::Start);
        assert_eq!(JustifyContent::Center, JustifyContent::Center);
        assert_eq!(JustifyContent::End, JustifyContent::End);
        assert_eq!(JustifyContent::SpaceBetween, JustifyContent::SpaceBetween);
        assert_eq!(JustifyContent::SpaceAround, JustifyContent::SpaceAround);
    }

    // AlignItems tests
    #[test]
    fn test_align_items_default() {
        assert_eq!(AlignItems::default(), AlignItems::Start);
    }

    #[test]
    fn test_align_items_variants() {
        assert_eq!(AlignItems::Start, AlignItems::Start);
        assert_eq!(AlignItems::Center, AlignItems::Center);
        assert_eq!(AlignItems::End, AlignItems::End);
        assert_eq!(AlignItems::Stretch, AlignItems::Stretch);
    }

    // Spacing tests
    #[test]
    fn test_spacing_default() {
        let spacing = Spacing::default();
        assert_eq!(spacing.top, 0);
        assert_eq!(spacing.right, 0);
        assert_eq!(spacing.bottom, 0);
        assert_eq!(spacing.left, 0);
    }

    #[test]
    fn test_spacing_all() {
        let spacing = Spacing::all(10);
        assert_eq!(spacing.top, 10);
        assert_eq!(spacing.right, 10);
        assert_eq!(spacing.bottom, 10);
        assert_eq!(spacing.left, 10);
    }

    #[test]
    fn test_spacing_vertical() {
        let spacing = Spacing::vertical(10);
        assert_eq!(spacing.top, 10);
        assert_eq!(spacing.right, 0);
        assert_eq!(spacing.bottom, 10);
        assert_eq!(spacing.left, 0);
    }

    #[test]
    fn test_spacing_horizontal() {
        let spacing = Spacing::horizontal(10);
        assert_eq!(spacing.top, 0);
        assert_eq!(spacing.right, 10);
        assert_eq!(spacing.bottom, 0);
        assert_eq!(spacing.left, 10);
    }

    #[test]
    fn test_spacing_new() {
        let spacing = Spacing::new(10, 20, 30, 40);
        assert_eq!(spacing.top, 10);
        assert_eq!(spacing.right, 20);
        assert_eq!(spacing.bottom, 30);
        assert_eq!(spacing.left, 40);
    }

    // Size tests
    #[test]
    fn test_size_default() {
        assert_eq!(Size::default(), Size::Auto);
    }

    #[test]
    fn test_size_auto() {
        assert_eq!(Size::Auto, Size::Auto);
    }

    #[test]
    fn test_size_fixed() {
        let size = Size::Fixed(100);
        assert_eq!(size, Size::Fixed(100));
    }

    #[test]
    fn test_size_percent() {
        let size = Size::Percent(50.0);
        assert_eq!(size, Size::Percent(50.0));
    }

    // BorderStyle tests
    #[test]
    fn test_border_style_default() {
        assert_eq!(BorderStyle::default(), BorderStyle::None);
    }

    #[test]
    fn test_border_style_variants() {
        assert_eq!(BorderStyle::None, BorderStyle::None);
        assert_eq!(BorderStyle::Solid, BorderStyle::Solid);
        assert_eq!(BorderStyle::Dashed, BorderStyle::Dashed);
        assert_eq!(BorderStyle::Double, BorderStyle::Double);
        assert_eq!(BorderStyle::Rounded, BorderStyle::Rounded);
    }

    // Color tests
    #[test]
    fn test_color_default() {
        let color = Color::default();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 0);
    }

    #[test]
    fn test_color_partial_eq() {
        let color1 = Color::rgb(255, 0, 0);
        let color2 = Color::rgb(255, 0, 0);
        assert_eq!(color1, color2);
    }

    #[test]
    fn test_color_partial_ne() {
        let color1 = Color::rgb(255, 0, 0);
        let color2 = Color::rgb(0, 255, 0);
        assert_ne!(color1, color2);
    }

    #[test]
    fn test_color_copy_trait() {
        let color1 = Color::rgb(255, 0, 0);
        let color2 = color1;
        assert_eq!(color1.r, 255);
        assert_eq!(color2.r, 255);
    }
}
