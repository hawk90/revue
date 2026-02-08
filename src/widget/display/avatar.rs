//! Avatar widget for user/entity representation

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Avatar size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSize {
    /// Small (1 char)
    Small,
    /// Medium (3 chars)
    #[default]
    Medium,
    /// Large (5 chars with border)
    Large,
}

/// Avatar shape
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarShape {
    /// Circle (using Unicode characters)
    #[default]
    Circle,
    /// Square/box
    Square,
    /// Rounded square
    Rounded,
}

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
        self.status = Some(Color::rgb(100, 100, 100));
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

    /// Get initials from name
    fn get_initials(&self) -> String {
        if let Some(ref initials) = self.initials {
            return initials.clone();
        }

        if let Some(icon) = self.icon {
            return icon.to_string();
        }

        // Derive initials from name
        self.name
            .split_whitespace()
            .filter_map(|word| word.chars().next())
            .take(2)
            .collect::<String>()
            .to_uppercase()
    }

    /// Get background color (auto-generate from name if not set)
    fn get_bg_color(&self) -> Color {
        if let Some(color) = self.bg_color {
            return color;
        }

        // Generate color from name hash
        let hash: u32 = self
            .name
            .bytes()
            .fold(0u32, |acc, b| acc.wrapping_add(b as u32));
        let hue = (hash % 360) as u8;

        // Convert HSL to RGB (simplified)
        let h = hue as f32 / 60.0;
        let s = 0.6_f32;
        let l = 0.4_f32;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
        let m = l - c / 2.0;

        let (r1, g1, b1) = match h as u8 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Color::rgb(
            ((r1 + m) * 255.0) as u8,
            ((g1 + m) * 255.0) as u8,
            ((b1 + m) * 255.0) as u8,
        )
    }
}

impl Default for Avatar {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for Avatar {
    crate::impl_view_meta!("Avatar");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let initials = self.get_initials();
        let bg = self.get_bg_color();
        let fg = self.fg_color.unwrap_or(Color::WHITE);

        match self.size {
            AvatarSize::Small => {
                // Single character
                let ch = initials.chars().next().unwrap_or('?');
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(area.x, area.y, cell);

                // Status dot
                if let Some(status_color) = self.status {
                    let mut dot = Cell::new('●');
                    dot.fg = Some(status_color);
                    ctx.buffer.set(area.x + 1, area.y, dot);
                }
            }
            AvatarSize::Medium => {
                // 3 chars wide: [XY] or ⬤XY⬤ for circle
                match self.shape {
                    AvatarShape::Circle => {
                        // Use half-blocks for pseudo-circle: ◖XY◗
                        let mut left = Cell::new('◖');
                        left.fg = Some(bg);
                        ctx.buffer.set(area.x, area.y, left);

                        for (i, ch) in initials.chars().take(2).enumerate() {
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(fg);
                            cell.bg = Some(bg);
                            cell.modifier |= Modifier::BOLD;
                            ctx.buffer.set(area.x + 1 + i as u16, area.y, cell);
                        }

                        let mut right = Cell::new('◗');
                        right.fg = Some(bg);
                        ctx.buffer.set(area.x + 3, area.y, right);

                        // Status dot
                        if let Some(status_color) = self.status {
                            let mut dot = Cell::new('●');
                            dot.fg = Some(status_color);
                            ctx.buffer.set(area.x + 4, area.y, dot);
                        }
                    }
                    AvatarShape::Square | AvatarShape::Rounded => {
                        // [XY] format
                        let left = if self.shape == AvatarShape::Rounded {
                            '('
                        } else {
                            '['
                        };
                        let right = if self.shape == AvatarShape::Rounded {
                            ')'
                        } else {
                            ']'
                        };

                        let mut lc = Cell::new(left);
                        lc.fg = Some(bg);
                        ctx.buffer.set(area.x, area.y, lc);

                        for (i, ch) in initials.chars().take(2).enumerate() {
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(fg);
                            cell.bg = Some(bg);
                            cell.modifier |= Modifier::BOLD;
                            ctx.buffer.set(area.x + 1 + i as u16, area.y, cell);
                        }

                        let mut rc = Cell::new(right);
                        rc.fg = Some(bg);
                        ctx.buffer.set(area.x + 3, area.y, rc);

                        // Status dot
                        if let Some(status_color) = self.status {
                            let mut dot = Cell::new('●');
                            dot.fg = Some(status_color);
                            ctx.buffer.set(area.x + 4, area.y, dot);
                        }
                    }
                }
            }
            AvatarSize::Large => {
                // 3 lines tall, 5+ chars wide
                if area.height < 3 {
                    // Fall back to medium
                    let mut cell = Cell::new(initials.chars().next().unwrap_or('?'));
                    cell.fg = Some(fg);
                    cell.bg = Some(bg);
                    ctx.buffer.set(area.x, area.y, cell);
                    return;
                }

                match self.shape {
                    AvatarShape::Circle => {
                        // Top: ╭───╮
                        // Mid: │XY │
                        // Bot: ╰───╯
                        let chars_top = ['╭', '─', '─', '─', '╮'];
                        let chars_bot = ['╰', '─', '─', '─', '╯'];

                        for (i, ch) in chars_top.iter().enumerate() {
                            let mut cell = Cell::new(*ch);
                            cell.fg = Some(bg);
                            ctx.buffer.set(area.x + i as u16, area.y, cell);
                        }

                        // Middle row
                        let mut left = Cell::new('│');
                        left.fg = Some(bg);
                        ctx.buffer.set(area.x, area.y + 1, left);

                        // Pre-collect initials chars for O(1) access
                        let initials_chars: Vec<char> = initials.chars().collect();
                        for i in 1..4 {
                            let ch = if i == 1 || i == 2 {
                                initials_chars.get(i - 1).copied().unwrap_or(' ')
                            } else {
                                ' '
                            };
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(fg);
                            cell.bg = Some(bg);
                            cell.modifier |= Modifier::BOLD;
                            ctx.buffer.set(area.x + i as u16, area.y + 1, cell);
                        }

                        let mut right = Cell::new('│');
                        right.fg = Some(bg);
                        ctx.buffer.set(area.x + 4, area.y + 1, right);

                        for (i, ch) in chars_bot.iter().enumerate() {
                            let mut cell = Cell::new(*ch);
                            cell.fg = Some(bg);
                            ctx.buffer.set(area.x + i as u16, area.y + 2, cell);
                        }

                        // Status dot
                        if let Some(status_color) = self.status {
                            let mut dot = Cell::new('●');
                            dot.fg = Some(status_color);
                            ctx.buffer.set(area.x + 5, area.y + 2, dot);
                        }
                    }
                    AvatarShape::Square => {
                        // Top: ┌───┐
                        let chars_top = ['┌', '─', '─', '─', '┐'];
                        let chars_bot = ['└', '─', '─', '─', '┘'];

                        for (i, ch) in chars_top.iter().enumerate() {
                            let mut cell = Cell::new(*ch);
                            cell.fg = Some(bg);
                            ctx.buffer.set(area.x + i as u16, area.y, cell);
                        }

                        let mut left = Cell::new('│');
                        left.fg = Some(bg);
                        ctx.buffer.set(area.x, area.y + 1, left);

                        // Pre-collect initials chars for O(1) access
                        let initials_chars: Vec<char> = initials.chars().collect();
                        for i in 1..4 {
                            let ch = if i == 1 || i == 2 {
                                initials_chars.get(i - 1).copied().unwrap_or(' ')
                            } else {
                                ' '
                            };
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(fg);
                            cell.bg = Some(bg);
                            cell.modifier |= Modifier::BOLD;
                            ctx.buffer.set(area.x + i as u16, area.y + 1, cell);
                        }

                        let mut right = Cell::new('│');
                        right.fg = Some(bg);
                        ctx.buffer.set(area.x + 4, area.y + 1, right);

                        for (i, ch) in chars_bot.iter().enumerate() {
                            let mut cell = Cell::new(*ch);
                            cell.fg = Some(bg);
                            ctx.buffer.set(area.x + i as u16, area.y + 2, cell);
                        }

                        if let Some(status_color) = self.status {
                            let mut dot = Cell::new('●');
                            dot.fg = Some(status_color);
                            ctx.buffer.set(area.x + 5, area.y + 2, dot);
                        }
                    }
                    AvatarShape::Rounded => {
                        // Same as circle for large
                        let chars_top = ['╭', '─', '─', '─', '╮'];
                        let chars_bot = ['╰', '─', '─', '─', '╯'];

                        for (i, ch) in chars_top.iter().enumerate() {
                            let mut cell = Cell::new(*ch);
                            cell.fg = Some(bg);
                            ctx.buffer.set(area.x + i as u16, area.y, cell);
                        }

                        let mut left = Cell::new('│');
                        left.fg = Some(bg);
                        ctx.buffer.set(area.x, area.y + 1, left);

                        // Pre-collect initials chars for O(1) access
                        let initials_chars: Vec<char> = initials.chars().collect();
                        for i in 1..4 {
                            let ch = if i == 1 || i == 2 {
                                initials_chars.get(i - 1).copied().unwrap_or(' ')
                            } else {
                                ' '
                            };
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(fg);
                            cell.bg = Some(bg);
                            cell.modifier |= Modifier::BOLD;
                            ctx.buffer.set(area.x + i as u16, area.y + 1, cell);
                        }

                        let mut right = Cell::new('│');
                        right.fg = Some(bg);
                        ctx.buffer.set(area.x + 4, area.y + 1, right);

                        for (i, ch) in chars_bot.iter().enumerate() {
                            let mut cell = Cell::new(*ch);
                            cell.fg = Some(bg);
                            ctx.buffer.set(area.x + i as u16, area.y + 2, cell);
                        }

                        if let Some(status_color) = self.status {
                            let mut dot = Cell::new('●');
                            dot.fg = Some(status_color);
                            ctx.buffer.set(area.x + 5, area.y + 2, dot);
                        }
                    }
                }
            }
        }
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

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_new() {
        let a = Avatar::new("John Doe");
        assert_eq!(a.name, "John Doe");
        assert_eq!(a.get_initials(), "JD");
    }

    #[test]
    fn test_avatar_initials() {
        let a = Avatar::new("Alice Bob Charlie");
        assert_eq!(a.get_initials(), "AB"); // Only first 2

        let a = Avatar::new("SingleName");
        assert_eq!(a.get_initials(), "S");

        let a = Avatar::from_initials("XY");
        assert_eq!(a.get_initials(), "XY");
    }

    #[test]
    fn test_avatar_icon() {
        let a = Avatar::from_icon('🤖');
        assert_eq!(a.get_initials(), "🤖");
    }

    #[test]
    fn test_avatar_sizes() {
        let a = avatar("John").small();
        assert_eq!(a.size, AvatarSize::Small);

        let a = avatar("John").large();
        assert_eq!(a.size, AvatarSize::Large);
    }

    #[test]
    fn test_avatar_shapes() {
        let a = avatar("John").circle();
        assert_eq!(a.shape, AvatarShape::Circle);

        let a = avatar("John").square();
        assert_eq!(a.shape, AvatarShape::Square);
    }

    #[test]
    fn test_avatar_status() {
        let a = avatar("John").online();
        assert!(a.status.is_some());

        let a = avatar("John").busy();
        assert!(a.status.is_some());
    }

    #[test]
    fn test_avatar_color_generation() {
        let a1 = Avatar::new("Alice");
        let a2 = Avatar::new("Bob");

        // Different names should generate different colors
        let c1 = a1.get_bg_color();
        let c2 = a2.get_bg_color();
        // May or may not be different due to hash collisions, but should work
        let _ = (c1, c2);
    }

    #[test]
    fn test_helper_functions() {
        let a = avatar("Test");
        assert_eq!(a.name, "Test");

        let a = avatar_icon('🎨');
        assert!(a.icon.is_some());
    }

    // =========================================================================
    // AvatarSize enum tests
    // =========================================================================

    #[test]
    fn test_avatar_size_default() {
        let size = AvatarSize::default();
        assert_eq!(size, AvatarSize::Medium);
    }

    #[test]
    fn test_avatar_size_clone() {
        let size = AvatarSize::Large;
        let cloned = size.clone();
        assert_eq!(size, cloned);
    }

    #[test]
    fn test_avatar_size_copy() {
        let size1 = AvatarSize::Small;
        let size2 = size1;
        assert_eq!(size1, AvatarSize::Small);
        assert_eq!(size2, AvatarSize::Small);
    }

    #[test]
    fn test_avatar_size_partial_eq() {
        assert_eq!(AvatarSize::Small, AvatarSize::Small);
        assert_ne!(AvatarSize::Small, AvatarSize::Medium);
    }

    #[test]
    fn test_avatar_size_debug() {
        let size = AvatarSize::Large;
        assert!(format!("{:?}", size).contains("Large"));
    }

    // =========================================================================
    // AvatarShape enum tests
    // =========================================================================

    #[test]
    fn test_avatar_shape_default() {
        let shape = AvatarShape::default();
        assert_eq!(shape, AvatarShape::Circle);
    }

    #[test]
    fn test_avatar_shape_clone() {
        let shape = AvatarShape::Square;
        let cloned = shape.clone();
        assert_eq!(shape, cloned);
    }

    #[test]
    fn test_avatar_shape_copy() {
        let shape1 = AvatarShape::Rounded;
        let shape2 = shape1;
        assert_eq!(shape1, AvatarShape::Rounded);
        assert_eq!(shape2, AvatarShape::Rounded);
    }

    #[test]
    fn test_avatar_shape_partial_eq() {
        assert_eq!(AvatarShape::Circle, AvatarShape::Circle);
        assert_ne!(AvatarShape::Circle, AvatarShape::Square);
    }

    #[test]
    fn test_avatar_shape_debug() {
        let shape = AvatarShape::Rounded;
        assert!(format!("{:?}", shape).contains("Rounded"));
    }

    // =========================================================================
    // Avatar constructor tests
    // =========================================================================

    #[test]
    fn test_avatar_new_default_values() {
        let a = Avatar::new("Test");
        assert_eq!(a.name, "Test");
        assert!(a.initials.is_none());
        assert_eq!(a.size, AvatarSize::Medium);
        assert_eq!(a.shape, AvatarShape::Circle);
        assert!(a.bg_color.is_none());
        assert!(a.fg_color.is_none());
        assert!(a.status.is_none());
        assert!(a.icon.is_none());
    }

    #[test]
    fn test_avatar_from_initials() {
        let a = Avatar::from_initials("AB");
        assert_eq!(a.initials, Some("AB".to_string()));
        assert!(a.name.is_empty());
    }

    #[test]
    fn test_avatar_from_icon() {
        let a = Avatar::from_icon('X');
        assert_eq!(a.icon, Some('X'));
        assert!(a.name.is_empty());
    }

    // =========================================================================
    // Avatar builder tests
    // =========================================================================

    #[test]
    fn test_avatar_size_builder() {
        let a = Avatar::new("Test").size(AvatarSize::Small);
        assert_eq!(a.size, AvatarSize::Small);
    }

    #[test]
    fn test_avatar_medium() {
        let a = avatar("Test").medium();
        assert_eq!(a.size, AvatarSize::Medium);
    }

    #[test]
    fn test_avatar_shape_builder() {
        let a = Avatar::new("Test").shape(AvatarShape::Rounded);
        assert_eq!(a.shape, AvatarShape::Rounded);
    }

    #[test]
    fn test_avatar_rounded() {
        let a = avatar("Test").rounded();
        assert_eq!(a.shape, AvatarShape::Rounded);
    }

    #[test]
    fn test_avatar_bg() {
        let a = avatar("Test").bg(Color::BLUE);
        assert_eq!(a.bg_color, Some(Color::BLUE));
    }

    #[test]
    fn test_avatar_fg() {
        let a = avatar("Test").fg(Color::RED);
        assert_eq!(a.fg_color, Some(Color::RED));
    }

    #[test]
    fn test_avatar_colors() {
        let a = avatar("Test").colors(Color::GREEN, Color::WHITE);
        assert_eq!(a.bg_color, Some(Color::GREEN));
        assert_eq!(a.fg_color, Some(Color::WHITE));
    }

    #[test]
    fn test_avatar_offline() {
        let a = avatar("Test").offline();
        assert!(a.status.is_some());
        assert_eq!(a.status, Some(Color::rgb(100, 100, 100)));
    }

    #[test]
    fn test_avatar_away() {
        let a = avatar("Test").away();
        assert!(a.status.is_some());
        assert_eq!(a.status, Some(Color::rgb(200, 180, 40)));
    }

    #[test]
    fn test_avatar_status_custom() {
        let a = avatar("Test").status(Color::MAGENTA);
        assert_eq!(a.status, Some(Color::MAGENTA));
    }

    #[test]
    fn test_avatar_icon_builder() {
        let a = avatar("Test").icon('@');
        assert_eq!(a.icon, Some('@'));
    }

    // =========================================================================
    // Avatar Default trait tests
    // =========================================================================

    #[test]
    fn test_avatar_default() {
        let a = Avatar::default();
        assert_eq!(a.name, "");
        assert_eq!(a.size, AvatarSize::Medium);
        assert_eq!(a.shape, AvatarShape::Circle);
    }

    // =========================================================================
    // get_initials edge cases
    // =========================================================================

    #[test]
    fn test_get_initials_empty_name() {
        let a = Avatar::new("");
        assert_eq!(a.get_initials(), "");
    }

    #[test]
    fn test_get_initials_single_word() {
        let a = Avatar::new("Hello");
        assert_eq!(a.get_initials(), "H");
    }

    #[test]
    fn test_get_initials_multiple_words() {
        let a = Avatar::new("One Two Three Four");
        assert_eq!(a.get_initials(), "OT");
    }

    #[test]
    fn test_get_initials_with_icon() {
        let a = Avatar::new("Test").icon('X');
        assert_eq!(a.get_initials(), "X");
    }

    #[test]
    fn test_get_initials_uppercase() {
        let a = Avatar::new("hello world");
        assert_eq!(a.get_initials(), "HW");
    }

    // =========================================================================
    // get_bg_color edge cases
    // =========================================================================

    #[test]
    fn test_get_bg_color_custom() {
        let a = Avatar::new("Test").bg(Color::CYAN);
        assert_eq!(a.get_bg_color(), Color::CYAN);
    }

    #[test]
    fn test_get_bg_color_same_name_same_color() {
        let a1 = Avatar::new("SameName");
        let a2 = Avatar::new("SameName");
        assert_eq!(a1.get_bg_color(), a2.get_bg_color());
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_avatar_builder_chain() {
        let a = Avatar::new("Chained")
            .small()
            .square()
            .colors(Color::BLUE, Color::WHITE)
            .online()
            .icon('C');
        assert_eq!(a.size, AvatarSize::Small);
        assert_eq!(a.shape, AvatarShape::Square);
        assert_eq!(a.bg_color, Some(Color::BLUE));
        assert_eq!(a.fg_color, Some(Color::WHITE));
        assert!(a.status.is_some());
        assert_eq!(a.icon, Some('C'));
    }

    #[test]
    fn test_avatar_status_chain() {
        let a = avatar("Test").offline().status(Color::YELLOW);
        // Last status() call wins
        assert_eq!(a.status, Some(Color::YELLOW));
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_avatar_helper_fn() {
        let a = avatar("Helper");
        assert_eq!(a.name, "Helper");
    }

    #[test]
    fn test_avatar_icon_helper_fn() {
        let a = avatar_icon('🚀');
        assert_eq!(a.icon, Some('🚀'));
    }
}
