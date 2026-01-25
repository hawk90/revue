//! Form widgets - Form and input validation components
//!
//! Widgets for building forms with validation.

#[allow(clippy::module_inception)]
pub mod form;
pub mod masked_input;
pub mod rich_text_editor;

// Re-exports for convenience
pub use form::{form, form_field, ErrorDisplayStyle, Form, FormField, FormFieldWidget, InputType};
pub use masked_input::{
    credit_card_input, masked_input, password_input, pin_input, MaskStyle, MaskedInput,
    ValidationState,
};
pub use rich_text_editor::{
    rich_text_editor, Block, BlockType, EditorViewMode, FormattedSpan, ImageRef,
    Link as MarkdownLink, RichTextEditor, TextFormat, ToolbarAction,
};
