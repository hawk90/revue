//! Tests for Menu widgets

use super::context_menu::ContextMenu;
use super::helpers::{context_menu, menu, menu_bar, menu_item};
use super::menu_bar::MenuBar;
use super::types::{Menu, MenuItem};
use crate::event::Key;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

#[test]
fn test_menu_item() {
    let item = MenuItem::new("Open").shortcut("Ctrl+O").disabled(false);

    assert_eq!(item.label, "Open");
    assert_eq!(item.shortcut, Some("Ctrl+O".to_string()));
    assert!(!item.disabled);
}

#[test]
fn test_menu() {
    let m = Menu::new("File")
        .item(MenuItem::new("New"))
        .item(MenuItem::new("Open"))
        .separator()
        .item(MenuItem::new("Exit"));

    assert_eq!(m.title, "File");
    assert_eq!(m.items.len(), 4);
    assert!(m.items[2].separator);
}

#[test]
fn test_menu_bar() {
    let mut bar = MenuBar::new()
        .menu(Menu::new("File").item(MenuItem::new("New")))
        .menu(Menu::new("Edit").item(MenuItem::new("Copy")));

    assert_eq!(bar.menus.len(), 2);
    assert!(!bar.is_open());

    bar.open_menu(0);
    assert!(bar.is_open());

    bar.next_menu();
    assert_eq!(bar.selected_menu, 1);

    bar.close();
    assert!(!bar.is_open());
}

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
fn test_menu_bar_render() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = MenuBar::new()
        .menu(Menu::new("File"))
        .menu(Menu::new("Edit"));

    bar.render(&mut ctx);
    // Smoke test
}

#[test]
fn test_menu_item_checked() {
    let item = MenuItem::new("Toggle").checked(true);
    assert_eq!(item.checked, Some(true));

    let item2 = MenuItem::new("Toggle2").checked(false);
    assert_eq!(item2.checked, Some(false));
}

#[test]
fn test_menu_item_submenu() {
    let item =
        MenuItem::new("Parent").submenu(vec![MenuItem::new("Child 1"), MenuItem::new("Child 2")]);
    assert!(item.has_submenu());
    assert_eq!(item.submenu.len(), 2);
}

#[test]
fn test_menu_item_has_submenu() {
    let item1 = MenuItem::new("No Submenu");
    assert!(!item1.has_submenu());

    let item2 = MenuItem::new("With Submenu").submenu(vec![MenuItem::new("Child")]);
    assert!(item2.has_submenu());
}

#[test]
fn test_menu_item_on_select_and_execute() {
    use std::cell::Cell;
    use std::rc::Rc;

    let called = Rc::new(Cell::new(false));
    let called_clone = called.clone();

    let item = MenuItem::new("Action").on_select(move || {
        called_clone.set(true);
    });

    item.execute();
    assert!(called.get());
}

#[test]
fn test_menu_item_execute_disabled() {
    use std::cell::Cell;
    use std::rc::Rc;

    let called = Rc::new(Cell::new(false));
    let called_clone = called.clone();

    let item = MenuItem::new("Disabled Action")
        .disabled(true)
        .on_select(move || {
            called_clone.set(true);
        });

    item.execute();
    assert!(!called.get()); // Should not be called
}

#[test]
fn test_menu_items() {
    let m = Menu::new("Edit").items(vec![
        MenuItem::new("Cut"),
        MenuItem::new("Copy"),
        MenuItem::new("Paste"),
    ]);
    assert_eq!(m.items.len(), 3);
}

#[test]
fn test_menu_bar_colors() {
    let bar = MenuBar::new().bg(Color::BLACK).fg(Color::WHITE);
    assert_eq!(bar.bg, Color::BLACK);
    assert_eq!(bar.fg, Color::WHITE);
}

#[test]
fn test_menu_bar_toggle() {
    let mut bar = MenuBar::new().menu(Menu::new("File").item(MenuItem::new("New")));

    assert!(!bar.is_open());

    bar.toggle();
    assert!(bar.is_open());

    bar.toggle();
    assert!(!bar.is_open());
}

#[test]
fn test_menu_bar_prev_menu() {
    let mut bar = MenuBar::new()
        .menu(Menu::new("File"))
        .menu(Menu::new("Edit"))
        .menu(Menu::new("View"));

    bar.selected_menu = 2;
    bar.prev_menu();
    assert_eq!(bar.selected_menu, 1);

    bar.prev_menu();
    assert_eq!(bar.selected_menu, 0);

    // Wrap around
    bar.prev_menu();
    assert_eq!(bar.selected_menu, 2);
}

#[test]
fn test_menu_bar_next_prev_item() {
    let mut bar = MenuBar::new().menu(
        Menu::new("File")
            .item(MenuItem::new("New"))
            .item(MenuItem::new("Open"))
            .item(MenuItem::new("Save")),
    );

    bar.open_menu(0);
    assert_eq!(bar.selected_item, Some(0));

    bar.next_item();
    assert_eq!(bar.selected_item, Some(1));

    bar.next_item();
    assert_eq!(bar.selected_item, Some(2));

    // Wrap around
    bar.next_item();
    assert_eq!(bar.selected_item, Some(0));

    bar.prev_item();
    assert_eq!(bar.selected_item, Some(2));
}

#[test]
fn test_menu_bar_skip_separators() {
    let mut bar = MenuBar::new().menu(
        Menu::new("File")
            .item(MenuItem::new("New"))
            .separator()
            .item(MenuItem::new("Exit")),
    );

    bar.open_menu(0);
    bar.next_item();
    // Should skip separator and go to Exit
    assert_eq!(bar.selected_item, Some(2));
}

#[test]
fn test_menu_bar_execute_selected() {
    use std::cell::Cell;
    use std::rc::Rc;

    let called = Rc::new(Cell::new(false));
    let called_clone = called.clone();

    let mut bar = MenuBar::new().menu(Menu::new("File").item(MenuItem::new("Action").on_select(
        move || {
            called_clone.set(true);
        },
    )));

    bar.open_menu(0);
    let result = bar.execute_selected();
    assert!(result);
    assert!(called.get());
    assert!(!bar.is_open()); // Menu should close after execution
}

#[test]
fn test_menu_bar_handle_key() {
    let mut bar = MenuBar::new()
        .menu(Menu::new("File").item(MenuItem::new("New")))
        .menu(Menu::new("Edit").item(MenuItem::new("Copy")));

    // Left/Right navigation
    assert!(bar.handle_key(&Key::Right));
    assert_eq!(bar.selected_menu, 1);

    assert!(bar.handle_key(&Key::Left));
    assert_eq!(bar.selected_menu, 0);

    // Vim keys
    assert!(bar.handle_key(&Key::Char('l')));
    assert_eq!(bar.selected_menu, 1);

    assert!(bar.handle_key(&Key::Char('h')));
    assert_eq!(bar.selected_menu, 0);

    // Enter to open
    assert!(bar.handle_key(&Key::Enter));
    assert!(bar.is_open());

    // Up/Down when open
    assert!(bar.handle_key(&Key::Down));
    assert!(bar.handle_key(&Key::Char('j')));
    assert!(bar.handle_key(&Key::Up));
    assert!(bar.handle_key(&Key::Char('k')));

    // Escape to close
    assert!(bar.handle_key(&Key::Escape));
    assert!(!bar.is_open());

    // Space to toggle
    assert!(bar.handle_key(&Key::Char(' ')));
    assert!(bar.is_open());

    // Unknown key
    bar.close();
    assert!(!bar.handle_key(&Key::Char('x')));
}

#[test]
fn test_menu_bar_default() {
    let bar = MenuBar::default();
    assert!(bar.menus.is_empty());
    assert!(!bar.is_open());
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
fn test_menu_bar_render_open() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut bar = MenuBar::new().menu(
        Menu::new("File")
            .item(MenuItem::new("New").shortcut("Ctrl+N"))
            .item(MenuItem::new("Open").shortcut("Ctrl+O"))
            .separator()
            .item(MenuItem::new("Exit")),
    );

    bar.open_menu(0);
    bar.render(&mut ctx);
}

#[test]
fn test_menu_bar_empty() {
    let mut bar = MenuBar::new();

    // Should not panic on empty menu bar
    bar.next_menu();
    bar.prev_menu();
    bar.next_item();
    bar.prev_item();
    assert!(!bar.execute_selected());
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
    assert_eq!(menu.fg, Color::WHITE);
}

#[test]
fn test_menu_bar_open_menu_with_items() {
    let mut bar = MenuBar::new()
        .menu(Menu::new("File").item(MenuItem::new("New")))
        .menu(Menu::new("Empty"));

    bar.open_menu(0);
    assert_eq!(bar.selected_item, Some(0));

    bar.open_menu(1);
    assert_eq!(bar.selected_item, None); // Empty menu
}

#[test]
fn test_menu_bar_open_out_of_bounds() {
    let mut bar = MenuBar::new().menu(Menu::new("File"));

    bar.open_menu(10); // Out of bounds
    assert!(!bar.is_open());
}
