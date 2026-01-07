//! StatusIndicator Widget Demo - Demonstrates status states and styles
//!
//! Run with: cargo run --example status_indicator

use revue::prelude::*;
use revue::widget::{
    away_indicator, busy_indicator, offline, online, status_indicator, Status, StatusSize,
    StatusStyle, Text,
};

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
}

impl StatusIndicatorDemo {
    fn new() -> Self {
        Self {
            tab: ViewTab::States,
            pulsing: false,
        }
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

    fn render_states_demo(&self) -> impl View {
        vstack()
            .gap(1)
            .child(Text::new("Status States:").bold())
            .child(Text::new(""))
            .child(
                hstack()
                    .gap(2)
                    .child(online().pulsing(self.pulsing))
                    .child(Text::new("Online - User is available")),
            )
            .child(Text::new(""))
            .child(
                hstack()
                    .gap(2)
                    .child(offline())
                    .child(Text::new("Offline - User is disconnected")),
            )
            .child(Text::new(""))
            .child(
                hstack()
                    .gap(2)
                    .child(busy_indicator().pulsing(self.pulsing))
                    .child(Text::new("Busy - Do not disturb")),
            )
            .child(Text::new(""))
            .child(
                hstack()
                    .gap(2)
                    .child(away_indicator())
                    .child(Text::new("Away - Temporarily unavailable")),
            )
            .child(Text::new(""))
            .child(
                hstack()
                    .gap(2)
                    .child(status_indicator(Status::Unknown))
                    .child(Text::new("Unknown - Status not determined")),
            )
            .child(Text::new(""))
            .child(
                hstack()
                    .gap(2)
                    .child(status_indicator(Status::Error).pulsing(self.pulsing))
                    .child(Text::new("Error - Connection issue")),
            )
            .child(Text::new(""))
            .child(
                Text::new(format!(
                    "Press 'p' to toggle pulsing (currently: {})",
                    if self.pulsing { "ON" } else { "OFF" }
                ))
                .fg(Color::rgb(100, 100, 100)),
            )
    }

    fn render_styles_demo(&self) -> impl View {
        vstack()
            .gap(1)
            .child(Text::new("Display Styles:").bold())
            .child(Text::new(""))
            .child(Text::new("Dot (default):").fg(Color::rgb(150, 150, 150)))
            .child(
                hstack()
                    .gap(4)
                    .child(online().indicator_style(StatusStyle::Dot))
                    .child(busy_indicator().indicator_style(StatusStyle::Dot))
                    .child(away_indicator().indicator_style(StatusStyle::Dot))
                    .child(offline().indicator_style(StatusStyle::Dot)),
            )
            .child(Text::new(""))
            .child(Text::new("Dot with Label:").fg(Color::rgb(150, 150, 150)))
            .child(
                hstack()
                    .gap(2)
                    .child(online().indicator_style(StatusStyle::DotWithLabel))
                    .child(busy_indicator().indicator_style(StatusStyle::DotWithLabel))
                    .child(away_indicator().indicator_style(StatusStyle::DotWithLabel)),
            )
            .child(Text::new(""))
            .child(Text::new("Label Only:").fg(Color::rgb(150, 150, 150)))
            .child(
                hstack()
                    .gap(2)
                    .child(online().indicator_style(StatusStyle::LabelOnly))
                    .child(busy_indicator().indicator_style(StatusStyle::LabelOnly))
                    .child(offline().indicator_style(StatusStyle::LabelOnly)),
            )
            .child(Text::new(""))
            .child(Text::new("Badge:").fg(Color::rgb(150, 150, 150)))
            .child(
                hstack()
                    .gap(2)
                    .child(online().indicator_style(StatusStyle::Badge))
                    .child(busy_indicator().indicator_style(StatusStyle::Badge))
                    .child(away_indicator().indicator_style(StatusStyle::Badge)),
            )
            .child(Text::new(""))
            .child(Text::new("Custom Label:").fg(Color::rgb(150, 150, 150)))
            .child(
                hstack()
                    .gap(2)
                    .child(
                        online()
                            .indicator_style(StatusStyle::DotWithLabel)
                            .label("Available"),
                    )
                    .child(
                        busy_indicator()
                            .indicator_style(StatusStyle::DotWithLabel)
                            .label("In Meeting"),
                    ),
            )
    }

    fn render_sizes_demo(&self) -> impl View {
        vstack()
            .gap(1)
            .child(Text::new("Size Variants:").bold())
            .child(Text::new(""))
            .child(Text::new("Small:").fg(Color::rgb(150, 150, 150)))
            .child(
                hstack()
                    .gap(2)
                    .child(
                        online()
                            .size(StatusSize::Small)
                            .indicator_style(StatusStyle::DotWithLabel),
                    )
                    .child(
                        busy_indicator()
                            .size(StatusSize::Small)
                            .indicator_style(StatusStyle::Badge),
                    ),
            )
            .child(Text::new(""))
            .child(Text::new("Medium (default):").fg(Color::rgb(150, 150, 150)))
            .child(
                hstack()
                    .gap(2)
                    .child(
                        online()
                            .size(StatusSize::Medium)
                            .indicator_style(StatusStyle::DotWithLabel),
                    )
                    .child(
                        busy_indicator()
                            .size(StatusSize::Medium)
                            .indicator_style(StatusStyle::Badge),
                    ),
            )
            .child(Text::new(""))
            .child(Text::new("Large:").fg(Color::rgb(150, 150, 150)))
            .child(
                hstack()
                    .gap(2)
                    .child(
                        online()
                            .size(StatusSize::Large)
                            .indicator_style(StatusStyle::DotWithLabel),
                    )
                    .child(
                        busy_indicator()
                            .size(StatusSize::Large)
                            .indicator_style(StatusStyle::Badge),
                    ),
            )
    }
}

impl View for StatusIndicatorDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let header = hstack()
            .child(Text::new(" StatusIndicator Demo ").fg(Color::CYAN).bold())
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
    let demo = StatusIndicatorDemo::new();

    app.run_with_handler(demo, |key_event, demo| demo.handle_key(&key_event.key))
}
