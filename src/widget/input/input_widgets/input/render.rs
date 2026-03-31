//! Rendering implementation for the Input widget

use super::types::Input;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::{RenderContext, View};

impl View for Input {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let display_text = if self.value.is_empty() && !self.focused {
            &self.placeholder
        } else {
            &self.value
        };

        let is_placeholder = self.value.is_empty() && !self.focused;
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

        let mut x: u16 = 0;
        for (i, ch) in display_text.chars().enumerate() {
            let char_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) as u16;
            if x + char_width > area.width {
                break;
            }

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

            ctx.set(x, 0, cell);
            x += char_width;
        }

        // Draw cursor at end if cursor is at the end of text
        if self.focused && self.cursor >= display_text.chars().count() && x < area.width {
            let mut cursor_cell = Cell::new(' ');
            cursor_cell.fg = self.cursor_fg;
            cursor_cell.bg = self.cursor_bg;
            ctx.set(x, 0, cursor_cell);
        }
    }

    crate::impl_view_meta!("Input");
}
