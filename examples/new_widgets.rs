//! New Widgets Showcase - Demonstrates Digits, Link, MaskedInput, SelectionList, OptionList
//!
//! Run with: cargo run --example new_widgets

use revue::prelude::*;
use revue::widget::{
    DigitStyle, Digits, Link, LinkStyle, MaskedInput, OptionItem, OptionList, SelectionItem,
    SelectionList, SelectionStyle, ValidationState,
};

/// Current demo tab
#[derive(Clone, Copy, PartialEq)]
enum DemoTab {
    Digits,
    Links,
    MaskedInput,
    SelectionList,
    OptionList,
}

impl DemoTab {
    fn name(&self) -> &str {
        match self {
            DemoTab::Digits => "Digits",
            DemoTab::Links => "Links",
            DemoTab::MaskedInput => "MaskedInput",
            DemoTab::SelectionList => "SelectionList",
            DemoTab::OptionList => "OptionList",
        }
    }

    fn all() -> &'static [DemoTab] {
        &[
            DemoTab::Digits,
            DemoTab::Links,
            DemoTab::MaskedInput,
            DemoTab::SelectionList,
            DemoTab::OptionList,
        ]
    }
}

/// Demo application state
struct NewWidgetsDemo {
    /// Current tab
    tab: DemoTab,
    /// Timer seconds
    timer_secs: u64,
    /// Digit style index
    digit_style: usize,
    /// Password input
    password: MaskedInput,
    /// PIN input
    pin: MaskedInput,
    /// Selection list
    features: SelectionList,
    /// Option list
    menu: OptionList,
    /// Active input (for MaskedInput tab)
    active_input: usize,
    /// Frame counter for animations
    frame: usize,
}

impl NewWidgetsDemo {
    fn new() -> Self {
        let password = MaskedInput::password()
            .placeholder("Enter password...")
            .label("Password")
            .show_strength(true)
            .allow_reveal(true)
            .min_length(8)
            .width(30);

        let pin = MaskedInput::pin(6)
            .placeholder("______")
            .label("PIN Code")
            .width(20);

        let features = SelectionList::new(vec![
            SelectionItem::new("Dark Mode").description("Enable dark theme"),
            SelectionItem::new("Notifications").description("Push notifications"),
            SelectionItem::new("Auto-save").description("Save every 5 minutes"),
            SelectionItem::new("Spell Check").description("Check spelling as you type"),
            SelectionItem::new("Line Numbers").description("Show line numbers in editor"),
            SelectionItem::new("Word Wrap").description("Wrap long lines"),
            SelectionItem::new("Minimap").description("Show code minimap"),
            SelectionItem::new("Breadcrumbs").description("Show file path breadcrumbs"),
        ])
        .title("Editor Features")
        .show_descriptions(true)
        .show_count(true)
        .max_selections(5)
        .style(SelectionStyle::Checkbox)
        .selected(vec![0, 2, 4])
        .focused(true);

        let menu = OptionList::new()
            .title("Actions")
            .group("File")
            .add_option(OptionItem::new("New File").hint("Ctrl+N").icon("📄 "))
            .add_option(OptionItem::new("Open File").hint("Ctrl+O").icon("📂 "))
            .add_option(OptionItem::new("Save").hint("Ctrl+S").icon("💾 "))
            .add_option(
                OptionItem::new("Save As...")
                    .hint("Ctrl+Shift+S")
                    .icon("📝 "),
            )
            .separator()
            .group("Edit")
            .add_option(OptionItem::new("Undo").hint("Ctrl+Z").icon("↩️ "))
            .add_option(OptionItem::new("Redo").hint("Ctrl+Y").icon("↪️ "))
            .add_option(OptionItem::new("Cut").hint("Ctrl+X").icon("✂️ "))
            .add_option(OptionItem::new("Copy").hint("Ctrl+C").icon("📋 "))
            .add_option(OptionItem::new("Paste").hint("Ctrl+V").icon("📌 "))
            .separator()
            .group("View")
            .add_option(OptionItem::new("Zoom In").hint("Ctrl++"))
            .add_option(OptionItem::new("Zoom Out").hint("Ctrl+-"))
            .add_option(OptionItem::new("Full Screen").hint("F11").disabled(true))
            .max_visible(12)
            .show_icons(true)
            .focused(true);

        Self {
            tab: DemoTab::Digits,
            timer_secs: 3661, // 1:01:01
            digit_style: 0,
            password,
            pin,
            features,
            menu,
            active_input: 0,
            frame: 0,
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        // Tab switching
        match key {
            Key::Char('1') => {
                self.tab = DemoTab::Digits;
                return true;
            }
            Key::Char('2') => {
                self.tab = DemoTab::Links;
                return true;
            }
            Key::Char('3') => {
                self.tab = DemoTab::MaskedInput;
                return true;
            }
            Key::Char('4') => {
                self.tab = DemoTab::SelectionList;
                return true;
            }
            Key::Char('5') => {
                self.tab = DemoTab::OptionList;
                return true;
            }
            Key::Tab => {
                let tabs = DemoTab::all();
                let idx = tabs.iter().position(|&t| t == self.tab).unwrap_or(0);
                self.tab = tabs[(idx + 1) % tabs.len()];
                return true;
            }
            Key::BackTab => {
                let tabs = DemoTab::all();
                let idx = tabs.iter().position(|&t| t == self.tab).unwrap_or(0);
                self.tab = tabs[(idx + tabs.len() - 1) % tabs.len()];
                return true;
            }
            _ => {}
        }

        // Tab-specific handling
        match self.tab {
            DemoTab::Digits => self.handle_digits_key(key),
            DemoTab::Links => false, // Links are display-only
            DemoTab::MaskedInput => self.handle_masked_input_key(key),
            DemoTab::SelectionList => self.handle_selection_list_key(key),
            DemoTab::OptionList => self.handle_option_list_key(key),
        }
    }

    fn handle_digits_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up | Key::Char('+') => {
                self.timer_secs += 1;
                true
            }
            Key::Down | Key::Char('-') => {
                self.timer_secs = self.timer_secs.saturating_sub(1);
                true
            }
            Key::Left => {
                self.timer_secs = self.timer_secs.saturating_sub(60);
                true
            }
            Key::Right => {
                self.timer_secs += 60;
                true
            }
            Key::Char('s') => {
                self.digit_style = (self.digit_style + 1) % 4;
                true
            }
            Key::Char('r') => {
                self.timer_secs = 0;
                true
            }
            _ => false,
        }
    }

    fn handle_masked_input_key(&mut self, key: &Key) -> bool {
        // Switch between inputs
        if matches!(key, Key::Down | Key::Up) {
            self.active_input = 1 - self.active_input;
            return true;
        }

        let input = if self.active_input == 0 {
            &mut self.password
        } else {
            &mut self.pin
        };

        match key {
            Key::Backspace => {
                input.delete_backward();
                true
            }
            Key::Delete => {
                input.delete_forward();
                true
            }
            Key::Left => {
                input.move_left();
                true
            }
            Key::Right => {
                input.move_right();
                true
            }
            Key::Home => {
                input.move_start();
                true
            }
            Key::End => {
                input.move_end();
                true
            }
            Key::Enter => {
                input.validate();
                true
            }
            Key::Char(c) => {
                // 'r' toggles reveal for password input
                if *c == 'r' && self.active_input == 0 {
                    self.password.toggle_reveal();
                } else {
                    input.insert_char(*c);
                }
                true
            }
            _ => false,
        }
    }

    fn handle_selection_list_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up | Key::Char('k') => {
                self.features.highlight_previous();
                true
            }
            Key::Down | Key::Char('j') => {
                self.features.highlight_next();
                true
            }
            Key::Char(' ') | Key::Enter => {
                self.features.toggle_highlighted();
                true
            }
            Key::Char('a') => {
                self.features.select_all();
                true
            }
            Key::Char('n') => {
                self.features.deselect_all();
                true
            }
            Key::Home | Key::Char('g') => {
                self.features.highlight_first();
                true
            }
            Key::End | Key::Char('G') => {
                self.features.highlight_last();
                true
            }
            _ => false,
        }
    }

    fn handle_option_list_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up | Key::Char('k') => {
                self.menu.highlight_previous();
                true
            }
            Key::Down | Key::Char('j') => {
                self.menu.highlight_next();
                true
            }
            Key::Enter | Key::Char(' ') => {
                self.menu.select_highlighted();
                true
            }
            Key::Home | Key::Char('g') => {
                self.menu.highlight_first();
                true
            }
            Key::End | Key::Char('G') => {
                self.menu.highlight_last();
                true
            }
            Key::Escape => {
                self.menu.clear_selection();
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

        for (i, tab) in DemoTab::all().iter().enumerate() {
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

    fn render_digits_demo(&self) -> impl View {
        let styles = [
            DigitStyle::Block,
            DigitStyle::Thin,
            DigitStyle::Ascii,
            DigitStyle::Braille,
        ];
        let style = styles[self.digit_style];
        let style_name = match style {
            DigitStyle::Block => "Block",
            DigitStyle::Thin => "Thin",
            DigitStyle::Ascii => "ASCII",
            DigitStyle::Braille => "Braille",
        };

        let timer_display = Digits::timer(self.timer_secs).style(style).fg(Color::CYAN);

        let counter = Digits::new(self.frame)
            .style(style)
            .fg(Color::GREEN)
            .min_width(4);

        let price = Digits::from_float(1234.56, 2)
            .style(style)
            .fg(Color::YELLOW)
            .separator(',');

        let digit_rows = timer_display.height() as u16;

        vstack()
            .child_sized(
                Text::new(format!("Style: {} (press 's' to change)", style_name)).bold(),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Timer (↑↓: +/-1s, ←→: +/-1m, r: reset):"), 1)
            .child_sized(timer_display, digit_rows)
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Frame Counter:"), 1)
            .child_sized(counter, digit_rows)
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Price Display:"), 1)
            .child_sized(price, digit_rows)
    }

    fn render_links_demo(&self) -> impl View {
        let link1 = Link::new("https://github.com/anthropics/claude-code")
            .text("Claude Code on GitHub")
            .style(LinkStyle::Underline)
            .fg(Color::CYAN);

        let link2 = Link::new("https://docs.rs")
            .text("Rust Documentation")
            .style(LinkStyle::Bracketed)
            .fg(Color::GREEN);

        let link3 = Link::new("https://crates.io")
            .text("Crates.io")
            .style(LinkStyle::Arrow)
            .fg(Color::YELLOW);

        let link4 = Link::new("https://rust-lang.org")
            .text("Rust Language")
            .style(LinkStyle::Icon)
            .fg(Color::MAGENTA);

        let link5 = Link::new("https://example.com/disabled")
            .text("Disabled Link")
            .disabled(true);

        let row = |label: &str, link: Link| {
            hstack()
                .child_sized(Text::new(label), cols(label))
                .child(link)
        };

        vstack()
            .child_sized(Text::new("Link Styles:").bold(), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(row("Underline: ", link1), 1)
            .child_sized(row("Bracketed: ", link2), 1)
            .child_sized(row("Arrow:     ", link3), 1)
            .child_sized(row("Icon:      ", link4), 1)
            .child_sized(row("Disabled:  ", link5), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new("Links support OSC 8 hyperlinks in compatible terminals.")
                    .fg(Color::rgb(100, 100, 100)),
                1,
            )
            .child_sized(
                Text::new("Click or Ctrl+Click to open in browser.").fg(Color::rgb(100, 100, 100)),
                1,
            )
    }

    fn render_masked_input_demo(&self) -> impl View {
        let pwd_focused = self.active_input == 0;
        let pin_focused = self.active_input == 1;

        let password = self.password.clone().focused(pwd_focused);
        let pin = self.pin.clone().focused(pin_focused);

        let password_rows = masked_input_rows(&password);
        let pin_rows = masked_input_rows(&pin);

        vstack()
            .child_sized(Text::new("Masked Input Fields:").bold(), 1)
            .child_sized(
                Text::new("(↑↓: switch fields, type to enter, 'r': toggle reveal)"),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(
                if pwd_focused {
                    Text::new("> Password").fg(Color::CYAN)
                } else {
                    Text::new("  Password")
                },
                1,
            )
            .child_sized(password, password_rows)
            .child_sized(Text::new(""), 1)
            .child_sized(
                if pin_focused {
                    Text::new("> PIN").fg(Color::CYAN)
                } else {
                    Text::new("  PIN")
                },
                1,
            )
            .child_sized(pin, pin_rows)
            .child_sized(Text::new(""), 1)
            .child_sized(Text::new("Features:").bold(), 1)
            .child_sized(Text::new("  - Password strength indicator"), 1)
            .child_sized(Text::new("  - Reveal toggle (press 'r')"), 1)
            .child_sized(Text::new("  - Validation on Enter"), 1)
            .child_sized(Text::new("  - Show last 4 digits for credit cards"), 1)
    }

    fn render_selection_list_demo(&self) -> impl View {
        let selected_count = self.features.get_selected().len();
        let selected_items: Vec<&str> = self
            .features
            .get_selected_items()
            .iter()
            .map(|item| item.text.as_str())
            .collect();

        // The list is 19 rows: title, count, 8 items with a description line
        // each, and its help line.
        vstack()
            .child_sized(Text::new("Multi-Selection List:").bold(), 1)
            .child_sized(
                Text::new("(↑↓: navigate, Space: toggle, a: all, n: none)"),
                1,
            )
            .child_sized(Text::new(""), 1)
            .child_sized(self.features.clone(), 19)
            .child_sized(
                Text::new(format!(
                    "Selected ({}): {}",
                    selected_count,
                    selected_items.join(", ")
                ))
                .fg(Color::GREEN),
                1,
            )
    }

    fn render_option_list_demo(&self) -> impl View {
        let selected = self
            .menu
            .get_selected()
            .map(|item| item.text.as_str())
            .unwrap_or("None");

        // The menu is 18 rows: title, 3 groups, 2 separators and 12 options.
        vstack()
            .child_sized(Text::new("Option List (Menu):").bold(), 1)
            .child_sized(Text::new("(↑↓: navigate, Enter: select, Esc: clear)"), 1)
            .child_sized(Text::new(""), 1)
            .child_sized(self.menu.clone(), 18)
            .child_sized(Text::new(""), 1)
            .child_sized(
                Text::new(format!("Selected: {}", selected)).fg(Color::GREEN),
                1,
            )
    }

    fn update(&mut self) {
        self.frame += 1;
        self.password.update();
        self.pin.update();
    }
}

impl View for NewWidgetsDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let title = " New Widgets Demo ";
        let header = hstack()
            .child_sized(Text::new(title).fg(Color::CYAN).bold(), cols(title))
            .child(Text::new(" | Tab/Shift+Tab or 1-5 to switch").fg(Color::rgb(100, 100, 100)));

        let tabs = self.render_tabs();

        let content = match self.tab {
            DemoTab::Digits => Border::rounded()
                .title("Digits Widget")
                .child(self.render_digits_demo()),
            DemoTab::Links => Border::rounded()
                .title("Link Widget")
                .child(self.render_links_demo()),
            DemoTab::MaskedInput => Border::rounded()
                .title("MaskedInput Widget")
                .child(self.render_masked_input_demo()),
            DemoTab::SelectionList => Border::rounded()
                .title("SelectionList Widget")
                .child(self.render_selection_list_demo()),
            DemoTab::OptionList => Border::rounded()
                .title("OptionList Widget")
                .child(self.render_option_list_demo()),
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

/// Rows a `MaskedInput` draws: its label, the field, the strength bar once
/// something is typed, and a validation message when there is one.
fn masked_input_rows(input: &MaskedInput) -> u16 {
    let label = input.get_label().is_some() as u16;
    let strength = (input.get_show_strength() && !input.get_value().is_empty()) as u16;
    let message = matches!(input.get_validation(), ValidationState::Invalid(_)) as u16;
    label + 1 + strength + message
}

/// Columns `s` takes on screen.
fn cols(s: &str) -> u16 {
    revue::utils::unicode::display_width(s) as u16
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let demo = NewWidgetsDemo::new();

    app.run_with_handler(demo, |key_event, demo| {
        demo.update();
        demo.handle_key(&key_event.key)
    })
}
