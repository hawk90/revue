//! Selection and the clipboard

use super::Input;

impl Input {
    /// The selection's (start, end) in characters, if anything is selected
    pub fn selection(&self) -> Option<(usize, usize)> {
        let cursor = self.state.primary();
        cursor.has_selection().then(|| {
            let (from, to) = cursor.range();
            (self.char_of(from.col), self.char_of(to.col))
        })
    }

    /// Get selected text
    pub fn selected_text(&self) -> Option<&str> {
        self.selection()
            .map(|(start, end)| self.substring(start, end))
    }

    /// Check if there's an active selection
    pub fn has_selection(&self) -> bool {
        self.state.has_selection()
    }

    /// Kept for 3.x: a selection now starts with the first Shift move (or
    /// [`select_all`](Self::select_all)), so this does nothing
    #[deprecated(
        since = "3.12.0",
        note = "does nothing: a selection starts with the first Shift move or select_all"
    )]
    pub fn start_selection(&mut self) {}

    /// Clear selection, keeping the cursor where it is
    pub fn clear_selection(&mut self) {
        let cursor = self.state.cursor();
        self.state.set_cursor(cursor);
    }

    /// Select all text
    pub fn select_all(&mut self) {
        self.state.select_all();
    }

    /// Copy selection to clipboard
    pub fn copy(&mut self) {
        if let Some(text) = self.selected_text().map(|s| s.to_string()) {
            self.clipboard = Some(text.clone());
            // Try to copy to system clipboard
            #[cfg(feature = "clipboard")]
            if let Ok(mut ctx) = arboard::Clipboard::new() {
                let _ = ctx.set_text(&text);
            }
        }
    }

    /// Cut selection to clipboard
    pub fn cut(&mut self) -> bool {
        self.copy();
        self.state.delete_selection()
    }

    /// Paste from the clipboard, replacing the selection, as one undo step.
    /// Newlines and tabs become spaces.
    pub fn paste(&mut self) -> bool {
        // Try system clipboard first
        #[cfg(feature = "clipboard")]
        if let Ok(mut ctx) = arboard::Clipboard::new() {
            if let Ok(text) = ctx.get_text() {
                if !text.is_empty() {
                    return self.paste_text(&text);
                }
            }
        }
        // Fall back to internal clipboard
        match self.clipboard.clone() {
            Some(text) => self.paste_text(&text),
            None => false,
        }
    }
}
