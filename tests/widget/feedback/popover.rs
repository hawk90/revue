//! Popover widget tests
//!
//! Popover's fields are private and it has no getters, so its state is read
//! through is_open(), the key and click handlers, and what it renders.

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::{
    popover, Popover, PopoverArrow, PopoverPosition, PopoverStyle, PopoverTrigger, RenderContext,
    View,
};

fn render(p: &Popover) -> Buffer {
    let mut buffer = Buffer::new(40, 20);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 20));
    p.render(&mut ctx);
    buffer
}

fn symbol(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).map(|c| c.symbol).unwrap_or(' ')
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width()).map(|x| symbol(buffer, x, y)).collect()
}

fn is_blank(buffer: &Buffer) -> bool {
    (0..buffer.height()).all(|y| row(buffer, y).trim().is_empty())
}

/// Top-left corner of the popover's border.
fn corner(buffer: &Buffer) -> Option<(u16, u16)> {
    (0..buffer.height()).find_map(|y| {
        (0..buffer.width())
            .find(|&x| matches!(symbol(buffer, x, y), '┌' | '╭'))
            .map(|x| (x, y))
    })
}

// =========================================================================
// Enums
// =========================================================================

#[test]
fn test_popover_enum_defaults() {
    assert_eq!(PopoverPosition::default(), PopoverPosition::Bottom);
    assert_eq!(PopoverTrigger::default(), PopoverTrigger::Click);
    assert_eq!(PopoverArrow::default(), PopoverArrow::None);
    assert_eq!(PopoverStyle::default(), PopoverStyle::Default);
}

#[test]
fn test_popover_enum_variants_are_distinct() {
    let positions = [
        PopoverPosition::Top,
        PopoverPosition::Bottom,
        PopoverPosition::Left,
        PopoverPosition::Right,
        PopoverPosition::Auto,
    ];
    let triggers = [
        PopoverTrigger::Click,
        PopoverTrigger::Hover,
        PopoverTrigger::Focus,
        PopoverTrigger::Manual,
    ];
    let arrows = [
        PopoverArrow::None,
        PopoverArrow::Simple,
        PopoverArrow::Unicode,
    ];
    let styles = [
        PopoverStyle::Default,
        PopoverStyle::Rounded,
        PopoverStyle::Minimal,
        PopoverStyle::Elevated,
    ];
    for (i, a) in positions.iter().enumerate() {
        for (j, b) in positions.iter().enumerate() {
            assert_eq!(i == j, a == b);
        }
    }
    for (i, a) in triggers.iter().enumerate() {
        for (j, b) in triggers.iter().enumerate() {
            assert_eq!(i == j, a == b);
        }
    }
    for (i, a) in arrows.iter().enumerate() {
        for (j, b) in arrows.iter().enumerate() {
            assert_eq!(i == j, a == b);
        }
    }
    for (i, a) in styles.iter().enumerate() {
        for (j, b) in styles.iter().enumerate() {
            assert_eq!(i == j, a == b);
        }
    }
}

// =========================================================================
// Open state
// =========================================================================

#[test]
fn test_popover_starts_closed_and_draws_nothing() {
    for p in [Popover::new("Test").anchor(10, 5), Popover::default()] {
        assert!(!p.is_open());
        assert!(is_blank(&render(&p)));
    }
}

#[test]
fn test_popover_visibility() {
    let mut p = Popover::new("Test");
    p.show();
    assert!(p.is_open());
    p.hide();
    assert!(!p.is_open());
    p.toggle();
    assert!(p.is_open());
    p.toggle();
    assert!(!p.is_open());
    assert!(Popover::new("Test").open(true).is_open());
}

#[test]
fn test_popover_handle_escape() {
    let mut p = Popover::new("Test").open(true);
    assert!(p.handle_key(&Key::Escape));
    assert!(!p.is_open());

    let mut other = Popover::new("Test").open(true);
    assert!(!other.handle_key(&Key::Enter));
    assert!(other.is_open());
}

#[test]
fn test_popover_handle_escape_disabled() {
    let mut p = Popover::new("Test").open(true).close_on_escape(false);
    assert!(!p.handle_key(&Key::Escape));
    assert!(p.is_open());
}

#[test]
fn test_popover_handle_key_closed() {
    let mut p = Popover::new("Test");
    assert!(!p.handle_key(&Key::Escape));
    assert!(!p.is_open());
}

#[test]
fn test_popover_handle_click_inside_keeps_open() {
    // "Test" below (20, 10): a 10 x 3 box at (15, 11).
    let mut p = Popover::new("Test").anchor(20, 10).open(true);
    assert!(p.handle_click(15, 11, 40, 20));
    assert!(p.handle_click(24, 13, 40, 20));
    assert!(p.is_open());
}

#[test]
fn test_popover_handle_click_outside_closes() {
    let mut p = Popover::new("Test").anchor(20, 10).open(true);
    assert!(p.handle_click(25, 11, 40, 20));
    assert!(!p.is_open());
}

#[test]
fn test_popover_handle_click_outside_disabled() {
    let mut p = Popover::new("Test")
        .anchor(20, 10)
        .open(true)
        .close_on_click_outside(false);
    assert!(!p.handle_click(0, 0, 40, 20));
    assert!(p.is_open());
}

#[test]
fn test_popover_click_trigger_opens_on_anchor() {
    let mut p = Popover::new("Test")
        .anchor(10, 5)
        .trigger(PopoverTrigger::Click);
    assert!(!p.handle_click(11, 5, 40, 20));
    assert!(!p.is_open());
    assert!(p.handle_click(10, 5, 40, 20));
    assert!(p.is_open());
}

#[test]
fn test_popover_other_triggers_ignore_anchor_click() {
    for trigger in [
        PopoverTrigger::Hover,
        PopoverTrigger::Focus,
        PopoverTrigger::Manual,
    ] {
        let mut p = Popover::new("Test").anchor(10, 5).trigger(trigger);
        assert!(!p.handle_click(10, 5, 40, 20), "{trigger:?}");
        assert!(!p.is_open(), "{trigger:?}");
    }
}

// =========================================================================
// Content and placement
// =========================================================================

#[test]
fn test_popover_helper_and_content() {
    let buffer = render(&popover("Quick").anchor(20, 5).open(true));
    assert_eq!(
        &row(&buffer, 7)[..],
        format!("{:15}│ Quick  │{:15}", "", "")
    );

    let replaced = Popover::new("Original")
        .content("Updated")
        .anchor(20, 5)
        .open(true);
    assert!(row(&render(&replaced), 7).contains("│ Updated │"));
}

#[test]
fn test_popover_positions() {
    // "Test" is 10 x 3; anchor (20, 5).
    let cases = [
        (PopoverPosition::Bottom, (15, 6)),
        (PopoverPosition::Top, (15, 2)),
        (PopoverPosition::Left, (10, 4)),
        (PopoverPosition::Right, (21, 4)),
        (PopoverPosition::Auto, (15, 6)),
    ];
    for (position, expected) in cases {
        let p = Popover::new("Test")
            .anchor(20, 5)
            .position(position)
            .open(true);
        assert_eq!(corner(&render(&p)), Some(expected), "{position:?}");
    }
}

#[test]
fn test_popover_auto_goes_above_without_room_below() {
    let p = Popover::new("Test")
        .anchor(20, 18)
        .position(PopoverPosition::Auto)
        .open(true);
    assert_eq!(corner(&render(&p)), Some((15, 15)));
}

#[test]
fn test_popover_set_anchor() {
    let mut p = Popover::new("Test").anchor(20, 5).open(true);
    p.set_anchor(10, 10);
    assert_eq!(corner(&render(&p)), Some((5, 11)));
}

#[test]
fn test_popover_arrow_points_at_anchor() {
    let cases = [
        (PopoverArrow::Unicode, PopoverPosition::Top, (20, 9), '▼'),
        (
            PopoverArrow::Unicode,
            PopoverPosition::Bottom,
            (20, 11),
            '▲',
        ),
        (PopoverArrow::Unicode, PopoverPosition::Left, (19, 10), '▶'),
        (PopoverArrow::Unicode, PopoverPosition::Right, (21, 10), '◀'),
        (PopoverArrow::Simple, PopoverPosition::Top, (20, 9), 'v'),
        (PopoverArrow::Simple, PopoverPosition::Bottom, (20, 11), '^'),
        (PopoverArrow::Simple, PopoverPosition::Left, (19, 10), '>'),
        (PopoverArrow::Simple, PopoverPosition::Right, (21, 10), '<'),
    ];
    for (arrow, position, (x, y), glyph) in cases {
        let p = Popover::new("Test")
            .anchor(20, 10)
            .arrow(arrow)
            .position(position)
            .open(true);
        assert_eq!(
            symbol(&render(&p), x, y),
            glyph,
            "{arrow:?} at {position:?}"
        );
    }
}

#[test]
fn test_popover_title() {
    let p = Popover::new("Content")
        .title("Title")
        .anchor(20, 5)
        .open(true);
    let buffer = render(&p);
    let (x, y) = corner(&buffer).unwrap();
    assert_eq!(symbol(&buffer, x + 2, y + 1), 'T');
    assert!(buffer
        .get(x + 2, y + 1)
        .unwrap()
        .modifier
        .contains(revue::render::Modifier::BOLD));
    assert!(row(&buffer, y + 2).contains("│ Content"));
    assert_eq!(symbol(&buffer, x, y + 3), '└');
}

#[test]
fn test_popover_max_width_wraps_content() {
    let p = Popover::new("aaa bbb ccc")
        .max_width(5)
        .anchor(20, 5)
        .open(true);
    let buffer = render(&p);
    let (x, y) = corner(&buffer).unwrap();
    for (i, word) in ["aaa", "bbb", "ccc"].iter().enumerate() {
        let line: String = (x + 2..x + 5)
            .map(|cx| symbol(&buffer, cx, y + 1 + i as u16))
            .collect();
        assert_eq!(&line, word);
    }
}

#[test]
fn test_popover_styles() {
    let draw = |style| {
        render(
            &Popover::new("Test")
                .anchor(20, 5)
                .popover_style(style)
                .open(true),
        )
    };

    assert_eq!(symbol(&draw(PopoverStyle::Default), 15, 6), '┌');
    assert_eq!(symbol(&draw(PopoverStyle::Rounded), 15, 6), '╭');
    assert_eq!(symbol(&draw(PopoverStyle::Elevated), 15, 6), '┌');

    // Minimal: no border, content one cell in from the left edge.
    let minimal = draw(PopoverStyle::Minimal);
    assert!(corner(&minimal).is_none());
    assert_eq!(row(&minimal, 6).trim(), "Test");
    assert_eq!(symbol(&minimal, 16, 6), 'T');
}

#[test]
fn test_popover_minimal_draws_every_line() {
    let p = Popover::new("a\nb\nc")
        .anchor(20, 5)
        .popover_style(PopoverStyle::Minimal)
        .open(true);
    let buffer = render(&p);
    let lines: Vec<String> = (6..9).map(|y| row(&buffer, y).trim().to_string()).collect();
    assert_eq!(lines, ["a", "b", "c"]);
}

#[test]
fn test_popover_elevated_shadow() {
    let p = Popover::new("Test")
        .anchor(20, 5)
        .popover_style(PopoverStyle::Elevated)
        .open(true);
    let buffer = render(&p);
    // Box is (15, 6) to (24, 8); the shadow is one cell right and below.
    let shadow = Some(Color::rgb(15, 15, 15));
    assert_eq!(buffer.get(25, 7).unwrap().bg, shadow);
    assert_eq!(buffer.get(16, 9).unwrap().bg, shadow);
    assert_ne!(buffer.get(24, 8).unwrap().bg, shadow);
}

#[test]
fn test_popover_default_colors() {
    let buffer = render(&Popover::new("Test").anchor(20, 5).open(true));
    let border = buffer.get(15, 6).unwrap();
    assert_eq!(border.fg, Some(Color::rgb(70, 70, 80)));
    assert_eq!(border.bg, Some(Color::rgb(30, 30, 35)));
    let text = buffer.get(17, 7).unwrap();
    assert_eq!(text.symbol, 'T');
    assert_eq!(text.fg, Some(Color::WHITE));
}

#[test]
fn test_popover_custom_colors() {
    let p = Popover::new("Test")
        .fg(Color::RED)
        .bg(Color::BLUE)
        .border_color(Color::GREEN)
        .anchor(20, 5)
        .open(true);
    let buffer = render(&p);
    assert_eq!(buffer.get(15, 6).unwrap().fg, Some(Color::GREEN));
    let text = buffer.get(17, 7).unwrap();
    assert_eq!(text.fg, Some(Color::RED));
    assert_eq!(text.bg, Some(Color::BLUE));
}

#[test]
fn test_popover_builder_chain() {
    let p = Popover::new("Chain test")
        .anchor(10, 10)
        .position(PopoverPosition::Right)
        .trigger(PopoverTrigger::Hover)
        .popover_style(PopoverStyle::Rounded)
        .arrow(PopoverArrow::Unicode)
        .title("Chain Title")
        .max_width(60)
        .open(true);
    let buffer = render(&p);
    // Right of the anchor, past the arrow.
    let (x, _) = corner(&buffer).unwrap();
    assert_eq!(x, 12);
    assert_eq!(symbol(&buffer, 11, 10), '◀');
    assert_eq!(symbol(&buffer, x, corner(&buffer).unwrap().1), '╭');
}

// Found by tests/event_sequences.rs: the shrunk sequence was `<anchor far>`,
// an anchor at (u16::MAX, u16::MAX).
#[test]
fn test_anchor_at_the_far_corner_does_not_overflow() {
    for position in [
        PopoverPosition::Auto,
        PopoverPosition::Top,
        PopoverPosition::Bottom,
        PopoverPosition::Left,
        PopoverPosition::Right,
    ] {
        let mut p = Popover::new("content").position(position).open(true);
        p.set_anchor(u16::MAX, u16::MAX);
        render(&p);
    }
}
