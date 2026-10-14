//! Drawing the avatar: one initial (small), the initials between two edge
//! glyphs (medium) or inside a three-row box (large), and the status dot

use super::{Avatar, AvatarShape, AvatarSize};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

/// A bold initial on the avatar's background.
fn initial_cell(ch: char, fg: Color, bg: Color) -> Cell {
    let mut cell = Cell::new(ch);
    cell.fg = Some(fg);
    cell.bg = Some(bg);
    cell.modifier |= Modifier::BOLD;
    cell
}

/// An outline glyph drawn in the avatar's background color.
fn edge_cell(ch: char, bg: Color) -> Cell {
    let mut cell = Cell::new(ch);
    cell.fg = Some(bg);
    cell
}

impl Avatar {
    /// The status dot at `(x, y)`, if a status is set.
    fn draw_status(&self, ctx: &mut RenderContext, x: u16, y: u16) {
        if let Some(status_color) = self.status {
            let mut dot = Cell::new('●');
            dot.fg = Some(status_color);
            ctx.set(x, y, dot);
        }
    }

    /// One bold initial, then the status dot.
    fn draw_small(&self, ctx: &mut RenderContext, initials: &str, fg: Color, bg: Color) {
        // Single character
        let ch = initials.chars().next().unwrap_or('?');
        ctx.set(0, 0, initial_cell(ch, fg, bg));

        self.draw_status(ctx, 1, 0);
    }

    /// Two initials between the shape's edges, `◖XY◗`, `[XY]` or `(XY)`,
    /// then the status dot.
    fn draw_medium(&self, ctx: &mut RenderContext, initials: &str, fg: Color, bg: Color) {
        let (left, right) = match self.shape {
            // Use half-blocks for pseudo-circle: ◖XY◗
            AvatarShape::Circle => ('◖', '◗'),
            AvatarShape::Rounded => ('(', ')'),
            AvatarShape::Square => ('[', ']'),
        };

        ctx.set(0, 0, edge_cell(left, bg));

        for (i, ch) in initials.chars().take(2).enumerate() {
            ctx.set(1 + i as u16, 0, initial_cell(ch, fg, bg));
        }

        ctx.set(3, 0, edge_cell(right, bg));

        self.draw_status(ctx, 4, 0);
    }

    /// A 5x3 box with the initials in its middle row and the status dot past
    /// its bottom-right corner; in fewer than three rows, one plain initial.
    fn draw_large(&self, ctx: &mut RenderContext, initials: &str, fg: Color, bg: Color) {
        // 3 lines tall, 5+ chars wide
        if ctx.area.height < 3 {
            // Fall back to medium
            let mut cell = Cell::new(initials.chars().next().unwrap_or('?'));
            cell.fg = Some(fg);
            cell.bg = Some(bg);
            ctx.set(0, 0, cell);
            return;
        }

        // Top: ╭───╮ or ┌───┐ (Rounded draws as Circle)
        // Mid: │XY │
        // Bot: ╰───╯ or └───┘
        let (chars_top, chars_bot) = match self.shape {
            AvatarShape::Circle | AvatarShape::Rounded => {
                (['╭', '─', '─', '─', '╮'], ['╰', '─', '─', '─', '╯'])
            }
            AvatarShape::Square => (['┌', '─', '─', '─', '┐'], ['└', '─', '─', '─', '┘']),
        };

        for (i, ch) in chars_top.iter().enumerate() {
            ctx.set(i as u16, 0, edge_cell(*ch, bg));
        }

        // Middle row
        ctx.set(0, 1, edge_cell('│', bg));

        // Pre-collect initials chars for O(1) access
        let initials_chars: Vec<char> = initials.chars().collect();
        for i in 1..4 {
            let ch = if i == 1 || i == 2 {
                initials_chars.get(i - 1).copied().unwrap_or(' ')
            } else {
                ' '
            };
            ctx.set(i as u16, 1, initial_cell(ch, fg, bg));
        }

        ctx.set(4, 1, edge_cell('│', bg));

        for (i, ch) in chars_bot.iter().enumerate() {
            ctx.set(i as u16, 2, edge_cell(*ch, bg));
        }

        self.draw_status(ctx, 5, 2);
    }
}

impl View for Avatar {
    crate::impl_view_meta!("Avatar");

    fn render(&self, ctx: &mut RenderContext) {
        let initials = self.get_initials();
        let bg = self.get_bg_color();
        // Builder, then the stylesheet, then the widget's own default.
        let fg = self.fg_color.unwrap_or_else(|| ctx.css_color(Color::WHITE));

        match self.size {
            AvatarSize::Small => self.draw_small(ctx, &initials, fg, bg),
            AvatarSize::Medium => self.draw_medium(ctx, &initials, fg, bg),
            AvatarSize::Large => self.draw_large(ctx, &initials, fg, bg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn name_hue_reaches_the_magenta_range() {
        // "AAAAA" sums to 325, a hue in the magenta range (300..360).
        let avatar = Avatar::new("AAAAA").size(AvatarSize::Small);
        let mut buf = Buffer::new(2, 1);
        let mut ctx = RenderContext::new(&mut buf, Rect::new(0, 0, 2, 1));
        avatar.render(&mut ctx);
        let bg = buf.get(0, 0).unwrap().bg.unwrap();
        assert!(
            bg.r > bg.g && bg.b > bg.g,
            "hue 325 should be magenta (green lowest), got {bg:?}"
        );
    }
}
