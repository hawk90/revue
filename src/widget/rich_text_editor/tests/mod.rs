//! Tests for rich_text_editor widget

// Re-export parent module items for test modules
pub use crate::event::Key;
pub use crate::layout::Rect;
pub use crate::render::Buffer;
pub use crate::style::Color;
pub use crate::widget::rich_text_editor::*;
pub use crate::widget::traits::RenderContext;
pub use crate::widget::traits::View;

mod block;
mod block_type;
mod cursor_navigation;
mod edge_cases;
mod editor_creation;
mod formatted_span;
mod formatting;
mod image_ref;
mod key_handling;
mod link;
mod link_image_dialog;
mod markdown_parsing;
mod markdown_shortcuts;
mod render;
mod selection;
mod text_editing;
mod text_format;
mod toolbar;
mod undo_redo;
mod view_mode;
