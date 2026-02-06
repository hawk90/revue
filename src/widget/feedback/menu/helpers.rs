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
}
