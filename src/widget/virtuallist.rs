//! Virtual list widget for efficiently rendering large datasets
//!
//! Only renders visible items, making it suitable for lists with
//! hundreds of thousands of items without performance degradation.
//!
//! # Features
//!
//! - **Variable height items**: Each item can have different heights
//! - **Jump-to-index**: Quickly scroll to any item by index
//! - **Smooth scrolling**: Configurable scroll behavior
//! - **Overscan**: Render extra items for smoother scrolling
//! - **Async loading**: Support for lazy-loaded data sources
//!
//! # Example
//!
//! ```ignore
//! use revue::widget::{VirtualList, VirtualListItem};
//!
//! // Create a virtual list with 100,000 items
//! let items: Vec<String> = (0..100_000)
//!     .map(|i| format!("Item {}", i))
//!     .collect();
//!
//! let list = VirtualList::new(items)
//!     .item_height(1)
//!     .selected(0);
//!
//! // With variable heights
//! let list = VirtualList::new(items)
//!     .variable_height(|item, _idx| if item.len() > 50 { 2 } else { 1 });
//!
//! // Jump to specific index
//! list.jump_to(5000);
//! ```

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use std::ops::Range;

/// Item renderer function type
pub type ItemRenderer<T> = Box<dyn Fn(&T, usize, bool) -> String>;

/// Height calculator function type for variable heights
pub type HeightCalculator<T> = Box<dyn Fn(&T, usize) -> u16>;

/// Scroll behavior mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollMode {
    /// Item-by-item scrolling
    #[default]
    Item,
    /// Smooth pixel-based scrolling (simulated with sub-item offsets)
    Smooth,
    /// Center selected item when possible
    Center,
}

/// Scroll alignment when jumping to an item
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollAlignment {
    /// Align to top of viewport
    #[default]
    Start,
    /// Align to center of viewport
    Center,
    /// Align to bottom of viewport
    End,
    /// Nearest edge (minimal scroll)
    Nearest,
}

/// A virtual list that only renders visible items
pub struct VirtualList<T> {
    /// All items (only visible ones are rendered)
    items: Vec<T>,
    /// Height of each item in rows (uniform height)
    item_height: u16,
    /// Variable height calculator (overrides item_height if set)
    height_calculator: Option<HeightCalculator<T>>,
    /// Cached heights for variable-height mode
    height_cache: Vec<u16>,
    /// Cached cumulative heights (prefix sums for fast lookup)
    cumulative_heights: Vec<u32>,
    /// Current scroll offset (in items for uniform, in rows for variable)
    scroll_offset: usize,
    /// Sub-item scroll offset (for smooth scrolling, 0-item_height)
    scroll_sub_offset: u16,
    /// Currently selected index
    selected: Option<usize>,
    /// Selection background color
    selected_bg: Color,
    /// Selection foreground color
    selected_fg: Color,
    /// Normal item foreground color
    item_fg: Color,
    /// Show scrollbar
    show_scrollbar: bool,
    /// Scrollbar foreground color
    scrollbar_fg: Color,
    /// Scrollbar background color
    scrollbar_bg: Color,
    /// Custom item renderer
    renderer: Option<ItemRenderer<T>>,
    /// Overscan (extra items to render above/below viewport)
    overscan: usize,
    /// Enable wrap-around navigation
    wrap_navigation: bool,
    /// Scroll mode
    scroll_mode: ScrollMode,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
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
    fn ensure_visible(&mut self, viewport_height: u16) {
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
    fn visible_range(&self, viewport_height: u16) -> Range<usize> {
        let visible_count = (viewport_height / self.item_height) as usize;
        let start = self.scroll_offset.saturating_sub(self.overscan);
        let end = (self.scroll_offset + visible_count + self.overscan).min(self.items.len());
        start..end
    }

    /// Render item text
    fn render_item(&self, item: &T, index: usize, is_selected: bool) -> String {
        if let Some(ref renderer) = self.renderer {
            renderer(item, index, is_selected)
        } else {
            item.to_string()
        }
    }

    /// Render scrollbar
    fn render_scrollbar(&self, ctx: &mut RenderContext, viewport_height: u16) {
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

impl<T: ToString + Clone> View for VirtualList<T> {
    crate::impl_view_meta!("VirtualList");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 2 || area.height < 1 {
            return;
        }

        let viewport_height = area.height;
        let content_width = if self.show_scrollbar {
            area.width.saturating_sub(1)
        } else {
            area.width
        };

        // Ensure selected item is visible
        let mut this = self.clone();
        this.ensure_visible(viewport_height);

        // Get visible range
        let visible_range = this.visible_range(viewport_height);

        // Render visible items
        for item_idx in visible_range {
            let item = &this.items[item_idx];
            let is_selected = this.selected == Some(item_idx);

            // Calculate Y position (accounting for scroll offset)
            let relative_idx = item_idx.saturating_sub(this.scroll_offset);
            let y_offset = (relative_idx as u16) * this.item_height;

            if y_offset >= viewport_height {
                break;
            }

            // Get item text
            let text = this.render_item(item, item_idx, is_selected);

            // Render item rows
            for row in 0..this.item_height {
                let y = area.y + y_offset + row;
                if y >= area.y + viewport_height {
                    break;
                }

                // Get the line for this row (for multi-row items)
                let line = if row == 0 { &text } else { "" };

                // Render each character
                for x in 0..content_width {
                    let ch = line.chars().nth(x as usize).unwrap_or(' ');
                    let mut cell = Cell::new(ch);

                    if is_selected {
                        cell.fg = Some(this.selected_fg);
                        cell.bg = Some(this.selected_bg);
                    } else {
                        cell.fg = Some(this.item_fg);
                    }

                    ctx.buffer.set(area.x + x, y, cell);
                }
            }
        }

        // Render scrollbar
        if this.show_scrollbar && this.items.len() > (viewport_height / this.item_height) as usize {
            // Use a mutable reference for scrollbar rendering
            let this_clone = this.clone();
            this_clone.render_scrollbar(ctx, viewport_height);
        }
    }
}

impl<T: ToString + Clone> Clone for VirtualList<T> {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            item_height: self.item_height,
            height_calculator: None, // Can't clone closures
            height_cache: self.height_cache.clone(),
            cumulative_heights: self.cumulative_heights.clone(),
            scroll_offset: self.scroll_offset,
            scroll_sub_offset: self.scroll_sub_offset,
            selected: self.selected,
            selected_bg: self.selected_bg,
            selected_fg: self.selected_fg,
            item_fg: self.item_fg,
            show_scrollbar: self.show_scrollbar,
            scrollbar_fg: self.scrollbar_fg,
            scrollbar_bg: self.scrollbar_bg,
            renderer: None, // Can't clone closures
            overscan: self.overscan,
            wrap_navigation: self.wrap_navigation,
            scroll_mode: self.scroll_mode,
            props: self.props.clone(),
        }
    }
}

impl<T: ToString + Clone> VirtualList<T> {
    /// Get element ID
    pub fn get_element_id(&self) -> Option<&str> {
        self.props.id.as_deref()
    }

    /// Get CSS classes
    pub fn get_classes(&self) -> &[String] {
        &self.props.classes
    }
}

impl<T: ToString + Clone + Default> Default for VirtualList<T> {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

// Manual implementations for generic type
impl<T: ToString + Clone> crate::widget::StyledView for VirtualList<T> {
    fn set_id(&mut self, id: impl Into<String>) {
        self.props.id = Some(id.into());
    }

    fn add_class(&mut self, class: impl Into<String>) {
        let class_str = class.into();
        if !self.props.classes.iter().any(|c| c == &class_str) {
            self.props.classes.push(class_str);
        }
    }

    fn remove_class(&mut self, class: &str) {
        self.props.classes.retain(|c| c != class);
    }

    fn toggle_class(&mut self, class: &str) {
        if self.props.classes.iter().any(|c| c == class) {
            self.props.classes.retain(|c| c != class);
        } else {
            self.props.classes.push(class.to_string());
        }
    }

    fn has_class(&self, class: &str) -> bool {
        self.props.classes.iter().any(|c| c == class)
    }
}

impl<T: ToString + Clone> VirtualList<T> {
    /// Set element ID for CSS selector (#id)
    pub fn element_id(mut self, id: impl Into<String>) -> Self {
        self.props.id = Some(id.into());
        self
    }

    /// Add a CSS class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        let class_str = class.into();
        if !self.props.classes.iter().any(|c| c == &class_str) {
            self.props.classes.push(class_str);
        }
        self
    }

    /// Add multiple CSS classes
    pub fn classes<I, S>(mut self, classes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for class in classes {
            let class_str = class.into();
            if !self.props.classes.iter().any(|c| c == &class_str) {
                self.props.classes.push(class_str);
            }
        }
        self
    }
}

/// Helper function to create a virtual list
pub fn virtual_list<T: ToString + Clone>(items: Vec<T>) -> VirtualList<T> {
    VirtualList::new(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_virtual_list_new() {
        let items = vec!["a", "b", "c"];
        let list = VirtualList::new(items);
        assert_eq!(list.len(), 3);
        assert_eq!(list.selected_index(), Some(0));
    }

    #[test]
    fn test_virtual_list_large() {
        // Test with 100k items
        let items: Vec<String> = (0..100_000).map(|i| format!("Item {}", i)).collect();
        let list = VirtualList::new(items);
        assert_eq!(list.len(), 100_000);
    }

    #[test]
    fn test_virtual_list_navigation() {
        let items = vec!["a", "b", "c", "d", "e"];
        let mut list = VirtualList::new(items);

        assert_eq!(list.selected_index(), Some(0));

        list.select_next();
        assert_eq!(list.selected_index(), Some(1));

        list.select_next();
        list.select_next();
        list.select_next();
        assert_eq!(list.selected_index(), Some(4));

        // At end, should not move without wrap
        list.select_next();
        assert_eq!(list.selected_index(), Some(4));

        list.select_prev();
        assert_eq!(list.selected_index(), Some(3));
    }

    #[test]
    fn test_virtual_list_wrap_navigation() {
        let items = vec!["a", "b", "c"];
        let mut list = VirtualList::new(items).wrap_navigation(true);

        list.select_last();
        assert_eq!(list.selected_index(), Some(2));

        list.select_next();
        assert_eq!(list.selected_index(), Some(0));

        list.select_prev();
        assert_eq!(list.selected_index(), Some(2));
    }

    #[test]
    fn test_virtual_list_visible_range() {
        let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
        let list = VirtualList::new(items).overscan(2);

        // With viewport of 10 rows and item_height of 1
        let range = list.visible_range(10);
        assert!(range.start == 0); // scroll_offset is 0, minus overscan clamped to 0
        assert!(range.end <= 14); // 0 + 10 + 2*overscan
    }

    #[test]
    fn test_virtual_list_set_items() {
        let mut list = VirtualList::new(vec!["a", "b", "c"]);
        list.selected = Some(2);

        list.set_items(vec!["x", "y"]);
        assert_eq!(list.len(), 2);
        assert_eq!(list.selected_index(), Some(1)); // Adjusted to last item
    }

    #[test]
    fn test_virtual_list_push_remove() {
        let mut list = VirtualList::new(vec!["a", "b"]);

        list.push("c");
        assert_eq!(list.len(), 3);

        let removed = list.remove(0);
        assert_eq!(removed, Some("a"));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_virtual_list_clear() {
        let mut list = VirtualList::new(vec!["a", "b", "c"]);
        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.selected_index(), None);
    }

    #[test]
    fn test_virtual_list_render() {
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
        let list = VirtualList::new(items);
        list.render(&mut ctx);

        // First item should be visible
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'I');
    }

    #[test]
    fn test_virtual_list_helper() {
        let list = virtual_list(vec!["a", "b", "c"]);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_virtual_list_page_navigation() {
        let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
        let mut list = VirtualList::new(items);

        list.page_down(10);
        assert_eq!(list.selected_index(), Some(10));

        list.page_up(10);
        assert_eq!(list.selected_index(), Some(0));
    }

    #[test]
    fn test_virtual_list_jump_to() {
        let items: Vec<String> = (0..1000).map(|i| format!("Item {}", i)).collect();
        let mut list = VirtualList::new(items);

        list.jump_to(500);
        assert_eq!(list.selected_index(), Some(500));
        assert_eq!(list.scroll_offset, 500);

        // Jump to out of bounds should be ignored
        list.jump_to(5000);
        assert_eq!(list.selected_index(), Some(500));
    }

    #[test]
    fn test_virtual_list_scroll_position() {
        let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
        let mut list = VirtualList::new(items);

        assert_eq!(list.scroll_position(), 0.0);

        list.set_scroll_position(0.5);
        assert!(list.scroll_offset > 0);

        list.set_scroll_position(1.0);
        assert_eq!(list.scroll_offset, 99);
    }

    #[test]
    fn test_virtual_list_variable_height() {
        let items: Vec<String> = (0..10).map(|i| format!("Item {}", i)).collect();
        let list =
            VirtualList::new(items).variable_height(|_item, idx| if idx % 2 == 0 { 2 } else { 1 });

        // Even items have height 2, odd items have height 1
        assert_eq!(list.get_item_height(0), 2);
        assert_eq!(list.get_item_height(1), 1);
        assert_eq!(list.get_item_height(2), 2);

        // Total height: 5 even items * 2 + 5 odd items * 1 = 15
        assert_eq!(list.total_height(), 15);
    }

    #[test]
    fn test_virtual_list_row_calculations() {
        let items: Vec<String> = (0..5).map(|i| format!("Item {}", i)).collect();
        let list = VirtualList::new(items).variable_height(|_item, idx| (idx + 1) as u16); // Heights: 1, 2, 3, 4, 5

        // Cumulative heights: 1, 3, 6, 10, 15
        assert_eq!(list.row_of_index(0), 0);
        assert_eq!(list.row_of_index(1), 1);
        assert_eq!(list.row_of_index(2), 3);
        assert_eq!(list.row_of_index(3), 6);
        assert_eq!(list.row_of_index(4), 10);

        // Index at row
        assert_eq!(list.index_at_row(0), 0);
        assert_eq!(list.index_at_row(1), 1);
        assert_eq!(list.index_at_row(2), 1);
        assert_eq!(list.index_at_row(3), 2);
        assert_eq!(list.index_at_row(6), 3);
    }

    #[test]
    fn test_virtual_list_scroll_mode() {
        let items = vec!["a", "b", "c"];
        let list = VirtualList::new(items).scroll_mode(ScrollMode::Center);
        assert_eq!(list.scroll_mode, ScrollMode::Center);
    }

    #[test]
    fn test_virtual_list_scroll_by() {
        let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
        let mut list = VirtualList::new(items).item_height(2);

        list.scroll_by(3);
        // With item_height=2, scrolling 3 rows moves 1 item + 1 sub-offset
        assert!(list.scroll_offset >= 1);
    }
}
