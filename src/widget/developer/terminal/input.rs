//! The command line: keys edit the input buffer (as every single-line field
//! is edited, #985), walk the history and scroll; Enter, Escape and Tab
//! report an action

use super::types::TerminalAction;
use super::Terminal;
use crate::event::{Key, KeyEvent};

impl Terminal {
    /// Get input buffer
    pub fn get_input(&self) -> &str {
        self.input_buffer.line(0).unwrap_or("")
    }

    /// Clear input buffer
    pub fn clear_input(&mut self) {
        self.input_buffer.set_text("");
    }

    /// Put `text` on the command line, the cursor at its end
    fn set_input(&mut self, text: &str) {
        self.input_buffer.set_text(text);
        let end = self.input_buffer.end();
        self.input_buffer.set_cursor(end);
    }

    /// Handle key event. Enter submits the line, Up and Down walk the
    /// history, PageUp and PageDown scroll, Ctrl+End scrolls to the
    /// bottom, Escape cancels and Tab asks for completion; the other keys
    /// edit the line as every single-line field is edited (Home and End
    /// go to its start and end, as in readline). An unbound Ctrl or Alt
    /// letter is not typed.
    pub fn handle_key(&mut self, key: KeyEvent) -> Option<TerminalAction> {
        match key.key {
            Key::Enter => {
                let input = self.get_input().to_string();
                self.clear_input();
                if !input.is_empty() {
                    self.history.push(input.clone());
                    self.history_pos = self.history.len();
                }
                Some(TerminalAction::Submit(input))
            }
            Key::Up => {
                if self.history_pos > 0 {
                    self.history_pos -= 1;
                    let entry = self
                        .history
                        .get(self.history_pos)
                        .cloned()
                        .unwrap_or_default();
                    self.set_input(&entry);
                }
                None
            }
            Key::Down => {
                if self.history_pos < self.history.len() {
                    self.history_pos += 1;
                    let entry = self
                        .history
                        .get(self.history_pos)
                        .cloned()
                        .unwrap_or_default();
                    self.set_input(&entry);
                }
                None
            }
            Key::PageUp => {
                self.scroll_up(self.height as usize / 2);
                None
            }
            Key::PageDown => {
                self.scroll_down(self.height as usize / 2);
                None
            }
            Key::End if key.ctrl => {
                self.scroll_to_bottom();
                None
            }
            Key::Escape => Some(TerminalAction::Cancel),
            Key::Tab => Some(TerminalAction::TabComplete(self.get_input().to_string())),
            _ => {
                self.input_buffer.edit_key(&key, 1);
                None
            }
        }
    }
}
