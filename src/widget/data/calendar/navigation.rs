//! Moving the shown month and the selected date, by method or by key

use super::{Calendar, Date};

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

    /// Select a date, and show its month
    pub fn select(&mut self, date: Date) {
        self.selected = Some(date);
        self.show_selected();
    }

    /// Show the month that holds the selection: the view follows every
    /// change of it (#973). `[`, `]`, `{` and `}` move the view alone.
    fn show_selected(&mut self) {
        if let Some(date) = self.selected {
            self.year = date.year;
            self.month = date.month;
        }
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
        self.selected = Some(match self.selected {
            Some(date) => date.next_day(),
            None => Date::new(self.year, self.month, 1),
        });
        self.show_selected();
    }

    /// Select previous day
    pub fn select_prev_day(&mut self) {
        self.selected = Some(match self.selected {
            Some(date) => date.prev_day(),
            None => Date::new(self.year, self.month, 1),
        });
        self.show_selected();
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
