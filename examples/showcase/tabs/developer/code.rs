//! Code widget demos

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{CodeEditor, Language, Markdown};

pub fn examples() -> Vec<Example> {
    let (primary, _success, _warning, _error, info, muted, text, _) = theme_colors();

    vec![
        Example::new(
            "Code Editor",
            "Editable code with syntax highlighting and line numbers",
            Border::rounded().title(" Code Editor ").child(
                vstack()
                    .gap(1)
                    .child(
                        CodeEditor::new()
                            .language(Language::Rust)
                            .content(
                                r#"fn main() {
    let greeting = "Hello, World!";
    println!("{}", greeting);
}"#,
                            )
                            .line_numbers(true),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Syntax highlighting").fg(muted))
                    .child(Text::new("• Line numbers").fg(muted))
                    .child(Text::new("• Multiple languages").fg(muted)),
            ),
        ),
        Example::new(
            "Code Block",
            "Read-only code display with copy support",
            Border::rounded().title(" Code Block ").child(
                vstack()
                    .gap(1)
                    .child(
                        CodeEditor::new()
                            .language(Language::Rust)
                            .content(
                                r#"use revue::prelude::*;

fn main() -> Result<()> {
    let app = App::builder().build();
    app.run(MyView, |event, state, app| {
        true
    })
}"#,
                            )
                            .read_only(true),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Read-only display").fg(muted))
                    .child(Text::new("• Copy button").fg(muted))
                    .child(Text::new("• File title").fg(muted)),
            ),
        ),
        Example::new(
            "Inline Code",
            "Code spans mixed with regular text",
            Border::rounded().title(" Inline Code ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Use the ").fg(text))
                    .child(Text::new("  use revue::prelude::*;").fg(info))
                    .child(Text::new(" macro to import.").fg(text))
                    .child(Text::new(""))
                    .child(Text::new("The ").fg(text))
                    .child(Text::new("  App::builder()").fg(info))
                    .child(Text::new(" method creates a new app.").fg(text))
                    .child(Text::new(""))
                    .child(Text::new("• Inline code spans").fg(muted))
                    .child(Text::new("• Mixed with text").fg(muted))
                    .child(Text::new("• Monospace font").fg(muted)),
            ),
        ),
        Example::new(
            "Markdown Renderer",
            "Render markdown with headers, lists, code blocks, and links",
            Border::rounded().title(" Markdown Renderer ").child(
                vstack()
                    .gap(1)
                    .child(Markdown::new(
                        r#"# Documentation

## Features
- **Fast**: Built with Rust
- **Simple**: Easy to use API
- **Flexible**: 92+ widgets

## Example
```rust
Text::new("Hello, World!")
```

> Note: This is a blockquote.

See [docs](https://docs.rs) for more.
                            "#,
                    ))
                    .child(Text::new(""))
                    .child(Text::new("• Headers and lists").fg(muted))
                    .child(Text::new("• Code blocks").fg(muted))
                    .child(Text::new("• Links and quotes").fg(muted)),
            ),
        ),
        Example::new(
            "Diagram",
            "ASCII art diagrams with boxes and arrows",
            Border::rounded().title(" Diagram ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("ASCII diagram:").fg(primary))
                    .child(Text::new(""))
                    .child(Text::new("+--------+          +--------+").fg(text))
                    .child(Text::new("| Client | -------> | Server |").fg(text))
                    .child(Text::new("+--------+ Request  +--------+").fg(text))
                    .child(Text::new("    ^                  |").fg(muted))
                    .child(Text::new("    +---- Response ---+").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("• ASCII art diagrams").fg(muted))
                    .child(Text::new("• Boxes and arrows").fg(muted))
                    .child(Text::new("• Flow visualization").fg(muted)),
            ),
        ),
        Example::new(
            "Syntax Highlight",
            "Supported programming languages for syntax highlighting",
            Border::rounded().title(" Syntax Highlight ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Supported languages:").fg(primary))
                    .child(Text::new("• Rust").fg(text))
                    .child(Text::new("• Python").fg(text))
                    .child(Text::new("• JavaScript/TypeScript").fg(text))
                    .child(Text::new("• Go").fg(text))
                    .child(Text::new("• C/C++").fg(text))
                    .child(Text::new("• JSON/YAML/TOML").fg(text))
                    .child(Text::new("• SQL").fg(text))
                    .child(Text::new("• Shell/Bash").fg(text))
                    .child(Text::new(""))
                    .child(Text::new("Custom themes available").fg(muted)),
            ),
        ),
    ]
}
