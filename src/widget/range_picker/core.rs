//! Range picker core struct and constructors

use super::types::{PresetRange, RangeFocus};
use crate::style::Color;
use crate::widget::calendar::{Date, FirstDayOfWeek};
use crate::widget::datetime_picker::{DateTime, Time};
use crate::widget::traits::{WidgetProps, WidgetState};

/// A date/time range picker widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::widget::{range_picker, RangePicker, Date};
///
/// // Basic date range picker
/// let picker = range_picker()
///     .start_date(Date::new(2025, 1, 1))
///     .end_date(Date::new(2025, 1, 31));
///
/// // With presets
/// let picker = range_picker()
///     .with_presets(true);
///
/// // Analytics-style range picker
/// let picker = analytics_range_picker();
/// ```
pub struct RangePicker {
    /// Start datetime
    pub(crate) start: DateTime,
    /// End datetime
    pub(crate) end: DateTime,
    /// Currently active preset
    pub(crate) active_preset: Option<PresetRange>,
    /// Available presets
    pub(crate) presets: Vec<PresetRange>,
    /// Cursor position in presets
    pub(crate) preset_cursor: usize,
    /// Current focus area
    pub(crate) focus: RangeFocus,
    /// First day of week
    pub(crate) first_day: FirstDayOfWeek,
    /// Show time selection
    pub(crate) show_time: bool,
    /// Calendar cursor day (for start)
    pub(crate) start_cursor_day: u32,
    /// Calendar cursor day (for end)
    pub(crate) end_cursor_day: u32,
    /// Minimum allowed date
    pub(crate) min_date: Option<Date>,
    /// Maximum allowed date
    pub(crate) max_date: Option<Date>,
    /// Show presets panel
    pub(crate) show_presets: bool,
    /// Colors
    pub(crate) header_fg: Color,
    pub(crate) selected_fg: Color,
    pub(crate) selected_bg: Color,
    pub(crate) range_bg: Color,
    pub(crate) preset_fg: Color,
    pub(crate) preset_selected_fg: Color,
    pub(crate) preset_selected_bg: Color,
    /// Widget state
    pub state: WidgetState,
    /// Widget props
    pub props: WidgetProps,
}

impl RangePicker {
    /// Create a new range picker
    pub fn new() -> Self {
        let today = Date::today();
        Self {
            start: DateTime::new(today, Time::new(0, 0, 0)),
            end: DateTime::new(today, Time::new(23, 59, 59)),
            active_preset: Some(PresetRange::Today),
            presets: PresetRange::common().to_vec(),
            preset_cursor: 0,
            focus: RangeFocus::Start,
            first_day: FirstDayOfWeek::Sunday,
            show_time: false,
            start_cursor_day: today.day,
            end_cursor_day: today.day,
            min_date: None,
            max_date: None,
            show_presets: true,
            header_fg: Color::CYAN,
            selected_fg: Color::BLACK,
            selected_bg: Color::CYAN,
            range_bg: Color::rgb(60, 100, 140),
            preset_fg: Color::WHITE,
            preset_selected_fg: Color::BLACK,
            preset_selected_bg: Color::CYAN,
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }
}

impl Default for RangePicker {
    fn default() -> Self {
        Self::new()
    }
}
