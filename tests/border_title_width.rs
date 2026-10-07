//! A border title is laid out in terminal columns, not bytes or chars.
//!
//! A wide glyph (emoji, CJK) spans two cells and a variation selector spans
//! none; the title must take exactly the cells it shows, and the border line
//! must resume right after it.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::Border;

/// Read a buffer row as the terminal shows it: a wide glyph is one `char`
/// spanning two cells, so its continuation cell contributes nothing.
fn row_text(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .filter_map(|x| buffer.get(x, y))
        .filter(|cell| !cell.is_continuation())
        .map(|cell| cell.symbol)
        .collect()
}

fn render_titled(width: u16, title: &str) -> Buffer {
    let mut buffer = Buffer::new(width, 3);
    let area = Rect::new(0, 0, width, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    Border::single().title(title).render(&mut ctx);
    buffer
}

#[test]
fn emoji_title_resumes_border_right_after_glyph() {
    let buffer = render_titled(20, "🔄Counter");

    // 🔄 takes columns 2-3, "Counter" 4-10, and the line picks up at 11.
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '🔄');
    assert!(buffer.get(3, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'C');
    assert_eq!(buffer.get(11, 0).unwrap().symbol, '─');
    assert_eq!(row_text(&buffer, 0), "┌─🔄Counter────────┐");
}

#[test]
fn vs16_title_adds_no_cells() {
    // U+2699 U+FE0F: the variation selector is zero-width and must not take
    // a cell of its own.
    let buffer = render_titled(20, "⚙\u{FE0F}Settings");

    assert_eq!(buffer.get(2, 0).unwrap().symbol, '⚙');
    assert!(buffer.get(3, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'S');
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '─');
    assert_eq!(row_text(&buffer, 0), "┌─⚙Settings───────┐");
}

#[test]
fn wide_title_truncates_by_columns() {
    // Width 10 leaves 6 title columns: three CJK glyphs, not six.
    let buffer = render_titled(10, "日本語テキスト");
    assert_eq!(row_text(&buffer, 0), "┌─日本語─┐");

    // A wide glyph that would straddle the limit is dropped whole.
    let buffer = render_titled(10, "a日本語");
    assert_eq!(row_text(&buffer, 0), "┌─a日本──┐");
}

#[test]
fn ascii_title_unchanged() {
    let buffer = render_titled(20, "Panel");
    assert_eq!(row_text(&buffer, 0), "┌─Panel────────────┐");
}

/// Left and right titles run down the edge one glyph per row. A zero-width
/// char (VS16) takes no row, the same as it takes no column on the top edge.
mod side_titles {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::utils::border::{draw_border_title, BorderTitle};
    use revue::widget::traits::RenderContext;

    fn column(title: BorderTitle, height: u16) -> String {
        let mut buffer = Buffer::new(4, height);
        let area = Rect::new(0, 0, 4, height);
        let mut ctx = RenderContext::new(&mut buffer, area);
        draw_border_title(&mut ctx, area, &title);
        (0..height)
            .map(|y| buffer.get(0, y).unwrap().symbol)
            .collect()
    }

    #[test]
    fn vs16_takes_no_row() {
        // Three glyphs fit the three rows between the corners
        let col = column(BorderTitle::new("⚙\u{FE0F}ab").padding(0).left(), 5);
        assert_eq!(col, " ⚙ab ");
    }

    #[test]
    fn end_position_measures_glyph_rows() {
        let col = column(BorderTitle::new("⚙\u{FE0F}ab").padding(0).left().end(), 6);
        assert_eq!(col, "  ⚙ab ");
    }
}
