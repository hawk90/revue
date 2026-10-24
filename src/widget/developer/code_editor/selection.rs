//! Code editor selection

impl super::CodeEditor {
    // =========================================================================
    // Selection
    // =========================================================================

    /// Start selection mode at the cursor: every move (the move methods,
    /// the moving keys, [`set_cursor`](Self::set_cursor)) extends the
    /// selection from here until it is cleared or the text is edited. Shift
    /// with a moving key selects without it.
    pub fn start_selection(&mut self) {
        self.selecting = true;
    }

    /// Clear the selection, keeping the cursor where it is
    pub fn clear_selection(&mut self) {
        self.selecting = false;
        let head = self.edit.cursor();
        self.edit.set_cursor(head);
    }

    /// Whether text is selected, or selection mode is on
    pub fn has_selection(&self) -> bool {
        self.selecting || self.edit.has_selection()
    }

    /// Get selected text: empty in selection mode before the cursor moves
    pub fn get_selection(&self) -> Option<String> {
        let cursor = self.edit.primary();
        if cursor.has_selection() {
            let (from, to) = cursor.range();
            Some(self.edit.text_in(from, to))
        } else {
            self.selecting.then(String::new)
        }
    }

    /// Delete the selected text (nothing in a read-only editor)
    pub fn delete_selection(&mut self) {
        let changed = self.edit.delete_selection();
        self.edited(changed);
    }

    /// Select all, in selection mode (a move then extends the selection
    /// from the start of the text)
    pub fn select_all(&mut self) {
        self.edit.select_all();
        self.selecting = true;
    }

    /// The selection's start and end, in chars, when anything is selected
    pub(super) fn selection_chars(&self) -> Option<((usize, usize), (usize, usize))> {
        let cursor = self.edit.primary();
        cursor.has_selection().then(|| {
            let (from, to) = cursor.range();
            (self.char_pos(from), self.char_pos(to))
        })
    }
}
