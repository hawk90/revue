//! Progress widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView};
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

// ==================== CSS Integration Tests ====================

#[test]
fn test_progress_element_id() {
    let p = Progress::new(0.5).element_id("upload-progress");
    assert_eq!(View::id(&p), Some("upload-progress"));
}

#[test]
fn test_progress_classes() {
    let p = Progress::new(0.5).class("progress-bar").class("upload");
    assert!(p.has_class("progress-bar"));
    assert!(p.has_class("upload"));
    assert!(!p.has_class("hidden"));
}

#[test]
fn test_progress_styled_view_methods() {
    let mut p = Progress::new(0.5);

    p.set_id("my-progress");
    assert_eq!(View::id(&p), Some("my-progress"));

    p.add_class("active");
    assert!(p.has_class("active"));

    p.remove_class("active");
    assert!(!p.has_class("active"));

    p.toggle_class("visible");
    assert!(p.has_class("visible"));

    p.toggle_class("visible");
    assert!(!p.has_class("visible"));
}

#[test]
fn test_progress_meta() {
    let p = Progress::new(0.5)
        .element_id("test")
        .class("class1")
        .class("class2");

    let meta = p.meta();
    assert_eq!(meta.id, Some("test".to_string()));
    assert_eq!(meta.classes.len(), 2);
}

// ==================== Color Tests ====================

#[test]
fn test_progress_render_with_colors() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5)
        .filled_color(Color::GREEN)
        .empty_color(Color::rgb(128, 128, 128));

    p.render(&mut ctx);

    // First cell should be filled with green color
    if let Some(cell) = buffer.get(0, 0) {
        assert_eq!(cell.fg, Some(Color::GREEN));
    }

    // Last cell should be gray
    if let Some(cell) = buffer.get(9, 0) {
        assert_eq!(cell.fg, Some(Color::rgb(128, 128, 128)));
    }
}

#[test]
fn test_progress_render_same_colors() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5)
        .filled_color(Color::WHITE)
        .empty_color(Color::WHITE);

    p.render(&mut ctx);

    // All cells should have white color
    for x in 0..10 {
        if let Some(cell) = buffer.get(x, 0) {
            assert_eq!(cell.fg, Some(Color::WHITE));
        }
    }
}

// ==================== Percentage Edge Cases ====================

#[test]
fn test_progress_percentage_small_value() {
    let mut buffer = Buffer::new(15, 1);
    let area = Rect::new(0, 0, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.01).show_percentage(true);
    p.render(&mut ctx);

    // Should show "  1%" at the end
    assert_eq!(buffer.get(13, 0).unwrap().symbol, '1');
    assert_eq!(buffer.get(14, 0).unwrap().symbol, '%');
}

#[test]
fn test_progress_percentage_99() {
    let mut buffer = Buffer::new(15, 1);
    let area = Rect::new(0, 0, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.99).show_percentage(true);
    p.render(&mut ctx);

    // Should show " 99%" at the end
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '9');
    assert_eq!(buffer.get(13, 0).unwrap().symbol, '9');
    assert_eq!(buffer.get(14, 0).unwrap().symbol, '%');
}

#[test]
fn test_progress_percentage_33() {
    let mut buffer = Buffer::new(15, 1);
    let area = Rect::new(0, 0, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.33).show_percentage(true);
    p.render(&mut ctx);

    // Should show " 33%" at the end
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '3');
    assert_eq!(buffer.get(13, 0).unwrap().symbol, '3');
    assert_eq!(buffer.get(14, 0).unwrap().symbol, '%');
}

#[test]
fn test_progress_percentage_66() {
    let mut buffer = Buffer::new(15, 1);
    let area = Rect::new(0, 0, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.66).show_percentage(true);
    p.render(&mut ctx);

    // Should show " 66%" at the end
    assert_eq!(buffer.get(12, 0).unwrap().symbol, '6');
    assert_eq!(buffer.get(13, 0).unwrap().symbol, '6');
    assert_eq!(buffer.get(14, 0).unwrap().symbol, '%');
}

#[test]
fn test_progress_without_percentage() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5).show_percentage(false);
    p.render(&mut ctx);

    // All space should be used for the bar (no percentage)
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '░');
}

// ==================== Width Edge Cases ====================

#[test]
fn test_progress_very_wide_bar() {
    let mut buffer = Buffer::new(100, 1);
    let area = Rect::new(0, 0, 100, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.75);
    p.render(&mut ctx);

    // 75% of 100 = 75 filled
    assert_eq!(buffer.get(74, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(75, 0).unwrap().symbol, '░');
}

#[test]
fn test_progress_width_1() {
    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5);
    p.render(&mut ctx);

    // Single character should be filled or empty based on value
    assert!(buffer.get(0, 0).unwrap().symbol == '█' || buffer.get(0, 0).unwrap().symbol == '░');
}

#[test]
fn test_progress_width_2() {
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5);
    p.render(&mut ctx);

    // 50% of 2 = 1 filled, 1 empty
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '░');
}

// ==================== Value Update Tests ====================

#[test]
fn test_progress_update_from_zero() {
    let mut p = Progress::new(0.0);
    assert_eq!(p.value(), 0.0);

    p.set_progress(0.5);
    assert!((p.value() - 0.5).abs() < f32::EPSILON);

    p.set_progress(1.0);
    assert_eq!(p.value(), 1.0);
}

#[test]
fn test_progress_update_from_full() {
    let mut p = Progress::new(1.0);
    assert_eq!(p.value(), 1.0);

    p.set_progress(0.5);
    assert!((p.value() - 0.5).abs() < f32::EPSILON);

    p.set_progress(0.0);
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_progress_multiple_updates() {
    let mut p = Progress::new(0.0);

    for i in 1..=10 {
        let expected = i as f32 / 10.0;
        p.set_progress(expected);
        assert!((p.value() - expected).abs() < f32::EPSILON);
    }
}

#[test]
fn test_progress_oscillating_value() {
    let mut p = Progress::new(0.5);

    p.set_progress(0.75);
    assert!((p.value() - 0.75).abs() < f32::EPSILON);

    p.set_progress(0.25);
    assert!((p.value() - 0.25).abs() < f32::EPSILON);

    p.set_progress(0.75);
    assert!((p.value() - 0.75).abs() < f32::EPSILON);
}

// ==================== Precision Tests ====================

#[test]
fn test_progress_floating_point_precision() {
    let p = Progress::new(0.333333);
    // Value should be stored with reasonable precision
    assert!((p.value() - 0.333333).abs() < 0.0001);
}

#[test]
fn test_progress_very_small_increment() {
    let mut p = Progress::new(0.0);

    p.set_progress(0.0001);
    assert!(p.value() >= 0.0 && p.value() < 0.001);

    p.set_progress(0.0002);
    assert!(p.value() >= 0.0 && p.value() < 0.001);
}

#[test]
fn test_progress_boundary_clamp() {
    let mut p = Progress::new(0.5);

    // Test exact boundary values
    p.set_progress(0.0);
    assert_eq!(p.value(), 0.0);

    p.set_progress(1.0);
    assert_eq!(p.value(), 1.0);

    // Test values just outside boundaries
    p.set_progress(-0.0001);
    assert_eq!(p.value(), 0.0);

    p.set_progress(1.0001);
    assert_eq!(p.value(), 1.0);
}

// ==================== Style Transition Tests ====================

#[test]
fn test_progress_change_style() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p1 = Progress::new(0.5).style(ProgressStyle::Block);
    p1.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '█');

    buffer.clear();

    let p2 = Progress::new(0.5).style(ProgressStyle::Line);
    let mut ctx = RenderContext::new(&mut buffer, area);
    p2.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '━');
}

#[test]
fn test_progress_all_styles_render() {
    let styles = [
        ProgressStyle::Block,
        ProgressStyle::Line,
        ProgressStyle::Ascii,
        ProgressStyle::Braille,
    ];

    for style in styles {
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let p = Progress::new(0.5).style(style);
        p.render(&mut ctx);

        // Should render without panic
        let mut has_content = false;
        for x in 0..10 {
            if let Some(cell) = buffer.get(x, 0) {
                if cell.symbol != ' ' {
                    has_content = true;
                    break;
                }
            }
        }
        assert!(has_content, "Style {:?} should produce output", style);
    }
}

// ==================== Layout Tests ====================

#[test]
fn test_progress_render_offset_area() {
    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(10, 2, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5);
    p.render(&mut ctx);

    // Should render at offset position
    if let Some(cell) = buffer.get(10, 2) {
        assert!(cell.symbol != ' ');
    }
}

#[test]
fn test_progress_render_multiple_times() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);

    let mut p = Progress::new(0.0);

    for i in 0..=10 {
        buffer.clear();
        let mut ctx = RenderContext::new(&mut buffer, area);
        p.set_progress(i as f32 / 10.0);
        p.render(&mut ctx);
        // Should render without panic
    }
}

// ==================== Special Value Tests ====================

#[test]
fn test_progress_infinity() {
    let p = Progress::new(f32::INFINITY);
    // Should clamp to 1.0
    assert_eq!(p.value(), 1.0);
}

#[test]
fn test_progress_neg_infinity() {
    let p = Progress::new(f32::NEG_INFINITY);
    // Should clamp to 0.0
    assert_eq!(p.value(), 0.0);
}

#[test]
fn test_progress_nan() {
    let p = Progress::new(f32::NAN);
    // NaN is stored as-is - just verify it doesn't crash
    assert!(p.value().is_nan());
}

// ==================== Builder Pattern Tests ====================

#[test]
fn test_progress_builder_preserves_value() {
    let p1 = Progress::new(0.75);
    let p2 = p1.style(ProgressStyle::Line);

    // Builder should preserve value
    assert!((p2.value() - 0.75).abs() < f32::EPSILON);
}

#[test]
fn test_progress_multiple_builders() {
    let p = Progress::new(0.5)
        .style(ProgressStyle::Braille)
        .filled_color(Color::CYAN)
        .empty_color(Color::BLUE)
        .show_percentage(true);

    assert!((p.value() - 0.5).abs() < f32::EPSILON);
}

// ==================== Edge Case Values ====================

#[test]
fn test_progress_one_third() {
    let mut buffer = Buffer::new(9, 1);
    let area = Rect::new(0, 0, 9, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(1.0 / 3.0);
    p.render(&mut ctx);

    // 1/3 of 9 = 3 filled
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, '░');
}

#[test]
fn test_progress_two_thirds() {
    let mut buffer = Buffer::new(9, 1);
    let area = Rect::new(0, 0, 9, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(2.0 / 3.0);
    p.render(&mut ctx);

    // 2/3 of 9 = 6 filled
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '░');
}

#[test]
fn test_progress_99_percent() {
    let mut buffer = Buffer::new(100, 1);
    let area = Rect::new(0, 0, 100, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.99);
    p.render(&mut ctx);

    // 99% of 100 = 99 filled
    assert_eq!(buffer.get(98, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(99, 0).unwrap().symbol, '░');
}

#[test]
fn test_progress_1_percent() {
    let mut buffer = Buffer::new(100, 1);
    let area = Rect::new(0, 0, 100, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.01);
    p.render(&mut ctx);

    // 1% of 100 = 1 filled
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '█');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '░');
}

// ==================== Percentage Position Tests ====================

#[test]
fn test_progress_percentage_with_narrow_bar() {
    let mut buffer = Buffer::new(8, 1);
    let area = Rect::new(0, 0, 8, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5).show_percentage(true);
    p.render(&mut ctx);

    // With very narrow area, percentage might not fit
    // Should still render without panic
}

#[test]
fn test_progress_percentage_with_wide_bar() {
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5).show_percentage(true);
    p.render(&mut ctx);

    // Percentage should be at the end
    assert_eq!(buffer.get(26, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(27, 0).unwrap().symbol, '5');
    assert_eq!(buffer.get(28, 0).unwrap().symbol, '0');
    assert_eq!(buffer.get(29, 0).unwrap().symbol, '%');
}

// ==================== Color With Percentage Tests ====================

#[test]
fn test_progress_colors_with_percentage() {
    let mut buffer = Buffer::new(15, 1);
    let area = Rect::new(0, 0, 15, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5)
        .filled_color(Color::GREEN)
        .empty_color(Color::rgb(128, 128, 128))
        .show_percentage(true);

    p.render(&mut ctx);

    // Filled portion should be green
    if let Some(cell) = buffer.get(0, 0) {
        assert_eq!(cell.fg, Some(Color::GREEN));
    }

    // Percentage text should also be colored
    if let Some(cell) = buffer.get(12, 0) {
        assert!(cell.fg.is_some());
    }
}

// ==================== Special Character Tests ====================

#[test]
fn test_progress_unicode_labels() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let p = Progress::new(0.5);
    p.render(&mut ctx);

    // Should render unicode characters correctly
    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains('█') || text.contains('━'));
}
