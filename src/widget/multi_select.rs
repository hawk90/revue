//! Multi-select widget for choosing multiple options from a list
//!
//! Provides a dropdown with:
//! - Multiple selection with tag display
//! - Fuzzy search filtering
//! - Tag navigation and removal
//! - Optional maximum selection limit

use super::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::event::Key;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{fuzzy_match, FuzzyMatch};
use crate::{impl_styled_view, impl_view_meta, impl_widget_builders};

/// An option in the multi-select widget
#[derive(Debug, Clone)]
pub struct MultiSelectOption {
    /// Display label
    pub label: String,
    /// Value (can be same as label)
    pub value: String,
    /// Whether this option is disabled
    pub disabled: bool,
}

impl MultiSelectOption {
    /// Create a new option
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    /// Create an option where label equals value
    pub fn simple(label: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            value: label.clone(),
            label,
            disabled: false,
        }
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

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
#[derive(Debug, Clone)]
pub struct MultiSelect {
    /// Available options
    options: Vec<MultiSelectOption>,
    /// Selected option indices
    selected: Vec<usize>,
    /// Whether dropdown is open
    open: bool,
    /// Cursor position in dropdown
    dropdown_cursor: usize,
    /// Cursor position in tags (for navigation/deletion)
    tag_cursor: Option<usize>,
    /// Search query for filtering
    query: String,
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
    state: WidgetState,
    /// Widget props
    props: WidgetProps,
}

impl MultiSelect {
    /// Create a new multi-select widget
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            selected: Vec::new(),
            open: false,
            dropdown_cursor: 0,
            tag_cursor: None,
            query: String::new(),
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

    /// Set options from a vector of strings
    pub fn options(mut self, options: Vec<impl Into<String>>) -> Self {
        self.options = options
            .into_iter()
            .map(|o| MultiSelectOption::simple(o))
            .collect();
        self.reset_filter();
        self
    }

    /// Set options from MultiSelectOption items
    pub fn options_detailed(mut self, options: Vec<MultiSelectOption>) -> Self {
        self.options = options;
        self.reset_filter();
        self
    }

    /// Add a single option
    pub fn option(mut self, label: impl Into<String>) -> Self {
        self.options.push(MultiSelectOption::simple(label));
        self.reset_filter();
        self
    }

    /// Add a detailed option
    pub fn option_detailed(mut self, option: MultiSelectOption) -> Self {
        self.options.push(option);
        self.reset_filter();
        self
    }

    /// Set pre-selected indices
    pub fn selected_indices(mut self, indices: Vec<usize>) -> Self {
        self.selected = indices
            .into_iter()
            .filter(|&i| i < self.options.len())
            .collect();
        self
    }

    /// Set pre-selected values
    pub fn selected_values(mut self, values: Vec<impl AsRef<str>>) -> Self {
        self.selected = values
            .iter()
            .filter_map(|v| self.options.iter().position(|opt| opt.value == v.as_ref()))
            .collect();
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set maximum number of selections
    pub fn max_selections(mut self, max: usize) -> Self {
        self.max_selections = Some(max);
        self
    }

    /// Set widget width
    pub fn width(mut self, width: u16) -> Self {
        self.width = Some(width);
        self
    }

    /// Enable or disable search
    pub fn searchable(mut self, enable: bool) -> Self {
        self.searchable = enable;
        self
    }

    /// Set highlight color for matched characters
    pub fn highlight_fg(mut self, color: Color) -> Self {
        self.highlight_fg = Some(color);
        self
    }

    /// Set tag background color
    pub fn tag_bg(mut self, color: Color) -> Self {
        self.tag_bg = Some(color);
        self
    }

    // =========================================================================
    // State queries
    // =========================================================================

    /// Check if dropdown is open
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Get selected indices
    pub fn get_selected_indices(&self) -> &[usize] {
        &self.selected
    }

    /// Get selected values
    pub fn get_selected_values(&self) -> Vec<&str> {
        self.selected
            .iter()
            .filter_map(|&i| self.options.get(i).map(|o| o.value.as_str()))
            .collect()
    }

    /// Get selected labels
    pub fn get_selected_labels(&self) -> Vec<&str> {
        self.selected
            .iter()
            .filter_map(|&i| self.options.get(i).map(|o| o.label.as_str()))
            .collect()
    }

    /// Get number of selected items
    pub fn selection_count(&self) -> usize {
        self.selected.len()
    }

    /// Check if an option is selected
    pub fn is_selected(&self, index: usize) -> bool {
        self.selected.contains(&index)
    }

    /// Check if can select more items
    pub fn can_select_more(&self) -> bool {
        match self.max_selections {
            Some(max) => self.selected.len() < max,
            None => true,
        }
    }

    /// Get number of options
    pub fn len(&self) -> usize {
        self.options.len()
    }

    /// Check if there are no options
    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }

    // =========================================================================
    // Selection manipulation
    // =========================================================================

    /// Open the dropdown
    pub fn open(&mut self) {
        self.open = true;
        self.tag_cursor = None;
        self.reset_filter();
    }

    /// Close the dropdown
    pub fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.reset_filter();
    }

    /// Toggle dropdown
    pub fn toggle(&mut self) {
        if self.open {
            self.close();
        } else {
            self.open();
        }
    }

    /// Select an option by index
    pub fn select_option(&mut self, index: usize) {
        if index >= self.options.len() {
            return;
        }
        if self.options[index].disabled {
            return;
        }
        if !self.selected.contains(&index) && self.can_select_more() {
            self.selected.push(index);
        }
    }

    /// Deselect an option by index
    pub fn deselect_option(&mut self, index: usize) {
        self.selected.retain(|&i| i != index);
    }

    /// Toggle selection of an option
    pub fn toggle_option(&mut self, index: usize) {
        if self.is_selected(index) {
            self.deselect_option(index);
        } else {
            self.select_option(index);
        }
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) {
        self.selected.clear();
    }

    /// Select all options
    pub fn select_all(&mut self) {
        self.selected = (0..self.options.len())
            .filter(|&i| !self.options[i].disabled)
            .collect();
        if let Some(max) = self.max_selections {
            self.selected.truncate(max);
        }
    }

    /// Remove the last selected tag
    pub fn remove_last_tag(&mut self) {
        self.selected.pop();
    }

    /// Remove tag at cursor position
    pub fn remove_tag_at_cursor(&mut self) {
        if let Some(cursor) = self.tag_cursor {
            if cursor < self.selected.len() {
                self.selected.remove(cursor);
                // Adjust cursor
                if self.selected.is_empty() {
                    self.tag_cursor = None;
                } else if cursor >= self.selected.len() {
                    self.tag_cursor = Some(self.selected.len() - 1);
                }
            }
        }
    }

    // =========================================================================
    // Navigation
    // =========================================================================

    /// Move dropdown cursor down
    pub fn cursor_down(&mut self) {
        if self.filtered.is_empty() {
            return;
        }
        self.dropdown_cursor = (self.dropdown_cursor + 1) % self.filtered.len();
    }

    /// Move dropdown cursor up
    pub fn cursor_up(&mut self) {
        if self.filtered.is_empty() {
            return;
        }
        self.dropdown_cursor = self
            .dropdown_cursor
            .checked_sub(1)
            .unwrap_or(self.filtered.len() - 1);
    }

    /// Move tag cursor left
    pub fn tag_cursor_left(&mut self) {
        if self.selected.is_empty() {
            return;
        }
        match self.tag_cursor {
            None => self.tag_cursor = Some(self.selected.len() - 1),
            Some(0) => {} // Already at start
            Some(pos) => self.tag_cursor = Some(pos - 1),
        }
    }

    /// Move tag cursor right
    pub fn tag_cursor_right(&mut self) {
        match self.tag_cursor {
            None => {}
            Some(pos) if pos >= self.selected.len() - 1 => self.tag_cursor = None,
            Some(pos) => self.tag_cursor = Some(pos + 1),
        }
    }

    /// Get current dropdown option index
    pub fn current_option(&self) -> Option<usize> {
        self.filtered.get(self.dropdown_cursor).copied()
    }

    // =========================================================================
    // Search/Filter
    // =========================================================================

    /// Get current search query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Set search query
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.update_filter();
    }

    /// Clear search query
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.reset_filter();
    }

    /// Reset filter to show all options
    fn reset_filter(&mut self) {
        self.filtered = (0..self.options.len()).collect();
        self.dropdown_cursor = 0;
    }

    /// Update filter based on query
    fn update_filter(&mut self) {
        if self.query.is_empty() {
            self.reset_filter();
            return;
        }

        let mut matches: Vec<(usize, i32)> = self
            .options
            .iter()
            .enumerate()
            .filter_map(|(i, opt)| fuzzy_match(&self.query, &opt.label).map(|m| (i, m.score)))
            .collect();

        matches.sort_by(|a, b| b.1.cmp(&a.1));
        self.filtered = matches.into_iter().map(|(i, _)| i).collect();
        self.dropdown_cursor = 0;
    }

    /// Get fuzzy match for an option
    pub fn get_match(&self, text: &str) -> Option<FuzzyMatch> {
        if self.query.is_empty() {
            None
        } else {
            fuzzy_match(&self.query, text)
        }
    }

    // =========================================================================
    // Key handling
    // =========================================================================

    /// Handle key input, returns true if needs redraw
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.state.disabled {
            return false;
        }

        match key {
            // Open/close/select
            Key::Enter => {
                if self.open {
                    if let Some(idx) = self.current_option() {
                        self.toggle_option(idx);
                    }
                } else {
                    self.open();
                }
                true
            }

            Key::Escape => {
                if self.open {
                    self.close();
                    true
                } else if self.tag_cursor.is_some() {
                    self.tag_cursor = None;
                    true
                } else {
                    false
                }
            }

            Key::Char(' ') if self.open && !self.searchable => {
                if let Some(idx) = self.current_option() {
                    self.toggle_option(idx);
                }
                true
            }

            // Dropdown navigation
            Key::Down | Key::Char('j') if self.open => {
                self.cursor_down();
                true
            }

            Key::Up | Key::Char('k') if self.open => {
                self.cursor_up();
                true
            }

            // Tag navigation
            Key::Left if !self.open => {
                self.tag_cursor_left();
                true
            }

            Key::Right if !self.open => {
                self.tag_cursor_right();
                true
            }

            // Delete tag
            Key::Backspace if !self.open => {
                if self.tag_cursor.is_some() {
                    self.remove_tag_at_cursor();
                } else if !self.selected.is_empty() {
                    self.remove_last_tag();
                }
                true
            }

            Key::Backspace if self.open && self.searchable => {
                self.query.pop();
                self.update_filter();
                true
            }

            Key::Delete if !self.open && self.tag_cursor.is_some() => {
                self.remove_tag_at_cursor();
                true
            }

            // Search typing
            Key::Char(c) if self.open && self.searchable => {
                self.query.push(*c);
                self.update_filter();
                true
            }

            // Select all
            Key::Char('a') if !self.open => {
                self.select_all();
                true
            }

            // Clear selection
            Key::Char('c') if !self.open => {
                self.clear_selection();
                true
            }

            _ => false,
        }
    }

    // =========================================================================
    // Display helpers
    // =========================================================================

    /// Calculate display width
    fn display_width(&self, max_width: u16) -> u16 {
        if let Some(w) = self.width {
            return w.min(max_width);
        }

        let max_option_len = self
            .options
            .iter()
            .map(|o| o.label.len())
            .max()
            .unwrap_or(self.placeholder.len());

        ((max_option_len + 4) as u16).min(max_width)
    }
}

impl Default for MultiSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl View for MultiSelect {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 5 || area.height < 1 {
            return;
        }

        let (fg, bg) =
            self.state
                .resolve_colors_interactive(ctx.style, Color::WHITE, Color::rgb(50, 50, 50));

        let width = self.display_width(area.width);

        // Draw background for header row
        for x in 0..width {
            let mut cell = Cell::new(' ');
            cell.fg = Some(fg);
            cell.bg = Some(bg);
            ctx.buffer.set(area.x + x, area.y, cell);
        }

        // Draw arrow
        let arrow = if self.open { '▲' } else { '▼' };
        ctx.draw_char(area.x + width - 1, area.y, arrow, fg);

        // Draw tags or placeholder
        let mut x = area.x;
        let max_x = area.x + width - 2; // Leave room for arrow

        if self.selected.is_empty() && !self.open {
            // Draw placeholder
            ctx.draw_text(x, area.y, &self.placeholder, Color::rgb(128, 128, 128));
        } else {
            // Draw tags
            for (i, &opt_idx) in self.selected.iter().enumerate() {
                if x >= max_x {
                    break;
                }

                if let Some(opt) = self.options.get(opt_idx) {
                    let label = &opt.label;
                    let tag_len = (label.chars().count() + 3) as u16; // "[label] "

                    if x + tag_len > max_x {
                        // Draw overflow indicator
                        ctx.draw_text(x, area.y, "...", Color::rgb(150, 150, 150));
                        break;
                    }

                    let is_tag_selected = self.tag_cursor == Some(i);
                    let tag_fg = if is_tag_selected {
                        Color::WHITE
                    } else {
                        Color::rgb(200, 200, 200)
                    };
                    let tag_bg_color = if is_tag_selected {
                        Color::rgb(100, 100, 200)
                    } else {
                        self.tag_bg.unwrap_or(Color::rgb(60, 60, 140))
                    };

                    // Draw tag with brackets
                    ctx.draw_char_bg(x, area.y, '[', tag_fg, tag_bg_color);
                    x += 1;

                    for ch in label.chars() {
                        if x >= max_x - 1 {
                            break;
                        }
                        ctx.draw_char_bg(x, area.y, ch, tag_fg, tag_bg_color);
                        x += 1;
                    }

                    ctx.draw_char_bg(x, area.y, ']', tag_fg, tag_bg_color);
                    x += 1;

                    // Space between tags
                    if x < max_x {
                        x += 1;
                    }
                }
            }

            // Draw search query if open
            if self.open && self.searchable && !self.query.is_empty() {
                let query_display = format!(" {}", self.query);
                ctx.draw_text(x.min(max_x), area.y, &query_display, Color::CYAN);
            }
        }

        // Draw dropdown if open
        if self.open && area.height > 1 {
            let max_visible = (area.height - 1) as usize;

            for (row, &opt_idx) in self.filtered.iter().enumerate().take(max_visible) {
                let y = area.y + 1 + row as u16;
                let is_cursor = row == self.dropdown_cursor;
                let is_selected = self.is_selected(opt_idx);

                if let Some(opt) = self.options.get(opt_idx) {
                    let (row_fg, row_bg) = if is_cursor {
                        (Color::WHITE, Color::rgb(80, 80, 150))
                    } else {
                        (fg, bg)
                    };

                    // Draw row background
                    for dx in 0..width {
                        let mut cell = Cell::new(' ');
                        cell.fg = Some(row_fg);
                        cell.bg = Some(row_bg);
                        ctx.buffer.set(area.x + dx, y, cell);
                    }

                    // Draw checkbox
                    let checkbox_str = if is_selected { "[x]" } else { "[ ]" };
                    ctx.draw_text_bg(area.x, y, checkbox_str, row_fg, row_bg);

                    // Draw label with highlight
                    let match_indices: Vec<usize> = self
                        .get_match(&opt.label)
                        .map(|m| m.indices)
                        .unwrap_or_default();

                    let label_x = area.x + 4;
                    for (j, ch) in opt.label.chars().enumerate() {
                        if label_x + j as u16 >= area.x + width {
                            break;
                        }

                        let char_fg = if match_indices.contains(&j) {
                            self.highlight_fg.unwrap_or(Color::YELLOW)
                        } else if opt.disabled {
                            Color::rgb(100, 100, 100)
                        } else {
                            row_fg
                        };

                        ctx.draw_char_bg(label_x + j as u16, y, ch, char_fg, row_bg);
                    }
                }
            }
        }
    }

    impl_view_meta!("MultiSelect");
}

impl_styled_view!(MultiSelect);
impl_widget_builders!(MultiSelect);

// =============================================================================
// Helper functions
// =============================================================================

/// Create a basic multi-select widget
///
/// # Example
/// ```rust,ignore
/// let select = multi_select()
///     .option("Apple")
///     .option("Banana");
/// ```
pub fn multi_select() -> MultiSelect {
    MultiSelect::new()
}

/// Create a multi-select from an iterable of strings
///
/// # Example
/// ```rust,ignore
/// let fruits = vec!["Apple", "Banana", "Cherry"];
/// let select = multi_select_from(fruits);
/// ```
pub fn multi_select_from<I, S>(items: I) -> MultiSelect
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    MultiSelect::new().options(items.into_iter().map(|s| s.into()).collect())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_multi_select_new() {
        let select = MultiSelect::new();
        assert!(select.is_empty());
        assert!(!select.is_open());
        assert_eq!(select.selection_count(), 0);
    }

    #[test]
    fn test_multi_select_options() {
        let select = multi_select()
            .option("Apple")
            .option("Banana")
            .option("Cherry");

        assert_eq!(select.len(), 3);
        assert!(!select.is_selected(0));
    }

    #[test]
    fn test_multi_select_selection() {
        let mut select = multi_select().options(vec!["A", "B", "C"]);

        select.select_option(0);
        assert!(select.is_selected(0));
        assert_eq!(select.selection_count(), 1);

        select.select_option(2);
        assert!(select.is_selected(2));
        assert_eq!(select.selection_count(), 2);

        select.deselect_option(0);
        assert!(!select.is_selected(0));
        assert_eq!(select.selection_count(), 1);
    }

    #[test]
    fn test_multi_select_toggle() {
        let mut select = multi_select().options(vec!["A", "B"]);

        select.toggle_option(0);
        assert!(select.is_selected(0));

        select.toggle_option(0);
        assert!(!select.is_selected(0));
    }

    #[test]
    fn test_multi_select_max_selections() {
        let mut select = multi_select()
            .options(vec!["A", "B", "C"])
            .max_selections(2);

        select.select_option(0);
        select.select_option(1);
        assert!(select.can_select_more() == false);

        select.select_option(2); // Should not add
        assert!(!select.is_selected(2));
        assert_eq!(select.selection_count(), 2);
    }

    #[test]
    fn test_multi_select_get_values() {
        let mut select = multi_select().options(vec!["Apple", "Banana", "Cherry"]);

        select.select_option(0);
        select.select_option(2);

        let values = select.get_selected_values();
        assert_eq!(values, vec!["Apple", "Cherry"]);
    }

    #[test]
    fn test_multi_select_navigation() {
        let mut select = multi_select().options(vec!["A", "B", "C"]);
        select.open();

        assert_eq!(select.dropdown_cursor, 0);

        select.cursor_down();
        assert_eq!(select.dropdown_cursor, 1);

        select.cursor_down();
        assert_eq!(select.dropdown_cursor, 2);

        select.cursor_down(); // Wraps
        assert_eq!(select.dropdown_cursor, 0);

        select.cursor_up(); // Wraps backward
        assert_eq!(select.dropdown_cursor, 2);
    }

    #[test]
    fn test_multi_select_tag_navigation() {
        let mut select = multi_select().options(vec!["A", "B", "C"]);
        select.select_option(0);
        select.select_option(1);
        select.select_option(2);

        assert!(select.tag_cursor.is_none());

        select.tag_cursor_left();
        assert_eq!(select.tag_cursor, Some(2));

        select.tag_cursor_left();
        assert_eq!(select.tag_cursor, Some(1));

        select.tag_cursor_right();
        assert_eq!(select.tag_cursor, Some(2));

        select.tag_cursor_right();
        assert!(select.tag_cursor.is_none());
    }

    #[test]
    fn test_multi_select_remove_tag() {
        let mut select = multi_select().options(vec!["A", "B", "C"]);
        select.select_option(0);
        select.select_option(1);
        select.select_option(2);

        select.tag_cursor = Some(1);
        select.remove_tag_at_cursor();

        assert_eq!(select.selection_count(), 2);
        assert!(select.is_selected(0));
        assert!(!select.is_selected(1));
        assert!(select.is_selected(2));
    }

    #[test]
    fn test_multi_select_search() {
        let mut select = multi_select()
            .options(vec!["Apple", "Apricot", "Banana", "Blueberry"])
            .searchable(true);

        select.open();
        select.set_query("ap");

        assert_eq!(select.filtered.len(), 2);
        assert!(select.filtered.contains(&0)); // Apple
        assert!(select.filtered.contains(&1)); // Apricot
    }

    #[test]
    fn test_multi_select_key_handling() {
        let mut select = multi_select().options(vec!["A", "B", "C"]);

        // Open
        select.handle_key(&Key::Enter);
        assert!(select.is_open());

        // Navigate
        select.handle_key(&Key::Down);
        assert_eq!(select.dropdown_cursor, 1);

        // Select
        select.handle_key(&Key::Enter);
        assert!(select.is_selected(1));

        // Close
        select.handle_key(&Key::Escape);
        assert!(!select.is_open());
    }

    #[test]
    fn test_multi_select_disabled_option() {
        let mut select = multi_select()
            .option_detailed(MultiSelectOption::new("Disabled", "disabled").disabled(true));

        select.select_option(0);
        assert!(!select.is_selected(0)); // Can't select disabled
    }

    #[test]
    fn test_multi_select_select_all() {
        let mut select = multi_select().options(vec!["A", "B", "C"]);

        select.select_all();
        assert_eq!(select.selection_count(), 3);

        select.clear_selection();
        assert_eq!(select.selection_count(), 0);
    }

    #[test]
    fn test_multi_select_pre_selected() {
        let select = multi_select()
            .options(vec!["A", "B", "C"])
            .selected_indices(vec![0, 2]);

        assert!(select.is_selected(0));
        assert!(!select.is_selected(1));
        assert!(select.is_selected(2));
    }

    #[test]
    fn test_multi_select_from() {
        let select = multi_select_from(vec!["X", "Y", "Z"]);
        assert_eq!(select.len(), 3);
    }

    #[test]
    fn test_multi_select_render() {
        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut select = multi_select()
            .options(vec!["Apple", "Banana"])
            .focused(true);
        select.select_option(0);

        select.render(&mut ctx);

        // Should show tag [Apple]
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '[');
    }

    #[test]
    fn test_multi_select_render_dropdown() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut select = multi_select()
            .options(vec!["Apple", "Banana"])
            .focused(true);
        select.open();

        select.render(&mut ctx);

        // Should show checkbox on second row
        assert_eq!(buffer.get(0, 1).unwrap().symbol, '[');
    }
}
