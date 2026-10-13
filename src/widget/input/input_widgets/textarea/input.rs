//! Keys: plain keys edit and move; modifier combinations open find and
//! replace, add cursors, undo, and select

use crate::event::{Key, KeyEvent};
use crate::widget::traits::{EventResult, Interactive};

use super::TextArea;

impl TextArea {
    /// Handle key event
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if !self.focused {
            return false;
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
                self.clear_selection();
                self.move_left();
                true
            }
            Key::Right => {
                self.clear_selection();
                self.move_right();
                true
            }
            Key::Up => {
                self.clear_selection();
                self.move_up();
                true
            }
            Key::Down => {
                self.clear_selection();
                self.move_down();
                true
            }
            Key::Home => {
                self.clear_selection();
                self.move_home();
                true
            }
            Key::End => {
                self.clear_selection();
                self.move_end();
                true
            }
            Key::PageUp => {
                let page = self.last_viewport_height.get().max(1);
                self.page_up(page);
                true
            }
            Key::PageDown => {
                let page = self.last_viewport_height.get().max(1);
                self.page_down(page);
                true
            }
            _ => false,
        }
    }

    /// Handle a key event, including modifier combinations.
    ///
    /// This is the modifier-aware entry point. It wires the find/replace and
    /// multi-cursor shortcuts (whose logic already exists), then delegates
    /// unmodified keys to [`handle_key`](Self::handle_key).
    ///
    /// Bindings:
    /// - `Ctrl+F` — open the find panel
    /// - `Ctrl+H` — open the find/replace panel
    /// - `Ctrl+D` — select the next occurrence of the word/selection
    /// - `F3` / `Shift+F3` — jump to the next / previous match
    /// - `Ctrl+Alt+Up` / `Ctrl+Alt+Down` — add a cursor above / below
    /// - `Esc` — close the find panel, or collapse to a single cursor
    ///
    /// Returns `true` if the event was handled (a redraw may be needed).
    pub fn handle_key_event(&mut self, event: &KeyEvent) -> bool {
        if !self.focused {
            return false;
        }

        // Ctrl+Alt cursor stacking (multi-cursor).
        if event.ctrl && event.alt {
            match event.key {
                Key::Up => {
                    self.add_cursor_above();
                    return true;
                }
                Key::Down => {
                    self.add_cursor_below();
                    return true;
                }
                _ => {}
            }
        }

        // Ctrl combinations: find/replace, multi-cursor selection, undo and
        // line commands.
        if event.ctrl && !event.alt {
            match event.key {
                Key::Char('f') => {
                    self.open_find();
                    return true;
                }
                Key::Char('h') => {
                    self.open_replace();
                    return true;
                }
                Key::Char('d') => {
                    self.select_next_occurrence();
                    return true;
                }
                // Shift may come as the flag, as the capital, or both
                Key::Char('z' | 'Z') if event.shift => {
                    self.redo();
                    return true;
                }
                Key::Char('z') => {
                    self.undo();
                    return true;
                }
                Key::Char('y') => {
                    self.redo();
                    return true;
                }
                Key::Char('a') => {
                    self.select_all();
                    return true;
                }
                Key::Char('k' | 'K') if event.shift => {
                    self.delete_line();
                    return true;
                }
                _ => {}
            }
        }

        // Find navigation (works whether or not the panel is focused).
        if event.key == Key::F(3) {
            if event.shift {
                self.find_previous();
            } else {
                self.find_next();
            }
            return true;
        }

        // Escape: close the find panel first, then collapse to a single cursor.
        if event.key == Key::Escape {
            if self.is_find_open() {
                self.close_find();
                return true;
            }
            if !self.cursors.is_single() {
                self.clear_secondary_cursors();
                return true;
            }
            return false;
        }

        // Ctrl or Alt with a letter is a command, and this one is not
        // bound: leave it to the app rather than type the letter. Ctrl+Alt
        // with a letter is text, as Windows reports AltGr.
        if matches!(event.key, Key::Char(_)) && event.ctrl != event.alt {
            return false;
        }

        // Fall back to unmodified key handling.
        self.handle_key(&event.key)
    }
}

impl Interactive for TextArea {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        EventResult::from(self.handle_key_event(event))
    }

    fn on_focus(&mut self) {
        self.focused = true;
    }

    fn on_blur(&mut self) {
        self.focused = false;
    }
}
