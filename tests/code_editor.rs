//! Tests for CodeEditor widget - split into modules

#[path = "code_editor/basic.rs"]
mod basic;
#[path = "code_editor/bracket.rs"]
mod bracket;
#[path = "code_editor/config.rs"]
mod config;
#[path = "code_editor/cursor.rs"]
mod cursor;
#[path = "code_editor/edge_cases.rs"]
mod edge_cases;
#[path = "code_editor/find.rs"]
mod find;
#[path = "code_editor/goto_line.rs"]
mod goto_line;
#[path = "code_editor/language.rs"]
mod language;
#[path = "code_editor/rendering.rs"]
mod rendering;
#[path = "code_editor/selection.rs"]
mod selection;
#[path = "code_editor/text_edit.rs"]
mod text_edit;
#[path = "code_editor/undo_redo.rs"]
mod undo_redo;
