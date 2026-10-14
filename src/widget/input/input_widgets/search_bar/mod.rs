//! SearchBar widget with Query DSL support
//!
//! A search input widget that parses queries in real-time and provides
//! visual feedback for query syntax.

mod edit;
mod view;

use crate::query::{ParseError, Query};
use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// A search bar widget with query DSL parsing
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::{search_bar, SearchBar};
///
/// let mut search = search_bar()
///     .placeholder("Search (e.g., author:john status:active)")
///     .width(50);
///
/// // Handle input
/// search.input('a');
/// search.set_query("author:john");
///
/// // Get parsed query
/// if let Some(query) = search.query() {
///     let filtered = query.filter_items(&items);
/// }
/// ```
#[derive(Clone)]
pub struct SearchBar {
    /// Current input text
    input: String,
    /// Cursor position
    cursor: usize,
    /// Parsed query (if valid)
    parsed_query: Option<Query>,
    /// Parse error (if invalid)
    parse_error: Option<ParseError>,
    /// Placeholder text
    placeholder: String,
    /// Width
    width: u16,
    /// Whether input is focused
    focused: bool,
    /// Whether to show query hints
    show_hints: bool,
    /// Search icon
    icon: char,
    /// Colors
    bg_color: Color,
    border_color: Color,
    /// The color the builder named, if it named one - see #656.
    text_color: Option<Color>,
    placeholder_color: Color,
    error_color: Color,
    /// Widget properties
    props: WidgetProps,
}

impl SearchBar {
    /// Create a new search bar
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor: 0,
            parsed_query: Some(Query::new()),
            parse_error: None,
            placeholder: "Search...".to_string(),
            width: 40,
            focused: false,
            show_hints: true,
            icon: '🔍',
            bg_color: Color::rgb(30, 30, 40),
            border_color: Color::rgb(80, 80, 100),
            text_color: None,
            placeholder_color: Color::rgb(100, 100, 120),
            error_color: Color::RED,
            props: WidgetProps::new(),
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set width
    pub fn width(mut self, width: u16) -> Self {
        self.width = width.max(10);
        self
    }

    /// Set search icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = icon;
        self
    }

    /// Show/hide query hints
    pub fn show_hints(mut self, show: bool) -> Self {
        self.show_hints = show;
        self
    }

    /// Set colors
    pub fn colors(mut self, bg: Color, border: Color, text: Color) -> Self {
        self.bg_color = bg;
        self.border_color = border;
        self.text_color = Some(text);
        self
    }

    /// Set error color
    pub fn error_color(mut self, color: Color) -> Self {
        self.error_color = color;
        self
    }

    /// Focus the search bar
    pub fn focus(&mut self) {
        self.focused = true;
    }

    /// Unfocus the search bar
    pub fn blur(&mut self) {
        self.focused = false;
    }

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Get current input text
    pub fn get_input(&self) -> &str {
        &self.input
    }

    /// Set input text
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.input = query.into();
        self.cursor = self.input.chars().count();
        self.parse_input();
    }

    /// Clear input
    pub fn clear(&mut self) {
        self.input.clear();
        self.cursor = 0;
        self.parsed_query = Some(Query::new());
        self.parse_error = None;
    }

    /// Get parsed query (if valid)
    pub fn query(&self) -> Option<&Query> {
        self.parsed_query.as_ref()
    }

    /// Get parse error (if any)
    pub fn error(&self) -> Option<&ParseError> {
        self.parse_error.as_ref()
    }

    /// Check if query is valid
    pub fn is_valid(&self) -> bool {
        self.parse_error.is_none()
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(SearchBar);
impl_props_builders!(SearchBar);

/// Helper function to create a search bar
pub fn search_bar() -> SearchBar {
    SearchBar::new()
}
