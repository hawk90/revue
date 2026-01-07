//! Card Widget Demo - Demonstrates Card layouts and variants
//!
//! Run with: cargo run --example card

use revue::prelude::*;
use revue::widget::{card, CardBorder, CardVariant, Text};

/// Current view mode
#[derive(Clone, Copy, PartialEq)]
enum ViewTab {
    Variants,
    Borders,
    Collapsible,
}

impl ViewTab {
    fn name(&self) -> &str {
        match self {
            ViewTab::Variants => "Variants",
            ViewTab::Borders => "Borders",
            ViewTab::Collapsible => "Collapsible",
        }
    }

    fn all() -> &'static [ViewTab] {
        &[ViewTab::Variants, ViewTab::Borders, ViewTab::Collapsible]
    }
}

/// Demo application state
struct CardDemo {
    /// Current tab
    tab: ViewTab,
    /// Collapsible card states
    card1_expanded: bool,
    card2_expanded: bool,
    card3_expanded: bool,
    /// Currently focused collapsible (0-2)
    focused_idx: usize,
}

impl CardDemo {
    fn new() -> Self {
        Self {
            tab: ViewTab::Variants,
            card1_expanded: true,
            card2_expanded: false,
            card3_expanded: true,
            focused_idx: 0,
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('1') => {
                self.tab = ViewTab::Variants;
                true
            }
            Key::Char('2') => {
                self.tab = ViewTab::Borders;
                true
            }
            Key::Char('3') => {
                self.tab = ViewTab::Collapsible;
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
            _ => {
                if self.tab == ViewTab::Collapsible {
                    self.handle_collapsible_key(key)
                } else {
                    false
                }
            }
        }
    }

    fn handle_collapsible_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up | Key::Char('k') => {
                if self.focused_idx > 0 {
                    self.focused_idx -= 1;
                }
                true
            }
            Key::Down | Key::Char('j') => {
                if self.focused_idx < 2 {
                    self.focused_idx += 1;
                }
                true
            }
            Key::Enter | Key::Char(' ') => {
                match self.focused_idx {
                    0 => self.card1_expanded = !self.card1_expanded,
                    1 => self.card2_expanded = !self.card2_expanded,
                    2 => self.card3_expanded = !self.card3_expanded,
                    _ => {}
                }
                true
            }
            Key::Left | Key::Char('h') => {
                match self.focused_idx {
                    0 => self.card1_expanded = false,
                    1 => self.card2_expanded = false,
                    2 => self.card3_expanded = false,
                    _ => {}
                }
                true
            }
            Key::Right | Key::Char('l') => {
                match self.focused_idx {
                    0 => self.card1_expanded = true,
                    1 => self.card2_expanded = true,
                    2 => self.card3_expanded = true,
                    _ => {}
                }
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

    fn render_variants_demo(&self) -> impl View {
        vstack()
            .gap(1)
            .child(Text::new("Card Variants:").bold())
            .child(Text::new(""))
            .child(
                card()
                    .title("Outlined")
                    .subtitle("Default variant with border")
                    .body(Text::new("Content goes here"))
                    .variant(CardVariant::Outlined),
            )
            .child(Text::new(""))
            .child(
                card()
                    .title("Filled")
                    .subtitle("With background color")
                    .body(Text::new("Content goes here"))
                    .variant(CardVariant::Filled),
            )
            .child(Text::new(""))
            .child(
                card()
                    .title("Elevated")
                    .subtitle("Elevated appearance")
                    .body(Text::new("Content goes here"))
                    .variant(CardVariant::Elevated),
            )
            .child(Text::new(""))
            .child(
                card()
                    .title("Flat")
                    .subtitle("No border, minimal style")
                    .body(Text::new("Content goes here"))
                    .variant(CardVariant::Flat),
            )
    }

    fn render_borders_demo(&self) -> impl View {
        vstack()
            .gap(1)
            .child(Text::new("Border Styles:").bold())
            .child(Text::new(""))
            .child(
                card()
                    .title("Single Border")
                    .body(Text::new("Standard single-line border"))
                    .border_style(CardBorder::Single),
            )
            .child(Text::new(""))
            .child(
                card()
                    .title("Rounded Border")
                    .body(Text::new("Rounded corners"))
                    .border_style(CardBorder::Rounded),
            )
            .child(Text::new(""))
            .child(
                card()
                    .title("Double Border")
                    .body(Text::new("Double-line border"))
                    .border_style(CardBorder::Double),
            )
            .child(Text::new(""))
            .child(
                card()
                    .title("No Border")
                    .body(Text::new("No visible border"))
                    .border_style(CardBorder::None)
                    .variant(CardVariant::Filled),
            )
    }

    fn render_collapsible_demo(&self) -> impl View {
        let indicator = |idx: usize, expanded: bool| -> Text {
            let arrow = if expanded { "[-]" } else { "[+]" };
            if idx == self.focused_idx {
                Text::new(format!(" > {}", arrow)).fg(Color::CYAN)
            } else {
                Text::new(format!("   {}", arrow)).fg(Color::rgb(100, 100, 100))
            }
        };

        vstack()
            .gap(1)
            .child(Text::new("Collapsible Cards:").bold())
            .child(Text::new(
                "(j/k or arrows: navigate, Space/Enter: toggle, h/l: collapse/expand)",
            ))
            .child(Text::new(""))
            .child(indicator(0, self.card1_expanded))
            .child(
                card()
                    .title("User Profile")
                    .subtitle("Account Information")
                    .body(Text::new("Name: John Doe\nEmail: john@example.com"))
                    .collapsible(true)
                    .expanded(self.card1_expanded),
            )
            .child(Text::new(""))
            .child(indicator(1, self.card2_expanded))
            .child(
                card()
                    .title("Settings")
                    .body(Text::new("Theme: Dark\nLanguage: English"))
                    .collapsible(true)
                    .expanded(self.card2_expanded),
            )
            .child(Text::new(""))
            .child(indicator(2, self.card3_expanded))
            .child(
                card()
                    .title("Notifications")
                    .subtitle("3 unread")
                    .body(Text::new("- New message\n- Update available\n- Reminder"))
                    .collapsible(true)
                    .expanded(self.card3_expanded),
            )
    }
}

impl View for CardDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let header = hstack()
            .child(Text::new(" Card Widget Demo ").fg(Color::CYAN).bold())
            .child(Text::new(" | Tab/Shift+Tab or 1-3 to switch").fg(Color::rgb(100, 100, 100)));

        let tabs = self.render_tabs();

        let content = match self.tab {
            ViewTab::Variants => Border::rounded()
                .title("Variants")
                .child(self.render_variants_demo()),
            ViewTab::Borders => Border::rounded()
                .title("Border Styles")
                .child(self.render_borders_demo()),
            ViewTab::Collapsible => Border::rounded()
                .title("Collapsible Cards")
                .child(self.render_collapsible_demo()),
        };

        let help =
            Text::new("Press 'q' to quit | Tab: next | Shift+Tab: prev").fg(Color::rgb(80, 80, 80));

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
    let demo = CardDemo::new();

    app.run_with_handler(demo, |key_event, demo| demo.handle_key(&key_event.key))
}
