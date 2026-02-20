//! RichTextEditor widget for rich text editing with markdown support
//!
//! A WYSIWYG-style text editor with formatting toolbar, markdown shortcuts,
//! live preview, and export capabilities.

// Public modules for testing
pub mod block;
pub mod core;
pub mod cursor;
pub mod dialog;
pub mod editing;
mod editor_view;
pub mod format;
pub mod link;
mod preview;
pub mod selection;
pub mod text_edit;
pub mod text_format;
mod toolbar;
pub mod types;
pub mod undo;

use crate::render::Cell;
use crate::widget::traits::{RenderContext, View};

// Public exports (also used internally)
pub use block::{Block, BlockType, FormattedSpan};
pub use core::RichTextEditor;
pub use link::{ImageRef, Link};
pub use text_format::TextFormat;
pub use types::{EditorViewMode, ToolbarAction};

/// Edit operation for undo/redo
#[derive(Clone, Debug)]
#[allow(dead_code)] // Used in undo.rs module
pub(super) enum EditOp {
    InsertChar {
        block: usize,
        col: usize,
        ch: char,
    },
    DeleteChar {
        block: usize,
        col: usize,
        ch: char,
    },
    InsertBlock {
        index: usize,
        block: Block,
    },
    DeleteBlock {
        index: usize,
        block: Block,
    },
    MergeBlocks {
        index: usize,
        split_col: usize,
    },
    SplitBlock {
        block: usize,
        col: usize,
    },
    ChangeBlockType {
        block: usize,
        old: BlockType,
        new: BlockType,
    },
    SetFormat {
        block: usize,
        start: usize,
        end: usize,
        old: TextFormat,
        new: TextFormat,
    },
}

/// Dialog type
#[derive(Clone, Debug)]
pub(super) enum DialogType {
    None,
    InsertLink {
        text: String,
        url: String,
        field: usize,
    },
    InsertImage {
        alt: String,
        src: String,
        field: usize,
    },
}

impl View for RichTextEditor {
    crate::impl_view_meta!("RichTextEditor");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 2 || area.height < 1 {
            return;
        }

        // Fill background
        if let Some(bg) = self.bg {
            for y in 0..area.height {
                for x in 0..area.width {
                    let mut cell = Cell::new(' ');
                    cell.bg = Some(bg);
                    ctx.buffer.set(area.x + x, area.y + y, cell);
                }
            }
        }

        let mut y = area.y;

        // Render toolbar if enabled
        if self.show_toolbar {
            self.render_toolbar(ctx, area.x, y, area.width);
            y += 1;
        }

        let content_height = area
            .height
            .saturating_sub(if self.show_toolbar { 1 } else { 0 });

        match self.view_mode {
            EditorViewMode::Editor => {
                self.render_editor(ctx, area.x, y, area.width, content_height);
            }
            EditorViewMode::Preview => {
                self.render_preview(ctx, area.x, y, area.width, content_height);
            }
            EditorViewMode::Split => {
                let half_width = area.width / 2;
                self.render_editor(ctx, area.x, y, half_width, content_height);
                self.render_preview(
                    ctx,
                    area.x + half_width,
                    y,
                    area.width - half_width,
                    content_height,
                );
            }
        }

        // Render dialog if open
        if self.is_dialog_open() {
            self.render_dialog(ctx, area.x, area.y, area.width, area.height);
        }
    }
}

/// Create a new rich text editor
pub fn rich_text_editor() -> RichTextEditor {
    RichTextEditor::new()
}
