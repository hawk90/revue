//! Tests for render_context

#![allow(unused_imports)]

use super::super::event::FocusStyle;
use super::*;
use crate::render::Modifier;
use crate::style::{Color, Style};
use std::collections::HashMap;

#[allow(dead_code)]
fn test_buffer() -> Buffer {
    Buffer::new(20, 10)
}

#[allow(dead_code)]
fn test_area() -> Rect {
    Rect::new(0, 0, 20, 10)
}

// =========================================================================
// Constructor tests
// =========================================================================

#[test]
fn test_render_context_new() {
    let mut buffer = test_buffer();
    let area = test_area();
    let ctx = RenderContext::new(&mut buffer, area);

    assert_eq!(ctx.area, area);
    assert!(ctx.style.is_none());
    assert!(ctx.state.is_none());
}

#[test]
fn test_render_context_with_style() {
    let mut buffer = test_buffer();
    let area = test_area();
    let style = Style::default();
    let ctx = RenderContext::with_style(&mut buffer, area, &style);

    assert!(ctx.style.is_some());
    assert!(ctx.state.is_none());
}

#[test]
fn test_render_context_full() {
    let mut buffer = test_buffer();
    let area = test_area();
    let style = Style::default();
    let state = NodeState::default();
    let ctx = RenderContext::full(&mut buffer, area, &style, &state);

    assert!(ctx.style.is_some());
    assert!(ctx.state.is_some());
}

#[test]
fn test_render_context_with_transitions() {
    let mut buffer = test_buffer();
    let area = test_area();
    let mut transitions = HashMap::new();
    transitions.insert("opacity".to_string(), 0.5f32);

    let ctx = RenderContext::new(&mut buffer, area).with_transitions(&transitions);

    assert_eq!(ctx.transition("opacity"), Some(0.5));
    assert_eq!(ctx.transition("nonexistent"), None);
}

#[test]
fn test_transition_or() {
    let mut buffer = test_buffer();
    let area = test_area();
    let mut transitions = HashMap::new();
    transitions.insert("opacity".to_string(), 0.5f32);

    let ctx = RenderContext::new(&mut buffer, area).with_transitions(&transitions);

    assert_eq!(ctx.transition_or("opacity", 1.0), 0.5);
    assert_eq!(ctx.transition_or("nonexistent", 1.0), 1.0);
}

// =========================================================================
// State check tests
// =========================================================================

#[test]
fn test_is_focused_no_state() {
    let mut buffer = test_buffer();
    let ctx = RenderContext::new(&mut buffer, test_area());
    assert!(!ctx.is_focused());
}

#[test]
fn test_is_focused_with_state() {
    let mut buffer = test_buffer();
    let style = Style::default();
    let mut state = NodeState::default();
    state.focused = true;
    let ctx = RenderContext::full(&mut buffer, test_area(), &style, &state);
    assert!(ctx.is_focused());
}

#[test]
fn test_is_hovered() {
    let mut buffer = test_buffer();
    let style = Style::default();
    let mut state = NodeState::default();
    state.hovered = true;
    let ctx = RenderContext::full(&mut buffer, test_area(), &style, &state);
    assert!(ctx.is_hovered());
}

#[test]
fn test_is_disabled() {
    let mut buffer = test_buffer();
    let style = Style::default();
    let mut state = NodeState::default();
    state.disabled = true;
    let ctx = RenderContext::full(&mut buffer, test_area(), &style, &state);
    assert!(ctx.is_disabled());
}

// =========================================================================
// Drawing tests
// =========================================================================

#[test]
fn test_draw_char() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_char(0, 0, 'A', color);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'A');
}

#[test]
fn test_draw_char_bg() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let fg = Color::rgb(255, 255, 255);
    let bg = Color::rgb(0, 0, 0);

    ctx.draw_char_bg(0, 0, 'X', fg, bg);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'X');
    assert_eq!(cell.bg, Some(bg));
}

#[test]
fn test_draw_char_bold() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_char_bold(0, 0, 'B', color);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, 'B');
    assert!(cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_draw_text() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_text(0, 0, "Hello", color);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'e');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'l');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 'l');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'o');
}

#[test]
fn test_draw_text_bg() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let fg = Color::rgb(255, 255, 255);
    let bg = Color::rgb(100, 100, 100);

    ctx.draw_text_bg(0, 0, "Hi", fg, bg);

    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(bg));
    assert_eq!(buffer.get(1, 0).unwrap().bg, Some(bg));
}

#[test]
fn test_draw_text_bold() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_text_bold(0, 0, "Bold", color);

    assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_draw_hline() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_hline(0, 0, 5, '-', color);

    for i in 0..5 {
        assert_eq!(buffer.get(i, 0).unwrap().symbol, '-');
    }
}

#[test]
fn test_draw_vline() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_vline(0, 0, 5, '|', color);

    for i in 0..5 {
        assert_eq!(buffer.get(0, i).unwrap().symbol, '|');
    }
}

#[test]
fn test_draw_box_rounded() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_box_rounded(0, 0, 5, 3, color);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '╮');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '╰');
    assert_eq!(buffer.get(4, 2).unwrap().symbol, '╯');
}

#[test]
fn test_draw_box_rounded_too_small() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    // Should not panic with small dimensions
    ctx.draw_box_rounded(0, 0, 1, 1, color);
}

#[test]
fn test_draw_box_single() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_box_single(0, 0, 5, 3, color);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '┐');
}

#[test]
fn test_draw_box_double() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_box_double(0, 0, 5, 3, color);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╔');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '╗');
}

#[test]
fn test_fill() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.fill(0, 0, 3, 2, '#', color);

    for y in 0..2 {
        for x in 0..3 {
            assert_eq!(buffer.get(x, y).unwrap().symbol, '#');
        }
    }
}

#[test]
fn test_fill_bg() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let bg = Color::rgb(100, 100, 100);

    ctx.fill_bg(0, 0, 3, 2, bg);

    for y in 0..2 {
        for x in 0..3 {
            assert_eq!(buffer.get(x, y).unwrap().bg, Some(bg));
        }
    }
}

#[test]
fn test_clear() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    // First fill with content
    ctx.fill(0, 0, 3, 2, '#', color);
    // Then clear
    ctx.clear(0, 0, 3, 2);

    for y in 0..2 {
        for x in 0..3 {
            assert_eq!(buffer.get(x, y).unwrap().symbol, ' ');
        }
    }
}

#[test]
fn test_draw_text_clipped() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_text_clipped(0, 0, "Hello World", color, 5);

    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'o');
    // Should not draw beyond max_width
    assert_eq!(buffer.get(5, 0).unwrap().symbol, ' ');
}

#[test]
fn test_draw_text_centered() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_text_centered(0, 0, 10, "Hi", color);

    // "Hi" is 2 chars, centered in 10 width = starts at position 4
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, 'i');
}

#[test]
fn test_draw_text_right() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_text_right(0, 0, 10, "Hi", color);

    // "Hi" is 2 chars, right-aligned in 10 width = starts at position 8
    assert_eq!(buffer.get(8, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, 'i');
}

#[test]
fn test_draw_text_dim() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_text_dim(0, 0, "dim", color);

    assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::DIM));
}

#[test]
fn test_draw_text_italic() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_text_italic(0, 0, "italic", color);

    assert!(buffer
        .get(0, 0)
        .unwrap()
        .modifier
        .contains(Modifier::ITALIC));
}

#[test]
fn test_draw_text_underline() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_text_underline(0, 0, "underline", color);

    assert!(buffer
        .get(0, 0)
        .unwrap()
        .modifier
        .contains(Modifier::UNDERLINE));
}

// =========================================================================
// Progress bar tests
// =========================================================================

#[test]
fn test_draw_progress_bar() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());

    let config = ProgressBarConfig {
        x: 0,
        y: 0,
        width: 10,
        progress: 0.5,
        filled_char: '█',
        empty_char: '░',
        fg: Color::rgb(255, 255, 255),
    };

    ctx.draw_progress_bar(&config);

    // 50% of 10 = 5 filled
    for i in 0..5 {
        assert_eq!(buffer.get(i, 0).unwrap().symbol, '█');
    }
    for i in 5..10 {
        assert_eq!(buffer.get(i, 0).unwrap().symbol, '░');
    }
}

#[test]
fn test_draw_progress_bar_clamp() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());

    // Test progress > 1.0 is clamped
    let config = ProgressBarConfig {
        x: 0,
        y: 0,
        width: 10,
        progress: 1.5,
        filled_char: '█',
        empty_char: '░',
        fg: Color::rgb(255, 255, 255),
    };

    ctx.draw_progress_bar(&config);

    // Should be fully filled (clamped to 1.0)
    for i in 0..10 {
        assert_eq!(buffer.get(i, 0).unwrap().symbol, '█');
    }
}

#[test]
fn test_draw_progress_bar_zero() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());

    let config = ProgressBarConfig {
        x: 0,
        y: 0,
        width: 10,
        progress: 0.0,
        filled_char: '█',
        empty_char: '░',
        fg: Color::rgb(255, 255, 255),
    };

    ctx.draw_progress_bar(&config);

    // Should be all empty
    for i in 0..10 {
        assert_eq!(buffer.get(i, 0).unwrap().symbol, '░');
    }
}

#[test]
fn test_draw_progress_bar_negative() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());

    let config = ProgressBarConfig {
        x: 0,
        y: 0,
        width: 10,
        progress: -0.5,
        filled_char: '█',
        empty_char: '░',
        fg: Color::rgb(255, 255, 255),
    };

    ctx.draw_progress_bar(&config);

    // Should be clamped to 0.0 (all empty)
    for i in 0..10 {
        assert_eq!(buffer.get(i, 0).unwrap().symbol, '░');
    }
}

#[test]
fn test_draw_progress_bar_labeled() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_progress_bar_labeled(0, 0, 5, 0.5, color);

    // Should have label " 50%"
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '5');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '0');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, '%');
    // Should have opening bracket
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '[');
    // 50% of 5 width = 2-3 filled
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '█');
}

#[test]
fn test_draw_progress_bar_labeled_zero() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_progress_bar_labeled(0, 0, 5, 0.0, color);

    // Should have label "  0%"
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '0');
}

#[test]
fn test_draw_progress_bar_labeled_full() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_progress_bar_labeled(0, 0, 5, 1.0, color);

    // Should have label "100%"
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '1');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '0');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '0');
}

#[test]
fn test_draw_progress_bar_labeled_clamp() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    // Test that values > 1.0 are clamped
    ctx.draw_progress_bar_labeled(0, 0, 5, 1.5, color);

    // Should show 100%
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '1');
}

// =========================================================================
// CSS integration tests
// =========================================================================

#[test]
fn test_css_background_no_style() {
    let mut buffer = test_buffer();
    let default = Color::rgb(100, 100, 100);
    let ctx = RenderContext::new(&mut buffer, test_area());

    assert_eq!(ctx.css_background(default), default);
}

#[test]
fn test_css_border_color_no_style() {
    let mut buffer = test_buffer();
    let default = Color::rgb(50, 50, 50);
    let ctx = RenderContext::new(&mut buffer, test_area());

    assert_eq!(ctx.css_border_color(default), default);
}

#[test]
fn test_css_padding_no_style() {
    let mut buffer = test_buffer();
    let ctx = RenderContext::new(&mut buffer, test_area());

    let padding = ctx.css_padding();
    assert_eq!(padding.top, 0);
    assert_eq!(padding.right, 0);
    assert_eq!(padding.bottom, 0);
    assert_eq!(padding.left, 0);
}

#[test]
fn test_css_margin_no_style() {
    let mut buffer = test_buffer();
    let ctx = RenderContext::new(&mut buffer, test_area());

    let margin = ctx.css_margin();
    assert_eq!(margin.top, 0);
}

#[test]
fn test_css_width_no_style() {
    let mut buffer = test_buffer();
    let ctx = RenderContext::new(&mut buffer, test_area());

    let width = ctx.css_width();
    assert_eq!(width.value, 0);
}

#[test]
fn test_css_height_no_style() {
    let mut buffer = test_buffer();
    let ctx = RenderContext::new(&mut buffer, test_area());

    let height = ctx.css_height();
    assert_eq!(height.value, 0);
}

#[test]
fn test_css_border_style_no_style() {
    let mut buffer = test_buffer();
    let ctx = RenderContext::new(&mut buffer, test_area());

    let border_style = ctx.css_border_style();
    assert_eq!(border_style, BorderStyle::default());
}

#[test]
fn test_css_gap_no_style() {
    let mut buffer = test_buffer();
    let ctx = RenderContext::new(&mut buffer, test_area());

    assert_eq!(ctx.css_gap(), 0);
}

#[test]
fn test_css_background_with_style() {
    use crate::style::Style;

    let mut buffer = test_buffer();
    let default = Color::rgb(100, 100, 100);
    let styled = Color::rgb(200, 200, 200);
    let mut style = Style::default();
    style.visual.background = styled;

    let ctx = RenderContext::with_style(&mut buffer, test_area(), &style);

    // Should return the styled color, not default
    assert_eq!(ctx.css_background(default), styled);
}

#[test]
fn test_css_background_default_color_with_style() {
    use crate::style::Style;

    let mut buffer = test_buffer();
    let default = Color::rgb(100, 100, 100);
    let mut style = Style::default();
    // Style background is default (Black)
    style.visual.background = Color::default();

    let ctx = RenderContext::with_style(&mut buffer, test_area(), &style);

    // Should return default when style color is default
    assert_eq!(ctx.css_background(default), default);
}

#[test]
fn test_css_border_color_with_style() {
    use crate::style::Style;

    let mut buffer = test_buffer();
    let default = Color::rgb(50, 50, 50);
    let styled = Color::rgb(150, 150, 150);
    let mut style = Style::default();
    style.visual.border_color = styled;

    let ctx = RenderContext::with_style(&mut buffer, test_area(), &style);

    assert_eq!(ctx.css_border_color(default), styled);
}

#[test]
fn test_css_opacity_with_style() {
    use crate::style::Style;

    let mut buffer = test_buffer();
    let mut style = Style::default();
    style.visual.opacity = 0.5;

    let ctx = RenderContext::with_style(&mut buffer, test_area(), &style);

    assert_eq!(ctx.css_opacity(), 0.5);
}

#[test]
fn test_css_visible_false_with_style() {
    use crate::style::Style;

    let mut buffer = test_buffer();
    let mut style = Style::default();
    style.visual.visible = false;

    let ctx = RenderContext::with_style(&mut buffer, test_area(), &style);

    assert!(!ctx.css_visible());
}

#[test]
fn test_css_padding_with_style() {
    use crate::style::{Spacing, Style};

    let mut buffer = test_buffer();
    let mut style = Style::default();
    let spacing = Spacing::new(1, 2, 3, 4);
    style.spacing.padding = spacing;

    let ctx = RenderContext::with_style(&mut buffer, test_area(), &style);

    let padding = ctx.css_padding();
    assert_eq!(padding.top, 1);
    assert_eq!(padding.right, 2);
    assert_eq!(padding.bottom, 3);
    assert_eq!(padding.left, 4);
}

#[test]
fn test_css_margin_with_style() {
    use crate::style::{Spacing, Style};

    let mut buffer = test_buffer();
    let mut style = Style::default();
    let spacing = Spacing::new(5, 6, 7, 8);
    style.spacing.margin = spacing;

    let ctx = RenderContext::with_style(&mut buffer, test_area(), &style);

    let margin = ctx.css_margin();
    assert_eq!(margin.top, 5);
    assert_eq!(margin.right, 6);
}

#[test]
fn test_css_width_with_style() {
    use crate::style::{Size, Style};

    let mut buffer = test_buffer();
    let mut style = Style::default();
    style.sizing.width = Size::px(100);

    let ctx = RenderContext::with_style(&mut buffer, test_area(), &style);

    let width = ctx.css_width();
    assert_eq!(width.value, 100);
}

#[test]
fn test_css_height_with_style() {
    use crate::style::{Size, Style};

    let mut buffer = test_buffer();
    let mut style = Style::default();
    style.sizing.height = Size::px(50);

    let ctx = RenderContext::with_style(&mut buffer, test_area(), &style);

    let height = ctx.css_height();
    assert_eq!(height.value, 50);
}

#[test]
fn test_css_border_style_with_style() {
    use crate::style::{BorderStyle, Style};

    let mut buffer = test_buffer();
    let mut style = Style::default();
    style.visual.border_style = BorderStyle::Dashed;

    let ctx = RenderContext::with_style(&mut buffer, test_area(), &style);

    assert_eq!(ctx.css_border_style(), BorderStyle::Dashed);
}

#[test]
fn test_css_gap_with_style() {
    use crate::style::Style;

    let mut buffer = test_buffer();
    let mut style = Style::default();
    style.layout.gap = 10;

    let ctx = RenderContext::with_style(&mut buffer, test_area(), &style);

    assert_eq!(ctx.css_gap(), 10);
}

// =========================================================================
// Segment drawing tests
// =========================================================================

#[test]
fn test_draw_segments() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());

    let c1 = Color::rgb(255, 0, 0);
    let c2 = Color::rgb(0, 255, 0);
    let segments: &[(&str, Color)] = &[("AB", c1), ("CD", c2)];

    let end_x = ctx.draw_segments(0, 0, segments);

    assert_eq!(end_x, 4);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'C');
}

#[test]
fn test_draw_segments_sep() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());

    let c1 = Color::rgb(255, 0, 0);
    let c2 = Color::rgb(0, 255, 0);
    let sep_color = Color::rgb(128, 128, 128);
    let segments: &[(&str, Color)] = &[("A", c1), ("B", c2)];

    let end_x = ctx.draw_segments_sep(0, 0, segments, "|", sep_color);

    assert_eq!(end_x, 3); // "A" + "|" + "B"
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '|');
}

#[test]
fn test_draw_text_selectable_selected() {
    let mut buffer = test_buffer();
    let normal = Color::rgb(200, 200, 200);
    let selected = Color::rgb(255, 255, 0);

    {
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        ctx.draw_text_selectable(0, 0, "Item", true, normal, selected);
    }

    assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_draw_text_selectable_not_selected() {
    let mut buffer = test_buffer();
    let normal = Color::rgb(200, 200, 200);
    let selected = Color::rgb(255, 255, 0);

    {
        let mut ctx = RenderContext::new(&mut buffer, test_area());
        ctx.draw_text_selectable(0, 0, "Item", false, normal, selected);
    }

    assert!(!buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
}

// =========================================================================
// Metric color tests
// =========================================================================

#[test]
fn test_metric_color() {
    let low = Color::rgb(0, 255, 0);
    let mid = Color::rgb(255, 255, 0);
    let high = Color::rgb(255, 0, 0);

    assert_eq!(RenderContext::metric_color(10, 50, 80, low, mid, high), low);
    assert_eq!(RenderContext::metric_color(60, 50, 80, low, mid, high), mid);
    assert_eq!(
        RenderContext::metric_color(90, 50, 80, low, mid, high),
        high
    );
}

// =========================================================================
// CSS integration tests
// =========================================================================

#[test]
fn test_css_color_no_style() {
    let mut buffer = test_buffer();
    let default = Color::rgb(255, 255, 255);
    let ctx = RenderContext::new(&mut buffer, test_area());

    assert_eq!(ctx.css_color(default), default);
}

#[test]
fn test_css_opacity() {
    let mut buffer = test_buffer();
    let ctx = RenderContext::new(&mut buffer, test_area());

    assert_eq!(ctx.css_opacity(), 1.0);
}

#[test]
fn test_css_visible() {
    let mut buffer = test_buffer();
    let ctx = RenderContext::new(&mut buffer, test_area());

    assert!(ctx.css_visible());
}

// =========================================================================
// Focus ring tests
// =========================================================================

#[test]
fn test_draw_focus_ring_solid() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 0);

    ctx.draw_focus_ring(0, 0, 5, 3, color, FocusStyle::Solid);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '┐');
}

#[test]
fn test_draw_focus_ring_rounded() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 0);

    ctx.draw_focus_ring(0, 0, 5, 3, color, FocusStyle::Rounded);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
}

#[test]
fn test_draw_focus_ring_double() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 0);

    ctx.draw_focus_ring(0, 0, 5, 3, color, FocusStyle::Double);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╔');
}

#[test]
fn test_draw_focus_ring_too_small() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 0);

    // Should not panic
    ctx.draw_focus_ring(0, 0, 1, 1, color, FocusStyle::Solid);
}

#[test]
fn test_draw_focus_underline() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 0);

    ctx.draw_focus_underline(0, 0, 5, color);

    for i in 0..5 {
        assert_eq!(buffer.get(i, 0).unwrap().symbol, '▔');
    }
}

#[test]
fn test_draw_focus_marker() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 0);

    ctx.draw_focus_marker(0, 0, color);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▶');
}

#[test]
fn test_invert_colors() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let fg = Color::rgb(255, 255, 255);
    let bg = Color::rgb(0, 0, 0);

    ctx.draw_char_bg(0, 0, 'X', fg, bg);
    ctx.invert_colors(0, 0, 1, 1);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(bg));
    assert_eq!(cell.bg, Some(fg));
}

// =========================================================================
// Additional shape tests
// =========================================================================

#[test]
fn test_draw_box_no_top() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_box_no_top(0, 0, 5, 3, color);

    // Bottom corners should be drawn
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '╰');
    assert_eq!(buffer.get(4, 2).unwrap().symbol, '╯');
    // Vertical sides should be drawn
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '│');
    assert_eq!(buffer.get(4, 1).unwrap().symbol, '│');
}

#[test]
fn test_draw_box_no_top_too_small() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    // Should not panic with small dimensions
    ctx.draw_box_no_top(0, 0, 1, 1, color);
}

#[test]
fn test_draw_header_line() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let border = Color::rgb(100, 100, 100);
    let text_color = Color::rgb(255, 255, 255);

    let parts = &[("Test", text_color)];
    ctx.draw_header_line(0, 0, 10, parts, border);

    // Should have corners
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '╮');
}

#[test]
fn test_draw_header_line_multiple_parts() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let border = Color::rgb(100, 100, 100);
    let color1 = Color::rgb(255, 0, 0);
    let color2 = Color::rgb(0, 255, 0);

    let parts = &[("A", color1), ("B", color2)];
    ctx.draw_header_line(0, 0, 10, parts, border);

    // Should have text drawn
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'A');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 'B');
}

#[test]
fn test_draw_header_line_too_small() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let border = Color::rgb(100, 100, 100);
    let text_color = Color::rgb(255, 255, 255);

    let parts = &[("Test", text_color)];
    // Should not panic with small width
    ctx.draw_header_line(0, 0, 3, parts, border);
}

#[test]
fn test_draw_box_titled() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_box_titled(0, 0, 10, 3, "Title", color);

    // Should have corners
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '╮');
    // Should have title text
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'T');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 'i');
}

#[test]
fn test_draw_box_titled_too_small() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    // Should not panic with small dimensions
    ctx.draw_box_titled(0, 0, 1, 1, "Title", color);
}

#[test]
fn test_draw_box_titled_single() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_box_titled_single(0, 0, 10, 3, "Title", color);

    // Should have single-line corners
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '┐');
}

#[test]
fn test_draw_box_titled_double() {
    let mut buffer = test_buffer();
    let mut ctx = RenderContext::new(&mut buffer, test_area());
    let color = Color::rgb(255, 255, 255);

    ctx.draw_box_titled_double(0, 0, 10, 3, "Title", color);

    // Should have double-line corners
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╔');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '╗');
}
