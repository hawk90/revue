//! Moving the shown month and the selected date, by method or by key

use super::{days_in_month, Calendar, Date};

impl Calendar {
    /// Navigate to previous month
    pub fn prev_month(&mut self) {
        if self.month == 1 {
            self.month = 12;
            self.year -= 1;
        } else {
            self.month -= 1;
        }
    }

    /// Navigate to next month
    pub fn next_month(&mut self) {
        if self.month == 12 {
            self.month = 1;
            self.year += 1;
        } else {
            self.month += 1;
        }
    }

    /// Navigate to previous year
    pub fn prev_year(&mut self) {
        self.year -= 1;
    }

    /// Navigate to next year
    pub fn next_year(&mut self) {
        self.year += 1;
    }

    /// Select a date
    pub fn select(&mut self, date: Date) {
        self.selected = Some(date);
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected = None;
        self.range_end = None;
    }

    /// Get selected date
    pub fn get_selected(&self) -> Option<Date> {
        self.selected
    }

    /// Select next day
    pub fn select_next_day(&mut self) {
        if let Some(date) = self.selected {
            let days = days_in_month(date.year, date.month);
            if date.day < days {
                self.selected = Some(Date::new(date.year, date.month, date.day + 1));
            } else if date.month < 12 {
                self.selected = Some(Date::new(date.year, date.month + 1, 1));
                self.month = date.month + 1;
            } else {
                self.selected = Some(Date::new(date.year + 1, 1, 1));
                self.year = date.year + 1;
                self.month = 1;
            }
        } else {
            self.selected = Some(Date::new(self.year, self.month, 1));
        }
    }

    /// Select previous day
    pub fn select_prev_day(&mut self) {
        if let Some(date) = self.selected {
            if date.day > 1 {
                self.selected = Some(Date::new(date.year, date.month, date.day - 1));
            } else if date.month > 1 {
                let prev_month = date.month - 1;
                let days = days_in_month(date.year, prev_month);
                self.selected = Some(Date::new(date.year, prev_month, days));
                self.month = prev_month;
            } else {
                let days = days_in_month(date.year - 1, 12);
                self.selected = Some(Date::new(date.year - 1, 12, days));
                self.year = date.year - 1;
                self.month = 12;
            }
        } else {
            self.selected = Some(Date::new(self.year, self.month, 1));
        }
    }

    /// Select next week
    pub fn select_next_week(&mut self) {
        for _ in 0..7 {
            self.select_next_day();
        }
    }

    /// Select previous week
    pub fn select_prev_week(&mut self) {
        for _ in 0..7 {
            self.select_prev_day();
        }
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        if !self.focused {
            return false;
        }

        match key {
            Key::Left | Key::Char('h') => {
                self.select_prev_day();
                true
            }
            Key::Right | Key::Char('l') => {
                self.select_next_day();
                true
            }
            Key::Up | Key::Char('k') => {
                self.select_prev_week();
                true
            }
            Key::Down | Key::Char('j') => {
                self.select_next_week();
                true
            }
            Key::Char('[') => {
                self.prev_month();
                true
            }
            Key::Char(']') => {
                self.next_month();
                true
            }
            Key::Char('{') => {
                self.prev_year();
                true
            }
            Key::Char('}') => {
                self.next_year();
                true
            }
            _ => false,
        }
    }
}
