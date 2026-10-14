//! Tooltip widget tests

mod props;
mod view;

use revue::layout::Rect;
use revue::render::{Buffer, Cell, Modifier};
use revue::style::Color;
use revue::widget::theme::{DARK_BG, EDITOR_BG};
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

fn render_in(t: &Tooltip, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    t.render(&mut ctx);
    buffer
}

/// Row `y` as text, one char per cell.
fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width()).map(|x| symbol(buffer, x, y)).collect()
}

/// Columns `range` of row `y` as text.
fn cols(buffer: &Buffer, y: u16, range: std::ops::Range<u16>) -> String {
    range.map(|x| symbol(buffer, x, y)).collect()
}

/// The first cell where `text` starts, searching row by row. Positions
/// are in cells, so only for text of 1-column chars.
fn find(buffer: &Buffer, text: &str) -> Option<(u16, u16)> {
    (0..buffer.height()).find_map(|y| {
        let r: Vec<char> = row(buffer, y).chars().collect();
        let t: Vec<char> = text.chars().collect();
        (0..=r.len().saturating_sub(t.len()))
            .find(|&x| r[x..].starts_with(&t))
            .map(|x| (x as u16, y))
    })
}

/// How many cells the box covers: the cells with a background (the
/// arrow has none).
fn box_cells(buffer: &Buffer) -> usize {
    (0..buffer.height())
        .flat_map(|y| (0..buffer.width()).map(move |x| (x, y)))
        .filter(|&(x, y)| buffer.get(x, y).unwrap().bg.is_some())
        .count()
}

/// The box covers exactly `(x, y, w, h)`: a bordered box with its corners
/// there, and no background outside it.
fn assert_box(buffer: &Buffer, (x, y, w, h): (u16, u16, u16, u16)) {
    assert!(
        matches!(symbol(buffer, x, y), '┌' | '╭'),
        "top-left at ({x}, {y})"
    );
    assert!(
        matches!(symbol(buffer, x + w - 1, y + h - 1), '┘' | '╯'),
        "bottom-right"
    );
    assert_eq!(box_cells(buffer), (w * h) as usize);
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
    let t = Tooltip::new("Initial").text("Updated text").anchor(10, 4);
    let buffer = render_in(&t, 20, 5);
    // 12 + 4 wide, 3 high, above the anchor: the text is on row 1.
    assert_eq!(row(&buffer, 1), "  │ Updated text │  ");
    assert!(find(&buffer, "Initial").is_none());
}

#[test]
fn test_tooltip_builder_position() {
    let t = Tooltip::new("Test").position(TooltipPosition::Bottom);
    let buffer = render(&t);
    // Below the anchor (0, 0): the arrow on row 1, the box from row 2.
    assert_eq!(symbol(&buffer, 0, 1), '▲');
    assert_eq!(cols(&buffer, 2, 0..8), "┌──────┐");
    assert_eq!(cols(&buffer, 3, 0..8), "│ Test │");
    assert_eq!(row(&buffer, 0).trim(), "");
}

#[test]
fn test_tooltip_builder_anchor() {
    let t = Tooltip::new("Test").anchor(10, 5);
    let buffer = render(&t);
    // 8 x 3, centered over x = 10 and one arrow row above y = 5.
    assert_eq!(find(&buffer, "┌──────┐"), Some((6, 1)));
    assert_eq!(find(&buffer, "│ Test │"), Some((6, 2)));
    assert_eq!(symbol(&buffer, 10, 4), '▼');
}

#[test]
fn test_tooltip_builder_style() {
    let t = Tooltip::new("Test")
        .style(TooltipStyle::Info)
        .anchor(20, 10);
    let buffer = render(&t);
    let (x, y) = find(&buffer, "Test").unwrap();
    let cell = buffer.get(x, y).unwrap();
    assert_eq!(cell.fg, Some(Color::WHITE));
    assert_eq!(cell.bg, Some(Color::rgb(30, 80, 100)));
    assert_eq!(symbol(&buffer, x - 2, y - 1), '┌');
}

#[test]
fn test_tooltip_builder_arrow() {
    let with = |arrow| render(&Tooltip::new("Test").anchor(20, 10).arrow(arrow));
    // Unicode: the box sits one row higher and the arrow fills the gap.
    let buffer = with(TooltipArrow::Unicode);
    assert_eq!(symbol(&buffer, 20, 9), '▼');
    assert_eq!(find(&buffer, "└──────┘"), Some((16, 8)));
    // None: the box touches the anchor's row.
    let buffer = with(TooltipArrow::None);
    assert_eq!(find(&buffer, "└──────┘"), Some((16, 9)));
}

#[test]
fn test_tooltip_builder_max_width() {
    let t = Tooltip::new("one two three four five six seven eight")
        .max_width(30)
        .position(TooltipPosition::Bottom)
        .anchor(20, 0);
    let buffer = render(&t);
    // 39 columns wrap at 30: 27 + 11; the box is 27 + 4 wide.
    let (x, y) = find(&buffer, "one two three four five six").unwrap();
    assert_eq!(find(&buffer, "seven eight"), Some((x, y + 1)));
    assert_eq!(symbol(&buffer, x + 28, y), '│');
}

#[test]
fn test_tooltip_builder_visible() {
    let t = Tooltip::new("Test").visible(false);
    assert!(!t.is_visible());
}

#[test]
fn test_tooltip_builder_fg() {
    let t = Tooltip::new("Test").fg(Color::RED).anchor(20, 10);
    let buffer = render(&t);
    let (x, y) = find(&buffer, "Test").unwrap();
    for (cx, cy) in [(x, y), (x - 2, y - 1), (20, 9)] {
        assert_eq!(
            buffer.get(cx, cy).unwrap().fg,
            Some(Color::RED),
            "({cx}, {cy})"
        );
    }
}

#[test]
fn test_tooltip_builder_bg() {
    let t = Tooltip::new("Test").bg(Color::BLUE).anchor(20, 10);
    let buffer = render(&t);
    // Every cell of the 8 x 3 box; the arrow has no background.
    for y in 6..9 {
        for x in 16..24 {
            assert_eq!(
                buffer.get(x, y).unwrap().bg,
                Some(Color::BLUE),
                "({x}, {y})"
            );
        }
    }
    assert_eq!(buffer.get(20, 9).unwrap().bg, None);
}

#[test]
fn test_tooltip_builder_title() {
    let t = Tooltip::new("Test").title("My Title").anchor(20, 10);
    let buffer = render(&t);
    // The title sets the width (8 + 2 + 4) and takes the first row inside.
    let (x, y) = find(&buffer, "My Title").unwrap();
    assert!(buffer.get(x, y).unwrap().modifier.contains(Modifier::BOLD));
    assert_eq!(find(&buffer, "Test"), Some((x, y + 1)));
    assert!(!buffer
        .get(x, y + 1)
        .unwrap()
        .modifier
        .contains(Modifier::BOLD));
    assert_eq!(symbol(&buffer, x + 11, y), '│');
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
    // 8 x 3 centered on x = 20, ending above the arrow row 9.
    assert_box(&render(&t), (16, 6, 8, 3));
    assert_eq!(symbol(&render(&t), 20, 9), '▼');
}

#[test]
fn test_tooltip_calculate_position_bottom() {
    let t = Tooltip::new("Test")
        .position(TooltipPosition::Bottom)
        .anchor(20, 10)
        .style(TooltipStyle::Bordered);
    assert_box(&render(&t), (16, 12, 8, 3));
    assert_eq!(symbol(&render(&t), 20, 11), '▲');
}

#[test]
fn test_tooltip_calculate_position_left() {
    let t = Tooltip::new("Test")
        .position(TooltipPosition::Left)
        .anchor(20, 10)
        .style(TooltipStyle::Bordered);
    // Ends left of the arrow column 19, centered on row 10.
    assert_box(&render(&t), (11, 9, 8, 3));
    assert_eq!(symbol(&render(&t), 19, 10), '▶');
}

#[test]
fn test_tooltip_calculate_position_right() {
    let t = Tooltip::new("Test")
        .position(TooltipPosition::Right)
        .anchor(20, 10)
        .style(TooltipStyle::Bordered);
    assert_box(&render(&t), (22, 9, 8, 3));
    assert_eq!(symbol(&render(&t), 21, 10), '◀');
}

#[test]
fn test_tooltip_calculate_position_auto() {
    let auto = |anchor: (u16, u16)| {
        render(
            &Tooltip::new("Test")
                .position(TooltipPosition::Auto)
                .anchor(anchor.0, anchor.1)
                .style(TooltipStyle::Bordered),
        )
    };
    // Room above: above. None above but below: below. Then right, then left.
    assert_box(&auto((20, 10)), (16, 6, 8, 3));
    assert_box(&auto((20, 1)), (16, 3, 8, 3));
    let buffer = render_in(
        &Tooltip::new("Test")
            .position(TooltipPosition::Auto)
            .anchor(5, 1),
        40,
        3,
    );
    assert_box(&buffer, (7, 0, 8, 3));
    assert_eq!(symbol(&buffer, 6, 1), '◀');
    let buffer = render_in(
        &Tooltip::new("Test")
            .position(TooltipPosition::Auto)
            .anchor(35, 1),
        40,
        3,
    );
    assert_box(&buffer, (26, 0, 8, 3));
    assert_eq!(symbol(&buffer, 34, 1), '▶');
}

#[test]
fn test_tooltip_calculate_position_edge_top_left() {
    let t = Tooltip::new("Test")
        .position(TooltipPosition::Top)
        .anchor(0, 0)
        .style(TooltipStyle::Bordered);

    // Kept inside the area at the left edge, drawn whole. (Where it goes
    // with no room above is #1045.)
    let buffer = render_in(&t, 20, 20);
    let (x, _) = find(&buffer, "│ Test │").unwrap();
    assert_eq!(x, 0);
}

#[test]
fn test_tooltip_calculate_position_edge_bottom_right() {
    let t = Tooltip::new("Test")
        .position(TooltipPosition::Bottom)
        .anchor(19, 19)
        .style(TooltipStyle::Bordered);

    // Pushed left so its right border is the area's last column, drawn
    // whole. (Where it goes with no room below is #1045.)
    let buffer = render_in(&t, 20, 20);
    let (x, _) = find(&buffer, "│ Test │").unwrap();
    assert_eq!(x, 12);
}

// =============================================================================
// Text Wrapping Tests
// =============================================================================

#[test]
fn test_tooltip_wrap_text_short() {
    let t = Tooltip::new("Short text").max_width(20).anchor(20, 10);
    let buffer = render(&t);
    // One line: the bottom border follows the text row.
    let (x, y) = find(&buffer, "Short text").unwrap();
    assert_eq!(find(&buffer, "└────────────┘"), Some((x - 2, y + 1)));
}

#[test]
fn test_tooltip_wrap_text_long() {
    let t = Tooltip::new("This is a very long text that should be wrapped")
        .max_width(20)
        .anchor(20, 10);
    let buffer = render(&t);
    let (x, y) = find(&buffer, "This is a very long").unwrap();
    assert_eq!(find(&buffer, "text that should be"), Some((x, y + 1)));
    assert_eq!(find(&buffer, "wrapped"), Some((x, y + 2)));
    assert_eq!(symbol(&buffer, x - 2, y + 3), '└');
}

#[test]
fn test_tooltip_wrap_text_multiple_lines() {
    let t = Tooltip::new("Line 1\nLine 2\nLine 3")
        .max_width(20)
        .anchor(20, 10);
    let buffer = render(&t);
    let (x, y) = find(&buffer, "Line 1").unwrap();
    assert_eq!(find(&buffer, "Line 2"), Some((x, y + 1)));
    assert_eq!(find(&buffer, "Line 3"), Some((x, y + 2)));
}

#[test]
fn test_tooltip_wrap_text_empty() {
    let t = Tooltip::new("").max_width(20).anchor(20, 10);
    let buffer = render(&t);
    // One empty line: 0 + 4 wide, 3 high.
    assert_box(&buffer, (18, 6, 4, 3));
    assert_eq!(row(&buffer, 7).trim(), "│  │");
}

#[test]
fn test_tooltip_wrap_text_zero_max_width() {
    // 0 means 40: nine 4-letter words (44 columns) wrap after the eighth.
    let t = Tooltip::new(["word"; 9].join(" "))
        .max_width(0)
        .position(TooltipPosition::Bottom)
        .anchor(25, 0);
    let buffer = render_in(&t, 60, 10);
    let (x, y) = find(&buffer, &["word"; 8].join(" ")).unwrap();
    assert_eq!(find(&buffer, "word "), Some((x, y)));
    assert_eq!(cols(&buffer, y + 1, x..x + 5), "word ");
}

// =============================================================================
// Dimension Calculation Tests
// =============================================================================

#[test]
fn test_tooltip_calculate_dimensions_plain() {
    let t = Tooltip::new("Short")
        .style(TooltipStyle::Plain)
        .arrow(TooltipArrow::None)
        .anchor(20, 10);
    let buffer = render(&t);
    // 5 + 2 padding, one row: 7 cells.
    assert_eq!(box_cells(&buffer), 7);
    assert_eq!(find(&buffer, " Short "), Some((17, 9)));
}

#[test]
fn test_tooltip_calculate_dimensions_bordered() {
    let t = Tooltip::new("Short")
        .style(TooltipStyle::Bordered)
        .arrow(TooltipArrow::None)
        .anchor(20, 10);
    // 5 + 4 wide, 1 + 2 high.
    assert_eq!(box_cells(&render(&t)), 9 * 3);
}

#[test]
fn test_tooltip_calculate_dimensions_with_title() {
    let t = Tooltip::new("Content")
        .title("Title")
        .style(TooltipStyle::Bordered)
        .arrow(TooltipArrow::None)
        .anchor(20, 10);
    // max(7, 5 + 2) + 4 wide; the text, the title row and the border high.
    assert_eq!(box_cells(&render(&t)), 11 * 4);
}

#[test]
fn test_tooltip_calculate_dimensions_long_text() {
    let t = Tooltip::new("This is a longer text")
        .style(TooltipStyle::Bordered)
        .arrow(TooltipArrow::None)
        .anchor(20, 10);
    assert_eq!(box_cells(&render(&t)), 25 * 3);
}

#[test]
fn test_tooltip_calculate_dimensions_multiline() {
    let t = Tooltip::new("Line 1\nLine 2\nLine 3")
        .style(TooltipStyle::Bordered)
        .arrow(TooltipArrow::None)
        .anchor(20, 10);
    assert_eq!(box_cells(&render(&t)), 10 * 5);
}

// =============================================================================
// Render Tests - Styles and Arrows
// =============================================================================

#[test]
fn test_tooltip_render_all_styles() {
    let styles = [
        (TooltipStyle::Plain, ' ', Color::WHITE, DARK_BG),
        (TooltipStyle::Bordered, '┌', Color::WHITE, EDITOR_BG),
        (TooltipStyle::Rounded, '╭', Color::WHITE, EDITOR_BG),
        (
            TooltipStyle::Info,
            '┌',
            Color::WHITE,
            Color::rgb(30, 80, 100),
        ),
        (
            TooltipStyle::Warning,
            '┌',
            Color::BLACK,
            Color::rgb(180, 150, 0),
        ),
        (
            TooltipStyle::Error,
            '┌',
            Color::WHITE,
            Color::rgb(150, 30, 30),
        ),
        (
            TooltipStyle::Success,
            '┌',
            Color::WHITE,
            Color::rgb(30, 100, 50),
        ),
    ];

    for (style, corner, fg, bg) in styles {
        let t = Tooltip::new("Test")
            .anchor(20, 10)
            .style(style)
            .arrow(TooltipArrow::None);
        let buffer = render(&t);
        let (x, y) = find(&buffer, "Test").unwrap();
        let cell = buffer.get(x, y).unwrap();
        assert_eq!((cell.fg, cell.bg), (Some(fg), Some(bg)), "{style:?}");
        if corner == ' ' {
            assert_eq!(box_cells(&buffer), 6, "{style:?}");
        } else {
            assert_eq!(symbol(&buffer, x - 2, y - 1), corner, "{style:?}");
        }
    }
}

#[test]
fn test_tooltip_render_all_arrow_types() {
    // The cell between the box and the anchor (20, 10).
    let between = |arrow| {
        let t = Tooltip::new("Test")
            .anchor(20, 10)
            .arrow(arrow)
            .style(TooltipStyle::Bordered);
        symbol(&render(&t), 20, 9)
    };
    // No arrow: the box moves down a row, so its bottom border is there.
    assert_eq!(between(TooltipArrow::None), '─');
    assert_eq!(between(TooltipArrow::Simple), 'v');
    assert_eq!(between(TooltipArrow::Unicode), '▼');
}

#[test]
fn test_tooltip_render_plain() {
    let t = Tooltip::new("Hello World")
        .anchor(20, 10)
        .position(TooltipPosition::Top)
        .style(TooltipStyle::Plain);

    let buffer = render(&t);
    // 11 + 2 wide, one row, centered on x = 20 above the arrow.
    assert_eq!(cols(&buffer, 8, 14..27), " Hello World ");
    assert_eq!(box_cells(&buffer), 13);
    assert_eq!(symbol(&buffer, 20, 9), '▼');
}

#[test]
fn test_tooltip_render_bordered() {
    let t = Tooltip::new("Hello World")
        .anchor(20, 10)
        .position(TooltipPosition::Top)
        .style(TooltipStyle::Bordered);

    let buffer = render(&t);
    assert_eq!(cols(&buffer, 6, 13..28), "┌─────────────┐");
    assert_eq!(cols(&buffer, 7, 13..28), "│ Hello World │");
    assert_eq!(cols(&buffer, 8, 13..28), "└─────────────┘");
}

#[test]
fn test_tooltip_render_rounded() {
    let t = Tooltip::new("Hello")
        .anchor(20, 10)
        .style(TooltipStyle::Rounded);

    let buffer = render(&t);
    assert_eq!(cols(&buffer, 6, 16..25), "╭───────╮");
    assert_eq!(cols(&buffer, 7, 16..25), "│ Hello │");
    assert_eq!(cols(&buffer, 8, 16..25), "╰───────╯");
}

#[test]
fn test_tooltip_render_with_title() {
    let t = Tooltip::new("Content")
        .title("Title")
        .anchor(20, 10)
        .style(TooltipStyle::Bordered);

    let buffer = render(&t);
    // 11 x 4: the border, the bold title, the text, the border.
    assert_eq!(cols(&buffer, 5, 15..26), "┌─────────┐");
    assert_eq!(cols(&buffer, 6, 15..26), "│ Title   │");
    assert_eq!(cols(&buffer, 7, 15..26), "│ Content │");
    assert_eq!(cols(&buffer, 8, 15..26), "└─────────┘");
    for x in 17..22 {
        assert!(buffer.get(x, 6).unwrap().modifier.contains(Modifier::BOLD));
    }
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
    let mut t = Tooltip::new("Now visible").delay(1).anchor(20, 10);
    assert!(find(&render(&t), "Now visible").is_none());
    t.tick();

    assert!(find(&render(&t), "Now visible").is_some());
}

#[test]
fn test_tooltip_render_custom_colors() {
    let t = Tooltip::new("Colored")
        .anchor(20, 10)
        .fg(Color::CYAN)
        .bg(Color::RED);

    let buffer = render(&t);
    let (x, y) = find(&buffer, "Colored").unwrap();
    // The text, the padding and the border.
    for (cx, cy) in [(x, y), (x - 1, y), (x - 2, y - 1)] {
        let cell = buffer.get(cx, cy).unwrap();
        assert_eq!((cell.fg, cell.bg), (Some(Color::CYAN), Some(Color::RED)));
    }
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
        let t = Tooltip::new("Test")
            .anchor(20, 10)
            .position(pos)
            .style(TooltipStyle::Bordered);

        let buffer = render(&t);
        // The anchor cell is left alone and the text is on the asked side.
        assert_eq!(symbol(&buffer, 20, 10), ' ', "{pos:?}");
        let (x, y) = find(&buffer, "Test").unwrap();
        let side = match pos {
            TooltipPosition::Top => y < 10,
            TooltipPosition::Bottom => y > 10,
            TooltipPosition::Left => x + 4 < 20 && y == 10,
            _ => x > 20 && y == 10,
        };
        assert!(side, "{pos:?} at ({x}, {y})");
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
    let buffer = render_in(&t, 60, 30);
    // The word is drawn from the left border on, as far as the area goes.
    let (_, y) = find(&buffer, &format!("│ {}", "A".repeat(48))).unwrap();
    assert_eq!(cols(&buffer, y, 0..2), "│ ");
}

#[test]
fn test_tooltip_zero_anchor() {
    let t = Tooltip::new("Test").anchor(0, 0);
    let buffer = render_in(&t, 20, 20);
    // Centering would start left of the area: it starts at column 0.
    assert_eq!(find(&buffer, "│ Test │").map(|(x, _)| x), Some(0));
}

#[test]
fn test_tooltip_large_anchor() {
    let t = Tooltip::new("Test").anchor(100, 100);
    let buffer = render_in(&t, 120, 120);
    assert_box(&buffer, (96, 96, 8, 3));
    assert_eq!(symbol(&buffer, 100, 99), '▼');
}

#[test]
fn test_tooltip_very_small_area() {
    let t = Tooltip::new("Test")
        .anchor(0, 0)
        .style(TooltipStyle::Bordered);

    // A 5 x 5 area in a larger buffer: the 8-wide box starts at the
    // area's corner and nothing is drawn outside the area.
    let mut buffer = Buffer::new(10, 10);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 5, 5));
    t.render(&mut ctx);
    assert_eq!(symbol(&buffer, 0, 0), '┌');
    for y in 0..10 {
        for x in 0..10 {
            if x >= 5 || y >= 5 {
                assert_eq!(buffer.get(x, y).unwrap(), &Cell::default(), "({x}, {y})");
            }
        }
    }
}

#[test]
fn test_tooltip_text_with_special_chars() {
    let t = Tooltip::new("Text with emoji and symbols: @#$%").anchor(20, 10);
    let buffer = render(&t);
    let (x, y) = find(&buffer, "Text with emoji and symbols: @#$%").unwrap();
    // 33 columns, then the padding and the right border.
    assert_eq!(symbol(&buffer, x + 34, y), '│');
}

#[test]
fn test_tooltip_multiline_with_wrapping() {
    let t = Tooltip::new("Line 1 is very long and should wrap\nLine 2")
        .max_width(20)
        .anchor(20, 10);
    let buffer = render(&t);
    // The long line wraps; the second line keeps its own row.
    let (x, y) = find(&buffer, "Line 1 is very long").unwrap();
    assert_eq!(find(&buffer, "and should wrap"), Some((x, y + 1)));
    assert_eq!(find(&buffer, "Line 2"), Some((x, y + 2)));
}

#[test]
fn test_tooltip_title_longer_than_content() {
    let t = Tooltip::new("Short")
        .title("This is a very long title")
        .style(TooltipStyle::Bordered)
        .anchor(20, 10);

    let buffer = render(&t);
    // The title sets the width: 25 + 2 + 4.
    let (x, y) = find(&buffer, "This is a very long title").unwrap();
    assert_eq!(symbol(&buffer, x - 2, y), '│');
    assert_eq!(symbol(&buffer, x + 28, y), '│');
    assert_eq!(find(&buffer, "Short"), Some((x, y + 1)));
}

#[test]
fn test_tooltip_text_with_tabs() {
    let t = Tooltip::new("Text\twith\ttabs").anchor(20, 10);
    let buffer = render(&t);
    // The words are drawn in order, and the box ends one padding cell
    // after the last one.
    let (_, y) = find(&buffer, "Text").unwrap();
    let line = row(&buffer, y);
    let (a, b, c) = (
        line.find("Text").unwrap(),
        line.find("with").unwrap(),
        line.find("tabs").unwrap(),
    );
    assert!(a < b && b < c);
    assert!(line[c..].starts_with("tabs │"));
}

#[test]
fn test_tooltip_max_width_boundary() {
    let text = ["word"; 18].join(" ");
    let t = Tooltip::new(&text).max_width(100).anchor(60, 10);
    let buffer = render_in(&t, 120, 20);
    // 89 columns fit under 100: one line.
    let (x, y) = find(&buffer, &text).unwrap();
    assert_eq!(symbol(&buffer, x - 2, y + 1), '└');
}

#[test]
fn test_tooltip_default_trait() {
    let t: Tooltip = Default::default();
    // The same as an empty `new`.
    let (a, b) = (render(&t), render(&Tooltip::new("")));
    for y in 0..20 {
        for x in 0..40 {
            assert_eq!(a.get(x, y), b.get(x, y));
        }
    }
    assert!(box_cells(&a) > 0);
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

    let buffer = render(&t);
    assert_eq!(symbol(&buffer, 0, 1), '▲');
    let (x, y) = find(&buffer, "│ Test │").unwrap();
    assert_eq!((x, y), (0, 3));
    let cell = buffer.get(2, 3).unwrap();
    assert_eq!(
        (cell.fg, cell.bg),
        (Some(Color::BLACK), Some(Color::rgb(180, 150, 0)))
    );
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
            let t = Tooltip::new("Test")
                .anchor(20, 10)
                .position(pos)
                .style(style)
                .arrow(TooltipArrow::None);

            let buffer = render(&t);
            // With no arrow the box touches the anchor; its text row or
            // column is next to it whatever the style.
            let (x, y) = find(&buffer, "Test").unwrap();
            let bordered = style != TooltipStyle::Plain;
            let (ex, ey) = match pos {
                TooltipPosition::Top => (18, if bordered { 8 } else { 9 }),
                TooltipPosition::Bottom => (18, if bordered { 12 } else { 11 }),
                TooltipPosition::Left => (if bordered { 14 } else { 15 }, 10),
                _ => (if bordered { 23 } else { 22 }, 10),
            };
            assert_eq!((x, y), (ex, ey), "{pos:?} {style:?}");
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
        let t = Tooltip::new("Content")
            .title("Title")
            .anchor(20, 10)
            .style(style);

        let buffer = render(&t);
        // Every bordered style draws the bold title above the text.
        let (x, y) = find(&buffer, "Title").unwrap();
        assert!(buffer.get(x, y).unwrap().modifier.contains(Modifier::BOLD));
        assert_eq!(find(&buffer, "Content"), Some((x, y + 1)), "{style:?}");
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
        let t = Tooltip::new("Colored")
            .anchor(20, 10)
            .style(style)
            .fg(Color::CYAN)
            .bg(Color::rgb(80, 80, 80));

        let buffer = render(&t);
        // The builder colors win over every style's own, in every box cell.
        let mut n = 0;
        for y in 0..20 {
            for x in 0..40 {
                let cell = buffer.get(x, y).unwrap();
                if cell.bg.is_some() {
                    n += 1;
                    assert_eq!(cell.bg, Some(Color::rgb(80, 80, 80)), "{style:?}");
                    assert_eq!(cell.fg, Some(Color::CYAN), "{style:?}");
                }
            }
        }
        assert!(n > 0, "{style:?}");
    }
}

// =============================================================================

mod snapshots {
    use revue::prelude::*;
    use revue::testing::{Pilot, TestApp, TestConfig};

    #[test]
    fn test_tooltip_basic() {
        use revue::widget::Tooltip;

        let view = Tooltip::new("This is helpful information")
            .visible(true)
            .anchor(10, 5);

        let config = TestConfig::with_size(50, 10);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("tooltip_basic");
    }

    #[test]
    fn test_tooltip_variants() {
        use revue::widget::Tooltip;

        let view = vstack()
            .gap(2)
            .child(Tooltip::info("Info tooltip").visible(true).anchor(5, 1))
            .child(
                Tooltip::warning("Warning tooltip")
                    .visible(true)
                    .anchor(5, 4),
            )
            .child(Tooltip::error("Error tooltip").visible(true).anchor(5, 7));

        let config = TestConfig::with_size(50, 12);
        let mut app = TestApp::with_config(view, config);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("tooltip_variants");
    }
}
