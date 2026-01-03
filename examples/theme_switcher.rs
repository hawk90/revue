//! Theme Switcher Example
//!
//! Demonstrates runtime theme switching with the reactive theme system.
//!
//! Run with: cargo run --example theme_switcher

use revue::prelude::*;

fn main() -> revue::Result<()> {
    let mut app = App::builder()
        .css(
            r#"
            .container {
                padding: 2;
            }
            .title {
                color: var(--theme-primary);
            }
            .hint {
                color: var(--theme-text-muted);
            }
        "#,
        )
        .build();

    let state = ThemeSwitcherState::new();

    app.run_with_handler(state, |event, state| state.handle_key(&event.key))
}

struct ThemeSwitcherState {
    picker: ThemePicker,
}

impl ThemeSwitcherState {
    fn new() -> Self {
        Self {
            picker: theme_picker()
                .themes([
                    "dark",
                    "light",
                    "dracula",
                    "nord",
                    "monokai",
                    "solarized_dark",
                ])
                .width(40),
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('q') => false,
            Key::Char('t') | Key::Tab => {
                cycle_theme();
                true
            }
            Key::Char('d') => {
                toggle_theme();
                true
            }
            Key::Enter | Key::Char(' ') | Key::Up | Key::Down | Key::Escape => {
                let event = KeyEvent::new(*key);
                self.picker.handle_key(&event);
                true
            }
            _ => false,
        }
    }
}

impl View for ThemeSwitcherState {
    fn render(&self, ctx: &mut RenderContext) {
        let theme = use_theme().get();

        vstack()
            .class("container")
            .gap(1)
            .child(Text::new("Theme Switcher Demo").class("title").bold())
            .child(divider())
            .child(self.picker.clone())
            .child(divider())
            .child(
                vstack()
                    .gap(0)
                    .child(Text::new(format!("Current: {}", theme.name)))
                    .child(Text::new(format!("Variant: {:?}", theme.variant))),
            )
            .child(divider())
            .child(
                hstack()
                    .gap(1)
                    .child(badge("Primary".to_string()).variant(BadgeVariant::Primary))
                    .child(badge("Success".to_string()).variant(BadgeVariant::Success))
                    .child(badge("Warning".to_string()).variant(BadgeVariant::Warning))
                    .child(badge("Error".to_string()).variant(BadgeVariant::Error)),
            )
            .child(divider())
            .child(
                Text::new("[t] Cycle theme  [d] Toggle dark/light  [Enter] Open picker  [q] Quit")
                    .class("hint"),
            )
            .render(ctx);
    }
}
