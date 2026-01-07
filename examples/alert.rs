//! Alert Widget Demo - Demonstrates alert levels and variants
//!
//! Run with: cargo run --example alert

use revue::prelude::*;
use revue::widget::{
    alert, error_alert, info_alert, success_alert, warning_alert, Alert, AlertLevel, AlertVariant,
    Text,
};

/// Current view mode
#[derive(Clone, Copy, PartialEq)]
enum ViewTab {
    Levels,
    Variants,
    Features,
}

impl ViewTab {
    fn name(&self) -> &str {
        match self {
            ViewTab::Levels => "Levels",
            ViewTab::Variants => "Variants",
            ViewTab::Features => "Features",
        }
    }

    fn all() -> &'static [ViewTab] {
        &[ViewTab::Levels, ViewTab::Variants, ViewTab::Features]
    }
}

/// Demo application state
struct AlertDemo {
    tab: ViewTab,
    dismissed: [bool; 4],
}

impl AlertDemo {
    fn new() -> Self {
        Self {
            tab: ViewTab::Levels,
            dismissed: [false; 4],
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('1') => {
                self.tab = ViewTab::Levels;
                true
            }
            Key::Char('2') => {
                self.tab = ViewTab::Variants;
                true
            }
            Key::Char('3') => {
                self.tab = ViewTab::Features;
                true
            }
            Key::Char('r') => {
                self.dismissed = [false; 4];
                true
            }
            Key::Char('d') => {
                // Dismiss next non-dismissed alert
                for d in &mut self.dismissed {
                    if !*d {
                        *d = true;
                        break;
                    }
                }
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

    fn render_levels_demo(&self) -> impl View {
        vstack()
            .gap(1)
            .child(Text::new("Alert Levels:").bold())
            .child(Text::new(""))
            .child(info_alert("This is an informational message."))
            .child(Text::new(""))
            .child(success_alert("Operation completed successfully!"))
            .child(Text::new(""))
            .child(warning_alert("Please review before continuing."))
            .child(Text::new(""))
            .child(error_alert("An error occurred. Please try again."))
    }

    fn render_variants_demo(&self) -> impl View {
        vstack()
            .gap(1)
            .child(Text::new("Alert Variants:").bold())
            .child(Text::new(""))
            .child(Text::new("Filled (default):").fg(Color::rgb(150, 150, 150)))
            .child(
                alert("Filled variant with background color")
                    .level(AlertLevel::Info)
                    .variant(AlertVariant::Filled),
            )
            .child(Text::new(""))
            .child(Text::new("Outlined:").fg(Color::rgb(150, 150, 150)))
            .child(
                alert("Outlined variant with border")
                    .level(AlertLevel::Success)
                    .variant(AlertVariant::Outlined),
            )
            .child(Text::new(""))
            .child(Text::new("Minimal:").fg(Color::rgb(150, 150, 150)))
            .child(
                alert("Minimal variant - just icon and text")
                    .level(AlertLevel::Error)
                    .variant(AlertVariant::Minimal),
            )
    }

    fn render_features_demo(&self) -> impl View {
        let mut stack = vstack()
            .gap(1)
            .child(Text::new("Alert Features:").bold())
            .child(Text::new(""))
            .child(Text::new("With Title:").fg(Color::rgb(150, 150, 150)))
            .child(
                Alert::new("Check your inbox for the confirmation link.")
                    .level(AlertLevel::Info)
                    .title("Email Sent"),
            )
            .child(Text::new(""))
            .child(Text::new("Custom Icon:").fg(Color::rgb(150, 150, 150)))
            .child(
                alert("Your changes have been saved to the cloud.")
                    .level(AlertLevel::Success)
                    .custom_icon('☁'),
            )
            .child(Text::new(""))
            .child(Text::new("Without Icon:").fg(Color::rgb(150, 150, 150)))
            .child(
                alert("Simple message without icon.")
                    .level(AlertLevel::Info)
                    .icon(false),
            )
            .child(Text::new(""))
            .child(
                Text::new("Dismissible Alerts (press 'd' to dismiss, 'r' to reset):")
                    .fg(Color::rgb(150, 150, 150)),
            );

        if !self.dismissed[0] {
            stack = stack.child(info_alert("First dismissible alert").dismissible(true));
        }
        if !self.dismissed[1] {
            stack = stack.child(success_alert("Second dismissible alert").dismissible(true));
        }
        if !self.dismissed[2] {
            stack = stack.child(warning_alert("Third dismissible alert").dismissible(true));
        }
        if !self.dismissed[3] {
            stack = stack.child(error_alert("Fourth dismissible alert").dismissible(true));
        }

        if self.dismissed.iter().all(|&d| d) {
            stack = stack.child(
                Text::new("All alerts dismissed! Press 'r' to reset.")
                    .fg(Color::rgb(100, 100, 100)),
            );
        }

        stack
    }
}

impl View for AlertDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let header = hstack()
            .child(Text::new(" Alert Widget Demo ").fg(Color::CYAN).bold())
            .child(Text::new(" | Tab/1-3 to switch").fg(Color::rgb(100, 100, 100)));

        let tabs = self.render_tabs();

        let content = match self.tab {
            ViewTab::Levels => Border::rounded()
                .title("Alert Levels")
                .child(self.render_levels_demo()),
            ViewTab::Variants => Border::rounded()
                .title("Alert Variants")
                .child(self.render_variants_demo()),
            ViewTab::Features => Border::rounded()
                .title("Alert Features")
                .child(self.render_features_demo()),
        };

        let help = Text::new("Press 'q' to quit | Tab: next | 'd': dismiss | 'r': reset")
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
    let demo = AlertDemo::new();

    app.run_with_handler(demo, |key_event, demo| demo.handle_key(&key_event.key))
}
