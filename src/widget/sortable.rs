//! Sortable list widget with drag-and-drop reordering
//!
//! A list widget that allows items to be reordered via drag-and-drop.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::SortableList;
//!
//! let items = vec!["First", "Second", "Third"];
//! SortableList::new(items)
//!     .on_reorder(|from, to| {
//!         println!("Moved item from {} to {}", from, to);
//!     })
//! ```

use std::sync::atomic::{AtomicU64, Ordering};

use crate::event::drag::{DragData, DragId};
use crate::event::{KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use crate::impl_view_meta;
use crate::layout::Rect;
use crate::style::Color;
use crate::widget::traits::{
    Draggable, EventResult, Interactive, RenderContext, View, WidgetProps, WidgetState,
};

/// Atomic counter for generating unique sortable list IDs
static SORTABLE_ID_COUNTER: AtomicU64 = AtomicU64::new(1000);

/// Item in a sortable list
#[derive(Debug, Clone)]
pub struct SortableItem {
    /// Item label
    pub label: String,
    /// Is item selected
    pub selected: bool,
    /// Is item being dragged
    pub dragging: bool,
    /// Original index (before any reordering)
    pub original_index: usize,
}

impl SortableItem {
    /// Create a new sortable item
    pub fn new(label: impl Into<String>, index: usize) -> Self {
        Self {
            label: label.into(),
            selected: false,
            dragging: false,
            original_index: index,
        }
    }
}

/// Reorder callback type
pub type ReorderCallback = Box<dyn FnMut(usize, usize)>;

/// Sortable list widget
pub struct SortableList {
    /// List items
    items: Vec<SortableItem>,
    /// Selected item index
    selected: Option<usize>,
    /// Scroll offset
    scroll: usize,
    /// Item being dragged (index)
    dragging: Option<usize>,
    /// Drop target index (where to insert)
    drop_target: Option<usize>,
    /// Reorder callback
    on_reorder: Option<ReorderCallback>,
    /// Item height (usually 1)
    item_height: u16,
    /// Show drag handles
    show_handles: bool,
    /// Normal item color
    item_color: Color,
    /// Selected item color
    selected_color: Color,
    /// Drag indicator color
    drag_color: Color,
    /// Widget state (for future focus management)
    _state: WidgetState,
    /// Widget props
    props: WidgetProps,
    /// Unique ID for drag operations (for future drag tracking)
    _id: DragId,
}

impl SortableList {
    /// Create a new sortable list
    pub fn new<I, S>(items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let id = SORTABLE_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        let items: Vec<SortableItem> = items
            .into_iter()
            .enumerate()
            .map(|(i, s)| SortableItem::new(s, i))
            .collect();

        Self {
            items,
            selected: None,
            scroll: 0,
            dragging: None,
            drop_target: None,
            on_reorder: None,
            item_height: 1,
            show_handles: true,
            item_color: Color::rgb(200, 200, 200),
            selected_color: Color::rgb(100, 150, 255),
            drag_color: Color::rgb(255, 200, 100),
            _state: WidgetState::new(),
            props: WidgetProps::new(),
            _id: id,
        }
    }

    /// Set reorder callback
    pub fn on_reorder<F>(mut self, callback: F) -> Self
    where
        F: FnMut(usize, usize) + 'static,
    {
        self.on_reorder = Some(Box::new(callback));
        self
    }

    /// Show or hide drag handles
    pub fn handles(mut self, show: bool) -> Self {
        self.show_handles = show;
        self
    }

    /// Set item color
    pub fn item_color(mut self, color: Color) -> Self {
        self.item_color = color;
        self
    }

    /// Set selected color
    pub fn selected_color(mut self, color: Color) -> Self {
        self.selected_color = color;
        self
    }

    /// Get items
    pub fn items(&self) -> &[SortableItem] {
        &self.items
    }

    /// Get mutable items
    pub fn items_mut(&mut self) -> &mut Vec<SortableItem> {
        &mut self.items
    }

    /// Get selected index
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Set selected index
    pub fn set_selected(&mut self, index: Option<usize>) {
        self.selected = index;
    }

    /// Select next item
    pub fn select_next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => (i + 1).min(self.items.len() - 1),
            None => 0,
        });
    }

    /// Select previous item
    pub fn select_prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => i.saturating_sub(1),
            None => 0,
        });
    }

    /// Move selected item up
    pub fn move_up(&mut self) {
        if let Some(idx) = self.selected {
            if idx > 0 {
                self.items.swap(idx, idx - 1);
                self.selected = Some(idx - 1);
                if let Some(ref mut callback) = self.on_reorder {
                    callback(idx, idx - 1);
                }
            }
        }
    }

    /// Move selected item down
    pub fn move_down(&mut self) {
        if let Some(idx) = self.selected {
            if idx < self.items.len() - 1 {
                self.items.swap(idx, idx + 1);
                self.selected = Some(idx + 1);
                if let Some(ref mut callback) = self.on_reorder {
                    callback(idx, idx + 1);
                }
            }
        }
    }

    /// Start dragging selected item
    pub fn start_drag(&mut self) {
        if let Some(idx) = self.selected {
            self.dragging = Some(idx);
            self.items[idx].dragging = true;
        }
    }

    /// End drag and perform reorder
    pub fn end_drag(&mut self) {
        if let (Some(from), Some(to)) = (self.dragging, self.drop_target) {
            if from != to {
                let item = self.items.remove(from);
                let insert_idx = if to > from { to - 1 } else { to };
                self.items.insert(insert_idx, item);
                self.selected = Some(insert_idx);

                if let Some(ref mut callback) = self.on_reorder {
                    callback(from, insert_idx);
                }
            }
        }

        // Reset drag state
        if let Some(idx) = self.dragging {
            if idx < self.items.len() {
                self.items[idx].dragging = false;
            }
        }
        self.dragging = None;
        self.drop_target = None;
    }

    /// Cancel drag
    pub fn cancel_drag(&mut self) {
        if let Some(idx) = self.dragging {
            if idx < self.items.len() {
                self.items[idx].dragging = false;
            }
        }
        self.dragging = None;
        self.drop_target = None;
    }

    /// Update drop target based on y position
    pub fn update_drop_target(&mut self, y: u16, area_y: u16) {
        if self.dragging.is_some() {
            let relative_y = y.saturating_sub(area_y) as usize;
            let target_idx =
                (relative_y / self.item_height as usize + self.scroll).min(self.items.len());
            self.drop_target = Some(target_idx);
        }
    }

    /// Check if currently dragging
    pub fn is_dragging(&self) -> bool {
        self.dragging.is_some()
    }

    /// Add an item
    pub fn push(&mut self, label: impl Into<String>) {
        let idx = self.items.len();
        self.items.push(SortableItem::new(label, idx));
    }

    /// Remove an item by index
    pub fn remove(&mut self, index: usize) -> Option<SortableItem> {
        if index < self.items.len() {
            let item = self.items.remove(index);
            if let Some(sel) = self.selected {
                if sel >= self.items.len() {
                    self.selected = if self.items.is_empty() {
                        None
                    } else {
                        Some(self.items.len() - 1)
                    };
                }
            }
            Some(item)
        } else {
            None
        }
    }

    /// Get current order as indices
    pub fn order(&self) -> Vec<usize> {
        self.items.iter().map(|i| i.original_index).collect()
    }
}

impl View for SortableList {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let visible_count = (area.height / self.item_height) as usize;

        for (i, item) in self
            .items
            .iter()
            .enumerate()
            .skip(self.scroll)
            .take(visible_count)
        {
            let y = area.y + ((i - self.scroll) as u16 * self.item_height);

            // Determine colors
            let is_selected = self.selected == Some(i);
            let is_dragging = item.dragging;
            let is_drop_target = self.drop_target == Some(i);

            let fg = if is_dragging {
                self.drag_color
            } else if is_selected {
                self.selected_color
            } else {
                self.item_color
            };

            // Draw drop indicator
            if is_drop_target && self.dragging.is_some() {
                ctx.draw_hline(area.x, y, area.width, '─', self.drag_color);
                continue;
            }

            // Draw handle if enabled
            let mut x = area.x;
            if self.show_handles {
                let handle = if is_dragging { "↕ " } else { "≡ " };
                for ch in handle.chars() {
                    if let Some(cell) = ctx.buffer.get_mut(x, y) {
                        cell.symbol = ch;
                        cell.fg = Some(Color::rgb(100, 100, 100));
                    }
                    x += 1;
                }
            }

            // Selection indicator
            let prefix = if is_selected { "▶ " } else { "  " };
            for ch in prefix.chars() {
                if let Some(cell) = ctx.buffer.get_mut(x, y) {
                    cell.symbol = ch;
                    cell.fg = Some(fg);
                }
                x += 1;
            }

            // Item label
            let max_len = (area.x + area.width).saturating_sub(x) as usize;
            for (j, ch) in item.label.chars().take(max_len).enumerate() {
                if let Some(cell) = ctx.buffer.get_mut(x + j as u16, y) {
                    cell.symbol = ch;
                    cell.fg = Some(fg);
                    if is_selected {
                        cell.modifier |= crate::render::Modifier::BOLD;
                    }
                    if is_dragging {
                        cell.modifier |= crate::render::Modifier::DIM;
                    }
                }
            }
        }

        // Draw final drop indicator at end if needed
        if let Some(target) = self.drop_target {
            if target == self.items.len() && self.dragging.is_some() {
                let y = area.y
                    + (visible_count.min(self.items.len() - self.scroll) as u16 * self.item_height);
                if y < area.y + area.height {
                    ctx.draw_hline(area.x, y, area.width, '─', self.drag_color);
                }
            }
        }
    }

    impl_view_meta!("SortableList");
}

impl Interactive for SortableList {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        use crate::event::Key;

        match event.key {
            Key::Up | Key::Char('k') => {
                if event.shift || event.alt {
                    self.move_up();
                } else {
                    self.select_prev();
                }
                EventResult::ConsumedAndRender
            }
            Key::Down | Key::Char('j') => {
                if event.shift || event.alt {
                    self.move_down();
                } else {
                    self.select_next();
                }
                EventResult::ConsumedAndRender
            }
            Key::Home => {
                self.selected = if self.items.is_empty() { None } else { Some(0) };
                EventResult::ConsumedAndRender
            }
            Key::End => {
                self.selected = if self.items.is_empty() {
                    None
                } else {
                    Some(self.items.len() - 1)
                };
                EventResult::ConsumedAndRender
            }
            Key::Escape if self.is_dragging() => {
                self.cancel_drag();
                EventResult::ConsumedAndRender
            }
            _ => EventResult::Ignored,
        }
    }

    fn handle_mouse(&mut self, event: &MouseEvent, area: Rect) -> EventResult {
        if !area.contains(event.x, event.y) {
            return EventResult::Ignored;
        }

        let relative_y = event.y.saturating_sub(area.y) as usize;
        let clicked_idx = (relative_y / self.item_height as usize + self.scroll)
            .min(self.items.len().saturating_sub(1));

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                self.selected = Some(clicked_idx);
                // Check if clicking on handle area to start drag
                if self.show_handles && event.x < area.x + 2 {
                    self.start_drag();
                }
                EventResult::ConsumedAndRender
            }
            MouseEventKind::Drag(MouseButton::Left) if self.is_dragging() => {
                self.update_drop_target(event.y, area.y);
                EventResult::ConsumedAndRender
            }
            MouseEventKind::Up(MouseButton::Left) if self.is_dragging() => {
                self.end_drag();
                EventResult::ConsumedAndRender
            }
            MouseEventKind::ScrollDown => {
                if self.scroll < self.items.len().saturating_sub(1) {
                    self.scroll += 1;
                }
                EventResult::ConsumedAndRender
            }
            MouseEventKind::ScrollUp => {
                self.scroll = self.scroll.saturating_sub(1);
                EventResult::ConsumedAndRender
            }
            _ => EventResult::Ignored,
        }
    }
}

impl Draggable for SortableList {
    fn can_drag(&self) -> bool {
        self.selected.is_some()
    }

    fn drag_data(&self) -> Option<DragData> {
        self.selected.map(|idx| {
            let label = self
                .items
                .get(idx)
                .map(|i| i.label.clone())
                .unwrap_or_default();
            DragData::list_item(idx, label)
        })
    }

    fn drag_preview(&self) -> Option<String> {
        self.selected
            .and_then(|idx| self.items.get(idx).map(|i| format!("↕ {}", i.label)))
    }

    fn on_drag_start(&mut self) {
        self.start_drag();
    }

    fn on_drag_end(&mut self, result: crate::event::drag::DropResult) {
        match result {
            crate::event::drag::DropResult::Accepted => self.end_drag(),
            _ => self.cancel_drag(),
        }
    }

    fn can_drop(&self) -> bool {
        true
    }

    fn accepted_types(&self) -> &[&'static str] {
        &["list_item"]
    }

    fn on_drop(&mut self, data: DragData) -> bool {
        if let Some(from_idx) = data.as_list_index() {
            if let Some(to_idx) = self.drop_target {
                // Reorder
                if from_idx < self.items.len() && from_idx != to_idx {
                    let item = self.items.remove(from_idx);
                    let insert_idx = if to_idx > from_idx {
                        to_idx - 1
                    } else {
                        to_idx
                    };
                    self.items.insert(insert_idx.min(self.items.len()), item);
                    self.selected = Some(insert_idx.min(self.items.len() - 1));

                    if let Some(ref mut callback) = self.on_reorder {
                        callback(from_idx, insert_idx);
                    }
                    return true;
                }
            }
        }
        false
    }
}

/// Create a sortable list
pub fn sortable_list<I, S>(items: I) -> SortableList
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    SortableList::new(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sortable_list_new() {
        let list = SortableList::new(["A", "B", "C"]);
        assert_eq!(list.items.len(), 3);
        assert_eq!(list.items[0].label, "A");
        assert_eq!(list.items[1].label, "B");
        assert_eq!(list.items[2].label, "C");
    }

    #[test]
    fn test_sortable_list_selection() {
        let mut list = SortableList::new(["A", "B", "C"]);
        assert!(list.selected().is_none());

        list.select_next();
        assert_eq!(list.selected(), Some(0));

        list.select_next();
        assert_eq!(list.selected(), Some(1));

        list.select_prev();
        assert_eq!(list.selected(), Some(0));
    }

    #[test]
    fn test_sortable_list_move() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(0));

        list.move_down();
        assert_eq!(list.items[0].label, "B");
        assert_eq!(list.items[1].label, "A");
        assert_eq!(list.selected(), Some(1));

        list.move_up();
        assert_eq!(list.items[0].label, "A");
        assert_eq!(list.items[1].label, "B");
        assert_eq!(list.selected(), Some(0));
    }

    #[test]
    fn test_sortable_list_drag() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(1));

        list.start_drag();
        assert!(list.is_dragging());
        assert!(list.items[1].dragging);

        list.cancel_drag();
        assert!(!list.is_dragging());
        assert!(!list.items[1].dragging);
    }

    #[test]
    fn test_sortable_list_order() {
        let list = SortableList::new(["A", "B", "C"]);
        assert_eq!(list.order(), vec![0, 1, 2]);
    }

    #[test]
    fn test_sortable_list_push_remove() {
        let mut list = SortableList::new(["A", "B"]);
        assert_eq!(list.items.len(), 2);

        list.push("C");
        assert_eq!(list.items.len(), 3);
        assert_eq!(list.items[2].label, "C");

        let removed = list.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().label, "B");
        assert_eq!(list.items.len(), 2);
    }

    #[test]
    fn test_sortable_list_draggable_trait() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(1));

        assert!(list.can_drag());
        let data = list.drag_data();
        assert!(data.is_some());
        assert_eq!(data.unwrap().as_list_index(), Some(1));
    }

    #[test]
    fn test_sortable_list_on_reorder() {
        use std::cell::Cell;
        use std::rc::Rc;

        let called = Rc::new(Cell::new(false));
        let called_clone = called.clone();

        let mut list = SortableList::new(["A", "B", "C"]).on_reorder(move |from, to| {
            called_clone.set(true);
            assert!(from != to);
        });

        list.set_selected(Some(0));
        list.move_down();

        assert!(called.get());
    }

    #[test]
    fn test_sortable_list_handles() {
        let list = SortableList::new(["A"]).handles(false);
        assert!(!list.show_handles);

        let list2 = SortableList::new(["A"]).handles(true);
        assert!(list2.show_handles);
    }

    #[test]
    fn test_sortable_list_colors() {
        let list = SortableList::new(["A"])
            .item_color(Color::RED)
            .selected_color(Color::BLUE);

        assert_eq!(list.item_color, Color::RED);
        assert_eq!(list.selected_color, Color::BLUE);
    }

    #[test]
    fn test_sortable_list_items() {
        let list = SortableList::new(["A", "B"]);
        let items = list.items();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].label, "A");
    }

    #[test]
    fn test_sortable_list_items_mut() {
        let mut list = SortableList::new(["A", "B"]);
        let items = list.items_mut();
        items[0].label = "X".to_string();
        assert_eq!(list.items()[0].label, "X");
    }

    #[test]
    fn test_sortable_list_end_drag() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(0));
        list.start_drag();
        list.drop_target = Some(2);

        list.end_drag();

        assert!(!list.is_dragging());
        // Item A moved from 0 to after B (index 1)
        assert_eq!(list.items[0].label, "B");
        assert_eq!(list.items[1].label, "A");
    }

    #[test]
    fn test_sortable_list_update_drop_target() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(0));
        list.start_drag();

        list.update_drop_target(5, 0);

        assert!(list.drop_target.is_some());
    }

    #[test]
    fn test_sortable_list_remove_updates_selection() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(2)); // Select last item

        list.remove(2); // Remove selected item

        // Selection should move to new last item
        assert_eq!(list.selected(), Some(1));
    }

    #[test]
    fn test_sortable_list_remove_all() {
        let mut list = SortableList::new(["A"]);
        list.set_selected(Some(0));

        list.remove(0);

        assert!(list.items.is_empty());
        assert_eq!(list.selected(), None);
    }

    #[test]
    fn test_sortable_list_select_empty() {
        let mut list = SortableList::new::<[&str; 0], &str>([]);

        list.select_next();
        assert!(list.selected().is_none());

        list.select_prev();
        assert!(list.selected().is_none());
    }

    #[test]
    fn test_sortable_list_render() {
        use crate::render::Buffer;

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let list = SortableList::new(["A", "B", "C"]);
        list.render(&mut ctx);
    }

    #[test]
    fn test_sortable_list_render_with_selection() {
        use crate::render::Buffer;

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(1));
        list.render(&mut ctx);
    }

    #[test]
    fn test_sortable_list_render_dragging() {
        use crate::render::Buffer;

        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(1));
        list.start_drag();
        list.drop_target = Some(0);
        list.render(&mut ctx);
    }

    #[test]
    fn test_sortable_list_handle_key() {
        let mut list = SortableList::new(["A", "B", "C"]);

        // j/k for navigation
        list.handle_key(&KeyEvent::new(crate::event::Key::Char('j')));
        assert_eq!(list.selected(), Some(0));

        list.handle_key(&KeyEvent::new(crate::event::Key::Down));
        assert_eq!(list.selected(), Some(1));

        list.handle_key(&KeyEvent::new(crate::event::Key::Char('k')));
        assert_eq!(list.selected(), Some(0));

        list.handle_key(&KeyEvent::new(crate::event::Key::Up));
        assert_eq!(list.selected(), Some(0)); // Already at top
    }

    #[test]
    fn test_sortable_list_handle_key_move() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(0));

        // Shift+J/K for moving items
        let mut shift_j = KeyEvent::new(crate::event::Key::Down);
        shift_j.shift = true;
        list.handle_key(&shift_j);
        assert_eq!(list.items[0].label, "B");
        assert_eq!(list.items[1].label, "A");

        let mut shift_k = KeyEvent::new(crate::event::Key::Up);
        shift_k.shift = true;
        list.handle_key(&shift_k);
        assert_eq!(list.items[0].label, "A");
        assert_eq!(list.items[1].label, "B");
    }

    #[test]
    fn test_sortable_list_handle_mouse() {
        let mut list = SortableList::new(["A", "B", "C"]);
        let area = Rect::new(0, 0, 40, 10);

        // Click to select
        let event = MouseEvent::new(5, 1, MouseEventKind::Down(MouseButton::Left));

        list.handle_mouse(&event, area);
        assert_eq!(list.selected(), Some(1));
    }

    #[test]
    fn test_sortable_list_can_drop() {
        let list = SortableList::new(["A", "B", "C"]);
        assert!(list.can_drop());
    }

    #[test]
    fn test_sortable_list_accepted_types() {
        let list = SortableList::new(["A", "B", "C"]);
        let types = list.accepted_types();
        assert!(types.contains(&"list_item"));
    }

    #[test]
    fn test_sortable_list_drag_enter_leave() {
        let mut list = SortableList::new(["A", "B", "C"]);

        let data = DragData::list_item(1, "B");
        list.on_drag_enter(&data);

        list.on_drag_leave();
    }

    #[test]
    fn test_sortable_list_helper() {
        let list = sortable_list(["A", "B"]);
        assert_eq!(list.items().len(), 2);
    }

    #[test]
    fn test_sortable_item_new() {
        let item = SortableItem::new("Test", 5);
        assert_eq!(item.label, "Test");
        assert_eq!(item.original_index, 5);
        assert!(!item.selected);
        assert!(!item.dragging);
    }

    #[test]
    fn test_sortable_list_order_after_reorder() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(0));
        list.move_down();

        // After moving A from 0 to 1, order is [B, A, C]
        // Original indices are [1, 0, 2]
        let order = list.order();
        assert_eq!(order, vec![1, 0, 2]);
    }

    #[test]
    fn test_sortable_list_cancel_drag_out_of_bounds() {
        let mut list = SortableList::new(["A"]);
        list.set_selected(Some(0));
        list.start_drag();

        // Remove the item while dragging
        list.items.clear();

        // Should not panic
        list.cancel_drag();
        assert!(!list.is_dragging());
    }

    #[test]
    fn test_sortable_list_end_drag_same_position() {
        let mut list = SortableList::new(["A", "B", "C"]);
        list.set_selected(Some(1));
        list.start_drag();
        list.drop_target = Some(1); // Same position

        list.end_drag();

        // Should not reorder
        assert_eq!(list.items[0].label, "A");
        assert_eq!(list.items[1].label, "B");
    }

    #[test]
    fn test_sortable_list_remove_out_of_bounds() {
        let mut list = SortableList::new(["A"]);
        let removed = list.remove(10);
        assert!(removed.is_none());
    }

    #[test]
    fn test_sortable_list_move_at_boundary() {
        let mut list = SortableList::new(["A", "B"]);

        // Try to move up from first item
        list.set_selected(Some(0));
        list.move_up();
        assert_eq!(list.items[0].label, "A"); // No change

        // Try to move down from last item
        list.set_selected(Some(1));
        list.move_down();
        assert_eq!(list.items[1].label, "B"); // No change
    }
}
