//! The minimap: one row per slice of lines, a mark per non-blank char

use crate::render::Cell;
use crate::widget::code_editor::CodeEditor;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::RenderContext;

impl CodeEditor {
    /// Draw the minimap in the area's right `minimap_width` columns, marking
    /// the rows of the lines from `scroll` that the view shows
    pub(super) fn draw_minimap(
        &self,
        ctx: &mut RenderContext,
        minimap_width: u16,
        scroll: usize,
        visible_lines: usize,
    ) {
        let area = ctx.area;
        let minimap_x = area.width - minimap_width;

        // Minimap background
        for y in 0..area.height {
            for x in 0..minimap_width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.minimap_bg);
                ctx.set(minimap_x + x, y, cell);
            }
        }

        // Calculate minimap scale
        let total_lines = self.edit.line_count();
        let minimap_height = area.height as usize;
        let lines_per_row = (total_lines as f32 / minimap_height as f32).max(1.0);

        // Draw minimap content
        for row in 0..minimap_height {
            let start_line = (row as f32 * lines_per_row) as usize;
            let y = row as u16;

            // Highlight visible area
            if start_line >= scroll && start_line < scroll + visible_lines {
                for x in 0..minimap_width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(self.minimap_visible_bg);
                    ctx.set(minimap_x + x, y, cell);
                }
            }

            // Draw condensed line representation
            if let Some(line) = self.edit.line(start_line) {
                let chars: Vec<char> = line.chars().collect();
                for (i, &ch) in chars.iter().take(minimap_width as usize).enumerate() {
                    if !ch.is_whitespace() {
                        let mut cell = Cell::new('▪');
                        cell.fg = Some(PLACEHOLDER_FG);
                        ctx.set(minimap_x + i as u16, y, cell);
                    }
                }
            }
        }
    }
}
