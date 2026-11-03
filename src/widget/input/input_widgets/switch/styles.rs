//! The six styles, each drawing the switch itself at (x, y)

use super::Switch;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::theme::{DISABLED_FG, SUBTLE_GRAY};
use crate::widget::traits::RenderContext;

/// The colors a switch is drawn in
pub(super) struct Palette {
    /// The on state: the knob and the fill
    pub on: Color,
    /// The off state's knob
    pub off: Color,
    /// The empty track
    pub track: Color,
    /// White parts: the iOS knob, the Text style's brackets
    pub plain: Color,
}

impl Switch {
    /// The switch's colors: its own, or one gray for every part when
    /// disabled (lighter when on, as Checkbox draws its mark)
    pub(super) fn palette(&self, ctx: &RenderContext) -> Palette {
        if self.disabled {
            let gray = if self.on {
                SUBTLE_GRAY
            } else {
                Color::rgb(70, 70, 70)
            };
            return Palette {
                on: gray,
                off: gray,
                track: gray,
                plain: DISABLED_FG,
            };
        }
        // A single `color` cannot describe every part of this widget, so it
        // sets the primary one, the on state, and the rest keep their
        // defaults
        Palette {
            on: self.on_color.unwrap_or_else(|| ctx.css_color(Color::GREEN)),
            off: self.off_color,
            track: self.track_color,
            plain: Color::WHITE,
        }
    }

    /// Whether the focus is drawn: focused and not disabled
    pub(super) fn shows_focus(&self) -> bool {
        self.focused && !self.disabled
    }

    /// Render default style
    pub(super) fn render_default(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        let p = self.palette(ctx);
        let on_color = p.on;
        let color = if self.on { on_color } else { p.off };
        let track_len = self.width.saturating_sub(2);

        // Opening bracket
        let mut open = Cell::new('[');
        open.fg = Some(if self.shows_focus() {
            Color::CYAN
        } else {
            color
        });
        ctx.set(x, y, open);

        // Track
        for i in 0..track_len {
            let is_knob = if self.on { i == track_len - 1 } else { i == 0 };

            let ch = if is_knob { '●' } else { '━' };
            let mut cell = Cell::new(ch);
            cell.fg = Some(if is_knob { color } else { p.track });
            ctx.set(x + 1 + i, y, cell);
        }

        // Closing bracket
        let mut close = Cell::new(']');
        close.fg = Some(if self.shows_focus() {
            Color::CYAN
        } else {
            color
        });
        ctx.set(x + self.width - 1, y, close);
    }

    /// Render iOS style
    pub(super) fn render_ios(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        let p = self.palette(ctx);
        let on_color = p.on;
        let color = if self.on { on_color } else { p.off };
        let bg = if self.on { on_color } else { p.track };
        let track_len = self.width.saturating_sub(2);

        // Opening paren
        let mut open = Cell::new('(');
        open.fg = Some(color);
        ctx.set(x, y, open);

        // Track with knob
        for i in 0..track_len {
            let is_knob = if self.on { i == track_len - 1 } else { i == 0 };

            let ch = if is_knob { '●' } else { ' ' };
            let mut cell = Cell::new(ch);
            cell.fg = Some(p.plain);
            cell.bg = Some(bg);
            ctx.set(x + 1 + i, y, cell);
        }

        // Closing paren
        let mut close = Cell::new(')');
        close.fg = Some(color);
        ctx.set(x + self.width - 1, y, close);
    }

    /// Render Material style
    pub(super) fn render_material(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        let p = self.palette(ctx);
        let on_color = p.on;
        let color = if self.on { on_color } else { p.off };
        let track_len = self.width;

        for i in 0..track_len {
            let is_left_knob = i == 0;
            let is_right_knob = i == track_len - 1;

            let (ch, fg) = if self.on {
                if is_right_knob {
                    ('●', color)
                } else if is_left_knob {
                    ('○', p.track)
                } else {
                    ('━', color)
                }
            } else if is_left_knob {
                ('●', color)
            } else if is_right_knob {
                ('○', p.track)
            } else {
                ('━', p.track)
            };

            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.set(x + i, y, cell);
        }
    }

    /// Render text style
    pub(super) fn render_text(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        let p = self.palette(ctx);
        let on_color = p.on;
        let (text, color) = if self.on {
            (self.on_text.as_deref().unwrap_or("ON"), on_color)
        } else {
            (self.off_text.as_deref().unwrap_or("OFF"), p.off)
        };

        let mut open = Cell::new('[');
        open.fg = Some(if self.shows_focus() {
            Color::CYAN
        } else {
            p.plain
        });
        ctx.set(x, y, open);

        let on = self.on;
        let text_width = ctx.put_str_with(x + 1, y, text, u16::MAX, |ch| {
            let mut cell = Cell::new(ch);
            cell.fg = Some(color);
            if on {
                cell.modifier |= Modifier::BOLD;
            }
            cell
        });

        let mut close = Cell::new(']');
        close.fg = Some(if self.shows_focus() {
            Color::CYAN
        } else {
            p.plain
        });
        ctx.set(x + 1 + text_width, y, close);
    }

    /// Render emoji style
    pub(super) fn render_emoji(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        let p = self.palette(ctx);
        let on_color = p.on;
        let ch = if self.on { '✅' } else { '❌' };
        let mut cell = Cell::new(ch);
        cell.fg = Some(if self.on { on_color } else { p.off });
        // A wide emoji: `set` writes its continuation cell too
        ctx.set(x, y, cell);
    }

    /// Render block style
    pub(super) fn render_block(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        let track_len = self.width;
        let half = track_len / 2;
        let p = self.palette(ctx);
        let on_color = p.on;

        for i in 0..track_len {
            let is_filled = if self.on { i >= half } else { i < half };
            let ch = if is_filled { '▓' } else { '░' };
            let color = if self.on { on_color } else { p.off };

            let mut cell = Cell::new(ch);
            cell.fg = Some(color);
            ctx.set(x + i, y, cell);
        }
    }
}
