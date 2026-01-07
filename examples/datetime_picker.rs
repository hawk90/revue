//! DateTime Picker Widget Demo
//!
//! Run with: cargo run --example datetime_picker

use revue::prelude::*;
use revue::widget::{
    date_picker, datetime_picker, time_picker, Date, DateTimeFormat, DateTimeMode, DateTimePicker,
    Text, Time,
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
        let mut tabs = hstack().gap(2);

        for (i, tab) in ViewTab::all().iter().enumerate() {
            let label = format!("[{}] {}", i + 1, tab.name());
            let text = if *tab == self.tab {
                Text::new(label).fg(Color::CYAN).bold()
            } else {
                Text::new(label).fg(Color::rgb(128, 128, 128))
            };
            tabs = tabs.child(text);
        }

        tabs
    }

    fn render_datetime_demo(&self) -> impl View {
        let dt = self.datetime_picker.get_datetime();
        let mode_str = match self.datetime_picker.get_mode() {
            DateTimeMode::Date => "Date",
            DateTimeMode::Time => "Time",
        };

        vstack()
            .gap(1)
            .child(Text::new("Combined DateTime Picker:").bold())
            .child(Text::new(""))
            .child(self.datetime_picker.clone_picker())
            .child(Text::new(""))
            .child(Text::new(format!(
                "Selected: {}-{:02}-{:02} {:02}:{:02}:{:02}",
                dt.date.year,
                dt.date.month,
                dt.date.day,
                dt.time.hour,
                dt.time.minute,
                dt.time.second
            )))
            .child(Text::new(format!("Current mode: {}", mode_str)).fg(Color::rgb(150, 150, 150)))
            .child(Text::new(""))
            .child(
                Text::new("Navigation: arrows/hjkl | [/]: month | {/}: year | Tab: date/time")
                    .fg(Color::rgb(100, 100, 100)),
            )
    }

    fn render_date_demo(&self) -> impl View {
        let date = self.date_picker.get_date();

        vstack()
            .gap(1)
            .child(Text::new("Date Only Picker:").bold())
            .child(Text::new(""))
            .child(self.date_picker.clone_picker())
            .child(Text::new(""))
            .child(Text::new(format!(
                "Selected: {}-{:02}-{:02}",
                date.year, date.month, date.day
            )))
            .child(Text::new(""))
            .child(
                Text::new("Navigation: arrows/hjkl | [/]: month | {/}: year | Enter: select")
                    .fg(Color::rgb(100, 100, 100)),
            )
    }

    fn render_time_demo(&self) -> impl View {
        let time = self.time_picker.get_time();

        vstack()
            .gap(1)
            .child(Text::new("Time Only Picker:").bold())
            .child(Text::new(""))
            .child(self.time_picker.clone_picker())
            .child(Text::new(""))
            .child(Text::new(format!(
                "Selected: {:02}:{:02}",
                time.hour, time.minute
            )))
            .child(Text::new(""))
            .child(Text::new("Navigation: ←→: field | ↑↓: value").fg(Color::rgb(100, 100, 100)))
    }
}

// Helper trait to clone the picker for rendering
trait ClonePicker {
    fn clone_picker(&self) -> DateTimePicker;
}

impl ClonePicker for DateTimePicker {
    fn clone_picker(&self) -> DateTimePicker {
        // Recreate based on current state
        let date = self.get_date();
        let time = self.get_time();

        datetime_picker()
            .selected_date(date)
            .selected_time(time)
            .format(DateTimeFormat::DateTime)
    }
}

impl View for DateTimePickerDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let header = hstack()
            .child(Text::new(" DateTime Picker Demo ").fg(Color::CYAN).bold())
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
            .child(header)
            .child(tabs)
            .child(Text::new(""))
            .child(content)
            .child(Text::new(""))
            .child(help)
            .render(ctx);
    }
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let demo = DateTimePickerDemo::new();

    app.run_with_handler(demo, |key_event, demo| demo.handle_key(&key_event.key))
}
