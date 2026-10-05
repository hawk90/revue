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

    fn render_levels_demo(&self) -> impl View {
        // A filled alert is 3 rows: its border around the message. The
        // outlined and minimal ones are a single row.
        vstack()
            .child_sized(Text::new("Alert Levels:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(info_alert("This is an informational message."), 3)
            .child_sized(Text::new(""), 1)
            .child_sized(success_alert("Operation completed successfully!"), 3)
            .child_sized(Text::new(""), 1)
            .child_sized(warning_alert("Please review before continuing."), 3)
            .child_sized(Text::new(""), 1)
            .child_sized(error_alert("An error occurred. Please try again."), 3)
    }

    fn render_variants_demo(&self) -> impl View {
        vstack()
            .child_sized(Text::new("Alert Variants:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new("Filled (default):").fg(Color::rgb(150, 150, 150)),
                1,
            )
            .child_sized(
                alert("Filled variant with background color")
                    .level(AlertLevel::Info)
                    .variant(AlertVariant::Filled),
                3,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Outlined:").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(
                alert("Outlined variant with border")
                    .level(AlertLevel::Success)
                    .variant(AlertVariant::Outlined),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Minimal:").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(
                alert("Minimal variant - just icon and text")
                    .level(AlertLevel::Error)
                    .variant(AlertVariant::Minimal),
                1,
            )
    }

    fn render_features_demo(&self) -> impl View {
        // Everything here is 31 rows in one column, so the dismissible alerts
        // get a column of their own.
        let features = vstack()
            .child_sized(Text::new("With Title:").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(
                Alert::new("Check your inbox for the confirmation link.")
                    .level(AlertLevel::Info)
                    .title("Email Sent"),
                4,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Custom Icon:").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(
                alert("Your changes have been saved to the cloud.")
                    .level(AlertLevel::Success)
                    .custom_icon('☁'),
                3,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Without Icon:").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(
                alert("Simple message without icon.")
                    .level(AlertLevel::Info)
                    .icon(false),
                3,
            );

        let mut dismissible = vstack()
            .child_sized(
                Text::new("Dismissible Alerts:").fg(Color::rgb(150, 150, 150)),
                1,
            )
            .child_sized(
                Text::new("(press 'd' to dismiss, 'r' to reset)").fg(Color::rgb(150, 150, 150)),
                1,
            );

        if !self.dismissed[0] {
            dismissible =
                dismissible.child_sized(info_alert("First dismissible alert").dismissible(true), 3);
        }
        if !self.dismissed[1] {
            dismissible = dismissible.child_sized(
                success_alert("Second dismissible alert").dismissible(true),
                3,
            );
        }
        if !self.dismissed[2] {
            dismissible = dismissible.child_sized(
                warning_alert("Third dismissible alert").dismissible(true),
                3,
            );
        }
        if !self.dismissed[3] {
            dismissible = dismissible
                .child_sized(error_alert("Fourth dismissible alert").dismissible(true), 3);
        }

        if self.dismissed.iter().all(|&d| d) {
            dismissible = dismissible.child_sized(
                Text::new("All alerts dismissed! Press 'r' to reset.")
                    .fg(Color::rgb(100, 100, 100)),
                1,
            );
        }

        vstack()
            .child_sized(Text::new("Alert Features:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child(hstack().gap(2).child(features).child_sized(dismissible, 42))
    }
}

impl View for AlertDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let title = " Alert Widget Demo ";
        let header = hstack()
            .child_sized(Text::new(title).fg(Color::CYAN).bold(), cols(title))
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
    let demo = AlertDemo::new();

    app.run_with_handler(demo, |key_event, demo| demo.handle_key(&key_event.key))
}
