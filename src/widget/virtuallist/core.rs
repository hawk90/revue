use super::types::*;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, WidgetProps};
use std::ops::Range;

/// A virtual list that only renders visible items
pub struct VirtualList<T> {
    /// All items (only visible ones are rendered)
    pub items: Vec<T>,
    /// Height of each item in rows (uniform height)
    pub item_height: u16,
    /// Variable height calculator (overrides item_height if set)
    pub height_calculator: Option<HeightCalculator<T>>,
    /// Cached heights for variable-height mode
    pub height_cache: Vec<u16>,
    /// Cached cumulative heights (prefix sums for fast lookup)
    pub cumulative_heights: Vec<u32>,
    /// Current scroll offset (in items for uniform, in rows for variable)
    pub scroll_offset: usize,
    /// Sub-item scroll offset (for smooth scrolling, 0-item_height)
    pub scroll_sub_offset: u16,
    /// Currently selected index
    pub selected: Option<usize>,
    /// Selection background color
    pub selected_bg: Color,
    /// Selection foreground color
    pub selected_fg: Color,
    /// Normal item foreground color
    pub item_fg: Color,
    /// Show scrollbar
    pub show_scrollbar: bool,
    /// Scrollbar foreground color
    pub scrollbar_fg: Color,
    /// Scrollbar background color
    pub scrollbar_bg: Color,
    /// Custom item renderer
    pub renderer: Option<ItemRenderer<T>>,
    /// Overscan (extra items to render above/below viewport)
    pub overscan: usize,
    /// Enable wrap-around navigation
    pub wrap_navigation: bool,
    /// Scroll mode
    pub scroll_mode: ScrollMode,
    /// CSS styling properties (id, classes)
    pub props: WidgetProps,
}

impl<T: ToString + Clone> VirtualList<T> {
    /// Create a new virtual list
    pub fn new(items: Vec<T>) -> Self {
        let len = items.len();
        Self {
            items,
            item_height: 1,
            height_calculator: None,
            height_cache: Vec::new(),
            cumulative_heights: Vec::new(),
            scroll_offset: 0,
            scroll_sub_offset: 0,
            selected: if len > 0 { Some(0) } else { None },
            selected_bg: Color::rgb(60, 60, 120),
            selected_fg: Color::WHITE,
            item_fg: Color::WHITE,
            show_scrollbar: true,
            scrollbar_fg: Color::WHITE,
            scrollbar_bg: Color::rgb(40, 40, 40),
            renderer: None,
            overscan: 2,
            wrap_navigation: false,
            scroll_mode: ScrollMode::default(),
            props: WidgetProps::new(),
        }
    }

    /// Set item height (number of rows per item)
    pub fn item_height(mut self, height: u16) -> Self {
        self.item_height = height.max(1);
        self
    }

    /// Set selected index
    pub fn selected(mut self, index: usize) -> Self {
        if index < self.items.len() {
            self.selected = Some(index);
        }
        self
    }

    /// Set selection colors
    pub fn selected_style(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = fg;
        self.selected_bg = bg;
        self
    }

    /// Set item foreground color
    pub fn item_fg(mut self, color: Color) -> Self {
        self.item_fg = color;
        self
    }

    /// Enable/disable scrollbar
    pub fn show_scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }

    /// Set scrollbar colors
    pub fn scrollbar_style(mut self, fg: Color, bg: Color) -> Self {
        self.scrollbar_fg = fg;
        self.scrollbar_bg = bg;
        self
    }

    /// Set overscan (extra items to render for smoother scrolling)
    pub fn overscan(mut self, count: usize) -> Self {
        self.overscan = count;
        self
    }

    /// Enable wrap-around navigation
    pub fn wrap_navigation(mut self, enable: bool) -> Self {
        self.wrap_navigation = enable;
        self
    }

    /// Set custom item renderer
    pub fn renderer<F>(mut self, f: F) -> Self
    where
        F: Fn(&T, usize, bool) -> String + 'static,
    {
        self.renderer = Some(Box::new(f));
        self
    }

    /// Set scroll mode
    pub fn scroll_mode(mut self, mode: ScrollMode) -> Self {
        self.scroll_mode = mode;
        self
    }

    /// Set variable height calculator
    ///
    /// When set, each item can have a different height based on its content.
    /// The calculator receives the item and its index, returning the height in rows.
    pub fn variable_height<F>(mut self, calculator: F) -> Self
    where
        F: Fn(&T, usize) -> u16 + 'static,
    {
        self.height_calculator = Some(Box::new(calculator));
        self.rebuild_height_cache();
        self
    }

    /// Rebuild the height cache for variable-height mode
    fn rebuild_height_cache(&mut self) {
        if let Some(ref calc) = self.height_calculator {
            self.height_cache.clear();
            self.cumulative_heights.clear();

            let mut cumulative: u32 = 0;
            for (idx, item) in self.items.iter().enumerate() {
                let height = calc(item, idx).max(1);
                self.height_cache.push(height);
                cumulative += height as u32;
                self.cumulative_heights.push(cumulative);
            }
        }
    }

    /// Get height of item at index
    #[cfg(test)]
    fn get_item_height(&self, index: usize) -> u16 {
        if self.height_calculator.is_some() && index < self.height_cache.len() {
            self.height_cache[index]
        } else {
            self.item_height
        }
    }

    /// Get total height of all items
    fn total_height(&self) -> u32 {
        if self.height_calculator.is_some() && !self.cumulative_heights.is_empty() {
            *self.cumulative_heights.last().unwrap_or(&0)
        } else {
            self.items.len() as u32 * self.item_height as u32
        }
    }

    /// Find item index at given row offset (for variable heights)
    fn index_at_row(&self, row: u32) -> usize {
        if self.height_calculator.is_none() || self.cumulative_heights.is_empty() {
            return (row / self.item_height as u32) as usize;
        }

        // Binary search for the item containing this row
        match self.cumulative_heights.binary_search(&row) {
            Ok(idx) => idx + 1, // Exact match means we're at the start of next item
            Err(idx) => idx,    // Insert position is the item index
        }
        .min(self.items.len().saturating_sub(1))
    }

    /// Get row offset of item at index (for variable heights)
    fn row_of_index(&self, index: usize) -> u32 {
        if self.height_calculator.is_none() || self.cumulative_heights.is_empty() {
            return index as u32 * self.item_height as u32;
        }

        if index == 0 {
            0
        } else if index <= self.cumulative_heights.len() {
            self.cumulative_heights[index - 1]
        } else {
            *self.cumulative_heights.last().unwrap_or(&0)
        }
    }

    /// Jump to a specific index with alignment
    pub fn jump_to(&mut self, index: usize) {
        self.jump_to_with_alignment(index, ScrollAlignment::Start);
    }

    /// Jump to index with specific alignment
    pub fn jump_to_with_alignment(&mut self, index: usize, alignment: ScrollAlignment) {
        if index >= self.items.len() {
            return;
        }

        self.selected = Some(index);
        // The actual scroll adjustment happens in ensure_visible during render
        // For now, just set the scroll offset directly
        self.scroll_offset = index;
        self.scroll_sub_offset = 0;

        // Store alignment preference (would need viewport height for proper calculation)
        match alignment {
            ScrollAlignment::Start => {
                self.scroll_offset = index;
            }
            ScrollAlignment::Center => {
                // Will be handled in ensure_visible with viewport height
                self.scroll_offset = index.saturating_sub(5);
            }
            ScrollAlignment::End => {
                self.scroll_offset = index;
            }
            ScrollAlignment::Nearest => {
                // Just set to index, ensure_visible will handle it
                self.scroll_offset = index;
            }
        }
    }

    /// Scroll by a number of rows (for smooth scrolling)
    pub fn scroll_by(&mut self, rows: i32) {
        if self.height_calculator.is_some() {
            // Variable height mode - scroll by rows
            let current_row =
                self.row_of_index(self.scroll_offset) as i32 + self.scroll_sub_offset as i32;
            let new_row = (current_row + rows).max(0) as u32;
            let max_row = self.total_height().saturating_sub(1);
            let clamped_row = new_row.min(max_row);

            self.scroll_offset = self.index_at_row(clamped_row);
            let item_start = self.row_of_index(self.scroll_offset);
            self.scroll_sub_offset = (clamped_row - item_start) as u16;
        } else {
            // Uniform height mode
            let total_rows = rows.unsigned_abs() as u16 / self.item_height;
            let sub_rows = rows.unsigned_abs() as u16 % self.item_height;

            if rows > 0 {
                let new_sub = self.scroll_sub_offset + sub_rows;
                if new_sub >= self.item_height {
                    self.scroll_offset = self.scroll_offset.saturating_add(total_rows as usize + 1);
                    self.scroll_sub_offset = new_sub - self.item_height;
                } else {
                    self.scroll_offset = self.scroll_offset.saturating_add(total_rows as usize);
                    self.scroll_sub_offset = new_sub;
                }
            } else if sub_rows > self.scroll_sub_offset {
                self.scroll_offset = self.scroll_offset.saturating_sub(total_rows as usize + 1);
                self.scroll_sub_offset = self.item_height - (sub_rows - self.scroll_sub_offset);
            } else {
                self.scroll_offset = self.scroll_offset.saturating_sub(total_rows as usize);
                self.scroll_sub_offset -= sub_rows;
            }

            // Clamp scroll offset
            let max_offset = self.items.len().saturating_sub(1);
            self.scroll_offset = self.scroll_offset.min(max_offset);
        }
    }

    /// Get scroll position as percentage (0.0 - 1.0)
    pub fn scroll_position(&self) -> f32 {
        if self.items.is_empty() {
            return 0.0;
        }

        let max = self.items.len().saturating_sub(1) as f32;
        if max == 0.0 {
            0.0
        } else {
            self.scroll_offset as f32 / max
        }
    }

    /// Set scroll position by percentage (0.0 - 1.0)
    pub fn set_scroll_position(&mut self, position: f32) {
        let position = position.clamp(0.0, 1.0);
        let max = self.items.len().saturating_sub(1);
        self.scroll_offset = (position * max as f32) as usize;
        self.scroll_sub_offset = 0;
    }

    /// Get total item count
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if list is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get currently selected index
    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }

    /// Get currently selected item
    pub fn selected_item(&self) -> Option<&T> {
        self.selected.and_then(|i| self.items.get(i))
    }

    /// Set items (replacing all)
    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        // Adjust selection if out of bounds
        if let Some(idx) = self.selected {
            if idx >= self.items.len() {
                self.selected = if self.items.is_empty() {
                    None
                } else {
                    Some(self.items.len() - 1)
                };
            }
        }
        // Adjust scroll offset
        if self.scroll_offset >= self.items.len() {
            self.scroll_offset = self.items.len().saturating_sub(1);
        }
    }

    /// Add item to the end
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Remove item at index
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.items.len() {
            let item = self.items.remove(index);
            // Adjust selection
            if let Some(sel) = self.selected {
                if sel >= self.items.len() {
                    self.selected = if self.items.is_empty() {
                        None
                    } else {
                        Some(self.items.len() - 1)
                    };
                } else if sel > index {
                    self.selected = Some(sel - 1);
                }
            }
            Some(item)
        } else {
            None
        }
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.selected = None;
        self.scroll_offset = 0;
    }

    /// Select next item
    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        match self.selected {
            Some(idx) if idx + 1 < self.items.len() => {
                self.selected = Some(idx + 1);
            }
            Some(_) if self.wrap_navigation => {
                self.selected = Some(0);
            }
            None => {
                self.selected = Some(0);
            }
            _ => {}
        }
    }

    /// Select previous item
    pub fn select_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        match self.selected {
            Some(0) if self.wrap_navigation => {
                self.selected = Some(self.items.len() - 1);
            }
            Some(idx) if idx > 0 => {
                self.selected = Some(idx - 1);
            }
            None => {
                self.selected = Some(0);
            }
            _ => {}
        }
    }

    /// Select first item
    pub fn select_first(&mut self) {
        if !self.items.is_empty() {
            self.selected = Some(0);
            self.scroll_offset = 0;
        }
    }

    /// Select last item
    pub fn select_last(&mut self) {
        if !self.items.is_empty() {
            self.selected = Some(self.items.len() - 1);
        }
    }

    /// Page down
    pub fn page_down(&mut self, viewport_height: u16) {
        let page_size = (viewport_height / self.item_height) as usize;
        if let Some(idx) = self.selected {
            let new_idx = (idx + page_size).min(self.items.len().saturating_sub(1));
            self.selected = Some(new_idx);
        }
    }

    /// Page up
    pub fn page_up(&mut self, viewport_height: u16) {
        let page_size = (viewport_height / self.item_height) as usize;
        if let Some(idx) = self.selected {
            self.selected = Some(idx.saturating_sub(page_size));
        }
    }

    /// Scroll to make selected item visible
    pub fn ensure_visible(&mut self, viewport_height: u16) {
        let visible_count = (viewport_height / self.item_height) as usize;
        if let Some(idx) = self.selected {
            if idx < self.scroll_offset {
                self.scroll_offset = idx;
            } else if idx >= self.scroll_offset + visible_count {
                self.scroll_offset = idx.saturating_sub(visible_count - 1);
            }
        }
    }

    /// Get visible item range
    pub fn visible_range(&self, viewport_height: u16) -> Range<usize> {
        let visible_count = (viewport_height / self.item_height) as usize;
        let start = self.scroll_offset.saturating_sub(self.overscan);
        let end = (self.scroll_offset + visible_count + self.overscan).min(self.items.len());
        start..end
    }

    /// Render item text
    pub fn render_item(&self, item: &T, index: usize, is_selected: bool) -> String {
        if let Some(ref renderer) = self.renderer {
            renderer(item, index, is_selected)
        } else {
            item.to_string()
        }
    }

    /// Render scrollbar
    pub fn render_scrollbar(&self, ctx: &mut RenderContext, viewport_height: u16) {
        let area = ctx.area;
        let scrollbar_x = area.x + area.width - 1;

        // Calculate thumb position and size
        let total = self.items.len() as f32;
        let visible = (viewport_height / self.item_height) as f32;

        if total <= visible {
            // No scrollbar needed
            return;
        }

        let thumb_size = ((visible / total) * viewport_height as f32).max(1.0) as u16;
        let scroll_range = viewport_height.saturating_sub(thumb_size);
        let thumb_pos =
            ((self.scroll_offset as f32 / (total - visible)) * scroll_range as f32) as u16;

        // Draw scrollbar track
        for y in 0..viewport_height {
            let abs_y = area.y + y;
            if abs_y < area.y + area.height {
                let in_thumb = y >= thumb_pos && y < thumb_pos + thumb_size;
                let ch = if in_thumb { '█' } else { '░' };
                let color = if in_thumb {
                    self.scrollbar_fg
                } else {
                    self.scrollbar_bg
                };
                ctx.buffer.set(scrollbar_x, abs_y, Cell::new(ch).fg(color));
            }
        }
    }
}
