//! Convenience constructors for Menu widgets

use super::context_menu::ContextMenu;
use super::menu_bar::MenuBar;
use super::types::{Menu, MenuItem};

/// Create a new dropdown menu
pub fn menu(title: impl Into<String>) -> Menu {
    Menu::new(title)
}

/// Create a new menu item
pub fn menu_item(label: impl Into<String>) -> MenuItem {
    MenuItem::new(label)
}

/// Create a new menu bar
pub fn menu_bar() -> MenuBar {
    MenuBar::new()
}

/// Create a new context menu
pub fn context_menu() -> ContextMenu {
    ContextMenu::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_function() {
        let m = menu("File");
        let _ = m;
    }

    #[test]
    fn test_menu_function_with_string() {
        let m = menu("Edit".to_string());
        let _ = m;
    }

    #[test]
    fn test_menu_item_function() {
        let item = menu_item("Open");
        let _ = item;
    }

    #[test]
    fn test_menu_item_function_with_string() {
        let item = menu_item("Save".to_string());
        let _ = item;
    }

    #[test]
    fn test_menu_bar_function() {
        let bar = menu_bar();
        let _ = bar;
    }

    #[test]
    fn test_context_menu_function() {
        let ctx_menu = context_menu();
        let _ = ctx_menu;
    }

    // =========================================================================
    // Additional helper function tests
    // =========================================================================

    #[test]
    fn test_menu_empty_title() {
        let m = menu("");
        let _ = m;
    }

    #[test]
    fn test_menu_with_spaces() {
        let m = menu("View History");
        let _ = m;
    }

    #[test]
    fn test_menu_with_unicode() {
        let m = menu("📁 File");
        let _ = m;
    }

    #[test]
    fn test_menu_item_empty_label() {
        let item = menu_item("");
        let _ = item;
    }

    #[test]
    fn test_menu_item_with_special_chars() {
        let item = menu_item("Ctrl+S");
        let _ = item;
    }

    #[test]
    fn test_menu_item_with_unicode() {
        let item = menu_item("⚙️ Settings");
        let _ = item;
    }

    #[test]
    fn test_menu_multiple() {
        let m1 = menu("File");
        let m2 = menu("Edit");
        let m3 = menu("View");
        let _ = m1;
        let _ = m2;
        let _ = m3;
    }

    #[test]
    fn test_menu_item_multiple() {
        let i1 = menu_item("New");
        let i2 = menu_item("Open");
        let i3 = menu_item("Save");
        let _ = i1;
        let _ = i2;
        let _ = i3;
    }

    #[test]
    fn test_menu_bar_multiple() {
        let bar1 = menu_bar();
        let bar2 = menu_bar();
        let _ = bar1;
        let _ = bar2;
    }

    #[test]
    fn test_context_menu_multiple() {
        let ctx1 = context_menu();
        let ctx2 = context_menu();
        let _ = ctx1;
        let _ = ctx2;
    }

    #[test]
    fn test_helpers_do_not_panic() {
        // All helper functions should not panic
        let _ = menu("Test");
        let _ = menu_item("Item");
        let _ = menu_bar();
        let _ = context_menu();
    }

    #[test]
    fn test_menu_long_title() {
        let long_title = "A".repeat(1000);
        let m = menu(long_title);
        let _ = m;
    }

    #[test]
    fn test_menu_item_long_label() {
        let long_label = "B".repeat(1000);
        let item = menu_item(long_label);
        let _ = item;
    }
}
