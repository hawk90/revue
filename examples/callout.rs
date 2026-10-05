//! Callout Widget Demo - Demonstrates different callout types and variants
//!
//! Run with: cargo run --example callout

use revue::prelude::*;
use revue::widget::{Callout, CalloutVariant};

/// Current view mode
#[derive(Clone, Copy, PartialEq)]
enum ViewTab {
    Types,
    Variants,
    Collapsible,
}

impl ViewTab {
    fn name(&self) -> &str {
        match self {
            ViewTab::Types => "Types",
            ViewTab::Variants => "Variants",
            ViewTab::Collapsible => "Collapsible",
        }
    }

    fn all() -> &'static [ViewTab] {
        &[ViewTab::Types, ViewTab::Variants, ViewTab::Collapsible]
    }
}

/// Demo application state
struct CalloutDemo {
    /// Current tab
    tab: ViewTab,
    /// Collapsible callouts
    note_callout: Callout,
    tip_callout: Callout,
    warning_callout: Callout,
    /// Currently focused collapsible (0-2)
    focused_idx: usize,
}

impl CalloutDemo {
    fn new() -> Self {
        Self {
            tab: ViewTab::Types,
            note_callout: Callout::note(
                "This is a collapsible note.\nYou can hide or show the content.",
            )
            .collapsible(true)
            .expanded(true),
            tip_callout: Callout::tip(
                "Pro tip: Use keyboard shortcuts!\nPress Space or Enter to toggle.",
            )
            .collapsible(true)
            .expanded(false),
            warning_callout: Callout::warning("Be careful with this action.\nIt cannot be undone.")
                .collapsible(true)
                .expanded(true),
            focused_idx: 0,
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        // Tab switching
        match key {
            Key::Char('1') => {
                self.tab = ViewTab::Types;
                return true;
            }
            Key::Char('2') => {
                self.tab = ViewTab::Variants;
                return true;
            }
            Key::Char('3') => {
                self.tab = ViewTab::Collapsible;
                return true;
            }
            Key::Tab => {
                let tabs = ViewTab::all();
                let idx = tabs.iter().position(|&t| t == self.tab).unwrap_or(0);
                self.tab = tabs[(idx + 1) % tabs.len()];
                return true;
            }
            Key::BackTab => {
                let tabs = ViewTab::all();
                let idx = tabs.iter().position(|&t| t == self.tab).unwrap_or(0);
                self.tab = tabs[(idx + tabs.len() - 1) % tabs.len()];
                return true;
            }
            _ => {}
        }

        // Collapsible tab specific handling
        if self.tab == ViewTab::Collapsible {
            match key {
                Key::Up | Key::Char('k') => {
                    if self.focused_idx > 0 {
                        self.focused_idx -= 1;
                    }
                    return true;
                }
                Key::Down | Key::Char('j') => {
                    if self.focused_idx < 2 {
                        self.focused_idx += 1;
                    }
                    return true;
                }
                Key::Enter | Key::Char(' ') => {
                    match self.focused_idx {
                        0 => self.note_callout.toggle(),
                        1 => self.tip_callout.toggle(),
                        2 => self.warning_callout.toggle(),
                        _ => {}
                    }
                    return true;
                }
                Key::Left | Key::Char('h') => {
                    match self.focused_idx {
                        0 => self.note_callout.collapse(),
                        1 => self.tip_callout.collapse(),
                        2 => self.warning_callout.collapse(),
                        _ => {}
                    }
                    return true;
                }
                Key::Right | Key::Char('l') => {
                    match self.focused_idx {
                        0 => self.note_callout.expand(),
                        1 => self.tip_callout.expand(),
                        2 => self.warning_callout.expand(),
                        _ => {}
                    }
                    return true;
                }
                _ => {}
            }
        }

        false
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

    fn render_types_demo(&self) -> impl View {
        // A filled callout draws two rows: its title and its one-line message.
        vstack()
            .child_sized(Text::new("All Callout Types:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(
                Callout::note("This is a note callout for general information."),
                2,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Callout::tip("This is a tip callout for helpful suggestions."),
                2,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Callout::important("This is an important callout for key information."),
                2,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Callout::warning("This is a warning callout for potential issues."),
                2,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Callout::danger("This is a danger callout for critical warnings."),
                2,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Callout::info("This is an info callout for supplementary details."),
                2,
            )
    }

    fn render_variants_demo(&self) -> impl View {
        vstack()
            .child_sized(Text::new("Callout Variants:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new("Filled (default):").fg(Color::rgb(150, 150, 150)),
                1,
            )
            .child_sized(
                Callout::tip("Filled variant with background color.")
                    .variant(CalloutVariant::Filled),
                2,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Left Border:").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(
                Callout::warning("Left border variant - minimal with accent.")
                    .variant(CalloutVariant::LeftBorder),
                2,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Minimal:").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(
                Callout::info("Minimal variant - just icon and text.")
                    .variant(CalloutVariant::Minimal),
                2,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new("Custom Title and Icon:").fg(Color::rgb(150, 150, 150)),
                1,
            )
            .child_sized(
                Callout::note("You can customize the title and icon.")
                    .title("Custom Title")
                    .custom_icon('*'),
                2,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("No Icon:").fg(Color::rgb(150, 150, 150)), 1)
            .child_sized(Callout::important("Callout without icon.").icon(false), 2)
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
        // Expanded, these callouts draw their title and two lines of message;
        // collapsed, just the title.
        let rows = |callout: &Callout| if callout.is_expanded() { 3 } else { 1 };

        vstack()
            .child_sized(Text::new("Collapsible Callouts:").bold(), 1)
            .child_sized(
                Text::new("(j/k or arrows: navigate, Space/Enter: toggle, h/l: collapse/expand)"),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(indicator(0, self.note_callout.is_expanded()), 1)
            .child_sized(self.note_callout.clone(), rows(&self.note_callout))
            .child_sized(Text::new(""), 1)
            .child_sized(indicator(1, self.tip_callout.is_expanded()), 1)
            .child_sized(self.tip_callout.clone(), rows(&self.tip_callout))
            .child_sized(Text::new(""), 1)
            .child_sized(indicator(2, self.warning_callout.is_expanded()), 1)
            .child_sized(self.warning_callout.clone(), rows(&self.warning_callout))
    }
}

impl View for CalloutDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let title = " Callout Widget Demo ";
        let header = hstack()
            .child_sized(Text::new(title).fg(Color::CYAN).bold(), cols(title))
            .child(Text::new(" | Tab/Shift+Tab or 1-3 to switch").fg(Color::rgb(100, 100, 100)));

        let tabs = self.render_tabs();

        let content = match self.tab {
            ViewTab::Types => Border::rounded()
                .title("Callout Types")
                .child(self.render_types_demo()),
            ViewTab::Variants => Border::rounded()
                .title("Callout Variants")
                .child(self.render_variants_demo()),
            ViewTab::Collapsible => Border::rounded()
                .title("Collapsible Callouts")
                .child(self.render_collapsible_demo()),
        };

        let help =
            Text::new("Press 'q' to quit | Tab: next | Shift+Tab: prev").fg(Color::rgb(80, 80, 80));

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
    let demo = CalloutDemo::new();

    app.run_with_handler(demo, |key_event, demo| demo.handle_key(&key_event.key))
}
