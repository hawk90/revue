use super::types::*;
use crate::style::Color;
use crate::widget::theme::DARK_BG;
use crate::widget::traits::WidgetProps;

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
    pub item_fg: Option<Color>,
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
            item_fg: None,
            show_scrollbar: true,
            scrollbar_fg: Color::WHITE,
            scrollbar_bg: DARK_BG,
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
        self.item_fg = Some(color);
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
}
