//! DropZone drawing: each style's frame or indicator, the placeholder and
//! the drag verdict, colors, focus and the disabled state

use revue::event::drag::DragData;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::theme::{DISABLED_FG, LIGHT_GRAY};
use revue::widget::traits::{Draggable, RenderContext, View};
use revue::widget::{hstack, vstack, DropZone, DropZoneStyle, Stack, Text};

use super::{fg, render, row, symbol};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

// =========================================================================
// Styles
// =========================================================================

#[test]
fn test_drop_zone_style_solid() {
    let zone = DropZone::new("Test").style(DropZoneStyle::Solid);
    let buffer = render(&zone, 10, 3);
    assert_eq!(row(&buffer, 0), "┌────────┐");
    assert_eq!(symbol(&buffer, 0, 1), '│');
    assert_eq!(row(&buffer, 2), "└────────┘");
}
#[test]
fn test_drop_zone_style_dashed() {
    let zone = DropZone::new("Test").style(DropZoneStyle::Dashed);
    let buffer = render(&zone, 10, 3);
    assert_eq!(row(&buffer, 0), "┌╌╌╌╌╌╌╌╌┐");
    assert_eq!(symbol(&buffer, 0, 1), '╎');
    assert_eq!(symbol(&buffer, 9, 1), '╎');
}
#[test]
fn test_drop_zone_style_highlight() {
    let mut zone = DropZone::new("Test").style(DropZoneStyle::Highlight);
    let buffer = render(&zone, 10, 3);
    // No border at rest
    assert_eq!(row(&buffer, 0).trim(), "");
    assert_eq!(buffer.get(0, 0).unwrap().bg, None);

    // Filled while an acceptable drag is over it
    zone.set_hovered(true, true);
    let accept = render(&zone, 10, 3);
    assert!(accept.get(0, 0).unwrap().bg.is_some());

    // ... in a different color when the drag would be rejected
    zone.set_hovered(true, false);
    let reject = render(&zone, 10, 3);
    assert!(reject.get(0, 0).unwrap().bg.is_some());
    assert_ne!(accept.get(0, 0).unwrap().bg, reject.get(0, 0).unwrap().bg);
}
#[test]
fn test_drop_zone_style_minimal() {
    let mut zone = DropZone::new("Test").style(DropZoneStyle::Minimal);
    let buffer = render(&zone, 10, 3);
    assert_eq!(symbol(&buffer, 0, 0), '│');
    assert_eq!(symbol(&buffer, 9, 0), ' ');

    zone.set_hovered(true, true);
    assert_eq!(symbol(&render(&zone, 10, 3), 0, 0), '▶');

    zone.set_hovered(true, false);
    assert_eq!(symbol(&render(&zone, 10, 3), 0, 0), '✗');
}
// =========================================================================
// focused / disabled / fg / bg
// =========================================================================

#[test]
fn test_drop_zone_fg_colors_the_placeholder() {
    let zone = DropZone::new("Drop").fg(Color::MAGENTA);
    let buffer = render(&zone, 20, 5);
    // The placeholder sits on the middle row, from x = 2.
    assert_eq!(symbol(&buffer, 2, 2), 'D');
    assert_eq!(buffer.get(2, 2).unwrap().fg, Some(Color::MAGENTA));
}
#[test]
fn test_drop_zone_bg_fills_the_zone() {
    let zone = DropZone::new("Drop").bg(Color::BLUE);
    let buffer = render(&zone, 20, 5);
    assert_eq!(buffer.get(10, 1).unwrap().bg, Some(Color::BLUE));
    assert_eq!(buffer.get(2, 2).unwrap().bg, Some(Color::BLUE));
}
#[test]
fn test_drop_zone_focused_bolds_the_border() {
    use revue::render::Modifier;
    let rest = render(&DropZone::new("Drop"), 20, 5);
    assert!(!rest.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
    let focused = render(&DropZone::new("Drop").focused(true), 20, 5);
    assert!(focused.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
}
#[test]
fn test_drop_zone_disabled_refuses_drops_and_greys_out() {
    use std::cell::Cell as StdCell;
    use std::rc::Rc;

    let called = Rc::new(StdCell::new(false));
    let seen = called.clone();
    let mut zone = DropZone::new("Drop").disabled(true).on_drop(move |_| {
        seen.set(true);
        true
    });
    assert!(!zone.can_drop());
    zone.on_drag_enter(&DragData::text("x"));
    let buffer = render(&zone, 20, 5);
    assert_ne!(
        row(&buffer, 2).trim(),
        "Drop here!",
        "a disabled zone invited the drop"
    );
    assert!(!Draggable::on_drop(&mut zone, DragData::text("x")));
    assert!(!called.get(), "a disabled zone ran its drop handler");
}
#[test]
fn test_drop_zone_disabled_reaches_the_dom() {
    use revue::testing::PipelineHarness;
    let zone = DropZone::new("Drop").element_id("dz").disabled(true);
    let mut h = PipelineHarness::new(20, 5).dom_from_render(true);
    h.draw(&zone);
    assert!(h.node_state("dz").is_some_and(|s| s.disabled));
}
// =========================================================================
// Placement and edges
// =========================================================================

const STYLES: [DropZoneStyle; 4] = [
    DropZoneStyle::Solid,
    DropZoneStyle::Dashed,
    DropZoneStyle::Highlight,
    DropZoneStyle::Minimal,
];

fn bg(buffer: &Buffer, x: u16, y: u16) -> Option<Color> {
    buffer.get(x, y).unwrap().bg
}

fn bold(buffer: &Buffer, x: u16, y: u16) -> bool {
    buffer.get(x, y).unwrap().modifier.contains(Modifier::BOLD)
}

#[test]
fn zero_areas_and_offset_placement() {
    for style in STYLES {
        for (hovered, accept) in [(false, false), (true, true), (true, false)] {
            let mut zone = DropZone::new("Drop files")
                .style(style)
                .focused(true)
                .bg(Color::BLUE);
            zone.set_hovered(hovered, accept);
            assert_zero_areas_draw_nothing(&zone);
            assert_same_when_offset(zone, 20, 5);
        }
    }
}

#[test]
fn an_empty_placeholder_leaves_the_inside_blank() {
    for style in STYLES {
        let buffer = render(&DropZone::new("").style(style), 10, 3);
        let middle = row(&buffer, 1);
        let inside: String = middle.chars().skip(1).take(8).collect();
        assert_eq!(inside, " ".repeat(8), "{style:?}");
    }
}

#[test]
fn a_narrow_zone_keeps_its_frame_and_drops_the_text() {
    let two = render(&DropZone::new("Drop"), 2, 4);
    assert_eq!(
        (0..4).map(|y| row(&two, y)).collect::<Vec<_>>(),
        ["┌┐", "││", "││", "└┘"]
    );
    // Five columns leave one for the text, between "│ " and " │".
    let five = render(&DropZone::new("Drop"), 5, 3);
    assert_eq!(row(&five, 1), "│ D │");
}

#[test]
fn the_placeholder_sits_two_columns_in_on_the_middle_row() {
    let buffer = render(&DropZone::new("Drop"), 20, 7);
    assert_eq!(row(&buffer, 3), "│ Drop             │");
    for y in [1, 2, 4, 5] {
        assert_eq!(row(&buffer, y), "│                  │", "row {y}");
    }
}

#[test]
fn a_wide_placeholder_is_cut_before_a_glyph_that_does_not_fit() {
    // Nine columns leave five for the text: two wide glyphs, not half a third.
    let buffer = render(&DropZone::new("파일을"), 9, 3);
    assert_eq!(symbol(&buffer, 2, 1), '파');
    assert_eq!(symbol(&buffer, 4, 1), '일');
    assert_eq!(symbol(&buffer, 6, 1), ' ');
    assert_eq!(symbol(&buffer, 8, 1), '│');
}

// =========================================================================
// Colors
// =========================================================================

#[test]
fn at_rest_the_border_is_grey_and_the_placeholder_light_grey() {
    let buffer = render(&DropZone::new("Drop"), 20, 3);
    for x in 0..20 {
        assert_eq!(fg(&buffer, x, 0), Some(DISABLED_FG), "top {x}");
        assert_eq!(fg(&buffer, x, 2), Some(DISABLED_FG), "bottom {x}");
    }
    assert_eq!(fg(&buffer, 0, 1), Some(DISABLED_FG));
    assert_eq!(fg(&buffer, 19, 1), Some(DISABLED_FG));
    assert_eq!(fg(&buffer, 2, 1), Some(LIGHT_GRAY));
}

#[test]
fn the_drag_verdict_is_written_in_the_border_color() {
    let green = Color::rgb(100, 200, 100);
    let red = Color::rgb(200, 100, 100);
    let mut zone = DropZone::new("Drop").fg(Color::MAGENTA);
    zone.set_hovered(true, true);
    let accept = render(&zone, 20, 3);
    assert_eq!(row(&accept, 1), "│ Drop here!       │");
    assert_eq!(fg(&accept, 2, 1), Some(green));
    assert_eq!(fg(&accept, 0, 0), Some(green));

    zone.set_hovered(true, false);
    let reject = render(&zone, 20, 3);
    assert_eq!(row(&reject, 1), "│ Cannot drop here │");
    assert_eq!(fg(&reject, 2, 1), Some(red));
    assert_eq!(fg(&reject, 19, 2), Some(red));

    let custom = {
        let mut z = DropZone::new("Drop").hover_color(Color::CYAN);
        z.set_hovered(true, true);
        render(&z, 20, 3)
    };
    assert_eq!(fg(&custom, 2, 1), Some(Color::CYAN));
}

#[test]
fn disabled_greys_the_border_and_text_until_a_drag_is_rejected() {
    let mut zone = DropZone::new("Drop")
        .disabled(true)
        .border_color(Color::YELLOW)
        .fg(Color::MAGENTA);
    let rest = render(&zone, 20, 3);
    assert_eq!(fg(&rest, 0, 0), Some(DISABLED_FG));
    assert_eq!(fg(&rest, 2, 1), Some(DISABLED_FG));

    zone.on_drag_enter(&DragData::text("x"));
    let hovered = render(&zone, 20, 3);
    assert_eq!(row(&hovered, 1), "│ Cannot drop here │");
    assert_eq!(fg(&hovered, 0, 0), Some(Color::rgb(200, 100, 100)));
}

#[test]
fn the_highlight_fill_covers_the_background() {
    let mut zone = DropZone::new("Drop")
        .style(DropZoneStyle::Highlight)
        .bg(Color::BLUE);
    let rest = render(&zone, 20, 3);
    for (x, y) in [(0, 0), (2, 1), (19, 2)] {
        assert_eq!(bg(&rest, x, y), Some(Color::BLUE), "({x}, {y})");
    }
    // The verdict's own cells lose the fill (#982); the cells
    // around it are checked.
    zone.set_hovered(true, true);
    let accept = render(&zone, 20, 3);
    for (x, y) in [(0, 0), (1, 1), (19, 1), (19, 2)] {
        assert_eq!(
            bg(&accept, x, y),
            Some(Color::rgb(30, 60, 30)),
            "({x}, {y})"
        );
    }
    zone.set_hovered(true, false);
    let reject = render(&zone, 20, 3);
    assert_eq!(bg(&reject, 1, 1), Some(Color::rgb(60, 30, 30)));
}

#[test]
fn the_minimal_indicator_runs_down_every_row_in_the_border_color() {
    let zone = DropZone::new("Drop")
        .style(DropZoneStyle::Minimal)
        .border_color(Color::YELLOW);
    let buffer = render(&zone, 10, 5);
    for y in 0..5 {
        assert_eq!(symbol(&buffer, 0, y), '│', "row {y}");
        assert_eq!(fg(&buffer, 0, y), Some(Color::YELLOW), "row {y}");
        assert_eq!(symbol(&buffer, 9, y), ' ', "row {y}");
    }
    assert_eq!(row(&buffer, 2), "│ Drop    ");
}

// =========================================================================
// Focus
// =========================================================================

#[test]
fn focus_bolds_the_whole_frame_and_nothing_inside() {
    for style in [DropZoneStyle::Solid, DropZoneStyle::Dashed] {
        let buffer = render(&DropZone::new("Drop").style(style).focused(true), 8, 4);
        for y in 0..4 {
            for x in 0..8 {
                let on_frame = y == 0 || y == 3 || x == 0 || x == 7;
                assert_eq!(bold(&buffer, x, y), on_frame, "{style:?} ({x}, {y})");
            }
        }
    }
}

#[test]
fn focus_bolds_the_minimal_indicator_and_not_the_highlight_style() {
    let minimal = render(
        &DropZone::new("Drop")
            .style(DropZoneStyle::Minimal)
            .focused(true),
        8,
        3,
    );
    for y in 0..3 {
        assert!(bold(&minimal, 0, y), "row {y}");
        assert!(!bold(&minimal, 2, y), "row {y}");
    }
    let highlight = render(
        &DropZone::new("Drop")
            .style(DropZoneStyle::Highlight)
            .focused(true),
        8,
        3,
    );
    assert!((0..3).all(|y| (0..8).all(|x| !bold(&highlight, x, y))));
}

#[test]
fn a_disabled_zone_is_not_bold_when_focused() {
    let buffer = render(&DropZone::new("Drop").focused(true).disabled(true), 8, 3);
    assert!(!bold(&buffer, 0, 0));
    assert!(!bold(&buffer, 7, 2));
}

// =========================================================================
// Stylesheet
// =========================================================================

const INK: Color = Color {
    r: 17,
    g: 34,
    b: 51,
    a: 255,
};
const EDGE: Color = Color {
    r: 68,
    g: 85,
    b: 102,
    a: 255,
};

fn styled(css: &str, zone: &DropZone) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 12, 3).dom_from_render(true);
    h.draw(zone);
    h
}

#[test]
fn the_stylesheet_color_reaches_the_border_and_the_placeholder() {
    let h = styled(
        "#w { color: #112233; }",
        &DropZone::new("Drop").element_id("w"),
    );
    assert_eq!(fg(h.buffer(), 0, 0), Some(INK));
    assert_eq!(fg(h.buffer(), 2, 1), Some(INK));
}

#[test]
fn a_stylesheet_border_color_is_for_the_border_only() {
    let h = styled(
        "#w { color: #112233; border-color: #445566; }",
        &DropZone::new("Drop").element_id("w"),
    );
    assert_eq!(fg(h.buffer(), 0, 0), Some(EDGE));
    assert_eq!(fg(h.buffer(), 11, 2), Some(EDGE));
    assert_eq!(fg(h.buffer(), 2, 1), Some(INK));
}

#[test]
fn the_builders_and_a_drag_win_over_the_stylesheet() {
    let css = "#w { color: #112233; border-color: #445566; }";
    let built = styled(
        css,
        &DropZone::new("Drop")
            .element_id("w")
            .border_color(Color::YELLOW)
            .fg(Color::MAGENTA),
    );
    assert_eq!(fg(built.buffer(), 0, 0), Some(Color::YELLOW));
    assert_eq!(fg(built.buffer(), 2, 1), Some(Color::MAGENTA));

    let mut zone = DropZone::new("Drop").element_id("w");
    zone.set_hovered(true, true);
    let hovered = styled(css, &zone);
    assert_eq!(fg(hovered.buffer(), 0, 0), Some(Color::rgb(100, 200, 100)));
}

// =========================================================================
// #1107: small areas, min_height as a lower bound, an explicit grey border
// =========================================================================

fn rows_of(view: &dyn View, width: u16, height: u16) -> Vec<String> {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    view.render(&mut ctx);
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect()
        })
        .collect()
}

#[test]
fn the_frame_stays_closed_whatever_the_min_height() {
    for min in [0, 3, 5, 10, u16::MAX] {
        assert_eq!(
            rows_of(&DropZone::new("Drop").min_height(min), 8, 3),
            ["┌──────┐", "│ Drop │", "└──────┘"],
            "min_height {min}"
        );
        // Two rows are nothing but border; the text is not drawn over it.
        assert_eq!(
            rows_of(&DropZone::new("Drop").min_height(min), 8, 2),
            ["┌──────┐", "└──────┘"],
            "min_height {min}"
        );
        // One row: the sides and the text.
        assert_eq!(
            rows_of(&DropZone::new("Drop").min_height(min), 8, 1),
            ["│ Drop │"],
            "min_height {min}"
        );
    }
    // Taller than three rows: the text on the middle row.
    assert_eq!(
        rows_of(&DropZone::new("Drop"), 8, 5),
        ["┌──────┐", "│      │", "│ Drop │", "│      │", "└──────┘"]
    );
    // The narrowest areas stay in bounds.
    assert_eq!(rows_of(&DropZone::new("Drop"), 1, 1), ["│"]);
    assert_eq!(rows_of(&DropZone::new("Drop"), 2, 1), ["││"]);
    assert_eq!(rows_of(&DropZone::new("Drop"), 1, 2), ["┌", "└"]);
}

#[test]
fn the_other_styles_draw_into_small_areas_too() {
    let dashed = DropZone::new("Drop").style(DropZoneStyle::Dashed);
    assert_eq!(rows_of(&dashed, 8, 2), ["┌╌╌╌╌╌╌┐", "└╌╌╌╌╌╌┘"]);
    // No border to keep off: the text takes its row at any height.
    let minimal = DropZone::new("Drop").style(DropZoneStyle::Minimal);
    assert_eq!(rows_of(&minimal, 8, 2), ["│       ", "│ Drop  "]);
    assert_eq!(rows_of(&minimal, 8, 1), ["│ Drop  "]);
    let tall = DropZone::new("Drop")
        .style(DropZoneStyle::Minimal)
        .min_height(9);
    assert_eq!(rows_of(&tall, 8, 2), ["│       ", "│ Drop  "]);
}

#[test]
fn a_stack_gives_min_height_as_a_lower_bound() {
    fn column(zone: impl View + 'static) -> Stack {
        vstack()
            .child(Text::new("top"))
            .child(zone)
            .child(Text::new("end"))
    }
    let plain = || DropZone::new("Drop");
    // With room it fills, past its minimum.
    assert_eq!(
        rows_of(&column(plain().min_height(3)), 6, 7),
        [
            "top   ",
            "┌────┐",
            "│    │",
            "│ Dr │",
            "│    │",
            "└────┘",
            "end   "
        ]
    );
    // With exactly its minimum, it gets it.
    assert_eq!(
        rows_of(&column(plain().min_height(3)), 6, 5),
        ["top   ", "┌────┐", "│ Dr │", "└────┘", "end   "]
    );
    // Short of room it keeps the minimum; what no longer fits is cut, as a
    // CSS `min-height` overflows.
    assert_eq!(
        rows_of(&column(plain().min_height(3)), 6, 4),
        ["top   ", "┌────┐", "│ Dr │", "└────┘"]
    );
    // Without a minimum it takes what the others leave, as before.
    assert_eq!(
        rows_of(&column(plain().min_height(0)), 6, 4),
        ["top   ", "┌────┐", "└────┘", "end   "]
    );
    // Two fillers: the larger minimum is kept, the other takes the rest.
    let two = vstack()
        .child(plain().min_height(4))
        .child(plain().min_height(0));
    assert_eq!(
        rows_of(&two, 6, 6),
        ["┌────┐", "│    │", "│ Dr │", "└────┘", "┌────┐", "└────┘"]
    );
    // Through nested stacks: along a column it adds up, across a row it is
    // the largest.
    let nested = vstack()
        .child(vstack().child(plain().min_height(3)))
        .child(Text::new("end"));
    assert_eq!(rows_of(&nested, 6, 3), ["┌────┐", "│ Dr │", "└────┘"]);
    let across = vstack()
        .child(
            hstack()
                .child(plain().min_height(3))
                .child(plain().min_height(2)),
        )
        .child(Text::new("end"));
    assert_eq!(
        rows_of(&across, 12, 3),
        ["┌────┐┌────┐", "│ Dr ││ Dr │", "└────┘└────┘"]
    );
    // In a row it asks nothing of the width.
    let row = hstack().child(plain()).child(plain());
    assert_eq!(rows_of(&row, 8, 1), ["│  ││  │"]);
}

#[test]
fn an_explicit_grey_border_beats_the_stylesheet() {
    let zone = DropZone::new("Drop")
        .element_id("w")
        .border_color(DISABLED_FG);
    let mut h = PipelineHarness::with_css("#w { color: #112233; }", 12, 3).dom_from_render(true);
    h.draw(&zone);
    assert_eq!(h.buffer().get(0, 0).unwrap().fg, Some(DISABLED_FG));
    // Not named by the builder: the stylesheet, then the default grey.
    let zone = DropZone::new("Drop").element_id("w");
    let mut h = PipelineHarness::with_css("#w { color: #112233; }", 12, 3).dom_from_render(true);
    h.draw(&zone);
    assert_eq!(
        h.buffer().get(0, 0).unwrap().fg,
        Some(Color::rgb(17, 34, 51))
    );
    assert_eq!(
        {
            let mut buffer = Buffer::new(8, 3);
            let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 8, 3));
            DropZone::new("Drop").render(&mut ctx);
            buffer.get(0, 0).unwrap().fg
        },
        Some(DISABLED_FG)
    );
}
