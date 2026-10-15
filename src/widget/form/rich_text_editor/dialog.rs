//! The link and image dialogs: opening, typing into their two fields, and
//! inserting the result as markdown.

use super::{DialogType, RichTextEditor};
use crate::event::Key;

impl RichTextEditor {
    /// Open link dialog
    pub fn open_link_dialog(&mut self) {
        let text = self.get_selection().unwrap_or_default();
        self.dialog = DialogType::InsertLink {
            text,
            url: String::new(),
            field: 0,
        };
    }

    /// Open image dialog
    pub fn open_image_dialog(&mut self) {
        self.dialog = DialogType::InsertImage {
            alt: String::new(),
            src: String::new(),
            field: 0,
        };
    }

    /// Check if dialog is open
    pub fn is_dialog_open(&self) -> bool {
        !matches!(self.dialog, DialogType::None)
    }

    /// Close dialog
    pub fn close_dialog(&mut self) {
        self.dialog = DialogType::None;
    }

    /// Insert link at cursor
    pub fn insert_link(&mut self, text: &str, url: &str) {
        let md = format!("[{}]({})", text, url);
        self.insert_str(&md);
    }

    /// Insert image at cursor
    pub fn insert_image(&mut self, alt: &str, src: &str) {
        let md = format!("![{}]({})", alt, src);
        self.insert_str(&md);
    }

    /// Handle dialog key input
    ///
    /// Both dialogs have two fields (text and URL, or alt and source): Tab
    /// moves between them, Enter inserts, Escape closes.
    pub(super) fn handle_dialog_key(&mut self, key: &Key) -> bool {
        let is_link = matches!(self.dialog, DialogType::InsertLink { .. });
        let (first, second, field) = match &mut self.dialog {
            DialogType::InsertLink { text, url, field } => (text, url, field),
            DialogType::InsertImage { alt, src, field } => (alt, src, field),
            DialogType::None => return false,
        };
        match key {
            Key::Tab => {
                *field = (*field + 1) % 2;
            }
            Key::Enter => {
                let a = first.clone();
                let b = second.clone();
                self.dialog = DialogType::None;
                if is_link {
                    self.insert_link(&a, &b);
                } else {
                    self.insert_image(&a, &b);
                }
            }
            Key::Escape => {
                self.dialog = DialogType::None;
            }
            Key::Char(ch) => {
                if *field == 0 {
                    first.push(*ch);
                } else {
                    second.push(*ch);
                }
            }
            Key::Backspace => {
                if *field == 0 {
                    first.pop();
                } else {
                    second.pop();
                }
            }
            _ => return false,
        }
        true
    }
}
