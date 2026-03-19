//! Select/Dropdown widget for choosing from a list of options

use crate::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::{display_width, fuzzy_match, truncate_to_width, FuzzyMatch, Selection};
use crate::widget::traits::{EventResult, Interactive, RenderContext, View, WidgetProps};
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
    /// Cached auto-calculated width (invalidated on options change)
    cached_auto_width: Option<u16>,
    /// Search query for filtering options
    query: String,
    /// Enable fuzzy search when typing
    searchable: bool,
    /// Filtered indices (into options) based on query
    filtered: Vec<usize>,
    /// Selection state for filtered results (uses Selection utility)
    filtered_selection: Selection,
    /// Focused state
    focused: bool,
    /// Disabled state
    disabled: bool,
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
        self.reset_filter();
        self.update_cached_width();
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

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

    /// Calculate display width (uses cache when available)
    fn display_width(&self, max_width: u16) -> u16 {
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

impl View for Select {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 {
            return;
        }

        let width = self.display_width(area.width);
        let text_width = (width - 2) as usize;

        // Determine colors based on state
        let fg = if self.disabled {
            Some(Color::rgb(128, 128, 128))
        } else if self.focused {
            self.fg.or(Some(Color::CYAN))
        } else {
            self.fg
        };
        let bg = self.bg;

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
            cell.fg = fg;
            cell.bg = bg;
            ctx.set(x, 0, cell);
        }

        // Draw focus indicator
        if self.focused && !self.disabled {
            // Add brackets around select when focused
            if area.x > 0 {
                let mut left = Cell::new('[');
                left.fg = Some(Color::CYAN);
                ctx.buffer.set(area.x.saturating_sub(1), area.y, left);
            }

            if width < area.width {
                let mut right = Cell::new(']');
                right.fg = Some(Color::CYAN);
                ctx.set(width, 0, right);
            }
        }

        // Draw arrow (or search icon when searching)
        let icon = if self.open && self.searchable {
            "🔍".chars().next().unwrap_or('?')
        } else {
            arrow.chars().next().unwrap_or('▼')
        };
        let mut cell = Cell::new(icon);
        cell.fg = fg;
        cell.bg = bg;
        ctx.set(0, 0, cell);

        // Draw text
        let truncated = truncate_to_width(display_text, text_width);
        let mut cx: u16 = 2;
        for ch in truncated.chars() {
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            ctx.set(cx, 0, cell);
            cx += crate::utils::char_width(ch) as u16;
        }

        // If open, render dropdown options as overlay (escapes parent clipping)
        if self.open {
            // Determine which options to show
            let visible_options: Vec<(usize, &String)> = if self.query.is_empty() {
                self.options.iter().enumerate().collect()
            } else {
                self.filtered
                    .iter()
                    .filter_map(|&i| self.options.get(i).map(|opt| (i, opt)))
                    .collect()
            };

            // Calculate dropdown height (limited to 10 or option count)
            let dropdown_height = if visible_options.is_empty() {
                1u16 // "No results" row
            } else {
                (visible_options.len() as u16).min(10)
            };

            // Calculate absolute position for overlay, flip above if near bottom
            let (abs_x, abs_y) = ctx.absolute_position();
            let buf_height = ctx.buffer.height();
            let space_below = buf_height.saturating_sub(abs_y + 1);
            let overlay_y = if space_below >= dropdown_height {
                abs_y + 1 // Render below
            } else {
                abs_y.saturating_sub(dropdown_height) // Render above
            };
            let overlay_area = crate::layout::Rect::new(abs_x, overlay_y, width, dropdown_height);

            let mut entry = crate::widget::traits::OverlayEntry::new(100, overlay_area);

            if visible_options.is_empty() && !self.query.is_empty() {
                // "No results" message
                let msg = "No results";
                for (i, ch) in msg.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(128, 128, 128));
                    entry.push(2 + i as u16, 0, cell);
                }
            }

            for (row, (option_idx, option)) in visible_options
                .iter()
                .enumerate()
                .take(dropdown_height as usize)
            {
                let y = row as u16;
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
                    entry.push(x, y, cell);
                }

                // Draw selection indicator
                let indicator = if is_selected { '›' } else { ' ' };
                let mut cell = Cell::new(indicator);
                cell.fg = fg;
                cell.bg = bg;
                entry.push(0, y, cell);

                // Get fuzzy match indices for highlighting (HashSet for O(1) lookup)
                let match_indices: std::collections::HashSet<usize> = self
                    .get_match(option)
                    .map(|m| m.indices.into_iter().collect())
                    .unwrap_or_default();

                // Draw option text with highlighting
                let truncated = truncate_to_width(option, text_width);
                let mut cx: u16 = 2;
                for (j, ch) in truncated.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.bg = bg;

                    if match_indices.contains(&j) {
                        cell.fg = self.highlight_fg;
                    } else {
                        cell.fg = fg;
                    }

                    entry.push(cx, y, cell);
                    cx += crate::utils::char_width(ch) as u16;
                }
            }

            // Queue as overlay; falls back to inline if no overlay support
            if !ctx.queue_overlay(entry.clone()) {
                // Fallback: render inline (clipped by parent)
                for oc in &entry.cells {
                    ctx.set(oc.x, oc.y + 1, oc.cell);
                }
            }
        }
    }

    crate::impl_view_meta!("Select");
}

impl Interactive for Select {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        if self.disabled {
            return EventResult::Ignored;
        }

        let needs_render = match event.key {
            Key::Enter => {
                if self.open {
                    self.close();
                    self.clear_query();
                } else {
                    self.open();
                }
                true
            }
            Key::Char(' ') if !self.searchable => {
                self.toggle();
                true
            }
            Key::Up | Key::Char('k') if self.open && !self.searchable => {
                self.select_prev();
                true
            }
            Key::Down | Key::Char('j') if self.open && !self.searchable => {
                self.select_next();
                true
            }
            Key::Up if self.open && self.searchable => {
                if self.query.is_empty() {
                    self.select_prev();
                } else {
                    self.select_prev_filtered();
                }
                true
            }
            Key::Down if self.open && self.searchable => {
                if self.query.is_empty() {
                    self.select_next();
                } else {
                    self.select_next_filtered();
                }
                true
            }
            Key::Escape if self.open => {
                self.close();
                self.clear_query();
                true
            }
            Key::Backspace if self.open && self.searchable => {
                self.query.pop();
                self.update_filter();
                true
            }
            Key::Char(c) if self.open && self.searchable => {
                self.query.push(c);
                self.update_filter();
                true
            }
            Key::Home if self.open => {
                self.select_first();
                true
            }
            Key::End if self.open => {
                self.select_last();
                true
            }
            _ => false,
        };

        if needs_render {
            EventResult::ConsumedAndRender
        } else {
            EventResult::Ignored
        }
    }

    fn handle_mouse(&mut self, event: &MouseEvent, area: Rect) -> EventResult {
        if self.disabled {
            return EventResult::Ignored;
        }

        let inside = area.contains(event.x, event.y);

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) if inside => {
                self.toggle();
                EventResult::ConsumedAndRender
            }
            _ => EventResult::Ignored,
        }
    }

    fn focusable(&self) -> bool {
        !self.disabled
    }

    fn on_focus(&mut self) {
        self.focused = true;
    }

    fn on_blur(&mut self) {
        self.focused = false;
        if self.open {
            self.close();
            self.clear_query();
        }
    }
}

impl_styled_view!(Select);
impl_props_builders!(Select);

/// Helper function to create a select widget
pub fn select() -> Select {
    Select::new()
}

// All tests moved to tests/widget_tests.rs
