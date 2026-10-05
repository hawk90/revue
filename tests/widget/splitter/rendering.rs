//! Splitter divider placement and rendering

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{pane, splitter, Splitter, SplitterStyle, View};

fn render(s: &Splitter, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    s.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16, w: u16) -> String {
    (0..w)
        .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
        .collect()
}

#[test]
fn test_splitter_panes_and_dividers_fill_the_area() {
    let s = splitter().pane(pane("a")).pane(pane("b")).pane(pane("c"));
    let area = Rect::new(3, 0, 50, 10);
    let areas = s.pane_areas(area);
    assert_eq!(areas.len(), 3);
    // Each pane starts one column after the previous pane's divider ...
    for pair in areas.windows(2) {
        let (prev, next) = (&pair[0].1, &pair[1].1);
        assert_eq!(prev.x + prev.width + 1, next.x);
    }
    // ... and the last pane reaches the right edge of the area.
    let last = areas[2].1;
    assert_eq!(last.x + last.width, area.x + area.width);
}

#[test]
fn test_splitter_vertical_last_pane_reaches_bottom() {
    let s = splitter().vertical().pane(pane("top")).pane(pane("bottom"));
    let areas = s.pane_areas(Rect::new(0, 2, 10, 21));
    assert_eq!(areas[0].1, Rect::new(0, 2, 10, 10));
    assert_eq!(areas[1].1, Rect::new(0, 13, 10, 10));
}

#[test]
fn test_splitter_renders_horizontal_divider() {
    let s = splitter().pane(pane("a")).pane(pane("b"));
    let buffer = render(&s, 21, 2);
    let expected = format!("{}│{}", " ".repeat(10), " ".repeat(10));
    assert_eq!(row(&buffer, 0, 21), expected);
    assert_eq!(row(&buffer, 1, 21), expected);
}

#[test]
fn test_splitter_renders_vertical_divider() {
    let s = splitter().vertical().pane(pane("a")).pane(pane("b"));
    let buffer = render(&s, 4, 11);
    assert_eq!(row(&buffer, 4, 4), "    ");
    assert_eq!(row(&buffer, 5, 4), "────");
    assert_eq!(row(&buffer, 6, 4), "    ");
}

#[test]
fn test_splitter_divider_styles() {
    let cases = [
        (SplitterStyle::Double, '║'),
        (SplitterStyle::Thick, '┃'),
        (SplitterStyle::Hidden, ' '),
    ];
    for (style, ch) in cases {
        let s = splitter().style(style).pane(pane("a")).pane(pane("b"));
        let buffer = render(&s, 21, 1);
        assert_eq!(buffer.get(10, 0).unwrap().symbol, ch, "{style:?}");
    }
}

#[test]
fn test_splitter_active_divider_uses_active_color() {
    let mut s = splitter()
        .color(Color::RED)
        .active_color(Color::GREEN)
        .pane(pane("a"))
        .pane(pane("b"))
        .pane(pane("c"));
    s.start_resize(1);
    let areas = s.pane_areas(Rect::new(0, 0, 32, 1));
    let first = areas[0].1.x + areas[0].1.width;
    let second = areas[1].1.x + areas[1].1.width;

    let buffer = render(&s, 32, 1);
    assert_eq!(buffer.get(first, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(second, 0).unwrap().fg, Some(Color::GREEN));

    s.stop_resize();
    let buffer = render(&s, 32, 1);
    assert_eq!(buffer.get(second, 0).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_splitter_single_pane_draws_no_divider() {
    let s = splitter().pane(pane("only"));
    let buffer = render(&s, 10, 1);
    assert_eq!(row(&buffer, 0, 10), " ".repeat(10));
}
