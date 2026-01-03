//! Textual-style Dashboard Demo
//!
//! Recreating a Textual-like dashboard layout with Revue
//!
//! Run with: cargo run --example textual_dashboard

use revue::prelude::*;
use revue::widget::*;

// Catppuccin Mocha colors
#[allow(dead_code)]
const BG_BASE: Color = Color::rgb(30, 30, 46);
#[allow(dead_code)]
const BG_SURFACE: Color = Color::rgb(49, 50, 68);
const BG_OVERLAY: Color = Color::rgb(69, 71, 90);
const TEXT: Color = Color::rgb(205, 214, 244);
const SUBTEXT: Color = Color::rgb(166, 173, 200);
const BLUE: Color = Color::rgb(137, 180, 250);
const GREEN: Color = Color::rgb(166, 227, 161);
#[allow(dead_code)]
const YELLOW: Color = Color::rgb(249, 226, 175);
#[allow(dead_code)]
const RED: Color = Color::rgb(243, 139, 168);
const PINK: Color = Color::rgb(245, 194, 231);
#[allow(dead_code)]
const TEAL: Color = Color::rgb(148, 226, 213);

#[derive(Default)]
struct Dashboard {
    progress1: f32,
    progress2: f32,
    selected_row: usize,
}

impl Dashboard {
    fn new() -> Self {
        Self {
            progress1: 0.7,
            progress2: 0.45,
            selected_row: 0,
        }
    }

    fn render_header(&self) -> impl View {
        vstack()
            .child(
                Text::new(" Posting data to httpbin.org ")
                    .fg(BG_BASE)
                    .bg(BLUE)
                    .bold(),
            )
            .child(Text::new(""))
    }

    fn render_json_panel(&self) -> impl View {
        let json_content = r#"{
  "title": "Revue TUI Framework",
  "version": "0.1.0",
  "features": ["css", "reactive", "widgets"],
  "author": {
    "name": "Developer",
    "active": true
  }
}"#;

        Border::rounded()
            .title(" JSON ")
            .fg(BG_OVERLAY)
            .child(vstack().child(self.render_json_syntax(json_content)))
    }

    fn render_json_syntax(&self, json: &str) -> impl View {
        let mut rich = RichText::new();

        for line in json.lines() {
            let trimmed = line.trim_start();
            let indent = " ".repeat(line.len() - trimmed.len());

            if trimmed.starts_with('"') {
                // Key or string value
                if let Some(colon_pos) = trimmed.find(':') {
                    let key = &trimmed[..colon_pos];
                    let value = &trimmed[colon_pos..];
                    rich.append(&indent, Style::new());
                    rich.append(key, Style::new().fg(BLUE));
                    rich.append(value.trim_end_matches(','), Style::new().fg(GREEN));
                    if value.ends_with(',') {
                        rich.append(",", Style::new().fg(TEXT));
                    }
                } else {
                    rich.append(line, Style::new().fg(GREEN));
                }
            } else if trimmed.starts_with('{')
                || trimmed.starts_with('}')
                || trimmed.starts_with('[')
                || trimmed.starts_with(']')
            {
                rich.append(line, Style::new().fg(TEXT));
            } else if trimmed == "true"
                || trimmed == "false"
                || trimmed.starts_with("true")
                || trimmed.starts_with("false")
            {
                rich.append(&indent, Style::new());
                rich.append(trimmed.trim_end_matches(','), Style::new().fg(PINK));
                if trimmed.ends_with(',') {
                    rich.append(",", Style::new().fg(TEXT));
                }
            } else {
                rich.append(line, Style::new().fg(TEXT));
            }
            rich.append("\n", Style::new());
        }

        rich
    }

    fn render_markdown_panel(&self) -> impl View {
        Border::rounded().title(" Markdown ").fg(BG_OVERLAY).child(
            vstack()
                .child(RichText::markup(
                    "[bold]Revue[/] is a [cyan]Vue-style[/] TUI framework",
                ))
                .child(Text::new(""))
                .child(RichText::markup("Features:"))
                .child(RichText::markup("  [green]•[/] CSS styling"))
                .child(RichText::markup("  [green]•[/] Reactive state"))
                .child(RichText::markup("  [green]•[/] 70+ widgets"))
                .child(Text::new(""))
                .child(RichText::markup("[dim]Built with Rust[/]")),
        )
    }

    fn render_csv_panel(&self) -> impl View {
        Border::rounded().title(" CSV Data ").fg(BG_OVERLAY).child(
            vstack()
                .child(Text::new(" Name       │ Value │ Status").fg(SUBTEXT))
                .child(Text::new("────────────┼───────┼────────").fg(BG_OVERLAY))
                .child(self.csv_row("Alpha", "100", "OK", 0))
                .child(self.csv_row("Beta", "250", "OK", 1))
                .child(self.csv_row("Gamma", "75", "WARN", 2))
                .child(self.csv_row("Delta", "320", "OK", 3)),
        )
    }

    fn csv_row(&self, name: &str, value: &str, status: &str, idx: usize) -> Text {
        let line = format!(" {:<10} │ {:>5} │ {:<6}", name, value, status);

        if idx == self.selected_row {
            Text::new(line).fg(BG_BASE).bg(BLUE)
        } else {
            Text::new(line).fg(TEXT)
        }
    }

    fn render_progress_panel(&self) -> impl View {
        Border::rounded().title(" Progress ").fg(BG_OVERLAY).child(
            vstack()
                .gap(1)
                .child(Text::new(" Downloading...").fg(SUBTEXT))
                .child(self.render_progress_bar(self.progress1, BLUE))
                .child(Text::new(""))
                .child(Text::new(" Processing...").fg(SUBTEXT))
                .child(self.render_progress_bar(self.progress2, GREEN)),
        )
    }

    fn render_progress_bar(&self, value: f32, color: Color) -> impl View {
        let width = 30;
        let filled = (value * width as f32) as usize;
        let empty = width - filled;

        RichText::new()
            .push(&"━".repeat(filled), Style::new().fg(color))
            .push(&"─".repeat(empty), Style::new().fg(BG_OVERLAY))
            .push(
                &format!(" {:>3}%", (value * 100.0) as u32),
                Style::new().fg(SUBTEXT),
            )
    }

    fn render_log_panel(&self) -> impl View {
        Border::rounded().title(" Log ").fg(BG_OVERLAY).child(
            vstack()
                .child(RichText::markup(
                    "[dim]12:00:01[/] [green]INFO[/]  Application started",
                ))
                .child(RichText::markup(
                    "[dim]12:00:02[/] [green]INFO[/]  Loading config...",
                ))
                .child(RichText::markup(
                    "[dim]12:00:03[/] [yellow]WARN[/]  Cache miss",
                ))
                .child(RichText::markup(
                    "[dim]12:00:04[/] [green]INFO[/]  Connected to server",
                ))
                .child(RichText::markup(
                    "[dim]12:00:05[/] [red]ERROR[/] Request timeout",
                )),
        )
    }

    fn render_footer(&self) -> impl View {
        hstack()
            .gap(2)
            .child(RichText::markup(" [bold blue]Q[/] Quit"))
            .child(RichText::markup("[bold blue]↑↓[/] Navigate"))
            .child(RichText::markup("[bold blue]Tab[/] Switch panel"))
    }
}

impl View for Dashboard {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .gap(0)
            .child(self.render_header())
            .child(
                hstack()
                    .gap(1)
                    .child(
                        vstack()
                            .gap(1)
                            .child(self.render_json_panel())
                            .child(self.render_markdown_panel()),
                    )
                    .child(
                        vstack()
                            .gap(1)
                            .child(self.render_csv_panel())
                            .child(self.render_progress_panel()),
                    ),
            )
            .child(self.render_log_panel())
            .child(Text::new(""))
            .child(self.render_footer())
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

    let dashboard = Dashboard::new();

    app.run_with_handler(dashboard, |event, dashboard| match event.key {
        Key::Char('q') | Key::Escape => std::process::exit(0),
        Key::Up | Key::Char('k') => {
            dashboard.selected_row = dashboard.selected_row.saturating_sub(1);
            true
        }
        Key::Down | Key::Char('j') => {
            dashboard.selected_row = (dashboard.selected_row + 1).min(3);
            true
        }
        Key::Char('+') | Key::Char('=') => {
            dashboard.progress1 = (dashboard.progress1 + 0.05).min(1.0);
            dashboard.progress2 = (dashboard.progress2 + 0.03).min(1.0);
            true
        }
        Key::Char('-') => {
            dashboard.progress1 = (dashboard.progress1 - 0.05).max(0.0);
            dashboard.progress2 = (dashboard.progress2 - 0.03).max(0.0);
            true
        }
        _ => false,
    })
}
