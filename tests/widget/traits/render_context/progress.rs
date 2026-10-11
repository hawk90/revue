//! Integration tests for RenderContext progress bar methods

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{ProgressBarConfig, RenderContext};

#[test]
fn test_draw_progress_bar_zero() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let config = ProgressBarConfig {
        x: 5,
        y: 2,
        width: 10,
        progress: 0.0,
        filled_char: '█',
        empty_char: '░',
        fg: Color::GREEN,
    };

    ctx.draw_progress_bar(&config);

    // All should be empty char
    for i in 0..10 {
        if let Some(cell) = buffer.get(5 + i, 2) {
            assert_eq!(cell.symbol, '░');
        }
    }
}

#[test]
fn test_draw_progress_bar_half() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let config = ProgressBarConfig {
        x: 5,
        y: 2,
        width: 10,
        progress: 0.5,
        filled_char: '█',
        empty_char: '░',
        fg: Color::GREEN,
    };

    ctx.draw_progress_bar(&config);

    // First 5 should be filled, rest empty
    for i in 0..5 {
        if let Some(cell) = buffer.get(5 + i, 2) {
            assert_eq!(cell.symbol, '█');
        }
    }
    for i in 5..10 {
        if let Some(cell) = buffer.get(5 + i, 2) {
            assert_eq!(cell.symbol, '░');
        }
    }
}

#[test]
fn test_draw_progress_bar_full() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let config = ProgressBarConfig {
        x: 5,
        y: 2,
        width: 10,
        progress: 1.0,
        filled_char: '█',
        empty_char: '░',
        fg: Color::GREEN,
    };

    ctx.draw_progress_bar(&config);

    // All should be filled
    for i in 0..10 {
        if let Some(cell) = buffer.get(5 + i, 2) {
            assert_eq!(cell.symbol, '█');
        }
    }
}

#[test]
fn test_draw_progress_bar_clamps_above() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let config = ProgressBarConfig {
        x: 5,
        y: 2,
        width: 10,
        progress: 1.5, // Above 1.0
        filled_char: '█',
        empty_char: '░',
        fg: Color::GREEN,
    };

    ctx.draw_progress_bar(&config);

    // Should be clamped to full
    for i in 0..10 {
        if let Some(cell) = buffer.get(5 + i, 2) {
            assert_eq!(cell.symbol, '█');
        }
    }
}

#[test]
fn test_draw_progress_bar_clamps_below() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let config = ProgressBarConfig {
        x: 5,
        y: 2,
        width: 10,
        progress: -0.5, // Below 0.0
        filled_char: '█',
        empty_char: '░',
        fg: Color::GREEN,
    };

    ctx.draw_progress_bar(&config);

    // Should be clamped to empty
    for i in 0..10 {
        if let Some(cell) = buffer.get(5 + i, 2) {
            assert_eq!(cell.symbol, '░');
        }
    }
}

#[test]
fn test_draw_progress_bar_custom_chars() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let config = ProgressBarConfig {
        x: 5,
        y: 2,
        width: 10,
        progress: 0.3,
        filled_char: '=',
        empty_char: '-',
        fg: Color::CYAN,
    };

    ctx.draw_progress_bar(&config);

    // First 3 should be filled with '=', rest with '-'
    for i in 0..3 {
        if let Some(cell) = buffer.get(5 + i, 2) {
            assert_eq!(cell.symbol, '=');
        }
    }
    for i in 3..10 {
        if let Some(cell) = buffer.get(5 + i, 2) {
            assert_eq!(cell.symbol, '-');
        }
    }
}

#[test]
fn test_draw_progress_bar_single_width() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let config = ProgressBarConfig {
        x: 10,
        y: 2,
        width: 1,
        progress: 0.8,
        filled_char: '█',
        empty_char: '░',
        fg: Color::GREEN,
    };

    ctx.draw_progress_bar(&config);

    // 0.8 of 1 = 0.8 rounds to 1, so should be filled
    if let Some(cell) = buffer.get(10, 2) {
        assert_eq!(cell.symbol, '█');
    }
}

#[test]
fn test_draw_progress_bar_labeled_zero() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_progress_bar_labeled(5, 2, 10, 0.0, Color::GREEN);

    // Check percentage displays as "  0%"
    if let Some(cell) = buffer.get(5, 2) {
        assert_eq!(cell.symbol, ' ');
    }
    if let Some(cell) = buffer.get(7, 2) {
        assert_eq!(cell.symbol, '0');
    }
}

#[test]
fn test_draw_progress_bar_labeled_half() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_progress_bar_labeled(5, 2, 10, 0.5, Color::GREEN);

    // Check percentage displays as " 50%"
    if let Some(cell) = buffer.get(6, 2) {
        assert_eq!(cell.symbol, '5');
    }
    if let Some(cell) = buffer.get(7, 2) {
        assert_eq!(cell.symbol, '0');
    }
}

#[test]
fn test_draw_progress_bar_labeled_full() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_progress_bar_labeled(5, 2, 10, 1.0, Color::GREEN);

    // Check percentage displays as "100%"
    if let Some(cell) = buffer.get(5, 2) {
        assert_eq!(cell.symbol, '1');
    }
    if let Some(cell) = buffer.get(6, 2) {
        assert_eq!(cell.symbol, '0');
    }
    if let Some(cell) = buffer.get(7, 2) {
        assert_eq!(cell.symbol, '0');
    }
}

#[test]
fn test_draw_progress_bar_labeled_brackets() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    ctx.draw_progress_bar_labeled(5, 2, 10, 0.5, Color::GREEN);

    // Check brackets are drawn
    if let Some(cell) = buffer.get(9, 2) {
        assert_eq!(cell.symbol, '[');
    }
    if let Some(cell) = buffer.get(20, 2) {
        assert_eq!(cell.symbol, ']');
    }
}

#[test]
fn test_progress_bar_config_all_fields() {
    let config = ProgressBarConfig {
        x: 10,
        y: 5,
        width: 20,
        progress: 0.75,
        filled_char: '█',
        empty_char: '░',
        fg: Color::YELLOW,
    };

    assert_eq!(config.x, 10);
    assert_eq!(config.y, 5);
    assert_eq!(config.width, 20);
    assert_eq!(config.progress, 0.75);
    assert_eq!(config.filled_char, '█');
    assert_eq!(config.empty_char, '░');
}

mod float_inputs {
    //! NaN, infinite, negative and out-of-range floats do not overflow the
    //! `f32 as u16` conversions.

    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::style::Color;
    use revue::widget::traits::{ProgressBarConfig, RenderContext};

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
}
