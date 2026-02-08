//! Calendar type definitions

use crate::style::Color;

/// Calendar display mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CalendarMode {
    /// Single month view
    #[default]
    Month,
    /// Year overview (12 months)
    Year,
    /// Week view
    Week,
}

/// First day of week
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FirstDayOfWeek {
    /// Sunday first (US style)
    #[default]
    Sunday,
    /// Monday first (ISO style)
    Monday,
}

/// Date marker for highlighting specific dates
#[derive(Clone, Debug)]
pub struct DateMarker {
    /// Date to mark
    pub date: super::Date,
    /// Marker color
    pub color: Color,
    /// Optional symbol
    pub symbol: Option<char>,
}

impl DateMarker {
    /// Create a new marker
    pub fn new(date: super::Date, color: Color) -> Self {
        Self {
            date,
            color,
            symbol: None,
        }
    }

    /// Set symbol
    pub fn symbol(mut self, symbol: char) -> Self {
        self.symbol = Some(symbol);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // CalendarMode enum tests
    // =========================================================================

    #[test]
    fn test_calendar_mode_default() {
        let mode = CalendarMode::default();
        assert_eq!(mode, CalendarMode::Month);
    }

    #[test]
    fn test_calendar_mode_clone() {
        let mode = CalendarMode::Year;
        assert_eq!(mode.clone(), CalendarMode::Year);
    }

    #[test]
    fn test_calendar_mode_copy() {
        let mode1 = CalendarMode::Week;
        let mode2 = mode1;
        assert_eq!(mode2, CalendarMode::Week);
    }

    #[test]
    fn test_calendar_mode_partial_eq() {
        let mode1 = CalendarMode::Month;
        let mode2 = CalendarMode::Month;
        assert_eq!(mode1, mode2);
    }

    #[test]
    fn test_calendar_mode_partial_ne() {
        let mode1 = CalendarMode::Month;
        let mode2 = CalendarMode::Year;
        assert_ne!(mode1, mode2);
    }

    #[test]
    fn test_calendar_mode_month() {
        let mode = CalendarMode::Month;
        assert_eq!(mode, CalendarMode::Month);
    }

    #[test]
    fn test_calendar_mode_year() {
        let mode = CalendarMode::Year;
        assert_eq!(mode, CalendarMode::Year);
    }

    #[test]
    fn test_calendar_mode_week() {
        let mode = CalendarMode::Week;
        assert_eq!(mode, CalendarMode::Week);
    }

    // =========================================================================
    // FirstDayOfWeek enum tests
    // =========================================================================

    #[test]
    fn test_first_day_of_week_default() {
        let day = FirstDayOfWeek::default();
        assert_eq!(day, FirstDayOfWeek::Sunday);
    }

    #[test]
    fn test_first_day_of_week_clone() {
        let day = FirstDayOfWeek::Monday;
        assert_eq!(day.clone(), FirstDayOfWeek::Monday);
    }

    #[test]
    fn test_first_day_of_week_copy() {
        let day1 = FirstDayOfWeek::Monday;
        let day2 = day1;
        assert_eq!(day2, FirstDayOfWeek::Monday);
    }

    #[test]
    fn test_first_day_of_week_partial_eq() {
        let day1 = FirstDayOfWeek::Sunday;
        let day2 = FirstDayOfWeek::Sunday;
        assert_eq!(day1, day2);
    }

    #[test]
    fn test_first_day_of_week_partial_ne() {
        let day1 = FirstDayOfWeek::Sunday;
        let day2 = FirstDayOfWeek::Monday;
        assert_ne!(day1, day2);
    }

    #[test]
    fn test_first_day_of_week_sunday() {
        let day = FirstDayOfWeek::Sunday;
        assert_eq!(day, FirstDayOfWeek::Sunday);
    }

    #[test]
    fn test_first_day_of_week_monday() {
        let day = FirstDayOfWeek::Monday;
        assert_eq!(day, FirstDayOfWeek::Monday);
    }

    // =========================================================================
    // DateMarker::new tests
    // =========================================================================

    #[test]
    fn test_date_marker_new() {
        let date = super::super::Date {
            year: 2024,
            month: 1,
            day: 1,
        };
        let marker = DateMarker::new(date, Color::RED);
        assert_eq!(marker.date.year, 2024);
        assert_eq!(marker.date.month, 1);
        assert_eq!(marker.date.day, 1);
        assert_eq!(marker.color, Color::RED);
        assert!(marker.symbol.is_none());
    }

    #[test]
    fn test_date_marker_clone() {
        let date = super::super::Date {
            year: 2024,
            month: 1,
            day: 1,
        };
        let marker1 = DateMarker::new(date, Color::BLUE);
        let marker2 = marker1.clone();
        assert_eq!(marker1.date.year, marker2.date.year);
        assert_eq!(marker1.color, marker2.color);
    }

    // =========================================================================
    // DateMarker::symbol tests
    // =========================================================================

    #[test]
    fn test_date_marker_symbol() {
        let date = super::super::Date {
            year: 2024,
            month: 1,
            day: 1,
        };
        let marker = DateMarker::new(date, Color::GREEN).symbol('*');
        assert_eq!(marker.symbol, Some('*'));
    }

    #[test]
    fn test_date_marker_builder_chain() {
        let date = super::super::Date {
            year: 2024,
            month: 12,
            day: 25,
        };
        let marker = DateMarker::new(date, Color::RED).symbol('🎄');
        assert_eq!(marker.symbol, Some('🎄'));
        assert_eq!(marker.color, Color::RED);
    }

    #[test]
    fn test_date_marker_no_symbol() {
        let date = super::super::Date {
            year: 2024,
            month: 1,
            day: 1,
        };
        let marker = DateMarker::new(date, Color::YELLOW);
        assert!(marker.symbol.is_none());
    }
}
