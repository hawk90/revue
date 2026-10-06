//! StatusIndicator Widget Demo - Demonstrates status states and styles
//!
//! Run with: cargo run --example status_indicator

use revue::prelude::*;
use revue::widget::{
    away_indicator, busy_indicator, offline, online, status_indicator, Status, StatusIndicator,
    StatusSize, StatusStyle, Text,
};
use std::time::{Duration, Instant};

/// Current view mode
#[derive(Clone, Copy, PartialEq)]
enum ViewTab {
    States,
    Styles,
    Sizes,
}

impl ViewTab {
    fn name(&self) -> &str {
        match self {
            ViewTab::States => "States",
            ViewTab::Styles => "Styles",
            ViewTab::Sizes => "Sizes",
        }
    }

    fn all() -> &'static [ViewTab] {
        &[ViewTab::States, ViewTab::Styles, ViewTab::Sizes]
    }
}

/// Demo application state
struct StatusIndicatorDemo {
    tab: ViewTab,
    pulsing: bool,
    /// Pulse frame, kept here because the indicators are rebuilt every render
    pulse_frame: usize,
    last_pulse: Instant,
}

impl StatusIndicatorDemo {
    fn new() -> Self {
        Self {
            tab: ViewTab::States,
            pulsing: false,
            pulse_frame: 0,
            last_pulse: Instant::now(),
        }
    }

    /// Advance the pulse about every 100ms; redraw only when it moved
    fn tick(&mut self) -> bool {
        if !self.pulsing || self.last_pulse.elapsed() < Duration::from_millis(100) {
            return false;
        }
        self.last_pulse = Instant::now();
        self.pulse_frame = self.pulse_frame.wrapping_add(1);
        true
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('1') => {
                self.tab = ViewTab::States;
                true
            }
            Key::Char('2') => {
                self.tab = ViewTab::Styles;
                true
            }
            Key::Char('3') => {
                self.tab = ViewTab::Sizes;
                true
            }
            Key::Char('p') => {
                self.pulsing = !self.pulsing;
                true
            }
            Key::Tab => {
                let tabs = ViewTab::all();
                let idx = tabs.iter().position(|&t| t == self.tab).unwrap_or(0);
                self.tab = tabs[(idx + 1) % tabs.len()];
                true
            }
            Key::BackTab => {
                let tabs = ViewTab::all();
                let idx = tabs.iter().position(|&t| t == self.tab).unwrap_or(0);
                self.tab = tabs[(idx + tabs.len() - 1) % tabs.len()];
                true
            }
            _ => false,
        }
    }

    fn render_tabs(&self) -> impl View {
        // A stack sizes each child added with `child` to its content. The
        // rows and columns in this file are spelled out with `child_sized`
        // anyway, and the tab body is added with `child_flex` so it takes the
        // rest of the screen whatever it holds.
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

    fn render_states_demo(&self) -> impl View {
        vstack()
            .child_sized(Text::new("Status States:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(
                indicators(2, [online().pulsing(self.pulsing).frame(self.pulse_frame)])
                    .child(Text::new("Online - User is available")),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                indicators(2, [offline()]).child(Text::new("Offline - User is disconnected")),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                indicators(
                    2,
                    [busy_indicator()
                        .pulsing(self.pulsing)
                        .frame(self.pulse_frame)],
                )
                .child(Text::new("Busy - Do not disturb")),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                indicators(2, [away_indicator()])
                    .child(Text::new("Away - Temporarily unavailable")),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                indicators(2, [status_indicator(Status::Unknown)])
                    .child(Text::new("Unknown - Status not determined")),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                indicators(
                    2,
                    [status_indicator(Status::Error)
                        .pulsing(self.pulsing)
                        .frame(self.pulse_frame)],
                )
                .child(Text::new("Error - Connection issue")),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new(format!(
                    "Press 'p' to toggle pulsing (currently: {})",
                    if self.pulsing { "ON" } else { "OFF" }
                ))
                .fg(Color::rgb(100, 100, 100)),
                1,
            )
    }

    fn render_styles_demo(&self) -> impl View {
        vstack()
            .child_sized(Text::new("Display Styles:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Dot (default):").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(
                indicators(
                    4,
                    [
                        online().indicator_style(StatusStyle::Dot),
                        busy_indicator().indicator_style(StatusStyle::Dot),
                        away_indicator().indicator_style(StatusStyle::Dot),
                        offline().indicator_style(StatusStyle::Dot),
                    ],
                ),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new("Dot with Label:").fg(Color::rgb(150, 150, 150)),
                1,
            )
            .child_sized(
                indicators(
                    2,
                    [
                        online().indicator_style(StatusStyle::DotWithLabel),
                        busy_indicator().indicator_style(StatusStyle::DotWithLabel),
                        away_indicator().indicator_style(StatusStyle::DotWithLabel),
                    ],
                ),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Label Only:").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(
                indicators(
                    2,
                    [
                        online().indicator_style(StatusStyle::LabelOnly),
                        busy_indicator().indicator_style(StatusStyle::LabelOnly),
                        offline().indicator_style(StatusStyle::LabelOnly),
                    ],
                ),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Badge:").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(
                indicators(
                    2,
                    [
                        online().indicator_style(StatusStyle::Badge),
                        busy_indicator().indicator_style(StatusStyle::Badge),
                        away_indicator().indicator_style(StatusStyle::Badge),
                    ],
                ),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Custom Label:").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(
                indicators(
                    2,
                    [
                        online()
                            .indicator_style(StatusStyle::DotWithLabel)
                            .label("Available"),
                        busy_indicator()
                            .indicator_style(StatusStyle::DotWithLabel)
                            .label("In Meeting"),
                    ],
                ),
                1,
            )
    }

    fn render_sizes_demo(&self) -> impl View {
        vstack()
            .child_sized(Text::new("Size Variants:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Small:").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(
                indicators(
                    2,
                    [
                        online()
                            .size(StatusSize::Small)
                            .indicator_style(StatusStyle::DotWithLabel),
                        busy_indicator()
                            .size(StatusSize::Small)
                            .indicator_style(StatusStyle::Badge),
                    ],
                ),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new("Medium (default):").fg(Color::rgb(150, 150, 150)),
                1,
            )
            .child_sized(
                indicators(
                    2,
                    [
                        online()
                            .size(StatusSize::Medium)
                            .indicator_style(StatusStyle::DotWithLabel),
                        busy_indicator()
                            .size(StatusSize::Medium)
                            .indicator_style(StatusStyle::Badge),
                    ],
                ),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Large:").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(
                indicators(
                    2,
                    [
                        online()
                            .size(StatusSize::Large)
                            .indicator_style(StatusStyle::DotWithLabel),
                        busy_indicator()
                            .size(StatusSize::Large)
                            .indicator_style(StatusStyle::Badge),
                    ],
                ),
                1,
            )
    }
}

/// A row of indicators, each as wide as it draws.
fn indicators(gap: u16, items: impl IntoIterator<Item = StatusIndicator>) -> Stack {
    items.into_iter().fold(hstack().gap(gap), |row, indicator| {
        let width = indicator.width();
        row.child_sized(indicator, width)
    })
}

impl View for StatusIndicatorDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let title = " StatusIndicator Demo ";
        let header = hstack()
            .child_sized(Text::new(title).fg(Color::CYAN).bold(), cols(title))
            .child(Text::new(" | Tab/1-3 to switch").fg(Color::rgb(100, 100, 100)));

        let tabs = self.render_tabs();

        let content = match self.tab {
            ViewTab::States => Border::rounded()
                .title("Status States")
                .child(self.render_states_demo()),
            ViewTab::Styles => Border::rounded()
                .title("Display Styles")
                .child(self.render_styles_demo()),
            ViewTab::Sizes => Border::rounded()
                .title("Size Variants")
                .child(self.render_sizes_demo()),
        };

        let help = Text::new("Press 'q' to quit | Tab: next | 'p': toggle pulse")
            .fg(Color::rgb(80, 80, 80));

        vstack()
            .child_sized(header, 1)
            .child_sized(tabs, 1)
            .child_sized(Text::new(""), 1)
            .child_flex(content, 1.0)
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
    let demo = StatusIndicatorDemo::new();

    app.run(demo, |event, demo, _app| match event {
        Event::Key(key_event) => demo.handle_key(&key_event.key),
        Event::Tick => demo.tick(),
        _ => false,
    })
}
