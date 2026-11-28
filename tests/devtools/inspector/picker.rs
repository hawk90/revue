//! `devtools::inspector::picker`: the component picker's tooltip

use revue::devtools::{ComponentPicker, WidgetNode};
use revue::layout::Rect;
use revue::render::{Buffer, Cell, Modifier};
use revue::style::Color;
use std::collections::HashMap;

const TOOLTIP_BG: Color = Color::rgb(40, 40, 50);

/// A 40x6 screen of bold, red `X`s, each with a link and an escape sequence
fn styled_screen() -> Buffer {
    let mut buffer = Buffer::new(40, 6);
    let link = buffer.register_hyperlink("https://example.com");
    let seq = buffer.register_sequence("\x1b]1337;X\x07");
    for y in 0..6 {
        for x in 0..40 {
            let mut cell = Cell::new('X').fg(Color::RED).bold();
            cell.hyperlink_id = Some(link);
            cell.sequence_id = Some(seq);
            buffer.set(x, y, cell);
        }
    }
    buffer
}

/// A picker hovering a `Button#ok` (off screen, so no outline is drawn on
/// the tooltip row) with the mouse at `(x, y)`
fn hover(x: u16, y: u16, buffer: &mut Buffer) {
    let mut picker = ComponentPicker::new();
    picker.enable();
    picker.set_mouse_pos(x, y);
    picker.set_hovered(Some(1));
    let mut nodes = HashMap::new();
    nodes.insert(
        1,
        WidgetNode::new(1, "Button")
            .widget_id("ok")
            .rect(Rect::new(100, 100, 1, 1)),
    );
    let area = Rect::new(0, 0, buffer.width(), buffer.height());
    picker.render_overlay(buffer, area, &nodes);
}

/// The tooltip is drawn over whole cells: its padding and text take its
/// own fg and bg, and none of the app's modifiers, link or escape sequence;
/// the cells beside it keep the app's (#1548)
#[test]
fn the_tooltip_does_not_take_the_fg_modifiers_links_or_sequences_under_it() {
    let mut buffer = styled_screen();
    hover(3, 1, &mut buffer);

    // " Button#ok " at (5, 2): 9 columns of text and one of padding each side
    let row: String = (5..16).map(|x| buffer.get(x, 2).unwrap().symbol).collect();
    assert_eq!(row, " Button#ok ");
    for x in 5..16 {
        let cell = buffer.get(x, 2).unwrap();
        assert_eq!(cell.fg, Some(Color::WHITE), "({x}, 2)");
        assert_eq!(cell.bg, Some(TOOLTIP_BG), "({x}, 2)");
        assert_eq!(cell.modifier, Modifier::empty(), "({x}, 2)");
        assert_eq!(cell.hyperlink_id, None, "({x}, 2)");
        assert_eq!(cell.sequence_id, None, "({x}, 2)");
    }
    for x in [4, 16] {
        let cell = buffer.get(x, 2).unwrap();
        assert_eq!(cell.symbol, 'X', "({x}, 2)");
        assert_eq!(cell.fg, Some(Color::RED), "({x}, 2)");
        assert_eq!(cell.modifier, Modifier::BOLD, "({x}, 2)");
        assert!(cell.hyperlink_id.is_some(), "({x}, 2)");
    }
}

/// A tooltip past the right edge is cut at the last column
#[test]
fn a_tooltip_at_the_right_edge_is_cut_at_the_last_column() {
    let mut buffer = styled_screen();
    hover(33, 4, &mut buffer);
    let row: String = (35..40).map(|x| buffer.get(x, 5).unwrap().symbol).collect();
    assert_eq!(row, " Butt");
    let last = buffer.get(39, 5).unwrap();
    assert_eq!(last.bg, Some(TOOLTIP_BG));
    assert_eq!(last.modifier, Modifier::empty());
    assert_eq!(buffer.get(34, 5).unwrap().symbol, 'X');
}

/// A mouse at the largest coordinate draws nothing and does not overflow
#[test]
fn a_tooltip_at_the_largest_coordinate_draws_nothing() {
    let mut buffer = styled_screen();
    let before = buffer.clone();
    hover(u16::MAX, u16::MAX, &mut buffer);
    for y in 0..6 {
        for x in 0..40 {
            assert_eq!(buffer.get(x, y), before.get(x, y), "({x}, {y})");
        }
    }
}

/// A wide glyph the tooltip's left edge cuts in half is blanked
#[test]
fn a_wide_glyph_cut_by_the_tooltip_edge_is_blanked() {
    let mut buffer = Buffer::new(40, 6);
    buffer.put_str(4, 2, "가");
    hover(3, 1, &mut buffer);
    assert_eq!(buffer.get(4, 2).unwrap().symbol, ' ');
    assert_eq!(buffer.get(5, 2).unwrap().symbol, ' ');
    assert!(!buffer.get(5, 2).unwrap().is_continuation());
}
