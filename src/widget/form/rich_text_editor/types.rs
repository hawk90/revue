//! Common types for rich text editor

/// Toolbar action
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolbarAction {
    /// Toggle bold formatting
    Bold,
    /// Toggle italic formatting
    Italic,
    /// Toggle underline formatting
    Underline,
    /// Toggle strikethrough formatting
    Strikethrough,
    /// Toggle inline code formatting
    Code,
    /// Insert link
    Link,
    /// Insert image
    Image,
    /// Set block to heading 1
    Heading1,
    /// Set block to heading 2
    Heading2,
    /// Set block to heading 3
    Heading3,
    /// Set block to quote
    Quote,
    /// Set block to bullet list
    BulletList,
    /// Set block to numbered list
    NumberedList,
    /// Set block to code block
    CodeBlock,
    /// Insert horizontal rule
    HorizontalRule,
    /// Undo last action
    Undo,
    /// Redo last undone action
    Redo,
}

/// View mode for the editor
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EditorViewMode {
    /// Editor only
    #[default]
    Editor,
    /// Preview only
    Preview,
    /// Split view (editor + preview)
    Split,
}
