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
