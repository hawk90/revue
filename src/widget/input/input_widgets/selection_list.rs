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

use crate::style::Color;
use crate::widget::traits::WidgetProps;
use crate::widget::{RenderContext, View};
use crate::{impl_props_builders, impl_styled_view};

/// Selection list item
#[derive(Clone, Debug)]
pub struct SelectionItem {
    /// Display text
    pub text: String,
    /// Optional value (for forms)
    pub value: Option<String>,
    /// Whether item is disabled
    pub disabled: bool,
    /// Optional description
    pub description: Option<String>,
    /// Optional icon/prefix
    pub icon: Option<String>,
}

impl SelectionItem {
    /// Create a new item
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            value: None,
            disabled: false,
            description: None,
            icon: None,
        }
    }

    /// Set value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl<S: Into<String>> From<S> for SelectionItem {
    fn from(s: S) -> Self {
        Self::new(s)
    }
}

/// Selection display style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectionStyle {
    /// Checkboxes \[x\] / \[ \]
    #[default]
    Checkbox,
    /// Bullets ● / ○
    Bullet,
    /// Highlight only
    Highlight,
    /// Brackets \[item\] / item
    Bracket,
}

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

    /// Toggle selection of an item
    pub fn toggle(&mut self, index: usize) {
        if index >= self.items.len() {
            return;
        }

        if self.items[index].disabled {
            return;
        }

        if self.is_selected(index) {
            // Deselect if above minimum
            if self.selected.len() > self.min_selections {
                self.selected.retain(|&i| i != index);
            }
        } else {
            // Select if below maximum
            if self.max_selections == 0 || self.selected.len() < self.max_selections {
                self.selected.push(index);
                self.selected.sort();
            }
        }
    }

    /// Toggle highlighted item
    pub fn toggle_highlighted(&mut self) {
        self.toggle(self.highlighted);
    }

    /// Select item
    pub fn select(&mut self, index: usize) {
        if index >= self.items.len() || self.items[index].disabled {
            return;
        }

        if !self.is_selected(index)
            && (self.max_selections == 0 || self.selected.len() < self.max_selections)
        {
            self.selected.push(index);
            self.selected.sort();
        }
    }

    /// Deselect item
    pub fn deselect(&mut self, index: usize) {
        if self.selected.len() > self.min_selections {
            self.selected.retain(|&i| i != index);
        }
    }

    /// Select all
    pub fn select_all(&mut self) {
        self.selected = (0..self.items.len())
            .filter(|&i| !self.items[i].disabled)
            .collect();

        if self.max_selections > 0 {
            self.selected.truncate(self.max_selections);
        }
    }

    /// Deselect all
    pub fn deselect_all(&mut self) {
        if self.min_selections == 0 {
            self.selected.clear();
        } else {
            self.selected.truncate(self.min_selections);
        }
    }

    /// Move highlight up
    pub fn highlight_previous(&mut self) {
        if self.highlighted > 0 {
            self.highlighted -= 1;
            self.ensure_visible();
        }
    }

    /// Move highlight down
    pub fn highlight_next(&mut self) {
        if self.highlighted < self.items.len().saturating_sub(1) {
            self.highlighted += 1;
            self.ensure_visible();
        }
    }

    /// Move highlight to start
    pub fn highlight_first(&mut self) {
        self.highlighted = 0;
        self.scroll_offset = 0;
    }

    /// Move highlight to end
    pub fn highlight_last(&mut self) {
        self.highlighted = self.items.len().saturating_sub(1);
        self.ensure_visible();
    }

    /// Ensure highlighted item is visible
    fn ensure_visible(&mut self) {
        let max_visible = if self.max_visible > 0 {
            self.max_visible
        } else {
            self.items.len()
        };

        if self.highlighted < self.scroll_offset {
            self.scroll_offset = self.highlighted;
        } else if self.highlighted >= self.scroll_offset + max_visible {
            self.scroll_offset = self.highlighted - max_visible + 1;
        }
    }

    /// Get item prefix based on style
    fn item_prefix(&self, index: usize) -> String {
        let is_selected = self.is_selected(index);
        let is_disabled = self.items[index].disabled;

        match self.style {
            SelectionStyle::Checkbox => {
                if is_disabled {
                    "[-] ".to_string()
                } else if is_selected {
                    "[x] ".to_string()
                } else {
                    "[ ] ".to_string()
                }
            }
            SelectionStyle::Bullet => {
                if is_disabled {
                    "◌ ".to_string()
                } else if is_selected {
                    "● ".to_string()
                } else {
                    "○ ".to_string()
                }
            }
            SelectionStyle::Highlight => if is_selected { "▸ " } else { "  " }.to_string(),
            SelectionStyle::Bracket => {
                if is_selected {
                    "[".to_string()
                } else {
                    " ".to_string()
                }
            }
        }
    }

    /// Get item suffix based on style
    fn item_suffix(&self, index: usize) -> String {
        if self.style == SelectionStyle::Bracket && self.is_selected(index) {
            "]".to_string()
        } else {
            String::new()
        }
    }
}

impl View for SelectionList {
    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::vstack;
        use crate::widget::Text;

        let mut content = vstack();

        // Title
        if let Some(title) = &self.title {
            content = content.child(Text::new(title).bold());
        }

        // Selection count
        if self.show_count {
            let count_text = if self.max_selections > 0 {
                format!("Selected: {}/{}", self.selected.len(), self.max_selections)
            } else {
                format!("Selected: {}", self.selected.len())
            };
            content = content.child(Text::new(count_text).fg(Color::rgb(128, 128, 128)));
        }

        // Calculate visible range
        let max_visible = if self.max_visible > 0 {
            self.max_visible
        } else {
            self.items.len()
        };

        let start = self.scroll_offset;
        let end = (start + max_visible).min(self.items.len());

        // Show scroll indicator at top
        if start > 0 {
            content = content.child(Text::new("  ↑ more...").fg(Color::rgb(100, 100, 100)));
        }

        // Render items
        for i in start..end {
            let item = &self.items[i];
            let prefix = self.item_prefix(i);
            let suffix = self.item_suffix(i);

            let icon = item.icon.as_deref().unwrap_or("");
            let text = format!("{}{}{}{}", prefix, icon, item.text, suffix);

            let is_highlighted = i == self.highlighted && self.focused;
            let is_selected = self.is_selected(i);

            let fg = if item.disabled {
                Color::rgb(100, 100, 100)
            } else if is_highlighted {
                self.highlighted_fg.unwrap_or(Color::CYAN)
            } else if is_selected {
                self.selected_fg.unwrap_or(Color::GREEN)
            } else {
                self.fg.unwrap_or(Color::WHITE)
            };

            let mut text_widget = Text::new(&text).fg(fg);

            if is_highlighted {
                text_widget = text_widget.bold();
            }

            content = content.child(text_widget);

            // Show description
            if self.show_descriptions {
                if let Some(desc) = &item.description {
                    let desc_text = format!("    {}", desc);
                    content = content.child(Text::new(desc_text).fg(Color::rgb(128, 128, 128)));
                }
            }
        }

        // Show scroll indicator at bottom
        if end < self.items.len() {
            content = content.child(Text::new("  ↓ more...").fg(Color::rgb(100, 100, 100)));
        }

        // Help text
        if self.focused {
            content = content.child(
                Text::new("↑↓: Navigate | Space: Toggle | a: All | n: None")
                    .fg(Color::rgb(80, 80, 80)),
            );
        }

        content.render(ctx);
    }

    crate::impl_view_meta!("SelectionList");
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
