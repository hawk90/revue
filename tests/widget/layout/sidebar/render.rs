//! Sidebar rendering tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::CollapseMode;
use revue::widget::Sidebar;
use revue::widget::SidebarItem;
use revue::widget::SidebarSection;
use revue::widget::View;

use super::{bar_width, is_blank, render_rows, row, screen, BAR_BG};

// =========================================================================
// Render edge case tests
// =========================================================================

#[test]
fn test_render_sidebar_too_narrow() {
    // width < 3: nothing is drawn
    assert!(is_blank(&screen(&Sidebar::new().header("H"), 2, 10)));
}

#[test]
fn test_render_sidebar_too_short() {
    // height < 2: nothing is drawn
    assert!(is_blank(&screen(&Sidebar::new().header("H"), 10, 1)));
}

#[test]
fn test_render_sidebar_zero_width() {
    let sidebar = Sidebar::new().items(vec![SidebarItem::new("a", "A")]);
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 0, 10));
    sidebar.render(&mut ctx);
    assert!(is_blank(&buffer));
}

#[test]
fn test_render_sidebar_zero_height() {
    let sidebar = Sidebar::new().items(vec![SidebarItem::new("a", "A")]);
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 10, 0));
    sidebar.render(&mut ctx);
    assert!(is_blank(&buffer));
}

// =========================================================================
// Collapse mode tests
// =========================================================================

#[test]
fn test_render_sidebar_expanded_mode() {
    let sidebar = Sidebar::new()
        .collapse_mode(CollapseMode::Expanded)
        .expanded_width(20);
    // The bar takes 20 columns, border last; the rest is untouched.
    assert_eq!(bar_width(&screen(&sidebar, 30, 10)), 20);
}

#[test]
fn test_render_sidebar_collapsed_mode() {
    let sidebar = Sidebar::new()
        .collapse_mode(CollapseMode::Collapsed)
        .collapsed_width(5);
    assert_eq!(bar_width(&screen(&sidebar, 30, 10)), 5);
}

#[test]
fn test_render_sidebar_auto_mode_collapsed() {
    let sidebar = Sidebar::new()
        .collapse_mode(CollapseMode::Auto)
        .collapse_threshold(15)
        .expanded_width(20)
        .collapsed_width(5);
    // width 10 < threshold 15: the collapsed width
    assert_eq!(bar_width(&screen(&sidebar, 10, 10)), 5);
}

#[test]
fn test_render_sidebar_auto_mode_expanded() {
    let sidebar = Sidebar::new()
        .collapse_mode(CollapseMode::Auto)
        .collapse_threshold(15)
        .expanded_width(20)
        .collapsed_width(5);
    // width 30 >= threshold 15: the expanded width
    assert_eq!(bar_width(&screen(&sidebar, 30, 10)), 20);
}

// =========================================================================
// Header/footer rendering tests
// =========================================================================

#[test]
fn test_render_sidebar_with_header() {
    let sidebar = Sidebar::new().header("Test Header");
    let rows = render_rows(&sidebar, 20, 10);
    assert_eq!(rows[0], "    Test Header    │");
    // The separator under it, up to the border
    assert_eq!(rows[1], "───────────────────│");
}

#[test]
fn test_render_sidebar_with_footer() {
    let sidebar = Sidebar::new().footer("Test Footer");
    let rows = render_rows(&sidebar, 20, 10);
    // Separator at height - 2, footer on the last row
    assert_eq!(rows[8], "───────────────────│");
    assert_eq!(rows[9], "    Test Footer    │");
}

#[test]
fn test_render_sidebar_with_header_and_footer() {
    let sidebar = Sidebar::new()
        .header("Header")
        .footer("Footer")
        .items(vec![SidebarItem::new("a", "A")]);
    let rows = render_rows(&sidebar, 20, 10);
    assert_eq!(rows[0], "       Header      │");
    assert_eq!(rows[1], "───────────────────│");
    assert_eq!(rows[2], "   A               │");
    assert_eq!(rows[8], "───────────────────│");
    assert_eq!(rows[9], "       Footer      │");
}

#[test]
fn test_render_sidebar_header_truncation() {
    let long_header = "This is a very long header that should be truncated";
    let sidebar = Sidebar::new().header(long_header).expanded_width(15);
    // Cut to 13 columns (two less than the bar) and centered; the border
    // is drawn over the last column.
    assert_eq!(render_rows(&sidebar, 15, 10)[0], " This is a ver│");
}

// =========================================================================
// Background rendering tests
// =========================================================================

#[test]
fn test_render_sidebar_background() {
    let sidebar = Sidebar::new().bg(revue::style::Color::rgb(0, 0, 255));

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sidebar.render(&mut ctx);

    // Background should be rendered
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.bg, Some(revue::style::Color::rgb(0, 0, 255)));
}

// =========================================================================
// Border rendering tests
// =========================================================================

#[test]
fn test_render_sidebar_border() {
    let sidebar = Sidebar::new()
        .border_color(revue::style::Color::rgb(255, 0, 0))
        .expanded_width(15);
    let buffer = screen(&sidebar, 20, 10);
    // Right border at width - 1 on every row, in the border color
    for y in 0..10 {
        let cell = buffer.get(14, y).unwrap();
        assert_eq!(cell.symbol, '│', "y={y}");
        assert_eq!(cell.fg, Some(revue::style::Color::rgb(255, 0, 0)));
    }
    assert_eq!(bar_width(&buffer), 15);
}

// =========================================================================
// Color configuration tests
// =========================================================================

#[test]
fn test_render_sidebar_with_colors() {
    let white = revue::style::Color::rgb(255, 255, 255);
    let black = revue::style::Color::rgb(0, 0, 0);
    let gray = revue::style::Color::rgb(128, 128, 128);
    let cyan = revue::style::Color::rgb(0, 255, 255);
    let sidebar = Sidebar::new()
        .fg(white)
        .bg(black)
        .border_color(gray)
        .section_color(cyan)
        .section(SidebarSection::titled(
            "Sec",
            vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")],
        ));

    let buffer = screen(&sidebar, 20, 10);
    // Section title in the section color on the bar background
    let title = buffer.get(1, 0).unwrap();
    assert_eq!(
        (title.symbol, title.fg, title.bg),
        ('S', Some(cyan), Some(black))
    );
    // A plain item in fg on bg (row 0, the section row, holds the hover)
    let item = buffer.get(3, 2).unwrap();
    assert_eq!(
        (item.symbol, item.fg, item.bg),
        ('B', Some(white), Some(black))
    );
    // Border and empty rows
    assert_eq!(buffer.get(19, 5).unwrap().fg, Some(gray));
    assert_eq!(buffer.get(0, 9).unwrap().bg, Some(black));
}

// =========================================================================
// Width constraint tests
// =========================================================================

#[test]
fn test_render_sidebar_content_width_clamping() {
    let sidebar = Sidebar::new()
        .expanded_width(30)
        .collapse_mode(CollapseMode::Expanded);
    // Content width is clamped to the 20 available columns
    let buffer = screen(&sidebar, 20, 10);
    assert_eq!(buffer.get(19, 0).unwrap().symbol, '│');
    assert_eq!(bar_width(&buffer), 20);
}

// =========================================================================
// Empty sidebar tests
// =========================================================================

#[test]
fn test_render_empty_sidebar() {
    let buffer = screen(&Sidebar::new(), 20, 10);
    // An empty bar: its background and the border on every row
    for y in 0..10 {
        assert_eq!(row(&buffer, y), "                   │");
        assert!((0..20).all(|x| buffer.get(x, y).unwrap().bg == Some(BAR_BG)));
    }
}

#[test]
fn test_sidebar_render() {
    let sb = Sidebar::new()
        .header("My App")
        .items(vec![
            SidebarItem::new("home", "Home").icon('H'),
            SidebarItem::new("settings", "Settings").icon('S'),
        ])
        .footer("v1.0");
    let rows = render_rows(&sb, 30, 20);
    assert_eq!(rows[0], "         My App        │      ");
    assert_eq!(rows[1], "───────────────────────│      ");
    assert_eq!(rows[2], "   H Home              │      ");
    assert_eq!(rows[3], "   S Settings          │      ");
    assert_eq!(rows[18], "───────────────────────│      ");
    assert_eq!(rows[19], "          v1.0         │      ");
}

#[test]
fn test_sidebar_render_collapsed() {
    let sb = Sidebar::new()
        .collapse_mode(CollapseMode::Collapsed)
        .items(vec![
            SidebarItem::new("home", "Home").icon('H'),
            SidebarItem::new("settings", "Settings").icon('S'),
        ]);
    let rows = render_rows(&sb, 10, 20);
    // Icons only, no labels
    assert_eq!(rows[0], " H │      ");
    assert_eq!(rows[1], " S │      ");
    assert_eq!(rows[2], "   │      ");
}

#[test]
fn test_sidebar_render_with_badge() {
    let sb = Sidebar::new().items(vec![
        SidebarItem::new("inbox", "Inbox").icon('I').badge("99+"),
        SidebarItem::new("sent", "Sent").icon('S'),
    ]);
    let rows = render_rows(&sb, 30, 20);
    // The badge ends two columns before the bar's edge
    assert_eq!(rows[0], "   I Inbox         99+ │      ");
    assert_eq!(rows[1], "   S Sent              │      ");
}

#[test]
fn test_sidebar_render_small_area() {
    let sb = Sidebar::new().items(vec![SidebarItem::new("a", "A")]);
    // Below the 3x2 minimum: nothing drawn
    assert!(is_blank(&screen(&sb, 2, 1)));
}
