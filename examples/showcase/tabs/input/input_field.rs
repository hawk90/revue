//! Input field widget demos

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{number_input, password_input, Input, TextArea};

pub fn examples() -> Vec<Example> {
    let (primary, _, _, _, _info, muted, _text, _) = theme_colors();

    vec![
        Example::new(
            "Text Input",
            "Single line text input with placeholder and value",
            Border::rounded().title(" Text Input ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Basic text field:").fg(primary))
                    .child(Input::new().placeholder("Enter text..."))
                    .child(Text::new(""))
                    .child(Text::new("With default value:").fg(primary))
                    .child(Input::new().value("Pre-filled value"))
                    .child(Text::new(""))
                    .child(Text::new("• Single line text input").fg(muted))
                    .child(Text::new("• Placeholder for hints").fg(muted))
                    .child(Text::new("• Value for pre-filled content").fg(muted)),
            ),
        ),
        Example::new(
            "Search Input",
            "Search field with visual context and clear button",
            Border::rounded().title(" Search Input ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Search field:").fg(primary))
                    .child(Input::new().placeholder("Search..."))
                    .child(Text::new(""))
                    .child(Text::new("With clear button:").fg(primary))
                    .child(Input::new().value("Search term"))
                    .child(Text::new(""))
                    .child(Text::new("• Visual context").fg(muted))
                    .child(Text::new("• Quick reset").fg(muted)),
            ),
        ),
        Example::new(
            "Number Input",
            "Numeric input with range constraints",
            Border::rounded().title(" Number Input ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Numeric input:").fg(primary))
                    .child(number_input().value(0.0))
                    .child(Text::new(""))
                    .child(Text::new("With range:").fg(primary))
                    .child(number_input().value(50.0).min(0.0).max(100.0))
                    .child(Text::new(""))
                    .child(Text::new("• Number-only input").fg(muted))
                    .child(Text::new("• Min/max constraints").fg(muted)),
            ),
        ),
        Example::new(
            "Text Area",
            "Multi-line text input with scrolling support",
            Border::rounded().title(" Text Area ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Multi-line input:").fg(primary))
                    .child(
                        TextArea::new()
                            .placeholder("Enter description...\nMultiple lines supported."),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Multi-line text").fg(muted))
                    .child(Text::new("• Adjustable size").fg(muted))
                    .child(Text::new("• Scrolling for long content").fg(muted)),
            ),
        ),
        Example::new(
            "Specialized Inputs",
            "Password, email, and URL input fields",
            Border::rounded().title(" Specialized Inputs ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Password:").fg(primary))
                    .child(password_input("••••••••"))
                    .child(Text::new(""))
                    .child(Text::new("Email:").fg(primary))
                    .child(Input::new().placeholder("user@example.com"))
                    .child(Text::new(""))
                    .child(Text::new("URL:").fg(primary))
                    .child(Input::new().placeholder("https://example.com"))
                    .child(Text::new(""))
                    .child(Text::new("• Password: masked input").fg(muted))
                    .child(Text::new("• Email: email validation").fg(muted))
                    .child(Text::new("• URL: URL validation").fg(muted)),
            ),
        ),
    ]
}
