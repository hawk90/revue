//! Select/Dropdown widget for choosing from a list of options

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{fuzzy_match, FuzzyMatch, Selection};
use crate::{impl_props_builders, impl_styled_view};

/// A select/dropdown widget with optional fuzzy search
#[derive(Clone, Debug)]
pub struct Select {
    options: Vec<String>,
    /// Selection state for options (uses Selection utility)
    selection: Selection,
    open: bool,
    placeholder: String,
    fg: Option<Color>,
    bg: Option<Color>,
    selected_fg: Option<Color>,
    selected_bg: Option<Color>,
    highlight_fg: Option<Color>,
    width: Option<u16>,
    /// Search query for filtering options
    query: String,
    /// Enable fuzzy search when typing
    searchable: bool,
    /// Filtered indices (into options) based on query
    filtered: Vec<usize>,
    /// Selection state for filtered results (uses Selection utility)
    filtered_selection: Selection,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
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
            query: String::new(),
            searchable: false,
            filtered: Vec::new(),
            filtered_selection: Selection::new(0),
            props: WidgetProps::new(),
        }
    }

    /// Set options
    pub fn options(mut self, options: Vec<impl Into<String>>) -> Self {
        self.options = options.into_iter().map(|o| o.into()).collect();
        self.selection.set_len(self.options.len());
        self.reset_filter();
        self
    }

    /// Add a single option
    pub fn option(mut self, option: impl Into<String>) -> Self {
        self.options.push(option.into());
        self.selection.set_len(self.options.len());
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

    /// Set selected index
    pub fn selected(mut self, index: usize) -> Self {
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

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.selection.index
    }

    /// Get selected value
    pub fn value(&self) -> Option<&str> {
        self.options.get(self.selection.index).map(|s| s.as_str())
    }

    /// Check if dropdown is open
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Toggle dropdown open/close
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

    /// Select next option
    pub fn select_next(&mut self) {
        self.selection.next();
    }

    /// Select previous option
    pub fn select_prev(&mut self) {
        self.selection.prev();
    }

    /// Select first option
    pub fn select_first(&mut self) {
        self.selection.first();
    }

    /// Select last option
    pub fn select_last(&mut self) {
        self.selection.last();
    }

    // --- Fuzzy search methods ---

    /// Get current search query
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Set search query and update filter
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.update_filter();
    }

    /// Clear search query
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.reset_filter();
    }

    /// Check if searchable mode is enabled
    pub fn is_searchable(&self) -> bool {
        self.searchable
    }

    /// Get filtered options (indices into original options)
    pub fn filtered_options(&self) -> &[usize] {
        &self.filtered
    }

    /// Get number of visible (filtered) options
    pub fn visible_count(&self) -> usize {
        if self.query.is_empty() {
            self.options.len()
        } else {
            self.filtered.len()
        }
    }

    /// Reset filter to show all options
    fn reset_filter(&mut self) {
        self.filtered = (0..self.options.len()).collect();
        self.filtered_selection.set_len(self.filtered.len());
        self.filtered_selection.first();
    }

    /// Update filter based on current query
    fn update_filter(&mut self) {
        if self.query.is_empty() {
            self.reset_filter();
            return;
        }

        // Collect matches with scores
        let mut matches: Vec<(usize, i32)> = self
            .options
            .iter()
            .enumerate()
            .filter_map(|(i, opt)| fuzzy_match(&self.query, opt).map(|m| (i, m.score)))
            .collect();

        // Sort by score descending
        matches.sort_by(|a, b| b.1.cmp(&a.1));

        self.filtered = matches.into_iter().map(|(i, _)| i).collect();
        self.filtered_selection.set_len(self.filtered.len());
        self.filtered_selection.first();

        // Update selected to first filtered item if available
        if let Some(&first) = self.filtered.first() {
            self.selection.set(first);
        }
    }

    /// Get fuzzy match for an option
    pub fn get_match(&self, option: &str) -> Option<FuzzyMatch> {
        if self.query.is_empty() {
            None
        } else {
            fuzzy_match(&self.query, option)
        }
    }

    /// Select next in filtered results
    fn select_next_filtered(&mut self) {
        if !self.filtered.is_empty() {
            self.filtered_selection.next();
            self.selection
                .set(self.filtered[self.filtered_selection.index]);
        }
    }

    /// Select previous in filtered results
    fn select_prev_filtered(&mut self) {
        if !self.filtered.is_empty() {
            self.filtered_selection.prev();
            self.selection
                .set(self.filtered[self.filtered_selection.index]);
        }
    }

    /// Handle key input, returns true if selection changed
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        match key {
            Key::Enter => {
                if self.open {
                    self.close();
                    self.clear_query();
                } else {
                    self.open();
                }
                false
            }
            Key::Char(' ') if !self.searchable => {
                self.toggle();
                false
            }
            Key::Up | Key::Char('k') if self.open && !self.searchable => {
                let old = self.selection.index;
                self.select_prev();
                old != self.selection.index
            }
            Key::Down | Key::Char('j') if self.open && !self.searchable => {
                let old = self.selection.index;
                self.select_next();
                old != self.selection.index
            }
            Key::Up if self.open && self.searchable => {
                let old = self.selection.index;
                if self.query.is_empty() {
                    self.select_prev();
                } else {
                    self.select_prev_filtered();
                }
                old != self.selection.index
            }
            Key::Down if self.open && self.searchable => {
                let old = self.selection.index;
                if self.query.is_empty() {
                    self.select_next();
                } else {
                    self.select_next_filtered();
                }
                old != self.selection.index
            }
            Key::Escape if self.open => {
                self.close();
                self.clear_query();
                false
            }
            Key::Backspace if self.open && self.searchable => {
                self.query.pop();
                self.update_filter();
                true
            }
            Key::Char(c) if self.open && self.searchable => {
                self.query.push(*c);
                self.update_filter();
                true
            }
            Key::Home if self.open => {
                let old = self.selection.index;
                self.select_first();
                old != self.selection.index
            }
            Key::End if self.open => {
                let old = self.selection.index;
                self.select_last();
                old != self.selection.index
            }
            _ => false,
        }
    }

    /// Get number of options
    pub fn len(&self) -> usize {
        self.options.len()
    }

    /// Check if select has no options
    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }

    /// Calculate display width
    fn display_width(&self, max_width: u16) -> u16 {
        if let Some(w) = self.width {
            return w.min(max_width);
        }

        let max_option_len = self
            .options
            .iter()
            .map(|o| o.len())
            .max()
            .unwrap_or(self.placeholder.len());

        // +4 for "▼ " prefix and " " suffix and border
        ((max_option_len + 4) as u16).min(max_width)
    }
}

impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Select {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 {
            return;
        }

        let width = self.display_width(area.width);
        let text_width = (width - 2) as usize;

        // Render the closed/header row
        let display_text = if self.open && self.searchable && !self.query.is_empty() {
            // Show search query when searching
            &self.query
        } else {
            self.value().unwrap_or(&self.placeholder)
        };
        let arrow = if self.open { "▲" } else { "▼" };

        // Draw background for header
        for x in 0..width {
            let mut cell = Cell::new(' ');
            cell.fg = self.fg;
            cell.bg = self.bg;
            ctx.buffer.set(area.x + x, area.y, cell);
        }

        // Draw arrow (or search icon when searching)
        let icon = if self.open && self.searchable {
            "🔍".chars().next().unwrap_or('?')
        } else {
            arrow.chars().next().unwrap_or('▼')
        };
        let mut cell = Cell::new(icon);
        cell.fg = self.fg;
        cell.bg = self.bg;
        ctx.buffer.set(area.x, area.y, cell);

        // Draw text
        let truncated: String = display_text.chars().take(text_width).collect();
        for (i, ch) in truncated.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = self.fg;
            cell.bg = self.bg;
            ctx.buffer.set(area.x + 2 + i as u16, area.y, cell);
        }

        // If open, render dropdown options
        if self.open && area.height > 1 {
            let max_visible = (area.height - 1) as usize;

            // Determine which options to show
            let visible_options: Vec<(usize, &String)> = if self.query.is_empty() {
                self.options.iter().enumerate().collect()
            } else {
                self.filtered
                    .iter()
                    .filter_map(|&i| self.options.get(i).map(|opt| (i, opt)))
                    .collect()
            };

            for (row, (option_idx, option)) in visible_options.iter().enumerate().take(max_visible)
            {
                let y = area.y + 1 + row as u16;
                let is_selected = self.selection.is_selected(*option_idx);

                let (fg, bg) = if is_selected {
                    (self.selected_fg, self.selected_bg)
                } else {
                    (self.fg, self.bg)
                };

                // Draw background
                for x in 0..width {
                    let mut cell = Cell::new(' ');
                    cell.fg = fg;
                    cell.bg = bg;
                    ctx.buffer.set(area.x + x, y, cell);
                }

                // Draw selection indicator
                let indicator = if is_selected { '›' } else { ' ' };
                let mut cell = Cell::new(indicator);
                cell.fg = fg;
                cell.bg = bg;
                ctx.buffer.set(area.x, y, cell);

                // Get fuzzy match indices for highlighting
                let match_indices: Vec<usize> = self
                    .get_match(option)
                    .map(|m| m.indices)
                    .unwrap_or_default();

                // Draw option text with highlighting
                let truncated: String = option.chars().take(text_width).collect();
                for (j, ch) in truncated.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.bg = bg;

                    // Highlight matched characters
                    if match_indices.contains(&j) {
                        cell.fg = self.highlight_fg;
                    } else {
                        cell.fg = fg;
                    }

                    ctx.buffer.set(area.x + 2 + j as u16, y, cell);
                }
            }
        }
    }

    crate::impl_view_meta!("Select");
}

impl_styled_view!(Select);
impl_props_builders!(Select);

/// Helper function to create a select widget
pub fn select() -> Select {
    Select::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::StyledView;

    #[test]
    fn test_select_new() {
        let s = Select::new();
        assert!(s.is_empty());
        assert_eq!(s.selected_index(), 0);
        assert!(!s.is_open());
    }

    #[test]
    fn test_select_with_options() {
        let s = Select::new()
            .option("Apple")
            .option("Banana")
            .option("Cherry");

        assert_eq!(s.len(), 3);
        assert_eq!(s.value(), Some("Apple"));
    }

    #[test]
    fn test_select_options_vec() {
        let s = Select::new().options(vec!["One", "Two", "Three"]);

        assert_eq!(s.len(), 3);
        assert_eq!(s.value(), Some("One"));
    }

    #[test]
    fn test_select_navigation() {
        let mut s = Select::new().options(vec!["A", "B", "C"]);

        assert_eq!(s.selected_index(), 0);

        s.select_next();
        assert_eq!(s.selected_index(), 1);

        s.select_next();
        assert_eq!(s.selected_index(), 2);

        s.select_next(); // Wraps around
        assert_eq!(s.selected_index(), 0);

        s.select_prev(); // Wraps around backward
        assert_eq!(s.selected_index(), 2);

        s.select_first();
        assert_eq!(s.selected_index(), 0);

        s.select_last();
        assert_eq!(s.selected_index(), 2);
    }

    #[test]
    fn test_select_toggle() {
        let mut s = Select::new();
        assert!(!s.is_open());

        s.toggle();
        assert!(s.is_open());

        s.toggle();
        assert!(!s.is_open());

        s.open();
        assert!(s.is_open());

        s.close();
        assert!(!s.is_open());
    }

    #[test]
    fn test_select_handle_key() {
        use crate::event::Key;

        let mut s = Select::new().options(vec!["X", "Y", "Z"]);

        // Toggle open
        s.handle_key(&Key::Enter);
        assert!(s.is_open());

        // Navigate down
        let changed = s.handle_key(&Key::Down);
        assert!(changed);
        assert_eq!(s.selected_index(), 1);

        // Navigate up
        let changed = s.handle_key(&Key::Up);
        assert!(changed);
        assert_eq!(s.selected_index(), 0);

        // Close with Escape
        s.handle_key(&Key::Escape);
        assert!(!s.is_open());
    }

    #[test]
    fn test_select_render_closed() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let s = Select::new()
            .options(vec!["Option 1", "Option 2"])
            .placeholder("Choose...");

        s.render(&mut ctx);

        // Should show arrow
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '▼');
    }

    #[test]
    fn test_select_render_open() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut s = Select::new().options(vec!["Apple", "Banana"]);
        s.open();

        s.render(&mut ctx);

        // Should show up arrow when open
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '▲');
        // First option should have selection indicator
        assert_eq!(buffer.get(0, 1).unwrap().symbol, '›');
    }

    #[test]
    fn test_select_helper() {
        let s = select().option("Test").placeholder("Pick one");

        assert_eq!(s.len(), 1);
    }

    #[test]
    fn test_select_empty_value() {
        let s = Select::new();
        assert_eq!(s.value(), None);
    }

    #[test]
    fn test_select_searchable() {
        let mut s = Select::new()
            .options(vec!["Apple", "Apricot", "Banana", "Blueberry", "Cherry"])
            .searchable(true);

        assert!(s.is_searchable());
        assert_eq!(s.query(), "");

        // Set query
        s.set_query("ap");
        assert_eq!(s.query(), "ap");

        // Should filter to Apple and Apricot
        assert_eq!(s.visible_count(), 2);
        assert!(s.filtered_options().contains(&0)); // Apple
        assert!(s.filtered_options().contains(&1)); // Apricot

        // Clear query
        s.clear_query();
        assert_eq!(s.query(), "");
        assert_eq!(s.visible_count(), 5);
    }

    #[test]
    fn test_select_fuzzy_filter() {
        let mut s = Select::new()
            .options(vec!["Save File", "Open File", "Close Window", "Save All"])
            .searchable(true);

        // Fuzzy match "sf" -> "Save File", "Save All"
        s.set_query("sf");
        assert!(s.filtered_options().contains(&0)); // Save File
        assert!(!s.filtered_options().contains(&1)); // Open File - no match
        assert!(!s.filtered_options().contains(&2)); // Close Window - no match

        // Fuzzy match "ow" -> "Open Window" would match, "Close Window"
        s.set_query("ow");
        assert!(s.filtered_options().contains(&2)); // Close Window
    }

    #[test]
    fn test_select_searchable_keys() {
        use crate::event::Key;

        let mut s = Select::new()
            .options(vec!["Apple", "Banana", "Cherry"])
            .searchable(true);

        // Open
        s.handle_key(&Key::Enter);
        assert!(s.is_open());

        // Type 'a'
        s.handle_key(&Key::Char('a'));
        assert_eq!(s.query(), "a");
        assert_eq!(s.visible_count(), 2); // Apple and Banana (both have 'a')

        // Type 'p' -> "ap" only matches Apple
        s.handle_key(&Key::Char('p'));
        assert_eq!(s.query(), "ap");
        assert_eq!(s.visible_count(), 1); // Only Apple

        // Backspace
        s.handle_key(&Key::Backspace);
        assert_eq!(s.query(), "a");

        // Close and clear
        s.handle_key(&Key::Escape);
        assert!(!s.is_open());
        assert_eq!(s.query(), ""); // Query cleared on close
    }

    #[test]
    fn test_select_get_match() {
        let mut s = Select::new().options(vec!["Hello World"]).searchable(true);

        // No match when no query
        assert!(s.get_match("Hello World").is_none());

        // Set query
        s.set_query("hw");

        // Should have match with indices
        let m = s.get_match("Hello World");
        assert!(m.is_some());
        let m = m.unwrap();
        assert!(m.indices.contains(&0)); // H
        assert!(m.indices.contains(&6)); // W
    }

    // CSS integration tests
    #[test]
    fn test_select_css_id() {
        use crate::widget::View;

        let select = Select::new()
            .options(vec!["A", "B"])
            .element_id("country-select");
        assert_eq!(View::id(&select), Some("country-select"));

        let meta = select.meta();
        assert_eq!(meta.id, Some("country-select".to_string()));
    }

    #[test]
    fn test_select_css_classes() {
        let select = Select::new()
            .options(vec!["A", "B"])
            .class("dropdown")
            .class("form-control");

        assert!(select.has_class("dropdown"));
        assert!(select.has_class("form-control"));
        assert!(!select.has_class("hidden"));

        let meta = select.meta();
        assert!(meta.classes.contains("dropdown"));
        assert!(meta.classes.contains("form-control"));
    }

    #[test]
    fn test_select_styled_view() {
        use crate::widget::View;

        let mut select = Select::new().options(vec!["A", "B"]);

        select.set_id("test-select");
        assert_eq!(View::id(&select), Some("test-select"));

        select.add_class("active");
        assert!(select.has_class("active"));

        select.toggle_class("active");
        assert!(!select.has_class("active"));

        select.toggle_class("open");
        assert!(select.has_class("open"));

        select.remove_class("open");
        assert!(!select.has_class("open"));
    }
}
