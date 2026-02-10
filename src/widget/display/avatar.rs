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
