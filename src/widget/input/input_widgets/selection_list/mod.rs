//! Multi-selection list widget
//!
//! A list widget that allows selecting multiple items, with support for
//! checkboxes, highlighting, and keyboard navigation.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{SelectionList, selection_list};
//!
//! // Create a multi-select list
//! let list = SelectionList::new(vec![
//!     "Option 1",
//!     "Option 2",
//!     "Option 3",
//! ]).selected(vec![0, 2]);
//!
//! // With checkboxes
//! let features = selection_list(vec![
//!     "Feature A",
//!     "Feature B",
//!     "Feature C",
//! ]).show_checkboxes(true);
//! ```

mod render;
mod selection;
mod types;

pub use types::{SelectionItem, SelectionStyle};

use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Multi-selection list widget
#[derive(Clone, Debug)]
pub struct SelectionList {
    /// List items
    items: Vec<SelectionItem>,
    /// Selected indices
    selected: Vec<usize>,
    /// Currently highlighted index
    highlighted: usize,
    /// Selection style
    style: SelectionStyle,
    /// Maximum selections (0 = unlimited)
    max_selections: usize,
    /// Minimum selections
    min_selections: usize,
    /// Show descriptions
    show_descriptions: bool,
    /// Title
    title: Option<String>,
    /// Foreground color
    fg: Option<Color>,
    /// Selected item color
    selected_fg: Option<Color>,
    /// Highlighted item color
    highlighted_fg: Option<Color>,
    /// Background color
    bg: Option<Color>,
    /// Maximum visible items (0 = show all)
    max_visible: usize,
    /// Scroll offset
    scroll_offset: usize,
    /// Show selection count
    show_count: bool,
    /// Whether list is focused
    focused: bool,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl SelectionList {
    /// Create a new selection list
    pub fn new<I, T>(items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<SelectionItem>,
    {
        Self {
            items: items.into_iter().map(|i| i.into()).collect(),
            selected: Vec::new(),
            highlighted: 0,
            style: SelectionStyle::default(),
            max_selections: 0,
            min_selections: 0,
            show_descriptions: false,
            title: None,
            fg: None,
            selected_fg: None,
            highlighted_fg: None,
            bg: None,
            max_visible: 0,
            scroll_offset: 0,
            show_count: false,
            focused: false,
            props: WidgetProps::new(),
        }
    }

    /// Set initial selection
    pub fn selected(mut self, indices: Vec<usize>) -> Self {
        self.selected = indices;
        self
    }

    /// Set selection style
    pub fn style(mut self, style: SelectionStyle) -> Self {
        self.style = style;
        self
    }

    /// Show checkboxes (shorthand for style)
    pub fn show_checkboxes(self, show: bool) -> Self {
        if show {
            self.style(SelectionStyle::Checkbox)
        } else {
            self.style(SelectionStyle::Highlight)
        }
    }

    /// Set maximum selections
    pub fn max_selections(mut self, max: usize) -> Self {
        self.max_selections = max;
        self
    }

    /// Set minimum selections
    pub fn min_selections(mut self, min: usize) -> Self {
        self.min_selections = min;
        self
    }

    /// Show descriptions
    pub fn show_descriptions(mut self, show: bool) -> Self {
        self.show_descriptions = show;
        self
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set selected item color
    pub fn selected_fg(mut self, color: Color) -> Self {
        self.selected_fg = Some(color);
        self
    }

    /// Set highlighted item color
    pub fn highlighted_fg(mut self, color: Color) -> Self {
        self.highlighted_fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set maximum visible items
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }

    /// Show selection count
    pub fn show_count(mut self, show: bool) -> Self {
        self.show_count = show;
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Get selected indices
    pub fn get_selected(&self) -> &[usize] {
        &self.selected
    }

    /// Get selected values
    pub fn get_selected_values(&self) -> Vec<&str> {
        self.selected
            .iter()
            .filter_map(|&i| {
                self.items
                    .get(i)
                    .map(|item| item.value.as_deref().unwrap_or(&item.text))
            })
            .collect()
    }

    /// Get selected items
    pub fn get_selected_items(&self) -> Vec<&SelectionItem> {
        self.selected
            .iter()
            .filter_map(|&i| self.items.get(i))
            .collect()
    }

    /// Check if index is selected
    pub fn is_selected(&self, index: usize) -> bool {
        self.selected.contains(&index)
    }
}

impl_styled_view!(SelectionList);
impl_props_builders!(SelectionList);

/// Create a selection list
pub fn selection_list<I, T>(items: I) -> SelectionList
where
    I: IntoIterator<Item = T>,
    T: Into<SelectionItem>,
{
    SelectionList::new(items)
}

/// Create a selection item
pub fn selection_item(text: impl Into<String>) -> SelectionItem {
    SelectionItem::new(text)
}

// KEEP HERE: All public API tests extracted to tests/widget/input/selection_list.rs
