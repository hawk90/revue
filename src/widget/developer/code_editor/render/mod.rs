//! Code editor rendering

mod dialogs;
mod line;
mod minimap;

use crate::widget::code_editor::CodeEditor;
use crate::widget::traits::render_context::edit_line::{
    col_of, cursor_width, scroll_to_cursor, visible_from,
};
use crate::widget::traits::{RenderContext, View};

/// The columns the text takes: from `x`, `width` wide, scrolled `scroll_x`
/// terminal columns; and where the cursor is drawn and what is selected, in
/// chars, worked out once per render
struct TextColumns {
    x: u16,
    width: u16,
    scroll_x: usize,
    cursor: Option<(usize, usize)>,
    selection: Option<((usize, usize), (usize, usize))>,
}

/// Whether `(line, col)` is inside `selection` (start inclusive, end not)
fn selected(selection: Option<((usize, usize), (usize, usize))>, line: usize, col: usize) -> bool {
    selection.is_some_and(|(start, end)| (start..end).contains(&(line, col)))
}

impl CodeEditor {
    // =========================================================================
    // Rendering Helpers
    // =========================================================================

    /// Get line number width
    #[doc(hidden)]
    pub fn line_number_width(&self) -> u16 {
        if self.show_line_numbers {
            let digits = format!("{}", self.edit.line_count()).len();
            (digits + 2) as u16
        } else {
            0
        }
    }

    /// Get syntax highlights for a line
    #[doc(hidden)]
    pub fn get_highlights(&self, line: &str) -> Vec<crate::widget::syntax::HighlightSpan> {
        self.highlighter
            .as_ref()
            .map(|h: &crate::widget::syntax::SyntaxHighlighter| h.highlight_line(line))
            .unwrap_or_default()
    }

    /// Check if position is in selection
    #[doc(hidden)]
    pub fn is_selected(&self, line: usize, col: usize) -> bool {
        selected(self.selection_chars(), line, col)
    }

    /// Check if position is in a find match
    #[doc(hidden)]
    pub fn get_find_match_at(&self, line: usize, col: usize) -> Option<(bool, usize)> {
        for (idx, &(m_line, m_start, m_end)) in self.find_matches.iter().enumerate() {
            if m_line == line && col >= m_start && col < m_end {
                return Some((idx == self.find_index, idx));
            }
        }
        None
    }

    /// Horizontal scroll, in columns, that keeps the cursor (and the whole
    /// glyph under it) in a view `text_width` columns wide
    fn scroll_x_for_cursor(&self, text_width: u16) -> usize {
        let (line, col) = self.cursor_position();
        let line = self.line_text(line);
        scroll_to_cursor(
            self.scroll_x.get(),
            col_of(line, col),
            cursor_width(line, col),
            col_of(line, usize::MAX) + 1,
            text_width as usize,
        )
    }
}

impl View for CodeEditor {
    crate::impl_view_meta!("CodeEditor");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }

        let line_num_width = self.line_number_width();
        let minimap_width = if self.config.show_minimap {
            self.config.minimap_width.min(area.width)
        } else {
            0
        };
        let text_width = area.width.saturating_sub(line_num_width + minimap_width);
        // The find and go-to-line boxes take row 0 while open; lines start
        // below them so the box never covers the cursor's line.
        let top: u16 = if (self.find_mode || self.goto_line_mode) && area.height > 1 {
            1
        } else {
            0
        };
        let visible_lines = (area.height - top) as usize;
        self.page_height.set(visible_lines);

        // Find matching bracket
        let bracket_match = self.find_matching_bracket();

        // Draw background
        if let Some(bg) = self.bg {
            ctx.fill_box_background(bg);
        }

        let scroll_x = self.scroll_x_for_cursor(text_width);
        self.scroll_x.set(scroll_x);

        // Scroll vertically to keep the cursor's line in view, moving the
        // first visible line as little as possible.
        let scroll = self.scroll_for_cursor(visible_lines);
        self.scroll.set(scroll);

        // Render visible lines
        let start_line = scroll;
        let end_line = (start_line + visible_lines).min(self.edit.line_count());
        let columns = TextColumns {
            x: line_num_width,
            width: text_width,
            scroll_x,
            cursor: self.focused.then(|| {
                // A zero-width char takes no cell: the cursor on one is
                // drawn on the next glyph that has one
                let (line, col) = self.cursor_position();
                (line, visible_from(self.line_text(line), col))
            }),
            selection: self.selection_chars(),
        };

        for (view_row, line_idx) in (start_line..end_line).enumerate() {
            let y = top + view_row as u16;
            self.draw_line(ctx, y, line_idx, &columns, bracket_match.as_ref());
        }

        // Draw minimap if enabled
        if self.config.show_minimap && minimap_width > 0 {
            self.draw_minimap(ctx, minimap_width, scroll, visible_lines);
        }

        // Draw go-to-line dialog
        if self.goto_line_mode {
            self.draw_goto_dialog(ctx);
        }

        // Draw find dialog
        if self.find_mode {
            self.draw_find_dialog(ctx);
        }
    }
}
