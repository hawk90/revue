//! Zero, maximum and overflowing sizes, ratios and coordinates.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;

#[test]
fn test_render_context_with_zero_area() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Should not panic with zero area
    ctx.draw_char(0, 0, 'x', Color::WHITE);
}

/// Row `y` with `_` for a continuation cell
fn cells(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| match buffer.get(x, y).unwrap().symbol {
            '\0' => '_',
            c => c,
        })
        .collect()
}

#[test]
fn set_a_wide_glyph_in_the_last_column_of_an_offset_area_draws_a_space() {
    // The area is columns 3-5 of an 8-column row; the cell right of it
    // belongs to another view and must stay as it was (#974)
    use revue::render::Cell;
    let mut buffer = Buffer::new(8, 2);
    buffer.put_str(0, 1, "........");
    buffer.set(6, 1, Cell::new('Z'));
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(3, 1, 3, 1));
        ctx.set(0, 0, Cell::new('日'));
        ctx.set(2, 0, Cell::new('日').bg(Color::BLUE));
    }
    assert_eq!(cells(&buffer, 1), "...日_ Z.");
    assert_eq!(buffer.get(5, 1).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn set_a_wide_glyph_across_the_clip_draws_a_space() {
    // The clip ends inside the area: column 4 is the clip's last
    use revue::render::Cell;
    let mut buffer = Buffer::new(8, 1);
    buffer.put_str(0, 0, "........");
    {
        let mut ctx =
            RenderContext::new(&mut buffer, Rect::new(0, 0, 8, 1)).with_clip(Rect::new(2, 0, 3, 1));
        ctx.set(3, 0, Cell::new('日')); // 3-4, inside
        ctx.set(4, 0, Cell::new('月')); // 4-5, crosses the clip
        ctx.set(1, 0, Cell::new('火')); // left of the clip
    }
    // 火 left of the clip writes nothing; 月 would cross the clip, so a
    // space is drawn at 4, over 日's right half, which blanks 日 at 3;
    // column 5, past the clip, is left alone
    assert_eq!(cells(&buffer, 0), "...  ...");
}

#[test]
fn put_str_a_wide_glyph_across_the_clip_draws_a_space() {
    // `put_str` wrote the glyph with `Buffer::set`, which wrote its right
    // half past the context's clip (#1609). Column 4 is the clip's last.
    let mut buffer = Buffer::new(8, 1);
    buffer.put_str(0, 0, "........");
    {
        let mut ctx =
            RenderContext::new(&mut buffer, Rect::new(1, 0, 7, 1)).with_clip(Rect::new(2, 0, 3, 1));
        // 日 at 2-3 inside, 月 at 4-5 crossing the clip
        assert_eq!(ctx.put_str(1, 0, "日月"), 4);
    }
    assert_eq!(cells(&buffer, 0), "..日_ ...");
    // Both halves of 日 plain, as `put_str` draws
    let glyph = *buffer.get(2, 0).unwrap();
    let right = *buffer.get(3, 0).unwrap();
    assert_eq!(
        (right.fg, right.bg, right.modifier),
        (glyph.fg, glyph.bg, glyph.modifier)
    );
}

#[test]
fn put_str_a_wide_glyph_at_the_last_column_of_an_offset_area() {
    // The cell right of the area belongs to another view
    let mut buffer = Buffer::new(8, 1);
    buffer.put_str(0, 0, "........");
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(2, 0, 4, 1));
        // 日 fits at 2-3 and 月 at 4-5; 火 would cross the area's edge
        assert_eq!(ctx.put_str(0, 0, "日月火"), 4);
    }
    assert_eq!(cells(&buffer, 0), "..日_月_..");
}

/// Each `draw_char*` with its style, at relative `(x, 0)`
type DrawChar = fn(&mut RenderContext, u16, char);
const DRAW_CHARS: [(&str, DrawChar); 3] = [
    ("draw_char", |c, x, ch| c.draw_char(x, 0, ch, Color::RED)),
    ("draw_char_bg", |c, x, ch| {
        c.draw_char_bg(x, 0, ch, Color::RED, Color::BLUE)
    }),
    ("draw_char_bold", |c, x, ch| {
        c.draw_char_bold(x, 0, ch, Color::RED)
    }),
];

/// The colors and modifier `name` draws with
fn char_style(name: &str) -> (Option<Color>, Option<Color>, revue::render::Modifier) {
    use revue::render::Modifier;
    match name {
        "draw_char" => (Some(Color::RED), None, Modifier::empty()),
        "draw_char_bg" => (Some(Color::RED), Some(Color::BLUE), Modifier::empty()),
        _ => (Some(Color::RED), None, Modifier::BOLD),
    }
}

#[test]
fn draw_char_a_wide_glyph_at_the_last_column_of_an_offset_area_draws_a_space() {
    // `draw_char*` checked only the glyph's column, then `Buffer::set` wrote
    // its right half one column past the area (#1619). The area is columns
    // 1-3; column 4 belongs to another view.
    for (name, draw) in DRAW_CHARS {
        let mut buffer = Buffer::new(6, 1);
        buffer.put_str(0, 0, "......");
        {
            let mut ctx = RenderContext::new(&mut buffer, Rect::new(1, 0, 3, 1));
            draw(&mut ctx, 0, '日'); // 1-2, fits
            draw(&mut ctx, 2, '月'); // 3-4, crosses the area's edge
        }
        assert_eq!(cells(&buffer, 0), ".日_ ..", "{name}");
        for x in 1..4 {
            let cell = buffer.get(x, 0).unwrap();
            assert_eq!(
                (cell.fg, cell.bg, cell.modifier),
                char_style(name),
                "{name} {x}"
            );
        }
        for x in [0, 4, 5] {
            assert_eq!(buffer.get(x, 0).unwrap().fg, None, "{name} {x}");
        }
    }
}

#[test]
fn draw_char_a_wide_glyph_across_the_clip_draws_a_space() {
    // Column 4 is the clip's last; column 5 is past it
    for (name, draw) in DRAW_CHARS {
        let mut buffer = Buffer::new(8, 1);
        buffer.put_str(0, 0, "........");
        {
            let mut ctx = RenderContext::new(&mut buffer, Rect::new(1, 0, 7, 1))
                .with_clip(Rect::new(2, 0, 3, 1));
            draw(&mut ctx, 1, '日'); // 2-3, inside
            draw(&mut ctx, 3, '月'); // 4-5, crosses the clip
            draw(&mut ctx, 0, '火'); // 1, left of the clip
        }
        assert_eq!(cells(&buffer, 0), "..日_ ...", "{name}");
        for x in 2..5 {
            let cell = buffer.get(x, 0).unwrap();
            assert_eq!(
                (cell.fg, cell.bg, cell.modifier),
                char_style(name),
                "{name} {x}"
            );
        }
        for x in [0, 1, 5, 6, 7] {
            assert_eq!(buffer.get(x, 0).unwrap().fg, None, "{name} {x}");
        }
    }
}

#[test]
fn put_str_writes_plain_cells_over_a_style() {
    // `put_str` draws no colors or modifier, as `Buffer::put_str`: the
    // style already in the cells it writes is replaced (#1620)
    use revue::render::{Cell, Modifier};
    let mut buffer = Buffer::new(6, 1);
    for x in 0..6 {
        buffer.set(x, 0, Cell::new('.').fg(Color::RED).bg(Color::BLUE).bold());
    }
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(1, 0, 4, 1));
        assert_eq!(ctx.put_str(0, 0, "a日"), 3);
    }
    assert_eq!(cells(&buffer, 0), ".a日_..");
    for x in 1..4 {
        let cell = buffer.get(x, 0).unwrap();
        assert_eq!(
            (cell.fg, cell.bg, cell.modifier),
            (None, None, Modifier::empty()),
            "{x}"
        );
    }
    for x in [0, 4, 5] {
        let cell = buffer.get(x, 0).unwrap();
        assert_eq!(
            (cell.fg, cell.bg),
            (Some(Color::RED), Some(Color::BLUE)),
            "{x}"
        );
    }
}
