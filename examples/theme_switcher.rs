//! Theme Switcher Example
//!
//! Demonstrates runtime theme switching with the reactive theme system.
//!
//! Run with: cargo run --example theme_switcher

use revue::prelude::*;
use revue::utils::unicode::display_width;

/// Themes offered by the picker.
const THEMES: [&str; 6] = [
    "dark",
    "light",
    "dracula",
    "nord",
    "monokai",
    "solarized_dark",
];

fn main() -> revue::Result<()> {
    // Colors come from the active theme in `render`, so they follow every
    // switch; a stylesheet is parsed once and would not.
    let mut app = App::builder().build();

    let state = ThemeSwitcherState::new();

    app.run_with_handler(state, |event, state| state.handle_key(&event.key))
}

struct ThemeSwitcherState {
    picker: ThemePicker,
}

impl ThemeSwitcherState {
    fn new() -> Self {
        Self {
            picker: theme_picker().themes(THEMES).width(40),
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

        // ThemePicker draws its dropdown inside its own area (it is not an
        // overlay), so give it room for the header row plus a bordered list
        // while open: 1 + 2 + one row per theme.
        let picker_rows = if self.picker.is_open() {
            3 + THEMES.len() as u16
        } else {
            1
        };
        // A badge is its label plus one space of padding on each side.
        let badge_cols = |label: &str| display_width(label) as u16 + 2;

        // Every row is `child_sized` to its content. (A stack sizes children to
        // their content on its own; the explicit sizes pin the layout.)
        vstack()
            .gap(1)
            .child_sized(
                Text::new("Theme Switcher Demo")
                    .fg(theme.palette.primary)
                    .bold(),
                1,
            )
            .child_sized(divider(), 1)
            .child_sized(self.picker.clone(), picker_rows)
            .child_sized(divider(), 1)
            .child_sized(
                vstack()
                    .gap(0)
                    .child_sized(Text::new(format!("Current: {}", theme.name)), 1)
                    .child_sized(Text::new(format!("Variant: {:?}", theme.variant)), 1),
                2,
            )
            .child_sized(divider(), 1)
            .child_sized(
                hstack()
                    .gap(1)
                    .child_sized(
                        badge("Primary".to_string()).variant(BadgeVariant::Primary),
                        badge_cols("Primary"),
                    )
                    .child_sized(
                        badge("Success".to_string()).variant(BadgeVariant::Success),
                        badge_cols("Success"),
                    )
                    .child_sized(
                        badge("Warning".to_string()).variant(BadgeVariant::Warning),
                        badge_cols("Warning"),
                    )
                    .child_sized(
                        badge("Error".to_string()).variant(BadgeVariant::Error),
                        badge_cols("Error"),
                    ),
                1,
            )
            .child_sized(divider(), 1)
            .child_sized(
                Text::new("[t] Cycle theme  [d] Toggle dark/light  [Enter] Open picker  [q] Quit")
                    .fg(theme.colors.text_muted),
                1,
            )
            .render(ctx);
    }
}
