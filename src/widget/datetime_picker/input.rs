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
