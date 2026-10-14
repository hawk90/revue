//! Menu tests: the helpers (`menu`, `menu_bar`, `menu_item`,
//! `context_menu`), read from the values they build and from the screen

mod props;
mod view;

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::menu::{context_menu, menu, menu_bar, menu_item};
use revue::widget::theme::DARK_BG;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{ContextMenu, Menu, MenuBar, MenuItem};

/// Render `view` into a fresh `width` x `height` buffer.
pub(crate) fn render(view: &dyn View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    view.render(&mut ctx);
    buffer
}

/// The symbols of row `y`, without the continuation cells of wide glyphs.
pub(crate) fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap())
        .filter(|cell| !cell.is_continuation())
        .map(|cell| cell.symbol)
        .collect()
}

/// Every cell's symbol and colors, to compare two renders.
fn cells(buffer: &Buffer) -> Vec<String> {
    (0..buffer.height())
        .flat_map(|y| (0..buffer.width()).map(move |x| (x, y)))
        .map(|(x, y)| format!("{:?}", buffer.get(x, y).unwrap()))
        .collect()
}

/// A bar with one menu titled `title` holding `items`, opened.
fn open_bar(title: &str, items: Vec<MenuItem>) -> MenuBar {
    let mut bar = menu_bar().menu(menu(title).items(items));
    bar.open_menu(0);
    bar
}

#[test]
fn test_menu_function() {
    let m = menu("File");
    assert_eq!(m.title, "File");
    assert!(m.items.is_empty());
}

#[test]
fn test_menu_function_with_string() {
    let m = menu("Edit".to_string());
    assert_eq!(m.title, "Edit");
    assert!(m.items.is_empty());
}

#[test]
fn test_menu_item_function() {
    let item = menu_item("Open");
    assert_eq!(item.label, "Open");
    assert_eq!(item.shortcut, None);
    assert!(!item.disabled);
    assert_eq!(item.checked, None);
    assert!(item.submenu.is_empty());
    assert!(!item.separator);
}

#[test]
fn test_menu_item_function_with_string() {
    let item = menu_item("Save".to_string());
    assert_eq!(item.label, "Save");
    assert!(!item.separator);
}

#[test]
fn test_menu_bar_function() {
    let bar = menu_bar();
    assert!(!bar.is_open());
    // An empty bar: one blank row in the bar's background, nothing below
    let buffer = render(&bar, 6, 2);
    assert_eq!(row(&buffer, 0), "      ");
    assert!((0..6).all(|x| buffer.get(x, 0).unwrap().bg == Some(DARK_BG)));
    assert!((0..6).all(|x| buffer.get(x, 1).unwrap().bg.is_none()));
}

#[test]
fn test_context_menu_function() {
    let mut ctx_menu = context_menu();
    assert!(!ctx_menu.is_visible());
    assert!(!ctx_menu.handle_key(&Key::Down));
    // Shown with no items: visible, but nothing to draw
    ctx_menu.show(1, 1);
    assert!(ctx_menu.is_visible());
    let buffer = render(&ctx_menu, 8, 4);
    assert!((0..4).all(|y| row(&buffer, y).trim().is_empty()));
}

// =========================================================================
// Additional helper function tests
// =========================================================================

#[test]
fn test_menu_empty_title() {
    // An empty title still takes its two padding columns
    let bar = menu_bar().menu(menu("")).menu(menu("B"));
    let buffer = render(&bar, 6, 1);
    assert_eq!(row(&buffer, 0), "   B  ");
    assert_eq!(buffer.get(1, 0).unwrap().bg, Some(Color::rgb(60, 100, 180)));
    assert_eq!(buffer.get(2, 0).unwrap().bg, Some(DARK_BG));
}

#[test]
fn test_menu_with_spaces() {
    let bar = menu_bar().menu(menu("View History"));
    let buffer = render(&bar, 16, 1);
    assert_eq!(row(&buffer, 0), " View History   ");
}

#[test]
fn test_menu_with_unicode() {
    // The emoji takes two columns; the dropdown of the next menu starts after
    // them
    let mut bar = menu_bar()
        .menu(menu("📁 File"))
        .menu(menu("E").item(menu_item("x")));
    bar.open_menu(1);
    let buffer = render(&bar, 16, 4);
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '📁');
    assert!(buffer.get(2, 0).unwrap().is_continuation());
    assert_eq!(row(&buffer, 0), " 📁 File  E     ");
    assert_eq!(buffer.get(9, 1).unwrap().symbol, '┌');
    assert_eq!(buffer.get(8, 1).unwrap().symbol, ' ');
}

#[test]
fn test_menu_item_empty_label() {
    // Only the padding is left: a 4-column box with a blank row
    let buffer = render(&open_bar("M", vec![menu_item("")]), 8, 4);
    assert_eq!(row(&buffer, 1), "┌──┐    ");
    assert_eq!(row(&buffer, 2), "│  │    ");
    assert_eq!(row(&buffer, 3), "└──┘    ");
}

#[test]
fn test_menu_item_with_special_chars() {
    let buffer = render(&open_bar("M", vec![menu_item("Ctrl+S")]), 12, 4);
    assert_eq!(row(&buffer, 2), "│ Ctrl+S │  ");
}

#[test]
fn test_menu_item_with_unicode() {
    // A wide label: the glyph and its continuation, and the box is as wide
    // as the label's columns
    let buffer = render(&open_bar("M", vec![menu_item("설정 Set")]), 14, 4);
    assert_eq!(buffer.get(2, 2).unwrap().symbol, '설');
    assert!(buffer.get(3, 2).unwrap().is_continuation());
    assert_eq!(buffer.get(4, 2).unwrap().symbol, '정');
    assert_eq!(row(&buffer, 1), "┌──────────┐  ");
    assert_eq!(row(&buffer, 2), "│ 설정 Set │  ");
}

#[test]
fn test_menu_multiple() {
    let bar = menu_bar()
        .menu(menu("File"))
        .menu(menu("Edit"))
        .menu(menu("View"));
    let buffer = render(&bar, 20, 1);
    assert_eq!(row(&buffer, 0), " File  Edit  View   ");
}

#[test]
fn test_menu_item_multiple() {
    let buffer = render(
        &open_bar(
            "M",
            vec![menu_item("New"), menu_item("Open"), menu_item("Save")],
        ),
        10,
        6,
    );
    assert_eq!(row(&buffer, 2), "│ New  │  ");
    assert_eq!(row(&buffer, 3), "│ Open │  ");
    assert_eq!(row(&buffer, 4), "│ Save │  ");
    assert_eq!(row(&buffer, 5), "└──────┘  ");
}

#[test]
fn test_menu_bar_multiple() {
    // Two bars keep their own state
    let mut bar1 = menu_bar().menu(menu("A").item(menu_item("a")));
    let bar2 = menu_bar().menu(menu("A").item(menu_item("a")));
    bar1.toggle();
    assert!(bar1.is_open());
    assert!(!bar2.is_open());
    assert_eq!(row(&render(&bar1, 6, 3), 1), "┌───┐ ");
    assert_eq!(row(&render(&bar2, 6, 3), 1), "      ");
}

#[test]
fn test_context_menu_multiple() {
    let mut ctx1 = context_menu().item(menu_item("a"));
    let ctx2 = context_menu().item(menu_item("a"));
    ctx1.show(0, 0);
    assert!(ctx1.is_visible());
    assert!(!ctx2.is_visible());
    assert_eq!(row(&render(&ctx1, 6, 3), 0), "┌───┐ ");
    assert_eq!(row(&render(&ctx2, 6, 3), 0), "      ");
}

#[test]
fn test_helpers_match_their_constructors() {
    // Each helper builds what its type's constructor builds
    assert_eq!(menu("Test").title, Menu::new("Test").title);
    assert_eq!(menu_item("Item").label, MenuItem::new("Item").label);
    let mut a = menu_bar().menu(menu("T").item(menu_item("i")));
    let mut b = MenuBar::new().menu(Menu::new("T").item(MenuItem::new("i")));
    a.open_menu(0);
    b.open_menu(0);
    assert_eq!(cells(&render(&a, 8, 4)), cells(&render(&b, 8, 4)));
    let mut c = context_menu().item(menu_item("i"));
    let mut d = ContextMenu::new().item(MenuItem::new("i"));
    c.show(1, 1);
    d.show(1, 1);
    assert_eq!(cells(&render(&c, 8, 4)), cells(&render(&d, 8, 4)));
}

#[test]
fn test_menu_long_title() {
    // Clipped at the area's edge
    let long_title = "A".repeat(1000);
    let bar = menu_bar().menu(menu(long_title));
    let buffer = render(&bar, 20, 1);
    assert_eq!(row(&buffer, 0), format!(" {}", "A".repeat(19)));
}

#[test]
fn test_menu_item_long_label() {
    // The dropdown takes the whole width; the label stops before the
    // padding column and the border
    let long_label = "B".repeat(1000);
    let buffer = render(&open_bar("M", vec![menu_item(long_label)]), 20, 4);
    assert_eq!(row(&buffer, 1), format!("┌{}┐", "─".repeat(18)));
    assert_eq!(row(&buffer, 2), format!("│ {} │", "B".repeat(16)));
}

mod snapshots {

    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_menubar_basic() {
        use revue::widget::{Menu, MenuBar, MenuItem};

        let view = MenuBar::new()
            .menu(
                Menu::new("File")
                    .item(MenuItem::new("New"))
                    .item(MenuItem::new("Open"))
                    .item(MenuItem::separator())
                    .item(MenuItem::new("Save"))
                    .item(MenuItem::new("Exit")),
            )
            .menu(
                Menu::new("Edit")
                    .item(MenuItem::new("Cut"))
                    .item(MenuItem::new("Copy")),
            )
            .menu(Menu::new("Help").item(MenuItem::new("About")));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("menubar_basic");
    }

    #[test]
    fn test_menubar_with_shortcuts() {
        use revue::widget::{Menu, MenuBar, MenuItem};

        let view = MenuBar::new()
            .menu(
                Menu::new("File")
                    .item(MenuItem::new("New").shortcut("Ctrl+N"))
                    .item(MenuItem::new("Open").shortcut("Ctrl+O"))
                    .item(MenuItem::new("Save").shortcut("Ctrl+S")),
            )
            .menu(
                Menu::new("Edit")
                    .item(MenuItem::new("Cut").shortcut("Ctrl+X"))
                    .item(MenuItem::new("Copy").shortcut("Ctrl+C"))
                    .item(MenuItem::new("Paste").shortcut("Ctrl+V")),
            );

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("menubar_shortcuts");
    }
}
