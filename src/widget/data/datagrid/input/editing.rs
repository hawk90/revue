//! DataGrid cell editing

use crate::event::{Key, KeyEvent};
use crate::utils::EditOutcome;
use crate::widget::data::datagrid::DataGrid;
use crate::widget::traits::render_context::edit_line::EditField;

/// Cell editing state
///
/// The cell's text is edited with the shared single-line
/// [`EditState`](crate::utils::EditState) (#985). `buffer` and `cursor` are
/// kept in step with it after every key, and a value written to them is
/// read back before the next one.
#[derive(Clone, Debug)]
pub struct EditState {
    /// Currently editing
    pub active: bool,
    /// Row being edited (actual index, not filtered)
    pub row: usize,
    /// Column being edited
    pub col: usize,
    /// Edit buffer
    pub buffer: String,
    /// Cursor position in buffer (character index)
    pub cursor: usize,
    /// The edit model behind `buffer` and `cursor`
    field: crate::utils::EditState,
    /// Horizontal scroll of the cell, in columns, kept between renders
    scroll: std::cell::Cell<usize>,
}

impl Default for EditState {
    fn default() -> Self {
        Self {
            active: false,
            row: 0,
            col: 0,
            buffer: String::new(),
            cursor: 0,
            field: crate::utils::EditState::single_line(),
            scroll: std::cell::Cell::new(0),
        }
    }
}

impl EditState {
    /// Editing `value` at `row`, `col`, the cursor at its end
    fn editing(row: usize, col: usize, value: String) -> Self {
        let mut state = Self {
            active: true,
            row,
            col,
            buffer: value,
            ..Self::default()
        };
        state.cursor = state.buffer.chars().count();
        state.read_back();
        state
    }

    /// Take `buffer` and `cursor` into the model when they were changed
    /// from outside it
    fn read_back(&mut self) {
        let text = self.field.line(0).unwrap_or("");
        if text != self.buffer {
            self.field.set_text(&self.buffer);
            // The model may change the text (a newline becomes a space)
            self.buffer = self.field.line(0).unwrap_or("").to_string();
        }
        if self.field.char_index(self.field.cursor()) != self.cursor {
            let pos = self.field.pos_at_char(0, self.cursor);
            self.field.set_cursor(pos);
        }
        self.sync();
    }

    /// Copy the model's text and cursor into `buffer` and `cursor`
    fn sync(&mut self) {
        let text = self.field.line(0).unwrap_or("");
        if text != self.buffer {
            self.buffer = text.to_string();
        }
        self.cursor = self.field.char_index(self.field.cursor());
    }
}

impl DataGrid {
    /// Check if currently editing a cell
    pub fn is_editing(&self) -> bool {
        self.edit_state.active
    }

    /// Start editing the selected cell
    pub fn start_edit(&mut self) -> bool {
        // Early bounds check - no clone needed, just copy indices
        let selected_row = self.selected_row;
        let selected_col = self.selected_col;

        if selected_col >= self.columns.len() {
            return false;
        }

        if !self.columns[selected_col].editable {
            return false;
        }

        // Get actual row index from filtered cache (zero-copy access)
        let row_idx = match self.filtered_cache.get(selected_row) {
            Some(&idx) => idx,
            None => return false,
        };

        if row_idx >= self.rows.len() {
            return false;
        }

        // Get current cell value
        let col_key = &self.columns[selected_col].key;
        let value = self.rows[row_idx].get(col_key).unwrap_or("").to_string();

        self.edit_state = EditState::editing(row_idx, selected_col, value);
        true
    }

    /// Commit the current edit
    pub fn commit_edit(&mut self) -> bool {
        if !self.edit_state.active {
            return false;
        }

        // Validate bounds before accessing
        if self.edit_state.col >= self.columns.len() {
            self.edit_state.active = false;
            return false;
        }
        if self.edit_state.row >= self.rows.len() {
            self.edit_state.active = false;
            return false;
        }

        let col_key = self.columns[self.edit_state.col].key.clone();
        let row = &mut self.rows[self.edit_state.row];

        // Update the cell value
        if let Some(cell) = row.data.iter_mut().find(|(k, _)| k == &col_key) {
            cell.1 = self.edit_state.buffer.clone();
        } else {
            row.data.push((col_key, self.edit_state.buffer.clone()));
        }

        self.edit_state.active = false;
        self.recompute_cache();
        true
    }

    /// Cancel the current edit
    pub fn cancel_edit(&mut self) {
        self.edit_state.active = false;
    }

    /// Get the current edit buffer
    pub fn edit_buffer(&self) -> Option<&str> {
        if self.edit_state.active {
            Some(&self.edit_state.buffer)
        } else {
            None
        }
    }

    /// Handle key input in edit mode
    pub(crate) fn handle_edit_key(&mut self, key: &Key) -> bool {
        self.handle_edit_key_event(&KeyEvent::new(*key))
    }

    /// Handle a key in edit mode: Escape cancels, Enter commits, and the
    /// other keys are the standard editing keys every single-line field
    /// shares (#985)
    pub(crate) fn handle_edit_key_event(&mut self, event: &KeyEvent) -> bool {
        // Escape and Enter whatever the modifiers, as before
        match event.key {
            Key::Escape => {
                self.cancel_edit();
                true
            }
            Key::Enter => {
                self.commit_edit();
                true
            }
            _ => {
                let edit = &mut self.edit_state;
                edit.read_back();
                let outcome = edit.field.edit_key(event, 1);
                edit.sync();
                outcome != EditOutcome::Unhandled
            }
        }
    }

    /// The cell being edited, for drawing: its text, cursor and selection,
    /// and the scroll it was last drawn at
    pub(crate) fn edit_field(&self) -> (EditField<'_>, &std::cell::Cell<usize>) {
        let edit = &self.edit_state;
        let field = &edit.field;
        let in_step = field.line(0) == Some(edit.buffer.as_str())
            && field.char_index(field.cursor()) == edit.cursor;
        let shown = if in_step {
            EditField::of(field, "", true)
        } else {
            // Written from outside since the last key: draw what was written
            EditField {
                text: &edit.buffer,
                placeholder: "",
                cursor: Some(edit.cursor.min(edit.buffer.chars().count())),
                selection: None,
            }
        };
        (shown, &edit.scroll)
    }
}
