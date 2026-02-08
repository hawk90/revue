//! Segment-based drawing utilities for RenderContext

use crate::style::Color;
use crate::utils::unicode::display_width;

impl RenderContext<'_> {
    /// Draw multiple text segments with different colors on one line
    pub fn draw_segments(&mut self, x: u16, y: u16, segments: &[(&str, Color)]) -> u16 {
        let mut cx = x;
        for (text, color) in segments {
            self.draw_text(cx, y, text, *color);
            cx += display_width(text) as u16;
        }
        cx
    }

    /// Draw segments with a separator between them
    pub fn draw_segments_sep(
        &mut self,
        x: u16,
        y: u16,
        segments: &[(&str, Color)],
        sep: &str,
        sep_color: Color,
    ) -> u16 {
        let mut cx = x;
        for (i, (text, color)) in segments.iter().enumerate() {
            if i > 0 {
                self.draw_text(cx, y, sep, sep_color);
                cx += display_width(sep) as u16;
            }
            self.draw_text(cx, y, text, *color);
            cx += display_width(text) as u16;
        }
        cx
    }

    /// Draw key hints (key in bold color, action in dim)
    pub fn draw_key_hints(
        &mut self,
        x: u16,
        y: u16,
        hints: &[(&str, &str)],
        key_color: Color,
        action_color: Color,
    ) -> u16 {
        let mut cx = x;
        for (key, action) in hints {
            self.draw_text_bold(cx, y, key, key_color);
            cx += display_width(key) as u16 + 1;
            self.draw_text(cx, y, action, action_color);
            cx += display_width(action) as u16 + 2;
        }
        cx
    }

    /// Draw text with selection styling (bold + highlight color when selected)
    pub fn draw_text_selectable(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        selected: bool,
        normal_color: Color,
        selected_color: Color,
    ) {
        if selected {
            self.draw_text_bold(x, y, text, selected_color);
        } else {
            self.draw_text(x, y, text, normal_color);
        }
    }

    /// Get color based on value thresholds (for metrics)
    pub fn metric_color(
        value: u8,
        mid: u8,
        high: u8,
        low_color: Color,
        mid_color: Color,
        high_color: Color,
    ) -> Color {
        if value < mid {
            low_color
        } else if value < high {
            mid_color
        } else {
            high_color
        }
    }
}

use crate::widget::traits::render_context::RenderContext;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    // =========================================================================
    // draw_segments tests
    // =========================================================================

    #[test]
    fn test_draw_segments_empty() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let result = ctx.draw_segments(0, 0, &[]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_draw_segments_single() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let result = ctx.draw_segments(0, 0, &[("Hello", Color::WHITE)]);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_draw_segments_multiple() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let result = ctx.draw_segments(0, 0, &[("Hello", Color::WHITE), ("World", Color::CYAN)]);
        assert_eq!(result, 10); // 5 + 5
    }

    #[test]
    fn test_draw_segments_offset() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let result = ctx.draw_segments(5, 0, &[("Test", Color::WHITE)]);
        assert_eq!(result, 9); // 5 + 4
    }

    #[test]
    fn test_draw_segments_wide_char() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let result = ctx.draw_segments(0, 0, &[("你好", Color::WHITE)]);
        // Each Chinese character is width 2
        assert_eq!(result, 4);
    }

    // =========================================================================
    // draw_segments_sep tests
    // =========================================================================

    #[test]
    fn test_draw_segments_sep_empty() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let result = ctx.draw_segments_sep(0, 0, &[], " | ", Color::rgb(100, 100, 100));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_draw_segments_sep_single() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let result = ctx.draw_segments_sep(
            0,
            0,
            &[("Hello", Color::WHITE)],
            " | ",
            Color::rgb(100, 100, 100),
        );
        // No separator for single element
        assert_eq!(result, 5);
    }

    #[test]
    fn test_draw_segments_sep_multiple() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let result = ctx.draw_segments_sep(
            0,
            0,
            &[("A", Color::WHITE), ("B", Color::CYAN)],
            ":",
            Color::rgb(100, 100, 100),
        );
        // "A:B" = 1 + 1 + 1 = 3
        assert_eq!(result, 3);
    }

    #[test]
    fn test_draw_segments_sep_custom_sep() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let result = ctx.draw_segments_sep(
            0,
            0,
            &[("A", Color::WHITE), ("B", Color::CYAN)],
            " | ",
            Color::rgb(100, 100, 100),
        );
        // "A | B" = 1 + 3 + 1 = 5
        assert_eq!(result, 5);
    }

    // =========================================================================
    // draw_key_hints tests
    // =========================================================================

    #[test]
    fn test_draw_key_hints_empty() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let result = ctx.draw_key_hints(0, 0, &[], Color::CYAN, Color::WHITE);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_draw_key_hints_single() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let result = ctx.draw_key_hints(0, 0, &[("q", "quit")], Color::CYAN, Color::WHITE);
        // "q quit" + 2 spaces = 1 + 1 + 4 + 2 = 8
        assert_eq!(result, 8);
    }

    #[test]
    fn test_draw_key_hints_multiple() {
        let mut buffer = Buffer::new(20, 2);
        let area = Rect::new(0, 0, 20, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let result = ctx.draw_key_hints(
            0,
            0,
            &[("q", "quit"), ("s", "save")],
            Color::CYAN,
            Color::WHITE,
        );
        // "q quit" + 2 + "s save" + 2 = 8 + 8 = 16
        assert_eq!(result, 16);
    }

    #[test]
    fn test_draw_key_hints_multi_char_key() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let result = ctx.draw_key_hints(0, 0, &[("Ctrl", "action")], Color::CYAN, Color::WHITE);
        // "Ctrl action" + 2 = 4 + 1 + 6 + 2 = 13
        assert_eq!(result, 13);
    }

    // =========================================================================
    // draw_text_selectable tests
    // =========================================================================

    #[test]
    fn test_draw_text_selectable_not_selected() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_selectable(0, 0, "Test", false, Color::rgb(100, 100, 100), Color::CYAN);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_text_selectable_selected() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_selectable(0, 0, "Test", true, Color::rgb(100, 100, 100), Color::CYAN);
        // Just verify it doesn't panic
    }

    #[test]
    fn test_draw_text_selectable_toggle() {
        let mut buffer = Buffer::new(20, 2);
        let area = Rect::new(0, 0, 20, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);
        ctx.draw_text_selectable(0, 0, "Test", true, Color::rgb(100, 100, 100), Color::CYAN);
        ctx.draw_text_selectable(0, 1, "Test", false, Color::rgb(100, 100, 100), Color::CYAN);
        // Just verify both don't panic
    }

    // =========================================================================
    // metric_color tests
    // =========================================================================

    #[test]
    fn test_metric_color_low() {
        let color =
            RenderContext::metric_color(10, 50, 80, Color::GREEN, Color::YELLOW, Color::RED);
        assert_eq!(color, Color::GREEN);
    }

    #[test]
    fn test_metric_color_mid() {
        let color =
            RenderContext::metric_color(60, 50, 80, Color::GREEN, Color::YELLOW, Color::RED);
        assert_eq!(color, Color::YELLOW);
    }

    #[test]
    fn test_metric_color_high() {
        let color =
            RenderContext::metric_color(90, 50, 80, Color::GREEN, Color::YELLOW, Color::RED);
        assert_eq!(color, Color::RED);
    }

    #[test]
    fn test_metric_color_boundary_low_mid() {
        // Exactly at mid boundary
        let color =
            RenderContext::metric_color(50, 50, 80, Color::GREEN, Color::YELLOW, Color::RED);
        // value >= mid, so returns mid_color
        assert_eq!(color, Color::YELLOW);
    }

    #[test]
    fn test_metric_color_boundary_mid_high() {
        // Exactly at high boundary
        let color =
            RenderContext::metric_color(80, 50, 80, Color::GREEN, Color::YELLOW, Color::RED);
        // value >= high, so returns high_color
        assert_eq!(color, Color::RED);
    }

    #[test]
    fn test_metric_color_zero() {
        let color = RenderContext::metric_color(0, 50, 80, Color::GREEN, Color::YELLOW, Color::RED);
        assert_eq!(color, Color::GREEN);
    }

    #[test]
    fn test_metric_color_max_u8() {
        let color =
            RenderContext::metric_color(255, 50, 80, Color::GREEN, Color::YELLOW, Color::RED);
        assert_eq!(color, Color::RED);
    }

    #[test]
    fn test_metric_color_equal_thresholds() {
        let color =
            RenderContext::metric_color(50, 50, 50, Color::GREEN, Color::YELLOW, Color::RED);
        // value >= high, so returns high_color
        assert_eq!(color, Color::RED);
    }

    #[test]
    fn test_metric_color_custom_colors() {
        let color = RenderContext::metric_color(
            30,
            50,
            80,
            Color::rgb(100, 200, 100),
            Color::rgb(200, 200, 100),
            Color::rgb(200, 100, 100),
        );
        assert_eq!(color, Color::rgb(100, 200, 100));
    }
}
