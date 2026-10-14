//! ContextMenu tests that read its private state

use super::super::types::MenuItem;
use super::ContextMenu;
use crate::event::Key;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

#[test]
fn test_context_menu() {
    let mut menu = ContextMenu::new()
        .item(MenuItem::new("Cut"))
        .item(MenuItem::new("Copy"))
        .item(MenuItem::new("Paste"));

    assert!(!menu.is_visible());

    menu.show(10, 10);
    assert!(menu.is_visible());

    menu.handle_key(&Key::Down);
    assert_eq!(menu.selected, 1);

    menu.hide();
    assert!(!menu.is_visible());
}

#[test]
fn test_context_menu_navigation() {
    let mut menu = ContextMenu::new()
        .item(MenuItem::new("Cut"))
        .item(MenuItem::new("Copy"))
        .item(MenuItem::new("Paste"));

    menu.show(10, 10);

    // Navigate down
    menu.handle_key(&Key::Down);
    assert_eq!(menu.selected, 1);

    menu.handle_key(&Key::Char('j'));
    assert_eq!(menu.selected, 2);

    // Navigate up
    menu.handle_key(&Key::Up);
    assert_eq!(menu.selected, 1);

    menu.handle_key(&Key::Char('k'));
    assert_eq!(menu.selected, 0);
}

#[test]
fn test_context_menu_boundary() {
    let mut menu = ContextMenu::new()
        .item(MenuItem::new("Item 1"))
        .item(MenuItem::new("Item 2"));

    menu.show(0, 0);

    // Move down to last item
    menu.handle_key(&Key::Down);
    assert_eq!(menu.selected, 1);

    // Should stay at boundary
    menu.handle_key(&Key::Down);
    assert_eq!(menu.selected, 1);

    // Move up to first
    menu.handle_key(&Key::Up);
    assert_eq!(menu.selected, 0);

    // Should stay at boundary
    menu.handle_key(&Key::Up);
    assert_eq!(menu.selected, 0);
}

#[test]
fn test_context_menu_execute() {
    use std::cell::Cell;
    use std::rc::Rc;

    let called = Rc::new(Cell::new(false));
    let called_clone = called.clone();

    let mut menu = ContextMenu::new().item(MenuItem::new("Action").on_select(move || {
        called_clone.set(true);
    }));

    menu.show(0, 0);
    let result = menu.handle_key(&Key::Enter);
    assert!(result);
    assert!(called.get());
    assert!(!menu.is_visible());
}

#[test]
fn test_context_menu_escape() {
    let mut menu = ContextMenu::new().item(MenuItem::new("Item"));

    menu.show(0, 0);
    assert!(menu.is_visible());

    menu.handle_key(&Key::Escape);
    assert!(!menu.is_visible());
}

#[test]
fn test_context_menu_render() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut menu = ContextMenu::new()
        .item(MenuItem::new("Cut").shortcut("Ctrl+X"))
        .item(MenuItem::new("Copy").shortcut("Ctrl+C"))
        .item(MenuItem::separator())
        .item(MenuItem::new("Paste").shortcut("Ctrl+V"));

    menu.show(5, 5);
    menu.render(&mut ctx);
}

#[test]
fn test_context_menu_separator() {
    let menu = ContextMenu::new()
        .item(MenuItem::new("Cut"))
        .item(MenuItem::separator())
        .item(MenuItem::new("Paste"));

    assert_eq!(menu.items.len(), 3);
    assert!(menu.items[1].separator);
}

#[test]
fn test_context_menu_default_colors() {
    let menu = ContextMenu::new();
    // Default colors are set
    assert_eq!(menu.bg, Color::rgb(40, 40, 40));
    // The foreground defaults to "the builder said nothing", so a
    // stylesheet can reach it; white is applied at paint time.
    assert_eq!(menu.fg, None);
}
