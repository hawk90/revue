//! Rendering implementation for the Input widget

use super::types::Input;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::render_context::edit_line::{col_of, cursor_width, scroll_to_cursor};
use crate::widget::traits::{RenderContext, View};

impl View for Input {
    /// One row, as wide as it is offered: the field scrolls its text, so
    /// its width is the layout's to choose.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some((max_width, 1.min(max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        // An empty input shows its placeholder whether or not it is focused,
        // as browsers and most toolkits do. `Input::new()` starts focused, so
        // hiding the placeholder while focused meant it never showed at all.
        // When focused, the cursor (at index 0) is drawn over the first
        // placeholder cell, so it stays visible.
        let is_placeholder = self.value.is_empty() && !self.placeholder.is_empty();
        let display_text = if is_placeholder {
            &self.placeholder
        } else {
            &self.value
        };
        let selection = self.selection();

        // Get CSS colors with priority: inline > CSS > default
        let css_fg = self.fg.or_else(|| {
            ctx.style.and_then(|s| {
                let c = s.visual.color;
                if c != Color::default() {
                    Some(c)
                } else {
                    None
                }
            })
        });
        let css_bg = self.bg.or_else(|| {
            ctx.style.and_then(|s| {
                let c = s.visual.background;
                if c != Color::default() {
                    Some(c)
                } else {
                    None
                }
            })
        });

        // The cursor is a char index; the screen is in columns, where a wide
        // glyph (Hangul, CJK, emoji) takes two. Scroll by columns so the
        // cursor, and the whole glyph under it, stays in view.
        let width = area.width as usize;
        let char_len = display_text.chars().count();
        let scroll = if is_placeholder {
            0
        } else {
            let line_w = col_of(display_text, char_len) + 1;
            scroll_to_cursor(
                self.scroll_x.get(),
                col_of(display_text, self.cursor),
                cursor_width(display_text, self.cursor),
                line_w,
                width,
            )
        };
        self.scroll_x.set(scroll);

        ctx.put_edit_line(0, 0, display_text, scroll, area.width, |i, ch| {
            let is_cursor = self.focused && i == self.cursor;
            let is_selected = selection.is_some_and(|(start, end)| i >= start && i < end);
            let mut cell = Cell::new(ch);

            if is_cursor {
                cell.fg = self.cursor_fg;
                cell.bg = self.cursor_bg;
            } else if is_selected {
                cell.fg = Some(Color::WHITE);
                cell.bg = self.selection_bg;
            } else if is_placeholder {
                cell.fg = Some(PLACEHOLDER_FG); // Gray for placeholder
            } else {
                cell.fg = css_fg;
                cell.bg = css_bg;
            }
            cell
        });

        // Draw cursor at end if cursor is at the end of text
        if self.focused && self.cursor >= char_len {
            let x = col_of(display_text, char_len).saturating_sub(scroll);
            if x < width {
                let mut cursor_cell = Cell::new(' ');
                cursor_cell.fg = self.cursor_fg;
                cursor_cell.bg = self.cursor_bg;
                ctx.set(x as u16, 0, cursor_cell);
            }
        }
    }

    crate::impl_view_meta!("Input", focusable);
}
