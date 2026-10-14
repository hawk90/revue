//! Border drawing against a plain model of the frame, `measure` against
//! that model, and `draw_border` at any offset.

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{draw_border, hstack, vstack, Border, BorderType, Text};

use crate::traits::placement::assert_zero_areas_draw_nothing;

const TYPES: [BorderType; 6] = [
    BorderType::None,
    BorderType::Single,
    BorderType::Double,
    BorderType::Rounded,
    BorderType::Thick,
    BorderType::Ascii,
];

/// Each type's glyphs, written out here rather than read from the library:
/// top-left, top-right, bottom-left, bottom-right, horizontal, vertical.
fn glyphs(t: BorderType) -> [char; 6] {
    match t {
        BorderType::None => [' '; 6],
        BorderType::Single => ['┌', '┐', '└', '┘', '─', '│'],
        BorderType::Double => ['╔', '╗', '╚', '╝', '═', '║'],
        BorderType::Rounded => ['╭', '╮', '╰', '╯', '─', '│'],
        BorderType::Thick => ['┏', '┓', '┗', '┛', '━', '┃'],
        BorderType::Ascii => ['+', '+', '+', '+', '-', '|'],
    }
}

/// Title glyphs with their terminal widths known up front: narrow, wide
/// (CJK, Hangul, emoji) and the zero-width VS16.
const TITLE_CHARS: [(char, u16); 7] = [
    ('a', 1),
    ('Z', 1),
    (' ', 1),
    ('日', 2),
    ('한', 2),
    ('🔄', 2),
    ('\u{FE0F}', 0),
];

/// What a cell should hold: a glyph, or the right half of a wide one.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Want {
    Glyph(char),
    Cont,
}

/// The frame a `width` x `height` border draws, frame-relative, `None` where
/// nothing is drawn. The title sits after `─` at column 2 and is cut, by
/// columns and whole glyphs, to the `width - 4` columns the corners leave.
fn model(
    width: u16,
    height: u16,
    t: BorderType,
    title: Option<&[(char, u16)]>,
) -> Vec<Vec<Option<Want>>> {
    let (w, h) = (width as usize, height as usize);
    let mut grid = vec![vec![None; w]; h];
    if w < 2 || h < 2 {
        return grid;
    }
    let [tl, tr, bl, br, hz, vt] = glyphs(t);
    grid[0][0] = Some(Want::Glyph(tl));
    let mut start = 1;
    if let Some(title) = title {
        grid[0][1] = Some(Want::Glyph(hz));
        let max = w.saturating_sub(4);
        let (mut used, mut x) = (0usize, 2usize);
        for &(c, cw) in title {
            let cw = cw as usize;
            if used + cw > max {
                break;
            }
            used += cw;
            if cw == 0 {
                continue;
            }
            grid[0][x] = Some(Want::Glyph(c));
            for d in 1..cw {
                grid[0][x + d] = Some(Want::Cont);
            }
            x += cw;
        }
        start = x;
    }
    for cell in grid[0].iter_mut().take(w - 1).skip(start) {
        *cell = Some(Want::Glyph(hz));
    }
    grid[0][w - 1] = Some(Want::Glyph(tr));
    for row in grid.iter_mut().take(h - 1).skip(1) {
        row[0] = Some(Want::Glyph(vt));
        row[w - 1] = Some(Want::Glyph(vt));
    }
    grid[h - 1][0] = Some(Want::Glyph(bl));
    for cell in grid[h - 1].iter_mut().take(w - 1).skip(1) {
        *cell = Some(Want::Glyph(hz));
    }
    grid[h - 1][w - 1] = Some(Want::Glyph(br));
    grid
}

/// A buffer of '.' so a cell nobody drew is told apart from a drawn blank.
fn dotted(width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            buffer.set(x, y, Cell::new('.'));
        }
    }
    buffer
}

/// Check `buffer` holds `grid` at `(ox, oy)` in `fg`/`bg`, and '.' elsewhere.
fn check(
    buffer: &Buffer,
    grid: &[Vec<Option<Want>>],
    (ox, oy): (u16, u16),
    fg: Option<Color>,
    bg: Option<Color>,
) -> Result<(), TestCaseError> {
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            let cell = buffer.get(x, y).unwrap();
            let want = match (x.checked_sub(ox), y.checked_sub(oy)) {
                (Some(fx), Some(fy)) => grid
                    .get(fy as usize)
                    .and_then(|row| row.get(fx as usize))
                    .copied()
                    .flatten(),
                _ => None,
            };
            match want {
                None => prop_assert_eq!(cell.symbol, '.', "({}, {}) was drawn", x, y),
                Some(Want::Cont) => {
                    prop_assert!(cell.is_continuation(), "({}, {}) {:?}", x, y, cell)
                }
                Some(Want::Glyph(c)) => {
                    prop_assert_eq!(cell.symbol, c, "({}, {})", x, y);
                    prop_assert!(!cell.is_continuation(), "({}, {})", x, y);
                    prop_assert_eq!((cell.fg, cell.bg), (fg, bg), "({}, {})", x, y);
                }
            }
        }
    }
    Ok(())
}

fn title_strategy() -> impl Strategy<Value = Option<Vec<(char, u16)>>> {
    prop::option::of(prop::collection::vec(
        prop::sample::select(TITLE_CHARS.to_vec()),
        0..14,
    ))
}

fn color_strategy() -> impl Strategy<Value = Option<Color>> {
    prop::option::of(prop::sample::select(vec![
        Color::RED,
        Color::BLUE,
        Color::GREEN,
    ]))
}

#[test]
fn test_border_zero_areas() {
    for t in TYPES {
        assert_zero_areas_draw_nothing(&Border::new().border_type(t));
        assert_zero_areas_draw_nothing(
            &Border::new()
                .border_type(t)
                .title("Title")
                .fg(Color::RED)
                .child(Text::new("child")),
        );
    }
}

/// The child is drawn inside the frame and cut there: a long line stops
/// before the right edge, extra lines stop above the bottom edge.
#[test]
fn test_border_child_is_cut_at_the_frame() {
    let mut buffer = Buffer::new(8, 4);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 8, 4));
    let lines = vstack()
        .child(Text::new("abcdefghij"))
        .child(Text::new("second"))
        .child(Text::new("third"));
    Border::single().child(lines).render(&mut ctx);

    let rows: Vec<String> = (0..4)
        .map(|y| (0..8).map(|x| buffer.get(x, y).unwrap().symbol).collect())
        .collect();
    assert_eq!(rows, ["┌──────┐", "│abcdef│", "│second│", "└──────┘"]);
}

/// The frame takes one cell on each side: a child's size plus two.
#[test]
fn test_border_measure_adds_the_frame_to_the_child() {
    assert_eq!(
        Border::new().child(Text::new("Hi")).measure(80, 24),
        Some((4, 3))
    );
    assert_eq!(Border::new().measure(80, 24), Some((2, 2)));
    // A wider title than child sets the width: `┌─Title─┐`.
    assert_eq!(
        Border::new()
            .title("Title")
            .child(Text::new("Hi"))
            .measure(80, 24),
        Some((9, 3))
    );
}

/// A max below the min counts as the min (the rule of the shared
/// `constraints::constrain`): the frame is drawn at the min size.
#[test]
fn test_border_min_wins_over_a_smaller_max() {
    let mut buffer = Buffer::new(10, 6);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 10, 6));
    Border::new().min_size(6, 4).max_size(3, 2).render(&mut ctx);

    let rows: Vec<String> = (0..6)
        .map(|y| (0..10).map(|x| buffer.get(x, y).unwrap().symbol).collect())
        .collect();
    assert_eq!(
        rows,
        [
            "┌────┐    ",
            "│    │    ",
            "│    │    ",
            "└────┘    ",
            "          ",
            "          "
        ]
    );
}

/// A child that writes 20 cells from its origin straight into the buffer,
/// past its own area.
struct Spill;

impl View for Spill {
    fn render(&self, ctx: &mut RenderContext) {
        let (x, y) = (ctx.area.x, ctx.area.y);
        for i in 0..20 {
            ctx.buffer.set(x + i, y, Cell::new('#'));
        }
    }
}

/// An 8-column border `#b` around `Spill`, on a 20-column screen.
struct Narrow;

impl View for Narrow {
    fn render(&self, ctx: &mut RenderContext) {
        hstack()
            .child_sized(Border::new().child(Spill).element_id("b"), 8)
            .render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Narrow"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
}

/// The columns right of the border that the child wrote to.
fn spilled(css: &str) -> Vec<u16> {
    let mut h = PipelineHarness::with_css(css, 20, 4).dom_from_render(true);
    h.draw(&Narrow);
    (8..20)
        .filter(|&x| h.buffer().get(x, 1).unwrap().symbol == '#')
        .collect()
}

/// `overflow: hidden` on the border keeps a child that writes to the buffer
/// directly inside the border's box.
#[test]
fn test_border_overflow_hidden_clips_the_child_at_the_border() {
    // Without it the child spills, or the check below would prove nothing.
    assert_eq!(spilled(""), (8..20).collect::<Vec<_>>());
    assert_eq!(spilled("#b { overflow: hidden; }"), Vec::<u16>::new());
}

proptest! {
    /// Any type, title, colors and area: the border draws the model's frame
    /// at the area's origin, in its colors, and touches nothing else. A max
    /// size shrinks the frame; a min size within the area grows it.
    #[test]
    fn prop_border_draws_the_model_frame(
        t in prop::sample::select(TYPES.to_vec()),
        title in title_strategy(),
        fg in color_strategy(),
        bg in color_strategy(),
        (ax, ay, aw, ah) in (0u16..4, 0u16..3, 0u16..26, 0u16..8),
        min in (0u16..30, 0u16..10),
        extra in prop::option::of((0u16..30, 0u16..10)),
    ) {
        // The min is kept within the area: past it the frame is clipped at
        // the area's edge, a case the model leaves out.
        let (min_w, min_h) = (min.0.min(aw), min.1.min(ah));
        // A max below the min draws one size and measures another (see the
        // issue drafted in the border audit), so the max is the min or more.
        let (max_w, max_h) = extra.map_or((0, 0), |(ew, eh)| (min_w + ew, min_h + eh));
        let mut b = Border::new().border_type(t).constrain(min_w, min_h, max_w, max_h);
        let title_str: Option<String> = title.as_ref().map(|t| t.iter().map(|&(c, _)| c).collect());
        if let Some(s) = &title_str {
            b = b.title(s.clone());
        }
        if let Some(c) = fg {
            b = b.fg(c);
        }
        if let Some(c) = bg {
            b = b.bg(c);
        }

        let mut buffer = dotted(ax + aw + 3, ay + ah + 2);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(ax, ay, aw, ah));
        b.render(&mut ctx);

        let fw = if max_w > 0 { aw.min(max_w) } else { aw };
        let fh = if max_h > 0 { ah.min(max_h) } else { ah };
        let grid = model(fw, fh, t, title.as_deref());
        check(&buffer, &grid, (ax, ay), fg, bg)?;
    }

    /// `measure` is the frame around the title's columns (no child), at
    /// least the min and at most the max and the offer; and drawing at that
    /// size shows the whole title whenever the size left room for it.
    #[test]
    fn prop_border_measure_fits_the_title(
        title in title_strategy(),
        (offer_w, offer_h) in (0u16..40, 0u16..10),
        min in (0u16..20, 0u16..8),
        extra in prop::option::of((0u16..20, 0u16..8)),
    ) {
        // Max at or above the min, as above.
        let (max_w, max_h) = extra.map_or((0, 0), |(ew, eh)| (min.0 + ew, min.1 + eh));
        let mut b = Border::new().constrain(min.0, min.1, max_w, max_h);
        let title_cols: u16 = title.as_ref().map_or(0, |t| t.iter().map(|&(_, w)| w).sum::<u16>() + 4);
        if let Some(t) = &title {
            b = b.title(t.iter().map(|&(c, _)| c).collect::<String>());
        }

        let (w, h) = b.measure(offer_w, offer_h).unwrap();
        let cap = |v: u16, max: u16| if max > 0 { v.min(max) } else { v };
        let want_w = cap(2u16.max(title_cols).max(min.0), max_w).min(offer_w);
        let want_h = cap(2u16.max(min.1), max_h).min(offer_h);
        prop_assert_eq!((w, h), (want_w, want_h));

        // Drawn at its measured size. A min above the offer measures smaller
        // than the min and draws clipped; the model leaves that out.
        if w < min.0 || h < min.1 {
            return Ok(());
        }
        let mut buffer = dotted(w + 2, h + 2);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
        b.render(&mut ctx);
        let grid = model(w, h, BorderType::Single, title.as_deref());
        check(&buffer, &grid, (0, 0), None, None)?;
        if let Some(t) = title.as_ref().filter(|_| w >= title_cols && h >= 2) {
            let shown: String = (2..w - 2)
                .map(|x| buffer.get(x, 0).unwrap())
                .filter(|c| !c.is_continuation() && c.symbol != '─')
                .map(|c| c.symbol)
                .collect();
            let whole: String = t.iter().filter(|&&(_, cw)| cw > 0).map(|&(c, _)| c).collect();
            prop_assert_eq!(shown, whole);
        }
    }

    /// `draw_border` draws the same frame (no title) at any offset in the
    /// buffer, in its colors; `None` and areas under 2x2 draw nothing.
    #[test]
    fn prop_draw_border_draws_the_model_frame(
        t in prop::sample::select(TYPES.to_vec()),
        fg in color_strategy(),
        bg in color_strategy(),
        (x, y, w, h) in (0u16..5, 0u16..4, 0u16..20, 0u16..8),
    ) {
        let mut buffer = dotted(x + w + 2, y + h + 2);
        draw_border(&mut buffer, Rect::new(x, y, w, h), t, fg, bg);

        let grid = if t == BorderType::None { model(0, 0, t, None) } else { model(w, h, t, None) };
        check(&buffer, &grid, (x, y), fg, bg)?;
    }
}
