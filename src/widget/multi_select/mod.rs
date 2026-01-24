//! Multi-select widget for choosing multiple options from a list
//!
//! Provides a dropdown with:
//! - Multiple selection with tag display
//! - Fuzzy search filtering
//! - Tag navigation and removal
//! - Optional maximum selection limit

mod filter;
mod helpers;
mod key_handling;
mod navigation;
mod render;
mod selection;
#[cfg(test)]
mod tests {
    //! Unit tests for the multi-select widget

    #![allow(unused_imports)]

    use crate::event::Key;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::multi_select::helpers::{multi_select, multi_select_from};
    use crate::widget::multi_select::types::{MultiSelect, MultiSelectOption};
    use crate::widget::traits::{RenderContext, View};

    #[test]
    fn test_multi_select_new() {
        let select = MultiSelect::new();
        assert!(select.is_empty());
        assert!(!select.is_open());
        assert_eq!(select.selection_count(), 0);
    }

    #[test]
    fn test_multi_select_options() {
        let select = multi_select()
            .option("Apple")
            .option("Banana")
            .option("Cherry");

        assert_eq!(select.len(), 3);
        assert!(!select.is_selected(0));
    }

    #[test]
    fn test_multi_select_selection() {
        let mut select = multi_select().options(vec!["A", "B", "C"]);

        select.select_option(0);
        assert!(select.is_selected(0));
        assert_eq!(select.selection_count(), 1);

        select.select_option(2);
        assert!(select.is_selected(2));
        assert_eq!(select.selection_count(), 2);

        select.deselect_option(0);
        assert!(!select.is_selected(0));
        assert_eq!(select.selection_count(), 1);
    }

    #[test]
    fn test_multi_select_toggle() {
        let mut select = multi_select().options(vec!["A", "B"]);

        select.toggle_option(0);
        assert!(select.is_selected(0));

        select.toggle_option(0);
        assert!(!select.is_selected(0));
    }

    #[test]
    fn test_multi_select_max_selections() {
        let mut select = multi_select()
            .options(vec!["A", "B", "C"])
            .max_selections(2);

        select.select_option(0);
        select.select_option(1);
        assert!(select.can_select_more() == false);

        select.select_option(2); // Should not add
        assert!(!select.is_selected(2));
        assert_eq!(select.selection_count(), 2);
    }

    #[test]
    fn test_multi_select_get_values() {
        let mut select = multi_select().options(vec!["Apple", "Banana", "Cherry"]);

        select.select_option(0);
        select.select_option(2);

        let values = select.get_selected_values();
        assert_eq!(values, vec!["Apple", "Cherry"]);
    }

    #[test]
    fn test_multi_select_navigation() {
        let mut select = multi_select().options(vec!["A", "B", "C"]);
        select.open();

        assert_eq!(select.dropdown_cursor, 0);

        select.cursor_down();
        assert_eq!(select.dropdown_cursor, 1);

        select.cursor_down();
        assert_eq!(select.dropdown_cursor, 2);

        select.cursor_down(); // Wraps
        assert_eq!(select.dropdown_cursor, 0);

        select.cursor_up(); // Wraps backward
        assert_eq!(select.dropdown_cursor, 2);
    }

    #[test]
    fn test_multi_select_tag_navigation() {
        let mut select = multi_select().options(vec!["A", "B", "C"]);
        select.select_option(0);
        select.select_option(1);
        select.select_option(2);

        assert!(select.tag_cursor.is_none());

        select.tag_cursor_left();
        assert_eq!(select.tag_cursor, Some(2));

        select.tag_cursor_left();
        assert_eq!(select.tag_cursor, Some(1));

        select.tag_cursor_right();
        assert_eq!(select.tag_cursor, Some(2));

        select.tag_cursor_right();
        assert!(select.tag_cursor.is_none());
    }

    #[test]
    fn test_multi_select_remove_tag() {
        let mut select = multi_select().options(vec!["A", "B", "C"]);
        select.select_option(0);
        select.select_option(1);
        select.select_option(2);

        select.tag_cursor = Some(1);
        select.remove_tag_at_cursor();

        assert_eq!(select.selection_count(), 2);
        assert!(select.is_selected(0));
        assert!(!select.is_selected(1));
        assert!(select.is_selected(2));
    }

    #[test]
    fn test_multi_select_search() {
        let mut select = multi_select()
            .options(vec!["Apple", "Apricot", "Banana", "Blueberry"])
            .searchable(true);

        select.open();
        select.set_query("ap");

        assert_eq!(select.filtered.len(), 2);
        assert!(select.filtered.contains(&0)); // Apple
        assert!(select.filtered.contains(&1)); // Apricot
    }

    #[test]
    fn test_multi_select_key_handling() {
        let mut select = multi_select().options(vec!["A", "B", "C"]);

        // Open
        select.handle_key(&Key::Enter);
        assert!(select.is_open());

        // Navigate
        select.handle_key(&Key::Down);
        assert_eq!(select.dropdown_cursor, 1);

        // Select
        select.handle_key(&Key::Enter);
        assert!(select.is_selected(1));

        // Close
        select.handle_key(&Key::Escape);
        assert!(!select.is_open());
    }

    #[test]
    fn test_multi_select_disabled_option() {
        let mut select = multi_select()
            .option_detailed(MultiSelectOption::new("Disabled", "disabled").disabled(true));

        select.select_option(0);
        assert!(!select.is_selected(0)); // Can't select disabled
    }

    #[test]
    fn test_multi_select_select_all() {
        let mut select = multi_select().options(vec!["A", "B", "C"]);

        select.select_all();
        assert_eq!(select.selection_count(), 3);

        select.clear_selection();
        assert_eq!(select.selection_count(), 0);
    }

    #[test]
    fn test_multi_select_pre_selected() {
        let select = multi_select()
            .options(vec!["A", "B", "C"])
            .selected_indices(vec![0, 2]);

        assert!(select.is_selected(0));
        assert!(!select.is_selected(1));
        assert!(select.is_selected(2));
    }

    #[test]
    fn test_multi_select_from() {
        let select = multi_select_from(vec!["X", "Y", "Z"]);
        assert_eq!(select.len(), 3);
    }

    #[test]
    fn test_multi_select_render() {
        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut select = multi_select()
            .options(vec!["Apple", "Banana"])
            .focused(true);
        select.select_option(0);

        select.render(&mut ctx);

        // Should show tag [Apple]
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '[');
    }

    #[test]
    fn test_multi_select_render_dropdown() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut select = multi_select()
            .options(vec!["Apple", "Banana"])
            .focused(true);
        select.open();

        select.render(&mut ctx);

        // Should show checkbox on second row
        assert_eq!(buffer.get(0, 1).unwrap().symbol, '[');
    }
}
mod types;

// Re-export public types
pub use types::{MultiSelect, MultiSelectOption};

// Re-export constructor functions
pub use helpers::{multi_select, multi_select_from};

// Macro implementations
crate::impl_styled_view!(MultiSelect);
crate::impl_widget_builders!(MultiSelect);
