//! Theme picker widget for runtime theme switching
//!
//! Provides a dropdown-style widget for selecting themes from the registered
//! theme list. Integrates with the reactive theme system for automatic UI updates.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::theme_picker;
//!
//! // Basic theme picker
//! theme_picker().render(ctx);
//!
//! // With specific themes only
//! theme_picker()
//!     .themes(["dark", "light", "dracula", "nord"])
//!     .render(ctx);
//!
//! // Compact mode (shows only color swatches)
//! theme_picker()
//!     .compact(true)
//!     .render(ctx);
//! ```

mod input;
mod view;

use crate::style::{get_theme, set_theme_by_id, theme_ids, use_theme, Color, Theme};
use crate::utils::ScrollState;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Theme picker widget for selecting themes
#[derive(Clone, Debug)]
pub struct ThemePicker {
    /// Available theme IDs
    themes: Vec<String>,
    /// Currently selected index in themes list
    selected_index: usize,
    /// Where the open list is scrolled to
    scroll: ScrollState,
    /// Whether dropdown is open
    open: bool,
    /// Show compact color swatch mode
    compact: bool,
    /// Show theme preview
    show_preview: bool,
    /// Widget width
    width: Option<u16>,
    /// Foreground color override
    fg: Option<Color>,
    /// Background color override
    bg: Option<Color>,
    /// CSS styling properties
    props: WidgetProps,
}

impl ThemePicker {
    /// Create a new theme picker with all registered themes
    pub fn new() -> Self {
        let all_themes = theme_ids();
        let current = use_theme().get();
        let selected_index = all_themes
            .iter()
            .position(|id| {
                get_theme(id)
                    .map(|t| t.name == current.name)
                    .unwrap_or(false)
            })
            .unwrap_or(0);

        Self {
            themes: all_themes,
            selected_index,
            scroll: ScrollState::new(),
            open: false,
            compact: false,
            show_preview: true,
            width: None,
            fg: None,
            bg: None,
            props: WidgetProps::new(),
        }
    }

    /// Set specific themes to show (by ID)
    pub fn themes<I, S>(mut self, theme_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.themes = theme_ids.into_iter().map(|s| s.into()).collect();
        self.selected_index = 0;
        self
    }

    /// Enable compact mode (color swatches only)
    pub fn compact(mut self, enable: bool) -> Self {
        self.compact = enable;
        self
    }

    /// Show each theme's color swatch next to its name (default: true)
    ///
    /// With `false`, the header and the dropdown list show names only.
    /// Compact mode is made of swatches, so it ignores this.
    pub fn show_preview(mut self, show: bool) -> Self {
        self.show_preview = show;
        self
    }

    /// Set widget width
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
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

    /// Toggle dropdown open/closed
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    /// Open the dropdown
    pub fn open(&mut self) {
        self.open = true;
    }

    /// Close the dropdown
    pub fn close(&mut self) {
        self.open = false;
    }

    /// Check if dropdown is open
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Move selection up
    pub fn select_prev(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if self.selected_index < self.themes.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    /// Apply the selected theme
    pub fn apply_selected(&self) {
        if let Some(id) = self.themes.get(self.selected_index) {
            set_theme_by_id(id);
        }
    }

    /// Get currently selected theme ID
    pub fn selected_id(&self) -> Option<&str> {
        self.themes.get(self.selected_index).map(|s| s.as_str())
    }

    /// Get selected theme
    pub fn selected_theme(&self) -> Option<Theme> {
        self.selected_id().and_then(get_theme)
    }
}

impl Default for ThemePicker {
    fn default() -> Self {
        Self::new()
    }
}

// Implement styled view macros
impl_styled_view!(ThemePicker);
impl_props_builders!(ThemePicker);

/// Create a new theme picker
pub fn theme_picker() -> ThemePicker {
    ThemePicker::new()
}
