//! Menu bar and context menu widgets
//!
//! Provides horizontal menu bars and dropdown/context menus.

#![allow(dead_code)]

mod context_menu;
mod helpers;
mod menu_bar;
mod types;

pub use context_menu::ContextMenu;
pub use helpers::{context_menu, menu, menu_bar, menu_item};
pub use menu_bar::MenuBar;
pub use types::{Menu, MenuItem};

#[cfg(test)]
mod tests;
