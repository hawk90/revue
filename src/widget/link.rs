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

    // Getters for testing
    #[doc(hidden)]
    pub fn get_style(&self) -> LinkStyle {
        self.style
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
    pub fn get_tooltip(&self) -> &Option<String> {
        &self.tooltip
    }

    #[doc(hidden)]
    pub fn get_osc8(&self) -> bool {
        self.osc8
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
