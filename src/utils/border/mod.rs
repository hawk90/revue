//! Border rendering utilities
//!
//! Common border drawing functions used across widgets.

mod title;

use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::RenderContext;

pub use title::{
    draw_border_title, draw_border_titles, draw_title, draw_title_center, draw_title_right,
    BorderEdge, BorderTitle, TitlePosition,
};

/// Border character set
#[derive(Clone, Copy, Debug)]
pub struct BorderChars {
    /// Top-left corner
    pub top_left: char,
    /// Top-right corner
    pub top_right: char,
    /// Bottom-left corner
    pub bottom_left: char,
    /// Bottom-right corner
    pub bottom_right: char,
    /// Horizontal line
    pub horizontal: char,
    /// Vertical line
    pub vertical: char,
}

impl BorderChars {
    /// Standard single-line border
    pub const SINGLE: Self = Self {
        top_left: '┌',
        top_right: '┐',
        bottom_left: '└',
        bottom_right: '┘',
        horizontal: '─',
        vertical: '│',
    };

    /// Rounded corner border
    pub const ROUNDED: Self = Self {
        top_left: '╭',
        top_right: '╮',
        bottom_left: '╰',
        bottom_right: '╯',
        horizontal: '─',
        vertical: '│',
    };

    /// Double-line border
    pub const DOUBLE: Self = Self {
        top_left: '╔',
        top_right: '╗',
        bottom_left: '╚',
        bottom_right: '╝',
        horizontal: '═',
        vertical: '║',
    };

    /// Bold/thick border
    pub const BOLD: Self = Self {
        top_left: '┏',
        top_right: '┓',
        bottom_left: '┗',
        bottom_right: '┛',
        horizontal: '━',
        vertical: '┃',
    };

    /// ASCII border
    pub const ASCII: Self = Self {
        top_left: '+',
        top_right: '+',
        bottom_left: '+',
        bottom_right: '+',
        horizontal: '-',
        vertical: '|',
    };
}

impl Default for BorderChars {
    fn default() -> Self {
        Self::SINGLE
    }
}

/// Border style configuration
#[derive(Clone, Copy, Debug, Default)]
pub struct BorderStyle {
    /// Character set to use
    pub chars: BorderChars,
    /// Border color
    pub color: Option<Color>,
    /// Background color for border cells
    pub bg: Option<Color>,
}

impl BorderStyle {
    /// Create a new border style with color
    pub fn new(color: Color) -> Self {
        Self {
            chars: BorderChars::SINGLE,
            color: Some(color),
            bg: None,
        }
    }

    /// Use rounded corners
    pub fn rounded(mut self) -> Self {
        self.chars = BorderChars::ROUNDED;
        self
    }

    /// Use double lines
    pub fn double(mut self) -> Self {
        self.chars = BorderChars::DOUBLE;
        self
    }

    /// Use bold lines
    pub fn bold(mut self) -> Self {
        self.chars = BorderChars::BOLD;
        self
    }

    /// Use ASCII characters
    pub fn ascii(mut self) -> Self {
        self.chars = BorderChars::ASCII;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }
}

/// Render a border around the given area
///
/// # Arguments
/// * `ctx` - Render context
/// * `area` - Area to draw border around
/// * `color` - Border color
///
/// # Example
/// ```ignore
/// render_border(ctx, area, Color::WHITE);
/// ```
pub fn render_border(ctx: &mut RenderContext, area: Rect, color: Color) {
    render_border_with_style(ctx, area, BorderStyle::new(color));
}

/// Render a rounded border
pub fn render_rounded_border(ctx: &mut RenderContext, area: Rect, color: Color) {
    render_border_with_style(ctx, area, BorderStyle::new(color).rounded());
}

/// Render a border with custom style
pub fn render_border_with_style(ctx: &mut RenderContext, area: Rect, style: BorderStyle) {
    if area.width < 2 || area.height < 2 {
        return;
    }

    let chars = style.chars;
    let fg = style.color;
    let bg = style.bg;

    // Top border
    for x in area.x..area.x + area.width {
        let ch = if x == area.x {
            chars.top_left
        } else if x == area.x + area.width - 1 {
            chars.top_right
        } else {
            chars.horizontal
        };
        let mut cell = Cell::new(ch);
        cell.fg = fg;
        cell.bg = bg;
        ctx.buffer.set(x, area.y, cell);
    }

    // Bottom border
    for x in area.x..area.x + area.width {
        let ch = if x == area.x {
            chars.bottom_left
        } else if x == area.x + area.width - 1 {
            chars.bottom_right
        } else {
            chars.horizontal
        };
        let mut cell = Cell::new(ch);
        cell.fg = fg;
        cell.bg = bg;
        ctx.buffer.set(x, area.y + area.height - 1, cell);
    }

    // Left and right borders (excluding corners)
    for y in area.y + 1..area.y + area.height - 1 {
        let mut left = Cell::new(chars.vertical);
        left.fg = fg;
        left.bg = bg;
        ctx.buffer.set(area.x, y, left);

        let mut right = Cell::new(chars.vertical);
        right.fg = fg;
        right.bg = bg;
        ctx.buffer.set(area.x + area.width - 1, y, right);
    }
}

/// Fill area with background color
pub fn fill_bg(ctx: &mut RenderContext, area: Rect, color: Color) {
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(color);
            ctx.buffer.set(x, y, cell);
        }
    }
}

/// Fill area inside border with background color
pub fn fill_inner_bg(ctx: &mut RenderContext, area: Rect, color: Color) {
    if area.width <= 2 || area.height <= 2 {
        return;
    }
    let inner = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
    fill_bg(ctx, inner, color);
}

/// Draw a horizontal line
pub fn draw_hline(ctx: &mut RenderContext, x: u16, y: u16, width: u16, color: Color) {
    for dx in 0..width {
        let mut cell = Cell::new('─');
        cell.fg = Some(color);
        ctx.buffer.set(x + dx, y, cell);
    }
}

/// Draw a vertical line
pub fn draw_vline(ctx: &mut RenderContext, x: u16, y: u16, height: u16, color: Color) {
    for dy in 0..height {
        let mut cell = Cell::new('│');
        cell.fg = Some(color);
        ctx.buffer.set(x, y + dy, cell);
    }
}

/// Draw a horizontal separator (with T-junctions)
pub fn draw_separator(ctx: &mut RenderContext, area: Rect, y: u16, color: Color) {
    if y <= area.y || y >= area.y + area.height - 1 {
        return;
    }

    let mut left = Cell::new('├');
    left.fg = Some(color);
    ctx.buffer.set(area.x, y, left);

    for x in area.x + 1..area.x + area.width - 1 {
        let mut h = Cell::new('─');
        h.fg = Some(color);
        ctx.buffer.set(x, y, h);
    }

    let mut right = Cell::new('┤');
    right.fg = Some(color);
    ctx.buffer.set(area.x + area.width - 1, y, right);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;

    #[test]
    fn test_border_chars() {
        assert_eq!(BorderChars::SINGLE.top_left, '┌');
        assert_eq!(BorderChars::ROUNDED.top_left, '╭');
        assert_eq!(BorderChars::DOUBLE.top_left, '╔');
    }

    #[test]
    fn test_border_style() {
        let style = BorderStyle::new(Color::WHITE).rounded();
        assert_eq!(style.chars.top_left, '╭');
    }

    #[test]
    fn test_render_border() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render_border(&mut ctx, area, Color::WHITE);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, '┐');
        assert_eq!(buffer.get(0, 4).unwrap().symbol, '└');
        assert_eq!(buffer.get(9, 4).unwrap().symbol, '┘');
        assert_eq!(buffer.get(5, 0).unwrap().symbol, '─');
        assert_eq!(buffer.get(0, 2).unwrap().symbol, '│');
    }

    #[test]
    fn test_render_rounded_border() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render_rounded_border(&mut ctx, area, Color::WHITE);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, '╮');
    }

    #[test]
    fn test_fill_bg() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        fill_bg(&mut ctx, area, Color::RED);

        assert_eq!(buffer.get(5, 2).unwrap().bg, Some(Color::RED));
    }

    #[test]
    fn test_draw_separator() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render_border(&mut ctx, area, Color::WHITE);
        draw_separator(&mut ctx, area, 2, Color::WHITE);

        assert_eq!(buffer.get(0, 2).unwrap().symbol, '├');
        assert_eq!(buffer.get(9, 2).unwrap().symbol, '┤');
    }
}
