//! Segment-based drawing utilities for RenderContext

use crate::style::Color;
use crate::utils::unicode::display_width_u16;

impl RenderContext<'_> {
    /// Draw multiple text segments with different colors on one line
    pub fn draw_segments(&mut self, x: u16, y: u16, segments: &[(&str, Color)]) -> u16 {
        let mut cx = x;
        for (text, color) in segments {
            self.draw_text(cx, y, text, *color);
            cx = cx.saturating_add(display_width_u16(text));
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
                cx = cx.saturating_add(display_width_u16(sep));
            }
            self.draw_text(cx, y, text, *color);
            cx = cx.saturating_add(display_width_u16(text));
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
            cx = cx.saturating_add(display_width_u16(key)).saturating_add(1);
            self.draw_text(cx, y, action, action_color);
            cx = cx
                .saturating_add(display_width_u16(action))
                .saturating_add(2);
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
