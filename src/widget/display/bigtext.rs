//! BigText widget for rendering scaled text in terminal
//!
//! This widget renders text at different sizes using either:
//! - Kitty Text Sizing Protocol (OSC 66) when available
//! - Figlet ASCII art as fallback
//!
//! Primary use case: Rendering markdown headers (H1-H6) with visual size hierarchy.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! // Create a large H1 header
//! let header = BigText::h1("Welcome");
//!
//! // Or specify tier directly
//! let subheader = BigText::new("Subtitle", 2);
//! ```

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::figlet::{figlet_with_font, font_height, FigletFont};
use crate::utils::text_sizing::{is_supported as text_sizing_supported, TextSizing};
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// A widget for rendering scaled text
///
/// BigText renders text at different sizes based on a "tier" (1-6, like HTML headings).
/// It uses Kitty's Text Sizing Protocol when available, falling back to Figlet ASCII art.
pub struct BigText {
    /// The text to render
    text: String,
    /// Heading tier (1 = largest, 6 = smallest)
    tier: u8,
    /// Foreground color
    fg: Option<Color>,
    /// Background color
    bg: Option<Color>,
    /// Figlet font for fallback rendering
    figlet_font: FigletFont,
    /// Force Figlet rendering even if text sizing is available
    force_figlet: bool,
    /// Widget properties
    props: WidgetProps,
}

impl BigText {
    /// Create a new BigText with specified tier
    ///
    /// # Arguments
    /// * `text` - The text to render
    /// * `tier` - Heading level (1-6, where 1 is largest)
    pub fn new(text: impl Into<String>, tier: u8) -> Self {
        Self {
            text: text.into(),
            tier: tier.clamp(1, 6),
            fg: None,
            bg: None,
            figlet_font: FigletFont::Block,
            force_figlet: false,
            props: WidgetProps::new(),
        }
    }

    /// Create an H1-sized text (largest)
    pub fn h1(text: impl Into<String>) -> Self {
        Self::new(text, 1)
    }

    /// Create an H2-sized text
    pub fn h2(text: impl Into<String>) -> Self {
        Self::new(text, 2)
    }

    /// Create an H3-sized text
    pub fn h3(text: impl Into<String>) -> Self {
        Self::new(text, 3)
    }

    /// Create an H4-sized text
    pub fn h4(text: impl Into<String>) -> Self {
        Self::new(text, 4)
    }

    /// Create an H5-sized text
    pub fn h5(text: impl Into<String>) -> Self {
        Self::new(text, 5)
    }

    /// Create an H6-sized text (smallest)
    pub fn h6(text: impl Into<String>) -> Self {
        Self::new(text, 6)
    }

    /// Set the tier (1-6)
    pub fn tier(mut self, tier: u8) -> Self {
        self.tier = tier.clamp(1, 6);
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

    /// Set the Figlet font for fallback rendering
    pub fn figlet_font(mut self, font: FigletFont) -> Self {
        self.figlet_font = font;
        self
    }

    /// Force Figlet rendering even if text sizing is available
    pub fn force_figlet(mut self, force: bool) -> Self {
        self.force_figlet = force;
        self
    }

    /// Get the height in terminal rows this widget will occupy
    pub fn height(&self) -> u16 {
        if !self.force_figlet && text_sizing_supported() {
            TextSizing::height()
        } else {
            self.figlet_height()
        }
    }

    /// Get the Figlet font height for this tier
    fn figlet_height(&self) -> u16 {
        // Use smaller fonts for lower-tier headings
        let font = self.font_for_tier();
        font_height(font) as u16
    }

    /// Get the appropriate Figlet font based on tier
    fn font_for_tier(&self) -> FigletFont {
        match self.tier {
            1 => self.figlet_font,     // H1: use configured font
            2 => FigletFont::Slant,    // H2: slant
            3 => FigletFont::Small,    // H3: small
            4..=6 => FigletFont::Mini, // H4-H6: mini
            _ => FigletFont::Mini,
        }
    }

    /// Render using Figlet ASCII art
    fn render_figlet(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let font = self.font_for_tier();
        let figlet_text = figlet_with_font(&self.text, font);

        let fg = self.fg.unwrap_or(Color::WHITE);
        let modifier = Modifier::BOLD;

        for (row, line) in figlet_text.lines().enumerate() {
            if row as u16 >= area.height {
                break;
            }

            for (col, ch) in line.chars().enumerate() {
                if col as u16 >= area.width {
                    break;
                }

                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                if let Some(bg) = self.bg {
                    cell.bg = Some(bg);
                }
                cell.modifier = modifier;

                ctx.buffer
                    .set(area.x + col as u16, area.y + row as u16, cell);
            }
        }
    }

    /// Render using Text Sizing Protocol (OSC 66)
    fn render_text_sizing(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // Generate the OSC 66 escape sequence
        let seq = TextSizing::escape_sequence(&self.text, self.tier, area.width);

        // Write the sequence to the buffer
        // OSC 66 text occupies 2 rows of height
        let height = TextSizing::height();
        ctx.buffer
            .put_sequence(area.x, area.y, &seq, area.width, height);
    }
}

impl Default for BigText {
    fn default() -> Self {
        Self::new("", 1)
    }
}

impl View for BigText {
    crate::impl_view_meta!("BigText");

    fn render(&self, ctx: &mut RenderContext) {
        if ctx.area.width == 0 || ctx.area.height == 0 {
            return;
        }

        if self.text.is_empty() {
            return;
        }

        if !self.force_figlet && text_sizing_supported() {
            self.render_text_sizing(ctx);
        } else {
            self.render_figlet(ctx);
        }
    }
}

impl_styled_view!(BigText);
impl_props_builders!(BigText);

/// Create a new BigText widget
pub fn bigtext(text: impl Into<String>, tier: u8) -> BigText {
    BigText::new(text, tier)
}

/// Create an H1-sized text
pub fn h1(text: impl Into<String>) -> BigText {
    BigText::h1(text)
}

/// Create an H2-sized text
pub fn h2(text: impl Into<String>) -> BigText {
    BigText::h2(text)
}

/// Create an H3-sized text
pub fn h3(text: impl Into<String>) -> BigText {
    BigText::h3(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_bigtext_creation() {
        let bt = BigText::new("Hello", 1);
        assert_eq!(bt.text, "Hello");
        assert_eq!(bt.tier, 1);
    }

    #[test]
    fn test_tier_clamping() {
        let bt = BigText::new("Test", 10);
        assert_eq!(bt.tier, 6);

        let bt = BigText::new("Test", 0);
        assert_eq!(bt.tier, 1);
    }

    #[test]
    fn test_helper_functions() {
        let h1 = h1("Header 1");
        assert_eq!(h1.tier, 1);

        let h2 = h2("Header 2");
        assert_eq!(h2.tier, 2);

        let h3 = h3("Header 3");
        assert_eq!(h3.tier, 3);
    }

    #[test]
    fn test_builder_pattern() {
        let bt = BigText::h1("Test")
            .fg(Color::CYAN)
            .bg(Color::BLACK)
            .figlet_font(FigletFont::Slant)
            .force_figlet(true);

        assert_eq!(bt.fg, Some(Color::CYAN));
        assert_eq!(bt.bg, Some(Color::BLACK));
        assert_eq!(bt.figlet_font, FigletFont::Slant);
        assert!(bt.force_figlet);
    }

    #[test]
    fn test_render_figlet() {
        let mut buffer = Buffer::new(80, 10);
        let area = Rect::new(0, 0, 80, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bt = BigText::h1("Hi").force_figlet(true);
        bt.render(&mut ctx);

        // Should have rendered something (Figlet art)
        // Check that at least some non-space cells exist
        let mut found_content = false;
        for y in 0..10 {
            for x in 0..80 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol != ' ' {
                        found_content = true;
                        break;
                    }
                }
            }
        }
        assert!(found_content, "Figlet should render some content");
    }

    #[test]
    fn test_font_for_tier() {
        let bt = BigText::h1("Test").figlet_font(FigletFont::Block);
        assert_eq!(bt.font_for_tier(), FigletFont::Block);

        let bt = BigText::h2("Test");
        assert_eq!(bt.font_for_tier(), FigletFont::Slant);

        let bt = BigText::h3("Test");
        assert_eq!(bt.font_for_tier(), FigletFont::Small);

        let bt = BigText::h6("Test");
        assert_eq!(bt.font_for_tier(), FigletFont::Mini);
    }

    #[test]
    fn test_empty_text() {
        let mut buffer = Buffer::new(80, 10);
        let area = Rect::new(0, 0, 80, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bt = BigText::h1("");
        bt.render(&mut ctx);

        // Should not crash, and should not render anything
    }

    #[test]
    fn test_text_sizing_rendering() {
        let mut buffer = Buffer::new(80, 10);
        let area = Rect::new(0, 0, 80, 10);

        // Simulate text sizing support for testing
        // render_text_sizing writes an escape sequence to the buffer
        let bt = BigText::h1("Test");

        // Call render_text_sizing directly (bypasses the is_supported check)
        bt.render_text_sizing(&mut RenderContext::new(&mut buffer, area));

        // Verify that a sequence was registered
        assert_eq!(buffer.sequences().len(), 1);

        // Verify the sequence contains OSC 66 marker
        let seq = &buffer.sequences()[0];
        assert!(seq.contains("\x1b]66;"), "Should contain OSC 66 marker");
        assert!(seq.contains("Test"), "Should contain the text");

        // Verify the first cell has a sequence_id
        let first_cell = buffer.get(0, 0).unwrap();
        assert!(
            first_cell.sequence_id.is_some(),
            "First cell should have sequence_id"
        );

        // Verify continuation cells
        let cont_cell = buffer.get(1, 0).unwrap();
        assert!(
            cont_cell.is_continuation(),
            "Adjacent cells should be continuations"
        );
    }

    #[test]
    fn test_text_sizing_height() {
        let bt = BigText::h1("Test");
        // When text sizing is not supported, height is figlet height
        // When supported, height is TextSizing::height() = 2
        assert!(bt.height() > 0);
    }
}
