//! DateTime Picker Widget Demo
//!
//! Run with: cargo run --example datetime_picker

use revue::prelude::*;
use revue::widget::{
    date_picker, datetime_picker, time_picker, Date, DateTimeMode, DateTimePicker, Text, Time,
};

/// Current view mode
#[derive(Clone, Copy, PartialEq)]
enum ViewTab {
    DateTime,
    DateOnly,
    TimeOnly,
}

impl ViewTab {
    fn name(&self) -> &str {
        match self {
            ViewTab::DateTime => "DateTime",
            ViewTab::DateOnly => "Date Only",
            ViewTab::TimeOnly => "Time Only",
        }
    }

    fn all() -> &'static [ViewTab] {
        &[ViewTab::DateTime, ViewTab::DateOnly, ViewTab::TimeOnly]
    }
}

/// Demo application state
struct DateTimePickerDemo {
    tab: ViewTab,
    datetime_picker: DateTimePicker,
    date_picker: DateTimePicker,
    time_picker: DateTimePicker,
}

impl DateTimePickerDemo {
    fn new() -> Self {
        Self {
            tab: ViewTab::DateTime,
            datetime_picker: datetime_picker()
                .selected_date(Date::new(2025, 6, 15))
                .selected_time(Time::new(14, 30, 0))
                .show_seconds(true),
            date_picker: date_picker().selected_date(Date::new(2025, 1, 1)),
            time_picker: time_picker()
                .selected_time(Time::new(9, 0, 0))
                .show_seconds(false),
        }
    }

    fn current_picker(&mut self) -> &mut DateTimePicker {
        match self.tab {
            ViewTab::DateTime => &mut self.datetime_picker,
            ViewTab::DateOnly => &mut self.date_picker,
            ViewTab::TimeOnly => &mut self.time_picker,
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('1') => {
                self.tab = ViewTab::DateTime;
                true
            }
            Key::Char('2') => {
                self.tab = ViewTab::DateOnly;
                true
            }
            Key::Char('3') => {
                self.tab = ViewTab::TimeOnly;
                true
            }
            Key::BackTab => {
                let tabs = ViewTab::all();
                let idx = tabs.iter().position(|&t| t == self.tab).unwrap_or(0);
                self.tab = tabs[(idx + tabs.len() - 1) % tabs.len()];
                true
            }
            _ => self.current_picker().handle_key(key),
        }
    }

    fn render_tabs(&self) -> impl View {
        // A stack shares its space equally among the children added with
        // `child`; it does not size them to their content. So everything in
        // this file that should take only its own rows or columns says so with
        // `child_sized`, and only the piece that should take the rest is left
        // as a plain `child`.
        let mut tabs = hstack().gap(2);

        for (i, tab) in ViewTab::all().iter().enumerate() {
            let label = format!("[{}] {}", i + 1, tab.name());
            let width = cols(&label);
            let text = if *tab == self.tab {
                Text::new(label).fg(Color::CYAN).bold()
            } else {
                Text::new(label).fg(Color::rgb(128, 128, 128))
            };
            tabs = tabs.child_sized(text, width);
        }

        tabs
    }

    fn render_datetime_demo(&self) -> impl View {
        let dt = self.datetime_picker.get_datetime();
        let mode_str = match self.datetime_picker.get_mode() {
            DateTimeMode::Date => "Date",
            DateTimeMode::Time => "Time",
        };

        // The combined picker is 13 rows: the calendar, then the time below it.
        vstack()
            .child_sized(Text::new("Combined DateTime Picker:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(
                self.datetime_picker
                    .clone_picker(datetime_picker().show_seconds(true)),
                13,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new(format!(
                    "Selected: {}-{:02}-{:02} {:02}:{:02}:{:02}",
                    dt.date.year,
                    dt.date.month,
                    dt.date.day,
                    dt.time.hour,
                    dt.time.minute,
                    dt.time.second
                )),
                1,
            )
            .child_sized(
                Text::new(format!("Current mode: {}", mode_str)).fg(Color::rgb(150, 150, 150)),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new("Navigation: arrows/hjkl | [/]: month | {/}: year | Tab: date/time")
                    .fg(Color::rgb(100, 100, 100)),
                1,
            )
    }

    fn render_date_demo(&self) -> impl View {
        let date = self.date_picker.get_date();

        // The calendar is 8 rows: month, weekdays and six weeks.
        vstack()
            .child_sized(Text::new("Date Only Picker:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(self.date_picker.clone_picker(date_picker()), 8)
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new(format!(
                    "Selected: {}-{:02}-{:02}",
                    date.year, date.month, date.day
                )),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new("Navigation: arrows/hjkl | [/]: month | {/}: year | Enter: select")
                    .fg(Color::rgb(100, 100, 100)),
                1,
            )
    }

    fn render_time_demo(&self) -> impl View {
        let time = self.time_picker.get_time();

        // The time picker draws 4 rows (label, fields, a blank, its help
        // line) but draws nothing at all in fewer than 5.
        vstack()
            .child_sized(Text::new("Time Only Picker:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(
                self.time_picker
                    .clone_picker(time_picker().show_seconds(false)),
                5,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new(format!("Selected: {:02}:{:02}", time.hour, time.minute)),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new("Navigation: ←→: field | ↑↓: value").fg(Color::rgb(100, 100, 100)),
                1,
            )
    }
}

// Helper trait to clone the picker for rendering
trait ClonePicker {
    fn clone_picker(&self, fresh: DateTimePicker) -> DateTimePicker;
}

impl ClonePicker for DateTimePicker {
    /// `fresh`, built the way this picker was, showing this picker's date
    /// and time. Built from `fresh` so each tab keeps its own format: always
    /// rebuilding a combined picker put a calendar on the Time Only tab.
    fn clone_picker(&self, fresh: DateTimePicker) -> DateTimePicker {
        fresh
            .selected_date(self.get_date())
            .selected_time(self.get_time())
    }
}

impl View for DateTimePickerDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let title = " DateTime Picker Demo ";
        let header = hstack()
            .child_sized(Text::new(title).fg(Color::CYAN).bold(), cols(title))
            .child(Text::new(" | 1-3 to switch views").fg(Color::rgb(100, 100, 100)));

        let tabs = self.render_tabs();

        let content = match self.tab {
            ViewTab::DateTime => Border::rounded()
                .title("DateTime")
                .child(self.render_datetime_demo()),
            ViewTab::DateOnly => Border::rounded()
                .title("Date Only")
                .child(self.render_date_demo()),
            ViewTab::TimeOnly => Border::rounded()
                .title("Time Only")
                .child(self.render_time_demo()),
        };

        let help = Text::new("Press 'q' to quit").fg(Color::rgb(80, 80, 80));

        vstack()
            .child_sized(header, 1)
            .child_sized(tabs, 1)
            .child_sized(Text::new(""), 1)
            .child(content)
            .child_sized(Text::new(""), 1)
            .child_sized(help, 1)
            .render(ctx);
    }
}

/// Columns `s` takes on screen.
fn cols(s: &str) -> u16 {
    revue::utils::unicode::display_width(s) as u16
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let demo = DateTimePickerDemo::new();

    app.run_with_handler(demo, |key_event, demo| demo.handle_key(&key_event.key))
}
