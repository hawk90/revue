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

// Tests for menu helpers have been extracted to:
// tests/widget/feedback/menu_helpers.rs
//
// These tests only use public APIs and have been moved to maintain cleaner
// source code while ensuring test coverage remains intact.
