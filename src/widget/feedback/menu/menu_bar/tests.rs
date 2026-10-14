//! MenuBar tests that read its private state

use super::super::types::{Menu, MenuItem};
use super::MenuBar;
use crate::event::Key;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::widget::theme::{DARK_BG, LIGHT_GRAY};
use crate::widget::traits::{RenderContext, View, DISABLED_FG};

/// The symbols of row `y`
fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

const SELECTED_BG: Color = Color::rgb(60, 100, 180);

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
fn test_menu_bar_render() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = MenuBar::new()
        .menu(Menu::new("File"))
        .menu(Menu::new("Edit"));

    bar.render(&mut ctx);
    // Titles padded by one space each, the first one selected
    assert_eq!(row(&buffer, 0).trim_end(), " File  Edit");
    for x in 0..6 {
        let cell = buffer.get(x, 0).unwrap();
        assert_eq!(
            (cell.fg, cell.bg),
            (Some(Color::WHITE), Some(SELECTED_BG)),
            "{x}"
        );
    }
    for x in 6..12 {
        let cell = buffer.get(x, 0).unwrap();
        assert_eq!(
            (cell.fg, cell.bg),
            (Some(Color::WHITE), Some(DARK_BG)),
            "{x}"
        );
    }
    // The bar's background runs to the right edge; nothing below it
    assert_eq!(buffer.get(79, 0).unwrap().bg, Some(DARK_BG));
    assert!(row(&buffer, 1).trim().is_empty());
    assert_eq!(buffer.get(0, 1).unwrap().bg, None);
}

#[test]
fn test_menu_bar_colors() {
    let bar = MenuBar::new().bg(Color::BLACK).fg(Color::WHITE);
    assert_eq!(bar.bg, Color::BLACK);
    assert_eq!(bar.fg, Some(Color::WHITE));
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
    // 16 columns: "Open" (4) + "Ctrl+O" and its gap (8) + borders and padding (4)
    let rows: Vec<String> = (1..8)
        .map(|y| (0..17).map(|x| buffer.get(x, y).unwrap().symbol).collect())
        .collect();
    assert_eq!(
        rows,
        [
            "┌──────────────┐ ",
            "│ New   Ctrl+N │ ",
            "│ Open  Ctrl+O │ ",
            "├──────────────┤ ",
            "│ Exit         │ ",
            "└──────────────┘ ",
            "                 ",
        ]
    );
    // The first item is selected; the shortcut keeps its own color
    assert_eq!(buffer.get(2, 2).unwrap().bg, Some(SELECTED_BG));
    assert_eq!(buffer.get(14, 2).unwrap().bg, Some(SELECTED_BG));
    assert_eq!(buffer.get(13, 2).unwrap().fg, Some(LIGHT_GRAY));
    assert_eq!(buffer.get(2, 3).unwrap().bg, Some(DARK_BG));
    // The separator line is dim, its joints are in the frame's color
    assert_eq!(buffer.get(5, 4).unwrap().fg, Some(DISABLED_FG));
    assert_eq!(buffer.get(0, 4).unwrap().fg, Some(Color::WHITE));
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

// Found by tests/widget/catalog/sequences/mod.rs: the shrunk sequence was
// `[5x2] <open menu 1> Left Left` - a menu whose title starts past the
// right edge of a narrow area.
#[test]
fn test_menu_bar_dropdown_past_the_right_edge() {
    let mut bar = MenuBar::new()
        .menu(Menu::new("File").item(MenuItem::new("Open")))
        .menu(Menu::new("Edit").item(MenuItem::new("Undo")));
    bar.open_menu(1);
    let mut buffer = Buffer::new(5, 2);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 5, 2));
    bar.render(&mut ctx);
    // "Edit" starts at column 6: its dropdown has no room and is not drawn
    assert_eq!(row(&buffer, 0), " File");
    assert_eq!(row(&buffer, 1), "     ");
}
