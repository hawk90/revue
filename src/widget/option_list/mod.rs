//! Option list widget
//!
//! A flexible list for displaying options with rich formatting, grouping,
//! separators, and keyboard navigation. Unlike SelectionList which is for
//! multi-select, OptionList is for single selection with enhanced display.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{OptionList, Option, option_list};
//!
//! // Simple option list
//! let list = OptionList::new()
//!     .option("Open File", "Ctrl+O")
//!     .option("Save File", "Ctrl+S")
//!     .separator()
//!     .option("Exit", "Ctrl+Q");
//!
//! // With groups
//! let menu = option_list()
//!     .group("File")
//!     .option("New", "")
//!     .option("Open", "")
//!     .group("Edit")
//!     .option("Undo", "")
//!     .option("Redo", "");
//! ```

mod navigation;
mod render;
mod types;

pub use types::{OptionEntry, OptionItem, SeparatorStyle};

use crate::style::Color;
use crate::widget::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Option list widget
#[derive(Clone, Debug)]
pub struct OptionList {
    /// Entries (options, separators, groups)
    entries: Vec<OptionEntry>,
    /// Highlighted index (option index, not entry index)
    highlighted: usize,
    /// Selected option index
    selected: Option<usize>,
    /// Separator style
    separator_style: SeparatorStyle,
    /// Title
    title: Option<String>,
    /// Width
    width: Option<u16>,
    /// Show descriptions
    show_descriptions: bool,
    /// Foreground color
    fg: Option<Color>,
    /// Highlighted color
    highlighted_fg: Option<Color>,
    /// Selected color
    selected_fg: Option<Color>,
    /// Disabled color
    disabled_fg: Option<Color>,
    /// Background color
    bg: Option<Color>,
    /// Highlighted background
    highlighted_bg: Option<Color>,
    /// Max visible items
    max_visible: usize,
    /// Scroll offset
    scroll_offset: usize,
    /// Whether list is focused
    focused: bool,
    /// Show icons
    show_icons: bool,
    /// Widget properties
    props: WidgetProps,
}

impl OptionList {
    /// Create a new option list
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            highlighted: 0,
            selected: None,
            separator_style: SeparatorStyle::default(),
            title: None,
            width: None,
            show_descriptions: false,
            fg: None,
            highlighted_fg: None,
            selected_fg: None,
            disabled_fg: None,
            bg: None,
            highlighted_bg: None,
            max_visible: 10,
            scroll_offset: 0,
            focused: false,
            show_icons: true,
            props: WidgetProps::new(),
        }
    }

    /// Add an option
    pub fn option(mut self, text: impl Into<String>, hint: impl Into<String>) -> Self {
        let hint_str = hint.into();
        let mut item = OptionItem::new(text);
        if !hint_str.is_empty() {
            item.hint = Some(hint_str);
        }
        self.entries.push(OptionEntry::Option(item));
        self
    }

    /// Add an option with full configuration
    pub fn add_option(mut self, option: OptionItem) -> Self {
        self.entries.push(OptionEntry::Option(option));
        self
    }

    /// Add a separator
    pub fn separator(mut self) -> Self {
        self.entries.push(OptionEntry::Separator);
        self
    }

    /// Add a group header
    pub fn group(mut self, name: impl Into<String>) -> Self {
        self.entries.push(OptionEntry::Group(name.into()));
        self
    }

    /// Set separator style
    pub fn separator_style(mut self, style: SeparatorStyle) -> Self {
        self.separator_style = style;
        self
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set width
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Show descriptions
    pub fn show_descriptions(mut self, show: bool) -> Self {
        self.show_descriptions = show;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set highlighted foreground color
    pub fn highlighted_fg(mut self, color: Color) -> Self {
        self.highlighted_fg = Some(color);
        self
    }

    /// Set selected foreground color
    pub fn selected_fg(mut self, color: Color) -> Self {
        self.selected_fg = Some(color);
        self
    }

    /// Set disabled foreground color
    pub fn disabled_fg(mut self, color: Color) -> Self {
        self.disabled_fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set highlighted background color
    pub fn highlighted_bg(mut self, color: Color) -> Self {
        self.highlighted_bg = Some(color);
        self
    }

    /// Set max visible items
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Show/hide icons
    pub fn show_icons(mut self, show: bool) -> Self {
        self.show_icons = show;
        self
    }

    /// Get separator character
    fn separator_char(&self) -> &str {
        match self.separator_style {
            SeparatorStyle::Line => "─",
            SeparatorStyle::Dashed => "╌",
            SeparatorStyle::Double => "═",
            SeparatorStyle::Blank => " ",
        }
    }
}

// ============================================================================
// Test-only getters (doc(hidden))
// ============================================================================

impl OptionList {
    /// Get entries (test-only)
    #[doc(hidden)]
    pub fn __test_entries(&self) -> &Vec<OptionEntry> {
        &self.entries
    }

    /// Get highlighted index (test-only)
    #[doc(hidden)]
    pub fn __test_highlighted(&self) -> usize {
        self.highlighted
    }

    /// Get selected index (test-only)
    #[doc(hidden)]
    pub fn __test_selected(&self) -> Option<usize> {
        self.selected
    }

    /// Get separator style (test-only)
    #[doc(hidden)]
    pub fn __test_separator_style(&self) -> SeparatorStyle {
        self.separator_style
    }

    /// Get title (test-only)
    #[doc(hidden)]
    pub fn __test_title(&self) -> &Option<String> {
        &self.title
    }

    /// Get width (test-only)
    #[doc(hidden)]
    pub fn __test_width(&self) -> &Option<u16> {
        &self.width
    }

    /// Get show_descriptions (test-only)
    #[doc(hidden)]
    pub fn __test_show_descriptions(&self) -> bool {
        self.show_descriptions
    }

    /// Get fg (test-only)
    #[doc(hidden)]
    pub fn __test_fg(&self) -> &Option<Color> {
        &self.fg
    }

    /// Get highlighted_fg (test-only)
    #[doc(hidden)]
    pub fn __test_highlighted_fg(&self) -> &Option<Color> {
        &self.highlighted_fg
    }

    /// Get selected_fg (test-only)
    #[doc(hidden)]
    pub fn __test_selected_fg(&self) -> &Option<Color> {
        &self.selected_fg
    }

    /// Get disabled_fg (test-only)
    #[doc(hidden)]
    pub fn __test_disabled_fg(&self) -> &Option<Color> {
        &self.disabled_fg
    }

    /// Get bg (test-only)
    #[doc(hidden)]
    pub fn __test_bg(&self) -> &Option<Color> {
        &self.bg
    }

    /// Get highlighted_bg (test-only)
    #[doc(hidden)]
    pub fn __test_highlighted_bg(&self) -> &Option<Color> {
        &self.highlighted_bg
    }

    /// Get max_visible (test-only)
    #[doc(hidden)]
    pub fn __test_max_visible(&self) -> usize {
        self.max_visible
    }

    /// Get scroll_offset (test-only)
    #[doc(hidden)]
    pub fn __test_scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    /// Get focused (test-only)
    #[doc(hidden)]
    pub fn __test_focused(&self) -> bool {
        self.focused
    }

    /// Get show_icons (test-only)
    #[doc(hidden)]
    pub fn __test_show_icons(&self) -> bool {
        self.show_icons
    }

    /// Get separator character (test-only)
    #[doc(hidden)]
    pub fn __test_separator_char(&self) -> &str {
        match self.separator_style {
            SeparatorStyle::Line => "─",
            SeparatorStyle::Dashed => "╌",
            SeparatorStyle::Double => "═",
            SeparatorStyle::Blank => " ",
        }
    }
}

impl Default for OptionList {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(OptionList);
impl_props_builders!(OptionList);

/// Create an option list
pub fn option_list() -> OptionList {
    OptionList::new()
}

/// Create an option item
pub fn option_item(text: impl Into<String>) -> OptionItem {
    OptionItem::new(text)
}
