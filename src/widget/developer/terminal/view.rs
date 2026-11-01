//! Drawing: the background, the visible lines, the cursor, the input line
//! and the scroll indicator

#![allow(clippy::explicit_counter_loop)]

use super::types::CursorStyle;
use super::Terminal;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::{DARK_BG, SEPARATOR_COLOR};
use crate::widget::traits::render_context::edit_line::{EditField, FieldCell};
use crate::widget::traits::{RenderContext, View};

impl View for Terminal {
    crate::impl_view_meta!("Terminal");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 1 || area.height < 1 {
            return;
        }

        // The cell colors come from the *content* - the ANSI codes the program
        // wrote - so a rule cannot have them, the same reading a syntax
        // highlighter gets. The terminal's own defaults are the base.
        let default_fg = self
            .default_fg
            .unwrap_or_else(|| ctx.css_color(Color::WHITE));
        let default_bg = self
            .default_bg
            .unwrap_or_else(|| ctx.css_background(Color::BLACK));

        // Fill background
        for y in 0..area.height {
            for x in 0..area.width {
                ctx.set(x, y, Cell::new(' ').bg(default_bg));
            }
        }

        // The rows in view: as many as the area has (#973)
        self.scroll.set_viewport(area.height as usize);
        self.draw_lines(ctx, area, default_fg, default_bg);

        // Render cursor if visible and focused
        if self.show_cursor && self.focused {
            self.draw_cursor(ctx, area, default_fg);
        }

        // Render input line if there's input
        if !self.input_buffer.is_empty() && self.focused {
            self.draw_input_line(ctx, area, default_fg);
        }

        // Render scroll indicator if scrolled
        if self.lines_below() > 0 {
            self.draw_scroll_indicator(ctx, area);
        }
    }
}

impl Terminal {
    /// The lines `visible_range` picks, one per row from the top
    fn draw_lines(
        &self,
        ctx: &mut RenderContext,
        area: Rect,
        default_fg: Color,
        default_bg: Color,
    ) {
        // Get visible lines
        let visible = self.visible_range();
        let mut render_y = 0u16;

        for line_idx in visible {
            if render_y >= area.height {
                break;
            }

            if let Some(line) = self.lines.get(line_idx) {
                for (col, cell) in line.cells.iter().enumerate() {
                    if col >= area.width as usize {
                        break;
                    }
                    // The right half of a wide character: `set` writes it
                    // with the character
                    if cell.ch == '\0' {
                        continue;
                    }

                    // `Color::default()` is a color no ANSI code set (never
                    // set, or reset by 0, 39 or 49): the terminal's default.
                    // Any other color, white and black included, is the
                    // content's own, the reading a syntax highlighter's
                    // tokens get.
                    let fg = if cell.fg.is_default() {
                        default_fg
                    } else {
                        cell.fg
                    };
                    let bg = if cell.bg.is_default() {
                        default_bg
                    } else {
                        cell.bg
                    };
                    let mut render_cell = Cell::new(cell.ch).fg(fg).bg(bg);
                    render_cell.modifier = cell.modifiers;
                    ctx.set(col as u16, render_y, render_cell);
                }
            }

            render_y += 1;
        }
    }

    /// The cursor glyph at the write position, when its line is in view
    fn draw_cursor(&self, ctx: &mut RenderContext, area: Rect, default_fg: Color) {
        let visible = self.visible_range();
        if !visible.contains(&self.cursor_row) {
            return;
        }
        let cursor_screen_row = self.cursor_row - visible.start;

        if cursor_screen_row < area.height as usize && self.cursor_col < area.width as usize {
            let cursor_x = self.cursor_col as u16;
            let cursor_y = cursor_screen_row as u16;

            let cursor_char = match self.cursor_style {
                CursorStyle::Block => '█',
                CursorStyle::Underline => '_',
                CursorStyle::Bar => '│',
            };

            ctx.set(cursor_x, cursor_y, Cell::new(cursor_char).fg(default_fg));
        }
    }

    /// The `> ` prompt and the typed input over the bottom row
    fn draw_input_line(&self, ctx: &mut RenderContext, area: Rect, default_fg: Color) {
        let input_y = area.height - 1;
        let prompt = "> ";

        // Clear input line
        for x in 0..area.width {
            ctx.set(x, input_y, Cell::new(' ').bg(DARK_BG));
        }

        // Render prompt
        let mut px: u16 = 0;
        for ch in prompt.chars() {
            let cw = crate::utils::char_width(ch) as u16;
            if px + cw > area.width {
                break;
            }
            ctx.set(px, input_y, Cell::new(ch).fg(Color::CYAN).bg(DARK_BG));
            px += cw;
        }

        // Render input, with its cursor, scrolled to it
        let field = EditField::of(&self.input_buffer, "", true);
        let width = area.width.saturating_sub(px);
        let scroll = field.scroll(self.input_scroll.get(), width);
        self.input_scroll.set(scroll);
        let selected = crate::widget::traits::render_context::edit_line::selection_colors();
        ctx.put_edit_field(px, input_y, width, &field, scroll, |kind, ch| match kind {
            FieldCell::Cursor => Cell::new(ch).fg(DARK_BG).bg(default_fg),
            FieldCell::Selected => Cell::new(ch).fg(selected.0).bg(selected.1),
            FieldCell::Text | FieldCell::Placeholder => Cell::new(ch).fg(default_fg).bg(DARK_BG),
        });
    }

    /// `↑n` near the top-right corner: how many lines the view is scrolled up
    fn draw_scroll_indicator(&self, ctx: &mut RenderContext, area: Rect) {
        let indicator = format!("↑{}", self.lines_below());
        let indicator_len = indicator.len() as u16;
        if area.width > indicator_len + 1 {
            let start_x = area.width - indicator_len - 1;
            for (i, ch) in indicator.chars().enumerate() {
                ctx.set(
                    start_x + i as u16,
                    0,
                    Cell::new(ch).fg(Color::YELLOW).bg(SEPARATOR_COLOR),
                );
            }
        }
    }
}
