//! Tooltip state read back through its getters, and the arrow and border
//! glyphs it draws
//!
//! tests/widget/tooltip.rs covers show/hide/toggle, the delay countdown and
//! renders every style, arrow and position (mostly checking that something
//! is drawn).

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::{
    tooltip, RenderContext, Tooltip, TooltipArrow, TooltipPosition, TooltipStyle, View,
};

fn render(t: &Tooltip) -> Buffer {
    let mut buffer = Buffer::new(40, 20);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 20));
    t.render(&mut ctx);
    buffer
}

fn symbol(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).map(|c| c.symbol).unwrap_or(' ')
}

// =========================================================================
// Enums
// =========================================================================

#[test]
fn test_tooltip_enum_defaults() {
    assert_eq!(TooltipPosition::default(), TooltipPosition::Top);
    assert_eq!(TooltipArrow::default(), TooltipArrow::None);
    assert_eq!(TooltipStyle::default(), TooltipStyle::Plain);
}

#[test]
fn test_tooltip_enum_variants_are_distinct() {
    let positions = [
        TooltipPosition::Top,
        TooltipPosition::Bottom,
        TooltipPosition::Left,
        TooltipPosition::Right,
        TooltipPosition::Auto,
    ];
    let styles = [
        TooltipStyle::Plain,
        TooltipStyle::Bordered,
        TooltipStyle::Rounded,
        TooltipStyle::Info,
        TooltipStyle::Warning,
        TooltipStyle::Error,
        TooltipStyle::Success,
    ];
    let arrows = [
        TooltipArrow::None,
        TooltipArrow::Simple,
        TooltipArrow::Unicode,
    ];
    for (i, a) in positions.iter().enumerate() {
        for (j, b) in positions.iter().enumerate() {
            assert_eq!(i == j, a == b);
        }
    }
    for (i, a) in styles.iter().enumerate() {
        for (j, b) in styles.iter().enumerate() {
            assert_eq!(i == j, a == b);
        }
    }
    for (i, a) in arrows.iter().enumerate() {
        for (j, b) in arrows.iter().enumerate() {
            assert_eq!(i == j, a == b);
        }
    }
}

// =========================================================================
// Builder and getters
// =========================================================================

#[test]
fn test_tooltip_new() {
    let t = Tooltip::new("Test tooltip");
    assert_eq!(t.get_text(), "Test tooltip");
    assert!(t.is_visible());
    assert_eq!(t.get_title(), None);
}

#[test]
fn test_tooltip_builder() {
    let t = Tooltip::new("Hello")
        .text("Updated")
        .position(TooltipPosition::Bottom)
        .anchor(10, 5)
        .style(TooltipStyle::Info)
        .arrow(TooltipArrow::Simple)
        .max_width(30)
        .title("My Title");
    assert_eq!(t.get_text(), "Updated");
    assert_eq!(t.get_position(), TooltipPosition::Bottom);
    assert_eq!(t.get_anchor(), (10, 5));
    assert_eq!(t.get_style(), TooltipStyle::Info);
    assert_eq!(t.get_arrow(), TooltipArrow::Simple);
    assert_eq!(t.get_max_width(), 30);
    assert_eq!(t.get_title(), Some("My Title"));
}

#[test]
fn test_tooltip_presets() {
    let cases = [
        (Tooltip::info("i"), TooltipStyle::Info),
        (Tooltip::warning("w"), TooltipStyle::Warning),
        (Tooltip::error("e"), TooltipStyle::Error),
        (Tooltip::success("s"), TooltipStyle::Success),
    ];
    for (t, style) in cases {
        assert_eq!(t.get_style(), style);
    }
    assert_eq!(Tooltip::info("Info message").get_text(), "Info message");
}

#[test]
fn test_tooltip_helper() {
    let t = tooltip("Quick tooltip");
    assert_eq!(t.get_text(), "Quick tooltip");
    assert_eq!(t.get_style(), TooltipStyle::Bordered);
}

#[test]
fn test_tooltip_delay_counter() {
    let mut t = Tooltip::new("Test").delay(5);
    assert_eq!(t.get_delay(), 5);
    assert_eq!(t.get_delay_counter(), 0);
    t.tick();
    t.tick();
    assert_eq!(t.get_delay_counter(), 2);
    for _ in 0..3 {
        t.tick();
    }
    assert_eq!(t.get_delay_counter(), 5);
    assert!(t.is_visible());
}

#[test]
fn test_tooltip_set_anchor() {
    let mut t = Tooltip::new("Test").anchor(5, 5);
    t.set_anchor(15, 20);
    assert_eq!(t.get_anchor(), (15, 20));
}

#[test]
fn test_tooltip_default() {
    let t = Tooltip::default();
    assert_eq!(t.get_text(), "");
    assert!(t.is_visible());
    assert_eq!(t.get_position(), TooltipPosition::Top);
    assert_eq!(t.get_anchor(), (0, 0));
    assert_eq!(t.get_style(), TooltipStyle::Bordered);
    assert_eq!(t.get_arrow(), TooltipArrow::Unicode);
    assert_eq!(t.get_max_width(), 40);
    assert_eq!(t.get_delay(), 0);

    let new = Tooltip::new("");
    assert_eq!(new.get_style(), t.get_style());
    assert_eq!(new.get_arrow(), t.get_arrow());
    assert_eq!(new.get_max_width(), t.get_max_width());
}

// =========================================================================
// Drawn glyphs
// =========================================================================

#[test]
fn test_tooltip_arrow_points_at_anchor() {
    // The arrow sits in the cell between the anchor and the tooltip.
    let cases = [
        (TooltipArrow::Unicode, TooltipPosition::Top, (20, 9), '▼'),
        (
            TooltipArrow::Unicode,
            TooltipPosition::Bottom,
            (20, 11),
            '▲',
        ),
        (TooltipArrow::Unicode, TooltipPosition::Left, (19, 10), '▶'),
        (TooltipArrow::Unicode, TooltipPosition::Right, (21, 10), '◀'),
        (TooltipArrow::Simple, TooltipPosition::Top, (20, 9), 'v'),
        (TooltipArrow::Simple, TooltipPosition::Bottom, (20, 11), '^'),
        (TooltipArrow::Simple, TooltipPosition::Left, (19, 10), '>'),
        (TooltipArrow::Simple, TooltipPosition::Right, (21, 10), '<'),
    ];
    for (arrow, position, (x, y), glyph) in cases {
        let t = Tooltip::new("Hi")
            .style(TooltipStyle::Plain)
            .anchor(20, 10)
            .arrow(arrow)
            .position(position);
        assert_eq!(
            symbol(&render(&t), x, y),
            glyph,
            "{arrow:?} at {position:?}"
        );
    }
}

#[test]
fn test_tooltip_without_arrow_leaves_gap_empty() {
    let t = Tooltip::new("Hi")
        .style(TooltipStyle::Plain)
        .anchor(20, 10)
        .arrow(TooltipArrow::None)
        .position(TooltipPosition::Bottom);
    let buffer = render(&t);
    // With no arrow the tooltip starts right below the anchor.
    assert_eq!(symbol(&buffer, 20, 10), ' ');
    assert_eq!(
        (symbol(&buffer, 19, 11), symbol(&buffer, 20, 11)),
        ('H', 'i')
    );
}

#[test]
fn test_tooltip_plain_draws_every_line() {
    let t = Tooltip::new("ab\ncd")
        .style(TooltipStyle::Plain)
        .arrow(TooltipArrow::None)
        .position(TooltipPosition::Bottom)
        .anchor(3, 0);
    // 2 + 2 padding wide, centered under x = 3: the box is x 1..5, text at 2.
    let buffer = render(&t);
    assert_eq!((symbol(&buffer, 2, 1), symbol(&buffer, 3, 1)), ('a', 'b'));
    assert_eq!((symbol(&buffer, 2, 2), symbol(&buffer, 3, 2)), ('c', 'd'));
}

#[test]
fn test_tooltip_border_glyphs_by_style() {
    let top_left = |style| {
        let t = Tooltip::new("Hi")
            .style(style)
            .arrow(TooltipArrow::None)
            .position(TooltipPosition::Bottom)
            .anchor(10, 0);
        // Bottom of the anchor, centered on it: the box starts at row 1.
        let buffer = render(&t);
        let x = (0..40).find(|&x| symbol(&buffer, x, 1) != ' ').unwrap();
        symbol(&buffer, x, 1)
    };
    assert_eq!(top_left(TooltipStyle::Plain), 'H');
    assert_eq!(top_left(TooltipStyle::Bordered), '┌');
    assert_eq!(top_left(TooltipStyle::Info), '┌');
    assert_eq!(top_left(TooltipStyle::Rounded), '╭');
}
