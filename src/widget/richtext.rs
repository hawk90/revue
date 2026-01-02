//! Rich text widget with styled spans and hyperlinks
//!
//! Provides rich text rendering with inline styling, similar to Textual's Rich library.
//!
//! # Examples
//!
//! ```ignore
//! use revue::widget::{RichText, Span, Style};
//!
//! // Builder API
//! let text = RichText::new()
//!     .push("Hello ", Style::new().bold())
//!     .push("World", Style::new().fg(Color::GREEN))
//!     .push_link("Click here", "https://example.com");
//!
//! // Markup API
//! let text = RichText::markup("[bold]Hello[/] [green]World[/]");
//! ```

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Text style for spans
#[derive(Clone, Debug, Default)]
pub struct Style {
    /// Foreground color
    pub fg: Option<Color>,
    /// Background color
    pub bg: Option<Color>,
    /// Bold text
    pub bold: bool,
    /// Italic text
    pub italic: bool,
    /// Underlined text
    pub underline: bool,
    /// Dim text
    pub dim: bool,
    /// Strikethrough text
    pub strikethrough: bool,
}

impl Style {
    /// Create a new empty style
    pub fn new() -> Self {
        Self::default()
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

    /// Set bold
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set italic
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Set underline
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Set dim
    pub fn dim(mut self) -> Self {
        self.dim = true;
        self
    }

    /// Set strikethrough
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Preset styles
    // ─────────────────────────────────────────────────────────────────────────

    /// Red foreground
    pub fn red() -> Self {
        Self::new().fg(Color::RED)
    }

    /// Green foreground
    pub fn green() -> Self {
        Self::new().fg(Color::GREEN)
    }

    /// Blue foreground
    pub fn blue() -> Self {
        Self::new().fg(Color::BLUE)
    }

    /// Yellow foreground
    pub fn yellow() -> Self {
        Self::new().fg(Color::YELLOW)
    }

    /// Cyan foreground
    pub fn cyan() -> Self {
        Self::new().fg(Color::CYAN)
    }

    /// Magenta foreground
    pub fn magenta() -> Self {
        Self::new().fg(Color::MAGENTA)
    }

    /// White foreground
    pub fn white() -> Self {
        Self::new().fg(Color::WHITE)
    }

    /// Get modifier flags
    fn to_modifier(&self) -> Modifier {
        let mut m = Modifier::empty();
        if self.bold {
            m |= Modifier::BOLD;
        }
        if self.italic {
            m |= Modifier::ITALIC;
        }
        if self.underline {
            m |= Modifier::UNDERLINE;
        }
        if self.dim {
            m |= Modifier::DIM;
        }
        if self.strikethrough {
            m |= Modifier::CROSSED_OUT;
        }
        m
    }
}

/// A styled text span
#[derive(Clone, Debug)]
pub struct Span {
    /// Text content
    pub text: String,
    /// Style
    pub style: Style,
    /// Optional hyperlink URL
    pub link: Option<String>,
}

impl Span {
    /// Create a new span with text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: Style::default(),
            link: None,
        }
    }

    /// Create a styled span
    pub fn styled(text: impl Into<String>, style: Style) -> Self {
        Self {
            text: text.into(),
            style,
            link: None,
        }
    }

    /// Create a hyperlink span
    pub fn link(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: Style::new().fg(Color::CYAN).underline(),
            link: Some(url.into()),
        }
    }

    /// Set style
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set hyperlink
    pub fn href(mut self, url: impl Into<String>) -> Self {
        self.link = Some(url.into());
        self
    }

    /// Get text width
    pub fn width(&self) -> usize {
        unicode_width::UnicodeWidthStr::width(self.text.as_str())
    }
}

/// Rich text widget with multiple styled spans
pub struct RichText {
    /// Spans
    spans: Vec<Span>,
    /// Default style for unstyled text
    default_style: Style,
    /// Widget props for CSS integration
    props: WidgetProps,
}

impl RichText {
    /// Create a new empty rich text
    pub fn new() -> Self {
        Self {
            spans: Vec::new(),
            default_style: Style::default(),
            props: WidgetProps::new(),
        }
    }

    /// Create from a plain string
    pub fn plain(text: impl Into<String>) -> Self {
        Self::new().push(text, Style::default())
    }

    /// Create from markup string
    ///
    /// Supported tags:
    /// - `[bold]`, `[b]` - Bold text
    /// - `[italic]`, `[i]` - Italic text
    /// - `[underline]`, `[u]` - Underlined text
    /// - `[dim]` - Dimmed text
    /// - `[strike]`, `[s]` - Strikethrough
    /// - `[red]`, `[green]`, `[blue]`, `[yellow]`, `[cyan]`, `[magenta]`, `[white]` - Colors
    /// - `[link=URL]` - Hyperlink
    /// - `[/]` - Reset to default
    ///
    /// Tags can be combined: `[bold red]text[/]`
    pub fn markup(text: &str) -> Self {
        let mut rich = Self::new();
        rich.parse_markup(text);
        rich
    }

    /// Push a styled span
    pub fn push(mut self, text: impl Into<String>, style: Style) -> Self {
        self.spans.push(Span::styled(text, style));
        self
    }

    /// Push a plain text span
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.spans.push(Span::new(text));
        self
    }

    /// Push a hyperlink span
    pub fn push_link(mut self, text: impl Into<String>, url: impl Into<String>) -> Self {
        self.spans.push(Span::link(text, url));
        self
    }

    /// Push a span
    pub fn span(mut self, span: Span) -> Self {
        self.spans.push(span);
        self
    }

    /// Set default style
    pub fn default_style(mut self, style: Style) -> Self {
        self.default_style = style;
        self
    }

    /// Append styled text (mutable version)
    pub fn append(&mut self, text: impl Into<String>, style: Style) {
        self.spans.push(Span::styled(text, style));
    }

    /// Append a hyperlink (mutable version)
    pub fn append_link(&mut self, text: impl Into<String>, url: impl Into<String>) {
        self.spans.push(Span::link(text, url));
    }

    /// Get total width
    pub fn width(&self) -> usize {
        self.spans.iter().map(|s| s.width()).sum()
    }

    /// Get span count
    pub fn len(&self) -> usize {
        self.spans.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }

    /// Clear all spans
    pub fn clear(&mut self) {
        self.spans.clear();
    }

    /// Parse markup string
    fn parse_markup(&mut self, text: &str) {
        let mut current_style = Style::default();
        let mut current_link: Option<String> = None;
        let mut buffer = String::new();
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '[' {
                // Flush buffer with current style
                if !buffer.is_empty() {
                    let mut span = Span::styled(buffer.clone(), current_style.clone());
                    if let Some(ref url) = current_link {
                        span.link = Some(url.clone());
                    }
                    self.spans.push(span);
                    buffer.clear();
                }

                // Parse tag
                let mut tag = String::new();
                while let Some(&c) = chars.peek() {
                    if c == ']' {
                        chars.next();
                        break;
                    }
                    tag.push(chars.next().unwrap());
                }

                // Handle reset tag
                if tag == "/" {
                    current_style = Style::default();
                    current_link = None;
                    continue;
                }

                // Parse tag attributes
                for part in tag.split_whitespace() {
                    if let Some(link) = part.strip_prefix("link=") {
                        current_link = Some(link.to_string());
                        current_style.underline = true;
                        if current_style.fg.is_none() {
                            current_style.fg = Some(Color::CYAN);
                        }
                    } else {
                        match part.to_lowercase().as_str() {
                            "bold" | "b" => current_style.bold = true,
                            "italic" | "i" => current_style.italic = true,
                            "underline" | "u" => current_style.underline = true,
                            "dim" => current_style.dim = true,
                            "strike" | "s" => current_style.strikethrough = true,
                            "red" => current_style.fg = Some(Color::RED),
                            "green" => current_style.fg = Some(Color::GREEN),
                            "blue" => current_style.fg = Some(Color::BLUE),
                            "yellow" => current_style.fg = Some(Color::YELLOW),
                            "cyan" => current_style.fg = Some(Color::CYAN),
                            "magenta" => current_style.fg = Some(Color::MAGENTA),
                            "white" => current_style.fg = Some(Color::WHITE),
                            "black" => current_style.fg = Some(Color::BLACK),
                            // Background colors with "on_" prefix
                            "on_red" => current_style.bg = Some(Color::RED),
                            "on_green" => current_style.bg = Some(Color::GREEN),
                            "on_blue" => current_style.bg = Some(Color::BLUE),
                            "on_yellow" => current_style.bg = Some(Color::YELLOW),
                            "on_cyan" => current_style.bg = Some(Color::CYAN),
                            "on_magenta" => current_style.bg = Some(Color::MAGENTA),
                            "on_white" => current_style.bg = Some(Color::WHITE),
                            "on_black" => current_style.bg = Some(Color::BLACK),
                            _ => {}
                        }
                    }
                }
            } else {
                buffer.push(ch);
            }
        }

        // Flush remaining buffer
        if !buffer.is_empty() {
            let mut span = Span::styled(buffer, current_style);
            if let Some(url) = current_link {
                span.link = Some(url);
            }
            self.spans.push(span);
        }
    }
}

impl Default for RichText {
    fn default() -> Self {
        Self::new()
    }
}

impl View for RichText {
    crate::impl_view_meta!("RichText");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let mut x = area.x;
        let y = area.y;

        for span in &self.spans {
            // Register hyperlink if present
            let hyperlink_id = span
                .link
                .as_ref()
                .map(|url| ctx.buffer.register_hyperlink(url));

            let modifier = span.style.to_modifier();

            for ch in span.text.chars() {
                if x >= area.x + area.width {
                    break;
                }

                let char_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) as u16;

                let mut cell = Cell::new(ch);
                cell.fg = span.style.fg;
                cell.bg = span.style.bg;
                cell.modifier = modifier;
                cell.hyperlink_id = hyperlink_id;

                ctx.buffer.set(x, y, cell);

                // Handle wide characters
                if char_width == 2 && x + 1 < area.x + area.width {
                    let mut cont = Cell::continuation();
                    cont.bg = span.style.bg;
                    cont.hyperlink_id = hyperlink_id;
                    ctx.buffer.set(x + 1, y, cont);
                }

                x += char_width;
            }
        }
    }
}

impl_styled_view!(RichText);
impl_props_builders!(RichText);

// ─────────────────────────────────────────────────────────────────────────────
// Helper functions
// ─────────────────────────────────────────────────────────────────────────────

/// Create a new rich text
pub fn rich_text() -> RichText {
    RichText::new()
}

/// Create rich text from markup
pub fn markup(text: &str) -> RichText {
    RichText::markup(text)
}

/// Create a styled span
pub fn span(text: impl Into<String>) -> Span {
    Span::new(text)
}

/// Create a style
pub fn style() -> Style {
    Style::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_style_builder() {
        let s = Style::new().bold().fg(Color::RED);
        assert!(s.bold);
        assert_eq!(s.fg, Some(Color::RED));
    }

    #[test]
    fn test_span() {
        let span = Span::new("Hello").style(Style::new().bold());
        assert_eq!(span.text, "Hello");
        assert!(span.style.bold);
    }

    #[test]
    fn test_span_link() {
        let span = Span::link("Click", "https://example.com");
        assert_eq!(span.text, "Click");
        assert!(span.link.is_some());
        assert!(span.style.underline);
    }

    #[test]
    fn test_rich_text_builder() {
        let rt = RichText::new()
            .push("Hello ", Style::new().bold())
            .push("World", Style::green());

        assert_eq!(rt.len(), 2);
        assert_eq!(rt.width(), 11);
    }

    #[test]
    fn test_rich_text_markup_bold() {
        let rt = RichText::markup("[bold]Hello[/] World");
        assert_eq!(rt.len(), 2);
        assert!(rt.spans[0].style.bold);
        assert!(!rt.spans[1].style.bold);
    }

    #[test]
    fn test_rich_text_markup_color() {
        let rt = RichText::markup("[red]Error[/]");
        assert_eq!(rt.spans[0].style.fg, Some(Color::RED));
    }

    #[test]
    fn test_rich_text_markup_combined() {
        let rt = RichText::markup("[bold red]Important[/]");
        assert!(rt.spans[0].style.bold);
        assert_eq!(rt.spans[0].style.fg, Some(Color::RED));
    }

    #[test]
    fn test_rich_text_markup_link() {
        let rt = RichText::markup("[link=https://example.com]Click here[/]");
        assert!(rt.spans[0].link.is_some());
        assert_eq!(rt.spans[0].link.as_ref().unwrap(), "https://example.com");
    }

    #[test]
    fn test_rich_text_render() {
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let rt = RichText::new()
            .push("Hello ", Style::new())
            .push("World", Style::new().bold());

        rt.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
        assert_eq!(buffer.get(6, 0).unwrap().symbol, 'W');
        assert!(buffer.get(6, 0).unwrap().modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_rich_text_render_with_hyperlink() {
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let rt = RichText::new().push_link("Click", "https://example.com");

        rt.render(&mut ctx);

        // Check hyperlink was registered
        assert!(buffer.get(0, 0).unwrap().hyperlink_id.is_some());
        assert_eq!(buffer.get_hyperlink(0), Some("https://example.com"));
    }

    #[test]
    fn test_style_presets() {
        assert_eq!(Style::red().fg, Some(Color::RED));
        assert_eq!(Style::green().fg, Some(Color::GREEN));
        assert_eq!(Style::blue().fg, Some(Color::BLUE));
    }
}
