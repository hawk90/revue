//! Breadcrumb navigation widget
//!
//! Shows hierarchical navigation path with clickable segments.

mod core;
mod helper;
mod types;

pub use types::{BreadcrumbItem, SeparatorStyle};

// Re-export core types
pub use core::Breadcrumb;

// Re-export helper functions
pub use helper::{breadcrumb, crumb};

#[cfg(test)]
mod tests {
    use super::*;

    use crate::layout::Rect;
    use crate::style::Color;
    use crate::widget::breadcrumb::{breadcrumb, Breadcrumb, BreadcrumbItem, SeparatorStyle};
    use crate::widget::crumb;
    use crate::widget::traits::{RenderContext, View};

    #[test]
    fn test_breadcrumb_item() {
        let item = BreadcrumbItem::new("Home").icon('🏠');
        assert_eq!(item.label, "Home");
        assert_eq!(item.icon, Some('🏠'));
    }

    #[test]
    fn test_breadcrumb_new() {
        let bc = Breadcrumb::new();
        assert!(bc.is_empty());
    }

    #[test]
    fn test_breadcrumb_push() {
        let bc = Breadcrumb::new()
            .push("Home")
            .push("Documents")
            .push("Work");

        assert_eq!(bc.len(), 3);
        assert_eq!(bc.selected(), 2); // Last item selected
    }

    #[test]
    fn test_breadcrumb_path() {
        let bc = Breadcrumb::new().path("/home/user/documents");
        assert_eq!(bc.len(), 3);
        assert_eq!(bc.path_string(), "home/user/documents");
    }

    #[test]
    fn test_breadcrumb_selection() {
        let mut bc = Breadcrumb::new().push("A").push("B").push("C");

        assert_eq!(bc.selected(), 2);

        bc.select_prev();
        assert_eq!(bc.selected(), 1);

        bc.select_prev();
        assert_eq!(bc.selected(), 0);

        bc.select_prev();
        assert_eq!(bc.selected(), 0); // Can't go below 0

        bc.select_next();
        assert_eq!(bc.selected(), 1);
    }

    #[test]
    fn test_breadcrumb_navigate() {
        let mut bc = Breadcrumb::new().push("A").push("B").push("C").push("D");

        bc.navigate_to(1);
        assert_eq!(bc.len(), 2);
        assert_eq!(bc.selected(), 1);
    }

    #[test]
    fn test_breadcrumb_pop() {
        let mut bc = Breadcrumb::new().push("A").push("B");

        let item = bc.pop();
        assert_eq!(item.unwrap().label, "B");
        assert_eq!(bc.len(), 1);
    }

    #[test]
    fn test_separator_style() {
        assert_eq!(SeparatorStyle::Slash.char(), '/');
        assert_eq!(SeparatorStyle::Arrow.char(), '>');
        assert_eq!(SeparatorStyle::Chevron.char(), '›');
        assert_eq!(SeparatorStyle::Custom('→').char(), '→');
    }

    #[test]
    fn test_handle_key() {
        use crate::event::Key;

        let mut bc = Breadcrumb::new().push("A").push("B");

        bc.set_selected(0);
        assert!(bc.handle_key(&Key::Right));
        assert_eq!(bc.selected(), 1);

        assert!(bc.handle_key(&Key::Left));
        assert_eq!(bc.selected(), 0);
    }

    #[test]
    fn test_render() {
        let mut buffer = Buffer::new(60, 3);
        let area = Rect::new(0, 0, 60, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bc = Breadcrumb::new()
            .push("Home")
            .push("Documents")
            .push("Work");

        bc.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_helpers() {
        let bc = breadcrumb().item(crumb("Test"));

        assert_eq!(bc.len(), 1);
    }

    // =========================================================================
    // Additional coverage tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_default() {
        let bc = Breadcrumb::default();
        assert!(bc.is_empty());
    }

    #[test]
    fn test_breadcrumb_item_clickable() {
        let item = BreadcrumbItem::new("Test").clickable(false);
        assert!(!item.clickable);
    }

    #[test]
    fn test_breadcrumb_item_clone() {
        let item = BreadcrumbItem::new("Test").icon('📁');
        let cloned = item.clone();
        assert_eq!(cloned.label, "Test");
        assert_eq!(cloned.icon, Some('📁'));
    }

    #[test]
    fn test_separator_style_all() {
        assert_eq!(SeparatorStyle::DoubleArrow.char(), '»');
        assert_eq!(SeparatorStyle::Dot.char(), '•');
        assert_eq!(SeparatorStyle::Pipe.char(), '|');
    }

    #[test]
    fn test_separator_style_debug() {
        let style = SeparatorStyle::Chevron;
        let debug = format!("{:?}", style);
        assert!(debug.contains("Chevron"));
    }

    #[test]
    fn test_separator_style_eq() {
        assert_eq!(SeparatorStyle::Slash, SeparatorStyle::Slash);
        assert_ne!(SeparatorStyle::Slash, SeparatorStyle::Arrow);
    }

    #[test]
    fn test_breadcrumb_colors() {
        let bc = Breadcrumb::new()
            .item_color(Color::WHITE)
            .selected_color(Color::CYAN)
            .separator_color(Color::rgb(80, 80, 80));

        assert_eq!(bc.item_color, Color::WHITE);
        assert_eq!(bc.selected_color, Color::CYAN);
    }

    #[test]
    fn test_breadcrumb_home_settings() {
        let bc = Breadcrumb::new().home(false).home_icon('🏠');

        assert!(!bc.show_home);
        assert_eq!(bc.home_icon, '🏠');
    }

    #[test]
    fn test_breadcrumb_max_width() {
        let bc = Breadcrumb::new().max_width(50);
        assert_eq!(bc.max_width, 50);
    }

    #[test]
    fn test_breadcrumb_collapse() {
        let bc = Breadcrumb::new().collapse(false);
        assert!(!bc.collapse);
    }

    #[test]
    fn test_breadcrumb_total_width() {
        let bc = Breadcrumb::new().home(false).push("Home").push("Documents");

        let width = bc.total_width();
        // "Home" (4) + " › " (3) + "Documents" (9) = 16
        assert!(width > 0);
    }

    #[test]
    fn test_breadcrumb_total_width_with_icons() {
        let bc = Breadcrumb::new()
            .home(false)
            .item(BreadcrumbItem::new("Home").icon('📁'))
            .item(BreadcrumbItem::new("Work"));

        let width = bc.total_width();
        assert!(width > 0);
    }

    #[test]
    fn test_breadcrumb_total_width_with_home() {
        let bc = Breadcrumb::new().home(true).push("Documents");

        let width = bc.total_width();
        // Home icon + space + separator + Documents
        assert!(width > 0);
    }

    #[test]
    fn test_breadcrumb_render_empty() {
        let mut buffer = Buffer::new(40, 3);
        let area = Rect::new(0, 0, 40, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bc = Breadcrumb::new().home(false);
        bc.render(&mut ctx);
        // Empty breadcrumb should not panic
    }

    #[test]
    fn test_breadcrumb_render_small_area() {
        let mut buffer = Buffer::new(2, 1);
        let area = Rect::new(0, 0, 2, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bc = Breadcrumb::new().push("Test");
        bc.render(&mut ctx);
        // Small area should not panic
    }

    #[test]
    fn test_breadcrumb_render_with_collapse() {
        let mut buffer = Buffer::new(30, 3);
        let area = Rect::new(0, 0, 30, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bc = Breadcrumb::new()
            .home(false)
            .max_width(20)
            .collapse(true)
            .push("Very")
            .push("Long")
            .push("Path")
            .push("That")
            .push("Needs")
            .push("Collapse");

        bc.render(&mut ctx);
    }

    #[test]
    fn test_breadcrumb_render_with_icons() {
        let mut buffer = Buffer::new(60, 3);
        let area = Rect::new(0, 0, 60, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bc = Breadcrumb::new()
            .home(true)
            .item(BreadcrumbItem::new("Documents").icon('📁'))
            .item(BreadcrumbItem::new("Work").icon('💼'));

        bc.render(&mut ctx);
    }

    #[test]
    fn test_breadcrumb_handle_key_vim() {
        use crate::event::Key;

        let mut bc = Breadcrumb::new().push("A").push("B");

        bc.set_selected(0);

        // 'l' for right
        assert!(bc.handle_key(&Key::Char('l')));
        assert_eq!(bc.selected(), 1);

        // 'h' for left
        assert!(bc.handle_key(&Key::Char('h')));
        assert_eq!(bc.selected(), 0);
    }

    #[test]
    fn test_breadcrumb_handle_key_unhandled() {
        use crate::event::Key;

        let mut bc = Breadcrumb::new().push("Test");

        let handled = bc.handle_key(&Key::Escape);
        assert!(!handled);
    }

    #[test]
    fn test_breadcrumb_selected_item() {
        let bc = Breadcrumb::new().push("First").push("Second");

        let item = bc.selected_item();
        assert!(item.is_some());
        assert_eq!(item.unwrap().label, "Second");
    }

    #[test]
    fn test_breadcrumb_selected_item_empty() {
        let bc = Breadcrumb::new();
        assert!(bc.selected_item().is_none());
    }

    #[test]
    fn test_breadcrumb_navigate_to_out_of_bounds() {
        let mut bc = Breadcrumb::new().push("A").push("B").push("C");

        bc.navigate_to(10); // Out of bounds, should do nothing
        assert_eq!(bc.len(), 3);
    }

    #[test]
    fn test_breadcrumb_pop_empty() {
        let mut bc = Breadcrumb::new();
        let item = bc.pop();
        assert!(item.is_none());
    }

    #[test]
    fn test_crumb_helper() {
        let item = crumb("Test").icon('✓').clickable(false);
        assert_eq!(item.label, "Test");
        assert_eq!(item.icon, Some('✓'));
        assert!(!item.clickable);
    }
}
