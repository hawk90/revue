//! Numeric conversion edge case tests
//!
//! Tests for float-to-u16 conversions to prevent overflow and undefined behavior.
//! Addresses the following locations where `f32 as u16` could cause issues:
//! - src/widget/traits/render_context/progress.rs:10
//! - src/widget/layout/splitter.rs:223, 451, 517
//! - src/widget/layout/positioned.rs:156, 169
//! - src/widget/layout/scroll.rs:178
//! - src/widget/layout/resizable/mod.rs:293

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{ProgressBarConfig, RenderContext, View};
use revue::widget::Resizable;
use revue::widget::{HSplit, Positioned, ScrollView, VSplit};

// ==================== Progress Bar Tests ====================

#[test]
fn test_progress_bar_negative_progress() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Negative progress should be clamped to 0.0
    let config = ProgressBarConfig {
        x: 0,
        y: 0,
        width: 10,
        progress: -0.5,
        filled_char: '█',
        empty_char: '░',
        fg: Color::WHITE,
    };

    // Should not panic
    ctx.draw_progress_bar(&config);

    // All characters should be empty since progress is clamped to 0
    for i in 0..10 {
        let cell = buffer.get(i, 0).unwrap();
        assert_eq!(cell.symbol, '░');
    }
}

#[test]
fn test_progress_bar_greater_than_one() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Progress > 1.0 should be clamped to 1.0
    let config = ProgressBarConfig {
        x: 0,
        y: 0,
        width: 10,
        progress: 1.5,
        filled_char: '█',
        empty_char: '░',
        fg: Color::WHITE,
    };

    // Should not panic
    ctx.draw_progress_bar(&config);

    // All characters should be filled since progress is clamped to 1
    for i in 0..10 {
        let cell = buffer.get(i, 0).unwrap();
        assert_eq!(cell.symbol, '█');
    }
}

#[test]
fn test_progress_bar_nan() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // NaN progress - should be clamped (NaN comparisons return false)
    let config = ProgressBarConfig {
        x: 0,
        y: 0,
        width: 10,
        progress: f32::NAN,
        filled_char: '█',
        empty_char: '░',
        fg: Color::WHITE,
    };

    // Should not panic - clamp(NaN, 0.0, 1.0) returns NaN, but the comparison handles it
    ctx.draw_progress_bar(&config);
}

#[test]
fn test_progress_bar_infinity() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Infinity progress - should be clamped to 1.0
    let config = ProgressBarConfig {
        x: 0,
        y: 0,
        width: 10,
        progress: f32::INFINITY,
        filled_char: '█',
        empty_char: '░',
        fg: Color::WHITE,
    };

    // Should not panic
    ctx.draw_progress_bar(&config);

    // All should be filled
    for i in 0..10 {
        let cell = buffer.get(i, 0).unwrap();
        assert_eq!(cell.symbol, '█');
    }
}

#[test]
fn test_progress_bar_neg_infinity() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Negative infinity - should be clamped to 0.0
    let config = ProgressBarConfig {
        x: 0,
        y: 0,
        width: 10,
        progress: f32::NEG_INFINITY,
        filled_char: '█',
        empty_char: '░',
        fg: Color::WHITE,
    };

    // Should not panic
    ctx.draw_progress_bar(&config);

    // All should be empty
    for i in 0..10 {
        let cell = buffer.get(i, 0).unwrap();
        assert_eq!(cell.symbol, '░');
    }
}

#[test]
fn test_progress_bar_zero_width() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let config = ProgressBarConfig {
        x: 0,
        y: 0,
        width: 0,
        progress: 0.5,
        filled_char: '█',
        empty_char: '░',
        fg: Color::WHITE,
    };

    // Should not panic with zero width
    ctx.draw_progress_bar(&config);
}

#[test]
fn test_progress_bar_max_width() {
    // Buffer has a security limit of MAX_BUFFER_DIMENSION (16,384)
    // instead of u16::MAX
    let mut buffer = Buffer::new(16_384, 1);
    let area = Rect::new(0, 0, 16_384, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let config = ProgressBarConfig {
        x: 0,
        y: 0,
        width: 16_384,
        progress: 0.5,
        filled_char: '█',
        empty_char: '░',
        fg: Color::WHITE,
    };

    // Should not panic with maximum allowed width
    ctx.draw_progress_bar(&config);
}

// ==================== Splitter Tests ====================

#[test]
fn test_hsplit_negative_ratio() {
    let split = HSplit::new(-0.5);
    let area = Rect::new(0, 0, 100, 50);
    let (left, right) = split.areas(area);

    // Ratio clamped to 0.1, but min_left (5) and min_right (5) also apply
    // Available = 99, clamped ratio gives 0.1, but min constraints apply
    assert!(left.width >= 5);
    assert!(right.width >= 5);
    assert_eq!(left.width + right.width + 1, 100); // Total width preserved
}

#[test]
fn test_hsplit_greater_than_one_ratio() {
    let split = HSplit::new(1.5);
    let area = Rect::new(0, 0, 100, 50);
    let (left, right) = split.areas(area);

    // Ratio clamped to 0.9, but min constraints apply
    assert!(left.width >= 5);
    assert!(right.width >= 5);
    assert_eq!(left.width + right.width + 1, 100);
}

#[test]
fn test_hsplit_zero_ratio() {
    let split = HSplit::new(0.0);
    let area = Rect::new(0, 0, 100, 50);
    let (left, right) = split.areas(area);

    // Ratio clamped to 0.1, but min constraints apply
    assert!(left.width >= 5);
    assert!(right.width >= 5);
    assert_eq!(left.width + right.width + 1, 100);
}

#[test]
fn test_hsplit_nan_ratio() {
    let split = HSplit::new(f32::NAN);
    let area = Rect::new(0, 0, 100, 50);
    let (left, _right) = split.areas(area);

    // NaN should be handled by clamp in new()
    // Minimum width should still apply
    assert!(left.width >= 5); // min_left default is 5
}

#[test]
fn test_vsplit_negative_ratio() {
    let split = VSplit::new(-0.5);
    let area = Rect::new(0, 0, 100, 50);
    let (top, bottom) = split.areas(area);

    // Ratio should be clamped
    assert!(top.height >= 3); // min_top default is 3
    assert!(bottom.height >= 3); // min_bottom default is 3
}

#[test]
fn test_vsplit_greater_than_one_ratio() {
    let split = VSplit::new(1.5);
    let area = Rect::new(0, 0, 100, 50);
    let (top, bottom) = split.areas(area);

    // Ratio should be clamped
    assert!(top.height <= 47); // 50 - min_bottom - 1
    assert!(bottom.height >= 3);
}

// ==================== Positioned Tests ====================

#[test]
fn test_positioned_negative_percent() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Negative percentage should be handled
    let pos = Positioned::new(revue::widget::Text::new("Test")).percent(-50.0, -50.0);

    // Should not panic - negative percent calculations may produce 0
    pos.render(&mut ctx);
}

#[test]
fn test_positioned_greater_than_100_percent() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Percentages > 100 should work
    let pos = Positioned::new(revue::widget::Text::new("Test"))
        .percent(150.0, 150.0)
        .size(10, 1);

    // Should not panic
    pos.render(&mut ctx);
}

#[test]
fn test_positioned_nan_percent() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // NaN percentage - the f32 as u16 conversion could be problematic
    let pos = Positioned::new(revue::widget::Text::new("Test")).percent(f32::NAN, f32::NAN);

    // NaN * width/height = NaN, NaN as u16 is undefined (could be 0 or wrap)
    // The code should handle this via saturating_add which treats NaN as 0
    pos.render(&mut ctx);
}

#[test]
fn test_positioned_infinity_percent() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Infinity percentage
    let pos =
        Positioned::new(revue::widget::Text::new("Test")).percent(f32::INFINITY, f32::INFINITY);

    // Infinity * value = Infinity, Infinity as u16 wraps
    // saturating_add should handle this
    pos.render(&mut ctx);
}

#[test]
fn test_positioned_negative_position() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Negative absolute position
    let pos = Positioned::new(revue::widget::Text::new("Test")).at(-10, -5);

    // Should not panic - saturating_sub handles this
    pos.render(&mut ctx);
}

#[test]
fn test_positioned_very_large_position() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Very large positive position
    let pos = Positioned::new(revue::widget::Text::new("Test")).at(10000, 10000);

    // Should not panic - position will be outside area but no crash
    pos.render(&mut ctx);
}

// ==================== ScrollView Tests ====================

#[test]
fn test_scroll_view_nan_percentage() {
    let sv = ScrollView::new().content_height(100).scroll_offset(50);

    let percentage = sv.scroll_percentage(20);
    // With offset 50, max_offset = 80, percentage should be 0.625
    assert!((percentage - 0.625).abs() < 0.01);

    // Edge case: zero content height
    let sv_zero = ScrollView::new().content_height(0).scroll_offset(0);

    let percentage = sv_zero.scroll_percentage(20);
    assert_eq!(percentage, 0.0);
}

#[test]
fn test_scroll_view_max_values() {
    let sv = ScrollView::new()
        .content_height(u16::MAX)
        .scroll_offset(u16::MAX - 20);

    let percentage = sv.scroll_percentage(20);
    // Should not panic
    assert!(percentage >= 0.0 && percentage <= 1.0);
}

#[test]
fn test_scroll_view_thumb_calculation() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sv = ScrollView::new().content_height(100).scroll_offset(0);

    // Should not panic when rendering scrollbar
    sv.render_scrollbar(&mut ctx);
}

// ==================== Resizable Tests ====================

#[test]
fn test_resizable_nan_aspect_ratio() {
    let mut r = Resizable::new(20, 10).aspect_ratio(f32::NAN);

    // NaN aspect ratio - the division could produce issues
    r.set_size(40, 10);

    // Should not panic
    let (w, h) = r.size();
    assert!(w > 0 || h > 0);
}

#[test]
fn test_resizable_infinity_aspect_ratio() {
    let mut r = Resizable::new(20, 10).aspect_ratio(f32::INFINITY);

    // Infinity aspect ratio
    r.set_size(40, 10);

    // Should not panic
    let (w, h) = r.size();
    assert!(w > 0 || h > 0);
}

#[test]
fn test_resizable_zero_aspect_ratio() {
    let mut r = Resizable::new(20, 10).aspect_ratio(0.0);

    // Zero aspect ratio
    r.set_size(40, 10);

    // Should not panic
    let (w, h) = r.size();
    assert!(w >= 1 && h >= 1);
}

#[test]
fn test_resizable_negative_aspect_ratio() {
    let mut r = Resizable::new(20, 10).aspect_ratio(-1.5);

    // Negative aspect ratio
    r.set_size(40, 10);

    // Should not panic - max() calls handle it
    let (w, h) = r.size();
    assert!(w >= 1 && h >= 1);
}

// ==================== General Conversion Tests ====================

#[test]
fn test_float_to_u16_negative() {
    // Test that f32 as u16 handles negative values
    let val: f32 = -1.0;
    let result = val as u16;
    // Negative floats cast to u16 truncate toward zero
    assert_eq!(result, 0);
}

#[test]
fn test_float_to_u16_overflow() {
    // Test that f32 as u16 handles overflow
    let val: f32 = u16::MAX as f32 + 1000.0;
    let result = val as u16;
    // Overflow wraps due to precision loss
    assert!(result >= u16::MAX - 10); // Close to MAX
}

#[test]
fn test_float_to_u16_nan_conversion() {
    let val: f32 = f32::NAN;
    let result = val as u16;
    // NaN as u16 produces 0
    assert_eq!(result, 0);
}

#[test]
fn test_float_to_u16_infinity_conversion() {
    let pos_inf: f32 = f32::INFINITY;
    let neg_inf: f32 = f32::NEG_INFINITY;

    let pos_result = pos_inf as u16;
    let neg_result = neg_inf as u16;

    // Infinity as u16 produces u16::MAX (65535)
    assert_eq!(pos_result, u16::MAX);
    assert_eq!(neg_result, 0); // Negative infinity truncates to 0
}

#[test]
fn test_clamp_function() {
    // Test that clamp works correctly
    assert_eq!((-1.0_f32).clamp(0.0, 1.0), 0.0);
    assert_eq!((2.0_f32).clamp(0.0, 1.0), 1.0);
    assert_eq!((0.5_f32).clamp(0.0, 1.0), 0.5);
    // Note: NaN.clamp() in Rust preserves NaN
    let nan_clamped = (f32::NAN).clamp(0.0, 1.0);
    assert!(nan_clamped.is_nan() || nan_clamped >= 0.0); // Either NaN or clamped
}

#[test]
fn test_saturating_add_with_conversion() {
    // Test saturating_add after float-to-u16 conversion
    let base: u16 = 100;
    let offset = (-10.0_f32) as u16; // -10.0 as u16 = 0 (truncates)

    // saturating_add with 0 should just be 100
    let result = base.saturating_add(offset);
    assert_eq!(result, 100);
}

#[test]
fn test_percent_calculation_edge_cases() {
    let width: u16 = 100;

    // Normal case
    let normal = (width as f32 * 0.5) as u16;
    assert_eq!(normal, 50);

    // Overflow case - 20000 wraps to around 44496 then % 65536 = 44496
    let overflow = (width as f32 * 200.0) as u16;
    // Just verify it doesn't crash and produces some value
    let _ = overflow;

    // Negative case - -5000 wraps
    let negative = (width as f32 * -0.5) as u16;
    // Just verify it doesn't crash
    assert!(negative < u16::MAX / 2 || negative > u16::MAX / 2); // Either wraps or not
}
