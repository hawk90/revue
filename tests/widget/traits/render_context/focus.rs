//! RenderContext focus indicator tests
//!
//! The Solid/Rounded/Double ring corners, a plain underline and marker and
//! a single-cell invert are covered by the in-source tests in
//! src/widget/traits/render_context/tests.rs.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{FocusStyle, RenderContext};

fn symbol(buffer: &Buffer, x: u16, y: u16) -> char {
    buffer.get(x, y).unwrap().symbol
}

fn blank(buffer: &Buffer) -> bool {
    (0..buffer.height()).all(|y| (0..buffer.width()).all(|x| symbol(buffer, x, y) == ' '))
}

fn ring(style: FocusStyle) -> Buffer {
    let mut buffer = Buffer::new(5, 4);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 5, 4));
        ctx.draw_focus_ring(0, 0, 5, 4, Color::CYAN, style);
    }
    buffer
}

/// (top-left, top-right, bottom-left, bottom-right, horizontal, vertical)
fn ring_chars(buffer: &Buffer) -> (char, char, char, char, char, char) {
    (
        symbol(buffer, 0, 0),
        symbol(buffer, 4, 0),
        symbol(buffer, 0, 3),
        symbol(buffer, 4, 3),
        symbol(buffer, 2, 0),
        symbol(buffer, 0, 1),
    )
}

#[test]
fn test_focus_ring_styles() {
    assert_eq!(
        ring_chars(&ring(FocusStyle::Solid)),
        ('┌', '┐', '└', '┘', '─', '│')
    );
    assert_eq!(
        ring_chars(&ring(FocusStyle::Rounded)),
        ('╭', '╮', '╰', '╯', '─', '│')
    );
    assert_eq!(
        ring_chars(&ring(FocusStyle::Double)),
        ('╔', '╗', '╚', '╝', '═', '║')
    );
    assert_eq!(
        ring_chars(&ring(FocusStyle::Dotted)),
        ('┌', '┐', '└', '┘', '╌', '╎')
    );
    assert_eq!(
        ring_chars(&ring(FocusStyle::Bold)),
        ('┏', '┓', '┗', '┛', '━', '┃')
    );
    assert_eq!(
        ring_chars(&ring(FocusStyle::Ascii)),
        ('+', '+', '+', '+', '-', '|')
    );
}

#[test]
fn test_focus_ring_draws_full_border_and_leaves_inside_alone() {
    let buffer = ring(FocusStyle::Solid);
    for x in 1..4 {
        assert_eq!(symbol(&buffer, x, 0), '─');
        assert_eq!(symbol(&buffer, x, 3), '─');
    }
    for y in 1..3 {
        assert_eq!(symbol(&buffer, 0, y), '│');
        assert_eq!(symbol(&buffer, 4, y), '│');
        for x in 1..4 {
            assert_eq!(symbol(&buffer, x, y), ' ');
        }
    }
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_focus_ring_minimum_size_is_corners_only() {
    let mut buffer = Buffer::new(4, 4);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 4, 4));
        ctx.draw_focus_ring(0, 0, 2, 2, Color::CYAN, FocusStyle::Solid);
    }
    assert_eq!(symbol(&buffer, 0, 0), '┌');
    assert_eq!(symbol(&buffer, 1, 0), '┐');
    assert_eq!(symbol(&buffer, 0, 1), '└');
    assert_eq!(symbol(&buffer, 1, 1), '┘');
    assert_eq!(symbol(&buffer, 2, 0), ' ');
}

#[test]
fn test_focus_ring_too_narrow_or_short_draws_nothing() {
    let mut buffer = Buffer::new(10, 10);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
        ctx.draw_focus_ring(0, 0, 1, 5, Color::CYAN, FocusStyle::Solid);
        ctx.draw_focus_ring(0, 0, 5, 1, Color::CYAN, FocusStyle::Solid);
    }
    assert!(blank(&buffer));
}

#[test]
fn test_focus_ring_is_relative_to_the_area() {
    let mut buffer = Buffer::new(20, 20);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(3, 2, 15, 15));
        ctx.draw_focus_ring(5, 5, 8, 6, Color::CYAN, FocusStyle::Solid);
    }
    assert_eq!(symbol(&buffer, 8, 7), '┌');
    assert_eq!(symbol(&buffer, 15, 7), '┐');
    assert_eq!(symbol(&buffer, 8, 12), '└');
    assert_eq!(symbol(&buffer, 15, 12), '┘');
}

#[test]
fn test_focus_ring_auto_is_rounded() {
    let mut buffer = Buffer::new(5, 4);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 5, 4));
        ctx.draw_focus_ring_auto(0, 0, 5, 4, Color::CYAN);
    }
    assert_eq!(ring_chars(&buffer), ring_chars(&ring(FocusStyle::Rounded)));
}

#[test]
fn test_focus_underline_width_and_offset() {
    let mut buffer = Buffer::new(20, 2);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 2));
        ctx.draw_focus_underline(10, 1, 3, Color::CYAN);
        ctx.draw_focus_underline(0, 0, 0, Color::CYAN);
    }
    assert_eq!(symbol(&buffer, 9, 1), ' ');
    for x in 10..13 {
        assert_eq!(symbol(&buffer, x, 1), '▔');
        assert_eq!(buffer.get(x, 1).unwrap().fg, Some(Color::CYAN));
    }
    assert_eq!(symbol(&buffer, 13, 1), ' ');
    // Zero width draws nothing
    assert!((0..20).all(|x| symbol(&buffer, x, 0) == ' '));
}

#[test]
fn test_focus_marker_offset() {
    let mut buffer = Buffer::new(20, 20);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(2, 3, 15, 15));
        ctx.draw_focus_marker(10, 10, Color::CYAN);
    }
    assert_eq!(symbol(&buffer, 12, 13), '▶');
    assert_eq!(buffer.get(12, 13).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_focus_marker_left_at_buffer_edge_draws_inside() {
    let mut buffer = Buffer::new(10, 10);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 10, 10));
        ctx.draw_focus_marker_left(5, Color::CYAN);
    }
    assert_eq!(symbol(&buffer, 0, 5), '▶');
}

#[test]
fn test_focus_marker_left_draws_just_outside_the_area() {
    let mut buffer = Buffer::new(20, 10);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(5, 2, 15, 8));
        ctx.draw_focus_marker_left(3, Color::CYAN);
    }
    assert_eq!(symbol(&buffer, 4, 5), '▶');
    assert_eq!(symbol(&buffer, 5, 5), ' ');
}

#[test]
fn test_invert_colors_region() {
    let fg = Color::WHITE;
    let bg = Color::BLUE;
    let mut buffer = Buffer::new(6, 6);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 6, 6));
        for y in 0..6 {
            ctx.draw_text_bg(0, y, "abcdef", fg, bg);
        }
        ctx.invert_colors(1, 1, 3, 2);
        // Empty region changes nothing
        ctx.invert_colors(0, 0, 0, 0);
    }
    for y in 0..6 {
        for x in 0..6 {
            let cell = buffer.get(x, y).unwrap();
            let inside = (1..4).contains(&x) && (1..3).contains(&y);
            if inside {
                assert_eq!((cell.fg, cell.bg), (Some(bg), Some(fg)), "({x},{y})");
            } else {
                assert_eq!((cell.fg, cell.bg), (Some(fg), Some(bg)), "({x},{y})");
            }
        }
    }
}

#[test]
fn test_draw_focus_reverse_inverts_relative_to_the_area() {
    let fg = Color::WHITE;
    let bg = Color::BLUE;
    let mut buffer = Buffer::new(10, 10);
    {
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(2, 2, 8, 8));
        ctx.draw_text_bg(0, 0, "xyz", fg, bg);
        ctx.draw_focus_reverse(1, 0, 2, 1);
    }
    assert_eq!(buffer.get(2, 2).unwrap().bg, Some(bg));
    assert_eq!(buffer.get(3, 2).unwrap().bg, Some(fg));
    assert_eq!(buffer.get(4, 2).unwrap().bg, Some(fg));
    assert_eq!(buffer.get(3, 2).unwrap().fg, Some(bg));
}
