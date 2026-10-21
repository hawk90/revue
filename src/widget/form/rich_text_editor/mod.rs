//! RichTextEditor widget for rich text editing with markdown support
//!
//! A WYSIWYG-style text editor with formatting toolbar, markdown shortcuts,
//! live preview, and export capabilities.

// Public modules for testing
pub mod block;
pub mod core;
pub mod cursor;
pub mod dialog;
mod dialog_view;
pub mod editing;
mod editor_view;
pub mod format;
pub mod link;
mod markdown;
mod preview;
pub mod selection;
pub mod text_edit;
pub mod text_format;
mod toolbar;
pub mod types;
pub mod undo;
mod view;

// KEEP HERE - Private implementation tests in tests/ directory (accesses private fields of RichTextEditor)
#[cfg(test)]
mod tests;

use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

// Public exports (also used internally)
pub use block::{Block, BlockType, FormattedSpan};
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
    /// Blocks `index..index + old.len()` became `new`: one user action made
    /// of several changes (a markdown shortcut, a selection delete, a merge,
    /// emptying the last block), undone and redone as one step (#1131)
    ReplaceBlocks {
        index: usize,
        old: Vec<Block>,
        new: Vec<Block>,
        cursor_before: (usize, usize),
        cursor_after: (usize, usize),
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

/// Rich text editor widget
#[derive(Clone)]
pub struct RichTextEditor {
    /// Document blocks
    blocks: Vec<Block>,
    /// Cursor position (block, col)
    cursor: (usize, usize),
    /// Selection anchor (if selecting)
    anchor: Option<(usize, usize)>,
    /// Scroll offset
    scroll: usize,
    /// Current format for new text
    current_format: TextFormat,
    /// Undo stack
    undo_stack: Vec<EditOp>,
    /// Redo stack
    redo_stack: Vec<EditOp>,
    /// View mode
    view_mode: EditorViewMode,
    /// Show toolbar
    show_toolbar: bool,
    /// Focused state
    focused: bool,
    /// Active dialog
    dialog: DialogType,
    /// Colors
    bg: Option<Color>,
    fg: Option<Color>,
    toolbar_bg: Color,
    toolbar_fg: Color,
    toolbar_active_bg: Color,
    cursor_bg: Color,
    selection_bg: Color,
    preview_bg: Color,
    heading_fg: Color,
    code_bg: Color,
    quote_fg: Color,
    link_fg: Color,
    /// Widget props
    props: crate::widget::traits::WidgetProps,
}

/// The editor's own fill and text color when nothing says otherwise.
///
/// These live here rather than in `new()` because an `Option` filled in at
/// construction cannot express "the builder said nothing" - the state a
/// stylesheet needs to reach. The default is applied where the color is
/// painted instead.
const EDITOR_BG: Color = Color::rgb(30, 30, 46);
const EDITOR_FG: Color = Color::rgb(205, 214, 244);

impl RichTextEditor {
    /// Create a new rich text editor
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::paragraph("")],
            cursor: (0, 0),
            anchor: None,
            scroll: 0,
            current_format: TextFormat::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            view_mode: EditorViewMode::Editor,
            show_toolbar: true,
            focused: true,
            dialog: DialogType::None,
            bg: None,
            fg: None,
            toolbar_bg: Color::rgb(49, 50, 68),
            toolbar_fg: Color::rgb(166, 173, 200),
            toolbar_active_bg: Color::rgb(137, 180, 250),
            cursor_bg: Color::rgb(166, 227, 161),
            selection_bg: Color::rgb(69, 71, 90),
            preview_bg: Color::rgb(24, 24, 37),
            heading_fg: Color::rgb(137, 180, 250),
            code_bg: Color::rgb(49, 50, 68),
            quote_fg: Color::rgb(166, 173, 200),
            link_fg: Color::rgb(137, 180, 250),
            props: crate::widget::traits::WidgetProps::new(),
        }
    }
}

impl Default for RichTextEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(RichTextEditor);
impl_props_builders!(RichTextEditor);

/// Create a new rich text editor
pub fn rich_text_editor() -> RichTextEditor {
    RichTextEditor::new()
}
