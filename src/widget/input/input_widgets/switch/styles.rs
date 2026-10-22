//! The six styles, each drawing the switch itself at (x, y)

use super::Switch;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::RenderContext;

impl Switch {
    /// Render default style
    pub(super) fn render_default(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        // A single `color` cannot describe every part of this widget, so it
        // sets the primary one and the rest keep their defaults - the on state is the switch's primary element.
        let on_color = self.on_color.unwrap_or_else(|| ctx.css_color(Color::GREEN));
        let color = if self.on { on_color } else { self.off_color };
        let track_len = self.width.saturating_sub(2);

        // Opening bracket
        let mut open = Cell::new('[');
        open.fg = Some(if self.focused { Color::CYAN } else { color });
        ctx.set(x, y, open);

        // Track
        for i in 0..track_len {
            let is_knob = if self.on { i == track_len - 1 } else { i == 0 };

            let ch = if is_knob { '●' } else { '━' };
            let mut cell = Cell::new(ch);
            cell.fg = Some(if is_knob { color } else { self.track_color });
            ctx.set(x + 1 + i, y, cell);
        }

        // Closing bracket
        let mut close = Cell::new(']');
        close.fg = Some(if self.focused { Color::CYAN } else { color });
        ctx.set(x + self.width - 1, y, close);
    }

    /// Render iOS style
    pub(super) fn render_ios(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        // A single `color` cannot describe every part of this widget, so it
        // sets the primary one and the rest keep their defaults - the on state is the switch's primary element.
        let on_color = self.on_color.unwrap_or_else(|| ctx.css_color(Color::GREEN));
        let color = if self.on { on_color } else { self.off_color };
        let bg = if self.on { on_color } else { self.track_color };
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
            cell.fg = Some(Color::WHITE);
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
        // A single `color` cannot describe every part of this widget, so it
        // sets the primary one and the rest keep their defaults - the on state is the switch's primary element.
        let on_color = self.on_color.unwrap_or_else(|| ctx.css_color(Color::GREEN));
        let color = if self.on { on_color } else { self.off_color };
        let track_len = self.width;

        for i in 0..track_len {
            let is_left_knob = i == 0;
            let is_right_knob = i == track_len - 1;

            let (ch, fg) = if self.on {
                if is_right_knob {
                    ('●', color)
                } else if is_left_knob {
                    ('○', self.track_color)
                } else {
                    ('━', color)
                }
            } else if is_left_knob {
                ('●', color)
            } else if is_right_knob {
                ('○', self.track_color)
            } else {
                ('━', self.track_color)
            };

            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            ctx.set(x + i, y, cell);
        }
    }

    /// Render text style
    pub(super) fn render_text(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        // A single `color` cannot describe every part of this widget, so it
        // sets the primary one and the rest keep their defaults - the on state is the switch's primary element.
        let on_color = self.on_color.unwrap_or_else(|| ctx.css_color(Color::GREEN));
        let (text, color) = if self.on {
            (self.on_text.as_deref().unwrap_or("ON"), on_color)
        } else {
            (self.off_text.as_deref().unwrap_or("OFF"), self.off_color)
        };

        let mut open = Cell::new('[');
        open.fg = Some(if self.focused {
            Color::CYAN
        } else {
            Color::WHITE
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
        close.fg = Some(if self.focused {
            Color::CYAN
        } else {
            Color::WHITE
        });
        ctx.set(x + 1 + text_width, y, close);
    }

    /// Render emoji style
    pub(super) fn render_emoji(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        // A single `color` cannot describe every part of this widget, so it
        // sets the primary one and the rest keep their defaults - the on state is the switch's primary element.
        let on_color = self.on_color.unwrap_or_else(|| ctx.css_color(Color::GREEN));
        let ch = if self.on { '✅' } else { '❌' };
        let mut cell = Cell::new(ch);
        cell.fg = Some(if self.on { on_color } else { self.off_color });
        // A wide emoji: `set` writes its continuation cell too
        ctx.set(x, y, cell);
    }

    /// Render block style
    pub(super) fn render_block(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        let track_len = self.width;
        let half = track_len / 2;
        let on_color = self.on_color.unwrap_or_else(|| ctx.css_color(Color::GREEN));

        for i in 0..track_len {
            let is_filled = if self.on { i >= half } else { i < half };
            let ch = if is_filled { '▓' } else { '░' };
            let color = if self.on { on_color } else { self.off_color };

            let mut cell = Cell::new(ch);
            cell.fg = Some(color);
            ctx.set(x + i, y, cell);
        }
    }
}
