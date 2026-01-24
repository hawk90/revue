//! Render context for widget rendering

mod css;
mod focus;
mod progress;
mod segments;
mod shapes;
#[cfg(test)]
mod tests {
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

}
mod text;
mod types;

pub use types::ProgressBarConfig;

use crate::dom::NodeState;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Style;

/// Render context passed to widgets
pub struct RenderContext<'a> {
    /// Buffer to render into
    pub buffer: &'a mut Buffer,
    /// Available area for rendering
    pub area: Rect,
    /// Computed style from CSS cascade
    pub style: Option<&'a Style>,
    /// Current widget state
    pub state: Option<&'a NodeState>,
    /// Transition values for animations (property name -> current value)
    transitions: Option<&'a std::collections::HashMap<String, f32>>,
}

impl<'a> RenderContext<'a> {
    /// Create a basic render context (without style/state)
    pub fn new(buffer: &'a mut Buffer, area: Rect) -> Self {
        Self {
            buffer,
            area,
            style: None,
            state: None,
            transitions: None,
        }
    }

    /// Create a render context with style
    pub fn with_style(buffer: &'a mut Buffer, area: Rect, style: &'a Style) -> Self {
        Self {
            buffer,
            area,
            style: Some(style),
            state: None,
            transitions: None,
        }
    }

    /// Create a full render context
    pub fn full(
        buffer: &'a mut Buffer,
        area: Rect,
        style: &'a Style,
        state: &'a NodeState,
    ) -> Self {
        Self {
            buffer,
            area,
            style: Some(style),
            state: Some(state),
            transitions: None,
        }
    }

    /// Set transition values for this render context
    pub fn with_transitions(
        mut self,
        transitions: &'a std::collections::HashMap<String, f32>,
    ) -> Self {
        self.transitions = Some(transitions);
        self
    }

    /// Get current transition value for a property
    pub fn transition(&self, property: &str) -> Option<f32> {
        self.transitions.and_then(|t| t.get(property).copied())
    }

    /// Get transition value with a default fallback
    pub fn transition_or(&self, property: &str, default: f32) -> f32 {
        self.transition(property).unwrap_or(default)
    }

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.state.map(|s| s.focused).unwrap_or(false)
    }

    /// Check if hovered
    pub fn is_hovered(&self) -> bool {
        self.state.map(|s| s.hovered).unwrap_or(false)
    }

    /// Check if disabled
    pub fn is_disabled(&self) -> bool {
        self.state.map(|s| s.disabled).unwrap_or(false)
    }
}
