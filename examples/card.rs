//! Card Widget Demo - Demonstrates Card layouts and variants
//!
//! Run with: cargo run --example card

use revue::prelude::*;
use revue::widget::{card, BorderType, CardVariant, Text};

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

    fn render_variants_demo(&self) -> impl View {
        // A bordered card with a title, a subtitle and a one-line body is 6
        // rows (border, title, subtitle, separator, body, border). Four of
        // them are taller than the panel, so they go two by two.
        vstack()
            .child_sized(Text::new("Card Variants:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(
                hstack()
                    .gap(2)
                    .child(
                        card()
                            .title("Outlined")
                            .subtitle("Default variant with border")
                            .body(Text::new("Content goes here"))
                            .variant(CardVariant::Outlined),
                    )
                    .child(
                        card()
                            .title("Filled")
                            .subtitle("With background color")
                            .body(Text::new("Content goes here"))
                            .variant(CardVariant::Filled),
                    ),
                6,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                hstack()
                    .gap(2)
                    .child(
                        card()
                            .title("Elevated")
                            .subtitle("Elevated appearance")
                            .body(Text::new("Content goes here"))
                            .variant(CardVariant::Elevated),
                    )
                    .child(
                        card()
                            .title("Flat")
                            .subtitle("No border, minimal style")
                            .body(Text::new("Content goes here"))
                            .variant(CardVariant::Flat),
                    ),
                6,
            )
    }

    fn render_borders_demo(&self) -> impl View {
        // Border, title, separator, body, border: 5 rows, or 3 without a border.
        vstack()
            .child_sized(Text::new("Border Styles:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(
                card()
                    .title("Single Border")
                    .body(Text::new("Standard single-line border"))
                    .border_style(BorderType::Single),
                5,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                card()
                    .title("Rounded Border")
                    .body(Text::new("Rounded corners"))
                    .border_style(BorderType::Rounded),
                5,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                card()
                    .title("Double Border")
                    .body(Text::new("Double-line border"))
                    .border_style(BorderType::Double),
                5,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                card()
                    .title("No Border")
                    .body(Text::new("No visible border"))
                    .border_style(BorderType::None)
                    .variant(CardVariant::Filled),
                3,
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
        // A card's rows: its border, its title (and subtitle), and when
        // expanded a separator and the body's lines.
        let rows = |header: u16, body: u16, expanded: bool| {
            2 + header + if expanded { 1 + body } else { 0 }
        };

        // All three expanded are taller than the panel, so the third card
        // gets a column of its own.
        let left = vstack()
            .child_sized(indicator(0, self.card1_expanded), 1)
            .child_sized(
                card()
                    .title("User Profile")
                    .subtitle("Account Information")
                    .body(lines("Name: John Doe\nEmail: john@example.com"))
                    .collapsible(true)
                    .expanded(self.card1_expanded),
                rows(2, 2, self.card1_expanded),
            )
            .child_sized(Text::new(""), 1)
            .child_sized(indicator(1, self.card2_expanded), 1)
            .child_sized(
                card()
                    .title("Settings")
                    .body(lines("Theme: Dark\nLanguage: English"))
                    .collapsible(true)
                    .expanded(self.card2_expanded),
                rows(1, 2, self.card2_expanded),
            );

        let right = vstack()
            .child_sized(indicator(2, self.card3_expanded), 1)
            .child_sized(
                card()
                    .title("Notifications")
                    .subtitle("3 unread")
                    .body(lines("- New message\n- Update available\n- Reminder"))
                    .collapsible(true)
                    .expanded(self.card3_expanded),
                rows(2, 3, self.card3_expanded),
            );

        vstack()
            .child_sized(Text::new("Collapsible Cards:").bold(), 1)
            .child_sized(
                Text::new("(j/k or arrows: navigate, Space/Enter: toggle, h/l: collapse/expand)"),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child(hstack().gap(2).child(left).child(right))
    }
}

impl View for CardDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let title = " Card Widget Demo ";
        let header = hstack()
            .child_sized(Text::new(title).fg(Color::CYAN).bold(), cols(title))
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
            .child_sized(header, 1)
            .child_sized(tabs, 1)
            .child_sized(Text::new(""), 1)
            .child_flex(content, 1.0)
            .child_sized(Text::new(""), 1)
            .child_sized(help, 1)
            .render(ctx);
    }
}

/// One `Text` per line: a `Text` is a single line and does not break at `\n`.
fn lines(text: &str) -> impl View {
    text.lines().fold(vstack(), |stack, line| {
        stack.child_sized(Text::new(line), 1)
    })
}

/// Columns `s` takes on screen.
fn cols(s: &str) -> u16 {
    revue::utils::unicode::display_width(s) as u16
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let demo = CardDemo::new();

    app.run_with_handler(demo, |key_event, demo| demo.handle_key(&key_event.key))
}
