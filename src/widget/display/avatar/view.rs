//! Drawing the avatar: one initial (small), the initials between two edge
//! glyphs (medium) or inside a three-row box (large), and the status dot

use super::{Avatar, AvatarShape, AvatarSize};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::unicode::char_width;
use crate::widget::traits::{RenderContext, View};

/// A bold initial on the avatar's background.
fn initial_cell(ch: char, fg: Color, bg: Color) -> Cell {
    let mut cell = Cell::new(ch);
    cell.fg = Some(fg);
    cell.bg = Some(bg);
    cell.modifier |= Modifier::BOLD;
    cell
}

/// The char a small avatar shows: its first that takes a column.
fn first_initial(initials: &str) -> char {
    initials
        .chars()
        .find(|ch| char_width(*ch) > 0)
        .unwrap_or('?')
}

/// The first two initials from column `x` of row `y`, each as wide as it
/// is drawn, within `cols` columns; one that would cross the last column is
/// left out, and a zero-width char is not an initial.
fn draw_initials(
    ctx: &mut RenderContext,
    x: u16,
    y: u16,
    cols: u16,
    initials: &str,
    fg: Color,
    bg: Color,
) {
    let mut dx = 0;
    for ch in initials.chars().filter(|ch| char_width(*ch) > 0).take(2) {
        let w = char_width(ch) as u16;
        if dx + w > cols {
            break;
        }
        ctx.set(x + dx, y, initial_cell(ch, fg, bg));
        dx += w;
    }
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
        let ch = first_initial(initials);
        ctx.set(0, 0, initial_cell(ch, fg, bg));

        self.draw_status(ctx, char_width(ch) as u16, 0);
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

        draw_initials(ctx, 1, 0, 2, initials, fg, bg);

        ctx.set(3, 0, edge_cell(right, bg));

        self.draw_status(ctx, 4, 0);
    }

    /// A 5x3 box with the initials in its middle row and the status dot past
    /// its bottom-right corner; in fewer than three rows, the medium form.
    fn draw_large(&self, ctx: &mut RenderContext, initials: &str, fg: Color, bg: Color) {
        // 3 lines tall, 5+ chars wide
        if ctx.area.height < 3 {
            self.draw_medium(ctx, initials, fg, bg);
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

        // The background fills the box, then the initials go over it
        for i in 1..4 {
            ctx.set(i, 1, initial_cell(' ', fg, bg));
        }
        draw_initials(ctx, 1, 1, 3, initials, fg, bg);

        ctx.set(4, 1, edge_cell('│', bg));

        for (i, ch) in chars_bot.iter().enumerate() {
            ctx.set(i as u16, 2, edge_cell(*ch, bg));
        }

        self.draw_status(ctx, 5, 2);
    }
}

impl View for Avatar {
    /// The shape's cells, and a column for the status dot when there is one.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let (w, h) = match self.size {
            AvatarSize::Small => (char_width(first_initial(&self.get_initials())) as u16, 1),
            AvatarSize::Large if max_height >= 3 => (5, 3),
            // Below three rows a large avatar draws the medium form.
            AvatarSize::Medium | AvatarSize::Large => (4, 1),
        };
        let w = w + u16::from(self.status.is_some());
        Some((w.min(max_width), h.min(max_height)))
    }
    crate::impl_view_meta!("Avatar");

    fn render(&self, ctx: &mut RenderContext) {
        // The edges are drawn in the background color, so the stylesheet's
        // goes into the shape and not around it, as with the builder's.
        ctx.paint_background_to_shape();
        let initials = self.get_initials();
        // Builder, then the stylesheet, then the name's own color.
        let bg = self
            .bg_color
            .or_else(|| ctx.css_background_if_set())
            .unwrap_or_else(|| self.name_color());
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
