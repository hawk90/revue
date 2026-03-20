//! Tests for RichTextEditor widget - split into modules

#[path = "rich_text_editor/basic.rs"]
mod basic;
#[path = "rich_text_editor/block_type.rs"]
mod block_type;
#[path = "rich_text_editor/cursor.rs"]
mod cursor;
#[path = "rich_text_editor/edge_cases.rs"]
mod edge_cases;
#[path = "rich_text_editor/formatting.rs"]
mod formatting;
#[path = "rich_text_editor/helpers.rs"]
mod helpers;
#[path = "rich_text_editor/link_image.rs"]
mod link_image;
#[path = "rich_text_editor/markdown_export.rs"]
mod markdown_export;
#[path = "rich_text_editor/markdown_parse.rs"]
mod markdown_parse;
#[path = "rich_text_editor/rendering.rs"]
mod rendering;
#[path = "rich_text_editor/selection.rs"]
mod selection;
#[path = "rich_text_editor/text_edit.rs"]
mod text_edit;
#[path = "rich_text_editor/toolbar.rs"]
mod toolbar;
#[path = "rich_text_editor/undo_redo.rs"]
mod undo_redo;
#[path = "rich_text_editor/view_mode.rs"]
mod view_mode;
