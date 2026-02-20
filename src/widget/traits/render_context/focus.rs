//! Focus indicator methods for RenderContext

use super::super::event::FocusStyle;
use crate::render::Cell;
use crate::style::Color;

impl RenderContext<'_> {
    /// Draw a focus ring around an area
    pub fn draw_focus_ring(
        &mut self,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        color: Color,
        style: FocusStyle,
    ) {
        if w < 2 || h < 2 {
            return;
        }

        let (h_char, v_char, tl, tr, bl, br) = match style {
            FocusStyle::Solid => ('─', '│', '┌', '┐', '└', '┘'),
            FocusStyle::Rounded => ('─', '│', '╭', '╮', '╰', '╯'),
            FocusStyle::Double => ('═', '║', '╔', '╗', '╚', '╝'),
            FocusStyle::Dotted => ('╌', '╎', '┌', '┐', '└', '┘'),
            FocusStyle::Bold => ('━', '┃', '┏', '┓', '┗', '┛'),
            FocusStyle::Ascii => ('-', '|', '+', '+', '+', '+'),
        };

        self.draw_char(x, y, tl, color);
        self.draw_char(x + w - 1, y, tr, color);
        self.draw_char(x, y + h - 1, bl, color);
        self.draw_char(x + w - 1, y + h - 1, br, color);
        self.draw_hline(x + 1, y, w - 2, h_char, color);
        self.draw_hline(x + 1, y + h - 1, w - 2, h_char, color);
        self.draw_vline(x, y + 1, h - 2, v_char, color);
        self.draw_vline(x + w - 1, y + 1, h - 2, v_char, color);
    }

    /// Draw a focus ring with automatic style based on context
    pub fn draw_focus_ring_auto(&mut self, x: u16, y: u16, w: u16, h: u16, color: Color) {
        self.draw_focus_ring(x, y, w, h, color, FocusStyle::Rounded);
    }

    /// Draw a focus underline (for inline elements)
    pub fn draw_focus_underline(&mut self, x: u16, y: u16, w: u16, color: Color) {
        for i in 0..w {
            let cell = Cell::new('▔').fg(color);
            self.buffer.set(x + i, y, cell);
        }
    }

    /// Draw a focus indicator at a specific position
    pub fn draw_focus_marker(&mut self, x: u16, y: u16, color: Color) {
        self.draw_char(x, y, '▶', color);
    }

    /// Draw a focus indicator on the left side of an item
    pub fn draw_focus_marker_left(&mut self, y: u16, color: Color) {
        if self.area.x > 0 {
            self.draw_char(self.area.x - 1, y, '▶', color);
        } else {
            self.draw_char(self.area.x, y, '▶', color);
        }
    }

    /// Invert colors in a region (for high contrast focus indication)
    pub fn invert_colors(&mut self, x: u16, y: u16, w: u16, h: u16) {
        for dy in 0..h {
            for dx in 0..w {
                if let Some(cell) = self.buffer.get_mut(x + dx, y + dy) {
                    let old_fg = cell.fg;
                    let old_bg = cell.bg;
                    cell.fg = old_bg;
                    cell.bg = old_fg;
                }
            }
        }
    }

    /// Add reverse video effect to indicate focus
    pub fn draw_focus_reverse(&mut self, x: u16, y: u16, w: u16, h: u16) {
        self.invert_colors(x, y, w, h);
    }

    /// Apply a focus indicator around the full area, opt-in per widget
    ///
    /// Widgets call this at render start: `ctx.apply_focus_indicator(self.focused, style, color)`
    pub fn apply_focus_indicator(&mut self, focused: bool, style: FocusStyle, color: Color) {
        if focused {
            let area = self.area;
            self.draw_focus_ring(area.x, area.y, area.width, area.height, color, style);
        }
    }

    /// Apply a default focus indicator (Rounded + Cyan)
    ///
    /// Convenience method for the most common case.
    pub fn apply_default_focus(&mut self, focused: bool) {
        self.apply_focus_indicator(focused, FocusStyle::Rounded, Color::CYAN);
    }
}

use crate::widget::traits::render_context::RenderContext;
