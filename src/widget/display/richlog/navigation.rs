//! Scrolling, entry selection and key handling

use super::RichLog;
use crate::event::Key;

impl RichLog {
    /// Scroll up
    pub fn scroll_up(&mut self, lines: usize) {
        self.scroll = self.scroll.saturating_sub(lines);
        self.auto_scroll = false;
    }

    /// Scroll down
    pub fn scroll_down(&mut self, lines: usize) {
        let max_scroll = self.entries.len().saturating_sub(1);
        self.scroll = (self.scroll + lines).min(max_scroll);
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll = 0;
        self.auto_scroll = false;
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        if !self.entries.is_empty() {
            self.scroll = self.entries.len().saturating_sub(1);
        }
        self.auto_scroll = true;
    }

    /// Select next entry
    pub fn select_next(&mut self) {
        let count = self.visible_entries().len();
        match self.selected {
            Some(i) if i < count - 1 => self.selected = Some(i + 1),
            None if count > 0 => self.selected = Some(0),
            _ => {}
        }
    }

    /// Select previous entry
    pub fn select_prev(&mut self) {
        if let Some(i) = self.selected {
            if i > 0 {
                self.selected = Some(i - 1);
            }
        }
    }

    /// Toggle selected entry details
    pub fn toggle_selected(&mut self) {
        if let Some(i) = self.selected {
            if let Some(entry) = self.entries.get_mut(i) {
                entry.toggle();
            }
        }
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up | Key::Char('k') => {
                self.scroll_up(1);
                true
            }
            Key::Down | Key::Char('j') => {
                self.scroll_down(1);
                true
            }
            Key::PageUp => {
                self.scroll_up(10);
                true
            }
            Key::PageDown => {
                self.scroll_down(10);
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
            Key::Char('c') => {
                self.clear();
                true
            }
            _ => false,
        }
    }
}
