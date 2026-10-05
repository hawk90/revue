//! Canvas and BrailleCanvas widget tests
//!
//! Basic rendering through canvas()/braille_canvas() is covered in
//! tests/canvas/integration.rs; these cover the area handling and the
//! constructors not used there.

use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{BrailleCanvas, Canvas};

fn render(view: &impl View, buffer: &mut Buffer, area: Rect) {
    let mut ctx = RenderContext::new(buffer, area);
    view.render(&mut ctx);
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
        .collect()
}

#[test]
fn test_canvas_draws_relative_to_area() {
    let c = Canvas::new(|ctx| {
        assert_eq!(ctx.area(), Rect::new(3, 1, 4, 2));
        ctx.text(0, 0, "abcdef", None);
        ctx.set(0, 1, 'X');
    });
    let mut buffer = Buffer::new(10, 4);
    render(&c, &mut buffer, Rect::new(3, 1, 4, 2));
    assert_eq!(row(&buffer, 0), "          ");
    assert_eq!(row(&buffer, 1), "   abcd   ");
    assert_eq!(row(&buffer, 2), "   X      ");
}

#[test]
fn test_canvas_closure_capture() {
    let text = String::from("captured");
    let c = Canvas::new(move |ctx| ctx.text(0, 0, &text, Some(Color::CYAN)));
    let mut buffer = Buffer::new(10, 1);
    render(&c, &mut buffer, Rect::new(0, 0, 10, 1));
    assert_eq!(row(&buffer, 0), "captured  ");
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_canvas_rerenders_each_time() {
    let c = Canvas::new(|ctx| ctx.set(1, 0, '*'));
    let mut buffer = Buffer::new(3, 1);
    render(&c, &mut buffer, Rect::new(0, 0, 3, 1));
    buffer.set(1, 0, Cell::new(' '));
    render(&c, &mut buffer, Rect::new(0, 0, 3, 1));
    assert_eq!(row(&buffer, 0), " * ");
}

#[test]
fn test_canvas_clear_only_its_area() {
    let c = Canvas::new(|ctx| ctx.clear());
    let mut buffer = Buffer::new(6, 1);
    for x in 0..6 {
        buffer.set(x, 0, Cell::new('#'));
    }
    render(&c, &mut buffer, Rect::new(2, 0, 2, 1));
    assert_eq!(row(&buffer, 0), "##  ##");
}

#[test]
fn test_braille_canvas_new() {
    let bc = BrailleCanvas::new(|ctx| ctx.set(0, 0, Color::RED));
    let mut buffer = Buffer::new(2, 1);
    render(&bc, &mut buffer, Rect::new(0, 0, 2, 1));
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, '\u{2801}');
    assert_eq!(cell.fg, Some(Color::RED));
}

#[test]
fn test_braille_canvas_sized_from_area() {
    let bc = BrailleCanvas::new(|ctx| {
        // Two dots across and four down per terminal cell
        assert_eq!(ctx.width(), 10);
        assert_eq!(ctx.height(), 12);
        ctx.set(9, 11, Color::GREEN);
    });
    let mut buffer = Buffer::new(8, 4);
    render(&bc, &mut buffer, Rect::new(2, 1, 5, 3));
    // Bottom-right dot of the area's last cell
    assert_eq!(buffer.get(6, 3).unwrap().symbol, '\u{2880}');
}

#[test]
fn test_braille_canvas_leaves_empty_cells_alone() {
    // Cells with no dots are not written, so what is underneath shows through
    let bc = BrailleCanvas::new(|ctx| ctx.set(0, 0, Color::WHITE));
    let mut buffer = Buffer::new(3, 1);
    buffer.set(2, 0, Cell::new('#'));
    render(&bc, &mut buffer, Rect::new(0, 0, 3, 1));
    assert_eq!(row(&buffer, 0), "\u{2801} #");
}

#[test]
fn test_braille_canvas_clear() {
    let bc = BrailleCanvas::new(|ctx| {
        ctx.set(0, 0, Color::WHITE);
        ctx.clear();
        ctx.set(3, 0, Color::BLUE);
    });
    let mut buffer = Buffer::new(2, 1);
    render(&bc, &mut buffer, Rect::new(0, 0, 2, 1));
    assert_eq!(row(&buffer, 0), " \u{2808}");
}

#[test]
fn test_braille_canvas_closure_capture() {
    let points = vec![(0usize, 0usize), (1, 1), (2, 2), (3, 3)];
    let bc = BrailleCanvas::new(move |ctx| {
        for &(x, y) in &points {
            ctx.set(x, y, Color::YELLOW);
        }
    });
    let mut buffer = Buffer::new(2, 1);
    render(&bc, &mut buffer, Rect::new(0, 0, 2, 1));
    // (0,0)+(1,1) in the first cell, (2,2)+(3,3) in the second
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '\u{2811}');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '\u{2884}');
}
