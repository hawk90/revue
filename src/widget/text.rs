//! Text widget
//!
//! A simple text widget that internally uses RichText for rendering.
//! This ensures consistent text rendering across all widgets.

use super::richtext::{RichText, Style};
use super::traits::{RenderContext, View, WidgetProps};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Text alignment
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum Alignment {
    /// Left-aligned text (default)
    #[default]
    Left,
    /// Center-aligned text
    Center,
    /// Right-aligned text
    Right,
}

/// A text display widget
#[derive(Clone, Debug)]
pub struct Text {
    content: String,
    fg: Option<Color>,
    bg: Option<Color>,
    bold: bool,
    italic: bool,
    underline: bool,
    reverse: bool,
    align: Alignment,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Text {
    /// Create a new text widget
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            underline: false,
            reverse: false,
            align: Alignment::Left,
            props: WidgetProps::new(),
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Preset builders
    // ─────────────────────────────────────────────────────────────────────────

    /// Create a heading (bold white text)
    pub fn heading(content: impl Into<String>) -> Self {
        Self::new(content).bold().fg(Color::WHITE)
    }

    /// Create muted/secondary text (dimmed gray)
    pub fn muted(content: impl Into<String>) -> Self {
        Self::new(content).fg(Color::rgb(128, 128, 128))
    }

    /// Create error text (red)
    pub fn error(content: impl Into<String>) -> Self {
        Self::new(content).fg(Color::RED)
    }

    /// Create success text (green)
    pub fn success(content: impl Into<String>) -> Self {
        Self::new(content).fg(Color::GREEN)
    }

    /// Create warning text (yellow)
    pub fn warning(content: impl Into<String>) -> Self {
        Self::new(content).fg(Color::YELLOW)
    }

    /// Create info text (cyan)
    pub fn info(content: impl Into<String>) -> Self {
        Self::new(content).fg(Color::CYAN)
    }

    /// Create a label (bold)
    pub fn label(content: impl Into<String>) -> Self {
        Self::new(content).bold()
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Builder methods
    // ─────────────────────────────────────────────────────────────────────────

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

    /// Make text bold
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Make text italic
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Underline text
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Reverse video (swap foreground/background colors)
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    /// Set text alignment
    pub fn align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }

    /// Get the text content
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Text {
    /// Convert to RichText for rendering with CSS support
    fn to_rich_text_with_ctx(&self, ctx: &RenderContext) -> RichText {
        let mut style = Style::new();

        // Get foreground color: inline > CSS > none
        let fg = self.fg.or_else(|| {
            ctx.style.and_then(|s| {
                let c = s.visual.color;
                if c != Color::default() {
                    Some(c)
                } else {
                    None
                }
            })
        });
        if let Some(fg) = fg {
            style = style.fg(fg);
        }

        // Get background color: inline > CSS > none
        let bg = self.bg.or_else(|| {
            ctx.style.and_then(|s| {
                let c = s.visual.background;
                if c != Color::default() {
                    Some(c)
                } else {
                    None
                }
            })
        });
        if let Some(bg) = bg {
            style = style.bg(bg);
        }

        if self.bold {
            style = style.bold();
        }
        if self.italic {
            style = style.italic();
        }
        if self.underline {
            style = style.underline();
        }
        if self.reverse {
            style = style.reverse();
        }

        RichText::new().push(&self.content, style)
    }
}

impl View for Text {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        // Extract CSS colors before creating adjusted context (avoids borrow conflict)
        let rich_text = self.to_rich_text_with_ctx(ctx);

        // Calculate start position based on alignment
        let text_width = unicode_width::UnicodeWidthStr::width(self.content.as_str()) as u16;
        let x_offset = match self.align {
            Alignment::Left => 0,
            Alignment::Center => area.width.saturating_sub(text_width) / 2,
            Alignment::Right => area.width.saturating_sub(text_width),
        };

        // Create adjusted context with alignment offset
        let adjusted_area = crate::layout::Rect::new(
            area.x + x_offset,
            area.y,
            area.width.saturating_sub(x_offset),
            area.height,
        );
        let mut adjusted_ctx = RenderContext::new(ctx.buffer, adjusted_area);

        // Delegate to RichText for actual rendering
        rich_text.render(&mut adjusted_ctx);
    }

    crate::impl_view_meta!("Text");
}

impl Default for Text {
    fn default() -> Self {
        Self::new("")
    }
}

impl_styled_view!(Text);
impl_props_builders!(Text);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::StyledView;

    #[test]
    fn test_text_new() {
        let text = Text::new("Hello");
        assert_eq!(text.content(), "Hello");
    }

    #[test]
    fn test_text_builder() {
        let text = Text::new("Test")
            .fg(Color::RED)
            .bold()
            .align(Alignment::Center);

        assert_eq!(text.fg, Some(Color::RED));
        assert!(text.bold);
        assert_eq!(text.align, Alignment::Center);
    }

    #[test]
    fn test_text_render() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let text = Text::new("Hello");
        text.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'e');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, 'o');
    }

    #[test]
    fn test_text_render_centered() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let text = Text::new("Hi").align(Alignment::Center);
        text.render(&mut ctx);

        // "Hi" is 2 chars, in 20 width, centered at (20-2)/2 = 9
        assert_eq!(buffer.get(9, 0).unwrap().symbol, 'H');
        assert_eq!(buffer.get(10, 0).unwrap().symbol, 'i');
    }

    // CSS integration tests
    #[test]
    fn test_text_css_id() {
        use crate::widget::View;

        let text = Text::new("Title").element_id("page-title");
        assert_eq!(View::id(&text), Some("page-title"));

        let meta = text.meta();
        assert_eq!(meta.id, Some("page-title".to_string()));
    }

    #[test]
    fn test_text_css_classes() {
        let text = Text::new("Warning").class("alert").class("bold");

        assert!(text.has_class("alert"));
        assert!(text.has_class("bold"));
        assert!(!text.has_class("hidden"));

        let meta = text.meta();
        assert!(meta.classes.contains("alert"));
        assert!(meta.classes.contains("bold"));
    }

    #[test]
    fn test_text_styled_view() {
        use crate::widget::View;

        let mut text = Text::new("Test");

        text.set_id("test-text");
        assert_eq!(View::id(&text), Some("test-text"));

        text.add_class("highlight");
        assert!(text.has_class("highlight"));

        text.toggle_class("highlight");
        assert!(!text.has_class("highlight"));

        text.toggle_class("active");
        assert!(text.has_class("active"));

        text.remove_class("active");
        assert!(!text.has_class("active"));
    }

    #[test]
    fn test_text_css_colors_from_context() {
        use crate::style::{Style, VisualStyle};

        let text = Text::new("CSS Text");
        let mut buffer = Buffer::new(20, 3);
        let area = Rect::new(0, 0, 20, 1);

        let mut style = Style::default();
        style.visual = VisualStyle {
            color: Color::MAGENTA,
            ..VisualStyle::default()
        };

        let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
        text.render(&mut ctx);
        // Text should use CSS color
    }
}
