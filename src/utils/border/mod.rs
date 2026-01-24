//! Border rendering utilities
//!
//! Common border drawing functions used across widgets.

use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::RenderContext;

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

// ============================================================================
// Border Title
// ============================================================================

/// Position for border title
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum TitlePosition {
    /// Left/Top aligned
    #[default]
    Start,
    /// Center aligned
    Center,
    /// Right/Bottom aligned
    End,
}

/// Edge of the border for title placement
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum BorderEdge {
    /// Top border (horizontal)
    #[default]
    Top,
    /// Bottom border (horizontal)
    Bottom,
    /// Left border (vertical)
    Left,
    /// Right border (vertical)
    Right,
}

/// A title or info section to draw on a border
#[derive(Clone, Debug)]
pub struct BorderTitle {
    /// Text content (supports any chars including nerd font icons)
    pub text: String,
    /// Which edge to draw on
    pub edge: BorderEdge,
    /// Position along the edge
    pub position: TitlePosition,
    /// Foreground color
    pub fg: Option<Color>,
    /// Background color (uses border bg if None)
    pub bg: Option<Color>,
    /// Padding before text
    pub pad_start: u16,
    /// Padding after text
    pub pad_end: u16,
    /// Offset from calculated position (can be negative via wrapping)
    pub offset: i16,
}

impl BorderTitle {
    /// Create a new border title
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            edge: BorderEdge::Top,
            position: TitlePosition::Start,
            fg: None,
            bg: None,
            pad_start: 1,
            pad_end: 1,
            offset: 0,
        }
    }

    /// Set the edge (top, bottom, left, right)
    pub fn edge(mut self, edge: BorderEdge) -> Self {
        self.edge = edge;
        self
    }

    /// Set position along edge (start, center, end)
    pub fn position(mut self, pos: TitlePosition) -> Self {
        self.position = pos;
        self
    }

    /// Convenience: top edge
    pub fn top(mut self) -> Self {
        self.edge = BorderEdge::Top;
        self
    }

    /// Convenience: bottom edge
    pub fn bottom(mut self) -> Self {
        self.edge = BorderEdge::Bottom;
        self
    }

    /// Convenience: left edge
    pub fn left(mut self) -> Self {
        self.edge = BorderEdge::Left;
        self
    }

    /// Convenience: right edge
    pub fn right(mut self) -> Self {
        self.edge = BorderEdge::Right;
        self
    }

    /// Convenience: start position
    pub fn start(mut self) -> Self {
        self.position = TitlePosition::Start;
        self
    }

    /// Convenience: center position
    pub fn center(mut self) -> Self {
        self.position = TitlePosition::Center;
        self
    }

    /// Convenience: end position
    pub fn end(mut self) -> Self {
        self.position = TitlePosition::End;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set padding (both sides)
    pub fn padding(mut self, pad: u16) -> Self {
        self.pad_start = pad;
        self.pad_end = pad;
        self
    }

    /// Set start padding only
    pub fn pad_start(mut self, pad: u16) -> Self {
        self.pad_start = pad;
        self
    }

    /// Set end padding only
    pub fn pad_end(mut self, pad: u16) -> Self {
        self.pad_end = pad;
        self
    }

    /// Set offset from calculated position
    pub fn offset(mut self, offset: i16) -> Self {
        self.offset = offset;
        self
    }

    /// Get the display width of the title (text + padding)
    pub fn width(&self) -> u16 {
        crate::utils::unicode::display_width(&self.text) as u16 + self.pad_start + self.pad_end
    }
}

/// Draw a border title on the given area
///
/// This draws the title text with padding, replacing the border characters.
/// Use this after drawing the border.
///
/// # Example
/// ```ignore
/// render_border(ctx, area, Color::BLUE);
/// draw_border_title(ctx, area, &BorderTitle::new(" Title ").fg(Color::BLUE));
/// draw_border_title(ctx, area, &BorderTitle::new("Info").end().fg(Color::GREEN));
/// ```
pub fn draw_border_title(ctx: &mut RenderContext, area: Rect, title: &BorderTitle) {
    if area.width < 3 || area.height < 2 {
        return;
    }

    let text_width = crate::utils::unicode::display_width(&title.text) as u16;
    let total_width = text_width + title.pad_start + title.pad_end;

    match title.edge {
        BorderEdge::Top | BorderEdge::Bottom => {
            let available = area.width.saturating_sub(2); // Exclude corners
            if total_width > available {
                return;
            }

            let y = if title.edge == BorderEdge::Top {
                area.y
            } else {
                area.y + area.height - 1
            };

            // Calculate x position
            let base_x = match title.position {
                TitlePosition::Start => area.x + 1,
                TitlePosition::Center => area.x + 1 + (available.saturating_sub(total_width)) / 2,
                TitlePosition::End => area.x + area.width - 1 - total_width,
            };
            let x = (base_x as i16 + title.offset).max(area.x as i16 + 1) as u16;

            // Draw padding (spaces to clear border chars)
            for dx in 0..title.pad_start {
                let mut cell = Cell::new(' ');
                cell.fg = title.fg;
                cell.bg = title.bg;
                ctx.buffer.set(x + dx, y, cell);
            }

            // Draw text
            let text_x = x + title.pad_start;
            for (i, ch) in title.text.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = title.fg;
                cell.bg = title.bg;
                ctx.buffer.set(text_x + i as u16, y, cell);
            }

            // Draw end padding
            let end_x = text_x + text_width;
            for dx in 0..title.pad_end {
                let mut cell = Cell::new(' ');
                cell.fg = title.fg;
                cell.bg = title.bg;
                ctx.buffer.set(end_x + dx, y, cell);
            }
        }
        BorderEdge::Left | BorderEdge::Right => {
            let available = area.height.saturating_sub(2); // Exclude corners
            let text_len = title.text.chars().count() as u16;
            let total_height = text_len + title.pad_start + title.pad_end;

            if total_height > available {
                return;
            }

            let x = if title.edge == BorderEdge::Left {
                area.x
            } else {
                area.x + area.width - 1
            };

            // Calculate y position
            let base_y = match title.position {
                TitlePosition::Start => area.y + 1,
                TitlePosition::Center => area.y + 1 + (available.saturating_sub(total_height)) / 2,
                TitlePosition::End => area.y + area.height - 1 - total_height,
            };
            let y = (base_y as i16 + title.offset).max(area.y as i16 + 1) as u16;

            // Draw padding (spaces)
            for dy in 0..title.pad_start {
                let mut cell = Cell::new(' ');
                cell.fg = title.fg;
                cell.bg = title.bg;
                ctx.buffer.set(x, y + dy, cell);
            }

            // Draw text (vertically)
            let text_y = y + title.pad_start;
            for (i, ch) in title.text.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = title.fg;
                cell.bg = title.bg;
                ctx.buffer.set(x, text_y + i as u16, cell);
            }

            // Draw end padding
            let end_y = text_y + text_len;
            for dy in 0..title.pad_end {
                let mut cell = Cell::new(' ');
                cell.fg = title.fg;
                cell.bg = title.bg;
                ctx.buffer.set(x, end_y + dy, cell);
            }
        }
    }
}

/// Draw multiple border titles
pub fn draw_border_titles(ctx: &mut RenderContext, area: Rect, titles: &[BorderTitle]) {
    for title in titles {
        draw_border_title(ctx, area, title);
    }
}

/// Convenience function: draw a simple title on top-left
pub fn draw_title(ctx: &mut RenderContext, area: Rect, text: &str, color: Color) {
    draw_border_title(ctx, area, &BorderTitle::new(text).fg(color));
}

/// Convenience function: draw info on top-right
pub fn draw_title_right(ctx: &mut RenderContext, area: Rect, text: &str, color: Color) {
    draw_border_title(ctx, area, &BorderTitle::new(text).end().fg(color));
}

/// Convenience function: draw centered title
pub fn draw_title_center(ctx: &mut RenderContext, area: Rect, text: &str, color: Color) {
    draw_border_title(ctx, area, &BorderTitle::new(text).center().fg(color));
}

#[cfg(test)]
mod tests;
