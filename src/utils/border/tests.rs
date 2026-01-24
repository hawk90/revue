use super::*;
use crate::render::Buffer;

#[test]
fn test_border_chars() {
    assert_eq!(BorderChars::SINGLE.top_left, '┌');
    assert_eq!(BorderChars::ROUNDED.top_left, '╭');
    assert_eq!(BorderChars::DOUBLE.top_left, '╔');
}

#[test]
fn test_border_style() {
    let style = BorderStyle::new(Color::WHITE).rounded();
    assert_eq!(style.chars.top_left, '╭');
}

#[test]
fn test_render_border() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    render_border(&mut ctx, area, Color::WHITE);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '┐');
    assert_eq!(buffer.get(0, 4).unwrap().symbol, '└');
    assert_eq!(buffer.get(9, 4).unwrap().symbol, '┘');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '─');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '│');
}

#[test]
fn test_render_rounded_border() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    render_rounded_border(&mut ctx, area, Color::WHITE);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '╮');
}

#[test]
fn test_fill_bg() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    fill_bg(&mut ctx, area, Color::RED);

    assert_eq!(buffer.get(5, 2).unwrap().bg, Some(Color::RED));
}

#[test]
fn test_draw_separator() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    render_border(&mut ctx, area, Color::WHITE);
    draw_separator(&mut ctx, area, 2, Color::WHITE);

    assert_eq!(buffer.get(0, 2).unwrap().symbol, '├');
    assert_eq!(buffer.get(9, 2).unwrap().symbol, '┤');
}

#[test]
fn test_border_title_builder() {
    let title = BorderTitle::new("Test")
        .top()
        .start()
        .fg(Color::BLUE)
        .padding(2);

    assert_eq!(title.text, "Test");
    assert_eq!(title.edge, BorderEdge::Top);
    assert_eq!(title.position, TitlePosition::Start);
    assert_eq!(title.fg, Some(Color::BLUE));
    assert_eq!(title.pad_start, 2);
    assert_eq!(title.pad_end, 2);
}

#[test]
fn test_border_title_width() {
    let title = BorderTitle::new("Hello").padding(1);
    assert_eq!(title.width(), 7); // 5 + 1 + 1
}

#[test]
fn test_draw_border_title_start() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    render_border(&mut ctx, area, Color::WHITE);
    draw_border_title(&mut ctx, area, &BorderTitle::new("Title").padding(1));

    // Should have space, T, i, t, l, e, space at positions 1-7
    assert_eq!(buffer.get(1, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'T');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 'i');
    assert_eq!(buffer.get(6, 0).unwrap().symbol, 'e');
    assert_eq!(buffer.get(7, 0).unwrap().symbol, ' ');
}

#[test]
fn test_draw_border_title_end() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    render_border(&mut ctx, area, Color::WHITE);
    draw_border_title(&mut ctx, area, &BorderTitle::new("End").end().padding(1));

    // "End" with padding = 5 chars total, position = 20 - 1 - 5 = 14
    // 14: pad_start, 15: E, 16: n, 17: d, 18: pad_end
    assert_eq!(buffer.get(14, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(15, 0).unwrap().symbol, 'E');
    assert_eq!(buffer.get(16, 0).unwrap().symbol, 'n');
    assert_eq!(buffer.get(17, 0).unwrap().symbol, 'd');
}

#[test]
fn test_draw_border_title_center() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    render_border(&mut ctx, area, Color::WHITE);
    draw_border_title(&mut ctx, area, &BorderTitle::new("Hi").center().padding(1));

    // "Hi" with padding = 4 chars, available = 18, should be centered
    // Center position = 1 + (18 - 4) / 2 = 1 + 7 = 8
    assert_eq!(buffer.get(9, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(10, 0).unwrap().symbol, 'i');
}

#[test]
fn test_draw_border_title_bottom() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    render_border(&mut ctx, area, Color::WHITE);
    draw_border_title(
        &mut ctx,
        area,
        &BorderTitle::new("Bottom").bottom().padding(1),
    );

    // Should be on bottom border (y = 4)
    assert_eq!(buffer.get(2, 4).unwrap().symbol, 'B');
    assert_eq!(buffer.get(7, 4).unwrap().symbol, 'm');
}

#[test]
fn test_draw_multiple_titles() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    render_border(&mut ctx, area, Color::WHITE);
    draw_border_titles(
        &mut ctx,
        area,
        &[
            BorderTitle::new("Left").start(),
            BorderTitle::new("Right").end(),
        ],
    );

    // "Left" at start: pos 1 pad, 2 L, 3 e, 4 f, 5 t, 6 pad
    // "Right" at end: 30 - 1 - 7 = 22 pad, 23 R, 24 i, 25 g, 26 h, 27 t, 28 pad
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'L');
    assert_eq!(buffer.get(23, 0).unwrap().symbol, 'R');
}

#[test]
fn test_draw_title_convenience() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    render_border(&mut ctx, area, Color::WHITE);
    draw_title(&mut ctx, area, "Test", Color::BLUE);

    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'T');
    assert_eq!(buffer.get(2, 0).unwrap().fg, Some(Color::BLUE));
}
