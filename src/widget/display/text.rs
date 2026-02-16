//! Text widget
//!
//! A simple text widget that internally uses RichText for rendering.
//! This ensures consistent text rendering across all widgets.

use super::richtext::{RichText, Style};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
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
    /// Justified text (both edges aligned)
    Justify,
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
    dim: bool,
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
            dim: false,
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

    /// Dim text (reduced intensity/bright)
    pub fn dim(mut self) -> Self {
        self.dim = true;
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
        if self.dim {
            style = style.dim();
        }
        if self.reverse {
            style = style.reverse();
        }

        RichText::new().push(&self.content, style)
    }

    /// Render text with justify alignment (distribute space between words)
    fn render_justified(&self, ctx: &mut RenderContext) {
        use crate::render::{Cell, Modifier};
        use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

        let area = ctx.area;
        let words: Vec<&str> = self.content.split_whitespace().collect();

        // If no words or single word, fall back to left alignment
        if words.len() <= 1 {
            let rich_text = self.to_rich_text_with_ctx(ctx);
            rich_text.render(ctx);
            return;
        }

        // Calculate total text width (without spaces)
        let text_width: usize = words.iter().map(|w| w.width()).sum();
        let available_width = area.width as usize;

        // If text is too wide, fall back to left alignment
        if text_width >= available_width {
            let rich_text = self.to_rich_text_with_ctx(ctx);
            rich_text.render(ctx);
            return;
        }

        // Calculate space distribution
        let total_space = available_width - text_width;
        let gap_count = words.len() - 1;
        let base_space = total_space / gap_count;
        let extra_spaces = total_space % gap_count;

        // Build modifier from style
        let mut modifier = Modifier::empty();
        if self.bold {
            modifier |= Modifier::BOLD;
        }
        if self.italic {
            modifier |= Modifier::ITALIC;
        }
        if self.underline {
            modifier |= Modifier::UNDERLINE;
        }
        if self.dim {
            modifier |= Modifier::DIM;
        }
        if self.reverse {
            modifier |= Modifier::REVERSE;
        }

        // Render words with distributed spacing
        let mut x = area.x;
        for (i, word) in words.iter().enumerate() {
            // Render word
            for ch in word.chars() {
                if x >= area.x + area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = self.fg;
                cell.bg = self.bg;
                cell.modifier = modifier;
                ctx.buffer.set(x, area.y, cell);
                x += UnicodeWidthChar::width(ch).unwrap_or(0) as u16;
            }

            // Add spacing after word (except last word)
            if i < gap_count {
                let spaces = base_space + if i < extra_spaces { 1 } else { 0 };
                x += spaces as u16;
            }
        }
    }
}

impl View for Text {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        // Handle Justify alignment specially
        if self.align == Alignment::Justify {
            self.render_justified(ctx);
            return;
        }

        // Extract CSS colors before creating adjusted context (avoids borrow conflict)
        let rich_text = self.to_rich_text_with_ctx(ctx);

        // Calculate start position based on alignment
        let text_width = unicode_width::UnicodeWidthStr::width(self.content.as_str()) as u16;
        let x_offset = match self.align {
            Alignment::Left | Alignment::Justify => 0,
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

// Tests moved to tests/widget/display/text.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    // KEEP HERE - These tests access private fields and must stay inline
    // Public API tests have been extracted to tests/widget/display/text.rs

    #[test]
    fn test_text_private_initialization() {
        // Test private field initialization that can't be tested via public API
        use super::*;

        let text = Text::new("Test");
        // Test that private fields are properly initialized
        assert_eq!(text.content, "Test");
        assert!(text.fg.is_none());
        assert!(text.bg.is_none());
        assert!(!text.bold);
        assert!(!text.italic);
        assert!(!text.underline);
        assert!(!text.dim);
        assert!(!text.reverse);
    }

    #[test]
    fn test_text_private_builder_patterns() {
        // Test builder pattern implementation on private fields
        use super::*;

        let text = Text::new("Test")
            .fg(Color::RED)
            .bg(Color::BLUE)
            .bold()
            .italic();

        assert_eq!(text.content, "Test");
        assert_eq!(text.fg, Some(Color::RED));
        assert_eq!(text.bg, Some(Color::BLUE));
        assert!(text.bold);
        assert!(text.italic);
    }

    #[test]
    fn test_text_private_alignment() {
        // Test private alignment field that can't be tested via public API
        use super::*;

        let text = Text::new("Test").align(Alignment::Center);
        assert_eq!(text.align, Alignment::Center);
    }

    #[test]
    fn test_text_private_reverse() {
        // Test reverse private field implementation
        use super::*;

        let text = Text::new("Test").reverse();
        assert!(text.reverse);
    }
}
