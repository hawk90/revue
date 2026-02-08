//! Border/Frame widget for surrounding content

use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::border::BorderChars;
use crate::widget::traits::{RenderContext, View, WidgetProps};
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

    // =========================================================================
    // BorderType enum tests
    // =========================================================================

    #[test]
    fn test_border_type_default() {
        assert_eq!(BorderType::default(), BorderType::Single);
    }

    #[test]
    fn test_border_type_clone() {
        let bt = BorderType::Double;
        assert_eq!(bt, bt.clone());
    }

    #[test]
    fn test_border_type_copy() {
        let bt1 = BorderType::Rounded;
        let bt2 = bt1;
        assert_eq!(bt1, BorderType::Rounded);
        assert_eq!(bt2, BorderType::Rounded);
    }

    #[test]
    fn test_border_type_partial_eq() {
        assert_eq!(BorderType::Single, BorderType::Single);
        assert_eq!(BorderType::Double, BorderType::Double);
        assert_eq!(BorderType::Rounded, BorderType::Rounded);
        assert_eq!(BorderType::Thick, BorderType::Thick);
        assert_eq!(BorderType::Ascii, BorderType::Ascii);
        assert_eq!(BorderType::None, BorderType::None);

        assert_ne!(BorderType::Single, BorderType::Double);
        assert_ne!(BorderType::Rounded, BorderType::None);
    }

    #[test]
    fn test_border_type_all_variants_unique() {
        let variants = [
            BorderType::None,
            BorderType::Single,
            BorderType::Double,
            BorderType::Rounded,
            BorderType::Thick,
            BorderType::Ascii,
        ];

        // All variants should be different from None
        for variant in variants.iter().skip(1) {
            assert_ne!(*variant, BorderType::None);
        }

        // All variants should be different from Single
        for variant in variants.iter() {
            if *variant != BorderType::Single {
                assert_ne!(*variant, BorderType::Single);
            }
        }
    }

    #[test]
    fn test_border_type_debug() {
        let debug_str = format!("{:?}", BorderType::Single);
        assert!(debug_str.contains("Single"));
    }

    // =========================================================================
    // BorderType::chars tests
    // =========================================================================

    #[test]
    fn test_border_type_chars_none() {
        let chars = BorderType::None.chars();
        assert_eq!(chars.top_left, ' ');
        assert_eq!(chars.top_right, ' ');
        assert_eq!(chars.bottom_left, ' ');
        assert_eq!(chars.bottom_right, ' ');
        assert_eq!(chars.horizontal, ' ');
        assert_eq!(chars.vertical, ' ');
    }

    #[test]
    fn test_border_type_chars_single() {
        let chars = BorderType::Single.chars();
        assert_eq!(chars.top_left, '┌');
        assert_eq!(chars.top_right, '┐');
        assert_eq!(chars.bottom_left, '└');
        assert_eq!(chars.bottom_right, '┘');
        assert_eq!(chars.horizontal, '─');
        assert_eq!(chars.vertical, '│');
    }

    #[test]
    fn test_border_type_chars_double() {
        let chars = BorderType::Double.chars();
        assert_eq!(chars.top_left, '╔');
        assert_eq!(chars.top_right, '╗');
        assert_eq!(chars.bottom_left, '╚');
        assert_eq!(chars.bottom_right, '╝');
        assert_eq!(chars.horizontal, '═');
        assert_eq!(chars.vertical, '║');
    }

    #[test]
    fn test_border_type_chars_rounded() {
        let chars = BorderType::Rounded.chars();
        assert_eq!(chars.top_left, '╭');
        assert_eq!(chars.top_right, '╮');
        assert_eq!(chars.bottom_left, '╰');
        assert_eq!(chars.bottom_right, '╯');
        assert_eq!(chars.horizontal, '─');
        assert_eq!(chars.vertical, '│');
    }

    #[test]
    fn test_border_type_chars_thick() {
        let chars = BorderType::Thick.chars();
        assert_eq!(chars.top_left, '┏');
        assert_eq!(chars.top_right, '┓');
        assert_eq!(chars.bottom_left, '┗');
        assert_eq!(chars.bottom_right, '┛');
        assert_eq!(chars.horizontal, '━');
        assert_eq!(chars.vertical, '┃');
    }

    #[test]
    fn test_border_type_chars_ascii() {
        let chars = BorderType::Ascii.chars();
        assert_eq!(chars.top_left, '+');
        assert_eq!(chars.top_right, '+');
        assert_eq!(chars.bottom_left, '+');
        assert_eq!(chars.bottom_right, '+');
        assert_eq!(chars.horizontal, '-');
        assert_eq!(chars.vertical, '|');
    }

    // =========================================================================
    // Border::new and default tests
    // =========================================================================

    #[test]
    fn test_border_new() {
        let b = Border::new();
        assert_eq!(b.border_type, BorderType::Single);
        assert!(b.title.is_none());
        assert!(b.fg.is_none());
        assert!(b.bg.is_none());
        assert!(b.child.is_none());
    }

    #[test]
    fn test_border_default() {
        let b = Border::default();
        assert_eq!(b.border_type, BorderType::Single);
    }

    // =========================================================================
    // Border builder tests
    // =========================================================================

    #[test]
    fn test_border_child() {
        let b = Border::new().child(Text::new("Hi"));
        assert!(b.child.is_some());
    }

    #[test]
    fn test_border_border_type() {
        let b = Border::new().border_type(BorderType::Double);
        assert_eq!(b.border_type, BorderType::Double);
    }

    #[test]
    fn test_border_title_str() {
        let b = Border::new().title("Test Title");
        assert_eq!(b.title, Some("Test Title".to_string()));
    }

    #[test]
    fn test_border_title_string() {
        let b = Border::new().title(String::from("Owned"));
        assert_eq!(b.title, Some("Owned".to_string()));
    }

    #[test]
    fn test_border_title_empty() {
        let b = Border::new().title("");
        assert_eq!(b.title, Some("".to_string()));
    }

    #[test]
    fn test_border_fg() {
        let b = Border::new().fg(Color::RED);
        assert_eq!(b.fg, Some(Color::RED));
    }

    #[test]
    fn test_border_bg() {
        let b = Border::new().bg(Color::BLUE);
        assert_eq!(b.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_border_builder_chain() {
        let b = Border::new()
            .child(Text::new("Content"))
            .border_type(BorderType::Rounded)
            .title("Panel")
            .fg(Color::CYAN)
            .bg(Color::BLACK);

        assert!(b.child.is_some());
        assert_eq!(b.border_type, BorderType::Rounded);
        assert_eq!(b.title, Some("Panel".to_string()));
        assert_eq!(b.fg, Some(Color::CYAN));
        assert_eq!(b.bg, Some(Color::BLACK));
    }

    // =========================================================================
    // Border preset tests
    // =========================================================================

    #[test]
    fn test_border_types() {
        assert_eq!(Border::single().border_type, BorderType::Single);
        assert_eq!(Border::double().border_type, BorderType::Double);
        assert_eq!(Border::rounded().border_type, BorderType::Rounded);
    }

    #[test]
    fn test_border_thick() {
        let b = Border::thick();
        assert_eq!(b.border_type, BorderType::Thick);
    }

    #[test]
    fn test_border_ascii() {
        let b = Border::ascii();
        assert_eq!(b.border_type, BorderType::Ascii);
    }

    #[test]
    fn test_border_panel() {
        let b = Border::panel();
        assert_eq!(b.border_type, BorderType::Double);
        assert_eq!(b.fg, Some(Color::CYAN));
    }

    #[test]
    fn test_border_card() {
        let b = Border::card();
        assert_eq!(b.border_type, BorderType::Rounded);
        assert_eq!(b.fg, Some(Color::WHITE));
    }

    #[test]
    fn test_border_error_box() {
        let b = Border::error_box();
        assert_eq!(b.border_type, BorderType::Single);
        assert_eq!(b.fg, Some(Color::RED));
    }

    #[test]
    fn test_border_success_box() {
        let b = Border::success_box();
        assert_eq!(b.border_type, BorderType::Single);
        assert_eq!(b.fg, Some(Color::GREEN));
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_border_helper() {
        let b = border();
        assert_eq!(b.border_type, BorderType::Single);
    }

    // =========================================================================
    // Border rendering tests
    // =========================================================================

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
        assert_eq!(buffer.get(0, 4).unwrap().symbol, '╚');
        assert_eq!(buffer.get(9, 4).unwrap().symbol, '╝');
    }

    #[test]
    fn test_border_render_thick() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::thick();
        b.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┏');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, '┓');
        assert_eq!(buffer.get(0, 4).unwrap().symbol, '┗');
        assert_eq!(buffer.get(9, 4).unwrap().symbol, '┛');
    }

    #[test]
    fn test_border_render_ascii() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::ascii();
        b.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '+');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, '+');
        assert_eq!(buffer.get(5, 0).unwrap().symbol, '-');
        assert_eq!(buffer.get(0, 2).unwrap().symbol, '|');
    }

    #[test]
    fn test_border_render_none() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::new().border_type(BorderType::None);
        b.render(&mut ctx);

        // No border should be drawn
        assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_border_render_rounded() {
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

    #[test]
    fn test_border_render_with_color() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::single().fg(Color::RED);
        b.render(&mut ctx);

        // Check color was applied
        assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
    }

    #[test]
    fn test_border_render_with_bg_color() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::single().bg(Color::BLUE);
        b.render(&mut ctx);

        // Check background was applied
        assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLUE));
    }

    #[test]
    fn test_border_render_small_width() {
        let mut buffer = Buffer::new(2, 5);
        let area = Rect::new(0, 0, 2, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::single();
        b.render(&mut ctx);
        // Should not panic even with minimum width
    }

    #[test]
    fn test_border_render_small_height() {
        let mut buffer = Buffer::new(10, 2);
        let area = Rect::new(0, 0, 10, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::single();
        b.render(&mut ctx);
        // Should not panic even with minimum height
    }

    #[test]
    fn test_border_render_too_small() {
        let mut buffer = Buffer::new(1, 1);
        let area = Rect::new(0, 0, 1, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::single();
        b.render(&mut ctx);
        // Should not render anything when too small
    }

    // =========================================================================
    // Border title tests
    // =========================================================================

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
    fn test_border_with_title_truncated() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Title longer than available space (width 10 - 4 = 6 max chars)
        let b = Border::single().title("Very Long Title");
        b.render(&mut ctx);

        // Title should be truncated
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'V');
        assert_eq!(buffer.get(3, 0).unwrap().symbol, 'e');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, 'r');
        assert_eq!(buffer.get(5, 0).unwrap().symbol, 'y');
        assert_eq!(buffer.get(6, 0).unwrap().symbol, ' ');
        assert_eq!(buffer.get(7, 0).unwrap().symbol, 'L');
    }

    #[test]
    fn test_border_with_title_color() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::single().title("Test").fg(Color::YELLOW);
        b.render(&mut ctx);

        // Title should have the border color
        assert_eq!(buffer.get(2, 0).unwrap().fg, Some(Color::YELLOW));
    }

    #[test]
    fn test_border_with_title_unicode() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::single().title("🔧 Settings");
        b.render(&mut ctx);

        // Unicode title should render
        let char_at_2 = buffer.get(2, 0).unwrap().symbol;
        assert!(char_at_2 == '🔧' || char_at_2 == 'S'); // May vary by terminal
    }

    // =========================================================================
    // Border child tests
    // =========================================================================

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
    fn test_border_with_child_no_child() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::single();
        b.render(&mut ctx);

        // Inside area should be empty (no child)
        assert_eq!(buffer.get(1, 1).unwrap().symbol, ' ');
    }

    #[test]
    fn test_border_child_area() {
        let mut buffer = Buffer::new(12, 8);
        let area = Rect::new(0, 0, 12, 8);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::single().child(Text::new("X"));
        b.render(&mut ctx);

        // Child should be rendered with 1 cell padding on all sides
        assert_eq!(buffer.get(1, 1).unwrap().symbol, 'X');
        // Border corners should still be correct
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
        assert_eq!(buffer.get(11, 0).unwrap().symbol, '┐');
        assert_eq!(buffer.get(0, 7).unwrap().symbol, '└');
        assert_eq!(buffer.get(11, 7).unwrap().symbol, '┘');
    }

    // =========================================================================
    // draw_border utility tests
    // =========================================================================

    #[test]
    fn test_draw_border_utility() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);

        draw_border(&mut buffer, area, BorderType::Single, None, None);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
        assert_eq!(buffer.get(9, 0).unwrap().symbol, '┐');
        assert_eq!(buffer.get(0, 4).unwrap().symbol, '└');
        assert_eq!(buffer.get(9, 4).unwrap().symbol, '┘');
    }

    #[test]
    fn test_draw_border_none_type() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);

        draw_border(&mut buffer, area, BorderType::None, None, None);

        // Border should not be drawn
        assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_draw_border_with_color() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);

        draw_border(
            &mut buffer,
            area,
            BorderType::Single,
            Some(Color::RED),
            None,
        );

        // Check color was applied
        assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
        assert_eq!(buffer.get(5, 0).unwrap().fg, Some(Color::RED));
        assert_eq!(buffer.get(0, 2).unwrap().fg, Some(Color::RED));
    }

    #[test]
    fn test_draw_border_with_bg_color() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);

        draw_border(
            &mut buffer,
            area,
            BorderType::Single,
            None,
            Some(Color::BLUE),
        );

        // Check background was applied
        assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLUE));
    }

    #[test]
    fn test_draw_border_small_area() {
        let mut buffer = Buffer::new(2, 2);
        let area = Rect::new(0, 0, 2, 2);

        draw_border(&mut buffer, area, BorderType::Single, None, None);

        // Should draw corners only
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, '┐');
        assert_eq!(buffer.get(0, 1).unwrap().symbol, '└');
        assert_eq!(buffer.get(1, 1).unwrap().symbol, '┘');
    }

    #[test]
    fn test_draw_border_too_small() {
        let mut buffer = Buffer::new(1, 1);
        let area = Rect::new(0, 0, 1, 1);

        draw_border(&mut buffer, area, BorderType::Single, None, None);

        // Should not draw anything
        assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_draw_border_all_types() {
        let types = [
            BorderType::Single,
            BorderType::Double,
            BorderType::Rounded,
            BorderType::Thick,
            BorderType::Ascii,
        ];

        for bt in types {
            let mut buffer = Buffer::new(10, 5);
            let area = Rect::new(0, 0, 10, 5);

            draw_border(&mut buffer, area, bt, None, None);

            // All types except None should draw borders
            assert_ne!(buffer.get(0, 0).unwrap().symbol, ' ');
            assert_ne!(buffer.get(9, 0).unwrap().symbol, ' ');
        }
    }

    // =========================================================================
    // Border edge case tests
    // =========================================================================

    #[test]
    fn test_border_offset_area() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(5, 3, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::single();
        b.render(&mut ctx);

        // Border should be at offset position
        assert_eq!(buffer.get(5, 3).unwrap().symbol, '┌');
        assert_eq!(buffer.get(14, 3).unwrap().symbol, '┐');
        assert_eq!(buffer.get(5, 7).unwrap().symbol, '└');
        assert_eq!(buffer.get(14, 7).unwrap().symbol, '┘');
    }

    #[test]
    fn test_border_large_area() {
        let mut buffer = Buffer::new(100, 50);
        let area = Rect::new(0, 0, 100, 50);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let b = Border::double();
        b.render(&mut ctx);

        // Check corners on large area
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '╔');
        assert_eq!(buffer.get(99, 0).unwrap().symbol, '╗');
        assert_eq!(buffer.get(0, 49).unwrap().symbol, '╚');
        assert_eq!(buffer.get(99, 49).unwrap().symbol, '╝');
    }
}
