//! Data viewer widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{CsvViewer, Digits, JsonViewer, LogViewer};

pub fn render(frame: u64) -> impl View {
    let (primary, _success, _warning, _error, _info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" JSON Viewer ").child(
                        vstack()
                            .gap(1)
                            .child(JsonViewer::new())
                            .child(Text::new(""))
                            .child(Text::new("• Syntax highlighting").fg(muted))
                            .child(Text::new("• Collapsible nodes").fg(muted))
                            .child(Text::new("• Copy path/value").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" CSV Viewer ").child(
                        vstack()
                            .gap(1)
                            .child(CsvViewer::new())
                            .child(Text::new(""))
                            .child(Text::new("• Tabular display").fg(muted))
                            .child(Text::new("• Column sizing").fg(muted))
                            .child(Text::new("• Sort by column").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Log Viewer ").child(
                        vstack()
                            .gap(1)
                            .child(LogViewer::new())
                            .child(Text::new(""))
                            .child(Text::new("• Level coloring").fg(muted))
                            .child(Text::new("• Auto-scroll").fg(muted))
                            .child(Text::new("• Filter by level").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" YAML Viewer ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Text::new(
                                    r#"server:
  host: localhost
  port: 8080

database:
  driver: postgres
  host: db.example.com
logging:
  level: info"#,
                                )
                                .fg(text),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• YAML syntax").fg(muted))
                            .child(Text::new("• Key/value pairs").fg(muted))
                            .child(Text::new("• Nested structures").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" TOML Viewer ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Text::new(
                                    r#"[package]
name = "revue"
version = "2.52.0"
edition = "2021"

[dependencies]
ratatui = "0.29""#,
                                )
                                .fg(text),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• TOML syntax").fg(muted))
                            .child(Text::new("• Section headers").fg(muted))
                            .child(Text::new("• Cargo.toml support").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Digits ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Counter:").fg(primary))
                            .child(Digits::timer(frame))
                            .child(Text::new(""))
                            .child(Text::new("Large number:").fg(primary))
                            .child(Digits::new(12345))
                            .child(Text::new(""))
                            .child(Text::new("Percentage:").fg(primary))
                            .child(Digits::from_float(87.65, 1))
                            .child(Text::new(""))
                            .child(Text::new("• Large digit display").fg(muted))
                            .child(Text::new("• Timer format").fg(muted))
                            .child(Text::new("• Counter display").fg(muted)),
                    ),
                ),
        )
}
