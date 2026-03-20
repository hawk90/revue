//! Select/Dropdown widget for choosing from a list of options

mod filter;
mod input;
mod render;
mod state;

use crate::style::Color;
use crate::utils::{display_width, Selection};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// A select/dropdown widget with optional fuzzy search
///
/// # Keyboard Shortcuts
///
/// | Key | Action |
/// |-----|--------|
/// | `Enter` | Open dropdown (when closed) / Confirm selection (when open) |
/// | `Space` | Toggle dropdown open/close (non-searchable mode only) |
/// | `Up` / `k` | Move to previous option (when open, non-searchable mode) |
/// | `Down` / `j` | Move to next option (when open, non-searchable mode) |
/// | `Up` | Move to previous option (when open, searchable mode) |
/// | `Down` | Move to next option (when open, searchable mode) |
/// | `Home` | Jump to first option (when open) |
/// | `End` | Jump to last option (when open) |
/// | `Escape` | Close dropdown and clear search query (when open) |
/// | `Backspace` | Delete last character from search query (when open, searchable mode) |
/// | `Char` | Append character to search query (when open, searchable mode) |
#[derive(Clone, Debug)]
pub struct Select {
    pub(crate) options: Vec<String>,
    /// Selection state for options (uses Selection utility)
    pub(crate) selection: Selection,
    pub(crate) open: bool,
    pub(crate) placeholder: String,
    pub(crate) fg: Option<Color>,
    pub(crate) bg: Option<Color>,
    pub(crate) selected_fg: Option<Color>,
    pub(crate) selected_bg: Option<Color>,
    pub(crate) highlight_fg: Option<Color>,
    pub(crate) width: Option<u16>,
    /// Cached auto-calculated width (invalidated on options change)
    pub(crate) cached_auto_width: Option<u16>,
    /// Search query for filtering options
    pub(crate) query: String,
    /// Enable fuzzy search when typing
    pub(crate) searchable: bool,
    /// Filtered indices (into options) based on query
    pub(crate) filtered: Vec<usize>,
    /// Selection state for filtered results (uses Selection utility)
    pub(crate) filtered_selection: Selection,
    /// Focused state
    pub(crate) focused: bool,
    /// Disabled state
    pub(crate) disabled: bool,
    /// CSS styling properties (id, classes)
    pub(crate) props: WidgetProps,
}

impl Select {
    /// Create a new select widget
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            selection: Selection::new(0),
            open: false,
            placeholder: "Select...".to_string(),
            fg: None,
            bg: None,
            selected_fg: Some(Color::WHITE),
            selected_bg: Some(Color::BLUE),
            highlight_fg: Some(Color::YELLOW),
            width: None,
            cached_auto_width: None,
            query: String::new(),
            searchable: false,
            filtered: Vec::new(),
            filtered_selection: Selection::new(0),
            focused: false,
            disabled: false,
            props: WidgetProps::new(),
        }
    }

    /// Set options
    pub fn options(mut self, options: Vec<impl Into<String>>) -> Self {
        self.options = options.into_iter().map(|o| o.into()).collect();
        self.selection.set_len(self.options.len());
        self.reset_filter();
        self.update_cached_width();
        self
    }

    /// Add a single option
    pub fn option(mut self, option: impl Into<String>) -> Self {
        self.options.push(option.into());
        self.selection.set_len(self.options.len());
        self.cached_auto_width = None; // Lazy: recalculated on next display_width()
        self.reset_filter();
        self
    }

    /// Enable fuzzy search when dropdown is open
    pub fn searchable(mut self, enable: bool) -> Self {
        self.searchable = enable;
        self
    }

    /// Set highlight color for matched characters
    pub fn highlight_fg(mut self, color: Color) -> Self {
        self.highlight_fg = Some(color);
        self
    }

    /// Set selected index, clamped to the valid range
    pub fn selected(mut self, index: usize) -> Self {
        let index = index.min(self.options.len().saturating_sub(1));
        self.selection.set(index);
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
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

    /// Set selected item colors
    pub fn selected_style(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = Some(fg);
        self.selected_bg = Some(bg);
        self
    }

    /// Set fixed width
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        if !focused {
            self.open = false;
            self.clear_query();
        }
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Calculate display width (uses cache when available)
    pub(crate) fn display_width(&self, max_width: u16) -> u16 {
        if let Some(w) = self.width {
            return w.min(max_width);
        }

        if let Some(cached) = self.cached_auto_width {
            return cached.min(max_width);
        }

        let max_option_len = self
            .options
            .iter()
            .map(|o| display_width(o))
            .max()
            .unwrap_or(display_width(&self.placeholder));

        // +4 for "▼ " prefix and " " suffix and border
        ((max_option_len + 4) as u16).min(max_width)
    }

    /// Pre-compute and cache auto width (call after options change)
    fn update_cached_width(&mut self) {
        if self.width.is_some() {
            return;
        }
        let max_option_len = self
            .options
            .iter()
            .map(|o| display_width(o))
            .max()
            .unwrap_or(display_width(&self.placeholder));
        self.cached_auto_width = Some((max_option_len + 4) as u16);
    }
}

impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(Select);
impl_props_builders!(Select);

/// Helper function to create a select widget
pub fn select() -> Select {
    Select::new()
}
