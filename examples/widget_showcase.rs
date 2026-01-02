//! Textual-style Widget Showcase
//!
//! Run with: cargo run --example widget_showcase

use revue::prelude::*;

// Textual-style colors
const BG_DARK: Color = Color::rgb(30, 30, 46); // Base background
const BG_PANEL: Color = Color::rgb(45, 45, 65); // Panel background
const BG_SURFACE: Color = Color::rgb(55, 55, 75); // Surface/card
const BG_HIGHLIGHT: Color = Color::rgb(80, 80, 120); // Highlight
const FG_PRIMARY: Color = Color::rgb(205, 214, 244); // Primary text
const FG_MUTED: Color = Color::rgb(147, 153, 178); // Muted text
const ACCENT: Color = Color::rgb(137, 180, 250); // Blue accent
const GREEN: Color = Color::rgb(166, 227, 161); // Success
const RED: Color = Color::rgb(243, 139, 168); // Error
const YELLOW: Color = Color::rgb(249, 226, 175); // Warning
const PINK: Color = Color::rgb(245, 194, 231); // Pink

#[derive(Default)]
struct WidgetShowcase {
    tab: usize,
    list_idx: usize,
    checked: bool,
    switch_on: bool,
}

impl WidgetShowcase {
    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Tab | Key::Right | Key::Char('l') => {
                self.tab = (self.tab + 1) % 7;
                true
            }
            Key::BackTab | Key::Left | Key::Char('h') => {
                self.tab = if self.tab == 0 { 6 } else { self.tab - 1 };
                true
            }
            Key::Char('1') => {
                self.tab = 0;
                true
            }
            Key::Char('2') => {
                self.tab = 1;
                true
            }
            Key::Char('3') => {
                self.tab = 2;
                true
            }
            Key::Char('4') => {
                self.tab = 3;
                true
            }
            Key::Char('5') => {
                self.tab = 4;
                true
            }
            Key::Char('6') => {
                self.tab = 5;
                true
            }
            Key::Char('7') => {
                self.tab = 6;
                true
            }
            Key::Up | Key::Char('k') => {
                self.list_idx = self.list_idx.saturating_sub(1);
                true
            }
            Key::Down | Key::Char('j') => {
                self.list_idx = (self.list_idx + 1).min(4);
                true
            }
            Key::Char(' ') => {
                self.checked = !self.checked;
                self.switch_on = !self.switch_on;
                true
            }
            _ => false,
        }
    }

    fn render_tabs(&self) -> impl View {
        hstack()
            .gap(0)
            .child(self.tab_item("Layout", 0))
            .child(self.tab_item("Input", 1))
            .child(self.tab_item("Display", 2))
            .child(self.tab_item("Data", 3))
            .child(self.tab_item("Charts", 4))
            .child(self.tab_item("Feedback", 5))
            .child(self.tab_item("Special", 6))
    }

    fn tab_item(&self, name: &str, idx: usize) -> impl View {
        let is_active = self.tab == idx;
        if is_active {
            Text::new(format!(" {} ", name))
                .fg(BG_DARK)
                .bg(ACCENT)
                .bold()
        } else {
            Text::new(format!(" {} ", name)).fg(FG_MUTED).bg(BG_PANEL)
        }
    }

    fn render_layout(&self) -> impl View {
        vstack()
            .gap(1)
            .child(
                vstack()
                    .child(Text::new(" Stack Demo ").fg(ACCENT).bold())
                    .child(
                        hstack()
                            .gap(2)
                            .child(Text::new("│").fg(FG_MUTED))
                            .child(Text::new("Horizontal").fg(FG_PRIMARY))
                            .child(Text::new("│").fg(FG_MUTED))
                            .child(Text::new("Items").fg(FG_PRIMARY))
                            .child(Text::new("│").fg(FG_MUTED)),
                    ),
            )
            .child(
                hstack()
                    .gap(1)
                    .child(Text::new(" Card A ").fg(FG_PRIMARY).bg(BG_SURFACE))
                    .child(Text::new(" Card B ").fg(FG_PRIMARY).bg(BG_SURFACE))
                    .child(Text::new(" Card C ").fg(FG_PRIMARY).bg(BG_SURFACE)),
            )
    }

    fn render_input(&self) -> impl View {
        vstack()
            .gap(1)
            .child(Text::new(" Buttons ").fg(ACCENT).bold())
            .child(
                hstack()
                    .gap(1)
                    .child(Text::new(" OK ").fg(BG_DARK).bg(FG_PRIMARY))
                    .child(Text::new(" Save ").fg(BG_DARK).bg(ACCENT))
                    .child(Text::new(" Delete ").fg(BG_DARK).bg(RED))
                    .child(Text::new(" Cancel ").fg(FG_MUTED).bg(BG_SURFACE)),
            )
            .child(Text::new(""))
            .child(Text::new(" Toggle Controls ").fg(ACCENT).bold())
            .child(
                hstack()
                    .gap(2)
                    .child(if self.checked {
                        Text::new(" ☑ Checked ").fg(GREEN)
                    } else {
                        Text::new(" ☐ Unchecked ").fg(FG_MUTED)
                    })
                    .child(if self.switch_on {
                        Text::new(" ●━━ ON ").fg(GREEN)
                    } else {
                        Text::new(" ━━○ OFF ").fg(FG_MUTED)
                    }),
            )
    }

    fn render_display(&self) -> impl View {
        vstack()
            .gap(1)
            .child(Text::new(" Text Styles ").fg(ACCENT).bold())
            .child(Text::new("  Normal text").fg(FG_PRIMARY))
            .child(Text::new("  Bold text").fg(FG_PRIMARY).bold())
            .child(Text::new("  Muted text").fg(FG_MUTED))
            .child(Text::new("  Accent color").fg(ACCENT))
            .child(Text::new("  Success").fg(GREEN))
            .child(Text::new("  Warning").fg(YELLOW))
            .child(Text::new("  Error").fg(RED))
            .child(Text::new(""))
            .child(Text::new(" Progress ").fg(ACCENT).bold())
            .child(Text::new("  ████████░░░░░░░░ 50%").fg(ACCENT))
            .child(Text::new("  ████████████░░░░ 75%").fg(GREEN))
    }

    fn render_data(&self) -> impl View {
        let items = ["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"];

        vstack()
            .gap(1)
            .child(Text::new(" List (↑↓ to navigate) ").fg(ACCENT).bold())
            .child(self.list_item(items[0], 0))
            .child(self.list_item(items[1], 1))
            .child(self.list_item(items[2], 2))
            .child(self.list_item(items[3], 3))
            .child(self.list_item(items[4], 4))
            .child(Text::new(""))
            .child(Text::new(" Table ").fg(ACCENT).bold())
            .child(Text::new("  Name        Age   City").fg(FG_MUTED))
            .child(Text::new("  ─────────────────────────").fg(BG_HIGHLIGHT))
            .child(Text::new("  Alice       28    NYC").fg(FG_PRIMARY))
            .child(Text::new("  Bob         35    LA").fg(FG_PRIMARY))
    }

    fn list_item(&self, text: &str, idx: usize) -> Text {
        if self.list_idx == idx {
            Text::new(format!("  ▸ {} ", text)).fg(BG_DARK).bg(ACCENT)
        } else {
            Text::new(format!("    {} ", text)).fg(FG_PRIMARY)
        }
    }

    fn render_charts(&self) -> impl View {
        vstack()
            .gap(1)
            .child(Text::new(" Sparkline ").fg(ACCENT).bold())
            .child(Text::new("  ▂▄▆█▅▃▇▄▂▅▇▃").fg(ACCENT))
            .child(Text::new(""))
            .child(Text::new(" Bar Chart ").fg(ACCENT).bold())
            .child(Text::new("  Mon ████████░░░░ 40%").fg(PINK))
            .child(Text::new("  Tue ██████████░░ 60%").fg(ACCENT))
            .child(Text::new("  Wed ████████████ 80%").fg(GREEN))
    }

    fn render_feedback(&self) -> impl View {
        vstack()
            .gap(1)
            .child(Text::new(" Notifications ").fg(ACCENT).bold())
            .child(Text::new("  ℹ Info: System ready").fg(ACCENT))
            .child(Text::new("  ✓ Success: File saved").fg(GREEN))
            .child(Text::new("  ⚠ Warning: Low memory").fg(YELLOW))
            .child(Text::new("  ✗ Error: Connection lost").fg(RED))
            .child(Text::new(""))
            .child(Text::new(" Rating ").fg(ACCENT).bold())
            .child(Text::new("  ★★★★☆ 4.0/5").fg(YELLOW))
    }

    fn render_special(&self) -> impl View {
        vstack()
            .gap(1)
            .child(Text::new(" Digits ").fg(ACCENT).bold())
            .child(Text::new("  ╔═══╗ ╔═══╗ ╔═══╗").fg(ACCENT))
            .child(Text::new("  ║ 1 ║ ║ 2 ║ ║ 3 ║").fg(ACCENT))
            .child(Text::new("  ╚═══╝ ╚═══╝ ╚═══╝").fg(ACCENT))
            .child(Text::new(""))
            .child(Text::new(" Links ").fg(ACCENT).bold())
            .child(Text::new("  🔗 https://github.com").fg(ACCENT).underline())
            .child(Text::new(""))
            .child(Text::new(" Timer ").fg(ACCENT).bold())
            .child(Text::new("  ⏱ 05:00").fg(GREEN))
    }
}

impl View for WidgetShowcase {
    fn render(&self, ctx: &mut RenderContext) {
        // Header
        Text::new(" Revue Widget Showcase ")
            .fg(FG_PRIMARY)
            .bg(BG_PANEL)
            .bold()
            .render(ctx);

        // Render based on tab
        match self.tab {
            0 => {
                vstack()
                    .gap(0)
                    .child(self.render_tabs())
                    .child(Text::new(""))
                    .child(self.render_layout())
                    .render(ctx);
            }
            1 => {
                vstack()
                    .gap(0)
                    .child(self.render_tabs())
                    .child(Text::new(""))
                    .child(self.render_input())
                    .render(ctx);
            }
            2 => {
                vstack()
                    .gap(0)
                    .child(self.render_tabs())
                    .child(Text::new(""))
                    .child(self.render_display())
                    .render(ctx);
            }
            3 => {
                vstack()
                    .gap(0)
                    .child(self.render_tabs())
                    .child(Text::new(""))
                    .child(self.render_data())
                    .render(ctx);
            }
            4 => {
                vstack()
                    .gap(0)
                    .child(self.render_tabs())
                    .child(Text::new(""))
                    .child(self.render_charts())
                    .render(ctx);
            }
            5 => {
                vstack()
                    .gap(0)
                    .child(self.render_tabs())
                    .child(Text::new(""))
                    .child(self.render_feedback())
                    .render(ctx);
            }
            _ => {
                vstack()
                    .gap(0)
                    .child(self.render_tabs())
                    .child(Text::new(""))
                    .child(self.render_special())
                    .render(ctx);
            }
        }

        // Footer
        Text::new(" 1-7: Tab │ ↑↓: Navigate │ Space: Toggle │ q: Quit ")
            .fg(FG_MUTED)
            .bg(BG_PANEL)
            .render(ctx);
    }
}

fn main() -> Result<()> {
    let mut app = App::builder()
        .css(
            r#"
            * {
                background: #1e1e2e;
            }
        "#,
        )
        .build();

    let showcase = WidgetShowcase::default();

    app.run_with_handler(showcase, |key_event, showcase| {
        showcase.handle_key(&key_event.key)
    })
}
