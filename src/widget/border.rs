//! Border/Frame widget for surrounding content

use super::traits::{RenderContext, View, WidgetProps};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::border::BorderChars;
use crate::{impl_props_builders, impl_styled_view};

/// Border style characters
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BorderType {
    /// No border
    None,
    /// Single line: ┌─┐│└┘
    #[default]
    Single,
    /// Double line: ╔═╗║╚╝
    Double,
    /// Rounded: ╭─╮│╰╯
    Rounded,
    /// Thick: ┏━┓┃┗┛
    Thick,
    /// ASCII: +-+|
    Ascii,
}

/// Empty border chars (for BorderType::None)
const NONE_CHARS: BorderChars = BorderChars {
    top_left: ' ',
    top_right: ' ',
    bottom_left: ' ',
    bottom_right: ' ',
    horizontal: ' ',
    vertical: ' ',
};

impl BorderType {
    /// Get the character set for this border type
    pub fn chars(&self) -> BorderChars {
        match self {
            BorderType::None => NONE_CHARS,
            BorderType::Single => BorderChars::SINGLE,
            BorderType::Double => BorderChars::DOUBLE,
            BorderType::Rounded => BorderChars::ROUNDED,
            BorderType::Thick => BorderChars::BOLD,
            BorderType::Ascii => BorderChars::ASCII,
        }
    }
}

/// A border widget that wraps content
pub struct Border {
    child: Option<Box<dyn View>>,
    border_type: BorderType,
    title: Option<String>,
    fg: Option<Color>,
    bg: Option<Color>,
    props: WidgetProps,
}

impl Border {
    /// Create a new border
    pub fn new() -> Self {
        Self {
            child: None,
            border_type: BorderType::Single,
            title: None,
            fg: None,
            bg: None,
            props: WidgetProps::new(),
        }
    }

    /// Set the child widget
    pub fn child(mut self, child: impl View + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    /// Set border type
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    /// Set title (displayed in top border)
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set border color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Preset builders
    // ─────────────────────────────────────────────────────────────────────────

    /// Create single border
    pub fn single() -> Self {
        Self::new().border_type(BorderType::Single)
    }

    /// Create double border
    pub fn double() -> Self {
        Self::new().border_type(BorderType::Double)
    }

    /// Create rounded border
    pub fn rounded() -> Self {
        Self::new().border_type(BorderType::Rounded)
    }

    /// Create thick border
    pub fn thick() -> Self {
        Self::new().border_type(BorderType::Thick)
    }

    /// Create ASCII border (for basic terminals)
    pub fn ascii() -> Self {
        Self::new().border_type(BorderType::Ascii)
    }

    /// Create a panel (double border with cyan color)
    pub fn panel() -> Self {
        Self::new().border_type(BorderType::Double).fg(Color::CYAN)
    }

    /// Create a card (rounded border with white color)
    pub fn card() -> Self {
        Self::new()
            .border_type(BorderType::Rounded)
            .fg(Color::WHITE)
    }

    /// Create an error box (single border with red color)
    pub fn error_box() -> Self {
        Self::new().border_type(BorderType::Single).fg(Color::RED)
    }

    /// Create a success box (single border with green color)
    pub fn success_box() -> Self {
        Self::new().border_type(BorderType::Single).fg(Color::GREEN)
    }
}

impl Default for Border {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Border {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 2 || area.height < 2 {
            return;
        }

        let chars = self.border_type.chars();

        // Top border
        let mut cell = Cell::new(chars.top_left);
        cell.fg = self.fg;
        cell.bg = self.bg;
        ctx.buffer.set(area.x, area.y, cell);

        // Top horizontal line with optional title
        let title_start = if let Some(ref title) = self.title {
            let max_title_len = (area.width as usize).saturating_sub(4);
            let display_title: String = title.chars().take(max_title_len).collect();
            let title_len = display_title.len();

            // Draw horizontal before title
            for x in 1..2 {
                let mut c = Cell::new(chars.horizontal);
                c.fg = self.fg;
                c.bg = self.bg;
                ctx.buffer.set(area.x + x, area.y, c);
            }

            // Draw title
            for (i, ch) in display_title.chars().enumerate() {
                let mut c = Cell::new(ch);
                c.fg = self.fg;
                c.bg = self.bg;
                ctx.buffer.set(area.x + 2 + i as u16, area.y, c);
            }

            2 + title_len as u16
        } else {
            1
        };

        // Rest of top horizontal
        for x in title_start..(area.width - 1) {
            let mut c = Cell::new(chars.horizontal);
            c.fg = self.fg;
            c.bg = self.bg;
            ctx.buffer.set(area.x + x, area.y, c);
        }

        // Top right corner
        let mut cell = Cell::new(chars.top_right);
        cell.fg = self.fg;
        cell.bg = self.bg;
        ctx.buffer.set(area.x + area.width - 1, area.y, cell);

        // Left and right borders
        for y in 1..(area.height - 1) {
            let mut left = Cell::new(chars.vertical);
            left.fg = self.fg;
            left.bg = self.bg;
            ctx.buffer.set(area.x, area.y + y, left);

            let mut right = Cell::new(chars.vertical);
            right.fg = self.fg;
            right.bg = self.bg;
            ctx.buffer.set(area.x + area.width - 1, area.y + y, right);
        }

        // Bottom border
        let mut cell = Cell::new(chars.bottom_left);
        cell.fg = self.fg;
        cell.bg = self.bg;
        ctx.buffer.set(area.x, area.y + area.height - 1, cell);

        for x in 1..(area.width - 1) {
            let mut c = Cell::new(chars.horizontal);
            c.fg = self.fg;
            c.bg = self.bg;
            ctx.buffer.set(area.x + x, area.y + area.height - 1, c);
        }

        let mut cell = Cell::new(chars.bottom_right);
        cell.fg = self.fg;
        cell.bg = self.bg;
        ctx.buffer
            .set(area.x + area.width - 1, area.y + area.height - 1, cell);

        // Render child in inner area
        if let Some(ref child) = self.child {
            let inner = Rect::new(
                area.x + 1,
                area.y + 1,
                area.width.saturating_sub(2),
                area.height.saturating_sub(2),
            );
            let mut child_ctx = RenderContext::new(ctx.buffer, inner);
            child.render(&mut child_ctx);
        }
    }

    crate::impl_view_meta!("Border");
}

/// Helper function to create a border
pub fn border() -> Border {
    Border::new()
}

impl_styled_view!(Border);
impl_props_builders!(Border);

/// Draw a border on a buffer
///
/// This is a utility function that can be used by other widgets to draw borders
/// without needing to create a full Border widget.
///
/// # Arguments
/// * `buffer` - The buffer to draw on
/// * `area` - The area to draw the border in
/// * `border_type` - The type of border to draw
/// * `fg` - Optional foreground color
/// * `bg` - Optional background color
pub fn draw_border(
    buffer: &mut crate::render::Buffer,
    area: Rect,
    border_type: BorderType,
    fg: Option<Color>,
    bg: Option<Color>,
) {
    if area.width < 2 || area.height < 2 || border_type == BorderType::None {
        return;
    }

    let chars = border_type.chars();

    // Top-left corner
    let mut cell = Cell::new(chars.top_left);
    cell.fg = fg;
    cell.bg = bg;
    buffer.set(area.x, area.y, cell);

    // Top border
    for x in 1..(area.width - 1) {
        let mut c = Cell::new(chars.horizontal);
        c.fg = fg;
        c.bg = bg;
        buffer.set(area.x + x, area.y, c);
    }

    // Top-right corner
    let mut cell = Cell::new(chars.top_right);
    cell.fg = fg;
    cell.bg = bg;
    buffer.set(area.x + area.width - 1, area.y, cell);

    // Left and right borders
    for y in 1..(area.height - 1) {
        let mut left = Cell::new(chars.vertical);
        left.fg = fg;
        left.bg = bg;
        buffer.set(area.x, area.y + y, left);

        let mut right = Cell::new(chars.vertical);
        right.fg = fg;
        right.bg = bg;
        buffer.set(area.x + area.width - 1, area.y + y, right);
    }

    // Bottom-left corner
    let mut cell = Cell::new(chars.bottom_left);
    cell.fg = fg;
    cell.bg = bg;
    buffer.set(area.x, area.y + area.height - 1, cell);

    // Bottom border
    for x in 1..(area.width - 1) {
        let mut c = Cell::new(chars.horizontal);
        c.fg = fg;
        c.bg = bg;
        buffer.set(area.x + x, area.y + area.height - 1, c);
    }

    // Bottom-right corner
    let mut cell = Cell::new(chars.bottom_right);
    cell.fg = fg;
    cell.bg = bg;
    buffer.set(area.x + area.width - 1, area.y + area.height - 1, cell);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::widget::Text;

    #[test]
    fn test_border_new() {
        let b = Border::new();
        assert_eq!(b.border_type, BorderType::Single);
        assert!(b.title.is_none());
    }

    #[test]
    fn test_border_types() {
        assert_eq!(Border::single().border_type, BorderType::Single);
        assert_eq!(Border::double().border_type, BorderType::Double);
        assert_eq!(Border::rounded().border_type, BorderType::Rounded);
    }

    #[test]
    fn test_border_render_single() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::single();
        b.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, '┐');
        assert_eq!(buffer.get(0, 4).unwrap().symbol, '└');
        assert_eq!(buffer.get(9, 4).unwrap().symbol, '┘');
        assert_eq!(buffer.get(0, 2).unwrap().symbol, '│');
        assert_eq!(buffer.get(5, 0).unwrap().symbol, '─');
    }

    #[test]
    fn test_border_render_double() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::double();
        b.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '╔');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, '╗');
    }

    #[test]
    fn test_border_with_title() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::single().title("Test");
        b.render(&mut ctx);

        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'T');
        assert_eq!(buffer.get(3, 0).unwrap().symbol, 'e');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, 's');
        assert_eq!(buffer.get(5, 0).unwrap().symbol, 't');
    }

    #[test]
    fn test_border_with_child() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::single().child(Text::new("Hi"));
        b.render(&mut ctx);

        // Child rendered at (1, 1) inside border
        assert_eq!(buffer.get(1, 1).unwrap().symbol, 'H');
        assert_eq!(buffer.get(2, 1).unwrap().symbol, 'i');
    }

    #[test]
    fn test_border_rounded() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::rounded();
        b.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, '╮');
        assert_eq!(buffer.get(0, 4).unwrap().symbol, '╰');
        assert_eq!(buffer.get(9, 4).unwrap().symbol, '╯');
    }
}
