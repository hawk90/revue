//! Interactive implementation for Select

use crate::event::{Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use crate::layout::Rect;
use crate::widget::traits::{EventResult, Interactive};

use super::Select;

impl Interactive for Select {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        if self.disabled || !self.focused {
            return EventResult::Ignored;
        }

        let needs_render = match event.key {
            Key::Enter => {
                if self.open {
                    self.close();
                    self.clear_query();
                } else {
                    self.open();
                }
                true
            }
            Key::Char(' ') if !self.searchable => {
                self.toggle();
                true
            }
            Key::Up | Key::Char('k') if self.open && !self.searchable => {
                self.select_prev();
                true
            }
            Key::Down | Key::Char('j') if self.open && !self.searchable => {
                self.select_next();
                true
            }
            Key::Up if self.open && self.searchable => {
                if self.query.is_empty() {
                    self.select_prev();
                } else {
                    self.select_prev_filtered();
                }
                true
            }
            Key::Down if self.open && self.searchable => {
                if self.query.is_empty() {
                    self.select_next();
                } else {
                    self.select_next_filtered();
                }
                true
            }
            Key::Escape if self.open => {
                self.close();
                self.clear_query();
                true
            }
            Key::Backspace if self.open && self.searchable => {
                self.query.pop();
                self.update_filter();
                true
            }
            Key::Char(c) if self.open && self.searchable => {
                self.query.push(c);
                self.update_filter();
                true
            }
            Key::Home if self.open => {
                self.select_first();
                true
            }
            Key::End if self.open => {
                self.select_last();
                true
            }
            Key::Tab if self.open => {
                self.close();
                self.clear_query();
                true
            }
            _ => false,
        };

        if needs_render {
            EventResult::ConsumedAndRender
        } else {
            EventResult::Ignored
        }
    }

    fn handle_mouse(&mut self, event: &MouseEvent, area: Rect) -> EventResult {
        if self.disabled {
            return EventResult::Ignored;
        }

        let inside = area.contains(event.x, event.y);

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) if inside => {
                self.toggle();
                EventResult::ConsumedAndRender
            }
            _ => EventResult::Ignored,
        }
    }

    crate::impl_focus_handlers!(direct, no_blur);

    fn on_blur(&mut self) {
        self.focused = false;
        if self.open {
            self.close();
            self.clear_query();
        }
    }
}
