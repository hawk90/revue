//! RichTextEditor formatting functionality
//!
//! This module contains text formatting and block type operations.

use super::core::RichTextEditor;
use super::{BlockType, EditOp, TextFormat};

impl RichTextEditor {
    /// Toggle bold format
    pub fn toggle_bold(&mut self) {
        self.current_format.bold = !self.current_format.bold;
    }

    /// Toggle italic format
    pub fn toggle_italic(&mut self) {
        self.current_format.italic = !self.current_format.italic;
    }

    /// Toggle underline format
    pub fn toggle_underline(&mut self) {
        self.current_format.underline = !self.current_format.underline;
    }

    /// Toggle strikethrough format
    pub fn toggle_strikethrough(&mut self) {
        self.current_format.strikethrough = !self.current_format.strikethrough;
    }

    /// Toggle code format
    pub fn toggle_code(&mut self) {
        self.current_format.code = !self.current_format.code;
    }

    /// Get current format
    pub fn current_format(&self) -> TextFormat {
        self.current_format
    }

    /// Set block type for current block
    pub fn set_block_type(&mut self, block_type: BlockType) {
        let old_type = self.blocks[self.cursor.0].block_type;

        // Record for undo
        self.undo_stack.push(EditOp::ChangeBlockType {
            block: self.cursor.0,
            old: old_type,
            new: block_type,
        });
        self.redo_stack.clear();

        self.blocks[self.cursor.0].block_type = block_type;
    }

    /// Get current block type
    pub fn current_block_type(&self) -> BlockType {
        self.blocks[self.cursor.0].block_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // toggle_bold tests
    // =========================================================================

    #[test]
    fn test_toggle_bold_from_default() {
        let mut editor = RichTextEditor::new();
        editor.toggle_bold();
        assert!(editor.current_format().bold);
    }

    #[test]
    fn test_toggle_bold_twice() {
        let mut editor = RichTextEditor::new();
        editor.toggle_bold();
        assert!(editor.current_format().bold);
        editor.toggle_bold();
        assert!(!editor.current_format().bold);
    }

    // =========================================================================
    // toggle_italic tests
    // =========================================================================

    #[test]
    fn test_toggle_italic_from_default() {
        let mut editor = RichTextEditor::new();
        editor.toggle_italic();
        assert!(editor.current_format().italic);
    }

    #[test]
    fn test_toggle_italic_twice() {
        let mut editor = RichTextEditor::new();
        editor.toggle_italic();
        assert!(editor.current_format().italic);
        editor.toggle_italic();
        assert!(!editor.current_format().italic);
    }

    // =========================================================================
    // toggle_underline tests
    // =========================================================================

    #[test]
    fn test_toggle_underline_from_default() {
        let mut editor = RichTextEditor::new();
        editor.toggle_underline();
        assert!(editor.current_format().underline);
    }

    #[test]
    fn test_toggle_underline_twice() {
        let mut editor = RichTextEditor::new();
        editor.toggle_underline();
        assert!(editor.current_format().underline);
        editor.toggle_underline();
        assert!(!editor.current_format().underline);
    }

    // =========================================================================
    // toggle_strikethrough tests
    // =========================================================================

    #[test]
    fn test_toggle_strikethrough_from_default() {
        let mut editor = RichTextEditor::new();
        editor.toggle_strikethrough();
        assert!(editor.current_format().strikethrough);
    }

    #[test]
    fn test_toggle_strikethrough_twice() {
        let mut editor = RichTextEditor::new();
        editor.toggle_strikethrough();
        assert!(editor.current_format().strikethrough);
        editor.toggle_strikethrough();
        assert!(!editor.current_format().strikethrough);
    }

    // =========================================================================
    // toggle_code tests
    // =========================================================================

    #[test]
    fn test_toggle_code_from_default() {
        let mut editor = RichTextEditor::new();
        editor.toggle_code();
        assert!(editor.current_format().code);
    }

    #[test]
    fn test_toggle_code_twice() {
        let mut editor = RichTextEditor::new();
        editor.toggle_code();
        assert!(editor.current_format().code);
        editor.toggle_code();
        assert!(!editor.current_format().code);
    }

    // =========================================================================
    // current_format tests
    // =========================================================================

    #[test]
    fn test_current_format_default() {
        let editor = RichTextEditor::new();
        let fmt = editor.current_format();
        assert!(!fmt.bold);
        assert!(!fmt.italic);
        assert!(!fmt.underline);
        assert!(!fmt.strikethrough);
        assert!(!fmt.code);
    }

    #[test]
    fn test_current_format_after_toggles() {
        let mut editor = RichTextEditor::new();
        editor.toggle_bold();
        editor.toggle_italic();

        let fmt = editor.current_format();
        assert!(fmt.bold);
        assert!(fmt.italic);
        assert!(!fmt.underline);
        assert!(!fmt.strikethrough);
        assert!(!fmt.code);
    }

    // =========================================================================
    // set_block_type tests
    // =========================================================================

    #[test]
    fn test_set_block_type_heading1() {
        let mut editor = RichTextEditor::new();
        editor.set_block_type(BlockType::Heading1);
        assert_eq!(editor.current_block_type(), BlockType::Heading1);
    }

    #[test]
    fn test_set_block_type_quote() {
        let mut editor = RichTextEditor::new();
        editor.set_block_type(BlockType::Quote);
        assert_eq!(editor.current_block_type(), BlockType::Quote);
    }

    #[test]
    fn test_set_block_type_code_block() {
        let mut editor = RichTextEditor::new();
        editor.set_block_type(BlockType::CodeBlock);
        assert_eq!(editor.current_block_type(), BlockType::CodeBlock);
    }

    #[test]
    fn test_set_block_type_bullet_list() {
        let mut editor = RichTextEditor::new();
        editor.set_block_type(BlockType::BulletList);
        assert_eq!(editor.current_block_type(), BlockType::BulletList);
    }

    #[test]
    fn test_set_block_type_numbered_list() {
        let mut editor = RichTextEditor::new();
        editor.set_block_type(BlockType::NumberedList);
        assert_eq!(editor.current_block_type(), BlockType::NumberedList);
    }

    #[test]
    fn test_set_block_type_multiple() {
        let mut editor = RichTextEditor::new();
        editor.set_block_type(BlockType::Heading1);
        assert_eq!(editor.current_block_type(), BlockType::Heading1);

        editor.set_block_type(BlockType::Paragraph);
        assert_eq!(editor.current_block_type(), BlockType::Paragraph);

        editor.set_block_type(BlockType::Quote);
        assert_eq!(editor.current_block_type(), BlockType::Quote);
    }

    // =========================================================================
    // current_block_type tests
    // =========================================================================

    #[test]
    fn test_current_block_type_default() {
        let editor = RichTextEditor::new();
        assert_eq!(editor.current_block_type(), BlockType::Paragraph);
    }

    #[test]
    fn test_current_block_type_after_set() {
        let mut editor = RichTextEditor::new();
        editor.set_block_type(BlockType::Heading2);
        assert_eq!(editor.current_block_type(), BlockType::Heading2);
    }

    // =========================================================================
    // Format toggle chain tests
    // =========================================================================

    #[test]
    fn test_format_toggle_chain() {
        let mut editor = RichTextEditor::new();
        editor.toggle_bold();
        editor.toggle_italic();
        editor.toggle_underline();

        let fmt = editor.current_format();
        assert!(fmt.bold);
        assert!(fmt.italic);
        assert!(fmt.underline);
        assert!(!fmt.strikethrough);
        assert!(!fmt.code);
    }

    #[test]
    fn test_all_format_toggles() {
        let mut editor = RichTextEditor::new();
        editor.toggle_bold();
        editor.toggle_italic();
        editor.toggle_underline();
        editor.toggle_strikethrough();
        editor.toggle_code();

        let fmt = editor.current_format();
        assert!(fmt.bold);
        assert!(fmt.italic);
        assert!(fmt.underline);
        assert!(fmt.strikethrough);
        assert!(fmt.code);
    }

    #[test]
    fn test_format_persistence() {
        let mut editor = RichTextEditor::new();
        editor.toggle_bold();
        editor.toggle_code();

        // Format should persist across operations
        editor.set_block_type(BlockType::Heading1);
        assert!(editor.current_format().bold);
        assert!(editor.current_format().code);
    }
}
