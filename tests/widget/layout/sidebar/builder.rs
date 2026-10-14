//! Sidebar Builder tests

use revue::style::Color;
use revue::widget::{CollapseMode, FlattenedItem, Sidebar, SidebarItem, SidebarSection};

#[test]
fn test_sidebar_items() {
    let sb = Sidebar::new()
        .section(SidebarSection::titled(
            "Top",
            vec![SidebarItem::new("a", "A")],
        ))
        .items(vec![
            SidebarItem::new("home", "Home"),
            SidebarItem::new("settings", "Settings"),
        ]);

    // `items` appends an untitled section: no title row before its items.
    let ids: Vec<String> = sb
        .visible_items()
        .into_iter()
        .map(|f| match f {
            FlattenedItem::Section(t) => format!("section {t:?}"),
            FlattenedItem::Item { item, .. } => item.id,
        })
        .collect();
    assert_eq!(ids, ["section Some(\"Top\")", "a", "home", "settings"]);
}

#[test]
fn test_sidebar_section() {
    let sb = Sidebar::new()
        .section(SidebarSection::titled(
            "Navigation",
            vec![SidebarItem::new("home", "Home")],
        ))
        .section(SidebarSection::titled(
            "Settings",
            vec![SidebarItem::new("prefs", "Preferences")],
        ));

    // 2 items across 2 sections
    assert_eq!(sb.item_count(), 2);
}

#[test]
fn test_sidebar_selected() {
    let sb = Sidebar::new()
        .items(vec![
            SidebarItem::new("home", "Home"),
            SidebarItem::new("mail", "Mail"),
        ])
        .selected("home")
        .selected("mail");

    // The last call wins, and the row is drawn white on blue.
    assert_eq!(sb.selected_id(), Some("mail"));
    let buffer = super::screen(&sb, 24, 3);
    let cell = buffer.get(3, 1).unwrap();
    assert_eq!(
        (cell.symbol, cell.fg, cell.bg),
        ('M', Some(Color::WHITE), Some(Color::BLUE))
    );
}

#[test]
fn test_sidebar_collapse_mode() {
    let sb_expanded = Sidebar::new().collapse_mode(CollapseMode::Expanded);
    assert!(!sb_expanded.is_collapsed());

    let sb_collapsed = Sidebar::new().collapse_mode(CollapseMode::Collapsed);
    assert!(sb_collapsed.is_collapsed());
}

#[test]
fn test_sidebar_dimensions() {
    let sb = Sidebar::new()
        .expanded_width(30)
        .collapsed_width(5)
        .collapse_threshold(15);

    assert_eq!(sb.current_width(), 30);
}

#[test]
fn test_sidebar_header_footer() {
    let sb = Sidebar::new().header("App Name").footer("v1.0.0");

    let rows = super::render_rows(&sb, 24, 6);
    assert_eq!(rows[0], "        App Name       │");
    assert_eq!(rows[5], "         v1.0.0        │");
    assert_eq!(sb.item_count(), 0);
}

#[test]
fn test_sidebar_styling() {
    let sb = Sidebar::new()
        .fg(Color::WHITE)
        .bg(Color::BLACK)
        .selected_style(Color::WHITE, Color::BLUE)
        .hover_style(Color::WHITE, Color::rgb(50, 50, 50))
        .disabled_color(Color::rgb(100, 100, 100))
        .section_color(Color::CYAN)
        .badge_style(Color::WHITE, Color::RED)
        .border_color(Color::rgb(60, 60, 60))
        .section(SidebarSection::titled(
            "S",
            vec![
                SidebarItem::new("hover", "Hover"),
                SidebarItem::new("sel", "Sel").badge("1"),
                SidebarItem::new("off", "Off").disabled(true),
                SidebarItem::new("plain", "Plain"),
            ],
        ))
        .selected("sel");
    let mut sb = sb;
    sb.hover_down();

    let buffer = super::screen(&sb, 24, 6);
    let at = |x: u16, y: u16| {
        let c = buffer.get(x, y).unwrap();
        (c.symbol, c.fg, c.bg)
    };
    assert_eq!(at(1, 0), ('S', Some(Color::CYAN), Some(Color::BLACK)));
    assert_eq!(
        at(3, 1),
        ('H', Some(Color::WHITE), Some(Color::rgb(50, 50, 50)))
    );
    assert_eq!(at(3, 2), ('S', Some(Color::WHITE), Some(Color::BLUE)));
    assert_eq!(at(21, 2), ('1', Some(Color::WHITE), Some(Color::RED)));
    assert_eq!(
        at(3, 3),
        ('O', Some(Color::rgb(100, 100, 100)), Some(Color::BLACK))
    );
    assert_eq!(at(3, 4), ('P', Some(Color::WHITE), Some(Color::BLACK)));
    assert_eq!(
        at(23, 0),
        ('│', Some(Color::rgb(60, 60, 60)), Some(Color::BLACK))
    );
}
