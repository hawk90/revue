//! Range picker navigation and key handling

use super::core::RangePicker;
use super::types::RangeFocus;
use crate::event::Key;
use crate::widget::data::calendar::{days_in_month, Date};

impl RangePicker {
    // =========================================================================
    // Navigation
    // =========================================================================

    /// Move to next focus area
    pub(crate) fn next_focus(&mut self) {
        self.focus = match self.focus {
            RangeFocus::Start => RangeFocus::End,
            RangeFocus::End => {
                if self.show_presets {
                    RangeFocus::Presets
                } else {
                    RangeFocus::Start
                }
            }
            RangeFocus::Presets => RangeFocus::Start,
        };
    }

    /// Move to previous focus area
    pub(crate) fn prev_focus(&mut self) {
        self.focus = match self.focus {
            RangeFocus::Start => {
                if self.show_presets {
                    RangeFocus::Presets
                } else {
                    RangeFocus::End
                }
            }
            RangeFocus::End => RangeFocus::Start,
            RangeFocus::Presets => RangeFocus::End,
        };
    }

    /// Get mutable reference to current calendar date/cursor
    fn current_date_mut(&mut self) -> (&mut Date, &mut u32) {
        match self.focus {
            RangeFocus::Start => (&mut self.start.date, &mut self.start_cursor_day),
            RangeFocus::End | RangeFocus::Presets => (&mut self.end.date, &mut self.end_cursor_day),
        }
    }

    /// Move cursor left (previous day)
    fn move_day_left(&mut self) {
        let (date, cursor) = self.current_date_mut();
        if *cursor > 1 {
            *cursor -= 1;
        } else {
            // Previous month
            if date.month > 1 {
                date.month -= 1;
            } else {
                date.month = 12;
                date.year -= 1;
            }
            *cursor = days_in_month(date.year, date.month);
        }
    }

    /// Move cursor right (next day)
    fn move_day_right(&mut self) {
        let (date, cursor) = self.current_date_mut();
        let max_day = days_in_month(date.year, date.month);
        if *cursor < max_day {
            *cursor += 1;
        } else {
            // Next month
            if date.month < 12 {
                date.month += 1;
            } else {
                date.month = 1;
                date.year += 1;
            }
            *cursor = 1;
        }
    }

    /// Move cursor up (previous week)
    fn move_week_up(&mut self) {
        let (date, cursor) = self.current_date_mut();
        if *cursor > 7 {
            *cursor -= 7;
        } else {
            // Previous month
            if date.month > 1 {
                date.month -= 1;
            } else {
                date.month = 12;
                date.year -= 1;
            }
            let max_day = days_in_month(date.year, date.month);
            *cursor = max_day - (7 - *cursor);
        }
    }

    /// Move cursor down (next week)
    fn move_week_down(&mut self) {
        let (date, cursor) = self.current_date_mut();
        let max_day = days_in_month(date.year, date.month);
        if *cursor + 7 <= max_day {
            *cursor += 7;
        } else {
            let overflow = *cursor + 7 - max_day;
            if date.month < 12 {
                date.month += 1;
            } else {
                date.month = 1;
                date.year += 1;
            }
            *cursor = overflow;
        }
    }

    /// Go to previous month
    fn prev_month(&mut self) {
        let (date, cursor) = self.current_date_mut();
        if date.month > 1 {
            date.month -= 1;
        } else {
            date.month = 12;
            date.year -= 1;
        }
        let max_day = days_in_month(date.year, date.month);
        *cursor = (*cursor).min(max_day);
    }

    /// Go to next month
    fn next_month(&mut self) {
        let (date, cursor) = self.current_date_mut();
        if date.month < 12 {
            date.month += 1;
        } else {
            date.month = 1;
            date.year += 1;
        }
        let max_day = days_in_month(date.year, date.month);
        *cursor = (*cursor).min(max_day);
    }

    /// Go to previous year
    fn prev_year(&mut self) {
        let (date, cursor) = self.current_date_mut();
        date.year -= 1;
        let max_day = days_in_month(date.year, date.month);
        *cursor = (*cursor).min(max_day);
    }

    /// Go to next year
    fn next_year(&mut self) {
        let (date, cursor) = self.current_date_mut();
        date.year += 1;
        let max_day = days_in_month(date.year, date.month);
        *cursor = (*cursor).min(max_day);
    }

    /// Select current cursor position
    fn select_date(&mut self) {
        match self.focus {
            RangeFocus::Start => {
                self.start.date.day = self.start_cursor_day;
                self.active_preset = Some(super::types::PresetRange::Custom);
            }
            RangeFocus::End => {
                self.end.date.day = self.end_cursor_day;
                self.active_preset = Some(super::types::PresetRange::Custom);
            }
            RangeFocus::Presets => {
                if let Some(preset) = self.presets.get(self.preset_cursor) {
                    self.apply_preset(*preset);
                }
            }
        }
        self.swap_if_needed();
    }

    /// Move preset cursor up
    fn preset_up(&mut self) {
        if !self.presets.is_empty() {
            self.preset_cursor = self
                .preset_cursor
                .checked_sub(1)
                .unwrap_or(self.presets.len() - 1);
        }
    }

    /// Move preset cursor down
    fn preset_down(&mut self) {
        if !self.presets.is_empty() {
            self.preset_cursor = (self.preset_cursor + 1) % self.presets.len();
        }
    }

    // =========================================================================
    // Key handling
    // =========================================================================

    /// Handle key input
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.state.disabled {
            return false;
        }

        match key {
            // Switch focus areas
            Key::Tab => {
                self.next_focus();
                true
            }
            Key::BackTab => {
                self.prev_focus();
                true
            }

            // Navigation
            Key::Left | Key::Char('h') if self.focus != RangeFocus::Presets => {
                self.move_day_left();
                true
            }
            Key::Right | Key::Char('l') if self.focus != RangeFocus::Presets => {
                self.move_day_right();
                true
            }
            Key::Up | Key::Char('k') if self.focus != RangeFocus::Presets => {
                self.move_week_up();
                true
            }
            Key::Down | Key::Char('j') if self.focus != RangeFocus::Presets => {
                self.move_week_down();
                true
            }

            // Preset navigation
            Key::Up | Key::Char('k') if self.focus == RangeFocus::Presets => {
                self.preset_up();
                true
            }
            Key::Down | Key::Char('j') if self.focus == RangeFocus::Presets => {
                self.preset_down();
                true
            }

            // Month/Year navigation
            Key::Char('[') if self.focus != RangeFocus::Presets => {
                self.prev_month();
                true
            }
            Key::Char(']') if self.focus != RangeFocus::Presets => {
                self.next_month();
                true
            }
            Key::Char('{') if self.focus != RangeFocus::Presets => {
                self.prev_year();
                true
            }
            Key::Char('}') if self.focus != RangeFocus::Presets => {
                self.next_year();
                true
            }

            // Selection
            Key::Enter | Key::Char(' ') => {
                self.select_date();
                true
            }

            _ => false,
        }
    }
}
