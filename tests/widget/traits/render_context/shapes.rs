//! RenderContext shape drawing tests
//!
//! Lines, boxes, fill/clear and titled boxes with an ASCII title are
//! covered by tests/shapes_tests.rs and the in-source tests in
//! src/widget/traits/render_context/tests.rs. These cover titles that are
//! empty, wide or too long, and zero-sized shapes.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

fn draw(width: u16, height: u16, f: impl FnOnce(&mut RenderContext)) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
        f(&mut ctx);
    }
    buffer
}

#[test]
fn test_draw_box_titled_empty_title_is_a_plain_border() {
    let buffer = draw(6, 3, |ctx| {
        ctx.draw_box_titled(0, 0, 6, 3, "", Color::WHITE)
    });
    assert_eq!(row(&buffer, 0), "╭────╮");
    assert_eq!(row(&buffer, 1), "│    │");
    assert_eq!(row(&buffer, 2), "╰────╯");
}

#[test]
fn test_draw_box_titled_title_starts_after_one_border_cell() {
    let buffer = draw(10, 3, |ctx| {
        ctx.draw_box_titled(0, 0, 10, 3, "Ab", Color::WHITE)
    });
    assert_eq!(row(&buffer, 0), "╭─Ab─────╮");
}

#[test]
fn test_draw_box_titled_wide_title() {
    let buffer = draw(10, 3, |ctx| {
        ctx.draw_box_titled(0, 0, 10, 3, "标题", Color::WHITE)
    });
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '─');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '标');
    assert!(buffer.get(3, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '题');
    assert!(buffer.get(5, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '─');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '╮');
}

#[test]
fn test_draw_box_titled_long_title_is_cut_before_the_corner() {
    let buffer = draw(8, 3, |ctx| {
        ctx.draw_box_titled_single(0, 0, 8, 3, "Very long title", Color::WHITE)
    });
    assert_eq!(row(&buffer, 0), "┌─Very ┐");
}

#[test]
fn test_draw_box_titled_wide_char_that_does_not_fit_is_replaced_by_border() {
    // Width 6: title cells are 2..=4; '标' fits at 2-3, '题' would need 4-5
    let buffer = draw(6, 3, |ctx| {
        ctx.draw_box_titled_double(0, 0, 6, 3, "标题", Color::WHITE)
    });
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╔');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '═');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '标');
    assert!(buffer.get(3, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '═');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '╗');
}

#[test]
fn test_zero_sized_shapes_draw_nothing() {
    let buffer = draw(6, 6, |ctx| {
        ctx.fill(1, 1, 0, 0, '*', Color::WHITE);
        ctx.fill(1, 1, 3, 0, '*', Color::WHITE);
        ctx.fill(1, 1, 0, 3, '*', Color::WHITE);
        ctx.fill_bg(1, 1, 0, 0, Color::BLUE);
        ctx.draw_hline(1, 1, 0, '-', Color::WHITE);
        ctx.draw_vline(1, 1, 0, '|', Color::WHITE);
    });
    for y in 0..6 {
        for x in 0..6 {
            let cell = buffer.get(x, y).unwrap();
            assert_eq!(cell.symbol, ' ');
            assert!(cell.bg.is_none());
        }
    }
}

#[test]
fn test_zero_sized_clear_leaves_content() {
    let buffer = draw(4, 2, |ctx| {
        ctx.fill(0, 0, 4, 2, '#', Color::WHITE);
        ctx.clear(0, 0, 0, 0);
    });
    assert_eq!(row(&buffer, 0), "####");
    assert_eq!(row(&buffer, 1), "####");
}
