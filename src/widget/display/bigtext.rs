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

    // ─────────────────────────────────────────────────────────────────────────
    // Test helper getters (doc(hidden))
    // ─────────────────────────────────────────────────────────────────────────

    #[doc(hidden)]
    pub fn get_text(&self) -> &str {
        &self.text
    }

    #[doc(hidden)]
    pub fn get_tier(&self) -> u8 {
        self.tier
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
    pub fn get_figlet_font(&self) -> FigletFont {
        self.figlet_font
    }

    #[doc(hidden)]
    pub fn get_force_figlet(&self) -> bool {
        self.force_figlet
    }

    #[doc(hidden)]
    pub fn get_font_for_tier(&self) -> FigletFont {
        self.font_for_tier()
    }

    #[doc(hidden)]
    pub fn test_render_text_sizing(&self, ctx: &mut RenderContext) {
        self.render_text_sizing(ctx)
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

// Private tests extracted to tests/widget/display/bigtext.rs
// Tests using public APIs should be in tests/widget/display/bigtext.rs
