//! One line of the editor: current-line band, line number, text and the
//! cursor past the line's end

use super::TextColumns;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::code_editor::{BracketMatch, CodeEditor};
use crate::widget::syntax::HighlightSpan;
use crate::widget::traits::render_context::edit_line::col_of;
use crate::widget::traits::RenderContext;

impl CodeEditor {
    /// Draw line `line_idx` on row `y`
    pub(super) fn draw_line(
        &self,
        ctx: &mut RenderContext,
        y: u16,
        line_idx: usize,
        columns: &TextColumns,
        bracket_match: Option<&BracketMatch>,
    ) {
        let line_num_width = columns.x;
        let text_width = columns.width;
        let scroll_x = columns.scroll_x;
        let line = self.line_text(line_idx);
        let is_current_line = line_idx == self.edit.cursor().line;

        // Current line highlight
        if self.config.highlight_current_line && is_current_line && self.focused {
            for x in line_num_width..line_num_width + text_width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.current_line_bg);
                ctx.set(x, y, cell);
            }
        }

        // Draw line numbers
        if self.show_line_numbers {
            self.draw_line_number(ctx, y, line_idx, line_num_width, is_current_line);
        }

        // Get syntax highlights
        let highlights = self.get_highlights(line);

        // Draw text, in columns: a wide glyph takes two cells.
        ctx.put_edit_line(
            line_num_width,
            y,
            line,
            scroll_x,
            text_width,
            |char_idx, ch| {
                self.text_cell(
                    columns,
                    (line_idx, char_idx),
                    ch,
                    is_current_line,
                    bracket_match,
                    &highlights,
                )
            },
        );

        // Draw cursor at end of line if needed
        if columns
            .cursor
            .is_some_and(|(l, c)| l == line_idx && c >= line.chars().count())
        {
            if let Some(x) = col_of(line, usize::MAX).checked_sub(scroll_x) {
                if x < text_width as usize {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(self.cursor_bg);
                    ctx.set(line_num_width + x as u16, y, cell);
                }
            }
        }
    }

    /// Draw the number of line `line_idx`, right-aligned in the gutter
    fn draw_line_number(
        &self,
        ctx: &mut RenderContext,
        y: u16,
        line_idx: usize,
        line_num_width: u16,
        is_current_line: bool,
    ) {
        let num_str = format!(
            "{:>width$} ",
            line_idx + 1,
            width = (line_num_width - 2) as usize
        );
        for (i, ch) in num_str.chars().enumerate() {
            if (i as u16) < line_num_width {
                let mut cell = Cell::new(ch);
                cell.fg = Some(if is_current_line && self.focused {
                    Color::WHITE
                } else {
                    self.line_number_fg
                });
                cell.bg = self.bg;
                ctx.set(i as u16, y, cell);
            }
        }
    }

    /// The cell for char `ch` at `(line, char index)`: cursor, selection,
    /// bracket match and find match win over syntax colors, in that order
    fn text_cell(
        &self,
        columns: &TextColumns,
        (line_idx, char_idx): (usize, usize),
        ch: char,
        is_current_line: bool,
        bracket_match: Option<&BracketMatch>,
        highlights: &[HighlightSpan],
    ) -> Cell {
        // Whitespace markers, drawn in the line-number color
        let marker = match ch {
            ' ' if self.config.show_whitespace => Some('·'),
            '\t' if self.config.show_whitespace => Some('→'),
            _ => None,
        };
        let mut cell = Cell::new(marker.unwrap_or(ch));

        // Check cursor position
        let is_cursor = columns.cursor == Some((line_idx, char_idx));

        // Check selection
        let is_selected = super::selected(columns.selection, line_idx, char_idx);

        // Check bracket match
        let is_bracket_match = bracket_match
            .as_ref()
            .map(|m| m.position == (line_idx, char_idx))
            .unwrap_or(false);

        // Check find match
        let find_match = self.get_find_match_at(line_idx, char_idx);

        if is_cursor {
            cell.bg = Some(self.cursor_bg);
            cell.fg = Some(Color::BLACK);
        } else if is_selected {
            cell.bg = Some(self.selection_bg);
            cell.fg = self.fg;
        } else if is_bracket_match {
            cell.bg = Some(self.bracket_match_bg);
            cell.fg = Some(Color::BLACK);
            cell.modifier |= crate::render::Modifier::BOLD;
        } else if let Some((is_current, _)) = find_match {
            cell.bg = Some(if is_current {
                self.current_find_bg
            } else {
                self.find_match_bg
            });
            cell.fg = Some(Color::BLACK);
        } else {
            // Apply syntax highlighting
            let mut fg_set = false;
            for span in highlights {
                if char_idx >= span.start && char_idx < span.end {
                    cell.fg = Some(span.fg);
                    if span.bold {
                        cell.modifier |= crate::render::Modifier::BOLD;
                    }
                    if span.italic {
                        cell.modifier |= crate::render::Modifier::ITALIC;
                    }
                    fg_set = true;
                    break;
                }
            }
            if !fg_set {
                cell.fg = self.fg;
            }
            if marker.is_some() {
                cell.fg = Some(self.line_number_fg);
            }
            if self.config.highlight_current_line && is_current_line && self.focused {
                cell.bg = Some(self.current_line_bg);
            } else {
                cell.bg = self.bg;
            }
        }
        cell
    }
}
