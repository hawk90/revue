//! The DevTools panel draws the inspected widget's outline only on the
//! Inspector tab, over whole cells of the app under it.

use crate::{corners, devtools_with, screen};
use revue::devtools::{DevToolsConfig, DevToolsTab, Inspector};
use revue::layout::Rect;
use revue::render::{Buffer, Cell, Modifier};
use revue::style::Color;

#[test]
fn no_outline_on_another_tab() {
    let mut devtools = devtools_with(Inspector::new());
    devtools.set_tab(DevToolsTab::Styles);
    assert_eq!(corners(&screen(&devtools)), [None; 4]);
}

/// A 100x20 screen of bold, underlined, red `X`s, each with a link and an
/// escape sequence, as an app could leave under the DevTools
fn styled_screen() -> Buffer {
    let mut buffer = Buffer::new(100, 20);
    let link = buffer.register_hyperlink("https://example.com");
    let seq = buffer.register_sequence("\x1b]1337;X\x07");
    for y in 0..20 {
        for x in 0..100 {
            let mut cell = Cell::new('X').fg(Color::RED).bold().underline();
            cell.hyperlink_id = Some(link);
            cell.sequence_id = Some(seq);
            buffer.set(x, y, cell);
        }
    }
    buffer
}

/// The panel is drawn over whole cells: the app's modifiers, hyperlink and
/// escape sequence stay out of every panel cell (border, tabs, content,
/// blank), and stay on the app left of it (#1548).
#[test]
fn the_panel_does_not_take_the_modifiers_links_or_sequences_under_it() {
    let devtools = devtools_with(Inspector::new());
    let mut buffer = styled_screen();
    devtools.render(&mut buffer, Rect::new(0, 0, 100, 20));

    for y in 0..20 {
        for x in 50..100 {
            let cell = buffer.get(x, y).unwrap();
            assert_eq!(
                cell.modifier,
                Modifier::empty(),
                "{:?} ({x}, {y})",
                cell.symbol
            );
            assert_eq!(cell.hyperlink_id, None, "({x}, {y})");
            assert_eq!(cell.sequence_id, None, "({x}, {y})");
        }
        // The app's last column, right of the outline, keeps its own
        let app = buffer.get(49, y).unwrap();
        assert_eq!(app.symbol, 'X', "(49, {y})");
        assert_eq!(app.modifier, Modifier::BOLD | Modifier::UNDERLINE);
        assert!(app.hyperlink_id.is_some() && app.sequence_id.is_some());
    }
}

/// The blank cells of the panel take its colors, not the app's red fg
#[test]
fn blank_panel_cells_take_the_panel_colors() {
    let devtools = devtools_with(Inspector::new());
    let config = DevToolsConfig::default();
    let mut buffer = styled_screen();
    devtools.render(&mut buffer, Rect::new(0, 0, 100, 20));
    // Inside the border, below every tab and tree row
    let cell = buffer.get(98, 18).unwrap();
    assert_eq!(cell.symbol, ' ');
    assert_eq!(cell.fg, Some(config.fg_color));
    assert_eq!(cell.bg, Some(config.bg_color));
}

/// A panel at an offset area: a wide glyph of the app that the panel's
/// left edge cuts in half is blanked, not left half-drawn
#[test]
fn a_wide_glyph_cut_by_the_panel_edge_is_blanked() {
    let devtools = devtools_with(Inspector::new());
    let mut buffer = Buffer::new(110, 22);
    // The panel takes x 60..110 of the area at (10, 2); '가' spans 59-60
    for y in 2..22 {
        buffer.put_str(59, y, "가");
    }
    devtools.render(&mut buffer, Rect::new(10, 2, 100, 20));
    for y in 2..22 {
        assert_eq!(buffer.get(59, y).unwrap().symbol, ' ', "(59, {y})");
        assert!(!buffer.get(60, y).unwrap().is_continuation(), "(60, {y})");
    }
}
