//! Skeleton drawing: the shading character and one method per shape

use super::{Skeleton, SkeletonShape};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::SEPARATOR_COLOR;
use crate::widget::traits::{RenderContext, View};

impl Skeleton {
    /// Get the skeleton character based on animation
    fn skeleton_char(&self) -> char {
        if self.animate {
            // Cycle through shading characters
            match self.frame % 4 {
                0 => '░',
                1 => '▒',
                2 => '▓',
                _ => '▒',
            }
        } else {
            self.wave_char
        }
    }

    /// The width a rectangle or paragraph fills: the set width within the
    /// area, or the whole area when it is 0.
    fn fill_width(&self, area_width: u16) -> u16 {
        if self.width > 0 {
            self.width.min(area_width)
        } else {
            area_width
        }
    }

    fn draw_rectangle(&self, ctx: &mut RenderContext, ch: char, color: Color) {
        let area = ctx.area;
        let width = self.fill_width(area.width);
        let height = self.height.min(area.height);

        for y in 0..height {
            for x in 0..width {
                put(ctx, x, y, ch, color);
            }
        }
    }

    fn draw_circle(&self, ctx: &mut RenderContext, ch: char, color: Color) {
        let area = ctx.area;
        // Simple circle representation using unicode
        // For small sizes, just use filled block
        let size = self.height.max(1).min(area.height);

        if size == 1 {
            put(ctx, 0, 0, '●', color);
        } else if size == 2 {
            // 2x2 circle
            let chars = ['╭', '╮', '╰', '╯'];
            for (i, c) in chars.iter().enumerate() {
                let x = (i % 2) as u16;
                let y = (i / 2) as u16;
                put(ctx, x, y, *c, color);
            }
        } else {
            // Larger circle approximation
            // Top row
            put(ctx, 0, 0, '╭', color);
            for x in 1..size - 1 {
                put(ctx, x, 0, '─', color);
            }
            put(ctx, size - 1, 0, '╮', color);

            // Middle rows
            for y in 1..size - 1 {
                put(ctx, 0, y, '│', color);
                for x in 1..size - 1 {
                    put(ctx, x, y, ch, color);
                }
                put(ctx, size - 1, y, '│', color);
            }

            // Bottom row
            put(ctx, 0, size - 1, '╰', color);
            for x in 1..size - 1 {
                put(ctx, x, size - 1, '─', color);
            }
            put(ctx, size - 1, size - 1, '╯', color);
        }
    }

    fn draw_paragraph(&self, ctx: &mut RenderContext, ch: char, color: Color) {
        let area = ctx.area;
        let width = self.fill_width(area.width);
        let lines = self.lines.min(area.height);

        for line in 0..lines {
            // Vary line lengths for realism
            let line_width = if line == lines - 1 {
                width * 2 / 3 // Last line shorter
            } else if line % 2 == 1 {
                width.saturating_sub(4) // Alternate lines slightly shorter
            } else {
                width
            };

            for x in 0..line_width {
                put(ctx, x, line, ch, color);
            }
        }
    }
}

/// Draw `ch` in `color` at (`x`, `y`) of the area.
fn put(ctx: &mut RenderContext, x: u16, y: u16, ch: char, color: Color) {
    let mut cell = Cell::new(ch);
    cell.fg = Some(color);
    ctx.set(x, y, cell);
}

impl View for Skeleton {
    crate::impl_view_meta!("Skeleton");

    fn render(&self, ctx: &mut RenderContext) {
        // The builder can only outrank the stylesheet by moving off the
        // initial value - see `RenderContext::color_or`.
        let color = self.color.unwrap_or_else(|| ctx.css_color(SEPARATOR_COLOR));
        let ch = self.skeleton_char();

        match self.shape {
            SkeletonShape::Rectangle => self.draw_rectangle(ctx, ch, color),
            SkeletonShape::Circle => self.draw_circle(ctx, ch, color),
            SkeletonShape::Paragraph => self.draw_paragraph(ctx, ch, color),
        }
    }
}
