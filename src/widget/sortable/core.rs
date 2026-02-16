//! Core SortableList struct definition

use crate::event::drag::DragId;
use crate::style::Color;
use crate::widget::traits::{WidgetProps, WidgetState};

use super::types::{generate_id, ReorderCallback, SortableItem};

/// Sortable list widget
pub struct SortableList {
    /// List items
    pub items: Vec<SortableItem>,
    /// Selected item index
    pub selected: Option<usize>,
    /// Scroll offset
    pub scroll: usize,
    /// Item being dragged (index)
    pub dragging: Option<usize>,
    /// Drop target index (where to insert)
    pub drop_target: Option<usize>,
    /// Reorder callback
    pub on_reorder: Option<ReorderCallback>,
    /// Item height (usually 1)
    pub item_height: u16,
    /// Show drag handles
    pub show_handles: bool,
    /// Normal item color
    pub item_color: Color,
    /// Selected item color
    pub selected_color: Color,
    /// Drag indicator color
    pub drag_color: Color,
    /// Widget state
    pub state: WidgetState,
    /// Widget props
    pub props: WidgetProps,
    /// Unique ID for drag operations (for future drag tracking)
    pub _id: DragId,
}

impl SortableList {
    /// Create a new sortable list
    pub fn new<I, S>(items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let id = generate_id();

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
            state: WidgetState::new(),
            props: WidgetProps::new(),
            _id: id,
        }
    }
}
