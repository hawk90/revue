//! Undo/redo methods for TextArea

impl TextArea {
    /// Undo the last edit, putting back the cursors and selections it
    /// started with
    pub fn undo(&mut self) {
        self.selecting = false;
        self.edit.undo();
    }

    /// Redo the last undone edit, with the cursors it ended with
    pub fn redo(&mut self) {
        self.selecting = false;
        self.edit.redo();
    }

    /// Whether there is an edit to undo
    pub fn can_undo(&self) -> bool {
        self.edit.can_undo()
    }

    /// Whether there is an edit to redo
    pub fn can_redo(&self) -> bool {
        self.edit.can_redo()
    }
}

use super::TextArea;
