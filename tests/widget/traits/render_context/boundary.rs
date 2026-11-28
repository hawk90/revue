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
