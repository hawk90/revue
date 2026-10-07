//! Theme system for consistent styling
//!
//! Provides theme support for TUI applications including:
//! - Built-in themes (Dark, Light, High Contrast)
//! - Popular themes (Dracula, Nord, Monokai, Solarized)
//! - Theme switching at runtime
//! - Theme persistence
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::style::theme::{ThemeManager, Theme, Themes};
//!
//! let mut manager = ThemeManager::new();
//! // Default theme is "dark", toggle switches between "dark" and "light"
//! manager.register("dracula", Themes::dracula());
//!
//! manager.set_theme("dracula");
//! println!("Current: {}", manager.current().name);
//!
//! // toggle_dark_light() switches between dark_theme ("dark") and light_theme ("light")
//! // Use set_dark_theme()/set_light_theme() to customize toggle targets
//! manager.toggle_dark_light();
//! ```

mod builtin;
mod manager;
mod palette;

pub use builtin::Themes;
pub use manager::{shared_theme, theme_manager, SharedTheme, ThemeChangeListener, ThemeManager};
pub use palette::{Palette, ThemeColors};

use super::properties::Color;

/// Theme variant (dark, light, high contrast)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeVariant {
    /// Dark theme variant (default)
    #[default]
    Dark,
    /// Light theme variant
    Light,
    /// High contrast accessibility theme
    HighContrast,
}

/// Complete theme
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Theme variant
    pub variant: ThemeVariant,
    /// Color palette
    pub palette: Palette,
    /// Theme colors
    pub colors: ThemeColors,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// Create dark theme
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            variant: ThemeVariant::Dark,
            palette: Palette::dark(),
            colors: ThemeColors::dark(),
        }
    }

    /// Create light theme
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            variant: ThemeVariant::Light,
            palette: Palette::light(),
            colors: ThemeColors::light(),
        }
    }

    /// Create high contrast theme
    pub fn high_contrast() -> Self {
        Self {
            name: "High Contrast".to_string(),
            variant: ThemeVariant::HighContrast,
            palette: Palette::high_contrast(),
            colors: ThemeColors::high_contrast(),
        }
    }

    /// Create a custom theme
    pub fn custom(name: impl Into<String>) -> ThemeBuilder {
        ThemeBuilder::new(name)
    }

    /// Check if theme is dark
    pub fn is_dark(&self) -> bool {
        self.variant == ThemeVariant::Dark
    }

    /// Check if theme is light
    pub fn is_light(&self) -> bool {
        self.variant == ThemeVariant::Light
    }
}

/// Theme builder
pub struct ThemeBuilder {
    theme: Theme,
}

impl ThemeBuilder {
    /// Create a new theme builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            theme: Theme {
                name: name.into(),
                ..Theme::dark()
            },
        }
    }

    /// Set theme variant
    pub fn variant(mut self, variant: ThemeVariant) -> Self {
        self.theme.variant = variant;
        self
    }

    /// Set palette
    pub fn palette(mut self, palette: Palette) -> Self {
        self.theme.palette = palette;
        self
    }

    /// Set colors
    pub fn colors(mut self, colors: ThemeColors) -> Self {
        self.theme.colors = colors;
        self
    }

    /// Set primary color
    pub fn primary(mut self, color: Color) -> Self {
        self.theme.palette.primary = color;
        self
    }

    /// Set background color
    pub fn background(mut self, color: Color) -> Self {
        self.theme.colors.background = color;
        self
    }

    /// Set text color
    pub fn text(mut self, color: Color) -> Self {
        self.theme.colors.text = color;
        self
    }

    /// Build the theme
    pub fn build(self) -> Theme {
        self.theme
    }
}

// Tests

#[cfg(test)]
mod tests;
