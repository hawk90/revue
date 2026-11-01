//! The keys its help line lists, in each mode

use super::types::{PickerMode, PickerResult};
use super::FilePicker;
use crate::event::{Key, KeyEvent};
use crate::utils::EditOutcome;
use crate::widget::traits::{EventResult, Interactive};

impl FilePicker {
    /// Handle a key; returns whether it was the picker's. A key that finishes
    /// the picker leaves its answer for [`take_result`](Self::take_result).
    ///
    /// In every mode ↑↓ move the highlight and Esc cancels. In save mode any
    /// other key edits the file name, so a name may hold any letter, and Enter
    /// saves it, or opens the highlighted directory while the name is empty. In
    /// the other modes `q` cancels too, Backspace goes to the parent, and:
    ///
    /// - open: Enter opens a directory or selects a file, `h` shows hidden files
    /// - directory: Enter opens, Space selects the highlighted directory
    /// - multi-select: Enter opens a directory or confirms, Space toggles, `a`
    ///   selects every file listed, `n` clears the selection
    ///
    /// A Ctrl or Alt letter is left to the app.
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        match event.key {
            Key::Up => self.highlight_previous(),
            Key::Down => self.highlight_next(),
            Key::Escape => self.result = Some(PickerResult::Cancelled),
            _ if self.mode == PickerMode::Save => return self.save_key(event),
            _ if event.ctrl || event.alt => return false,
            Key::Backspace => self.go_up(),
            Key::Char('q') => self.result = Some(PickerResult::Cancelled),
            Key::Enter => self.enter_key(),
            Key::Char(' ') => return self.space_key(),
            Key::Char('h') if self.mode == PickerMode::Open => self.toggle_hidden(),
            Key::Char('a') if self.mode == PickerMode::MultiSelect => self.select_all(),
            Key::Char('n') if self.mode == PickerMode::MultiSelect => self.select_none(),
            _ => return false,
        }
        true
    }

    /// What the last key decided, if it finished the picker. Taking it clears it.
    pub fn take_result(&mut self) -> Option<PickerResult> {
        self.result.take()
    }

    fn save_key(&mut self, event: &KeyEvent) -> bool {
        if event.key == Key::Enter && !event.ctrl && !event.alt {
            if self.input_name.is_empty() {
                self.open_highlighted_dir();
            } else {
                self.finish_if_chosen();
            }
            return true;
        }
        self.input_name.edit_key(event, 1) != EditOutcome::Unhandled
    }

    fn enter_key(&mut self) {
        match self.mode {
            PickerMode::Open => self.result = self.enter(),
            PickerMode::Directory => {
                self.open_highlighted_dir();
            }
            PickerMode::MultiSelect => {
                if !self.open_highlighted_dir() {
                    self.finish_if_chosen();
                }
            }
            PickerMode::Save => {}
        }
    }

    fn space_key(&mut self) -> bool {
        match self.mode {
            PickerMode::Directory => self.finish_if_chosen(),
            PickerMode::MultiSelect => self.toggle_selection(),
            PickerMode::Open | PickerMode::Save => return false,
        }
        true
    }

    /// Open the highlighted entry if it is a directory; returns whether it was.
    fn open_highlighted_dir(&mut self) -> bool {
        match self.highlighted_entry().filter(|e| e.is_dir) {
            Some(entry) => {
                let path = entry.path.clone();
                let _ = self.navigate_to(&path);
                true
            }
            None => false,
        }
    }

    /// [`confirm`](Self::confirm), unless nothing valid is chosen.
    fn finish_if_chosen(&mut self) {
        match self.confirm() {
            PickerResult::None => {}
            result => self.result = Some(result),
        }
    }
}

impl Interactive for FilePicker {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        if self.handle_key_event(event) {
            EventResult::ConsumedAndRender
        } else {
            EventResult::Ignored
        }
    }

    fn focusable(&self) -> bool {
        true
    }
}
