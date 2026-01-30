//! Integration tests for RenderContext shape drawing methods

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;

#[test]
fn test_draw_hline() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 5,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_hline(2, 2, 5, '-', Color::RED);

    // Check characters were drawn
    for i in 0..5 {
        if let Some(cell) = buffer.get(2 + i, 2) {
            assert_eq!(cell.symbol, '-');
        } else {
            panic!("Expected cell at ({}, 2)", 2 + i);
        }
    }
}

#[test]
fn test_draw_hline_length_one() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 5,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_hline(5, 3, 1, 'x', Color::BLUE);

    if let Some(cell) = buffer.get(5, 3) {
        assert_eq!(cell.symbol, 'x');
    } else {
        panic!("Expected cell at (5, 3)");
    }
}

#[test]
fn test_draw_vline() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_vline(3, 2, 4, '|', Color::GREEN);

    // Check characters were drawn
    for i in 0..4 {
        if let Some(cell) = buffer.get(3, 2 + i) {
            assert_eq!(cell.symbol, '|');
        } else {
            panic!("Expected cell at (3, {})", 2 + i);
        }
    }
}

#[test]
fn test_draw_vline_length_one() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_vline(7, 5, 1, 'y', Color::YELLOW);

    if let Some(cell) = buffer.get(7, 5) {
        assert_eq!(cell.symbol, 'y');
    } else {
        panic!("Expected cell at (7, 5)");
    }
}

#[test]
fn test_draw_box_rounded() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_box_rounded(1, 1, 8, 6, Color::WHITE);

    // Check corners
    assert_eq!(buffer.get(1, 1).unwrap().symbol, '╭');
    assert_eq!(buffer.get(8, 1).unwrap().symbol, '╮');
    assert_eq!(buffer.get(1, 6).unwrap().symbol, '╰');
    assert_eq!(buffer.get(8, 6).unwrap().symbol, '╯');
}

#[test]
fn test_draw_box_rounded_min_size() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_box_rounded(2, 2, 2, 2, Color::CYAN);

    // Minimum size box - just corners
    assert_eq!(buffer.get(2, 2).unwrap().symbol, '╭');
    assert_eq!(buffer.get(3, 2).unwrap().symbol, '╮');
    assert_eq!(buffer.get(2, 3).unwrap().symbol, '╰');
    assert_eq!(buffer.get(3, 3).unwrap().symbol, '╯');
}

#[test]
fn test_draw_box_single() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_box_single(1, 1, 8, 6, Color::WHITE);

    // Check corners
    assert_eq!(buffer.get(1, 1).unwrap().symbol, '┌');
    assert_eq!(buffer.get(8, 1).unwrap().symbol, '┐');
    assert_eq!(buffer.get(1, 6).unwrap().symbol, '└');
    assert_eq!(buffer.get(8, 6).unwrap().symbol, '┘');
}

#[test]
fn test_draw_box_double() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_box_double(1, 1, 8, 6, Color::WHITE);

    // Check corners
    assert_eq!(buffer.get(1, 1).unwrap().symbol, '╔');
    assert_eq!(buffer.get(8, 1).unwrap().symbol, '╗');
    assert_eq!(buffer.get(1, 6).unwrap().symbol, '╚');
    assert_eq!(buffer.get(8, 6).unwrap().symbol, '╝');
}

#[test]
fn test_draw_box_no_top() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_box_no_top(1, 1, 8, 6, Color::WHITE);

    // Check bottom corners only (no top)
    assert_eq!(buffer.get(1, 6).unwrap().symbol, '╰');
    assert_eq!(buffer.get(8, 6).unwrap().symbol, '╯');

    // Check that top corners are NOT the rounded ones
    let top_left = buffer.get(1, 1).unwrap().symbol;
    let top_right = buffer.get(8, 1).unwrap().symbol;
    assert_ne!(top_left, '╭');
    assert_ne!(top_right, '╮');
}

#[test]
fn test_draw_box_titled() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 20,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_box_titled(2, 2, 15, 6, "Test", Color::WHITE);

    // Check corners
    assert_eq!(buffer.get(2, 2).unwrap().symbol, '╭');
    assert_eq!(buffer.get(16, 2).unwrap().symbol, '╮');
    assert_eq!(buffer.get(2, 7).unwrap().symbol, '╰');
    assert_eq!(buffer.get(16, 7).unwrap().symbol, '╯');
}

#[test]
fn test_draw_box_titled_single() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 20,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_box_titled_single(2, 2, 15, 6, "Test", Color::WHITE);

    // Check corners
    assert_eq!(buffer.get(2, 2).unwrap().symbol, '┌');
    assert_eq!(buffer.get(16, 2).unwrap().symbol, '┐');
    assert_eq!(buffer.get(2, 7).unwrap().symbol, '└');
    assert_eq!(buffer.get(16, 7).unwrap().symbol, '┘');
}

#[test]
fn test_draw_box_titled_double() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 20,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.draw_box_titled_double(2, 2, 15, 6, "Test", Color::WHITE);

    // Check corners
    assert_eq!(buffer.get(2, 2).unwrap().symbol, '╔');
    assert_eq!(buffer.get(16, 2).unwrap().symbol, '╗');
    assert_eq!(buffer.get(2, 7).unwrap().symbol, '╚');
    assert_eq!(buffer.get(16, 7).unwrap().symbol, '╝');
}

#[test]
fn test_draw_header_line() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect {
        x: 0,
        y: 0,
        width: 20,
        height: 5,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    let parts = &[("Test", Color::RED), ("Header", Color::GREEN)];
    ctx.draw_header_line(2, 1, 15, parts, Color::WHITE);

    // Should have left corner
    assert_eq!(buffer.get(2, 1).unwrap().symbol, '╭');
}

#[test]
fn test_draw_header_line_empty_parts() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect {
        x: 0,
        y: 0,
        width: 20,
        height: 5,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    let parts: &[(&str, Color)] = &[];
    ctx.draw_header_line(2, 1, 15, parts, Color::WHITE);

    // Should still draw corners and line
    assert_eq!(buffer.get(2, 1).unwrap().symbol, '╭');
    assert_eq!(buffer.get(16, 1).unwrap().symbol, '╮');
}

#[test]
fn test_fill() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.fill(2, 2, 4, 3, '*', Color::YELLOW);

    // Check filled area
    for dy in 0..3 {
        for dx in 0..4 {
            assert_eq!(buffer.get(2 + dx, 2 + dy).unwrap().symbol, '*');
        }
    }
}

#[test]
fn test_fill_single_cell() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.fill(5, 5, 1, 1, '@', Color::MAGENTA);

    assert_eq!(buffer.get(5, 5).unwrap().symbol, '@');
}

#[test]
fn test_fill_bg() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    ctx.fill_bg(2, 2, 4, 3, Color::BLUE);

    // Check filled area has blue background
    for dy in 0..3 {
        for dx in 0..4 {
            assert_eq!(buffer.get(2 + dx, 2 + dy).unwrap().bg, Some(Color::BLUE));
        }
    }
}

#[test]
fn test_clear() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect {
        x: 0,
        y: 0,
        width: 10,
        height: 10,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    // First draw something
    ctx.fill(2, 2, 4, 3, '*', Color::YELLOW);

    // Then clear
    ctx.clear(2, 2, 4, 3);

    // Check area is cleared
    for dy in 0..3 {
        for dx in 0..4 {
            assert_eq!(buffer.get(2 + dx, 2 + dy).unwrap().symbol, ' ');
        }
    }
}

#[test]
fn test_clear_entire_buffer() {
    let mut buffer = Buffer::new(5, 5);
    let area = Rect {
        x: 0,
        y: 0,
        width: 5,
        height: 5,
    };
    let mut ctx = RenderContext::new(&mut buffer, area);
    // Fill with content
    ctx.fill(0, 0, 5, 5, 'x', Color::WHITE);

    // Clear all
    ctx.clear(0, 0, 5, 5);

    // All should be spaces
    for y in 0..5 {
        for x in 0..5 {
            assert_eq!(buffer.get(x, y).unwrap().symbol, ' ');
        }
    }
}
