//! Rendering implementation for the FileTree widget

use super::types::{FileEntry, FileType};
use super::FileTree;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::unicode::display_width;
use crate::widget::theme::LIGHT_GRAY;
use crate::widget::traits::{RenderContext, View};

impl View for FileTree {
    crate::impl_view_meta!("FileTree");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        // The type colors are how the tree is read - a cyan directory against a
        // magenta symlink - so a `color` rule cannot have all of them. It gets
        // the plain file, which is the base every other type is a departure
        // from. Same split the syntax-colored viewers took.
        let base_fg = ctx.css_color(Color::WHITE);
        let entries = self.visible_entries();
        let visible_height = if self.height > 0 {
            self.height
        } else {
            area.height
        } as usize;
        // The rows in view, following the selection (#967, #973)
        self.scroll.set_content(entries.len());
        self.scroll
            .set_viewport(visible_height.min(area.height as usize));
        let selected = self.selected_index();
        self.scroll.track(selected..selected + 1);
        let scroll = self.scroll.offset();

        for (i, entry) in entries.iter().skip(scroll).take(visible_height).enumerate() {
            let y = i as u16;
            if y >= area.height {
                break;
            }
            let row = Row {
                y,
                bg: (scroll + i == selected).then_some(self.selected_bg),
                base_fg,
            };
            self.render_row(ctx, entry, &row);
        }
    }
}

/// What one row needs beyond its entry
struct Row {
    y: u16,
    /// The selection background, when this row is selected
    bg: Option<Color>,
    /// Plain-file color, from the stylesheet
    base_fg: Color,
}

impl Row {
    fn type_fg(&self, file_type: FileType) -> Color {
        if file_type == FileType::File {
            self.base_fg
        } else {
            file_type.color()
        }
    }

    fn blank(&self) -> Cell {
        let mut cell = Cell::new(' ');
        cell.bg = self.bg;
        cell
    }

    /// Draw `text` from `x`, stopping before any character that would cross
    /// the area edge; returns the column after it.
    fn text(&self, ctx: &mut RenderContext, x: u16, text: &str, fg: Color, bold: bool) -> u16 {
        let max = ctx.area.width.saturating_sub(x);
        match (self.bg, bold) {
            (Some(bg), true) => ctx.draw_text_clipped_bg_bold(x, self.y, text, fg, bg, max),
            (Some(bg), false) => ctx.draw_text_clipped_bg(x, self.y, text, fg, bg, max),
            (None, true) => ctx.draw_text_clipped_bold(x, self.y, text, fg, max),
            (None, false) => ctx.draw_text_clipped(x, self.y, text, fg, max),
        }
        x.saturating_add(display_width(text) as u16)
    }
}

impl FileTree {
    fn render_row(&self, ctx: &mut RenderContext, entry: &FileEntry, row: &Row) {
        let width = ctx.area.width;
        for x in 0..width {
            ctx.set(x, row.y, row.blank());
        }

        let mut x = (entry.depth as u16).saturating_mul(self.indent);

        // Expand/collapse indicator, or the space it would take
        if entry.is_dir() {
            let indicator = if entry.expanded { "▼" } else { "▶" };
            row.text(ctx, x, indicator, self.dir_fg, false);
        }
        x = x.saturating_add(2);

        if self.show_icons {
            let icon = if self.simple_icons {
                entry.file_type.simple_icon()
            } else {
                entry.file_type.icon()
            };
            row.text(
                ctx,
                x,
                icon.encode_utf8(&mut [0; 4]),
                row.type_fg(entry.file_type),
                false,
            );
            x = x.saturating_add(2);
        }

        let name_fg = if row.bg.is_some() {
            self.selected_fg
        } else {
            row.type_fg(entry.file_type)
        };
        x = row.text(ctx, x, &entry.name, name_fg, entry.is_dir());

        // Size, right-aligned with one free column, when the name leaves room
        if self.show_sizes && !entry.is_dir() {
            let size = entry.format_size();
            let size_width = display_width(&size) as u16;
            if let Some(size_x) = width.checked_sub(size_width.saturating_add(1)) {
                if size_x > x {
                    row.text(ctx, size_x, &size, LIGHT_GRAY, false);
                }
            }
        }
    }
}
