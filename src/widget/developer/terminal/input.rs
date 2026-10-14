//! The command line: keys edit the input buffer, walk the history and
//! scroll; Enter, Escape and Tab report an action

use super::types::TerminalAction;
use super::Terminal;
use crate::event::{Key, KeyEvent};

impl Terminal {
    /// Get input buffer
    pub fn get_input(&self) -> &str {
        &self.input_buffer
    }

    /// Clear input buffer
    pub fn clear_input(&mut self) {
        self.input_buffer.clear();
    }

    /// Handle key event
    pub fn handle_key(&mut self, key: KeyEvent) -> Option<TerminalAction> {
        match key.key {
            Key::Char(c) => {
                self.input_buffer.push(c);
                None
            }
            Key::Backspace => {
                self.input_buffer.pop();
                None
            }
            Key::Enter => {
                let input = std::mem::take(&mut self.input_buffer);
                if !input.is_empty() {
                    self.history.push(input.clone());
                    self.history_pos = self.history.len();
                }
                Some(TerminalAction::Submit(input))
            }
            Key::Up => {
                if self.history_pos > 0 {
                    self.history_pos -= 1;
                    self.input_buffer = self
                        .history
                        .get(self.history_pos)
                        .cloned()
                        .unwrap_or_default();
                }
                None
            }
            Key::Down => {
                if self.history_pos < self.history.len() {
                    self.history_pos += 1;
                    self.input_buffer = self
                        .history
                        .get(self.history_pos)
                        .cloned()
                        .unwrap_or_default();
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
            Key::Home => {
                self.input_buffer.clear();
                None
            }
            Key::End => {
                self.scroll_to_bottom();
                None
            }
            Key::Escape => Some(TerminalAction::Cancel),
            Key::Tab => Some(TerminalAction::TabComplete(self.input_buffer.clone())),
            _ => None,
        }
    }
}
