//! List widget

use crate::render::Cell;
use crate::style::Color;
use crate::utils::Selection;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use std::fmt::Display;

/// A list widget for displaying items
pub struct List<T> {
    items: Vec<T>,
    selection: Selection,
    highlight_fg: Option<Color>,
    highlight_bg: Option<Color>,
    props: WidgetProps,
}

impl<T> List<T> {
    /// Create a new list with items
    pub fn new(items: Vec<T>) -> Self {
        let len = items.len();
        Self {
            items,
            selection: Selection::new(len),
            highlight_fg: None,
            highlight_bg: Some(Color::BLUE),
            props: WidgetProps::new(),
        }
    }

    /// Set selected index
    pub fn selected(mut self, idx: usize) -> Self {
        self.selection.set(idx);
        self
    }

    /// Set highlight foreground color
    pub fn highlight_fg(mut self, color: Color) -> Self {
        self.highlight_fg = Some(color);
        self
    }

    /// Set highlight background color
    pub fn highlight_bg(mut self, color: Color) -> Self {
        self.highlight_bg = Some(color);
        self
    }

    /// Get items
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.selection.index
    }

    /// Get number of items
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Select next item (wraps around)
    pub fn select_next(&mut self) {
        self.selection.next();
    }

    /// Select previous item (wraps around)
    pub fn select_prev(&mut self) {
        self.selection.prev();
    }
}

impl<T: Display> View for List<T> {
    crate::impl_view_meta!("List");
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        // Render each visible item
        for (i, item) in self.items.iter().enumerate() {
            if i as u16 >= area.height {
                break;
            }

            let y = area.y + i as u16;
            let is_selected = self.selection.is_selected(i);

            let text = item.to_string();
            let mut x = area.x;

            for ch in text.chars() {
                if x >= area.x + area.width {
                    break;
                }

                let mut cell = Cell::new(ch);
                if is_selected {
                    cell.fg = self.highlight_fg;
                    cell.bg = self.highlight_bg;
                }

                ctx.buffer.set(x, y, cell);

                let char_width = crate::utils::unicode::char_width(ch).max(1) as u16;
                if char_width == 2 && x + 1 < area.x + area.width {
                    let mut cont = Cell::continuation();
                    if is_selected {
                        cont.bg = self.highlight_bg;
                    }
                    ctx.buffer.set(x + 1, y, cont);
                }
                x += char_width;
            }

            // Fill rest of line for selected item
            if is_selected {
                while x < area.x + area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = self.highlight_bg;
                    ctx.buffer.set(x, y, cell);
                    x += 1;
                }
            }
        }
    }
}

// Note: Cannot use impl_styled_view! macro with generic types
// Implement StyledView manually for List<T>
impl<T: Display> crate::widget::StyledView for List<T> {
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

/// Helper function to create a list widget
pub fn list<T>(items: Vec<T>) -> List<T> {
    List::new(items)
}

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_builder() {
        let list = List::new(vec!["A", "B", "C"])
            .selected(1)
            .highlight_fg(Color::WHITE)
            .highlight_bg(Color::RED);

        assert_eq!(list.selected_index(), 1);
        assert_eq!(list.highlight_fg, Some(Color::WHITE));
        assert_eq!(list.highlight_bg, Some(Color::RED));
    }
}
