//! The value, and undo and redo

use super::Input;

impl Input {
    /// Get the current value
    pub fn get_value(&self) -> &str {
        self.line()
    }

    /// Clear the input (also clears undo history)
    pub fn clear(&mut self) {
        self.state.set_text("");
    }

    /// Set value programmatically (also clears undo history). Control
    /// characters are dropped, and newlines and tabs become spaces, since
    /// Input is a single-line widget. The cursor goes to the end.
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.state.set_text(value.into());
        let end = self.state.end();
        self.state.set_cursor(end);
    }

    /// Undo the last edit, putting back the cursor and selection it started
    /// with
    pub fn undo(&mut self) -> bool {
        self.state.undo()
    }

    /// Redo the last undone edit
    pub fn redo(&mut self) -> bool {
        self.state.redo()
    }

    /// Whether there is an edit to undo
    pub fn can_undo(&self) -> bool {
        self.state.can_undo()
    }

    /// Whether there is an edit to redo
    pub fn can_redo(&self) -> bool {
        self.state.can_redo()
    }

    /// Forget the undo and redo history
    pub fn clear_history(&mut self) {
        self.state.clear_history();
    }

    /// Insert `text` at the cursor, replacing the selection, as one undo
    /// step (paste)
    pub(super) fn paste_text(&mut self, text: &str) -> bool {
        self.state.paste(text)
    }
}
