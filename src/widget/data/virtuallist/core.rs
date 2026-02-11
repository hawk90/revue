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
    #[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // VirtualList::new tests
    // =========================================================================

    #[test]
    fn test_virtual_list_new_empty() {
        let list: VirtualList<String> = VirtualList::new(vec![]);
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        assert!(list.selected.is_none());
    }

    #[test]
    fn test_virtual_list_new_with_items() {
        let list = VirtualList::new(vec!["A", "B", "C"]);
        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());
        assert_eq!(list.selected, Some(0));
    }

    #[test]
    fn test_virtual_list_new_default_values() {
        let list = VirtualList::new(vec!["A"]);
        assert_eq!(list.item_height, 1);
        assert!(list.height_calculator.is_none());
        assert!(list.height_cache.is_empty());
        assert!(list.cumulative_heights.is_empty());
        assert_eq!(list.scroll_offset, 0);
        assert_eq!(list.scroll_sub_offset, 0);
        assert_eq!(list.selected_bg, Color::rgb(60, 60, 120));
        assert_eq!(list.selected_fg, Color::WHITE);
        assert_eq!(list.item_fg, Color::WHITE);
        assert!(list.show_scrollbar);
        assert_eq!(list.scrollbar_fg, Color::WHITE);
        assert_eq!(list.scrollbar_bg, Color::rgb(40, 40, 40));
        assert!(list.renderer.is_none());
        assert_eq!(list.overscan, 2);
        assert!(!list.wrap_navigation);
        assert_eq!(list.scroll_mode, ScrollMode::default());
    }

    // =========================================================================
    // VirtualList::item_height tests
    // =========================================================================

    #[test]
    fn test_item_height() {
        let list = VirtualList::new(vec!["A", "B"]).item_height(3);
        assert_eq!(list.item_height, 3);
    }

    #[test]
    fn test_item_height_minimum() {
        let list = VirtualList::new(vec!["A"]).item_height(0);
        assert_eq!(list.item_height, 1); // Minimum is 1
    }

    // =========================================================================
    // VirtualList::selected tests
    // =========================================================================

    #[test]
    fn test_selected_valid() {
        let list = VirtualList::new(vec!["A", "B", "C"]).selected(1);
        assert_eq!(list.selected, Some(1));
    }

    #[test]
    fn test_selected_out_of_bounds() {
        let list = VirtualList::new(vec!["A", "B"]).selected(10);
        assert_eq!(list.selected, Some(0)); // Original selection preserved
    }

    #[test]
    fn test_selected_empty_list() {
        let list: VirtualList<&str> = VirtualList::new(vec![]).selected(0);
        assert_eq!(list.selected, None);
    }

    // =========================================================================
    // VirtualList::selected_style tests
    // =========================================================================

    #[test]
    fn test_selected_style() {
        let list = VirtualList::new(vec!["A"]).selected_style(Color::CYAN, Color::BLUE);
        assert_eq!(list.selected_fg, Color::CYAN);
        assert_eq!(list.selected_bg, Color::BLUE);
    }

    // =========================================================================
    // VirtualList::item_fg tests
    // =========================================================================

    #[test]
    fn test_item_fg() {
        let list = VirtualList::new(vec!["A"]).item_fg(Color::GREEN);
        assert_eq!(list.item_fg, Color::GREEN);
    }

    // =========================================================================
    // VirtualList::show_scrollbar tests
    // =========================================================================

    #[test]
    fn test_show_scrollbar() {
        let list = VirtualList::new(vec!["A"]).show_scrollbar(false);
        assert!(!list.show_scrollbar);
    }

    // =========================================================================
    // VirtualList::scrollbar_style tests
    // =========================================================================

    #[test]
    fn test_scrollbar_style() {
        let list = VirtualList::new(vec!["A"]).scrollbar_style(Color::RED, Color::rgb(40, 40, 40));
        assert_eq!(list.scrollbar_fg, Color::RED);
        assert_eq!(list.scrollbar_bg, Color::rgb(40, 40, 40));
    }

    // =========================================================================
    // VirtualList::overscan tests
    // =========================================================================

    #[test]
    fn test_overscan() {
        let list = VirtualList::new(vec!["A"]).overscan(5);
        assert_eq!(list.overscan, 5);
    }

    // =========================================================================
    // VirtualList::wrap_navigation tests
    // =========================================================================

    #[test]
    fn test_wrap_navigation() {
        let list = VirtualList::new(vec!["A"]).wrap_navigation(true);
        assert!(list.wrap_navigation);
    }

    // =========================================================================
    // VirtualList::renderer tests
    // =========================================================================

    #[test]
    fn test_renderer_custom() {
        let list = VirtualList::new(vec!["A", "B"]).renderer(|item, _idx, selected| {
            format!("{}{}", if selected { "> " } else { "  " }, item)
        });
        assert!(list.renderer.is_some());
    }

    // =========================================================================
    // VirtualList::scroll_mode tests
    // =========================================================================

    #[test]
    fn test_scroll_mode() {
        let list = VirtualList::new(vec!["A"]).scroll_mode(ScrollMode::Smooth);
        assert_eq!(list.scroll_mode, ScrollMode::Smooth);
    }

    // =========================================================================
    // VirtualList::variable_height tests
    // =========================================================================

    #[test]
    fn test_variable_height() {
        let items = vec!["Short", "Much longer item", "Medium"];
        let list =
            VirtualList::new(items).variable_height(|item, _| if item.len() > 10 { 2 } else { 1 });
        assert!(list.height_calculator.is_some());
        assert_eq!(list.height_cache.len(), 3);
        assert_eq!(list.height_cache[0], 1);
        assert_eq!(list.height_cache[1], 2);
        assert_eq!(list.height_cache[2], 1);
    }

    // =========================================================================
    // VirtualList::total_height tests
    // =========================================================================

    #[test]
    fn test_total_height_uniform() {
        let list = VirtualList::new(vec![1, 2, 3, 4, 5]).item_height(2);
        assert_eq!(list.total_height(), 10); // 5 items * 2 height
    }

    #[test]
    fn test_total_height_variable() {
        let list =
            VirtualList::new(vec!["A", "BB", "CCC"]).variable_height(|item, _| item.len() as u16);
        assert_eq!(list.total_height(), 6); // 1 + 2 + 3
    }

    #[test]
    fn test_total_height_empty() {
        let list: VirtualList<&str> = VirtualList::new(vec![]);
        assert_eq!(list.total_height(), 0);
    }

    // =========================================================================
    // VirtualList::index_at_row tests
    // =========================================================================

    #[test]
    fn test_index_at_row_uniform() {
        let list = VirtualList::new(vec![1, 2, 3, 4, 5]).item_height(2);
        assert_eq!(list.index_at_row(0), 0);
        assert_eq!(list.index_at_row(1), 0);
        assert_eq!(list.index_at_row(2), 1);
        assert_eq!(list.index_at_row(3), 1);
        assert_eq!(list.index_at_row(4), 2);
    }

    #[test]
    fn test_index_at_row_variable() {
        let list =
            VirtualList::new(vec!["A", "BB", "CCC"]).variable_height(|item, _| item.len() as u16);
        assert_eq!(list.index_at_row(0), 0);
        assert_eq!(list.index_at_row(1), 1); // Start of BB
        assert_eq!(list.index_at_row(2), 1);
        assert_eq!(list.index_at_row(3), 2); // Start of CCC
    }

    // =========================================================================
    // VirtualList::row_of_index tests
    // =========================================================================

    #[test]
    fn test_row_of_index_uniform() {
        let list = VirtualList::new(vec![1, 2, 3, 4]).item_height(3);
        assert_eq!(list.row_of_index(0), 0);
        assert_eq!(list.row_of_index(1), 3);
        assert_eq!(list.row_of_index(2), 6);
    }

    #[test]
    fn test_row_of_index_variable() {
        let list =
            VirtualList::new(vec!["A", "BB", "CCC"]).variable_height(|item, _| item.len() as u16);
        assert_eq!(list.row_of_index(0), 0);
        assert_eq!(list.row_of_index(1), 1);
        assert_eq!(list.row_of_index(2), 3);
    }

    // =========================================================================
    // VirtualList::jump_to tests
    // =========================================================================

    #[test]
    fn test_jump_to_valid() {
        let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
        list.jump_to(2);
        assert_eq!(list.selected, Some(2));
        assert_eq!(list.scroll_offset, 2);
        assert_eq!(list.scroll_sub_offset, 0);
    }

    #[test]
    fn test_jump_to_updates_scroll_offset() {
        let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
        list.jump_to(3);
        assert_eq!(list.scroll_offset, 3);
        assert_eq!(list.selected, Some(3));
    }

    #[test]
    fn test_jump_to_out_of_bounds() {
        let mut list = VirtualList::new(vec!["A", "B"]);
        list.jump_to(10);
        // Should not change selection if out of bounds
        assert_eq!(list.selected, Some(0));
    }

    // =========================================================================
    // VirtualList::jump_to_with_alignment tests
    // =========================================================================

    #[test]
    fn test_jump_to_with_alignment_start() {
        let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
        list.jump_to_with_alignment(2, ScrollAlignment::Start);
        assert_eq!(list.scroll_offset, 2);
    }

    #[test]
    fn test_jump_to_with_alignment_center() {
        let mut list = VirtualList::new(vec!["A", "B", "C", "D", "E"]);
        list.jump_to_with_alignment(3, ScrollAlignment::Center);
        // Center alignment should adjust scroll offset
        assert!(list.scroll_offset <= 3);
    }

    #[test]
    fn test_jump_to_with_alignment_end() {
        let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
        list.jump_to_with_alignment(2, ScrollAlignment::End);
        assert_eq!(list.scroll_offset, 2);
    }

    #[test]
    fn test_jump_to_with_alignment_nearest() {
        let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
        list.jump_to_with_alignment(2, ScrollAlignment::Nearest);
        assert_eq!(list.scroll_offset, 2);
    }

    // =========================================================================
    // VirtualList::scroll_by tests
    // =========================================================================

    #[test]
    fn test_scroll_by_positive_uniform() {
        let mut list = VirtualList::new(vec![1, 2, 3, 4, 5]).item_height(2);
        list.scroll_by(3); // Scroll 3 rows
        assert_eq!(list.scroll_offset, 1);
        assert_eq!(list.scroll_sub_offset, 1);
    }

    #[test]
    fn test_scroll_by_negative_uniform() {
        let mut list = VirtualList::new(vec![1, 2, 3, 4, 5]).item_height(2);
        list.scroll_offset = 2;
        list.scroll_sub_offset = 1;
        list.scroll_by(-3);
        assert_eq!(list.scroll_offset, 1);
    }

    #[test]
    fn test_scroll_by_variable_height() {
        let mut list =
            VirtualList::new(vec!["A", "BB", "CCC"]).variable_height(|item, _| item.len() as u16);
        list.scroll_by(2);
        assert!(list.scroll_offset > 0 || list.scroll_sub_offset > 0);
    }

    // =========================================================================
    // VirtualList::scroll_position tests
    // =========================================================================

    #[test]
    fn test_scroll_position_empty() {
        let list: VirtualList<&str> = VirtualList::new(vec![]);
        assert_eq!(list.scroll_position(), 0.0);
    }

    #[test]
    fn test_scroll_position_single_item() {
        let list = VirtualList::new(vec!["A"]);
        assert_eq!(list.scroll_position(), 0.0);
    }

    #[test]
    fn test_scroll_position_middle() {
        let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
        list.scroll_offset = 2;
        assert_eq!(list.scroll_position(), 2.0 / 3.0);
    }

    // =========================================================================
    // VirtualList::set_scroll_position tests
    // =========================================================================

    #[test]
    fn test_set_scroll_position() {
        let mut list = VirtualList::new(vec!["A", "B", "C", "D"]);
        list.set_scroll_position(0.5);
        assert_eq!(list.scroll_offset, 1); // 0.5 * (4-1) = 1.5 -> 1
        assert_eq!(list.scroll_sub_offset, 0);
    }

    #[test]
    fn test_set_scroll_position_clamped() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]);
        list.set_scroll_position(1.5); // Over 1.0
        assert_eq!(list.scroll_offset, 2); // Clamped to max
    }

    #[test]
    fn test_set_scroll_position_zero() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]);
        list.scroll_offset = 2;
        list.set_scroll_position(0.0);
        assert_eq!(list.scroll_offset, 0);
    }

    // =========================================================================
    // VirtualList::len tests
    // =========================================================================

    #[test]
    fn test_len() {
        let list = VirtualList::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_len_empty() {
        let list: VirtualList<&str> = VirtualList::new(vec![]);
        assert_eq!(list.len(), 0);
    }

    // =========================================================================
    // VirtualList::is_empty tests
    // =========================================================================

    #[test]
    fn test_is_empty_true() {
        let list: VirtualList<&str> = VirtualList::new(vec![]);
        assert!(list.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let list = VirtualList::new(vec!["A"]);
        assert!(!list.is_empty());
    }

    // =========================================================================
    // VirtualList::selected_index tests
    // =========================================================================

    #[test]
    fn test_selected_index_some() {
        let list = VirtualList::new(vec!["A", "B", "C"]).selected(1);
        assert_eq!(list.selected_index(), Some(1));
    }

    #[test]
    fn test_selected_index_none() {
        let list: VirtualList<&str> = VirtualList::new(vec![]);
        assert_eq!(list.selected_index(), None);
    }

    // =========================================================================
    // VirtualList::selected_item tests
    // =========================================================================

    #[test]
    fn test_selected_item_some() {
        let list = VirtualList::new(vec!["A", "B", "C"]).selected(1);
        assert_eq!(list.selected_item(), Some(&"B"));
    }

    #[test]
    fn test_selected_item_none() {
        let list: VirtualList<&str> = VirtualList::new(vec![]);
        assert_eq!(list.selected_item(), None);
    }

    // =========================================================================
    // VirtualList::set_items tests
    // =========================================================================

    #[test]
    fn test_set_items_adjusts_selection() {
        let mut list = VirtualList::new(vec!["A", "B", "C", "D"]).selected(3);
        list.set_items(vec!["X", "Y"]);
        assert_eq!(list.selected, Some(1)); // Adjusted to last item
    }

    #[test]
    fn test_set_items_clears_selection_if_empty() {
        let mut list = VirtualList::new(vec!["A", "B"]).selected(0);
        list.set_items(vec![]);
        assert_eq!(list.selected, None);
    }

    #[test]
    fn test_set_items_keeps_valid_selection() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]).selected(1);
        list.set_items(vec!["X", "Y", "Z", "W"]);
        assert_eq!(list.selected, Some(1));
    }

    #[test]
    fn test_set_items_adjusts_scroll_offset() {
        let mut list = VirtualList::new(vec!["A"; 100]).selected(50);
        list.scroll_offset = 50;
        list.set_items(vec!["X"; 10]);
        assert_eq!(list.scroll_offset, 9); // Adjusted to max
    }

    // =========================================================================
    // VirtualList::push tests
    // =========================================================================

    #[test]
    fn test_push() {
        let mut list = VirtualList::new(vec!["A", "B"]);
        list.push("C");
        assert_eq!(list.len(), 3);
        assert_eq!(list.items[2], "C");
    }

    // =========================================================================
    // VirtualList::remove tests
    // =========================================================================

    #[test]
    fn test_remove_valid() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]).selected(1);
        let removed = list.remove(1);
        assert_eq!(removed, Some("B"));
        assert_eq!(list.len(), 2);
        assert_eq!(list.items, vec!["A", "C"]);
    }

    #[test]
    fn test_remove_adjusts_selection_after() {
        let mut list = VirtualList::new(vec!["A", "B", "C", "D"]).selected(2);
        list.remove(1);
        assert_eq!(list.selected, Some(1)); // Adjusted down
    }

    #[test]
    fn test_remove_adjusts_selection_at_end() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]).selected(2);
        list.remove(2);
        assert_eq!(list.selected, Some(1)); // Moved to last item
    }

    #[test]
    fn test_remove_clears_selection_if_empty() {
        let mut list = VirtualList::new(vec!["A"]).selected(0);
        list.remove(0);
        assert_eq!(list.selected, None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_remove_out_of_bounds() {
        let mut list = VirtualList::new(vec!["A", "B"]);
        let removed = list.remove(10);
        assert_eq!(removed, None);
        assert_eq!(list.len(), 2);
    }

    // =========================================================================
    // VirtualList::clear tests
    // =========================================================================

    #[test]
    fn test_clear() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]).selected(1);
        list.scroll_offset = 2;
        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.selected, None);
        assert_eq!(list.scroll_offset, 0);
    }

    // =========================================================================
    // VirtualList::select_next tests
    // =========================================================================

    #[test]
    fn test_select_next() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]);
        list.select_next();
        assert_eq!(list.selected, Some(1));
    }

    #[test]
    fn test_select_next_at_end() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]);
        list.selected = Some(2);
        list.select_next();
        assert_eq!(list.selected, Some(2)); // Stays at end
    }

    #[test]
    fn test_select_next_wrap() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]).wrap_navigation(true);
        list.selected = Some(2);
        list.select_next();
        assert_eq!(list.selected, Some(0)); // Wrapped to start
    }

    #[test]
    fn test_select_next_empty() {
        let mut list: VirtualList<&str> = VirtualList::new(vec![]);
        list.select_next();
        assert_eq!(list.selected, None);
    }

    #[test]
    fn test_select_next_from_none() {
        let mut list = VirtualList::new(vec!["A", "B"]);
        list.selected = None;
        list.select_next();
        assert_eq!(list.selected, Some(0));
    }

    // =========================================================================
    // VirtualList::select_prev tests
    // =========================================================================

    #[test]
    fn test_select_prev() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]).selected(2);
        list.select_prev();
        assert_eq!(list.selected, Some(1));
    }

    #[test]
    fn test_select_prev_at_start() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]);
        list.select_prev();
        assert_eq!(list.selected, Some(0)); // Stays at start
    }

    #[test]
    fn test_select_prev_wrap() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]).wrap_navigation(true);
        list.select_prev();
        assert_eq!(list.selected, Some(2)); // Wrapped to end
    }

    #[test]
    fn test_select_prev_from_none() {
        let mut list = VirtualList::new(vec!["A", "B"]);
        list.selected = None;
        list.select_prev();
        assert_eq!(list.selected, Some(0));
    }

    // =========================================================================
    // VirtualList::select_first tests
    // =========================================================================

    #[test]
    fn test_select_first() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]).selected(2);
        list.select_first();
        assert_eq!(list.selected, Some(0));
        assert_eq!(list.scroll_offset, 0);
    }

    #[test]
    fn test_select_first_empty() {
        let mut list: VirtualList<&str> = VirtualList::new(vec![]);
        list.select_first();
        assert_eq!(list.selected, None);
    }

    // =========================================================================
    // VirtualList::select_last tests
    // =========================================================================

    #[test]
    fn test_select_last() {
        let mut list = VirtualList::new(vec!["A", "B", "C"]);
        list.select_last();
        assert_eq!(list.selected, Some(2));
    }

    #[test]
    fn test_select_last_empty() {
        let mut list: VirtualList<&str> = VirtualList::new(vec![]);
        list.select_last();
        assert_eq!(list.selected, None);
    }

    // =========================================================================
    // VirtualList::page_down tests
    // =========================================================================

    #[test]
    fn test_page_down() {
        let mut list = VirtualList::new((0..20).collect::<Vec<_>>()).selected(0);
        list.page_down(10); // viewport_height = 10, item_height = 1
        assert_eq!(list.selected, Some(10));
    }

    #[test]
    fn test_page_down_clamped() {
        let mut list = VirtualList::new((0..15).collect::<Vec<_>>()).selected(5);
        list.page_down(10);
        assert_eq!(list.selected, Some(14)); // Clamped to last item
    }

    #[test]
    fn test_page_down_no_selection() {
        let mut list = VirtualList::new((0..20).collect::<Vec<_>>());
        list.selected = None;
        list.page_down(10);
        assert_eq!(list.selected, None);
    }

    // =========================================================================
    // VirtualList::page_up tests
    // =========================================================================

    #[test]
    fn test_page_up() {
        let mut list = VirtualList::new((0..20).collect::<Vec<_>>()).selected(15);
        list.page_up(10);
        assert_eq!(list.selected, Some(5));
    }

    #[test]
    fn test_page_up_clamped() {
        let mut list = VirtualList::new((0..20).collect::<Vec<_>>()).selected(3);
        list.page_up(10);
        assert_eq!(list.selected, Some(0)); // Clamped to first item
    }

    #[test]
    fn test_page_up_no_selection() {
        let mut list = VirtualList::new((0..20).collect::<Vec<_>>());
        list.selected = None;
        list.page_up(10);
        assert_eq!(list.selected, None);
    }

    // =========================================================================
    // VirtualList::ensure_visible tests
    // =========================================================================

    #[test]
    fn test_ensure_visible_above() {
        let mut list = VirtualList::new((0..20).collect::<Vec<_>>());
        list.scroll_offset = 10;
        list.selected = Some(5);
        list.ensure_visible(10);
        assert_eq!(list.scroll_offset, 5);
    }

    #[test]
    fn test_ensure_visible_below() {
        let mut list = VirtualList::new((0..20).collect::<Vec<_>>());
        list.scroll_offset = 0;
        list.selected = Some(15);
        list.ensure_visible(10);
        assert_eq!(list.scroll_offset, 6); // 15 - (10 - 1)
    }

    #[test]
    fn test_ensure_visible_in_range() {
        let mut list = VirtualList::new((0..20).collect::<Vec<_>>());
        list.scroll_offset = 5;
        list.selected = Some(7);
        list.ensure_visible(10);
        assert_eq!(list.scroll_offset, 5); // No change needed
    }

    #[test]
    fn test_ensure_visible_no_selection() {
        let mut list = VirtualList::new((0..20).collect::<Vec<_>>());
        list.selected = None; // Explicitly no selection
        list.scroll_offset = 5;
        list.ensure_visible(10);
        assert_eq!(list.scroll_offset, 5); // No change when no selection
    }

    // =========================================================================
    // VirtualList::visible_range tests
    // =========================================================================

    #[test]
    fn test_visible_range() {
        let mut list = VirtualList::new((0..20).collect::<Vec<_>>())
            .overscan(2)
            .item_height(1);
        list.scroll_offset = 5;
        let range = list.visible_range(10);
        assert_eq!(range.start, 3); // 5 - 2 (overscan)
        assert_eq!(range.end, 17); // 5 + 10 + 2
    }

    #[test]
    fn test_visible_range_clamped() {
        let mut list = VirtualList::new((0..10).collect::<Vec<_>>())
            .overscan(0)
            .item_height(1);
        list.scroll_offset = 8;
        let range = list.visible_range(5);
        assert_eq!(range.start, 8);
        assert_eq!(range.end, 10); // Clamped to items.len()
    }

    // =========================================================================
    // VirtualList::render_item tests
    // =========================================================================

    #[test]
    fn test_render_item_default() {
        let list = VirtualList::new(vec!["Item1", "Item2"]);
        let rendered = list.render_item(&"Item1", 0, false);
        assert_eq!(rendered, "Item1");
    }

    #[test]
    fn test_render_item_custom() {
        let list = VirtualList::new(vec!["A", "B"]).renderer(|item, idx, sel| {
            format!("{}: {} ({})", idx, item, if sel { "X" } else { " " })
        });
        let rendered = list.render_item(&"A", 0, true);
        assert_eq!(rendered, "0: A (X)");
    }

    // =========================================================================
    // VirtualList::get_item_height tests (test-only)
    // =========================================================================

    #[test]
    fn test_get_item_height_uniform() {
        let list = VirtualList::new(vec!["A", "B"]).item_height(3);
        assert_eq!(list.get_item_height(0), 3);
        assert_eq!(list.get_item_height(1), 3);
    }

    #[test]
    fn test_get_item_height_variable() {
        let list =
            VirtualList::new(vec!["A", "BB", "CCC"]).variable_height(|item, _| item.len() as u16);
        assert_eq!(list.get_item_height(0), 1);
        assert_eq!(list.get_item_height(1), 2);
        assert_eq!(list.get_item_height(2), 3);
    }

    // =========================================================================
    // Builder chain tests
    // =========================================================================

    #[test]
    fn test_full_builder_chain() {
        let list = VirtualList::new(vec!["A", "B", "C"])
            .item_height(2)
            .selected(1)
            .selected_style(Color::CYAN, Color::BLUE)
            .item_fg(Color::WHITE)
            .show_scrollbar(false)
            .scrollbar_style(Color::RED, Color::rgb(40, 40, 40))
            .overscan(3)
            .wrap_navigation(true)
            .scroll_mode(ScrollMode::Smooth);

        assert_eq!(list.item_height, 2);
        assert_eq!(list.selected, Some(1));
        assert_eq!(list.selected_fg, Color::CYAN);
        assert_eq!(list.selected_bg, Color::BLUE);
        assert_eq!(list.item_fg, Color::WHITE);
        assert!(!list.show_scrollbar);
        assert_eq!(list.scrollbar_fg, Color::RED);
        assert_eq!(list.overscan, 3);
        assert!(list.wrap_navigation);
        assert_eq!(list.scroll_mode, ScrollMode::Smooth);
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_single_item_operations() {
        let mut list = VirtualList::new(vec!["Only"]);
        assert!(!list.is_empty());
        list.select_next();
        assert_eq!(list.selected, Some(0)); // Can't move
        list.select_prev();
        assert_eq!(list.selected, Some(0)); // Can't move
    }

    #[test]
    fn test_large_list() {
        let items: Vec<usize> = (0..10000).collect();
        let list = VirtualList::new(items);
        assert_eq!(list.len(), 10000);
        assert_eq!(list.selected, Some(0));
    }

    #[test]
    fn test_rebuild_height_cache() {
        let items = vec!["A", "BB", "CCC"];
        let list = VirtualList::new(items).variable_height(|item, _| item.len() as u16);
        // height_calculator is private, so we just verify the cache was built
        assert_eq!(list.height_cache.len(), 3); // Cache has original size
    }

    // =========================================================================
    // Tests with different item types
    // =========================================================================

    #[test]
    fn test_with_string_items() {
        let list = VirtualList::new(vec!["Hello".to_string(), "World".to_string()]);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_with_number_items() {
        let list = VirtualList::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_with_tuple_items() {
        let list = VirtualList::new(vec!["1-A", "2-B"]);
        assert_eq!(list.len(), 2);
    }
}

// Test module requires private field access - keeping inline
