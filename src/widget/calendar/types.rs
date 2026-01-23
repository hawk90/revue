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
