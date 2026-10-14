//! Tests for MenuItem and Menu

use super::types::{Menu, MenuItem};

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
