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

// KEEP HERE - Private implementation tests (accesses private fields: focus, show_presets)
#[cfg(test)]
mod tests {
    use super::super::types::PresetRange;
    use super::*;
    use crate::widget::data::calendar::Date;

    // =========================================================================
    // Focus navigation tests
    // =========================================================================

    #[test]
    fn test_next_focus_start_to_end() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::Start;
        picker.next_focus();
        assert_eq!(picker.focus, RangeFocus::End);
    }

    #[test]
    fn test_next_focus_end_to_presets() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::End;
        picker.show_presets = true;
        picker.next_focus();
        assert_eq!(picker.focus, RangeFocus::Presets);
    }

    #[test]
    fn test_next_focus_end_to_start_no_presets() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::End;
        picker.show_presets = false;
        picker.next_focus();
        assert_eq!(picker.focus, RangeFocus::Start);
    }

    #[test]
    fn test_next_focus_presets_to_start() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::Presets;
        picker.next_focus();
        assert_eq!(picker.focus, RangeFocus::Start);
    }

    #[test]
    fn test_prev_focus_start_to_presets() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::Start;
        picker.show_presets = true;
        picker.prev_focus();
        assert_eq!(picker.focus, RangeFocus::Presets);
    }

    #[test]
    fn test_prev_focus_start_to_end_no_presets() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::Start;
        picker.show_presets = false;
        picker.prev_focus();
        assert_eq!(picker.focus, RangeFocus::End);
    }

    #[test]
    fn test_prev_focus_end_to_start() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::End;
        picker.prev_focus();
        assert_eq!(picker.focus, RangeFocus::Start);
    }

    #[test]
    fn test_prev_focus_presets_to_end() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::Presets;
        picker.prev_focus();
        assert_eq!(picker.focus, RangeFocus::End);
    }

    // =========================================================================
    // Day movement tests - Start focus
    // =========================================================================

    #[test]
    fn test_move_day_left_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        picker.move_day_left();
        assert_eq!(picker.start_cursor_day, 14);
        assert_eq!(picker.start.date.day, 15); // Selected date unchanged
    }

    #[test]
    fn test_move_day_left_wrap_to_previous_month_start() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        picker.start_cursor_day = 1;
        picker.move_day_left();
        assert_eq!(picker.start_cursor_day, 31);
        assert_eq!(picker.start.date.month, 12);
        assert_eq!(picker.start.date.year, 2023);
    }

    #[test]
    fn test_move_day_right_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        picker.move_day_right();
        assert_eq!(picker.start_cursor_day, 16);
    }

    #[test]
    fn test_move_day_right_wrap_to_next_month_start() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        picker.start_cursor_day = 31;
        picker.move_day_right();
        assert_eq!(picker.start_cursor_day, 1);
        assert_eq!(picker.start.date.month, 2);
    }

    #[test]
    fn test_move_week_up_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        picker.move_week_up();
        assert_eq!(picker.start_cursor_day, 8);
    }

    #[test]
    fn test_move_week_up_wrap_to_previous_month_start() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        picker.start_cursor_day = 5;
        picker.move_week_up();
        assert_eq!(picker.start_cursor_day, 29);
        assert_eq!(picker.start.date.month, 12);
        assert_eq!(picker.start.date.year, 2023);
    }

    #[test]
    fn test_move_week_down_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        picker.move_week_down();
        assert_eq!(picker.start_cursor_day, 22);
    }

    #[test]
    fn test_move_week_down_wrap_to_next_month_start() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        picker.start_cursor_day = 28;
        picker.move_week_down();
        assert_eq!(picker.start_cursor_day, 4);
        assert_eq!(picker.start.date.month, 2);
    }

    // =========================================================================
    // Day movement tests - End focus
    // =========================================================================

    #[test]
    fn test_move_day_left_end_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::End;
        picker.move_day_left();
        assert_eq!(picker.end_cursor_day, 19);
    }

    #[test]
    fn test_move_day_right_end_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::End;
        picker.move_day_right();
        assert_eq!(picker.end_cursor_day, 21);
    }

    // =========================================================================
    // Month navigation tests
    // =========================================================================

    #[test]
    fn test_prev_month_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 3, 15))
            .end_date(Date::new(2024, 3, 20));
        picker.focus = RangeFocus::Start;
        picker.prev_month();
        assert_eq!(picker.start.date.month, 2);
        assert_eq!(picker.start.date.year, 2024);
        assert_eq!(picker.start_cursor_day, 15);
    }

    #[test]
    fn test_prev_month_clamps_cursor_start() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 3, 31))
            .end_date(Date::new(2024, 3, 31));
        picker.focus = RangeFocus::Start;
        picker.prev_month();
        // February 2024 has 29 days
        assert_eq!(picker.start.date.month, 2);
        assert_eq!(picker.start_cursor_day, 29);
    }

    #[test]
    fn test_next_month_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        picker.next_month();
        assert_eq!(picker.start.date.month, 2);
        assert_eq!(picker.start.date.year, 2024);
    }

    #[test]
    fn test_next_month_clamps_cursor_start() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 31))
            .end_date(Date::new(2024, 1, 31));
        picker.focus = RangeFocus::Start;
        picker.next_month();
        // February 2024 has 29 days
        assert_eq!(picker.start.date.month, 2);
        assert_eq!(picker.start_cursor_day, 29);
    }

    // =========================================================================
    // Year navigation tests
    // =========================================================================

    #[test]
    fn test_prev_year_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 6, 15))
            .end_date(Date::new(2024, 6, 20));
        picker.focus = RangeFocus::Start;
        picker.prev_year();
        assert_eq!(picker.start.date.year, 2023);
        assert_eq!(picker.start.date.month, 6);
    }

    #[test]
    fn test_prev_year_handles_feb_29_start() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 2, 29))
            .end_date(Date::new(2024, 3, 1));
        picker.focus = RangeFocus::Start;
        picker.prev_year();
        // 2023 is not a leap year
        assert_eq!(picker.start.date.year, 2023);
        assert_eq!(picker.start_cursor_day, 28);
    }

    #[test]
    fn test_next_year_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 6, 15))
            .end_date(Date::new(2024, 6, 20));
        picker.focus = RangeFocus::Start;
        picker.next_year();
        assert_eq!(picker.start.date.year, 2025);
    }

    #[test]
    fn test_next_year_handles_feb_29_start() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 2, 29))
            .end_date(Date::new(2024, 3, 1));
        picker.focus = RangeFocus::Start;
        picker.next_year();
        // 2025 is not a leap year
        assert_eq!(picker.start.date.year, 2025);
        assert_eq!(picker.start_cursor_day, 28);
    }

    // =========================================================================
    // Date selection tests
    // =========================================================================

    #[test]
    fn test_select_date_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        picker.start_cursor_day = 10;
        picker.select_date();
        assert_eq!(picker.start.date.day, 10);
        assert_eq!(picker.active_preset, Some(PresetRange::Custom));
    }

    #[test]
    fn test_select_date_end_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::End;
        picker.end_cursor_day = 25;
        picker.select_date();
        assert_eq!(picker.end.date.day, 25);
        assert_eq!(picker.active_preset, Some(PresetRange::Custom));
    }

    #[test]
    fn test_select_date_presets_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Presets;
        picker.preset_cursor = 1; // Should be Yesterday
        picker.select_date();
        // Should apply the preset
        assert!(picker.active_preset.is_some());
    }

    #[test]
    fn test_select_date_swaps_if_needed() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 20))
            .end_date(Date::new(2024, 1, 10));
        picker.focus = RangeFocus::End;
        picker.end_cursor_day = 5;
        picker.select_date();
        // Should swap to ensure start <= end
        assert!(picker.start.date <= picker.end.date);
    }

    // =========================================================================
    // Preset navigation tests
    // =========================================================================

    #[test]
    fn test_preset_up() {
        let mut picker = RangePicker::new();
        picker.preset_cursor = 5;
        picker.preset_up();
        assert_eq!(picker.preset_cursor, 4);
    }

    #[test]
    fn test_preset_up_wraps() {
        let mut picker = RangePicker::new();
        picker.preset_cursor = 0;
        picker.preset_up();
        // Should wrap to last preset
        assert!(picker.preset_cursor > 0);
    }

    #[test]
    fn test_preset_down() {
        let mut picker = RangePicker::new();
        picker.preset_cursor = 2;
        picker.preset_down();
        assert_eq!(picker.preset_cursor, 3);
    }

    #[test]
    fn test_preset_down_wraps() {
        let mut picker = RangePicker::new();
        let len = picker.presets.len();
        picker.preset_cursor = len - 1;
        picker.preset_down();
        assert_eq!(picker.preset_cursor, 0);
    }

    #[test]
    fn test_preset_up_empty_presets() {
        let mut picker = RangePicker::new();
        picker.presets.clear();
        let original = picker.preset_cursor;
        picker.preset_up();
        // Should not change
        assert_eq!(picker.preset_cursor, original);
    }

    #[test]
    fn test_preset_down_empty_presets() {
        let mut picker = RangePicker::new();
        picker.presets.clear();
        let original = picker.preset_cursor;
        picker.preset_down();
        // Should not change
        assert_eq!(picker.preset_cursor, original);
    }

    // =========================================================================
    // Key handling tests - Focus navigation
    // =========================================================================

    #[test]
    fn test_handle_key_tab() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::Start;
        assert!(picker.handle_key(&Key::Tab));
        assert_eq!(picker.focus, RangeFocus::End);
    }

    #[test]
    fn test_handle_key_back_tab() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::End;
        assert!(picker.handle_key(&Key::BackTab));
        assert_eq!(picker.focus, RangeFocus::Start);
    }

    // =========================================================================
    // Key handling tests - Arrow keys
    // =========================================================================

    #[test]
    fn test_handle_key_left_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        assert!(picker.handle_key(&Key::Left));
        assert_eq!(picker.start_cursor_day, 14);
    }

    #[test]
    fn test_handle_key_right_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        assert!(picker.handle_key(&Key::Right));
        assert_eq!(picker.start_cursor_day, 16);
    }

    #[test]
    fn test_handle_key_up_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        assert!(picker.handle_key(&Key::Up));
        assert_eq!(picker.start_cursor_day, 8);
    }

    #[test]
    fn test_handle_key_down_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        assert!(picker.handle_key(&Key::Down));
        assert_eq!(picker.start_cursor_day, 22);
    }

    #[test]
    fn test_handle_key_vim_h_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        assert!(picker.handle_key(&Key::Char('h')));
        assert_eq!(picker.start_cursor_day, 14);
    }

    #[test]
    fn test_handle_key_vim_l_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        assert!(picker.handle_key(&Key::Char('l')));
        assert_eq!(picker.start_cursor_day, 16);
    }

    #[test]
    fn test_handle_key_vim_k_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        assert!(picker.handle_key(&Key::Char('k')));
        assert_eq!(picker.start_cursor_day, 8);
    }

    #[test]
    fn test_handle_key_vim_j_start_focus() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        assert!(picker.handle_key(&Key::Char('j')));
        assert_eq!(picker.start_cursor_day, 22);
    }

    // =========================================================================
    // Key handling tests - Preset navigation
    // =========================================================================

    #[test]
    fn test_handle_key_up_presets_focus() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::Presets;
        picker.preset_cursor = 3;
        assert!(picker.handle_key(&Key::Up));
        assert_eq!(picker.preset_cursor, 2);
    }

    #[test]
    fn test_handle_key_down_presets_focus() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::Presets;
        picker.preset_cursor = 3;
        assert!(picker.handle_key(&Key::Down));
        assert_eq!(picker.preset_cursor, 4);
    }

    #[test]
    fn test_handle_key_vim_k_presets_focus() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::Presets;
        picker.preset_cursor = 3;
        assert!(picker.handle_key(&Key::Char('k')));
        assert_eq!(picker.preset_cursor, 2);
    }

    #[test]
    fn test_handle_key_vim_j_presets_focus() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::Presets;
        picker.preset_cursor = 3;
        assert!(picker.handle_key(&Key::Char('j')));
        assert_eq!(picker.preset_cursor, 4);
    }

    // =========================================================================
    // Key handling tests - Month/Year navigation
    // =========================================================================

    #[test]
    fn test_handle_key_left_bracket() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 3, 15))
            .end_date(Date::new(2024, 3, 20));
        picker.focus = RangeFocus::Start;
        assert!(picker.handle_key(&Key::Char('[')));
        assert_eq!(picker.start.date.month, 2);
    }

    #[test]
    fn test_handle_key_right_bracket() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        assert!(picker.handle_key(&Key::Char(']')));
        assert_eq!(picker.start.date.month, 2);
    }

    #[test]
    fn test_handle_key_left_brace() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 6, 15))
            .end_date(Date::new(2024, 6, 20));
        picker.focus = RangeFocus::Start;
        assert!(picker.handle_key(&Key::Char('{')));
        assert_eq!(picker.start.date.year, 2023);
    }

    #[test]
    fn test_handle_key_right_brace() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 6, 15))
            .end_date(Date::new(2024, 6, 20));
        picker.focus = RangeFocus::Start;
        assert!(picker.handle_key(&Key::Char('}')));
        assert_eq!(picker.start.date.year, 2025);
    }

    // =========================================================================
    // Key handling tests - Selection
    // =========================================================================

    #[test]
    fn test_handle_key_enter() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        picker.start_cursor_day = 10;
        assert!(picker.handle_key(&Key::Enter));
        assert_eq!(picker.start.date.day, 10);
    }

    #[test]
    fn test_handle_key_space() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Start;
        picker.start_cursor_day = 10;
        assert!(picker.handle_key(&Key::Char(' ')));
        assert_eq!(picker.start.date.day, 10);
    }

    // =========================================================================
    // Key handling tests - Disabled state
    // =========================================================================

    #[test]
    fn test_handle_key_disabled_returns_false() {
        let mut picker = RangePicker::new();
        picker.state.disabled = true;
        assert!(!picker.handle_key(&Key::Tab));
        assert!(!picker.handle_key(&Key::Left));
        assert!(!picker.handle_key(&Key::Enter));
    }

    // =========================================================================
    // Key handling tests - Preset focus ignores navigation keys
    // =========================================================================

    #[test]
    fn test_handle_key_left_presets_focus_ignored() {
        let mut picker = RangePicker::new();
        picker.focus = RangeFocus::Presets;
        let original_cursor = picker.start_cursor_day;
        assert!(!picker.handle_key(&Key::Left));
        assert_eq!(picker.start_cursor_day, original_cursor);
    }

    #[test]
    fn test_handle_key_month_brackets_presets_focus_ignored() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 6, 15))
            .end_date(Date::new(2024, 6, 20));
        picker.focus = RangeFocus::Presets;
        let original_month = picker.start.date.month;
        assert!(!picker.handle_key(&Key::Char('[')));
        assert_eq!(picker.start.date.month, original_month);
    }

    // =========================================================================
    // Key handling tests - Unhandled keys
    // =========================================================================

    #[test]
    fn test_handle_key_unhandled_char() {
        let mut picker = RangePicker::new();
        assert!(!picker.handle_key(&Key::Char('x')));
    }

    #[test]
    fn test_handle_key_unhandled_special_key() {
        let mut picker = RangePicker::new();
        // PageUp/PageDown are not handled
        assert!(!picker.handle_key(&Key::PageUp));
        assert!(!picker.handle_key(&Key::PageDown));
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_current_date_mut_presets_focus_uses_end() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::Presets;
        picker.move_day_left(); // Should operate on end date
                                // End date should change
        assert!(picker.end_cursor_day != 20 || picker.end.date.month != 1);
    }

    #[test]
    fn test_current_date_mut_end_focus_uses_end() {
        let mut picker = RangePicker::new()
            .start_date(Date::new(2024, 1, 15))
            .end_date(Date::new(2024, 1, 20));
        picker.focus = RangeFocus::End;
        let original_start_day = picker.start_cursor_day;
        picker.move_day_left();
        // Start date should not change
        assert_eq!(picker.start_cursor_day, original_start_day);
    }
}
