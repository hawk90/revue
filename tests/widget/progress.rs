//! Progress widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{progress, Progress, ProgressStyle, View};

// ==================== Constructor Tests ====================

#[test]
fn test_progress_new() {
    let p = Progress::new(0.5);
    assert!((p.value() - 0.5).abs() < f32::EPSILON);
}

#[test]
fn test_progress_new_zero() {
    let p = Progress::new(0.0);
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_progress_new_one() {
    let p = Progress::new(1.0);
    assert_eq!(p.value(), 1.0);
}

#[test]
fn test_progress_default() {
    let p = Progress::default();
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_progress_helper() {
    let p = progress(0.75);
    assert!((p.value() - 0.75).abs() < f32::EPSILON);
}

// ==================== Clamping Tests ====================

#[test]
fn test_progress_clamp_negative() {
    let p1 = Progress::new(-0.5);
    assert!((p1.value() - 0.0).abs() < f32::EPSILON);

    let p2 = Progress::new(-1.0);
    assert_eq!(p2.value(), 0.0);

    let p3 = Progress::new(-999.0);
    assert_eq!(p3.value(), 0.0);
}

#[test]
fn test_progress_clamp_above_one() {
    let p1 = Progress::new(1.5);
    assert!((p1.value() - 1.0).abs() < f32::EPSILON);

    let p2 = Progress::new(2.0);
    assert_eq!(p2.value(), 1.0);

    let p3 = Progress::new(999.0);
    assert_eq!(p3.value(), 1.0);
}

#[test]
fn test_progress_boundary_values() {
    let p1 = Progress::new(0.0);
    assert_eq!(p1.value(), 0.0);

    let p2 = Progress::new(1.0);
    assert_eq!(p2.value(), 1.0);

    let p3 = Progress::new(0.5);
    assert!((p3.value() - 0.5).abs() < f32::EPSILON);
}

// ==================== Builder Tests ====================

#[test]
fn test_progress_builder() {
    let p = Progress::new(0.5).style(ProgressStyle::Line);
    assert!((p.value() - 0.5).abs() < f32::EPSILON);
}

#[test]
fn test_progress_filled_color() {
    let _p = Progress::new(0.5).filled_color(Color::CYAN);
    // Private field - verify it compiles
}

#[test]
fn test_progress_empty_color() {
    let _p = Progress::new(0.5).empty_color(Color::rgb(128, 128, 128));
    // Private field - verify it compiles
}

#[test]
fn test_progress_show_percentage() {
    let _p = Progress::new(0.5).show_percentage(true);
    // Private field - verify it compiles
}

#[test]
fn test_progress_builder_chain() {
    let p = Progress::new(0.75)
        .style(ProgressStyle::Braille)
        .filled_color(Color::GREEN)
        .empty_color(Color::rgb(128, 128, 128))
        .show_percentage(true);

    assert!((p.value() - 0.75).abs() < f32::EPSILON);
}

// ==================== Getter/Setter Tests ====================

#[test]
fn test_progress_value() {
    let p = Progress::new(0.5);
    assert!((p.value() - 0.5).abs() < f32::EPSILON);
}

#[test]
fn test_progress_set() {
    let mut p = Progress::new(0.0);
    p.set_progress(0.75);
    assert!((p.value() - 0.75).abs() < f32::EPSILON);
}

#[test]
fn test_progress_set_clamps_negative() {
    let mut p = Progress::new(0.5);
    p.set_progress(-0.5);
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_progress_set_clamps_above_one() {
    let mut p = Progress::new(0.5);
    p.set_progress(1.5);
    assert_eq!(p.value(), 1.0);
}

#[test]
fn test_progress_progress_builder() {
    let p = Progress::new(0.0).progress(0.5);
    assert!((p.value() - 0.5).abs() < f32::EPSILON);
}

#[test]
fn test_progress_progress_builder_clamps() {
    let p1 = Progress::new(0.0).progress(-0.5);
    assert_eq!(p1.value(), 0.0);

    let p2 = Progress::new(0.0).progress(1.5);
    assert_eq!(p2.value(), 1.0);
}

// ==================== Style Tests ====================

#[test]
fn test_progress_style_default() {
    let style = ProgressStyle::default();
    assert_eq!(style, ProgressStyle::Block);
}

#[test]
fn test_progress_style_block() {
    let style = ProgressStyle::Block;
    assert_eq!(style, ProgressStyle::Block);
}

#[test]
fn test_progress_style_line() {
    let style = ProgressStyle::Line;
    assert_eq!(style, ProgressStyle::Line);
}

#[test]
fn test_progress_style_ascii() {
    let style = ProgressStyle::Ascii;
    assert_eq!(style, ProgressStyle::Ascii);
}

#[test]
fn test_progress_style_braille() {
    let style = ProgressStyle::Braille;
    assert_eq!(style, ProgressStyle::Braille);
}

#[test]
fn test_progress_style_all_variants() {
    let _ = ProgressStyle::Block;
    let _ = ProgressStyle::Line;
    let _ = ProgressStyle::Ascii;
    let _ = ProgressStyle::Braille;
}

// ==================== Rendering Tests ====================

#[test]
fn test_progress_render_zero() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.0);
    p.render(&mut ctx);

    for x in 0..10 {
        assert_eq!(buffer.get(x, 0).unwrap().symbol, '░');
    }
}

#[test]
fn test_progress_render_quarter() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.25);
    p.render(&mut ctx);

    // 25% of 10 chars = 2.5, rounds to 3 filled
    assert!(buffer.get(0, 0).unwrap().symbol == '█');
    assert!(buffer.get(2, 0).unwrap().symbol == '█');
    assert!(buffer.get(3, 0).unwrap().symbol == '░');
}

#[test]
fn test_progress_render_half() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5);
    p.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '░');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '░');
}

#[test]
fn test_progress_render_three_quarters() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.75);
    p.render(&mut ctx);

    assert_eq!(buffer.get(7, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(8, 0).unwrap().symbol, '░');
}

#[test]
fn test_progress_render_full() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(1.0);
    p.render(&mut ctx);

    for x in 0..10 {
        assert_eq!(buffer.get(x, 0).unwrap().symbol, '█');
    }
}

#[test]
fn test_progress_render_empty() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.0);
    p.render(&mut ctx);

    for x in 0..10 {
        assert_eq!(buffer.get(x, 0).unwrap().symbol, '░');
    }
}

#[test]
fn test_progress_render_line_style() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5).style(ProgressStyle::Line);
    p.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '━');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '━');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '─');
}

#[test]
fn test_progress_render_ascii_style() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5).style(ProgressStyle::Ascii);
    p.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '#');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '-');
}

#[test]
fn test_progress_render_braille_style() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5).style(ProgressStyle::Braille);
    p.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⣿');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '⡀');
}

#[test]
fn test_progress_with_percentage() {
    let mut buffer = Buffer::new(15, 1);
    let area = Rect::new(0, 0, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5).show_percentage(true);
    p.render(&mut ctx);

    assert_eq!(buffer.get(11, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '5');
    assert_eq!(buffer.get(13, 0).unwrap().symbol, '0');
    assert_eq!(buffer.get(14, 0).unwrap().symbol, '%');
}

#[test]
fn test_progress_with_percentage_zero() {
    let mut buffer = Buffer::new(15, 1);
    let area = Rect::new(0, 0, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.0).show_percentage(true);
    p.render(&mut ctx);

    // Percentage "  0%" starts at x=11 (after 10-char bar + 1 space)
    assert_eq!(buffer.get(11, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(12, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(13, 0).unwrap().symbol, '0');
    assert_eq!(buffer.get(14, 0).unwrap().symbol, '%');
}

#[test]
fn test_progress_with_percentage_full() {
    let mut buffer = Buffer::new(15, 1);
    let area = Rect::new(0, 0, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(1.0).show_percentage(true);
    p.render(&mut ctx);

    // Percentage "100%" starts at x=11 (after 10-char bar + 1 space)
    assert_eq!(buffer.get(11, 0).unwrap().symbol, '1');
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '0');
    assert_eq!(buffer.get(13, 0).unwrap().symbol, '0');
    assert_eq!(buffer.get(14, 0).unwrap().symbol, '%');
}

#[test]
fn test_progress_render_zero_width() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 0, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5);
    p.render(&mut ctx);
    // Should handle zero width gracefully
}

#[test]
fn test_progress_render_zero_height() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5);
    p.render(&mut ctx);
    // Should handle zero height gracefully
}

// ==================== Edge Cases ====================

#[test]
fn test_progress_small_values() {
    let p = Progress::new(0.001);
    assert!((p.value() - 0.001).abs() < f32::EPSILON);
}

#[test]
fn test_progress_large_values() {
    let p = Progress::new(0.999);
    assert!((p.value() - 0.999).abs() < f32::EPSILON);
}

#[test]
fn test_progress_exact_boundaries() {
    let p1 = Progress::new(0.0);
    assert_eq!(p1.value(), 0.0);

    let p2 = Progress::new(1.0);
    assert_eq!(p2.value(), 1.0);
}

#[test]
fn test_progress_set_exact_boundaries() {
    let mut p = Progress::new(0.5);
    p.set_progress(0.0);
    assert_eq!(p.value(), 0.0);

    p.set_progress(1.0);
    assert_eq!(p.value(), 1.0);
}

#[test]
fn test_progress_narrow_bar() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5);
    p.render(&mut ctx);
    // Should render in narrow bar
}

#[test]
fn test_progress_very_narrow_with_percentage() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5).show_percentage(true);
    p.render(&mut ctx);
    // Bar width would be 0, but should still render
}

#[test]
fn test_progress_width_with_percentage() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5).show_percentage(true);
    p.render(&mut ctx);

    // 15 chars for bar (20 - 5 for percentage)
    assert_eq!(buffer.get(7, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(8, 0).unwrap().symbol, '░');
}

// ==================== Character Mapping Tests ====================

#[test]
fn test_progress_block_chars() {
    let p = Progress::new(0.5).style(ProgressStyle::Block);
    // get_chars is private but we can verify rendering
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    p.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '░');
}

#[test]
fn test_progress_line_chars() {
    let p = Progress::new(0.5).style(ProgressStyle::Line);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    p.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '━');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '─');
}

#[test]
fn test_progress_ascii_chars() {
    let p = Progress::new(0.5).style(ProgressStyle::Ascii);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    p.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '#');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '-');
}

#[test]
fn test_progress_braille_chars() {
    let p = Progress::new(0.5).style(ProgressStyle::Braille);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    p.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '⣿');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '⡀');
}
