//! Builder methods for SortableList

use crate::style::Color;

use super::core::SortableList;

impl SortableList {
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
}
