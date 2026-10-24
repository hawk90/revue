//! Multi-select widget for choosing multiple options from a list
//!
//! Provides a dropdown with:
//! - Multiple selection with tag display
//! - Fuzzy search filtering
//! - Tag navigation and removal
//! - Optional maximum selection limit

mod builders;
mod filter;
mod helpers;
mod input;
mod navigation;
mod option;
mod selection;
#[cfg(test)]
mod tests;
mod view;

use crate::style::Color;
use crate::utils::EditState;
use crate::utils::ScrollState;
use crate::widget::traits::{WidgetProps, WidgetState};

// Re-export public types
pub use option::MultiSelectOption;

// Re-export constructor functions
pub use helpers::{multi_select, multi_select_from};

/// A multi-select widget for choosing multiple options
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::{multi_select, MultiSelect, MultiSelectOption};
///
/// // Basic multi-select
/// let select = multi_select()
///     .option("Apple")
///     .option("Banana")
///     .option("Cherry");
///
/// // With pre-selected values
/// let select = multi_select()
///     .options(vec!["Red", "Green", "Blue"])
///     .selected_indices(vec![0, 2]);  // Red and Blue selected
///
/// // From a list of items
/// let fruits = vec!["Apple", "Banana", "Cherry", "Date"];
/// let select = multi_select_from(fruits);
/// ```
///
/// # Keyboard Shortcuts
///
/// ## Dropdown navigation
///
/// | Key | Action |
/// |-----|--------|
/// | `Enter` | Toggle selection of highlighted option (when open) / Open dropdown (when closed) |
/// | `Escape` | Close dropdown (when open) / Deselect tag cursor (when closed) |
/// | `Space` | Toggle selection of highlighted option (when open, non-searchable mode) |
/// | `Down` / `j` | Move highlight down in dropdown (when open) |
/// | `Up` / `k` | Move highlight up in dropdown (when open) |
/// | the editing keys | Edit the search query (when open, searchable mode): the standard keys every single-line field shares (see [`EditState::edit_key`](crate::utils::EditState::edit_key)) |
///
/// ## Tag management (when dropdown is closed)
///
/// | Key | Action |
/// |-----|--------|
/// | `Left` | Move tag cursor left |
/// | `Right` | Move tag cursor right |
/// | `Backspace` | Remove tag at cursor (if cursor active) / Remove last tag |
/// | `Delete` | Remove tag at cursor (if cursor active) |
/// | `a` | Select all options |
/// | `c` | Clear all selections |
#[derive(Debug, Clone)]
pub struct MultiSelect {
    /// Available options
    options: Vec<MultiSelectOption>,
    /// Selected option indices
    selected: Vec<usize>,
    /// What a selection builder asked for, applied again whenever the
    /// options or the maximum change, so the builders give the same widget
    /// in any order (#1011); dropped once the user changes the selection
    requested: Option<Requested>,
    /// Whether dropdown is open
    open: bool,
    /// Cursor position in dropdown
    dropdown_cursor: usize,
    /// Where the open dropdown is scrolled to, one row per shown option
    dropdown_scroll: ScrollState,
    /// Cursor position in tags (for navigation/deletion)
    tag_cursor: Option<usize>,
    /// Where the tag row is scrolled to, in columns
    tag_scroll: ScrollState,
    /// Search query for filtering, edited as every single-line field is
    /// (#985)
    query: EditState,
    /// Horizontal scroll of the query in the header, in columns
    query_scroll: std::cell::Cell<usize>,
    /// Filtered option indices
    filtered: Vec<usize>,
    /// Placeholder text
    placeholder: String,
    /// Maximum number of selections (None = unlimited)
    max_selections: Option<usize>,
    /// Width of the widget
    width: Option<u16>,
    /// Whether search is enabled
    searchable: bool,
    /// Highlight color for matched characters
    highlight_fg: Option<Color>,
    /// Selected tag background color
    tag_bg: Option<Color>,
    /// Widget state
    pub state: WidgetState,
    /// Widget props
    pub props: WidgetProps,
}

/// A selection a builder asked for
#[derive(Clone, Debug)]
enum Requested {
    Indices(Vec<usize>),
    Values(Vec<String>),
}

impl MultiSelect {
    /// Create a new multi-select widget
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            selected: Vec::new(),
            requested: None,
            open: false,
            dropdown_cursor: 0,
            dropdown_scroll: ScrollState::new(),
            tag_scroll: ScrollState::new(),
            tag_cursor: None,
            query: EditState::single_line(),
            query_scroll: std::cell::Cell::new(0),
            filtered: Vec::new(),
            placeholder: "Select...".to_string(),
            max_selections: None,
            width: None,
            searchable: true,
            highlight_fg: Some(Color::YELLOW),
            tag_bg: Some(Color::rgb(60, 60, 140)),
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Reset filter to show all options
    fn reset_filter(&mut self) {
        self.filtered = (0..self.options.len()).collect();
        self.dropdown_cursor = 0;
    }
}

impl Default for MultiSelect {
    fn default() -> Self {
        Self::new()
    }
}

// Macro implementations
crate::impl_styled_view!(MultiSelect);
crate::impl_widget_builders!(MultiSelect);
