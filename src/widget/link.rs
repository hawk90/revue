//! Clickable link widget
//!
//! Displays hyperlinks that can be opened in the default browser.
//! Supports OSC 8 terminal hyperlinks for modern terminals.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Link, link};
//!
//! // Simple link
//! let github = Link::new("https://github.com/anthropics/claude-code");
//!
//! // Link with custom text
//! let docs = Link::new("https://docs.rs/revue")
//!     .text("Documentation")
//!     .fg(Color::CYAN);
//!
//! // Using helper function
//! let home = link("https://example.com", "Home Page");
//! ```

use crate::style::Color;
use crate::widget::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

#[cfg(not(target_arch = "wasm32"))]
use crate::utils::browser;

/// Link style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LinkStyle {
    /// Underlined text (default)
    #[default]
    Underline,
    /// Bracketed [link]
    Bracketed,
    /// With arrow: link →
    Arrow,
    /// With icon: 🔗 link
    Icon,
    /// Plain text (no decoration)
    Plain,
}

/// Clickable link widget
#[derive(Clone, Debug)]
pub struct Link {
    /// URL to open
    url: String,
    /// Display text (defaults to URL)
    text: Option<String>,
    /// Link style
    style: LinkStyle,
    /// Foreground color
    fg: Option<Color>,
    /// Background color
    bg: Option<Color>,
    /// Whether link is focused
    focused: bool,
    /// Whether link is disabled
    disabled: bool,
    /// Tooltip text
    tooltip: Option<String>,
    /// Use OSC 8 hyperlinks (terminal-dependent)
    osc8: bool,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Link {
    /// Create a new link with URL
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            text: None,
            style: LinkStyle::default(),
            fg: None,
            bg: None,
            focused: false,
            disabled: false,
            tooltip: None,
            osc8: true, // Enable by default
            props: WidgetProps::new(),
        }
    }

    /// Create a link with custom display text
    pub fn with_text(url: impl Into<String>, text: impl Into<String>) -> Self {
        Self::new(url).text(text)
    }

    /// Set display text
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Set link style
    pub fn style(mut self, style: LinkStyle) -> Self {
        self.style = style;
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

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set tooltip
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Enable/disable OSC 8 hyperlinks
    pub fn osc8(mut self, enabled: bool) -> Self {
        self.osc8 = enabled;
        self
    }

    /// Get the URL
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Get display text
    pub fn display_text(&self) -> &str {
        self.text.as_deref().unwrap_or(&self.url)
    }

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Check if disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Open the link in the system default browser
    ///
    /// Does nothing if the link is disabled.
    ///
    /// # Errors
    ///
    /// Returns `Err(BrowserError)` if:
    /// - The URL contains dangerous characters
    /// - The URL format is invalid
    /// - The browser cannot be opened
    ///
    /// See [`crate::utils::browser::open_url`] for details.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open(&self) -> Result<(), browser::BrowserError> {
        if self.disabled {
            return Ok(());
        }
        browser::open_url(&self.url)
    }

    /// Format display text with style
    fn format_display(&self) -> String {
        let text = self.display_text();

        match self.style {
            LinkStyle::Underline => text.to_string(),
            LinkStyle::Bracketed => format!("[{}]", text),
            LinkStyle::Arrow => format!("{} →", text),
            LinkStyle::Icon => format!("🔗 {}", text),
            LinkStyle::Plain => text.to_string(),
        }
    }

    /// Generate OSC 8 hyperlink escape sequence
    fn osc8_start(&self) -> String {
        if self.osc8 && !self.disabled {
            format!("\x1b]8;;{}\x1b\\", self.url)
        } else {
            String::new()
        }
    }

    fn osc8_end(&self) -> String {
        if self.osc8 && !self.disabled {
            "\x1b]8;;\x1b\\".to_string()
        } else {
            String::new()
        }
    }
}

impl View for Link {
    crate::impl_view_meta!("Link");

    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::Text;

        let display = self.format_display();

        // Build the text with OSC 8 sequences
        let content = if self.osc8 {
            format!("{}{}{}", self.osc8_start(), display, self.osc8_end())
        } else {
            display
        };

        let mut text = Text::new(content);

        // Apply colors
        let fg = if self.disabled {
            Some(Color::rgb(128, 128, 128))
        } else if self.focused {
            Some(Color::rgb(100, 200, 255))
        } else {
            self.fg.or(Some(Color::CYAN))
        };

        if let Some(fg) = fg {
            text = text.fg(fg);
        }

        if let Some(bg) = self.bg {
            text = text.bg(bg);
        }

        // Apply underline for underline style
        if self.style == LinkStyle::Underline && !self.disabled {
            text = text.underline();
        }

        text.render(ctx);
    }
}

impl_styled_view!(Link);
impl_props_builders!(Link);

/// Create a link
pub fn link(url: impl Into<String>, text: impl Into<String>) -> Link {
    Link::with_text(url, text)
}

/// Create a link with just URL
pub fn url_link(url: impl Into<String>) -> Link {
    Link::new(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_new() {
        let link = Link::new("https://example.com");
        assert_eq!(link.url(), "https://example.com");
        assert_eq!(link.display_text(), "https://example.com");
    }

    #[test]
    fn test_link_with_text() {
        let link = Link::with_text("https://example.com", "Example");
        assert_eq!(link.url(), "https://example.com");
        assert_eq!(link.display_text(), "Example");
    }

    #[test]
    fn test_link_style() {
        let link = Link::new("https://example.com").text("test");

        assert_eq!(
            link.clone().style(LinkStyle::Bracketed).format_display(),
            "[test]"
        );
        assert_eq!(
            link.clone().style(LinkStyle::Arrow).format_display(),
            "test →"
        );
        assert_eq!(
            link.clone().style(LinkStyle::Icon).format_display(),
            "🔗 test"
        );
    }

    #[test]
    fn test_link_focused() {
        let link = Link::new("https://example.com").focused(true);
        assert!(link.is_focused());
    }

    #[test]
    fn test_link_disabled() {
        let link = Link::new("https://example.com").disabled(true);
        assert!(link.is_disabled());
    }

    #[test]
    fn test_link_osc8() {
        let link = Link::new("https://example.com").osc8(true);
        let start = link.osc8_start();
        assert!(start.contains("https://example.com"));
        assert!(start.starts_with("\x1b]8;;"));
    }

    #[test]
    fn test_link_osc8_disabled_link() {
        let link = Link::new("https://example.com").disabled(true);
        assert!(link.osc8_start().is_empty());
    }

    #[test]
    fn test_helper_functions() {
        let l = link("https://example.com", "Example");
        assert_eq!(l.display_text(), "Example");

        let u = url_link("https://example.com");
        assert_eq!(u.display_text(), "https://example.com");
    }

    #[test]
    fn test_link_tooltip() {
        let link = Link::new("https://example.com").tooltip("Click to visit");
        assert_eq!(link.tooltip, Some("Click to visit".to_string()));
    }

    // =========================================================================
    // LinkStyle enum tests
    // =========================================================================

    #[test]
    fn test_link_style_default() {
        let style = LinkStyle::default();
        assert_eq!(style, LinkStyle::Underline);
    }

    #[test]
    fn test_link_style_clone() {
        let style = LinkStyle::Arrow;
        let cloned = style;
        assert_eq!(style, cloned);
    }

    #[test]
    fn test_link_style_copy() {
        let style1 = LinkStyle::Icon;
        let style2 = style1;
        assert_eq!(style1, LinkStyle::Icon);
        assert_eq!(style2, LinkStyle::Icon);
    }

    #[test]
    fn test_link_style_partial_eq() {
        assert_eq!(LinkStyle::Underline, LinkStyle::Underline);
        assert_ne!(LinkStyle::Underline, LinkStyle::Bracketed);
    }

    #[test]
    fn test_link_style_format_underline() {
        let link = Link::new("url").text("test").style(LinkStyle::Underline);
        assert_eq!(link.format_display(), "test");
    }

    #[test]
    fn test_link_style_format_plain() {
        let link = Link::new("url").text("test").style(LinkStyle::Plain);
        assert_eq!(link.format_display(), "test");
    }

    // =========================================================================
    // Link::text builder tests
    // =========================================================================

    #[test]
    fn test_link_text_builder() {
        let link = Link::new("https://example.com").text("Custom Text");
        assert_eq!(link.display_text(), "Custom Text");
        assert_eq!(link.url, "https://example.com");
    }

    #[test]
    fn test_link_text_overrides() {
        let link = Link::with_text("https://example.com", "First").text("Second");
        assert_eq!(link.display_text(), "Second");
    }

    // =========================================================================
    // Link::fg builder tests
    // =========================================================================

    #[test]
    fn test_link_fg() {
        let link = Link::new("url").fg(Color::RED);
        assert_eq!(link.fg, Some(Color::RED));
    }

    #[test]
    fn test_link_fg_none() {
        let link = Link::new("url");
        assert!(link.fg.is_none());
    }

    // =========================================================================
    // Link::bg builder tests
    // =========================================================================

    #[test]
    fn test_link_bg() {
        let link = Link::new("url").bg(Color::BLUE);
        assert_eq!(link.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_link_bg_none() {
        let link = Link::new("url");
        assert!(link.bg.is_none());
    }

    // =========================================================================
    // Link colors combined tests
    // =========================================================================

    #[test]
    fn test_link_colors_combined() {
        let link = Link::new("url").fg(Color::GREEN).bg(Color::BLACK);
        assert_eq!(link.fg, Some(Color::GREEN));
        assert_eq!(link.bg, Some(Color::BLACK));
    }

    // =========================================================================
    // Link::focused builder tests
    // =========================================================================

    #[test]
    fn test_link_focused_builder() {
        let link = Link::new("url").focused(true);
        assert!(link.is_focused());
    }

    #[test]
    fn test_link_not_focused() {
        let link = Link::new("url").focused(false);
        assert!(!link.is_focused());
    }

    // =========================================================================
    // Link::disabled builder tests
    // =========================================================================

    #[test]
    fn test_link_disabled_builder() {
        let link = Link::new("url").disabled(true);
        assert!(link.is_disabled());
    }

    #[test]
    fn test_link_not_disabled() {
        let link = Link::new("url").disabled(false);
        assert!(!link.is_disabled());
    }

    // =========================================================================
    // Link::osc8 builder tests
    // =========================================================================

    #[test]
    fn test_link_osc8_disabled() {
        let link = Link::new("url").osc8(false);
        assert!(link.osc8_start().is_empty());
        assert!(link.osc8_end().is_empty());
    }

    #[test]
    fn test_link_osc8_enabled() {
        let link = Link::new("url").osc8(true);
        assert!(!link.osc8_start().is_empty());
        assert!(!link.osc8_end().is_empty());
    }

    // =========================================================================
    // Link::open tests (platform-dependent)
    // =========================================================================

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_link_open_disabled() {
        let link = Link::new("https://example.com").disabled(true);
        // Disabled links return Ok(())
        assert!(link.open().is_ok());
    }

    // =========================================================================
    // Link state query tests
    // =========================================================================

    #[test]
    fn test_link_url() {
        let link = Link::new("https://example.com/path");
        assert_eq!(link.url(), "https://example.com/path");
    }

    #[test]
    fn test_link_display_text_with_text_set() {
        let link = Link::new("url").text("Custom");
        assert_eq!(link.display_text(), "Custom");
    }

    #[test]
    fn test_link_display_text_fallback_to_url() {
        let link = Link::new("https://example.com");
        assert_eq!(link.display_text(), "https://example.com");
    }

    #[test]
    fn test_link_is_focused() {
        let link = Link::new("url").focused(true);
        assert!(link.is_focused());
    }

    #[test]
    fn test_link_is_disabled() {
        let link = Link::new("url").disabled(true);
        assert!(link.is_disabled());
    }

    // =========================================================================
    // Link clone tests
    // =========================================================================

    #[test]
    fn test_link_clone() {
        let link1 = Link::new("url")
            .text("text")
            .fg(Color::RED)
            .bg(Color::BLUE)
            .focused(true)
            .disabled(false);
        let link2 = link1.clone();

        assert_eq!(link1.url(), link2.url());
        assert_eq!(link1.display_text(), link2.display_text());
        assert_eq!(link1.fg, link2.fg);
        assert_eq!(link1.bg, link2.bg);
        assert_eq!(link1.is_focused(), link2.is_focused());
        assert_eq!(link1.is_disabled(), link2.is_disabled());
    }

    // =========================================================================
    // Link Default tests
    // =========================================================================

    #[test]
    fn test_link_default_style() {
        let link = Link::new("url");
        assert_eq!(link.style, LinkStyle::default());
    }

    // =========================================================================
    // Link osc8_end tests
    // =========================================================================

    #[test]
    fn test_link_osc8_end_when_enabled() {
        let link = Link::new("url").osc8(true);
        assert_eq!(link.osc8_end(), "\x1b]8;;\x1b\\");
    }

    #[test]
    fn test_link_osc8_end_when_disabled() {
        let link = Link::new("url").osc8(false);
        assert!(link.osc8_end().is_empty());
    }

    #[test]
    fn test_link_osc8_end_when_link_disabled() {
        let link = Link::new("url").disabled(true);
        assert!(link.osc8_end().is_empty());
    }

    // =========================================================================
    // format_display edge cases
    // =========================================================================

    #[test]
    fn test_format_display_empty_text() {
        let link = Link::new("url").text("");
        assert_eq!(link.format_display(), "");
    }

    #[test]
    fn test_format_display_unicode() {
        let link = Link::new("url")
            .text("Hello 世界")
            .style(LinkStyle::Bracketed);
        assert_eq!(link.format_display(), "[Hello 世界]");
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_url_link_helper() {
        let link = url_link("https://example.com");
        assert_eq!(link.url(), "https://example.com");
        assert_eq!(link.display_text(), "https://example.com");
    }

    #[test]
    fn test_link_helper_with_text() {
        let link = link("https://example.com", "Click Here");
        assert_eq!(link.url(), "https://example.com");
        assert_eq!(link.display_text(), "Click Here");
    }

    // =========================================================================
    // Link builder chain tests
    // =========================================================================

    #[test]
    fn test_link_builder_chain() {
        let link = Link::new("https://example.com")
            .text("Example")
            .style(LinkStyle::Arrow)
            .fg(Color::CYAN)
            .bg(Color::BLACK)
            .focused(true)
            .disabled(false)
            .tooltip("Hover me")
            .osc8(true);

        assert_eq!(link.url(), "https://example.com");
        assert_eq!(link.display_text(), "Example");
        assert_eq!(link.style, LinkStyle::Arrow);
        assert_eq!(link.fg, Some(Color::CYAN));
        assert_eq!(link.bg, Some(Color::BLACK));
        assert!(link.is_focused());
        assert!(!link.is_disabled());
        assert_eq!(link.tooltip, Some("Hover me".to_string()));
    }

    // =========================================================================
    // Link Debug trait tests
    // =========================================================================

    #[test]
    fn test_link_debug() {
        let link = Link::new("url");
        let debug_str = format!("{:?}", link);
        assert!(debug_str.contains("url"));
    }
}
