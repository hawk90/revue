//! LogViewer key handling

use super::LogViewer;
use crate::event::Key;

impl LogViewer {
    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up | Key::Char('k') => {
                self.select_prev();
                true
            }
            Key::Down | Key::Char('j') => {
                self.select_next();
                true
            }
            Key::PageUp => {
                for _ in 0..10 {
                    self.select_prev();
                }
                true
            }
            Key::PageDown => {
                for _ in 0..10 {
                    self.select_next();
                }
                true
            }
            Key::Home | Key::Char('g') => {
                self.scroll_to_top();
                true
            }
            Key::End | Key::Char('G') => {
                self.scroll_to_bottom();
                true
            }
            Key::Char('f') => {
                self.toggle_tail();
                true
            }
            Key::Char('w') => {
                self.toggle_wrap();
                true
            }
            Key::Char('b') => {
                self.toggle_bookmark();
                true
            }
            Key::Char('n') => {
                self.next_match();
                true
            }
            Key::Char('N') => {
                self.prev_match();
                true
            }
            Key::Char(']') => {
                self.next_bookmark();
                true
            }
            Key::Char('[') => {
                self.prev_bookmark();
                true
            }
            Key::Enter => {
                self.toggle_selected_expanded();
                true
            }
            _ => false,
        }
    }
}
