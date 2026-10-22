//! RichTextEditor widget tests

mod block;
mod core;
mod cursor;
mod dialog;
mod edge_cases;
mod editing;
mod editor_view;
mod format;
mod link;
mod markdown_export;
mod markdown_parse;
mod props;
mod selection;
mod text_edit;
mod text_format;
mod toolbar;
mod types;
mod undo;
mod view;

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::RichTextEditor;

/// The editor's default colors
pub const BG: Color = Color::rgb(30, 30, 46);
pub const FG: Color = Color::rgb(205, 214, 244);
pub const TOOLBAR_BG: Color = Color::rgb(49, 50, 68);
pub const CURSOR_BG: Color = Color::rgb(166, 227, 161);
pub const SELECTION_BG: Color = Color::rgb(69, 71, 90);
pub const PREVIEW_BG: Color = Color::rgb(24, 24, 37);
pub const HEADING_FG: Color = Color::rgb(137, 180, 250);
pub const QUOTE_FG: Color = Color::rgb(166, 173, 200);
pub const CODE_BG: Color = Color::rgb(49, 50, 68);
pub const DIALOG_BG: Color = Color::rgb(49, 50, 68);

/// The toolbar row, trailing blanks trimmed
pub const TOOLBAR: &str = " B I U S ` │H1 H2 H3 │\" • 1. │🔗 📷";

/// Render `editor` in a `w` x `h` buffer
pub fn screen(editor: &RichTextEditor, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    editor.render(&mut ctx);
    buffer
}

/// The symbols of row `y` (a wide glyph once), trailing blanks trimmed
pub fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap())
        .filter(|cell| !cell.is_continuation())
        .map(|cell| cell.symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// The cells with background `bg`, as (x, y)
pub fn cells_with_bg(buffer: &Buffer, bg: Color) -> Vec<(u16, u16)> {
    let mut out = Vec::new();
    for y in 0..buffer.height() {
        for x in 0..buffer.width() {
            if buffer.get(x, y).unwrap().bg == Some(bg) {
                out.push((x, y));
            }
        }
    }
    out
}
