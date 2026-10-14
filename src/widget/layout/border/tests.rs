use super::*;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::traits::RenderContext;
use crate::widget::Text;

#[test]
fn test_border_new() {
    let b = Border::new();
    assert_eq!(b.get_border_type(), BorderType::Single);
    assert!(b.get_title().is_none());
    assert!(!b.has_child());
}

#[test]
fn test_border_builder() {
    let b = Border::new()
        .border_type(BorderType::Double)
        .title("Title")
        .fg(Color::RED)
        .bg(Color::BLACK)
        .child(Text::new("Content"));
    assert_eq!(b.get_border_type(), BorderType::Double);
    assert_eq!(b.get_title(), Some("Title"));
    assert_eq!(b.get_fg(), Some(Color::RED));
    assert_eq!(b.get_bg(), Some(Color::BLACK));
    assert!(b.has_child());
}

#[test]
fn test_border_presets() {
    assert_eq!(Border::single().get_border_type(), BorderType::Single);
    assert_eq!(Border::double().get_border_type(), BorderType::Double);
    assert_eq!(Border::rounded().get_border_type(), BorderType::Rounded);
    assert_eq!(Border::thick().get_border_type(), BorderType::Thick);
    assert_eq!(Border::ascii().get_border_type(), BorderType::Ascii);
    assert_eq!(Border::panel().get_border_type(), BorderType::Double);
    assert_eq!(Border::card().get_border_type(), BorderType::Rounded);
    assert_eq!(Border::error_box().get_fg(), Some(Color::RED));
    assert_eq!(Border::success_box().get_fg(), Some(Color::GREEN));
}

#[test]
fn test_border_type_chars() {
    let chars = BorderType::Single.chars();
    assert_eq!(chars.top_left, '┌');
    assert_eq!(chars.horizontal, '─');

    let chars = BorderType::Rounded.chars();
    assert_eq!(chars.top_left, '╭');

    let chars = BorderType::None.chars();
    assert_eq!(chars.top_left, ' ');
}

#[test]
fn test_border_render_single() {
    let mut buf = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buf, area);
    let b = Border::new();
    b.render(&mut ctx);
    assert_eq!(buf.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buf.get(9, 0).unwrap().symbol, '┐');
    assert_eq!(buf.get(0, 4).unwrap().symbol, '└');
    assert_eq!(buf.get(9, 4).unwrap().symbol, '┘');
}

#[test]
fn test_border_render_with_child() {
    let mut buf = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buf, area);
    let b = Border::new().child(Text::new("Hi"));
    b.render(&mut ctx);
    // Border should be drawn
    assert_eq!(buf.get(0, 0).unwrap().symbol, '┌');
    // Child content inside border
    assert_eq!(buf.get(1, 1).unwrap().symbol, 'H');
    assert_eq!(buf.get(2, 1).unwrap().symbol, 'i');
}

#[test]
fn test_border_render_small_area_no_panic() {
    let mut buf = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buf, area);
    let b = Border::new().child(Text::new("X"));
    b.render(&mut ctx); // Width/height < 2, should return early

    // Neither the frame nor the child reaches the buffer.
    for y in 0..10 {
        for x in 0..10 {
            assert_eq!(buf.get(x, y).unwrap().symbol, ' ', "({x}, {y})");
        }
    }
}

#[test]
fn test_draw_border_utility() {
    let mut buf = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    draw_border(&mut buf, area, BorderType::Double, Some(Color::CYAN), None);
    assert_eq!(buf.get(0, 0).unwrap().symbol, '╔');
    assert_eq!(buf.get(9, 0).unwrap().symbol, '╗');
}

#[test]
fn test_draw_border_none_type() {
    let mut buf = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    draw_border(&mut buf, area, BorderType::None, None, None);
    // BorderType::None should not draw anything
    assert_eq!(buf.get(0, 0).unwrap().symbol, ' ');
}
