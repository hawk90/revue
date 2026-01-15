//! Combobox/Autocomplete widget combining text input with searchable dropdown
//!
//! Features:
//! - Text input with dropdown suggestions
//! - Multiple filter modes (fuzzy, prefix, exact, contains)
//! - Keyboard navigation (arrow, enter, escape)
//! - Highlight matching text
//! - Allow custom values (free-form input)
//! - Multiple selection variant
//! - Loading and empty states

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{fuzzy_match, FuzzyMatch};
use crate::{impl_props_builders, impl_styled_view};

/// Filter mode for matching options
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FilterMode {
    /// Fuzzy matching (e.g., "hw" matches "Hello World")
    #[default]
    Fuzzy,
    /// Prefix matching (e.g., "Hel" matches "Hello")
    Prefix,
    /// Exact matching (case-insensitive)
    Exact,
    /// Contains matching (substring anywhere)
    Contains,
}

/// Option item for combobox
#[derive(Clone, Debug)]
pub struct ComboOption {
    /// Display label
    pub label: String,
    /// Optional value (defaults to label)
    pub value: Option<String>,
    /// Whether this option is disabled
    pub disabled: bool,
    /// Optional group/category
    pub group: Option<String>,
}

impl ComboOption {
    /// Create a new option
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: None,
            disabled: false,
            group: None,
        }
    }

    /// Set the value (different from label)
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Mark as disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set group/category
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Get the value (label if no explicit value)
    pub fn get_value(&self) -> &str {
        self.value.as_deref().unwrap_or(&self.label)
    }
}

impl<T: Into<String>> From<T> for ComboOption {
    fn from(s: T) -> Self {
        ComboOption::new(s)
    }
}

/// A combobox widget with text input and searchable dropdown
#[derive(Clone, Debug)]
pub struct Combobox {
    /// Available options
    options: Vec<ComboOption>,
    /// Current input value
    input: String,
    /// Cursor position in input
    cursor: usize,
    /// Whether dropdown is open
    open: bool,
    /// Selected index in filtered list
    selected_idx: usize,
    /// Filtered option indices
    filtered: Vec<usize>,
    /// Filter mode
    filter_mode: FilterMode,
    /// Allow custom values not in options
    allow_custom: bool,
    /// Multiple selection mode
    multi_select: bool,
    /// Selected values (for multi-select)
    selected_values: Vec<String>,
    /// Placeholder text
    placeholder: String,
    /// Loading state
    loading: bool,
    /// Loading text
    loading_text: String,
    /// Empty state text
    empty_text: String,
    /// Max visible options in dropdown
    max_visible: usize,
    /// Scroll offset in dropdown
    scroll_offset: usize,
    // Styling
    fg: Option<Color>,
    bg: Option<Color>,
    input_fg: Option<Color>,
    input_bg: Option<Color>,
    selected_fg: Option<Color>,
    selected_bg: Option<Color>,
    highlight_fg: Option<Color>,
    disabled_fg: Option<Color>,
    /// Fixed width
    width: Option<u16>,
    /// CSS styling properties
    props: WidgetProps,
}

impl Combobox {
    /// Create a new combobox
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            input: String::new(),
            cursor: 0,
            open: false,
            selected_idx: 0,
            filtered: Vec::new(),
            filter_mode: FilterMode::Fuzzy,
            allow_custom: false,
            multi_select: false,
            selected_values: Vec::new(),
            placeholder: "Type to search...".to_string(),
            loading: false,
            loading_text: "Loading...".to_string(),
            empty_text: "No results".to_string(),
            max_visible: 5,
            scroll_offset: 0,
            fg: None,
            bg: None,
            input_fg: None,
            input_bg: None,
            selected_fg: Some(Color::WHITE),
            selected_bg: Some(Color::BLUE),
            highlight_fg: Some(Color::YELLOW),
            disabled_fg: Some(Color::rgb(128, 128, 128)),
            width: None,
            props: WidgetProps::new(),
        }
    }

    /// Set options from strings
    pub fn options<I, S>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.options = options.into_iter().map(|s| ComboOption::new(s)).collect();
        self.update_filter();
        self
    }

    /// Set options from ComboOption items
    pub fn options_with<I>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = ComboOption>,
    {
        self.options = options.into_iter().collect();
        self.update_filter();
        self
    }

    /// Add a single option
    pub fn option(mut self, option: impl Into<ComboOption>) -> Self {
        self.options.push(option.into());
        self.update_filter();
        self
    }

    /// Set filter mode
    pub fn filter_mode(mut self, mode: FilterMode) -> Self {
        self.filter_mode = mode;
        self.update_filter();
        self
    }

    /// Allow custom values not in the options list
    pub fn allow_custom(mut self, allow: bool) -> Self {
        self.allow_custom = allow;
        self
    }

    /// Enable multiple selection mode
    pub fn multi_select(mut self, multi: bool) -> Self {
        self.multi_select = multi;
        self
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set loading state
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Set loading text
    pub fn loading_text(mut self, text: impl Into<String>) -> Self {
        self.loading_text = text.into();
        self
    }

    /// Set empty state text
    pub fn empty_text(mut self, text: impl Into<String>) -> Self {
        self.empty_text = text.into();
        self
    }

    /// Set max visible options
    pub fn max_visible(mut self, count: usize) -> Self {
        self.max_visible = count.max(1);
        self
    }

    /// Set fixed width
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

    /// Set input field colors
    pub fn input_style(mut self, fg: Color, bg: Color) -> Self {
        self.input_fg = Some(fg);
        self.input_bg = Some(bg);
        self
    }

    /// Set selected option colors
    pub fn selected_style(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = Some(fg);
        self.selected_bg = Some(bg);
        self
    }

    /// Set highlight color for matched characters
    pub fn highlight_fg(mut self, color: Color) -> Self {
        self.highlight_fg = Some(color);
        self
    }

    /// Set initial input value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.input = value.into();
        self.cursor = self.input.chars().count();
        self.update_filter();
        self
    }

    /// Set pre-selected values (for multi-select)
    pub fn selected_values(mut self, values: Vec<String>) -> Self {
        self.selected_values = values;
        self
    }

    // ─────────────────────────────────────────────────────────────────────────
    // State getters
    // ─────────────────────────────────────────────────────────────────────────

    /// Get current input text
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Get the selected value (for single-select mode)
    pub fn selected_value(&self) -> Option<&str> {
        if self.multi_select {
            return None;
        }

        // If input matches an option, return that option's value
        if let Some(opt) = self
            .options
            .iter()
            .find(|o| o.label == self.input || o.get_value() == self.input)
        {
            return Some(opt.get_value());
        }

        // Allow custom values if enabled
        if self.allow_custom && !self.input.is_empty() {
            Some(self.input.as_str())
        } else {
            None
        }
    }

    /// Get all selected values (for multi-select mode)
    pub fn selected_values_ref(&self) -> &[String] {
        &self.selected_values
    }

    /// Check if dropdown is open
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Check if loading
    pub fn is_loading(&self) -> bool {
        self.loading
    }

    /// Get number of options
    pub fn option_count(&self) -> usize {
        self.options.len()
    }

    /// Get number of filtered options
    pub fn filtered_count(&self) -> usize {
        self.filtered.len()
    }

    /// Check if a value is selected (for multi-select)
    pub fn is_selected(&self, value: &str) -> bool {
        self.selected_values.iter().any(|v| v == value)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Actions
    // ─────────────────────────────────────────────────────────────────────────

    /// Open the dropdown
    pub fn open_dropdown(&mut self) {
        self.open = true;
        self.update_filter();
    }

    /// Close the dropdown
    pub fn close_dropdown(&mut self) {
        self.open = false;
        self.selected_idx = 0;
        self.scroll_offset = 0;
    }

    /// Toggle dropdown
    pub fn toggle_dropdown(&mut self) {
        if self.open {
            self.close_dropdown();
        } else {
            self.open_dropdown();
        }
    }

    /// Set input value
    pub fn set_input(&mut self, value: impl Into<String>) {
        self.input = value.into();
        self.cursor = self.input.chars().count();
        self.update_filter();
        if !self.input.is_empty() {
            self.open = true;
        }
    }

    /// Clear input
    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor = 0;
        self.update_filter();
    }

    /// Select the currently highlighted option
    pub fn select_current(&mut self) -> bool {
        if self.filtered.is_empty() {
            return false;
        }

        if let Some(&option_idx) = self.filtered.get(self.selected_idx) {
            let option = &self.options[option_idx];
            if option.disabled {
                return false;
            }

            if self.multi_select {
                let value = option.get_value().to_string();
                if self.is_selected(&value) {
                    self.selected_values.retain(|v| v != &value);
                } else {
                    self.selected_values.push(value);
                }
            } else {
                self.input = option.label.clone();
                self.cursor = self.input.chars().count();
                self.close_dropdown();
            }
            return true;
        }
        false
    }

    /// Select next option in filtered list
    pub fn select_next(&mut self) {
        if self.filtered.is_empty() {
            return;
        }

        self.selected_idx = (self.selected_idx + 1) % self.filtered.len();
        self.ensure_visible();
    }

    /// Select previous option in filtered list
    pub fn select_prev(&mut self) {
        if self.filtered.is_empty() {
            return;
        }

        self.selected_idx = self
            .selected_idx
            .checked_sub(1)
            .unwrap_or(self.filtered.len() - 1);
        self.ensure_visible();
    }

    /// Select first option
    pub fn select_first(&mut self) {
        self.selected_idx = 0;
        self.scroll_offset = 0;
    }

    /// Select last option
    pub fn select_last(&mut self) {
        if !self.filtered.is_empty() {
            self.selected_idx = self.filtered.len() - 1;
            self.ensure_visible();
        }
    }

    /// Ensure selected option is visible in viewport
    fn ensure_visible(&mut self) {
        if self.selected_idx < self.scroll_offset {
            self.scroll_offset = self.selected_idx;
        } else if self.selected_idx >= self.scroll_offset + self.max_visible {
            self.scroll_offset = self.selected_idx - self.max_visible + 1;
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Input handling
    // ─────────────────────────────────────────────────────────────────────────

    /// Insert character at cursor
    pub fn insert_char(&mut self, c: char) {
        let byte_idx = self.char_to_byte_index(self.cursor);
        self.input.insert(byte_idx, c);
        self.cursor += 1;
        self.update_filter();
        self.open = true;
    }

    /// Delete character before cursor (backspace)
    pub fn delete_backward(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            let byte_idx = self.char_to_byte_index(self.cursor);
            if let Some((_, ch)) = self.input.char_indices().nth(self.cursor) {
                self.input.drain(byte_idx..byte_idx + ch.len_utf8());
            }
            self.update_filter();
        }
    }

    /// Delete character at cursor (delete)
    pub fn delete_forward(&mut self) {
        let char_count = self.input.chars().count();
        if self.cursor < char_count {
            let byte_idx = self.char_to_byte_index(self.cursor);
            if let Some((_, ch)) = self.input.char_indices().nth(self.cursor) {
                self.input.drain(byte_idx..byte_idx + ch.len_utf8());
            }
            self.update_filter();
        }
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        let char_count = self.input.chars().count();
        if self.cursor < char_count {
            self.cursor += 1;
        }
    }

    /// Move cursor to start
    pub fn move_to_start(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to end
    pub fn move_to_end(&mut self) {
        self.cursor = self.input.chars().count();
    }

    /// Handle key event
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        match key {
            Key::Char(c) => {
                self.insert_char(*c);
                true
            }
            Key::Backspace => {
                self.delete_backward();
                true
            }
            Key::Delete => {
                self.delete_forward();
                true
            }
            Key::Left => {
                self.move_left();
                false
            }
            Key::Right => {
                self.move_right();
                false
            }
            Key::Home => {
                self.move_to_start();
                false
            }
            Key::End => {
                self.move_to_end();
                false
            }
            Key::Up if self.open => {
                self.select_prev();
                true
            }
            Key::Down if self.open => {
                self.select_next();
                true
            }
            Key::Down if !self.open => {
                self.open_dropdown();
                true
            }
            Key::Enter if self.open => {
                self.select_current();
                true
            }
            Key::Enter if !self.open && self.allow_custom => {
                // Accept custom value
                true
            }
            Key::Escape if self.open => {
                self.close_dropdown();
                true
            }
            Key::Tab if self.open && !self.filtered.is_empty() => {
                // Tab completion: fill with highlighted option
                if let Some(&option_idx) = self.filtered.get(self.selected_idx) {
                    self.input = self.options[option_idx].label.clone();
                    self.cursor = self.input.chars().count();
                    self.update_filter();
                }
                true
            }
            _ => false,
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Filtering
    // ─────────────────────────────────────────────────────────────────────────

    /// Update filtered options based on input
    fn update_filter(&mut self) {
        if self.input.is_empty() {
            // Show all options when input is empty
            self.filtered = (0..self.options.len()).collect();
            self.selected_idx = 0;
            self.scroll_offset = 0;
            return;
        }

        let query = self.input.to_lowercase();
        let mut matches: Vec<(usize, i32)> = Vec::new();

        for (i, opt) in self.options.iter().enumerate() {
            let label_lower = opt.label.to_lowercase();

            let score = match self.filter_mode {
                FilterMode::Fuzzy => fuzzy_match(&self.input, &opt.label).map(|m| m.score),
                FilterMode::Prefix => {
                    if label_lower.starts_with(&query) {
                        Some(100 - (opt.label.len() as i32))
                    } else {
                        None
                    }
                }
                FilterMode::Exact => {
                    if label_lower == query {
                        Some(100)
                    } else {
                        None
                    }
                }
                FilterMode::Contains => {
                    if label_lower.contains(&query) {
                        // Score based on position (earlier = higher)
                        label_lower
                            .find(&query)
                            .map(|pos| 100 - (pos as i32) - (opt.label.len() as i32))
                    } else {
                        None
                    }
                }
            };

            if let Some(s) = score {
                matches.push((i, s));
            }
        }

        // Sort by score descending
        matches.sort_by(|a, b| b.1.cmp(&a.1));

        self.filtered = matches.into_iter().map(|(i, _)| i).collect();
        self.selected_idx = 0;
        self.scroll_offset = 0;
    }

    /// Get fuzzy match for an option (for highlighting)
    pub fn get_match(&self, option: &str) -> Option<FuzzyMatch> {
        if self.input.is_empty() || self.filter_mode != FilterMode::Fuzzy {
            None
        } else {
            fuzzy_match(&self.input, option)
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Helpers
    // ─────────────────────────────────────────────────────────────────────────

    /// Convert character index to byte index
    fn char_to_byte_index(&self, char_idx: usize) -> usize {
        self.input
            .char_indices()
            .nth(char_idx)
            .map(|(i, _)| i)
            .unwrap_or(self.input.len())
    }

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

        // +4 for padding and borders
        ((max_option_len.max(20) + 4) as u16).min(max_width)
    }
}

impl Default for Combobox {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Combobox {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 {
            return;
        }

        let width = self.display_width(area.width);
        let text_width = (width - 2) as usize;

        // ─────────────────────────────────────────────────────────────────────
        // Render input field
        // ─────────────────────────────────────────────────────────────────────

        let input_fg = self.input_fg.or(self.fg);
        let input_bg = self.input_bg.or(self.bg);

        // Draw background
        for x in 0..width {
            let mut cell = Cell::new(' ');
            cell.fg = input_fg;
            cell.bg = input_bg;
            ctx.buffer.set(area.x + x, area.y, cell);
        }

        // Draw dropdown indicator
        let icon = if self.loading {
            '⟳'
        } else if self.open {
            '▲'
        } else {
            '▼'
        };
        let mut cell = Cell::new(icon);
        cell.fg = input_fg;
        cell.bg = input_bg;
        ctx.buffer.set(area.x + width - 2, area.y, cell);

        // Draw text or placeholder
        let display_text = if self.input.is_empty() {
            &self.placeholder
        } else {
            &self.input
        };

        let is_placeholder = self.input.is_empty();
        let truncated: String = display_text.chars().take(text_width).collect();

        for (i, ch) in truncated.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = if is_placeholder {
                self.disabled_fg
            } else {
                input_fg
            };
            cell.bg = input_bg;
            ctx.buffer.set(area.x + 1 + i as u16, area.y, cell);
        }

        // Draw cursor (if not placeholder)
        if !is_placeholder && self.cursor <= truncated.chars().count() {
            let cursor_x = area.x + 1 + self.cursor as u16;
            if cursor_x < area.x + width - 2 {
                if let Some(cell) = ctx.buffer.get_mut(cursor_x, area.y) {
                    cell.bg = Some(Color::WHITE);
                    cell.fg = Some(Color::BLACK);
                }
            }
        }

        // ─────────────────────────────────────────────────────────────────────
        // Render dropdown (if open)
        // ─────────────────────────────────────────────────────────────────────

        if !self.open || area.height <= 1 {
            return;
        }

        let dropdown_height = (area.height - 1) as usize;
        let visible_count = dropdown_height.min(self.max_visible);

        // Loading state
        if self.loading {
            let y = area.y + 1;
            for x in 0..width {
                let mut cell = Cell::new(' ');
                cell.fg = self.fg;
                cell.bg = self.bg;
                ctx.buffer.set(area.x + x, y, cell);
            }
            for (i, ch) in self.loading_text.chars().take(text_width).enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = self.disabled_fg;
                cell.bg = self.bg;
                ctx.buffer.set(area.x + 1 + i as u16, y, cell);
            }
            return;
        }

        // Empty state
        if self.filtered.is_empty() {
            let y = area.y + 1;
            for x in 0..width {
                let mut cell = Cell::new(' ');
                cell.fg = self.fg;
                cell.bg = self.bg;
                ctx.buffer.set(area.x + x, y, cell);
            }
            for (i, ch) in self.empty_text.chars().take(text_width).enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = self.disabled_fg;
                cell.bg = self.bg;
                ctx.buffer.set(area.x + 1 + i as u16, y, cell);
            }
            return;
        }

        // Render visible options
        for (row, &option_idx) in self
            .filtered
            .iter()
            .skip(self.scroll_offset)
            .take(visible_count)
            .enumerate()
        {
            let y = area.y + 1 + row as u16;
            let option = &self.options[option_idx];
            let is_highlighted = row + self.scroll_offset == self.selected_idx;
            let is_multi_selected = self.multi_select && self.is_selected(option.get_value());

            let (fg, bg) = if is_highlighted {
                (self.selected_fg, self.selected_bg)
            } else {
                (self.fg, self.bg)
            };

            let fg = if option.disabled {
                self.disabled_fg
            } else {
                fg
            };

            // Draw background
            for x in 0..width {
                let mut cell = Cell::new(' ');
                cell.fg = fg;
                cell.bg = bg;
                ctx.buffer.set(area.x + x, y, cell);
            }

            // Draw selection indicator (for multi-select)
            if self.multi_select {
                let indicator = if is_multi_selected { '☑' } else { '☐' };
                let mut cell = Cell::new(indicator);
                cell.fg = fg;
                cell.bg = bg;
                ctx.buffer.set(area.x, y, cell);
            } else {
                let indicator = if is_highlighted { '›' } else { ' ' };
                let mut cell = Cell::new(indicator);
                cell.fg = fg;
                cell.bg = bg;
                ctx.buffer.set(area.x, y, cell);
            }

            // Get match indices for highlighting
            let match_indices: Vec<usize> = self
                .get_match(&option.label)
                .map(|m| m.indices)
                .unwrap_or_default();

            // Draw option text with highlighting
            let truncated: String = option.label.chars().take(text_width - 1).collect();
            for (j, ch) in truncated.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.bg = bg;

                if option.disabled {
                    cell.fg = self.disabled_fg;
                } else if match_indices.contains(&j) {
                    cell.fg = self.highlight_fg;
                } else {
                    cell.fg = fg;
                }

                ctx.buffer.set(area.x + 2 + j as u16, y, cell);
            }
        }

        // Draw scroll indicator if needed
        if self.filtered.len() > visible_count {
            let has_more_above = self.scroll_offset > 0;
            let has_more_below = self.scroll_offset + visible_count < self.filtered.len();

            if has_more_above {
                let mut cell = Cell::new('↑');
                cell.fg = self.disabled_fg;
                cell.bg = self.bg;
                ctx.buffer.set(area.x + width - 1, area.y + 1, cell);
            }

            if has_more_below {
                let y = area.y + visible_count as u16;
                let mut cell = Cell::new('↓');
                cell.fg = self.disabled_fg;
                cell.bg = self.bg;
                ctx.buffer.set(area.x + width - 1, y, cell);
            }
        }
    }

    crate::impl_view_meta!("Combobox");
}

impl_styled_view!(Combobox);
impl_props_builders!(Combobox);

/// Helper function to create a combobox
pub fn combobox() -> Combobox {
    Combobox::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_combobox_new() {
        let cb = Combobox::new();
        assert!(cb.input().is_empty());
        assert!(!cb.is_open());
        assert_eq!(cb.option_count(), 0);
    }

    #[test]
    fn test_combobox_options() {
        let cb = Combobox::new().options(vec!["Apple", "Banana", "Cherry"]);
        assert_eq!(cb.option_count(), 3);
        assert_eq!(cb.filtered_count(), 3);
    }

    #[test]
    fn test_combobox_options_with() {
        let cb = Combobox::new().options_with(vec![
            ComboOption::new("Apple").value("apple"),
            ComboOption::new("Banana").disabled(true),
        ]);
        assert_eq!(cb.option_count(), 2);
    }

    #[test]
    fn test_combobox_filtering_fuzzy() {
        let mut cb = Combobox::new()
            .options(vec!["Hello World", "Help Me", "Goodbye"])
            .filter_mode(FilterMode::Fuzzy);

        cb.set_input("hw");
        assert_eq!(cb.filtered_count(), 1); // Only "Hello World" matches "hw"
    }

    #[test]
    fn test_combobox_filtering_prefix() {
        let mut cb = Combobox::new()
            .options(vec!["Hello", "Help", "World"])
            .filter_mode(FilterMode::Prefix);

        cb.set_input("Hel");
        assert_eq!(cb.filtered_count(), 2); // "Hello" and "Help"
    }

    #[test]
    fn test_combobox_filtering_contains() {
        let mut cb = Combobox::new()
            .options(vec!["Hello", "Shell", "World"])
            .filter_mode(FilterMode::Contains);

        cb.set_input("ell");
        assert_eq!(cb.filtered_count(), 2); // "Hello" and "Shell"
    }

    #[test]
    fn test_combobox_filtering_exact() {
        let mut cb = Combobox::new()
            .options(vec!["Hello", "hello", "HELLO"])
            .filter_mode(FilterMode::Exact);

        cb.set_input("hello");
        assert_eq!(cb.filtered_count(), 3); // All match (case-insensitive)
    }

    #[test]
    fn test_combobox_navigation() {
        let mut cb = Combobox::new().options(vec!["A", "B", "C"]);

        cb.open_dropdown();
        assert!(cb.is_open());

        cb.select_next();
        assert_eq!(cb.selected_idx, 1);

        cb.select_next();
        assert_eq!(cb.selected_idx, 2);

        cb.select_next(); // Wraps
        assert_eq!(cb.selected_idx, 0);

        cb.select_prev(); // Wraps backward
        assert_eq!(cb.selected_idx, 2);

        cb.select_first();
        assert_eq!(cb.selected_idx, 0);

        cb.select_last();
        assert_eq!(cb.selected_idx, 2);
    }

    #[test]
    fn test_combobox_select_current() {
        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);

        cb.open_dropdown();
        cb.select_next(); // Select "Banana"
        cb.select_current();

        assert_eq!(cb.input(), "Banana");
        assert!(!cb.is_open()); // Closes after selection
    }

    #[test]
    fn test_combobox_multi_select() {
        let mut cb = Combobox::new()
            .options(vec!["A", "B", "C"])
            .multi_select(true);

        cb.open_dropdown();
        cb.select_current(); // Select "A"
        assert!(cb.is_selected("A"));
        assert!(cb.is_open()); // Stays open in multi-select

        cb.select_next();
        cb.select_current(); // Select "B"
        assert!(cb.is_selected("A"));
        assert!(cb.is_selected("B"));

        // Toggle off
        cb.select_first();
        cb.select_current(); // Deselect "A"
        assert!(!cb.is_selected("A"));
        assert!(cb.is_selected("B"));
    }

    #[test]
    fn test_combobox_allow_custom() {
        let cb = Combobox::new()
            .options(vec!["Apple", "Banana"])
            .allow_custom(true)
            .value("Custom Value");

        assert_eq!(cb.selected_value(), Some("Custom Value"));
    }

    #[test]
    fn test_combobox_disabled_option() {
        let mut cb = Combobox::new().options_with(vec![
            ComboOption::new("Enabled"),
            ComboOption::new("Disabled").disabled(true),
        ]);

        cb.open_dropdown();
        cb.select_next(); // Try to select disabled option
        let selected = cb.select_current();
        assert!(!selected); // Should not select
    }

    #[test]
    fn test_combobox_input_manipulation() {
        let mut cb = Combobox::new();

        cb.insert_char('H');
        cb.insert_char('i');
        assert_eq!(cb.input(), "Hi");
        assert_eq!(cb.cursor, 2);

        cb.delete_backward();
        assert_eq!(cb.input(), "H");
        assert_eq!(cb.cursor, 1);

        cb.move_left();
        assert_eq!(cb.cursor, 0);

        cb.insert_char('O');
        assert_eq!(cb.input(), "OH");

        cb.move_to_end();
        assert_eq!(cb.cursor, 2);

        cb.move_to_start();
        assert_eq!(cb.cursor, 0);
    }

    #[test]
    fn test_combobox_handle_key() {
        use crate::event::Key;

        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);

        // Type to filter
        cb.handle_key(&Key::Char('a'));
        assert_eq!(cb.input(), "a");
        assert!(cb.is_open()); // Opens on typing

        // Navigate
        cb.handle_key(&Key::Down);
        assert_eq!(cb.selected_idx, 1);

        // Select
        cb.handle_key(&Key::Enter);
        assert!(!cb.is_open());

        // Escape
        cb.open_dropdown();
        cb.handle_key(&Key::Escape);
        assert!(!cb.is_open());
    }

    #[test]
    fn test_combobox_loading_state() {
        let cb = Combobox::new().loading(true).loading_text("Fetching...");

        assert!(cb.is_loading());
    }

    #[test]
    fn test_combobox_render_closed() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let cb = Combobox::new()
            .options(vec!["Option 1", "Option 2"])
            .placeholder("Select...");

        cb.render(&mut ctx);

        // Should show dropdown arrow
        // The arrow is at width - 2
    }

    #[test]
    fn test_combobox_render_open() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
        cb.open_dropdown();

        cb.render(&mut ctx);

        // Options should be rendered below input
    }

    #[test]
    fn test_combobox_helper() {
        let cb = combobox().option("Test").placeholder("Pick one");
        assert_eq!(cb.option_count(), 1);
    }

    #[test]
    fn test_combobox_clear_input() {
        let mut cb = Combobox::new().options(vec!["A", "B"]).value("test");

        assert_eq!(cb.input(), "test");
        cb.clear_input();
        assert!(cb.input().is_empty());
    }

    #[test]
    fn test_combobox_scroll() {
        let mut cb = Combobox::new()
            .options(vec!["A", "B", "C", "D", "E", "F", "G", "H"])
            .max_visible(3);

        cb.open_dropdown();

        // Navigate to end
        for _ in 0..7 {
            cb.select_next();
        }

        // Should have scrolled
        assert!(cb.scroll_offset > 0);
    }

    #[test]
    fn test_combo_option_builder() {
        let opt = ComboOption::new("Label")
            .value("value")
            .disabled(true)
            .group("Category");

        assert_eq!(opt.label, "Label");
        assert_eq!(opt.get_value(), "value");
        assert!(opt.disabled);
        assert_eq!(opt.group, Some("Category".to_string()));
    }

    #[test]
    fn test_combobox_empty_filter() {
        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);

        cb.set_input("xyz"); // No match
        assert_eq!(cb.filtered_count(), 0);
    }

    #[test]
    fn test_filter_mode_default() {
        assert_eq!(FilterMode::default(), FilterMode::Fuzzy);
    }

    // Additional tests for coverage

    #[test]
    fn test_combobox_builder_methods() {
        let cb = Combobox::new()
            .loading_text("Please wait...")
            .empty_text("Nothing found")
            .width(50)
            .input_style(Color::WHITE, Color::BLACK)
            .selected_style(Color::BLACK, Color::WHITE)
            .highlight_fg(Color::YELLOW)
            .fg(Color::WHITE)
            .bg(Color::BLACK);

        assert_eq!(cb.loading_text, "Please wait...");
        assert_eq!(cb.empty_text, "Nothing found");
        assert_eq!(cb.width, Some(50));
    }

    #[test]
    fn test_combobox_selected_values() {
        let cb = Combobox::new()
            .multi_select(true)
            .selected_values(vec!["A".to_string(), "B".to_string()]);

        assert_eq!(cb.selected_values_ref(), &["A", "B"]);
    }

    #[test]
    fn test_combobox_delete_forward() {
        let mut cb = Combobox::new().value("Hello");
        cb.move_to_start();
        cb.delete_forward();
        assert_eq!(cb.input(), "ello");
    }

    #[test]
    fn test_combobox_delete_forward_at_end() {
        let mut cb = Combobox::new().value("Hi");
        // Cursor at end, delete_forward should do nothing
        cb.delete_forward();
        assert_eq!(cb.input(), "Hi");
    }

    #[test]
    fn test_combobox_delete_backward_at_start() {
        let mut cb = Combobox::new().value("Hi");
        cb.move_to_start();
        cb.delete_backward();
        assert_eq!(cb.input(), "Hi"); // Nothing deleted
    }

    #[test]
    fn test_combobox_move_right_at_end() {
        let mut cb = Combobox::new().value("Hi");
        cb.move_right(); // Already at end
        assert_eq!(cb.cursor, 2);
    }

    #[test]
    fn test_combobox_move_left_at_start() {
        let mut cb = Combobox::new().value("Hi");
        cb.move_to_start();
        cb.move_left(); // Already at start
        assert_eq!(cb.cursor, 0);
    }

    #[test]
    fn test_combobox_toggle_dropdown() {
        let mut cb = Combobox::new().options(vec!["A", "B"]);
        assert!(!cb.is_open());

        cb.toggle_dropdown();
        assert!(cb.is_open());

        cb.toggle_dropdown();
        assert!(!cb.is_open());
    }

    #[test]
    fn test_combobox_handle_key_down_when_closed() {
        use crate::event::Key;

        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
        assert!(!cb.is_open());

        cb.handle_key(&Key::Down);
        assert!(cb.is_open()); // Down opens dropdown
    }

    #[test]
    fn test_combobox_handle_key_tab_completion() {
        use crate::event::Key;

        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
        cb.open_dropdown();
        cb.handle_key(&Key::Tab);

        assert_eq!(cb.input(), "Apple"); // First option filled
    }

    #[test]
    fn test_combobox_handle_key_delete() {
        use crate::event::Key;

        let mut cb = Combobox::new().value("Hello");
        cb.move_to_start();
        cb.handle_key(&Key::Delete);
        assert_eq!(cb.input(), "ello");
    }

    #[test]
    fn test_combobox_handle_key_home_end() {
        use crate::event::Key;

        let mut cb = Combobox::new().value("Hello");
        cb.handle_key(&Key::Home);
        assert_eq!(cb.cursor, 0);

        cb.handle_key(&Key::End);
        assert_eq!(cb.cursor, 5);
    }

    #[test]
    fn test_combobox_handle_key_left_right() {
        use crate::event::Key;

        let mut cb = Combobox::new().value("Hi");
        cb.handle_key(&Key::Left);
        assert_eq!(cb.cursor, 1);

        cb.handle_key(&Key::Right);
        assert_eq!(cb.cursor, 2);
    }

    #[test]
    fn test_combobox_handle_key_up_when_open() {
        use crate::event::Key;

        let mut cb = Combobox::new().options(vec!["A", "B", "C"]);
        cb.open_dropdown();
        cb.select_next(); // Go to B
        cb.handle_key(&Key::Up);
        assert_eq!(cb.selected_idx, 0); // Back to A
    }

    #[test]
    fn test_combobox_handle_key_unhandled() {
        use crate::event::Key;

        let mut cb = Combobox::new();
        let handled = cb.handle_key(&Key::F(1));
        assert!(!handled);
    }

    #[test]
    fn test_combobox_selected_value_from_option() {
        let cb = Combobox::new()
            .options(vec!["Apple", "Banana"])
            .value("Apple");

        assert_eq!(cb.selected_value(), Some("Apple"));
    }

    #[test]
    fn test_combobox_selected_value_multi_select_returns_none() {
        let cb = Combobox::new()
            .options(vec!["A", "B"])
            .multi_select(true)
            .value("A");

        assert_eq!(cb.selected_value(), None);
    }

    #[test]
    fn test_combobox_selected_value_no_match_no_custom() {
        let cb = Combobox::new()
            .options(vec!["Apple", "Banana"])
            .value("Custom");

        assert_eq!(cb.selected_value(), None);
    }

    #[test]
    fn test_combobox_get_match_non_fuzzy() {
        let cb = Combobox::new()
            .options(vec!["Apple"])
            .filter_mode(FilterMode::Prefix)
            .value("App");

        // get_match only works for fuzzy mode
        assert!(cb.get_match("Apple").is_none());
    }

    #[test]
    fn test_combobox_select_on_empty_filtered() {
        let mut cb = Combobox::new().options(vec!["Apple"]);
        cb.set_input("xyz"); // No matches
        let selected = cb.select_current();
        assert!(!selected);
    }

    #[test]
    fn test_combobox_navigation_empty_options() {
        let mut cb = Combobox::new();
        cb.select_next(); // Should not panic
        cb.select_prev();
        cb.select_last();
        assert_eq!(cb.selected_idx, 0);
    }

    #[test]
    fn test_combobox_render_loading_state() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new()
            .options(vec!["A", "B"])
            .loading(true)
            .loading_text("Loading...")
            .width(30); // Set explicit width
        cb.open_dropdown();

        cb.render(&mut ctx);

        // Verify loading indicator is shown (at width - 2)
        assert_eq!(buffer.get(28, 0).unwrap().symbol, '⟳');
    }

    #[test]
    fn test_combobox_render_empty_state() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new()
            .options(vec!["Apple", "Banana"])
            .empty_text("No results");
        cb.set_input("xyz"); // No matches
        cb.open_dropdown();

        cb.render(&mut ctx);
        // Empty state should be rendered
    }

    #[test]
    fn test_combobox_render_with_scroll_indicators() {
        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new()
            .options(vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"])
            .max_visible(3);
        cb.open_dropdown();

        // Navigate down to trigger scroll
        for _ in 0..5 {
            cb.select_next();
        }

        cb.render(&mut ctx);
        // Scroll indicators should be visible
    }

    #[test]
    fn test_combobox_render_multi_select() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new()
            .options(vec!["A", "B", "C"])
            .multi_select(true);
        cb.open_dropdown();
        cb.select_current(); // Select "A"

        cb.render(&mut ctx);
        // Multi-select checkboxes should be rendered
    }

    #[test]
    fn test_combobox_render_with_input() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new().options(vec!["Apple", "Banana"]);
        cb.set_input("App");
        cb.open_dropdown();

        cb.render(&mut ctx);
        // Input and filtered options should be rendered with highlights
    }

    #[test]
    fn test_combobox_render_disabled_option() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new().options_with(vec![
            ComboOption::new("Enabled"),
            ComboOption::new("Disabled").disabled(true),
        ]);
        cb.open_dropdown();

        cb.render(&mut ctx);
        // Disabled option should be rendered with disabled color
    }

    #[test]
    fn test_combobox_render_small_area() {
        let mut buffer = Buffer::new(2, 1);
        let area = Rect::new(0, 0, 2, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let cb = Combobox::new().options(vec!["A"]);
        cb.render(&mut ctx);
        // Should handle small area gracefully (early return)
    }

    #[test]
    fn test_combobox_render_height_one() {
        let mut buffer = Buffer::new(30, 1);
        let area = Rect::new(0, 0, 30, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new().options(vec!["A", "B"]);
        cb.open_dropdown();
        cb.render(&mut ctx);
        // Dropdown shouldn't render when height is 1
    }

    #[test]
    fn test_combobox_default() {
        let cb = Combobox::default();
        assert!(cb.input().is_empty());
        assert!(!cb.is_open());
    }

    #[test]
    fn test_combo_option_from_string() {
        let opt: ComboOption = "Test".into();
        assert_eq!(opt.label, "Test");
        assert_eq!(opt.get_value(), "Test");
    }

    #[test]
    fn test_combobox_ensure_visible_scroll_up() {
        let mut cb = Combobox::new()
            .options(vec!["A", "B", "C", "D", "E", "F", "G", "H"])
            .max_visible(3);

        cb.open_dropdown();

        // Scroll down
        for _ in 0..7 {
            cb.select_next();
        }
        assert!(cb.scroll_offset > 0);

        // Now scroll back up
        for _ in 0..7 {
            cb.select_prev();
        }
        assert_eq!(cb.scroll_offset, 0);
    }

    #[test]
    fn test_combobox_handle_key_enter_not_open_allow_custom() {
        use crate::event::Key;

        let mut cb = Combobox::new()
            .options(vec!["A", "B"])
            .allow_custom(true)
            .value("Custom");

        // Enter when not open with allow_custom
        let handled = cb.handle_key(&Key::Enter);
        assert!(handled);
    }

    #[test]
    fn test_combobox_option_with_separate_value() {
        let mut cb = Combobox::new()
            .options_with(vec![ComboOption::new("Display Name").value("actual_value")]);

        cb.open_dropdown();
        cb.select_current();

        // Input should be label, but value lookup should work
        assert_eq!(cb.input(), "Display Name");
    }

    #[test]
    fn test_combobox_cursor_render_boundary() {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new().value("Very long text that exceeds width");
        cb.render(&mut ctx);
        // Should handle cursor at boundary correctly
    }

    #[test]
    fn test_combobox_render_highlighted_option() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut cb = Combobox::new().options(vec!["Apple", "Banana", "Cherry"]);
        cb.open_dropdown();
        cb.select_next(); // Highlight "Banana"

        cb.render(&mut ctx);
        // Should render with selected style
    }
}
