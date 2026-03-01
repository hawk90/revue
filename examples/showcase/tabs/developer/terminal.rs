//! Terminal widget demos

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::Terminal;

pub fn examples() -> Vec<Example> {
    let (primary, success, warning, error, info, muted, text, _) = theme_colors();

    vec![
        Example::new(
            "Terminal",
            "Interactive terminal emulator with command output",
            Border::rounded().title(" Terminal ").child(
                vstack()
                    .gap(1)
                    .child(Terminal::new(40, 5))
                    .child(Text::new(""))
                    .child(Text::new("• Command output").fg(muted))
                    .child(Text::new("• Interactive shell").fg(muted))
                    .child(Text::new("• Scroll history").fg(muted)),
            ),
        ),
        Example::new(
            "Shell Output",
            "Raw shell output display with ANSI color support",
            Border::rounded().title(" Shell Output ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("$ ls -la").fg(text))
                    .child(Text::new("total 48").fg(muted))
                    .child(Text::new("drwxr-xr-x  12 user  staff   384 Feb 28 10:00 .").fg(text))
                    .child(Text::new("drwxr-xr-x   5 user  staff   160 Feb 28 09:00 ..").fg(text))
                    .child(
                        Text::new("-rw-r--r--   1 user  staff  1024 Feb 28 10:00 Cargo.toml")
                            .fg(text),
                    )
                    .child(Text::new("drwxr-xr-x   6 user  staff   192 Feb 28 10:00 src").fg(text))
                    .child(Text::new(""))
                    .child(Text::new("• Raw output display").fg(muted))
                    .child(Text::new("• ANSI color support").fg(muted))
                    .child(Text::new("• Copy selection").fg(muted)),
            ),
        ),
        Example::new(
            "Command Log",
            "Command history with status indicators",
            Border::rounded().title(" Command Log ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("[OK] git status").fg(success))
                    .child(Text::new("     On branch main").fg(muted))
                    .child(Text::new("[OK] git pull").fg(success))
                    .child(Text::new("     Already up to date.").fg(muted))
                    .child(Text::new("[..] cargo test").fg(info))
                    .child(Text::new("     Running tests...").fg(muted))
                    .child(Text::new("[  ] cargo clippy").fg(muted))
                    .child(Text::new("     Queued").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("• Command history").fg(muted))
                    .child(Text::new("• Status indicators").fg(muted))
                    .child(Text::new("• Timestamped").fg(muted)),
            ),
        ),
        Example::new(
            "Console",
            "Log output with color-coded severity levels",
            Border::rounded().title(" Console ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("[INFO] Application started").fg(info))
                    .child(Text::new("[DEBUG] Loading configuration...").fg(muted))
                    .child(Text::new("[INFO] Configuration loaded").fg(info))
                    .child(Text::new("[WARN] Deprecated API used").fg(warning))
                    .child(Text::new("[ERROR] Connection failed").fg(error))
                    .child(Text::new("[INFO] Retrying connection...").fg(info))
                    .child(Text::new(""))
                    .child(Text::new("• Log levels").fg(muted))
                    .child(Text::new("• Color-coded").fg(muted))
                    .child(Text::new("• Filter by level").fg(muted)),
            ),
        ),
        Example::new(
            "REPL",
            "Read-Eval-Print Loop with input/output pairs",
            Border::rounded().title(" REPL ").child(
                vstack()
                    .gap(1)
                    .child(Text::new(">>> 2 + 2").fg(text))
                    .child(Text::new("4").fg(success))
                    .child(Text::new(""))
                    .child(Text::new(">>> \"hello\".to_uppercase()").fg(text))
                    .child(Text::new("\"HELLO\"").fg(success))
                    .child(Text::new(""))
                    .child(Text::new(">>> factorial(5)").fg(text))
                    .child(Text::new("_").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("• Read-Eval-Print Loop").fg(muted))
                    .child(Text::new("• Input/output pairs").fg(muted))
                    .child(Text::new("• History navigation").fg(muted)),
            ),
        ),
        Example::new(
            "Progress Commands",
            "Multi-step build pipelines and CI/CD output",
            Border::rounded().title(" Progress Commands ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Build output:").fg(primary))
                    .child(Text::new(""))
                    .child(Text::new("[OK] Compiling").fg(success))
                    .child(Text::new("[OK] Linking").fg(success))
                    .child(Text::new("[OK] Optimizing").fg(success))
                    .child(Text::new("[..] Packaging").fg(info))
                    .child(Text::new("[  ] Done").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("• Multi-step progress").fg(muted))
                    .child(Text::new("• Build pipelines").fg(muted))
                    .child(Text::new("• CI/CD output").fg(muted)),
            ),
        ),
    ]
}
