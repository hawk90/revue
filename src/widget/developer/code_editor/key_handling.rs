//! Code editor key handling

use crate::event::Key;

impl super::CodeEditor {
    // =========================================================================
    // Key Handling
    // =========================================================================

    /// Handle key event
    pub fn handle_key(&mut self, key: &Key) -> bool {
        // Handle modal modes first
        if self.goto_line_mode {
            return self.handle_goto_input(key);
        }

        if self.find_mode {
            return self.handle_find_input(key);
        }

        match key {
            Key::Char(ch) => {
                self.insert_char(*ch);
                true
            }
            Key::Enter => {
                self.insert_char('\n');
                true
            }
            Key::Tab => {
                self.insert_char('\t');
                true
            }
            Key::Backspace => {
                self.delete_char_before();
                true
            }
            Key::Delete => {
                self.delete_char_at();
                true
            }
            Key::Left => {
                self.move_left();
                true
            }
            Key::Right => {
                self.move_right();
                true
            }
            Key::Up => {
                self.move_up();
                true
            }
            Key::Down => {
                self.move_down();
                true
            }
            Key::Home => {
                self.move_home();
                true
            }
            Key::End => {
                self.move_end();
                true
            }
            Key::PageUp => {
                self.page_up(20);
                true
            }
            Key::PageDown => {
                self.page_down(20);
                true
            }
            _ => false,
        }
    }
}
