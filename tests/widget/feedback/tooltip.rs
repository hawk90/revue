//! Tooltip widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::tooltip;
use revue::widget::RenderContext;
use revue::widget::StyledView;
use revue::widget::Tooltip;
use revue::widget::TooltipArrow;
use revue::widget::TooltipPosition;
use revue::widget::TooltipStyle;
use revue::widget::View;

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

    let t = tooltip("Quick tooltip");
    assert!(t.is_visible());
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

    let mut t = Tooltip::new("Test").anchor(5, 5);

    t.set_anchor(10, 15);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
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

// =============================================================================
// Visibility and State Tests
// =============================================================================

#[test]
fn test_tooltip_visibility() {
    let mut t = Tooltip::new("Test");

    t.hide();
    assert!(!t.is_visible());

    t.show();
    assert!(t.is_visible());

    t.toggle();
    assert!(!t.is_visible());
}

#[test]
fn test_tooltip_delay() {
    let mut t = Tooltip::new("Test").delay(5);
    assert!(!t.is_visible());

    for _ in 0..4 {
        t.tick();
    }
    assert!(!t.is_visible());

    t.tick();
    assert!(t.is_visible());
}

#[test]
fn test_tooltip_delay_with_visibility() {
    let mut t = Tooltip::new("Test").delay(3).visible(false);
    assert!(!t.is_visible());

    t.show();
    assert!(!t.is_visible());

    for _ in 0..3 {
        t.tick();
    }
    assert!(t.is_visible());
}

#[test]
fn test_tooltip_reset_delay_on_show() {
    let mut t = Tooltip::new("Test").delay(5);
    t.tick();
    t.tick();
    assert!(!t.is_visible());

    t.show();
    assert!(!t.is_visible());

    for _ in 0..5 {
        t.tick();
    }
    assert!(t.is_visible());
}

// =============================================================================
// Builder Methods Tests
// =============================================================================

#[test]
fn test_tooltip_builder_text() {
    let t = Tooltip::new("Initial").text("Updated text");
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_builder_position() {
    let t = Tooltip::new("Test").position(TooltipPosition::Bottom);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_builder_anchor() {
    let t = Tooltip::new("Test").anchor(10, 5);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_builder_style() {
    let t = Tooltip::new("Test").style(TooltipStyle::Info);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_builder_arrow() {
    let t = Tooltip::new("Test").arrow(TooltipArrow::Unicode);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_builder_max_width() {
    let t = Tooltip::new("Test").max_width(30);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_builder_visible() {
    let t = Tooltip::new("Test").visible(false);
    assert!(!t.is_visible());
}

#[test]
fn test_tooltip_builder_fg() {
    let t = Tooltip::new("Test").fg(Color::RED);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_builder_bg() {
    let t = Tooltip::new("Test").bg(Color::BLUE);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_builder_title() {
    let t = Tooltip::new("Test").title("My Title");
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_builder_delay() {
    let t = Tooltip::new("Test").delay(10);
    assert!(!t.is_visible());
}

#[test]
fn test_tooltip_builder_chain() {
    let t = Tooltip::new("Hello")
        .position(TooltipPosition::Bottom)
        .anchor(10, 5)
        .style(TooltipStyle::Info)
        .arrow(TooltipArrow::Unicode)
        .max_width(30)
        .visible(false)
        .fg(Color::RED)
        .bg(Color::BLUE)
        .title("Title")
        .delay(5);

    assert!(!t.is_visible());

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

// =============================================================================
// Preset Constructors Tests
// =============================================================================

#[test]
fn test_tooltip_new_plain() {
    let t = Tooltip::new("Plain tooltip");
    assert!(t.is_visible());
}

#[test]
fn test_tooltip_info_preset() {
    let info = Tooltip::info("Info message");
    assert!(info.is_visible());
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    info.render(&mut ctx);
}

#[test]
fn test_tooltip_warning_preset() {
    let warning = Tooltip::warning("Warning!");
    assert!(warning.is_visible());
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    warning.render(&mut ctx);
}

#[test]
fn test_tooltip_error_preset() {
    let error = Tooltip::error("Error!");
    assert!(error.is_visible());
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    error.render(&mut ctx);
}

#[test]
fn test_tooltip_success_preset() {
    let success = Tooltip::success("Success!");
    assert!(success.is_visible());
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    success.render(&mut ctx);
}

// =============================================================================
// Position Calculation Tests
// =============================================================================

#[test]
fn test_tooltip_calculate_position_top() {
    let t = Tooltip::new("Test")
        .position(TooltipPosition::Top)
        .anchor(20, 10)
        .style(TooltipStyle::Bordered);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_calculate_position_bottom() {
    let t = Tooltip::new("Test")
        .position(TooltipPosition::Bottom)
        .anchor(20, 10)
        .style(TooltipStyle::Bordered);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_calculate_position_left() {
    let t = Tooltip::new("Test")
        .position(TooltipPosition::Left)
        .anchor(20, 10)
        .style(TooltipStyle::Bordered);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_calculate_position_right() {
    let t = Tooltip::new("Test")
        .position(TooltipPosition::Right)
        .anchor(20, 10)
        .style(TooltipStyle::Bordered);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_calculate_position_auto() {
    let t = Tooltip::new("Test")
        .position(TooltipPosition::Auto)
        .anchor(20, 10)
        .style(TooltipStyle::Bordered);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_calculate_position_edge_top_left() {
    let t = Tooltip::new("Test")
        .position(TooltipPosition::Top)
        .anchor(0, 0)
        .style(TooltipStyle::Bordered);

    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_calculate_position_edge_bottom_right() {
    let t = Tooltip::new("Test")
        .position(TooltipPosition::Bottom)
        .anchor(19, 19)
        .style(TooltipStyle::Bordered);

    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

// =============================================================================
// Text Wrapping Tests
// =============================================================================

#[test]
fn test_tooltip_wrap_text_short() {
    let t = Tooltip::new("Short text").max_width(20);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_wrap_text_long() {
    let t = Tooltip::new("This is a very long text that should be wrapped").max_width(20);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_wrap_text_multiple_lines() {
    let t = Tooltip::new("Line 1\nLine 2\nLine 3").max_width(20);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_wrap_text_empty() {
    let t = Tooltip::new("").max_width(20);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_wrap_text_zero_max_width() {
    let t = Tooltip::new("Test").max_width(0);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

// =============================================================================
// Dimension Calculation Tests
// =============================================================================

#[test]
fn test_tooltip_calculate_dimensions_plain() {
    let t = Tooltip::new("Short").style(TooltipStyle::Plain);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_calculate_dimensions_bordered() {
    let t = Tooltip::new("Short").style(TooltipStyle::Bordered);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_calculate_dimensions_with_title() {
    let t = Tooltip::new("Content")
        .title("Title")
        .style(TooltipStyle::Bordered);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_calculate_dimensions_long_text() {
    let t = Tooltip::new("This is a longer text").style(TooltipStyle::Bordered);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_calculate_dimensions_multiline() {
    let t = Tooltip::new("Line 1\nLine 2\nLine 3").style(TooltipStyle::Bordered);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

// =============================================================================
// Render Tests - Styles and Arrows
// =============================================================================

#[test]
fn test_tooltip_render_all_styles() {
    let styles = [
        TooltipStyle::Plain,
        TooltipStyle::Bordered,
        TooltipStyle::Rounded,
        TooltipStyle::Info,
        TooltipStyle::Warning,
        TooltipStyle::Error,
        TooltipStyle::Success,
    ];

    for style in styles {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tooltip::new("Test")
            .anchor(20, 10)
            .style(style)
            .arrow(TooltipArrow::None);

        t.render(&mut ctx);
    }
}

#[test]
fn test_tooltip_render_all_arrow_types() {
    let arrows = [
        TooltipArrow::None,
        TooltipArrow::Simple,
        TooltipArrow::Unicode,
    ];

    for arrow in arrows {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tooltip::new("Test")
            .anchor(20, 10)
            .arrow(arrow)
            .style(TooltipStyle::Bordered);

        t.render(&mut ctx);
    }
}

#[test]
fn test_tooltip_render_plain() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tooltip::new("Hello World")
        .anchor(20, 10)
        .position(TooltipPosition::Top)
        .style(TooltipStyle::Plain);

    t.render(&mut ctx);
}

#[test]
fn test_tooltip_render_bordered() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tooltip::new("Hello World")
        .anchor(20, 10)
        .position(TooltipPosition::Top)
        .style(TooltipStyle::Bordered);

    t.render(&mut ctx);
}

#[test]
fn test_tooltip_render_rounded() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tooltip::new("Hello")
        .anchor(20, 10)
        .style(TooltipStyle::Rounded);

    t.render(&mut ctx);
}

#[test]
fn test_tooltip_render_with_title() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tooltip::new("Content")
        .title("Title")
        .anchor(20, 10)
        .style(TooltipStyle::Bordered);

    t.render(&mut ctx);
}

#[test]
fn test_tooltip_render_not_visible() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tooltip::new("Hidden").visible(false);

    t.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, ' ');
}

#[test]
fn test_tooltip_render_with_delay() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tooltip::new("Delayed").delay(5).anchor(20, 10);

    t.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, ' ');
}

#[test]
fn test_tooltip_render_after_delay() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut t = Tooltip::new("Now visible").delay(1).anchor(20, 10);
    t.tick();

    t.render(&mut ctx);
}

#[test]
fn test_tooltip_render_custom_colors() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tooltip::new("Colored")
        .anchor(20, 10)
        .fg(Color::CYAN)
        .bg(Color::RED);

    t.render(&mut ctx);
}

#[test]
fn test_tooltip_render_all_positions() {
    let positions = [
        TooltipPosition::Top,
        TooltipPosition::Bottom,
        TooltipPosition::Left,
        TooltipPosition::Right,
    ];

    for pos in positions {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tooltip::new("Test")
            .anchor(20, 10)
            .position(pos)
            .style(TooltipStyle::Bordered);

        t.render(&mut ctx);
    }
}

// =============================================================================
// Edge Cases Tests
// =============================================================================

#[test]
fn test_tooltip_empty_text() {
    let t = Tooltip::new("");
    assert!(t.is_visible());
}

#[test]
fn test_tooltip_very_long_text() {
    let long_text = "A".repeat(1000);
    let t = Tooltip::new(&long_text).max_width(50);
    let mut buffer = Buffer::new(60, 30);
    let area = Rect::new(0, 0, 60, 30);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_zero_anchor() {
    let t = Tooltip::new("Test").anchor(0, 0);
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_large_anchor() {
    let t = Tooltip::new("Test").anchor(100, 100);
    let mut buffer = Buffer::new(120, 120);
    let area = Rect::new(0, 0, 120, 120);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_very_small_area() {
    let t = Tooltip::new("Test")
        .anchor(0, 0)
        .style(TooltipStyle::Bordered);

    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_text_with_special_chars() {
    let t = Tooltip::new("Text with emoji and symbols: @#$%");
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_multiline_with_wrapping() {
    let t = Tooltip::new("Line 1 is very long and should wrap\nLine 2").max_width(20);
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_title_longer_than_content() {
    let t = Tooltip::new("Short")
        .title("This is a very long title")
        .style(TooltipStyle::Bordered);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_text_with_tabs() {
    let t = Tooltip::new("Text\twith\ttabs");
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_max_width_boundary() {
    let t = Tooltip::new("Test").max_width(100);
    let mut buffer = Buffer::new(120, 20);
    let area = Rect::new(0, 0, 120, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

#[test]
fn test_tooltip_default_trait() {
    let t: Tooltip = Default::default();
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

// =============================================================================
// Mutator Tests
// =============================================================================

#[test]
fn test_tooltip_show_hide_toggle() {
    let mut t = Tooltip::new("Test").visible(false);

    t.show();
    assert!(t.is_visible());

    t.hide();
    assert!(!t.is_visible());

    t.toggle();
    assert!(t.is_visible());

    t.toggle();
    assert!(!t.is_visible());
}

#[test]
fn test_tooltip_tick_no_delay() {
    let mut t = Tooltip::new("Test").delay(0);
    t.tick();
    assert!(t.is_visible());
}

#[test]
fn test_tooltip_tick_beyond_delay() {
    let mut t = Tooltip::new("Test").delay(5);
    for _ in 0..10 {
        t.tick();
    }
    assert!(t.is_visible());
}

// =============================================================================
// Helper Function Tests
// =============================================================================

#[test]
fn test_tooltip_helper_with_builder() {
    let t = tooltip("Test")
        .position(TooltipPosition::Bottom)
        .style(TooltipStyle::Warning);

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    t.render(&mut ctx);
}

// =============================================================================
// View Trait Tests
// =============================================================================

#[test]
fn test_tooltip_meta() {
    let t = Tooltip::new("Test").element_id("my-tooltip");
    let meta = t.meta();
    assert_eq!(meta.id, Some("my-tooltip".to_string()));
}

#[test]
fn test_tooltip_view_id() {
    let t = Tooltip::new("Test").element_id("test-id");
    assert_eq!(View::id(&t), Some("test-id"));
}

#[test]
fn test_tooltip_classes() {
    let t = Tooltip::new("Test").class("tooltip").class("large");

    assert!(t.has_class("tooltip"));
    assert!(t.has_class("large"));
    assert!(!t.has_class("small"));

    let meta = t.meta();
    assert!(meta.classes.contains("tooltip"));
    assert!(meta.classes.contains("large"));
}

#[test]
fn test_tooltip_styled_view() {
    let mut t = Tooltip::new("Test");

    t.set_id("test-id");
    assert_eq!(View::id(&t), Some("test-id"));

    t.add_class("active");
    assert!(t.has_class("active"));

    t.remove_class("active");
    assert!(!t.has_class("active"));

    t.toggle_class("selected");
    assert!(t.has_class("selected"));

    t.toggle_class("selected");
    assert!(!t.has_class("selected"));
}

// =============================================================================
// Render Buffer Verification Tests
// =============================================================================

#[test]
fn test_tooltip_render_creates_content() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tooltip::new("Test")
        .anchor(20, 10)
        .position(TooltipPosition::Top)
        .style(TooltipStyle::Bordered);

    t.render(&mut ctx);

    let mut found_content = false;
    for y in 0..20 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' {
                    found_content = true;
                    break;
                }
            }
        }
        if found_content {
            break;
        }
    }
    assert!(found_content);
}

#[test]
fn test_tooltip_render_border_visible() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tooltip::new("Test")
        .anchor(10, 10)
        .position(TooltipPosition::Top)
        .style(TooltipStyle::Bordered);

    t.render(&mut ctx);

    let mut found_border = false;
    for y in 0..20 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if matches!(cell.symbol, '─' | '│' | '╭' | '╮' | '╰' | '╯') {
                    found_border = true;
                    break;
                }
            }
        }
        if found_border {
            break;
        }
    }
    assert!(found_border);
}

#[test]
fn test_tooltip_render_title_visible() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tooltip::new("Content")
        .title("MyTitle")
        .anchor(15, 10)
        .position(TooltipPosition::Top)
        .style(TooltipStyle::Bordered);

    t.render(&mut ctx);

    let mut found_title = false;
    for y in 0..20 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == 'M' || cell.symbol == 'y' || cell.symbol == 'T' {
                    found_title = true;
                    break;
                }
            }
        }
        if found_title {
            break;
        }
    }
    assert!(found_title);
}

#[test]
fn test_tooltip_render_arrow_visible() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tooltip::new("Test")
        .anchor(20, 10)
        .position(TooltipPosition::Top)
        .arrow(TooltipArrow::Unicode)
        .style(TooltipStyle::Bordered);

    t.render(&mut ctx);

    // Verify tooltip rendered something (not all empty)
    let mut found_content = false;
    for y in 0..20 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol != ' ' && cell.symbol != '\0' {
                    found_content = true;
                    break;
                }
            }
        }
        if found_content {
            break;
        }
    }
    assert!(found_content);
}

// =============================================================================
// Combinations Tests
// =============================================================================

#[test]
fn test_tooltip_all_positions_with_all_styles() {
    let positions = [
        TooltipPosition::Top,
        TooltipPosition::Bottom,
        TooltipPosition::Left,
        TooltipPosition::Right,
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

    for pos in positions {
        for style in styles {
            let mut buffer = Buffer::new(40, 20);
            let area = Rect::new(0, 0, 40, 20);
            let mut ctx = RenderContext::new(&mut buffer, area);

            let t = Tooltip::new("Test")
                .anchor(20, 10)
                .position(pos)
                .style(style)
                .arrow(TooltipArrow::None);

            t.render(&mut ctx);
        }
    }
}

#[test]
fn test_tooltip_with_title_and_all_styles() {
    let styles = [
        TooltipStyle::Bordered,
        TooltipStyle::Rounded,
        TooltipStyle::Info,
        TooltipStyle::Warning,
        TooltipStyle::Error,
        TooltipStyle::Success,
    ];

    for style in styles {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tooltip::new("Content")
            .title("Title")
            .anchor(20, 10)
            .style(style);

        t.render(&mut ctx);
    }
}

#[test]
fn test_tooltip_custom_colors_with_all_styles() {
    let styles = [
        TooltipStyle::Plain,
        TooltipStyle::Bordered,
        TooltipStyle::Rounded,
        TooltipStyle::Info,
        TooltipStyle::Warning,
        TooltipStyle::Error,
        TooltipStyle::Success,
    ];

    for style in styles {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tooltip::new("Colored")
            .anchor(20, 10)
            .style(style)
            .fg(Color::CYAN)
            .bg(Color::rgb(80, 80, 80));

        t.render(&mut ctx);
    }
}

// =============================================================================
