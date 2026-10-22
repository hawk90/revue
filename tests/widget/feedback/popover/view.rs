//! Popover placement, the stylesheet, the shadow and the content edges, read
//! from the screen

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::{Color, Style, VisualStyle};
use revue::widget::traits::{RenderContext, View};
use revue::widget::{Popover, PopoverArrow, PopoverPosition, PopoverStyle};

use super::{corner, render, row, symbol};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

#[test]
fn test_popover_zero_areas_and_offset() {
    let popovers = [
        Popover::new("Test").anchor(12, 4).open(true),
        Popover::new("").anchor(0, 0).open(true),
        Popover::new("내용 content")
            .title("제목")
            .anchor(14, 6)
            .arrow(PopoverArrow::Unicode)
            .position(PopoverPosition::Top)
            .open(true),
        Popover::new("a\nb")
            .popover_style(PopoverStyle::Minimal)
            .arrow(PopoverArrow::Simple)
            .position(PopoverPosition::Right)
            .anchor(3, 5)
            .open(true),
        Popover::new("Shadow")
            .popover_style(PopoverStyle::Elevated)
            .position(PopoverPosition::Auto)
            .anchor(29, 11)
            .open(true),
        Popover::new("closed").anchor(12, 4),
    ];
    for p in popovers {
        assert_zero_areas_draw_nothing(&p);
        assert_same_when_offset(p.clone(), 30, 12);
        // An area smaller than the box clips it to the area
        assert_same_when_offset(p, 8, 2);
    }
}

fn render_css(p: &Popover, style: &Style) -> Buffer {
    let mut buffer = Buffer::new(40, 20);
    let mut ctx = RenderContext::with_style(&mut buffer, Rect::new(0, 0, 40, 20), style);
    p.render(&mut ctx);
    buffer
}

fn with_visual(visual: VisualStyle) -> Style {
    Style {
        visual,
        ..Default::default()
    }
}

#[test]
fn test_popover_stylesheet_colors_under_the_builders() {
    // "Test" below (20, 5), past the arrow at (20, 6): the box starts at
    // (15, 7), its text at (17, 8)
    let p = || {
        Popover::new("Test")
            .anchor(20, 5)
            .arrow(PopoverArrow::Simple)
            .open(true)
    };
    let css = with_visual(VisualStyle {
        color: Color::rgb(7, 8, 9),
        background: Color::rgb(1, 2, 3),
        border_color: Color::rgb(4, 5, 6),
        ..VisualStyle::default()
    });
    let b = render_css(&p(), &css);
    assert_eq!(b.get(15, 8).unwrap().symbol, '│');
    assert_eq!(b.get(15, 8).unwrap().fg, Some(Color::rgb(4, 5, 6)));
    assert_eq!(b.get(15, 8).unwrap().bg, Some(Color::rgb(1, 2, 3)));
    assert_eq!(b.get(17, 8).unwrap().fg, Some(Color::rgb(7, 8, 9)));
    assert_eq!(b.get(17, 8).unwrap().bg, Some(Color::rgb(1, 2, 3)));
    // The arrow takes the border color
    assert_eq!(b.get(20, 6).unwrap().symbol, '^');
    assert_eq!(b.get(20, 6).unwrap().fg, Some(Color::rgb(4, 5, 6)));

    // Without a border color the border takes the text color
    let text_only = with_visual(VisualStyle {
        color: Color::rgb(7, 8, 9),
        ..VisualStyle::default()
    });
    let b = render_css(&p(), &text_only);
    assert_eq!(b.get(15, 8).unwrap().fg, Some(Color::rgb(7, 8, 9)));
    assert_eq!(b.get(15, 8).unwrap().bg, Some(Color::rgb(30, 30, 35)));

    // The builder's colors win over the stylesheet's
    let built = p()
        .fg(Color::RED)
        .bg(Color::BLUE)
        .border_color(Color::GREEN);
    let b = render_css(&built, &css);
    assert_eq!(b.get(15, 8).unwrap().fg, Some(Color::GREEN));
    assert_eq!(b.get(17, 8).unwrap().fg, Some(Color::RED));
    assert_eq!(b.get(17, 8).unwrap().bg, Some(Color::BLUE));
    assert_eq!(b.get(20, 6).unwrap().fg, Some(Color::GREEN));
}

#[test]
fn test_popover_style_palettes() {
    // (style, background, border)
    let cases = [
        (
            PopoverStyle::Default,
            Color::rgb(30, 30, 35),
            Color::rgb(70, 70, 80),
        ),
        (
            PopoverStyle::Rounded,
            Color::rgb(35, 35, 40),
            Color::rgb(80, 80, 90),
        ),
        (
            PopoverStyle::Elevated,
            Color::rgb(25, 25, 30),
            Color::rgb(60, 60, 70),
        ),
    ];
    for (style, bg, border) in cases {
        let p = Popover::new("Test")
            .anchor(20, 5)
            .popover_style(style)
            .open(true);
        let b = render(&p);
        assert_eq!(b.get(15, 7).unwrap().fg, Some(border), "{style:?}");
        assert_eq!(b.get(16, 7).unwrap().bg, Some(bg), "{style:?}");
        assert_eq!(b.get(17, 7).unwrap().fg, Some(Color::WHITE), "{style:?}");
    }
    // Minimal has no border: the whole 10 x 3 box is its background
    let p = Popover::new("Test")
        .anchor(20, 5)
        .popover_style(PopoverStyle::Minimal)
        .open(true);
    let b = render(&p);
    for y in 6..9 {
        for x in 15..25 {
            assert_eq!(b.get(x, y).unwrap().bg, Some(Color::rgb(40, 40, 45)));
        }
    }
    assert_eq!(b.get(14, 7).unwrap().bg, None);
    assert_eq!(b.get(25, 7).unwrap().bg, None);
}

#[test]
fn test_popover_elevated_shadow_is_one_column_and_one_row() {
    let shadow = Some(Color::rgb(15, 15, 15));
    let p = Popover::new("Test")
        .anchor(20, 5)
        .popover_style(PopoverStyle::Elevated)
        .open(true);
    let b = render(&p);
    // The box is (15, 6) to (24, 8)
    for y in 0..20 {
        for x in 0..40 {
            let expected = (x == 25 && (7..=9).contains(&y)) || (y == 9 && (16..=25).contains(&x));
            assert_eq!(b.get(x, y).unwrap().bg == shadow, expected, "({x}, {y})");
        }
    }

    // In the bottom-right corner there is no room for it
    let cornered = Popover::new("Test")
        .anchor(39, 19)
        .popover_style(PopoverStyle::Elevated)
        .open(true);
    let b = render(&cornered);
    assert_eq!(corner(&b), Some((30, 17)));
    assert!((0..20).all(|y| (0..40).all(|x| b.get(x, y).unwrap().bg != shadow)));
}

/// The symbols of row `y` in `xs`
fn span(b: &Buffer, y: u16, xs: std::ops::Range<u16>) -> String {
    xs.map(|x| symbol(b, x, y)).collect()
}

#[test]
fn test_popover_empty_content_draws_the_smallest_box() {
    let b = render(&Popover::new("").anchor(20, 5).open(true));
    assert_eq!(span(&b, 6, 15..25), "┌────────┐");
    assert_eq!(span(&b, 7, 15..25), "│        │");
    assert_eq!(span(&b, 8, 15..25), "└────────┘");
    assert!(row(&b, 9).trim().is_empty());
}

#[test]
fn test_popover_keeps_blank_lines() {
    let b = render(&Popover::new("word\n\nafter blank").anchor(20, 5).open(true));
    let (x, y) = corner(&b).unwrap();
    let inner = |dy: u16| span(&b, y + dy, x..x + 15);
    assert_eq!(inner(1), "│ word        │");
    assert_eq!(inner(2), "│             │");
    assert_eq!(inner(3), "│ after blank │");
    assert_eq!(inner(4), "└─────────────┘");
}

#[test]
fn test_popover_max_width_zero_wraps_at_forty() {
    let text = "one two three four five six seven eight nine ten eleven twelve";
    let draw = |max_width| {
        let p = Popover::new(text)
            .max_width(max_width)
            .anchor(30, 2)
            .open(true);
        let mut buffer = Buffer::new(60, 10);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 60, 10));
        p.render(&mut ctx);
        buffer
    };
    let (auto, forty) = (draw(0), draw(40));
    for y in 0..10 {
        assert_eq!(row(&auto, y), row(&forty, y), "row {y}");
    }
    // "one ... eight" is 39 columns; "nine" would make 44
    assert_eq!(corner(&auto), Some((9, 3)));
    assert_eq!(
        row(&auto, 4).trim(),
        "│ one two three four five six seven eight │"
    );
    assert_eq!(
        row(&auto, 5).trim(),
        "│ nine ten eleven twelve                  │"
    );
    assert_eq!(symbol(&auto, 9, 6), '└');
}

#[test]
fn test_popover_long_title_sets_the_width() {
    let p = Popover::new("Hi")
        .title("A rather long title")
        .anchor(20, 5)
        .open(true);
    let b = render(&p);
    // Title 19 + 2, plus border and padding 4 = 25
    let (x, y) = corner(&b).unwrap();
    assert_eq!((x, y), (8, 6));
    assert_eq!(symbol(&b, x + 24, y), '┐');
    assert_eq!(row(&b, y + 1).trim(), "│ A rather long title   │");
    assert!(b
        .get(x + 2, y + 1)
        .unwrap()
        .modifier
        .contains(Modifier::BOLD));
    assert!(!b
        .get(x + 2, y + 2)
        .unwrap()
        .modifier
        .contains(Modifier::BOLD));
    assert_eq!(row(&b, y + 2).trim(), "│ Hi                    │");
}

#[test]
fn test_popover_slides_along_the_edge_it_is_against() {
    // Too close to the left edge to center: the box starts at column 0, the
    // arrow stays at the anchor
    let left = Popover::new("Test")
        .anchor(2, 5)
        .arrow(PopoverArrow::Simple)
        .open(true);
    let b = render(&left);
    assert_eq!(corner(&b), Some((0, 7)));
    assert_eq!(symbol(&b, 2, 6), '^');
    // and at the right edge the box ends at the last column
    let right = Popover::new("Test")
        .anchor(38, 5)
        .arrow(PopoverArrow::Simple)
        .open(true);
    let b = render(&right);
    assert_eq!(corner(&b), Some((30, 7)));
    assert_eq!(symbol(&b, 39, 7), '┐');
    assert_eq!(symbol(&b, 38, 6), '^');
}

#[test]
fn test_popover_auto_goes_below_when_it_just_fits() {
    // Rows 17 to 19 are free below the anchor: the 3-row box just fits
    let p = Popover::new("Test")
        .anchor(20, 16)
        .position(PopoverPosition::Auto)
        .open(true);
    assert_eq!(corner(&render(&p)), Some((15, 17)));
    // With an arrow it needs one row more
    let p = Popover::new("Test")
        .anchor(20, 15)
        .arrow(PopoverArrow::Simple)
        .position(PopoverPosition::Auto)
        .open(true);
    let b = render(&p);
    assert_eq!(corner(&b), Some((15, 17)));
    assert_eq!(symbol(&b, 20, 16), '^');
    // One row less and it goes above
    let p = Popover::new("Test")
        .anchor(20, 17)
        .position(PopoverPosition::Auto)
        .open(true);
    assert_eq!(corner(&render(&p)), Some((15, 14)));
}

#[test]
fn a_word_longer_than_max_width_is_broken_by_columns() {
    let p = Popover::new("the supercalifragilistic word")
        .max_width(10)
        .position(PopoverPosition::Bottom)
        .anchor(10, 0)
        .open(true);
    let mut buffer = Buffer::new(30, 8);
    p.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, 30, 8)));
    let (x, y) = corner(&buffer).unwrap();
    let line = |row: u16| -> String { (x..x + 14).map(|cx| symbol(&buffer, cx, row)).collect() };
    assert_eq!(line(y), "┌────────────┐");
    assert_eq!(line(y + 1), "│ the        │");
    assert_eq!(line(y + 2), "│ supercalif │");
    assert_eq!(line(y + 3), "│ ragilistic │");
    assert_eq!(line(y + 4), "│ word       │");
    assert_eq!(line(y + 5), "└────────────┘");
}

#[test]
fn the_content_wraps_to_a_narrow_area_and_the_box_keeps_its_right_border() {
    let p = Popover::new("one two three four five six seven")
        .anchor(5, 0)
        .position(PopoverPosition::Bottom)
        .open(true);
    let mut buffer = Buffer::new(20, 8);
    p.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 8)));
    // Wrapped to the 16 columns the area leaves; the box takes the widest
    // line, 13 columns
    assert_eq!(row(&buffer, 1), "┌───────────────┐   ");
    assert_eq!(row(&buffer, 2), "│ one two three │   ");
    assert_eq!(row(&buffer, 3), "│ four five six │   ");
    assert_eq!(row(&buffer, 4), "│ seven         │   ");
    assert_eq!(row(&buffer, 5), "└───────────────┘   ");
}
