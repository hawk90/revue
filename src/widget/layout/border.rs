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
    /// Minimum width constraint (0 = no constraint)
    min_width: u16,
    /// Minimum height constraint (0 = no constraint)
    min_height: u16,
    /// Maximum width constraint (0 = no constraint)
    max_width: u16,
    /// Maximum height constraint (0 = no constraint)
    max_height: u16,
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
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,
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

    /// Set minimum width constraint
    pub fn min_width(mut self, width: u16) -> Self {
        self.min_width = width;
        self
    }

    /// Set minimum height constraint
    pub fn min_height(mut self, height: u16) -> Self {
        self.min_height = height;
        self
    }

    /// Set maximum width constraint (0 = no limit)
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Set maximum height constraint (0 = no limit)
    pub fn max_height(mut self, height: u16) -> Self {
        self.max_height = height;
        self
    }

    /// Set both min width and height
    pub fn min_size(self, width: u16, height: u16) -> Self {
        self.min_width(width).min_height(height)
    }

    /// Set both max width and height (0 = no limit)
    pub fn max_size(self, width: u16, height: u16) -> Self {
        self.max_width(width).max_height(height)
    }

    /// Set all size constraints at once
    pub fn constrain(self, min_w: u16, min_h: u16, max_w: u16, max_h: u16) -> Self {
        self.min_width(min_w)
            .min_height(min_h)
            .max_width(max_w)
            .max_height(max_h)
    }

    /// Apply size constraints to the available area
    fn apply_constraints(&self, area: Rect) -> Rect {
        let eff_max_w = if self.max_width > 0 {
            self.max_width.max(self.min_width)
        } else {
            u16::MAX
        };
        let eff_max_h = if self.max_height > 0 {
            self.max_height.max(self.min_height)
        } else {
            u16::MAX
        };
        let width = area.width.clamp(self.min_width, eff_max_w);
        let height = area.height.clamp(self.min_height, eff_max_h);

        Rect::new(area.x, area.y, width, height)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Getter methods (for testing)
    // ─────────────────────────────────────────────────────────────────────────

    #[doc(hidden)]
    pub fn get_border_type(&self) -> BorderType {
        self.border_type
    }

    #[doc(hidden)]
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    #[doc(hidden)]
    pub fn get_fg(&self) -> Option<Color> {
        self.fg
    }

    #[doc(hidden)]
    pub fn get_bg(&self) -> Option<Color> {
        self.bg
    }

    #[doc(hidden)]
    pub fn has_child(&self) -> bool {
        self.child.is_some()
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
        let area = self.apply_constraints(ctx.area);
        if area.width < 2 || area.height < 2 {
            return;
        }

        let chars = self.border_type.chars();

        // Top border
        let mut cell = Cell::new(chars.top_left);
        cell.fg = self.fg;
        cell.bg = self.bg;
        ctx.set(0, 0, cell);

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
                ctx.set(x, 0, c);
            }

            // Draw title
            for (i, ch) in display_title.chars().enumerate() {
                let mut c = Cell::new(ch);
                c.fg = self.fg;
                c.bg = self.bg;
                ctx.set(2 + i as u16, 0, c);
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
            ctx.set(x, 0, c);
        }

        // Top right corner
        let mut cell = Cell::new(chars.top_right);
        cell.fg = self.fg;
        cell.bg = self.bg;
        ctx.set(area.width - 1, 0, cell);

        // Left and right borders
        for y in 1..(area.height - 1) {
            let mut left = Cell::new(chars.vertical);
            left.fg = self.fg;
            left.bg = self.bg;
            ctx.set(0, y, left);

            let mut right = Cell::new(chars.vertical);
            right.fg = self.fg;
            right.bg = self.bg;
            ctx.set(area.width - 1, y, right);
        }

        // Bottom border
        let mut cell = Cell::new(chars.bottom_left);
        cell.fg = self.fg;
        cell.bg = self.bg;
        ctx.set(0, area.height - 1, cell);

        for x in 1..(area.width - 1) {
            let mut c = Cell::new(chars.horizontal);
            c.fg = self.fg;
            c.bg = self.bg;
            ctx.set(x, area.height - 1, c);
        }

        let mut cell = Cell::new(chars.bottom_right);
        cell.fg = self.fg;
        cell.bg = self.bg;
        ctx.set(area.width - 1, area.height - 1, cell);

        // Render child in inner area, respecting overflow style
        if let Some(ref child) = self.child {
            let overflow_hidden = ctx.css_overflow_hidden();
            let parent_clip = ctx.clip();
            let inner = ctx.sub_area(
                1,
                1,
                area.width.saturating_sub(2),
                area.height.saturating_sub(2),
            );
            let mut child_ctx = RenderContext::child_ctx_with_overflow(
                ctx.buffer,
                inner,
                overflow_hidden,
                parent_clip,
            );
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
        assert_eq!(b.get_border_type(), BorderType::Single);
        assert!(b.get_title().is_none());
        assert!(!b.has_child());
    }

    #[test]
    fn test_border_builder() {
        let b = Border::new()
            .border_type(BorderType::Double)
            .title("Title")
            .fg(Color::RED)
            .bg(Color::BLACK)
            .child(Text::new("Content"));
        assert_eq!(b.get_border_type(), BorderType::Double);
        assert_eq!(b.get_title(), Some("Title"));
        assert_eq!(b.get_fg(), Some(Color::RED));
        assert_eq!(b.get_bg(), Some(Color::BLACK));
        assert!(b.has_child());
    }

    #[test]
    fn test_border_presets() {
        assert_eq!(Border::single().get_border_type(), BorderType::Single);
        assert_eq!(Border::double().get_border_type(), BorderType::Double);
        assert_eq!(Border::rounded().get_border_type(), BorderType::Rounded);
        assert_eq!(Border::thick().get_border_type(), BorderType::Thick);
        assert_eq!(Border::ascii().get_border_type(), BorderType::Ascii);
        assert_eq!(Border::panel().get_border_type(), BorderType::Double);
        assert_eq!(Border::card().get_border_type(), BorderType::Rounded);
        assert_eq!(Border::error_box().get_fg(), Some(Color::RED));
        assert_eq!(Border::success_box().get_fg(), Some(Color::GREEN));
    }

    #[test]
    fn test_border_type_chars() {
        let chars = BorderType::Single.chars();
        assert_eq!(chars.top_left, '┌');
        assert_eq!(chars.horizontal, '─');

        let chars = BorderType::Rounded.chars();
        assert_eq!(chars.top_left, '╭');

        let chars = BorderType::None.chars();
        assert_eq!(chars.top_left, ' ');
    }

    #[test]
    fn test_border_render_single() {
        let mut buf = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buf, area);
        let b = Border::new();
        b.render(&mut ctx);
        assert_eq!(buf.get(0, 0).unwrap().symbol, '┌');
        assert_eq!(buf.get(9, 0).unwrap().symbol, '┐');
        assert_eq!(buf.get(0, 4).unwrap().symbol, '└');
        assert_eq!(buf.get(9, 4).unwrap().symbol, '┘');
    }

    #[test]
    fn test_border_render_with_child() {
        let mut buf = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buf, area);
        let b = Border::new().child(Text::new("Hi"));
        b.render(&mut ctx);
        // Border should be drawn
        assert_eq!(buf.get(0, 0).unwrap().symbol, '┌');
        // Child content inside border
        assert_eq!(buf.get(1, 1).unwrap().symbol, 'H');
        assert_eq!(buf.get(2, 1).unwrap().symbol, 'i');
    }

    #[test]
    fn test_border_render_small_area_no_panic() {
        let mut buf = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 1, 1);
        let mut ctx = RenderContext::new(&mut buf, area);
        let b = Border::new().child(Text::new("X"));
        b.render(&mut ctx); // Width/height < 2, should return early
    }

    #[test]
    fn test_draw_border_utility() {
        let mut buf = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        draw_border(&mut buf, area, BorderType::Double, Some(Color::CYAN), None);
        assert_eq!(buf.get(0, 0).unwrap().symbol, '╔');
        assert_eq!(buf.get(9, 0).unwrap().symbol, '╗');
    }

    #[test]
    fn test_draw_border_none_type() {
        let mut buf = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        draw_border(&mut buf, area, BorderType::None, None, None);
        // BorderType::None should not draw anything
        assert_eq!(buf.get(0, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_border_default() {
        let b = Border::default();
        assert_eq!(b.get_border_type(), BorderType::Single);
    }

    #[test]
    fn test_border_helper_fn() {
        let b = border();
        assert_eq!(b.get_border_type(), BorderType::Single);
    }
}
