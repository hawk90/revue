//! DateTime picker input handling

use super::types::{DateTimeFormat, DateTimeMode};
use crate::event::Key;

/// Input handling for DateTimePicker
pub trait Input {
    /// Get mode
    fn mode(&self) -> DateTimeMode;
    /// Get format
    fn format(&self) -> DateTimeFormat;

    /// Handle keyboard input
    fn handle_key(&mut self, key: &Key) -> bool {
        if self.is_disabled() {
            return false;
        }

        // Dispatch to navigation trait methods
        match key {
            // Switch between date and time modes
            Key::Tab => {
                if self.format() != DateTimeFormat::DateOnly
                    && self.format() != DateTimeFormat::TimeOnly
                {
                    self.toggle_mode();
                    return true;
                }
                false
            }

            // Date mode navigation
            Key::Left | Key::Char('h') if self.mode() == DateTimeMode::Date => {
                self.nav_move_day_left();
                true
            }
            Key::Right | Key::Char('l') if self.mode() == DateTimeMode::Date => {
                self.nav_move_day_right();
                true
            }
            Key::Up | Key::Char('k') if self.mode() == DateTimeMode::Date => {
                self.nav_move_week_up();
                true
            }
            Key::Down | Key::Char('j') if self.mode() == DateTimeMode::Date => {
                self.nav_move_week_down();
                true
            }
            Key::Char('[') if self.mode() == DateTimeMode::Date => {
                self.nav_prev_month();
                true
            }
            Key::Char(']') if self.mode() == DateTimeMode::Date => {
                self.nav_next_month();
                true
            }
            Key::Char('{') if self.mode() == DateTimeMode::Date => {
                self.nav_prev_year();
                true
            }
            Key::Char('}') if self.mode() == DateTimeMode::Date => {
                self.nav_next_year();
                true
            }
            Key::Enter | Key::Char(' ') if self.mode() == DateTimeMode::Date => {
                self.nav_select_date();
                true
            }

            // Time mode navigation
            Key::Left | Key::Char('h') if self.mode() == DateTimeMode::Time => {
                self.nav_prev_time_field();
                true
            }
            Key::Right | Key::Char('l') if self.mode() == DateTimeMode::Time => {
                self.nav_next_time_field();
                true
            }
            Key::Up | Key::Char('k') if self.mode() == DateTimeMode::Time => {
                self.nav_increment_time();
                true
            }
            Key::Down | Key::Char('j') if self.mode() == DateTimeMode::Time => {
                self.nav_decrement_time();
                true
            }

            _ => false,
        }
    }

    // Abstract methods to be implemented by DateTimePicker
    fn is_disabled(&self) -> bool;
    fn toggle_mode(&mut self);
    fn nav_move_day_left(&mut self);
    fn nav_move_day_right(&mut self);
    fn nav_move_week_up(&mut self);
    fn nav_move_week_down(&mut self);
    fn nav_prev_month(&mut self);
    fn nav_next_month(&mut self);
    fn nav_prev_year(&mut self);
    fn nav_next_year(&mut self);
    fn nav_select_date(&mut self);
    fn nav_prev_time_field(&mut self);
    fn nav_next_time_field(&mut self);
    fn nav_increment_time(&mut self);
    fn nav_decrement_time(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock struct for testing Input trait
    struct MockInputPicker {
        mode: DateTimeMode,
        format: DateTimeFormat,
        disabled: bool,
        // Track which methods were called
        move_day_left_called: bool,
        move_day_right_called: bool,
        move_week_up_called: bool,
        move_week_down_called: bool,
        prev_month_called: bool,
        next_month_called: bool,
        prev_year_called: bool,
        next_year_called: bool,
        select_date_called: bool,
        prev_time_field_called: bool,
        next_time_field_called: bool,
        increment_time_called: bool,
        decrement_time_called: bool,
        toggle_mode_called: bool,
    }

    impl MockInputPicker {
        fn new() -> Self {
            Self {
                mode: DateTimeMode::Date,
                format: DateTimeFormat::DateTime,
                disabled: false,
                move_day_left_called: false,
                move_day_right_called: false,
                move_week_up_called: false,
                move_week_down_called: false,
                prev_month_called: false,
                next_month_called: false,
                prev_year_called: false,
                next_year_called: false,
                select_date_called: false,
                prev_time_field_called: false,
                next_time_field_called: false,
                increment_time_called: false,
                decrement_time_called: false,
                toggle_mode_called: false,
            }
        }

        fn with_mode(mode: DateTimeMode) -> Self {
            let mut picker = Self::new();
            picker.mode = mode;
            picker
        }

        fn with_format(format: DateTimeFormat) -> Self {
            let mut picker = Self::new();
            picker.format = format;
            picker
        }

        fn disabled() -> Self {
            let mut picker = Self::new();
            picker.disabled = true;
            picker
        }

        fn reset_flags(&mut self) {
            self.move_day_left_called = false;
            self.move_day_right_called = false;
            self.move_week_up_called = false;
            self.move_week_down_called = false;
            self.prev_month_called = false;
            self.next_month_called = false;
            self.prev_year_called = false;
            self.next_year_called = false;
            self.select_date_called = false;
            self.prev_time_field_called = false;
            self.next_time_field_called = false;
            self.increment_time_called = false;
            self.decrement_time_called = false;
            self.toggle_mode_called = false;
        }
    }

    impl Input for MockInputPicker {
        fn mode(&self) -> DateTimeMode {
            self.mode
        }

        fn format(&self) -> DateTimeFormat {
            self.format
        }

        fn is_disabled(&self) -> bool {
            self.disabled
        }

        fn toggle_mode(&mut self) {
            self.toggle_mode_called = true;
            self.mode = match self.mode {
                DateTimeMode::Date => DateTimeMode::Time,
                DateTimeMode::Time => DateTimeMode::Date,
            };
        }

        fn nav_move_day_left(&mut self) {
            self.move_day_left_called = true;
        }

        fn nav_move_day_right(&mut self) {
            self.move_day_right_called = true;
        }

        fn nav_move_week_up(&mut self) {
            self.move_week_up_called = true;
        }

        fn nav_move_week_down(&mut self) {
            self.move_week_down_called = true;
        }

        fn nav_prev_month(&mut self) {
            self.prev_month_called = true;
        }

        fn nav_next_month(&mut self) {
            self.next_month_called = true;
        }

        fn nav_prev_year(&mut self) {
            self.prev_year_called = true;
        }

        fn nav_next_year(&mut self) {
            self.next_year_called = true;
        }

        fn nav_select_date(&mut self) {
            self.select_date_called = true;
        }

        fn nav_prev_time_field(&mut self) {
            self.prev_time_field_called = true;
        }

        fn nav_next_time_field(&mut self) {
            self.next_time_field_called = true;
        }

        fn nav_increment_time(&mut self) {
            self.increment_time_called = true;
        }

        fn nav_decrement_time(&mut self) {
            self.decrement_time_called = true;
        }
    }

    // =========================================================================
    // Disabled state tests
    // =========================================================================

    #[test]
    fn test_handle_key_disabled_returns_false() {
        let mut picker = MockInputPicker::disabled();
        assert!(!picker.handle_key(&Key::Left));
        assert!(!picker.handle_key(&Key::Tab));
    }

    // =========================================================================
    // Mode toggle tests
    // =========================================================================

    #[test]
    fn test_handle_key_tab_toggles_mode() {
        let mut picker = MockInputPicker::new();
        assert!(picker.handle_key(&Key::Tab));
        assert!(picker.toggle_mode_called);
        assert_eq!(picker.mode(), DateTimeMode::Time);
    }

    #[test]
    fn test_handle_key_tab_date_only_format() {
        let mut picker = MockInputPicker::with_format(DateTimeFormat::DateOnly);
        assert!(!picker.handle_key(&Key::Tab));
        assert!(!picker.toggle_mode_called);
    }

    #[test]
    fn test_handle_key_tab_time_only_format() {
        let mut picker = MockInputPicker::with_format(DateTimeFormat::TimeOnly);
        assert!(!picker.handle_key(&Key::Tab));
        assert!(!picker.toggle_mode_called);
    }

    // =========================================================================
    // Date mode navigation tests
    // =========================================================================

    #[test]
    fn test_handle_key_left_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Left));
        assert!(picker.move_day_left_called);
    }

    #[test]
    fn test_handle_key_char_h_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Char('h')));
        assert!(picker.move_day_left_called);
    }

    #[test]
    fn test_handle_key_right_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Right));
        assert!(picker.move_day_right_called);
    }

    #[test]
    fn test_handle_key_char_l_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Char('l')));
        assert!(picker.move_day_right_called);
    }

    #[test]
    fn test_handle_key_up_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Up));
        assert!(picker.move_week_up_called);
    }

    #[test]
    fn test_handle_key_char_k_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Char('k')));
        assert!(picker.move_week_up_called);
    }

    #[test]
    fn test_handle_key_down_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Down));
        assert!(picker.move_week_down_called);
    }

    #[test]
    fn test_handle_key_char_j_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Char('j')));
        assert!(picker.move_week_down_called);
    }

    #[test]
    fn test_handle_key_left_bracket_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Char('[')));
        assert!(picker.prev_month_called);
    }

    #[test]
    fn test_handle_key_right_bracket_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Char(']')));
        assert!(picker.next_month_called);
    }

    #[test]
    fn test_handle_key_left_brace_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Char('{')));
        assert!(picker.prev_year_called);
    }

    #[test]
    fn test_handle_key_right_brace_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Char('}')));
        assert!(picker.next_year_called);
    }

    #[test]
    fn test_handle_key_enter_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Enter));
        assert!(picker.select_date_called);
    }

    #[test]
    fn test_handle_key_space_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(picker.handle_key(&Key::Char(' ')));
        assert!(picker.select_date_called);
    }

    // =========================================================================
    // Time mode navigation tests
    // =========================================================================

    #[test]
    fn test_handle_key_left_in_time_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Time);
        assert!(picker.handle_key(&Key::Left));
        assert!(picker.prev_time_field_called);
    }

    #[test]
    fn test_handle_key_char_h_in_time_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Time);
        assert!(picker.handle_key(&Key::Char('h')));
        assert!(picker.prev_time_field_called);
    }

    #[test]
    fn test_handle_key_right_in_time_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Time);
        assert!(picker.handle_key(&Key::Right));
        assert!(picker.next_time_field_called);
    }

    #[test]
    fn test_handle_key_char_l_in_time_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Time);
        assert!(picker.handle_key(&Key::Char('l')));
        assert!(picker.next_time_field_called);
    }

    #[test]
    fn test_handle_key_up_in_time_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Time);
        assert!(picker.handle_key(&Key::Up));
        assert!(picker.increment_time_called);
    }

    #[test]
    fn test_handle_key_char_k_in_time_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Time);
        assert!(picker.handle_key(&Key::Char('k')));
        assert!(picker.increment_time_called);
    }

    #[test]
    fn test_handle_key_down_in_time_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Time);
        assert!(picker.handle_key(&Key::Down));
        assert!(picker.decrement_time_called);
    }

    #[test]
    fn test_handle_key_char_j_in_time_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Time);
        assert!(picker.handle_key(&Key::Char('j')));
        assert!(picker.decrement_time_called);
    }

    // =========================================================================
    // Unhandled key tests
    // =========================================================================

    #[test]
    fn test_handle_key_unhandled_in_date_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        assert!(!picker.handle_key(&Key::Char('x')));
        assert!(!picker.move_day_left_called);
    }

    #[test]
    fn test_handle_key_unhandled_in_time_mode() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Time);
        assert!(!picker.handle_key(&Key::Char('x')));
        assert!(!picker.increment_time_called);
    }

    #[test]
    fn test_handle_key_left_in_wrong_mode_does_nothing() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Time);
        // Left arrow in time mode should call prev_time_field, not move_day_left
        assert!(picker.handle_key(&Key::Left));
        assert!(picker.prev_time_field_called);
        assert!(!picker.move_day_left_called);
    }

    #[test]
    fn test_handle_key_up_in_wrong_mode_does_nothing() {
        let mut picker = MockInputPicker::with_mode(DateTimeMode::Date);
        picker.reset_flags();
        // Up arrow in date mode should call move_week_up, not increment_time
        assert!(picker.handle_key(&Key::Up));
        assert!(picker.move_week_up_called);
        assert!(!picker.increment_time_called);
    }
}
