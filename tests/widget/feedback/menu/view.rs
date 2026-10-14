//! MenuBar and ContextMenu on the screen: placement, the stylesheet, item
//! states, clipping, and keys that only an open menu takes

use std::cell::RefCell;
use std::rc::Rc;

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::{Color, Style, VisualStyle};
use revue::widget::theme::{DARK_BG, LIGHT_GRAY};
use revue::widget::traits::{RenderContext, View, DISABLED_FG};
use revue::widget::{ContextMenu, Menu, MenuBar, MenuItem};

use super::{render, row};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const SELECTED_BG: Color = Color::rgb(60, 100, 180);

/// A menu built on demand, so the placement checks (which need `Clone`) can
/// take MenuBar and ContextMenu, whose items hold boxed callbacks
#[derive(Clone)]
struct Built(fn() -> Box<dyn View>);

impl View for Built {
    fn render(&self, ctx: &mut RenderContext) {
        (self.0)().render(ctx)
    }
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        (self.0)().measure(max_width, max_height)
    }
    fn fills(&self) -> revue::widget::Fill {
        (self.0)().fills()
    }
}

fn file_edit() -> MenuBar {
    MenuBar::new()
        .menu(
            Menu::new("File")
                .item(MenuItem::new("New").shortcut("Ctrl+N"))
                .separator()
                .item(MenuItem::new("Quit").disabled(true)),
        )
        .menu(
            Menu::new("편집")
                .item(MenuItem::new("Wrap").checked(true))
                .item(MenuItem::new("More").submenu(vec![MenuItem::new("x")])),
        )
}

#[test]
fn test_menu_zero_areas_and_offset() {
    let menus: [fn() -> Box<dyn View>; 4] = [
        || Box::new(file_edit()),
        || {
            let mut bar = file_edit();
            bar.open_menu(1);
            Box::new(bar)
        },
        || {
            let mut m = ContextMenu::new()
                .item(MenuItem::new("Cut"))
                .item(MenuItem::new("붙여넣기"));
            m.show(2, 1);
            Box::new(m)
        },
        // Shown past the area: moved inside it
        || {
            let mut m = ContextMenu::new().item(MenuItem::new("Cut"));
            m.show(40, 40);
            Box::new(m)
        },
    ];
    for make in menus {
        assert_zero_areas_draw_nothing(&Built(make));
        assert_same_when_offset(Built(make), 16, 8);
    }
}

fn styled(view: &dyn View, width: u16, height: u16) -> Buffer {
    let style = Style {
        visual: VisualStyle {
            color: Color::GREEN,
            ..VisualStyle::default()
        },
        ..Default::default()
    };
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::with_style(&mut buffer, Rect::new(0, 0, width, height), &style);
    view.render(&mut ctx);
    buffer
}

#[test]
fn test_menu_bar_dropdown_takes_the_stylesheet_color() {
    let mut bar = file_edit();
    bar.open_menu(0);
    bar.next_item(); // skips the separator to the disabled "Quit"
    let b = styled(&bar, 30, 6);
    let fg = |x: u16, y: u16| b.get(x, y).unwrap().fg;
    // Unselected title, frame, separator joints and the plain "New" row
    assert_eq!(fg(6, 0), Some(Color::GREEN));
    assert_eq!(fg(0, 1), Some(Color::GREEN));
    assert_eq!(fg(0, 3), Some(Color::GREEN));
    assert_eq!(fg(2, 2), Some(Color::GREEN));
    // The selected title keeps its highlight; disabled, shortcut and
    // separator line keep their own colors
    assert_eq!(fg(1, 0), Some(Color::WHITE));
    assert_eq!(fg(2, 4), Some(DISABLED_FG));
    assert_eq!(b.get(2, 4).unwrap().bg, Some(SELECTED_BG));
    assert_eq!(fg(7, 2), Some(LIGHT_GRAY));
    assert_eq!(fg(3, 3), Some(DISABLED_FG));

    // A builder color outranks the stylesheet
    let b = styled(&file_edit().fg(Color::YELLOW), 30, 1);
    assert_eq!(b.get(6, 0).unwrap().fg, Some(Color::YELLOW));
}

#[test]
fn test_context_menu_takes_the_stylesheet_color() {
    let mut m = ContextMenu::new()
        .item(MenuItem::new("Cut"))
        .item(MenuItem::new("Copy"));
    m.show(0, 0);
    let b = styled(&m, 10, 5);
    // Frame and the unselected row take the stylesheet; the selected keeps
    // its highlight
    assert_eq!(b.get(0, 0).unwrap().fg, Some(Color::GREEN));
    assert_eq!(b.get(2, 2).unwrap().fg, Some(Color::GREEN));
    assert_eq!(b.get(2, 1).unwrap().fg, Some(Color::WHITE));
    assert_eq!(b.get(2, 1).unwrap().bg, Some(SELECTED_BG));
    assert_eq!(b.get(2, 2).unwrap().bg, Some(DARK_BG));
}

#[test]
fn test_menu_bar_item_states_on_screen() {
    let mut bar = MenuBar::new().menu(
        Menu::new("View")
            .item(MenuItem::new("A long label here"))
            .item(MenuItem::new("Wrap").checked(true))
            .item(MenuItem::new("Ruler").checked(false))
            .item(MenuItem::new("Off").disabled(true))
            .item(MenuItem::new("More").submenu(vec![MenuItem::new("x")]))
            .item(MenuItem::new("Go").shortcut("F5")),
    );
    bar.open_menu(0);
    let b = render(&bar, 24, 9);
    // 21 columns: the long label plus padding and borders
    assert_eq!(row(&b, 1), format!("┌{}┐   ", "─".repeat(19)));
    assert_eq!(row(&b, 3), format!("│ ✓ Wrap{}│   ", " ".repeat(12)));
    assert_eq!(row(&b, 4), format!("│   Ruler{}│   ", " ".repeat(11)));
    assert_eq!(row(&b, 6), format!("│ More{}▶│   ", " ".repeat(13)));
    assert_eq!(row(&b, 7), format!("│ Go{}F5 │   ", " ".repeat(13)));
    assert_eq!(row(&b, 8), format!("└{}┘   ", "─".repeat(19)));
    // The disabled row is dim, the shortcut has its own color
    assert_eq!(b.get(2, 5).unwrap().fg, Some(DISABLED_FG));
    assert_eq!(b.get(17, 7).unwrap().fg, Some(LIGHT_GRAY));
    // The first row is selected across the inside of the box
    assert_eq!(b.get(1, 2).unwrap().bg, Some(SELECTED_BG));
    assert_eq!(b.get(19, 2).unwrap().bg, Some(SELECTED_BG));
    assert_eq!(b.get(20, 2).unwrap().bg, Some(DARK_BG));
}

#[test]
fn test_menu_bar_dropdown_sits_under_its_title() {
    // " 파일 " is 6 columns and " Edit " 6: the third menu starts at 12
    let mut bar = MenuBar::new()
        .menu(Menu::new("파일"))
        .menu(Menu::new("Edit"))
        .menu(Menu::new("Go").item(MenuItem::new("Up")));
    bar.open_menu(2);
    let b = render(&bar, 24, 4);
    assert_eq!(row(&b, 0).trim_end(), " 파일  Edit  Go");
    assert_eq!(row(&b, 1), format!("{}┌────┐      ", " ".repeat(12)));
    assert_eq!(row(&b, 2), format!("{}│ Up │      ", " ".repeat(12)));
    assert_eq!(b.get(13, 0).unwrap().bg, Some(SELECTED_BG));
    assert_eq!(b.get(11, 0).unwrap().bg, Some(DARK_BG));
}

#[test]
fn test_menu_bar_dropdown_clipped_to_the_area() {
    // Taller and wider than the area: the box ends at the bottom row and
    // the right edge, with as many items as fit
    let mut bar = MenuBar::new().menu(Menu::new("A")).menu(
        Menu::new("B").items(
            (0..20)
                .map(|i| MenuItem::new(format!("Item {i}")))
                .collect(),
        ),
    );
    bar.open_menu(1);
    let b = render(&bar, 9, 5);
    assert_eq!(row(&b, 1), "   ┌────┐");
    assert_eq!(row(&b, 2), "   │ It │");
    assert_eq!(row(&b, 3), "   │ It │");
    assert_eq!(row(&b, 4), "   └────┘");
}

#[test]
fn test_menu_bar_open_empty_menu_draws_no_dropdown() {
    let mut bar = MenuBar::new().menu(Menu::new("A"));
    bar.open_menu(0);
    assert!(bar.is_open());
    let b = render(&bar, 6, 3);
    assert_eq!(row(&b, 1), "      ");
    assert_eq!(row(&b, 2), "      ");
    // One row: no room for the dropdown
    let mut bar = MenuBar::new().menu(Menu::new("A").item(MenuItem::new("a")));
    bar.open_menu(0);
    assert_eq!(row(&render(&bar, 6, 1), 0), " A    ");
}

#[test]
fn test_context_menu_kept_inside_the_area() {
    let make = |x, y| {
        let mut m = ContextMenu::new()
            .item(MenuItem::new("Cut"))
            .item(MenuItem::new("Copy"));
        m.show(x, y);
        m
    };
    // 8x4 box: inside at (3, 2), moved to the corner from past the edge
    let b = render(&make(3, 2), 12, 7);
    assert_eq!(row(&b, 2), "   ┌──────┐ ");
    assert_eq!(row(&b, 5), "   └──────┘ ");
    let b = render(&make(200, 200), 12, 7);
    assert_eq!(row(&b, 3), "    ┌──────┐");
    assert_eq!(row(&b, 6), "    └──────┘");
    // Wider than the area: from the left edge, cut at the right
    let b = render(&make(5, 0), 6, 4);
    assert_eq!(row(&b, 0), "┌─────");
    assert_eq!(row(&b, 1), "│ Cut ");
}

#[test]
fn test_context_menu_hidden_after_choosing_draws_nothing() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    let mut m = ContextMenu::new()
        .item(MenuItem::new("Cut"))
        .item(MenuItem::new("Copy").on_select(move || l.borrow_mut().push("copy")));
    m.show(0, 0);
    assert!(m.handle_key(&Key::Char('j')));
    assert!(m.handle_key(&Key::Char(' ')));
    assert_eq!(*log.borrow(), ["copy"]);
    assert!(!m.is_visible());
    assert!((0..4).all(|y| row(&render(&m, 10, 4), y).trim().is_empty()));
    // Hidden: keys pass through
    assert!(!m.handle_key(&Key::Enter));
    assert!(!m.handle_key(&Key::Escape));
    // Shown again: the selection starts over
    m.show(0, 0);
    assert_eq!(render(&m, 10, 4).get(2, 1).unwrap().bg, Some(SELECTED_BG));
}

#[test]
fn test_menu_bar_item_keys_only_when_open() {
    let mut bar = file_edit();
    for key in [
        Key::Up,
        Key::Down,
        Key::Char('j'),
        Key::Char('k'),
        Key::Escape,
    ] {
        assert!(!bar.handle_key(&key), "{key:?}");
        assert!(!bar.is_open());
    }
    // Left and Right move between titles even when closed
    assert!(bar.handle_key(&Key::Right));
    let b = render(&bar, 16, 1);
    assert_eq!(b.get(7, 0).unwrap().bg, Some(SELECTED_BG));
    assert_eq!(b.get(1, 0).unwrap().bg, Some(DARK_BG));
    // Moving while open opens the next menu at its first item
    assert!(bar.handle_key(&Key::Enter));
    assert!(bar.handle_key(&Key::Down));
    assert!(bar.handle_key(&Key::Left));
    assert!(bar.is_open());
    let b = render(&bar, 16, 6);
    assert_eq!(b.get(2, 2).unwrap().bg, Some(SELECTED_BG));
}

#[test]
fn test_menu_bar_disabled_item_is_not_chosen() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    let mut bar = MenuBar::new().menu(
        Menu::new("A").item(
            MenuItem::new("Off")
                .disabled(true)
                .on_select(move || l.borrow_mut().push("off")),
        ),
    );
    bar.open_menu(0);
    assert!(!bar.handle_key(&Key::Enter));
    assert!(!bar.execute_selected());
    assert!(bar.is_open());
    assert!(log.borrow().is_empty());
}

#[test]
fn test_menu_bar_separators_skipped_both_ways() {
    let mut bar = MenuBar::new().menu(
        Menu::new("A")
            .item(MenuItem::new("a"))
            .separator()
            .separator()
            .item(MenuItem::new("b")),
    );
    bar.open_menu(0);
    let selected_row = |bar: &MenuBar| {
        let b = render(bar, 10, 7);
        (2..6).find(|&y| b.get(2, y).unwrap().bg == Some(SELECTED_BG))
    };
    bar.next_item();
    assert_eq!(selected_row(&bar), Some(5));
    bar.next_item();
    assert_eq!(selected_row(&bar), Some(2));
    bar.prev_item();
    assert_eq!(selected_row(&bar), Some(5));
    bar.prev_item();
    assert_eq!(selected_row(&bar), Some(2));
}

#[test]
fn test_menu_bar_execute_after_close_does_nothing() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let l = log.clone();
    let mut bar = MenuBar::new()
        .menu(Menu::new("A").item(MenuItem::new("a").on_select(move || l.borrow_mut().push("a"))));
    bar.open_menu(0);
    bar.close();
    // Closing drops the selected item, so nothing is left to run
    assert!(!bar.execute_selected());
    assert!(log.borrow().is_empty());
}

#[test]
fn test_menu_bar_separator_only_menu_takes_the_fallback_width() {
    // Separators have no width of their own; with nothing else the
    // dropdown falls back to 10 columns
    let mut bar = MenuBar::new().menu(Menu::new("A").separator());
    bar.open_menu(0);
    let b = render(&bar, 14, 4);
    assert_eq!(row(&b, 1), format!("┌{}┐    ", "─".repeat(8)));
    assert_eq!(row(&b, 2), format!("├{}┤    ", "─".repeat(8)));
}
