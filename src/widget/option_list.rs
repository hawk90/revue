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

use crate::style::Color;
use crate::widget::{View, RenderContext, WidgetProps};
use crate::{impl_styled_view, impl_props_builders};

/// Option list entry type
#[derive(Clone, Debug)]
pub enum OptionEntry {
    /// Regular option
    Option(OptionItem),
    /// Separator line
    Separator,
    /// Group header
    Group(String),
}

/// Single option item
#[derive(Clone, Debug)]
pub struct OptionItem {
    /// Display text
    pub text: String,
    /// Optional secondary text (right-aligned)
    pub hint: Option<String>,
    /// Optional value/id
    pub value: Option<String>,
    /// Whether option is disabled
    pub disabled: bool,
    /// Optional icon/prefix
    pub icon: Option<String>,
    /// Optional description
    pub description: Option<String>,
}

impl OptionItem {
    /// Create a new option
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            hint: None,
            value: None,
            disabled: false,
            icon: None,
            description: None,
        }
    }

    /// Set hint text
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
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

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// Separator style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SeparatorStyle {
    /// Single line ─
    #[default]
    Line,
    /// Dashed ╌
    Dashed,
    /// Double ═
    Double,
    /// Blank line
    Blank,
}

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

    /// Get option count (excluding separators and groups)
    pub fn option_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| matches!(e, OptionEntry::Option(_)))
            .count()
    }

    /// Get highlighted option
    pub fn get_highlighted(&self) -> Option<&OptionItem> {
        let mut option_idx = 0;
        for entry in &self.entries {
            if let OptionEntry::Option(item) = entry {
                if option_idx == self.highlighted {
                    return Some(item);
                }
                option_idx += 1;
            }
        }
        None
    }

    /// Get selected option
    pub fn get_selected(&self) -> Option<&OptionItem> {
        let selected = self.selected?;
        let mut option_idx = 0;
        for entry in &self.entries {
            if let OptionEntry::Option(item) = entry {
                if option_idx == selected {
                    return Some(item);
                }
                option_idx += 1;
            }
        }
        None
    }

    /// Get selected value
    pub fn get_selected_value(&self) -> Option<&str> {
        self.get_selected()
            .and_then(|item| item.value.as_deref().or(Some(&item.text)))
    }

    /// Select highlighted option
    pub fn select_highlighted(&mut self) -> bool {
        if let Some(item) = self.get_highlighted() {
            if !item.disabled {
                self.selected = Some(self.highlighted);
                return true;
            }
        }
        false
    }

    /// Select by index
    pub fn select(&mut self, index: usize) {
        let mut option_idx = 0;
        for entry in &self.entries {
            if let OptionEntry::Option(item) = entry {
                if option_idx == index && !item.disabled {
                    self.selected = Some(index);
                    self.highlighted = index;
                    return;
                }
                option_idx += 1;
            }
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected = None;
    }

    /// Move highlight to previous option
    pub fn highlight_previous(&mut self) {
        if self.highlighted > 0 {
            self.highlighted -= 1;

            // Skip disabled items
            while self.highlighted > 0 {
                if let Some(item) = self.get_highlighted() {
                    if !item.disabled {
                        break;
                    }
                }
                self.highlighted -= 1;
            }

            self.ensure_visible();
        }
    }

    /// Move highlight to next option
    pub fn highlight_next(&mut self) {
        let max = self.option_count().saturating_sub(1);
        if self.highlighted < max {
            self.highlighted += 1;

            // Skip disabled items
            while self.highlighted < max {
                if let Some(item) = self.get_highlighted() {
                    if !item.disabled {
                        break;
                    }
                }
                self.highlighted += 1;
            }

            self.ensure_visible();
        }
    }

    /// Move to first option
    pub fn highlight_first(&mut self) {
        self.highlighted = 0;

        // Skip disabled items
        while self.highlighted < self.option_count() - 1 {
            if let Some(item) = self.get_highlighted() {
                if !item.disabled {
                    break;
                }
            }
            self.highlighted += 1;
        }

        self.scroll_offset = 0;
    }

    /// Move to last option
    pub fn highlight_last(&mut self) {
        self.highlighted = self.option_count().saturating_sub(1);

        // Skip disabled items
        while self.highlighted > 0 {
            if let Some(item) = self.get_highlighted() {
                if !item.disabled {
                    break;
                }
            }
            self.highlighted -= 1;
        }

        self.ensure_visible();
    }

    /// Ensure highlighted item is visible
    fn ensure_visible(&mut self) {
        if self.highlighted < self.scroll_offset {
            self.scroll_offset = self.highlighted;
        } else if self.highlighted >= self.scroll_offset + self.max_visible {
            self.scroll_offset = self.highlighted - self.max_visible + 1;
        }
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

impl Default for OptionList {
    fn default() -> Self {
        Self::new()
    }
}

impl View for OptionList {
    crate::impl_view_meta!("OptionList");

    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::Text;
        use crate::widget::stack::vstack;

        let width = self.width.unwrap_or(40) as usize;

        let mut content = vstack();

        // Title
        if let Some(title) = &self.title {
            content = content.child(Text::new(title).bold());
        }

        let mut option_idx = 0;
        let mut visible_count = 0;
        let mut skipped = 0;

        for entry in &self.entries {
            // Handle scrolling
            if let OptionEntry::Option(_) = entry {
                if option_idx < self.scroll_offset {
                    option_idx += 1;
                    skipped += 1;
                    continue;
                }
                if visible_count >= self.max_visible {
                    break;
                }
            }

            match entry {
                OptionEntry::Option(item) => {
                    let is_highlighted = option_idx == self.highlighted && self.focused;
                    let is_selected = self.selected == Some(option_idx);

                    // Build option text
                    let icon = if self.show_icons {
                        item.icon.as_deref().unwrap_or("")
                    } else {
                        ""
                    };

                    let prefix = if is_selected {
                        "▸ "
                    } else if is_highlighted {
                        "> "
                    } else {
                        "  "
                    };

                    let main_text = format!("{}{}{}", prefix, icon, item.text);

                    // Calculate padding for hint
                    let hint = item.hint.as_deref().unwrap_or("");
                    let padding = width.saturating_sub(main_text.len() + hint.len());

                    // Determine colors
                    let fg = if item.disabled {
                        self.disabled_fg.unwrap_or(Color::rgb(100, 100, 100))
                    } else if is_highlighted {
                        self.highlighted_fg.unwrap_or(Color::CYAN)
                    } else if is_selected {
                        self.selected_fg.unwrap_or(Color::GREEN)
                    } else {
                        self.fg.unwrap_or(Color::WHITE)
                    };

                    let bg = if is_highlighted {
                        self.highlighted_bg
                    } else {
                        self.bg
                    };

                    // Build row
                    let mut text = Text::new(format!(
                        "{}{}{}",
                        main_text,
                        " ".repeat(padding),
                        hint
                    )).fg(fg);

                    if let Some(bg) = bg {
                        text = text.bg(bg);
                    }

                    if is_highlighted || is_selected {
                        text = text.bold();
                    }

                    content = content.child(text);

                    // Show description
                    if self.show_descriptions {
                        if let Some(desc) = &item.description {
                            content = content.child(
                                Text::new(format!("    {}", desc))
                                    .fg(Color::rgb(128, 128, 128))
                            );
                        }
                    }

                    option_idx += 1;
                    visible_count += 1;
                }
                OptionEntry::Separator => {
                    let line = self.separator_char().repeat(width);
                    content = content.child(Text::new(line).fg(Color::rgb(80, 80, 80)));
                }
                OptionEntry::Group(name) => {
                    content = content.child(
                        Text::new(name).fg(Color::rgb(180, 180, 180)).bold()
                    );
                }
            }
        }

        // Scroll indicators
        if skipped > 0 {
            content = vstack()
                .child(Text::new("↑").fg(Color::rgb(100, 100, 100)))
                .child(content);
        }

        let remaining = self.option_count() - (self.scroll_offset + visible_count);
        if remaining > 0 {
            content = content.child(Text::new("↓").fg(Color::rgb(100, 100, 100)));
        }

        content.render(ctx);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_list_new() {
        let list = OptionList::new();
        assert_eq!(list.option_count(), 0);
    }

    #[test]
    fn test_add_options() {
        let list = OptionList::new()
            .option("Option 1", "Ctrl+1")
            .option("Option 2", "Ctrl+2");
        assert_eq!(list.option_count(), 2);
    }

    #[test]
    fn test_separators() {
        let list = OptionList::new()
            .option("A", "")
            .separator()
            .option("B", "");
        assert_eq!(list.entries.len(), 3);
        assert_eq!(list.option_count(), 2);
    }

    #[test]
    fn test_groups() {
        let list = OptionList::new()
            .group("Group 1")
            .option("A", "")
            .group("Group 2")
            .option("B", "");
        assert_eq!(list.entries.len(), 4);
        assert_eq!(list.option_count(), 2);
    }

    #[test]
    fn test_navigation() {
        let mut list = OptionList::new()
            .option("A", "")
            .option("B", "")
            .option("C", "")
            .focused(true);

        assert_eq!(list.highlighted, 0);
        list.highlight_next();
        assert_eq!(list.highlighted, 1);
        list.highlight_previous();
        assert_eq!(list.highlighted, 0);
        list.highlight_last();
        assert_eq!(list.highlighted, 2);
        list.highlight_first();
        assert_eq!(list.highlighted, 0);
    }

    #[test]
    fn test_selection() {
        let mut list = OptionList::new()
            .option("A", "")
            .option("B", "")
            .focused(true);

        assert!(list.selected.is_none());
        list.highlight_next();
        list.select_highlighted();
        assert_eq!(list.selected, Some(1));

        let selected = list.get_selected().unwrap();
        assert_eq!(selected.text, "B");
    }

    #[test]
    fn test_disabled_skip() {
        let mut list = OptionList::new()
            .add_option(OptionItem::new("A"))
            .add_option(OptionItem::new("B").disabled(true))
            .add_option(OptionItem::new("C"))
            .focused(true);

        assert_eq!(list.highlighted, 0);
        list.highlight_next(); // Should skip B
        assert_eq!(list.highlighted, 2);
    }

    #[test]
    fn test_disabled_no_select() {
        let mut list = OptionList::new()
            .add_option(OptionItem::new("A").disabled(true))
            .focused(true);

        assert!(!list.select_highlighted());
        assert!(list.selected.is_none());
    }

    #[test]
    fn test_get_selected_value() {
        let mut list = OptionList::new()
            .add_option(OptionItem::new("Display").value("actual_value"))
            .focused(true);

        list.select_highlighted();
        assert_eq!(list.get_selected_value(), Some("actual_value"));
    }

    #[test]
    fn test_separator_styles() {
        let list = OptionList::new().separator_style(SeparatorStyle::Double);
        assert_eq!(list.separator_char(), "═");
    }

    #[test]
    fn test_helper_functions() {
        let list = option_list().option("Test", "hint");
        assert_eq!(list.option_count(), 1);

        let item = option_item("Test").hint("hint").value("val");
        assert_eq!(item.hint, Some("hint".to_string()));
        assert_eq!(item.value, Some("val".to_string()));
    }
}
